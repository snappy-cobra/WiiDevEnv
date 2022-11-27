include!("renderer/grrlib.rs");

mod inline;
pub use inline::*;

mod model_factory;
pub use model_factory::{ModelFactory};

use hecs::*;
use ogc_rs::print;
use crate::{Position, Velocity};

pub struct Renderer {
    model_factory : ModelFactory<'static>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            model_factory: ModelFactory::new()
        }
    }

    /**
     * Render the main cube.
     */
    pub fn render_cube(&mut self) {
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
    pub fn init_render(&mut self) {
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
        self.model_factory.load_models();
    }

    /**
     * Cleanup the renderer
     */
    pub fn close_render(&mut self) {
        unsafe {
            GRRLIB_Exit();
        }
    }

    /**
     * Render the scene
     */
    pub fn render_world(&mut self, world: &World) {
        for (_id, (position, _velocity)) in  &mut world.query::<(&Position, &Velocity)>()
        {
            unsafe {
                GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
                GRRLIB_ObjectView(
                    position.x, position.y, position.z, 
                    0.0, 0.0, 0.0,
                    1.0, 1.0, 1.0
                );
                self.render_cube();
            }
        }
        unsafe {
            GRRLIB_Render();
        }
    }   
}