use crate::game::graphics::kernel::{ShaderCommand, ShaderCommandType, Texture};
use crate::module::Module;

use std::collections::HashMap;
use std::string::ToString;
use std::time::{Duration, Instant};

use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumDiscriminants};

use cimgui as ig;

struct Ptr<T>(*const T);
unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}
impl<T> Copy for Ptr<T> {}
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Display, EnumDiscriminants, EnumCount, Clone)]
enum CommandPayload {
    SetRenderTargets(Vec<Ptr<Texture>>),
    SetViewports,
    SetViewportsFancy,
    SetScissorRect,
    Clear,
    Draw,
    DrawIndexed,
    DrawIndexedInstanced,
    DispatchComputeShader,
    XIVRHijack,
    CopyTexture {
        dst: Ptr<Texture>,
        src: Ptr<Texture>,
    },
    UnknownDraw,
    CopyResource,
    ResetRendererMaybe,
    Unknown1,
    CopySubresourceRegion,
    SomethingWithStrings,
    XIVRMarker(String),
}

impl CommandPayload {
    fn title(&self) -> String {
        match self {
            Self::XIVRMarker(s) => s.to_string(),
            _ => self.to_string(),
        }
    }

    fn colour(&self) -> ig::Color {
        let type_index = CommandPayloadDiscriminants::from(self) as u32;
        let hue = type_index as f32 / CommandPayload::COUNT as f32;
        ig::Color::from_hsv(hue, 0.6, 0.8)
    }
}

#[derive(Clone)]
struct Command {
    payload: CommandPayload,
    backtrace: backtrace::Backtrace,
    thread_id: u32,
    duration: Duration,
}

enum CommandStreamState {
    Uncaptured,
    WantToCapture,
    Capturing {
        start_instant: Instant,
        stream: Vec<Command>,
    },
    Captured {
        stream: Vec<Command>,
        selected_index: Option<usize>,
    },
}

struct InspectedTexture {
    texture: *const Texture,
    width: u32,
    height: u32,
    format: u32,
}

struct CommandStreamUI {
    module_name_lookup: HashMap<*mut u8, String>,
    inspected_textures: HashMap<*const Texture, InspectedTexture>,
}
impl CommandStreamUI {
    pub fn new() -> CommandStreamUI {
        let module_name_lookup: HashMap<_, _> = Module::get_all()
            .iter()
            .map(|m| (m.base, m.filename().unwrap_or("unknown".to_string())))
            .collect();
        let inspected_textures = HashMap::new();

        CommandStreamUI {
            module_name_lookup,
            inspected_textures,
        }
    }

    fn module_name_from_mba(&self, mba: *const u8) -> String {
        self.module_name_lookup
            .get(&(mba as *mut _))
            .cloned()
            .unwrap_or(format!("{:X?}", mba))
    }

    fn inspect_texture(&mut self, texture: *const Texture) {
        use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;

        let mut desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        unsafe {
            (*(*texture).texture_ptr()).GetDesc(&mut desc);
        }

        self.inspected_textures.insert(
            texture,
            InspectedTexture {
                texture,
                width: desc.Width as u32,
                height: desc.Height as u32,
                format: desc.Format.0 as u32,
            },
        );
    }

