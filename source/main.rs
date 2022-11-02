#![no_std]
#![feature(start)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");
mod inline;
pub use inline::*;

use core::{panic::PanicInfo, alloc};

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut a: f32 = 0.0;
    let col: [u32; 3] = [0xFFFFFFFF, 0xAAAAAAFF, 0x666666FF];
    let cubeZ: f32 = 0.0;

    unsafe {
        GRRLIB_Init();
        WPAD_Init();

        GRRLIB_Settings.antialias = true;

        GRRLIB_SetBackgroundColour(0x00, 0x00, 0x00, 0xFF);
        GRRLIB_Camera3dSettings(0.0, 0.0, 13.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

        loop {
            WPAD_ScanPads();
            if WPAD_ButtonsDown(0) & WPAD_BUTTON_HOME > 0 {
                break;
            }

            GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
            GRRLIB_ObjectView(0.0,0.0,cubeZ, a,a*2.0,a*3.0,1.0,1.0,1.0);
            GX_Begin(GX_QUADS as u8, GX_VTXFMT0 as u8, 24);
                GX_Position3f32(-1.0,1.0,-1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(-1.0,-1.0,-1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0,-1.0,-1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0,1.0,-1.0);
                GX_Color1u32(col[0]);
    
                // GX_Position3f32(-1.0,1.0,1.0);
                // GX_Color1u32(col[0]);
                // GX_Position3f32(-1.0,-1.0,1.0);
                // GX_Color1u32(col[0]);
                // GX_Position3f32(1.0,-1.0,1.0);
                // GX_Color1u32(col[0]);
                // GX_Position3f32(1.0,1.0,1.0);
                // GX_Color1u32(col[0]);
    
                // GX_Position3f32(-1.0,1.0,1.0);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0,1.0,1.0);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0,1.0,-1.0);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(-1.0,1.0,-1.0);
                // GX_Color1u32(col[1]);
    
                // GX_Position3f32(-1.0,-1.0,1.0);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0,-1.0,1.0);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0,-1.0,-1.0);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(-1.0,-1.0,-1.0);
                // GX_Color1u32(col[1]);
    
                // GX_Position3f32(-1.0,1.0,1.0);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(-1.0,1.0,-1.0);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(-1.0,-1.0,-1.0);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(-1.0,-1.0,1.0);
                // GX_Color1u32(col[2]);
    
                // GX_Position3f32(1.0,1.0,1.0);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(1.0,1.0,-1.0);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(1.0,-1.0,-1.0);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(1.0,-1.0,1.0);
                // GX_Color1u32(col[2]);
            GX_End();
            a += 0.5;

            GRRLIB_Render();
        }
        GRRLIB_Exit();
    }
    return 0;
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}