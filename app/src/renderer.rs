#![allow(clippy::all)]
include!("renderer/grrlib.rs");

mod inline;

pub use inline::*;

mod indexed_model;
mod model_factory;
use crate::raw_data_store::ModelName;
use indexed_model::IndexedModel;
use model_factory::ModelFactory;

// use crate::{Position, Velocity};
use hecs::*;
use ogc_rs::{print, println};
use rust_wii_lib::{Position, Velocity};
use wavefront::{Obj, Vertex};

use libc::c_void;
use ogc_rs::prelude::Vec;

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
        let model = self.model_factory.get_model(ModelName::Suzanne).unwrap();
        for (entity, (position, _velocity)) in &mut world.query::<(&Position, &Velocity)>() {
            self.render_entity(model, entity, position);
        }
        self.redraw_world();
    }

    /// Render a single entity
    fn render_entity(&self, model: &IndexedModel, _entity: Entity, position: &Position) {
        unsafe {
            GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
            GRRLIB_ObjectView(
                position.x, position.y, position.z, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
            );
            Self::render_mesh(model);
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
     *
     * ## Safety
     * We call GX_SetArray which takes a pointer into the vertices of the model as '*void *' (C syntax) AKA '*mut c_void' (Rust syntax).
     * By cheking the implementation of GX_SetArray it is clear that this signature is wrong; the argument is only used for reading and not mutated.
     * In other words: The argument is treated as if it were a 'const *void' (C syntax) AKA '*const c_void' (Rust syntax).
     * As such, it is OK to turn the immutable reference into a mutable pointer.
     */
    fn render_mesh(model: &IndexedModel) {
        let vertices_ptr = model.vertices.as_ptr().cast_mut() as *mut c_void;
        unsafe {
            GX_SetArray(GX_VA_POS, vertices_ptr, (4 * 3) as u8);
            GX_SetVtxDesc(GX_VA_POS as u8, GX_INDEX16 as u8);
            GX_SetVtxDesc(GX_VA_CLR0 as u8, GX_DIRECT as u8);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_POS, GX_POS_XYZ, GX_F32, 0);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_CLR0, GX_CLR_RGB, GX_F32, 0);

            GX_Begin(
                GX_TRIANGLES as u8,
                GX_VTXFMT0 as u8,
                model.indices.len() as u16,
            );
            let indices_copy = model.indices.to_vec();
            for index in indices_copy {
                GX_Position1x16(index);
                GX_Color3f32(1.0, 1.0, 1.0);
            }
            GX_End();
        }
    }
}

impl Drop for Renderer {
    /// Cleanup the renderer
    fn drop(&mut self) {
        println!("Dropping Renderer");
        // unsafe {
        //     GRRLIB_Exit();
        // }
    }
}
