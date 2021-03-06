// E8 ? ? ? ? C6 83 ? ? ? ? ? 48 8B 4B 70

use crate::log;

use detour::static_detour;

static_detour! {
    pub static Swapchain_Present_Detour: fn(usize);
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { Swapchain_Present_Detour.disable() };
        if let Err(e) = res {
            log!(
                "error",
                "error while disabling swapchain detour: {}",
                e.to_string()
            );
        }
    }
}

fn swapchain_present_hook(swapchain: usize) {
    crate::util::handle_error_in_block(|| {
        use crate::xr::XR;
        if let Some(xr) = XR::get_mut() {
            xr.swapchain_present()?;
        }
        Ok(())
    });

    Swapchain_Present_Detour.call(swapchain);
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use std::mem;

    let module = crate::util::game_module_mut()?;
    let swapchain_present: fn(usize) = mem::transmute(
        module.scan_for_relative_callsite("E8 ? ? ? ? C6 83 ? ? ? ? ? 48 8B 4B 70")?,
    );

    Swapchain_Present_Detour.initialize(swapchain_present, swapchain_present_hook)?;
    Swapchain_Present_Detour.enable()?;

    Ok(HookState {})
}
