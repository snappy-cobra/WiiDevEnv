pub mod indexed_model;
pub mod texture;
pub mod textured_model;
pub mod model_factory;

use crate::raw_data_store::AssetName;
use model_factory::ModelFactory;
use textured_model::TexturedModel;
use hecs::*;
use ogc_rs::{print, println};
use rust_wii_lib::{Position, Velocity};
use wavefront::{Obj, Vertex};
use libc::c_void;
use ogc_rs::prelude::Vec;
use grrustlib::*;

use self::indexed_model::{BYTE_SIZE_POSITION, BYTE_SIZE_TEX_COORD};

/// Representation of the graphics rendering subsystem of the device
///
/// As the device only has _one_ graphics chip which is exposed as a globally mutable state machine,
/// at most one Renderer should be constructed at any time.
///
/// Graphics setup happens as part of initialization,
/// and cleanup happens automatically on drop.
pub struct Renderer {
    model_factory: ModelFactory,
}

impl Renderer {
    ///
    /// Create a new renderer.
    ///
    /// As part of this:
    /// - the graphics chip is initialized in the expected rendering mode.
    /// - The available models are constructed and indexed. (c.f. `ModelFactory`)
    pub fn new() -> Renderer {
        let res = Renderer {
            model_factory: ModelFactory::new(),
        };
        res.init_render();
        res
    }

    /**
     * Initialize the renderer, which means GRRLIB and loading all models.
     */
    fn init_render(&self) {
        unsafe {
            GRRLIB_Init();
            GRRLIB_Settings.antialias = true;

            GRRLIB_SetBackgroundColour(0x00, 0x00, 0x00, 0xFF);
            GRRLIB_Camera3dSettings(0.0, 0.0, 13.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        }
    }

    /**
     * Render the entire scene.
     * As part of this, refreshes the graphics buffer and wait for the next frame.
     */
    pub fn render_world(&self, world: &World) {
        let model = self.model_factory.get_model(&AssetName::Suzanne).unwrap();
        Self::pass_textured_model_data(model);
        for (entity, (position, _velocity)) in &mut world.query::<(&Position, &Velocity)>() {
            self.render_entity(model, entity, position);
        }
        self.redraw_world();
    }

    /// Render a single entity
    fn render_entity(&self, model: &TexturedModel, _entity: Entity, position: &Position) {
        unsafe {
            GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
            GRRLIB_ObjectView(
                position.x, position.y, position.z, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
            );
            Self::render_textured_model(model);
        }
    }

    /// Refreshes the visible graphics;
    /// Usually called as part of `render_world`
    /// but separately exposed for easier testing.
    pub fn redraw_world(&self) {
        unsafe {
            GRRLIB_Render();
        }
    }

    /**
     * Renders the given model at whatever position was set previously using other calls into GRRLIB / GX.
     */
    fn render_textured_model(textured_model: & TexturedModel) {
        textured_model.texture.set_active(true);
        Self::pass_textured_model_description();
        Self::pass_textured_model_data_indices(textured_model);
    }

    /**
     * Describe the data format we push to the GPU as indexed data.
     */
    fn pass_textured_model_description() {
        unsafe {
            GX_SetVtxDesc(GX_VA_POS as u8, GX_INDEX16 as u8);
            GX_SetVtxDesc(GX_VA_CLR0 as u8, GX_DIRECT as u8);
            GX_SetVtxDesc(GX_VA_TEX0 as u8, GX_INDEX16 as u8);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_POS, GX_POS_XYZ, GX_F32, 0);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_TEX0, GX_TEX_ST, GX_F32, 0);
        }
    }

    /**
     * Sets pointers to the textured model data for the GPU to access.
     *
     * ## Safety
     * We call GX_SetArray which takes a pointer into the vertices of the model as '*void *' (C syntax) AKA '*mut c_void' (Rust syntax).
     * By checking the implementation of GX_SetArray it is clear that this signature is wrong; the argument is only used for reading and not mutated.
     * In other words: The argument is treated as if it were a 'const *void' (C syntax) AKA '*const c_void' (Rust syntax).
     * As such, it is OK to turn the immutable reference into a mutable pointer.
     */
    fn pass_textured_model_data(textured_model: & TexturedModel) {
        let positions_ptr = textured_model.model.positions.as_ptr().cast_mut() as *mut c_void;
        let tex_coord_ptr = textured_model.model.tex_coords.as_ptr().cast_mut() as *mut c_void;
        unsafe {
            GX_SetArray(GX_VA_POS, positions_ptr, BYTE_SIZE_POSITION as u8);
            GX_SetArray(GX_VA_TEX0, tex_coord_ptr, BYTE_SIZE_TEX_COORD as u8);
        }
    }

    /**
     * Iterate over the index arrays and set them in direct mode for the GPU to use.
     * Expects data to be described and passed before being called.
     */
    fn pass_textured_model_data_indices(textured_model: & TexturedModel) {
        unsafe {
            // Provide all the indices (wii really wants this in direct mode it seems)
            GX_Begin(
                GX_TRIANGLES as u8,
                GX_VTXFMT0 as u8,
                textured_model.model.position_indices.len() as u16,
            );
            let vertex_count = textured_model.model.position_indices.len();
            let position_indices = textured_model.model.position_indices.to_vec();
            let tex_coord_indices = textured_model.model.tex_coord_indices.to_vec();
            for index in 0..vertex_count {
                GX_Position1x16(position_indices[index]);
                GX_Color1u32(0xFFFFFFFF);
                GX_TexCoord1x16(tex_coord_indices[index]);
            }
            GX_End();
        }
    }
}

impl Drop for Renderer {
    /// Cleanup the renderer
    fn drop(&mut self) {
        println!("Dropping Renderer");
        unsafe {
            GRRLIB_Exit();
        }
    }
}
