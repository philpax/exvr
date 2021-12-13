#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(core_intrinsics)]

mod game;
mod hooks;
#[macro_use]
mod log;
mod debugger;
mod module;
mod xr;
#[macro_use]
mod util;
mod ct_config;

use hooks::HookState;
use log::Logger;
use module::{Module, GAME_MODULE};

use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Error, Result};
use cimgui as ig;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console::{AllocConsole, FreeConsole};
use windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread;

static mut THIS_MODULE: Option<HINSTANCE> = None;
// should use a state machine for this, but don't want to deal with atomicity yet
static mut TIER1_LOADED: AtomicBool = AtomicBool::new(false);
static mut TIER2_LOADED: AtomicBool = AtomicBool::new(false);

#[repr(C, packed)]
pub struct LoadParameters {
    logger: log::LogType,
    imgui_context: *mut ig::Context,
    imgui_allocator_alloc: ig::MemAllocFunc,
    imgui_allocator_free: ig::MemFreeFunc,
    imgui_allocator_user_data: *mut c_void,
}

unsafe fn patch_symbol_search_path() -> Result<()> {
    use windows::Win32::Foundation::PWSTR;
    use windows::Win32::System::Diagnostics::Debug::{SymGetSearchPathW, SymSetSearchPathW};
    use windows::Win32::System::Threading::GetCurrentProcess;

    let current_process = GetCurrentProcess();
    let our_module = Module::from_handle(THIS_MODULE.as_ref().expect("module not set"));
    let directory = our_module
        .path()
        .and_then(|p| p.parent())
        .and_then(|p| p.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("failed to retrieve module"))?;

    // This is very silly. We would like to mutate the search path of the process
    // to include where our DLLs came from, but we can't do that without being sure
    // that dbghelp has been loaded and SymInitialize has been called. The backtrace
    // crate will do this for us, but only when a backtrace occurs. So... let's
    // call backtrace to force initialisation!
    backtrace::Backtrace::new_unresolved();

    let path = {
        let mut buf = vec![0u16; 1024];
        if !SymGetSearchPathW(current_process, PWSTR(buf.as_mut_ptr()), buf.len() as u32).as_bool()
        {
            Err(anyhow::anyhow!("failed to get search path"))?
        }

        let len = buf
            .iter()
            .position(|c| *c == 0)
            .ok_or_else(|| anyhow::anyhow!("failed to retrieve length"))?;

        String::from_utf16(&buf[..len])?
    };

    let new_path = if path.contains(&directory) {
        path
    } else {
        directory + ";" + &path
    };

    let mut new_buf: Vec<u16> = new_path.encode_utf16().collect();
    new_buf.push(0);
    if !SymSetSearchPathW(current_process, PWSTR(new_buf.as_mut_ptr())).as_bool() {
        Err(anyhow::anyhow!("failed to set search path"))?
    }

    Ok(())
}

unsafe fn tier1_load(parameters: Option<&LoadParameters>) -> Result<()> {
    log!("tier1", "start");
    let mut modules = Module::get_all();
    let ffxiv_module = modules
        .iter_mut()
        .find(|x| x.filename().as_deref() == Some("ffxiv_dx11.exe"))
        .ok_or_else(|| Error::msg("failed to find ff14 module"))?;
    ffxiv_module.backup_image();

    GAME_MODULE
        .set(ffxiv_module.clone())
        .map_err(|_| Error::msg("failed to set module"))?;
    log!("tier1", "located module");

    patch_symbol_search_path()?;
    log!("tier1", "patched symbol search path");

    hooks::Patcher::create()?;
    debugger::Debugger::create()?;
    HookState::create()?;
    log!("tier1", "installed hooks");

    if let Some(parameters) = parameters {
        ig::set_current_context(parameters.imgui_context);
        ig::set_allocator_functions(
            parameters.imgui_allocator_alloc,
            parameters.imgui_allocator_free,
            parameters.imgui_allocator_user_data,
        );
        log!("tier1", "initialised imgui");
    }
    TIER1_LOADED.store(true, Ordering::SeqCst);
    log!("tier1", "complete");

    Ok(())
}

fn tier1_loaded() -> bool {
    unsafe { TIER1_LOADED.load(Ordering::SeqCst) }
}

unsafe fn tier2_load() -> Result<()> {
    log!("tier2", "start");
    xr::XR::create()?;
    TIER2_LOADED.store(true, Ordering::SeqCst);
    log!("tier2", "complete");
    Ok(())
}

fn tier2_loaded() -> bool {
    unsafe { TIER2_LOADED.load(Ordering::SeqCst) }
}

unsafe fn xivr_load_impl(parameters: *const LoadParameters) -> Result<()> {
    if cfg!(not(feature = "dalamud")) {
        use c_str_macro::c_str;
        use libc::{fdopen, freopen};

        AllocConsole();

        let stdout = fdopen(1, c_str!("w").as_ptr());
        let stderr = fdopen(2, c_str!("w").as_ptr());
        freopen(c_str!("CONOUT$").as_ptr(), c_str!("w").as_ptr(), stdout);
        freopen(c_str!("CONOUT$").as_ptr(), c_str!("w").as_ptr(), stderr);
    }

    let parameters = parameters.as_ref();
    Logger::create(parameters.map(|x| x.logger))?;

    std::panic::set_hook(Box::new(|info| {
        match (info.payload().downcast_ref::<&str>(), info.location()) {
            (Some(msg), Some(loc)) => {
                log!("panic", "Panic! {:?} at {}:{}", msg, loc.file(), loc.line())
            }
            (Some(msg), None) => log!("panic", "Panic! {:?}", msg),
            (None, Some(loc)) => log!("panic", "Panic! at {}:{}", loc.file(), loc.line()),
            (None, None) => log!("panic", "Panic! something at somewhere"),
        };

        log!("panic", "{:?}", backtrace::Backtrace::new_unresolved());
    }));

    let r = std::panic::catch_unwind(|| tier1_load(parameters));
    match r {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(err),
        Err(msg) => Err(Error::msg(
            msg.downcast_ref::<&str>()
                .copied()
                .unwrap_or("Failed initialisation"),
        )),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn xivr_load(parameters: *const LoadParameters) -> bool {
    let result = xivr_load_impl(parameters);
    util::handle_error(result).is_some()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn xivr_unload() {
    std::thread::spawn(|| {
        log!("xivr", "unloading!");
        xr::XR::destroy();
        HookState::destroy();
        debugger::Debugger::destroy();
        hooks::Patcher::destroy();

        Logger::destroy();

        if cfg!(not(feature = "dalamud")) {
            FreeConsole();
        }

        FreeLibraryAndExitThread(THIS_MODULE, 0);
    });
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
#[cfg(feature = "dalamud")]
pub unsafe extern "system" fn xivr_draw_ui() {
    util::handle_error_in_block(|| {
        if let Some(debugger) = debugger::Debugger::get_mut() {
            debugger.draw_ui()?;
        }
        Ok(())
    });
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
#[cfg(feature = "dalamud")]
pub unsafe extern "system" fn DllMain(module: HINSTANCE, _reason: u32, _: *mut c_void) -> bool {
    if THIS_MODULE.is_none() {
        THIS_MODULE = Some(module);
    }
    true
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
#[cfg(not(feature = "dalamud"))]
pub unsafe extern "system" fn DllMain(module: HINSTANCE, reason: u32, _: *mut c_void) -> bool {
    use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

    if THIS_MODULE.is_none() {
        THIS_MODULE = Some(module);
    }

    match reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(|| {
                xivr_load(std::ptr::null());
            });
        }
        _ => {}
    }
    true
}
