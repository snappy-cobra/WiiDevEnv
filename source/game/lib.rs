#![no_std]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");

use core::{panic::PanicInfo, array};

#[no_mangle]
pub extern "C" fn run() {
    let mut a: f32 = 0.0;
    let col: [u32; 3] = [0xFFFFFFFF, 0xAAAAAAFF, 0x666666FF];
    let cubeZ: i32 = 0;

    unsafe {
        GRRLIB_Init();
        WPAD_Init();

        GRRLIB_Settings.antialias = true;

        GRRLIB_SetBackgroundColour(0x00, 0x00, 0x00, 0xFF);
        GRRLIB_Camera3dSettings(0.0f,0.0f,13.0f, 0,1,0, 0,0,0);

        loop {
            WPAD_ScanPads();
            if (WPAD_ButtonsDown(0) & WPAD_BUTTON_HOME) {
                break;
            }

            GRRLIB_3dMode(0.1,1000,45,0,0);
            GRRLIB_ObjectView(0,0,cubeZ, a,a*2,a*3,1,1,1);
            GX_Begin(GX_QUADS, GX_VTXFMT0, 24);
                GX_Position3f32(-1.0f,1.0f,-1.0f);
                GX_Color1u32(col[0]);
                GX_Position3f32(-1.0f,-1.0f,-1.0f);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0f,-1.0f,-1.0f);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0f,1.0f,-1.0f);
                GX_Color1u32(col[0]);
    
                // GX_Position3f32(-1.0f,1.0f,1.0f);
                // GX_Color1u32(col[0]);
                // GX_Position3f32(-1.0f,-1.0f,1.0f);
                // GX_Color1u32(col[0]);
                // GX_Position3f32(1.0f,-1.0f,1.0f);
                // GX_Color1u32(col[0]);
                // GX_Position3f32(1.0f,1.0f,1.0f);
                // GX_Color1u32(col[0]);
    
                // GX_Position3f32(-1.0f,1.0f,1.0f);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0f,1.0f,1.0f);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0f,1.0f,-1.0f);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(-1.0f,1.0f,-1.0f);
                // GX_Color1u32(col[1]);
    
                // GX_Position3f32(-1.0f,-1.0f,1.0f);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0f,-1.0f,1.0f);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(1.0f,-1.0f,-1.0f);
                // GX_Color1u32(col[1]);
                // GX_Position3f32(-1.0f,-1.0f,-1.0f);
                // GX_Color1u32(col[1]);
    
                // GX_Position3f32(-1.0f,1.0f,1.0f);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(-1.0f,1.0f,-1.0f);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(-1.0f,-1.0f,-1.0f);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(-1.0f,-1.0f,1.0f);
                // GX_Color1u32(col[2]);
    
                // GX_Position3f32(1.0f,1.0f,1.0f);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(1.0f,1.0f,-1.0f);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(1.0f,-1.0f,-1.0f);
                // GX_Color1u32(col[2]);
                // GX_Position3f32(1.0f,-1.0f,1.0f);
                // GX_Color1u32(col[2]);
            GX_End();
            a += 0.5;

            GRRLIB_Render();
        }
        GRRLIB_Exit();
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}