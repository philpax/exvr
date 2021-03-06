use crate::game::graphics::kernel::Texture;
use macros::game_class;

use windows::Win32::Graphics::Dxgi::IDXGISwapChain;

game_class!(SwapChain, {
    size: 0x70,
    fields: {
        [0x38] width: u32,
        [0x3C] height: u32,
        [0x58] back_buffer: &'static mut Texture,
        [0x60] depth_stencil: &'static mut Texture,
        [0x68] swapchain: IDXGISwapChain,
    }
});
