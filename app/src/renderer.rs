#![allow(clippy::all)]
include!("renderer/grrlib.rs");

mod inline;

pub use inline::*;

pub mod indexed_model;
pub mod texture;
pub mod textured_model;
pub mod model_factory;

use crate::raw_data_store::AssetName;
use model_factory::ModelFactory;
use textured_model::TexturedModel;
use crate::{Position, Velocity};
use hecs::*;
use ogc_rs::{print, println};
use wavefront::{Obj, Vertex};
use libc::c_void;
use ogc_rs::prelude::Vec;

use self::indexed_model::{BYTE_SIZE_POSITION, BYTE_SIZE_TEX_COORD};

/**
 * Data structure for the renderer.
 */
pub struct Renderer {
    model_factory: ModelFactory,
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
            model_factory: ModelFactory::new(),
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
            GRRLIB_Camera3dSettings(0.0, 0.0, 13.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
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
        let mut model = self.model_factory.get_model(AssetName::Cube).unwrap();
        for (_id, (position, _velocity)) in &mut world.query::<(&Position, &Velocity)>() {
            unsafe {
                GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
                GRRLIB_ObjectView(
                    position.x, position.y, position.z, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
                );
                Self::render_textured_model(&mut model);
            }
        }
        unsafe {
            GRRLIB_Render();
        }
    }

    /**
     * Allows for rendering the given object.
     */
    fn render_textured_model(textured_model: &mut TexturedModel) {
        unsafe {
            // Set the repeating texture to active.
            textured_model.texture.set_active(true);

            // Describe the data as indexed
            GX_SetVtxDesc(GX_VA_POS as u8, GX_INDEX16 as u8);
            GX_SetVtxDesc(GX_VA_CLR0 as u8, GX_DIRECT as u8);
            GX_SetVtxDesc(GX_VA_TEX0 as u8, GX_INDEX16 as u8);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_POS, GX_POS_XYZ, GX_F32, 0);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_TEX0, GX_TEX_ST, GX_F32, 0);

            // Pass the data to the GPU
            GX_SetArray(
                GX_VA_POS,
                textured_model.model.positions.as_mut_ptr() as *mut c_void,
                BYTE_SIZE_POSITION,
            );
            GX_SetArray(
                GX_VA_TEX0,
                textured_model.model.tex_coords.as_mut_ptr() as *mut c_void,
                BYTE_SIZE_TEX_COORD,
            );
            
            // Provide all the indices (wii really wants this in direct mode it seems)
            GX_Begin(
                GX_TRIANGLES as u8,
                GX_VTXFMT0 as u8,
                textured_model.model.position_indices.len() as u16,
            );
            let num_verts = textured_model.model.position_indices.len();
            let position_indices = textured_model.model.position_indices.to_vec();
            let tex_coord_indices = textured_model.model.tex_coord_indices.to_vec();
            for index in 0..num_verts {
                GX_Position1x16(position_indices[index]);
                GX_Color1u32(0xFFFFFFFF);
                GX_TexCoord1x16(tex_coord_indices[index]);
            }
            GX_End();
        }
    }
}
