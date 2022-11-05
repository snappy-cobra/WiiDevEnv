
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// FIXME: This allow will eventually cancel an error, eventually remove it.
#![allow(unaligned_references)]
#![allow(unused_imports)]
#![no_std]
#![feature(start)]

extern crate alloc;
use ogc_rs::prelude::*;

include!("grrlib.rs");

mod inline;
pub use inline::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let col: [u32; 3] = [0xFFFFFFFF, 0xAAAAAAFF, 0x666666FF];
    let mut a = 0.0;
    let mut cubeZ = 0.0;

    unsafe {
        // Initialize
        GRRLIB_Init();
        PAD_Init();
        GRRLIB_Settings.antialias = true;

        GRRLIB_SetBackgroundColour(0x00, 0x00, 0x00, 0xFF);
        GRRLIB_Camera3dSettings(
            0.0, 0.0, 13.0, 
            0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0
        );

        // Main render loop
        loop {
            // Process input
            PAD_ScanPads();
            if (PAD_ButtonsDown(0) as u32 & PAD_BUTTON_START) > 0 {break}
            if (PAD_ButtonsHeld(0) as u32 & PAD_BUTTON_A) > 0 {cubeZ += 1.0}
            if (PAD_ButtonsHeld(0) as u32 & PAD_BUTTON_B) > 0 {cubeZ -= 1.0}

            // Set the projection
            GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
            GRRLIB_ObjectView(
                0.0, 0.0, cubeZ, 
                a, a*2.0, a*3.0,
                1.0, 1.0, 1.0
            );
            
            // Define the cube
            GX_Begin(GX_QUADS as u8, GX_VTXFMT0 as u8, 24u16);
                GX_Position3f32(-1.0, 1.0, -1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(-1.0,-1.0,-1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0,-1.0,-1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0,1.0,-1.0);
                GX_Color1u32(col[0]);

                GX_Position3f32(-1.0,1.0,1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(-1.0,-1.0,1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0,-1.0,1.0);
                GX_Color1u32(col[0]);
                GX_Position3f32(1.0,1.0,1.0);
                GX_Color1u32(col[0]);

                GX_Position3f32(-1.0,1.0,1.0);
                GX_Color1u32(col[1]);
                GX_Position3f32(1.0,1.0,1.0);
                GX_Color1u32(col[1]);
                GX_Position3f32(1.0,1.0,-1.0);
                GX_Color1u32(col[1]);
                GX_Position3f32(-1.0,1.0,-1.0);
                GX_Color1u32(col[1]);

                GX_Position3f32(-1.0,-1.0,1.0);
                GX_Color1u32(col[1]);
                GX_Position3f32(1.0,-1.0,1.0);
                GX_Color1u32(col[1]);
                GX_Position3f32(1.0,-1.0,-1.0);
                GX_Color1u32(col[1]);
                GX_Position3f32(-1.0,-1.0,-1.0);
                GX_Color1u32(col[1]);

                GX_Position3f32(-1.0,1.0,1.0);
                GX_Color1u32(col[2]);
                GX_Position3f32(-1.0,1.0,-1.0);
                GX_Color1u32(col[2]);
                GX_Position3f32(-1.0,-1.0,-1.0);
                GX_Color1u32(col[2]);
                GX_Position3f32(-1.0,-1.0,1.0);
                GX_Color1u32(col[2]);

                GX_Position3f32(1.0,1.0,1.0);
                GX_Color1u32(col[2]);
                GX_Position3f32(1.0,1.0,-1.0);
                GX_Color1u32(col[2]);
                GX_Position3f32(1.0,-1.0,-1.0);
                GX_Color1u32(col[2]);
                GX_Position3f32(1.0,-1.0,1.0);
                GX_Color1u32(col[2]);
            GX_End();
            
            // Animate and render
            a += 0.5;
            GRRLIB_Render();
        }

        GRRLIB_Exit();
    }

    return 0;
}