    fn draw_cmd(&mut self, index: usize, cmd: &Command) -> anyhow::Result<()> {
        ig::begin_group();
        if ig::begin_child("Item view", Some(ig::Vec2::new(0.0, -1.0)), None, None)? {
            {
                ig::same_line(Some(0.0), Some(0.0));
                ig::textf!("#{}", index);

                ig::same_line(Some(0.0), Some(4.0));
                ig::push_style_color(ig::Col::Text, cmd.payload.colour());
                ig::textf!("{}", cmd.payload.title());
                ig::pop_style_color(1);
            }

            ig::separator();
            {
                ig::textf!("Thread ID: {}", cmd.thread_id);
                ig::textf!("Timestamp: {:.3}ms", cmd.duration.as_secs_f64() * 1_000.0);
            }

            ig::separator();
            if ig::collapsing_header("Data", None, None)? {
                match &cmd.payload {
                    CommandPayload::SetRenderTargets(rts) => {
                        ig::text("Render Targets: ");

                        for rt in rts {
                            ig::bullet();
                            if ig::small_button(&format!("{:X?}", rt.0))? {
                                self.inspect_texture(rt.0);
                            }
                        }
                    }
                    CommandPayload::CopyTexture { dst, src } => {
                        ig::text("Destination: ");
                        ig::same_line(None, Some(0.0));
                        if ig::small_button(&format!("{:X?}", dst.0))? {
                            self.inspect_texture(dst.0);
                        }

                        ig::text("Source: ");
                        ig::same_line(None, Some(0.0));
                        if ig::small_button(&format!("{:X?}", src.0))? {
                            self.inspect_texture(src.0);
                        }
                    }
                    _ => {
                        ig::text("No additional data available.");
                    }
                }
            }

            if ig::collapsing_header("Callstack", None, None)? {
                if ig::begin_table("xivr_debugger_callstack", 2, None, None, None)? {
                    ig::table_setup_column("Module", None, None, None)?;
                    ig::table_setup_column("Address", None, None, None)?;
                    ig::table_headers_row();

                    for frame in cmd.backtrace.frames().iter().skip(10) {
                        let mba = frame.module_base_address().unwrap_or(std::ptr::null_mut());
                        let address = unsafe { frame.ip().offset_from(mba) };

                        ig::table_next_row(None, None);
                        {
                            ig::table_next_column();
                            ig::text(&self.module_name_from_mba(mba as *const _));
                            ig::table_next_column();
                            let addr_str = format!("0x{:0width$X}", address, width = 6);
                            if ig::small_button(&addr_str)? {
                                ig::set_clipboard_text(&addr_str)?;
                            }
                        }
                    }
                    ig::end_table();
                }
            }
            ig::end_child();
        }
        ig::end_group();

        Ok(())
    }

    pub fn draw_captured(&mut self, state: &mut CommandStreamState) -> anyhow::Result<()> {
        if let CommandStreamState::Captured {
            stream,
            ref mut selected_index,
        } = state
        {
            if ig::begin_child(
                "Command Stream",
                Some(ig::Vec2::new(300.0, 0.0)),
                Some(true),
                None,
            )? {
                for (i, cmd) in stream.iter().enumerate() {
                    let is_selected = *selected_index == Some(i);
                    let name = format!("{}: {}", i, cmd.payload.title());

                    ig::push_style_color(ig::Col::Text, cmd.payload.colour());
                    if ig::selectable(&name, Some(is_selected), None, None)? {
                        *selected_index = Some(i);
                    }
                    ig::pop_style_color(1);

                    if is_selected {
                        ig::set_item_default_focus();
                    }
                }
                ig::end_child();
            }

            ig::same_line(None, None);
            if let Some(index) = selected_index {
                let cmd = &stream[*index];
                self.draw_cmd(*index, cmd)?;
            }
        }

        Ok(())
    }

    fn draw_inspected_texture(&self, tex: &InspectedTexture) -> anyhow::Result<bool> {
        let mut open = true;
        let rt_size = ig::Vec2::new(tex.width as f32 / 4.0, tex.height as f32 / 4.0);

        ig::set_next_window_size(
            ig::Vec2::new(rt_size.x, rt_size.y + 100.0),
            Some(ig::Cond::FirstUseEver),
        );
        if ig::begin(
            &format!("Texture {:X?}", tex.texture),
            Some(&mut open),
            None,
        )? {
            use windows::Abi;

            ig::image(
                unsafe { (*(*tex.texture).shader_resource_view_ptr()).abi() },
                rt_size,
                None,
                None,
                None,
                None,
            );

            ig::textf!("Width: {}", tex.width);
            ig::textf!("Height: {}", tex.height);
            ig::textf!("Format: {}", tex.format);

            ig::end();
        }

        Ok(open)
    }

    fn draw(&mut self, state: &mut CommandStreamState) -> anyhow::Result<()> {
        ig::new_line();
        {
            ig::same_line(None, Some(0.0));
            if ig::button("Capture", None)? {
                *state = CommandStreamState::WantToCapture;
            }

            ig::same_line(None, None);
            if let CommandStreamState::Captured { stream, .. } = state {
                ig::textf!("{} commands", stream.len());
            }
        }

        if let CommandStreamState::Captured { .. } = state {
            self.draw_captured(state)?;
        } else {
            ig::new_line();
            ig::separator();
            ig::text("Capture a frame to proceed.");
        }

        let mut textures_to_remove = vec![];
        for inspected_texture in self.inspected_textures.values() {
            if !self.draw_inspected_texture(&inspected_texture)? {
                textures_to_remove.push(inspected_texture.texture);
            }
        }
        for texture in textures_to_remove {
            self.inspected_textures.remove(&texture);
        }

        Ok(())
    }
}

