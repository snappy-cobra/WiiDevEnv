include!("renderer/grrlib.rs");

mod inline;
pub use inline::*;

mod model_factory;
pub use model_factory::{ModelFactory};

use hecs::*;
use ogc_rs::print;
use wavefront::Obj;
use crate::{Position, Velocity};

/**
 * Data structure for the renderer.
 */
pub struct Renderer {
    model_factory : ModelFactory<'static>
}

/**
 * Renderer implementation: provides the main interface for rendering stuff in the game.
 */
impl Renderer {
    /**
     * Create a new renderer.
     */
    pub fn new() -> Renderer {
        Renderer {
            model_factory: ModelFactory::new()
        }
    }

    /**
     * Allows for rendering the given object.
     */
    fn render_mesh(object : & Obj) {
        let col: [u32; 3] = [0xFFFFFFFF, 0xAAAAAAFF, 0x666666FF];
        unsafe {
            let vertex_count = (object.triangles().count() * 3) as u16;
            GX_Begin(GX_TRIANGLES as u8, GX_VTXFMT0 as u8, vertex_count);
            for triangle in object.triangles() {
                for vertex in triangle {
                    let position = vertex.position();
                    GX_Position3f32(position[0], position[1], position[2]);
                    GX_Color1u32(col[0]);
                }
            }
            GX_End();
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
     * Initialize the renderer, which means GRRLIB and loading all models.
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
        let model = self.model_factory.get_model("Suzanne").unwrap();
        for (_id, (position, _velocity)) in  &mut world.query::<(&Position, &Velocity)>()
        {
            unsafe {
                GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
                GRRLIB_ObjectView(
                    position.x, position.y, position.z, 
                    0.0, 0.0, 0.0,
                    1.0, 1.0, 1.0
                );
                Self::render_mesh(& model);
            }
        }
        unsafe {
            GRRLIB_Render();
        }
    }   
}