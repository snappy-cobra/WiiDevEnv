include!("renderer/grrlib.rs");

mod inline;
pub use inline::*;

use hecs::*;
use crate::Vector3;

/**
 * Render the main cube.
 */
pub fn render_cube() {
    let col: [u32; 3] = [0xFFFFFFFF, 0xAAAAAAFF, 0x666666FF];
    unsafe {
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
    }
}

/**
 * Initialize the renderer
 */
pub fn init_render() {
    unsafe {
        GRRLIB_Init();
        GRRLIB_Settings.antialias = true;

        GRRLIB_SetBackgroundColour(0x00, 0x00, 0x00, 0xFF);
        GRRLIB_Camera3dSettings(
            0.0, 0.0, 13.0, 
            0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0
        );
    }
}

/**
 * Cleanup the renderer
 */
pub fn close_render() {
    unsafe {
        GRRLIB_Exit();
    }
}

/**
 * Render the scene
 */
pub fn render_world(world: &World) {
    let a = 0.0;
    let cubeZ = 0.0;
    unsafe {
        GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
        GRRLIB_ObjectView(
            0.0, 0.0, cubeZ, 
            a, a*2.0, a*3.0,
            1.0, 1.0, 1.0
        );
        render_cube();
        GRRLIB_Render();
    }

    // for (_id, (position, _velocity)) in  &mut world.query::<(&Vector3, &Vector3)>()
    // {
    //     unsafe {
    //         GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
    //         GRRLIB_ObjectView(
    //             0.0, 0.0, 0.0, 
    //             0.0, 0.0, 0.0,
    //             1.0, 1.0, 1.0
    //         );
    //         render_cube();
    //     }
    // }
    // unsafe {
    //     GRRLIB_Render();
    // }
}