pub struct CommandStream {
    state: CommandStreamState,
    ui: CommandStreamUI,
}
impl CommandStream {
    pub fn new() -> CommandStream {
        CommandStream {
            state: CommandStreamState::Uncaptured,
            ui: CommandStreamUI::new(),
        }
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        self.end_capture()?;
        let should_capture = if let CommandStreamState::WantToCapture = &self.state {
            true
        } else {
            false
        };

        if should_capture {
            self.start_capture()?;
        }
        Ok(())
    }

    pub fn start_capture(&mut self) -> anyhow::Result<()> {
        self.state = CommandStreamState::Capturing {
            start_instant: Instant::now(),
            stream: vec![],
        };
        Ok(())
    }

    pub fn end_capture(&mut self) -> anyhow::Result<()> {
        if let CommandStreamState::Capturing { stream, .. } = &self.state {
            self.state = CommandStreamState::Captured {
                stream: stream.clone(),
                selected_index: None,
            };
        }
        Ok(())
    }

    fn push_back_command(&mut self, payload: CommandPayload) -> anyhow::Result<()> {
        if let CommandStreamState::Capturing {
            stream,
            start_instant,
        } = &mut self.state
        {
            use bindings::Windows::Win32::System::Threading::GetCurrentThreadId;
            let backtrace = backtrace::Backtrace::new_unresolved();

            stream.push(Command {
                payload,
                backtrace,
                thread_id: unsafe { GetCurrentThreadId() },
                duration: Instant::now() - *start_instant,
            });
        }

        Ok(())
    }

    pub fn add_command(&mut self, cmd: &'static ShaderCommand) -> anyhow::Result<()> {
        self.push_back_command(match cmd.cmd_type {
            ShaderCommandType::SetRenderTargets => unsafe {
                let rts = cmd.payload.set_render_targets.get_render_target_slice();
                CommandPayload::SetRenderTargets(rts.iter().map(|x| Ptr(*x)).collect())
            },
            ShaderCommandType::SetViewports => CommandPayload::SetViewports,
            ShaderCommandType::SetViewportsFancy => CommandPayload::SetViewportsFancy,
            ShaderCommandType::SetScissorRect => CommandPayload::SetScissorRect,
            ShaderCommandType::Clear => CommandPayload::Clear,
            ShaderCommandType::Draw => CommandPayload::Draw,
            ShaderCommandType::DrawIndexed => CommandPayload::DrawIndexed,
            ShaderCommandType::DrawIndexedInstanced => CommandPayload::DrawIndexedInstanced,
            ShaderCommandType::DispatchComputeShader => CommandPayload::DispatchComputeShader,
            ShaderCommandType::XIVRHijack => CommandPayload::XIVRHijack,
            ShaderCommandType::CopyTexture => unsafe {
                let p = &cmd.payload.copy_texture;
                CommandPayload::CopyTexture {
                    dst: Ptr(*p.dst_resource_ptr()),
                    src: Ptr(*p.src_resource_ptr()),
                }
            },
            ShaderCommandType::UnknownDraw => CommandPayload::UnknownDraw,
            ShaderCommandType::CopyResource => CommandPayload::CopyResource,
            ShaderCommandType::ResetRendererMaybe => CommandPayload::ResetRendererMaybe,
            ShaderCommandType::Unknown1 => CommandPayload::Unknown1,
            ShaderCommandType::CopySubresourceRegion => CommandPayload::CopySubresourceRegion,
            ShaderCommandType::SomethingWithStrings => CommandPayload::SomethingWithStrings,
        })
    }

    pub fn add_marker(&mut self, msg: &str) -> anyhow::Result<()> {
        self.push_back_command(CommandPayload::XIVRMarker(msg.to_string()))
    }

    pub fn draw_ui(&mut self) -> anyhow::Result<()> {
        self.ui.draw(&mut self.state)
    }
}
