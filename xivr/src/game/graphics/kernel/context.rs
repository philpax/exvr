#![allow(dead_code)]

use crate::game::graphics::kernel::{ImmediateContext, ShaderCommand};
use crate::hooks::graphics::kernel::immediate_context::{XIVRCommand, XIVRCommandPayload};
use crate::module::{Module, GAME_MODULE};
use macros::game_class;

game_class!(Context, {
    size: 0x1158,
    fields: {
        [0x8] sort_key_maybe: u32,
    },
    functions: {
        [signature("4C 8D 4A 0F 4C 8B C1")] fn alloc(&mut self, size: usize) -> *mut u8;
        [signature("83 41 30 FF")] fn push_back_command(&mut self, cmd: &ShaderCommand) -> usize;
    }
});

fn get_tls_index(module: &Module) -> u32 {
    struct TlsDirectory {
        _tls_start: *const u8,
        _tls_end: *const u8,
        tls_index: *const u32,
        // rest elided
    }

    unsafe {
        let dir_offset = module.rel_to_abs_addr(0x240) as *const u32;
        let dir = module.rel_to_abs_addr((*dir_offset) as isize) as *const TlsDirectory;
        *((*dir).tls_index)
    }
}

#[naked]
pub(self) extern "C" fn get_context(_tls_index: u32) -> &'static mut Context {
    unsafe {
        asm! {
            "MOV rax, gs:58h",
            "MOV rax, [rax+rcx*8]",
            "MOV rcx, 250h",
            "MOV rax, [rax+rcx]",
            "RET",
            options(noreturn)
        }
    }
}

impl Context {
    pub fn get_for_current_thread() -> Option<&'static mut Context> {
        unsafe {
            let module = GAME_MODULE.get()?;
            let tls_index = get_tls_index(module);

            Some(get_context(tls_index))
        }
    }

    pub fn push_back_xivr_command(
        &mut self,
        callback: fn(&'static ImmediateContext, &XIVRCommandPayload) -> (),
        payload: XIVRCommandPayload,
    ) {
        unsafe {
            let cmd = self.alloc(std::mem::size_of::<XIVRCommand>()) as *mut XIVRCommand;
            *cmd = XIVRCommand::new(callback, payload);
            self.push_back_command(&*(cmd as *mut ShaderCommand));
        }
    }
}