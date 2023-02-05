use super::display_cache::DisplayCache;
use super::indexed_model::{BYTE_SIZE_POSITION, BYTE_SIZE_TEX_COORD};
use super::model_factory::ModelFactory;
use super::textured_model::TexturedModel;
use alloc::vec;
use gamelib::data_store::asset_name::AssetName;
use gamelib::data_store::textured_model_name::TexturedModelName;
use gamelib::game_state::changes::controls::Direction;
use gamelib::game_state::components::controller_assignment::FryAssignment;
use gamelib::game_state::components::game::Camera;
use gamelib::game_state::components::motion::Rotation;
use gamelib::game_state::components::physics::SphereCollider;
use gamelib::game_state::components::render::MeshInstance;
use gamelib::game_states::GameStateName;
use gamelib::{
    game_state::components::motion::Position, game_state::components::motion::Velocity,
    game_state::GameState, servers::renderer::RenderServer,
};

use grrustlib::*;
use hecs::*;
use libc::c_void;
use ogc_rs::prelude::Vec;
use ogc_rs::{print, println};
use physicslib::{
    Connection, Joint, TPE_Body, TPE_Joint, TPE_World, TPE_worldInit, Vec3, WorldWrapper,
};
use wavefront::{Obj, Vertex};

/// Representation of the graphics rendering subsystem of the device
///
/// As the device only has _one_ graphics chip which is exposed as a globally mutable state machine,
/// at most one Renderer should be constructed at any time.
///
/// Graphics setup happens as part of initialization,
/// and cleanup happens automatically on drop.
pub struct WiiRenderServer {
    model_factory: ModelFactory,
    display_cache: DisplayCache,
    world_wrapper: WorldWrapper,
    sim_step: usize,
}

impl WiiRenderServer {
    ///
    /// Create a new renderer.
    ///
    /// As part of this:
    /// - the graphics chip is initialized in the expected rendering mode.
    /// - The available models are constructed and indexed. (c.f. `ModelFactory`)
    pub fn new() -> Self {
        let mut world_wrapper = WorldWrapper::new();
        // world_wrapper.add_body(
        //     vec![Joint::new(Vec3(0.0, -500.0, 0.0), 1000.0)],
        //     vec![],
        //     100000.0,
        // );
        let res = Self {
            model_factory: ModelFactory::new(),
            display_cache: DisplayCache::new(),
            world_wrapper,
            sim_step: 0,
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
            GRRLIB_Camera3dSettings(0.0, 35.0, 10.0, 0.0, 1.0, 0.0, 10.0, 10.0, 0.0);
        }
    }

    /// Render a single entity
    fn set_camera(&mut self, camera: &Camera, pos: &Position) {
        unsafe {
            GRRLIB_Settings.antialias = true;

            GRRLIB_SetBackgroundColour(camera.r, camera.g, camera.b, 0xFF);
            GRRLIB_Camera3dSettings(
                pos.x,
                pos.y,
                pos.z,
                camera.up_x,
                camera.up_y,
                camera.up_z,
                camera.lookat_x,
                camera.lookat_y,
                camera.lookat_z,
            );
        }
    }

    /// Render a single entity
    fn render_entity(
        &mut self,
        model_name: &TexturedModelName,
        position: &Position,
        rotation: &Rotation,
    ) {
        unsafe {
            GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
            GRRLIB_ObjectView(
                position.x, position.y, position.z, rotation.x, rotation.y, rotation.z, 1.0, 1.0,
                1.0,
            );
            self.render_textured_model(model_name);
        }
    }

    /**
     * Renders the given model at whatever position was set previously using other calls into GRRLIB / GX.
     */
    fn render_textured_model(&mut self, model_name: &TexturedModelName) {
        let textured_model = self.model_factory.get_model(model_name).unwrap();
        textured_model.texture.set_active(true);
        Self::pass_textured_model_data(textured_model);
        Self::pass_textured_model_description();

        let display_list = self.display_cache.get_display_list(model_name);
        if !display_list.is_initialized() {
            display_list.open();
            Self::pass_textured_model_data_indices(textured_model);
            display_list.close();
        }
        display_list.set_active();
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
    fn pass_textured_model_data(textured_model: &TexturedModel) {
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
    fn pass_textured_model_data_indices(textured_model: &TexturedModel) {
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

impl Drop for WiiRenderServer {
    /// Cleanup the renderer
    fn drop(&mut self) {
        println!("Dropping Renderer");
        unsafe {
            GRRLIB_Exit();
        }
    }
}

/**
 * Implement the render state implementation for the game to use.
 */
impl RenderServer for WiiRenderServer {
    /*
     * Render all given meshes.
     * As part of this, refreshes the graphics buffer and wait for the next frame.
     */
    fn render_meshes(&mut self, meshes: Vec<(&MeshInstance, &Position, &Rotation)>) {
        for (mesh_instance, position, rotation) in meshes {
            self.render_entity(&mesh_instance.model_name, position, rotation);
        }
    }

    fn render_debug(&mut self, data: Vec<(&Position, &SphereCollider, &Rotation)>) {
        for (pos, collider, rot) in data {
            self.render_entity(&TexturedModelName::Cube, pos, rot);
        }
    }

    fn update_camera(&mut self, pos: &Position, camera: &Camera) {
        self.set_camera(camera, pos)
    }

    /**
     * Render a new frame.
     */
    fn render_frame(&mut self) {
        unsafe {
            GRRLIB_Render();
        }
    }

    fn register_collider(&mut self, colliders: &mut Vec<&mut SphereCollider>) {
        // TODO: make this not happen every iteration
        for collider in colliders.iter_mut() {
            if !collider.has_been_registered {
                /// POOTAATOO
                let joints = vec![
                    Joint::new(Vec3(0.0, 19.7, 0.0), 0.4),
                    Joint::new(Vec3(0.0, 20.0, 0.0), 1.0),
                    Joint::new(Vec3(0.0, 20.3, 0.0), 0.5),
                ];
                let connections = vec![
                    Connection::new(0, 1, 0.5),
                    Connection::new(0, 2, 0.5),
                    Connection::new(1, 2, 0.5),
                ];

                collider.body_index = self.world_wrapper.add_body(joints, connections, 10.0);
                let body = self.world_wrapper.get_body(collider.body_index);
                body.move_by(Vec3(
                    collider.body_index as f32 * 0.5,
                    collider.body_index as f32 * 0.5,
                    collider.body_index as f32 * 0.5,
                ));
                collider.has_been_registered = true;
            }
        }
    }

    fn world_step(&mut self) {
        const step_scale_velocity: usize = 1;
        for body in self.world_wrapper.bodies_iter() {
            let plate_height = 0.0;
            let pos = body.center_of_mass();
            let distance_from_center = pos.0 * pos.0 + pos.2 * pos.2;
            let max_dis_from_center = 11.0;
            if distance_from_center < max_dis_from_center * max_dis_from_center
                && body.center_of_mass().1 < plate_height
                && body.center_of_mass().1 > plate_height - 2.0
            {
                let mut pos = body.center_of_mass();
                pos.1 = plate_height;
                // TODO Add friction. using velocity
                body.move_to(pos);
                // println!("{:?}", self.world_wrapper.sim_step);
                // Only scale velocity every n sim_steps
                if self.sim_step >= step_scale_velocity {
                    body.scale_velocity(0.99);
                }
            } else {
                body.apply_gravity(1.0 / 100.0);
                if body.center_of_mass().1 < plate_height - 20.0 {
                    body.move_to(Vec3 {
                        0: 0.0,
                        1: 10.0,
                        2: 0.0,
                    });
                    body.scale_velocity(0.0);
                }
            }
        }

        if self.sim_step >= step_scale_velocity {
            self.sim_step = 0;
        } else {
            self.sim_step += 1;
        }
        self.world_wrapper.step();
    }

    fn physics_to_position(
        &mut self,
        objs: &mut Vec<(&mut SphereCollider, &mut Position, &mut Rotation)>,
    ) {
        for (col, pos, rot) in objs.iter_mut() {
            let body = self.world_wrapper.get_body(col.body_index);
            let center_of_mass = body.center_of_mass();
            pos.x = center_of_mass.0;
            pos.y = center_of_mass.1;
            pos.z = center_of_mass.2;

            let rotation = body.rotation();

            rot.x = rotation.0;
            rot.y = rotation.1;
            rot.z = rotation.2;
        }
    }

    fn apply_movement(&mut self, obj: &SphereCollider, dir: Direction) {
        let body = self.world_wrapper.get_body(obj.body_index);
        let move_magnitude = 0.1;
        let move_help_jump_magnitude = 0.1;
        let jump_magnitude = 0.3;
        let jump_down_magnitude = 0.2;
        let rotation = match dir {
            Direction::Xp => Vec3 {
                0: -move_magnitude,
                1: move_help_jump_magnitude,
                2: 0.0,
            },
            Direction::Xn => Vec3 {
                0: move_magnitude,
                1: move_help_jump_magnitude,
                2: 0.0,
            },
            Direction::Yp | Direction::Zp => Vec3 {
                0: 0.0,
                1: move_help_jump_magnitude,
                2: move_magnitude,
            },
            Direction::Yn | Direction::Zn => Vec3 {
                0: 0.0,
                1: move_help_jump_magnitude,
                2: -move_magnitude,
            },
            // Direction::Zp => Vec3 {
            //     0: 0.0,
            //     1: jump_magnitude,
            //     2: 0.0,
            // },
            // Direction::Zn => Vec3 {
            //     0: 0.0,
            //     1: -jump_down_magnitude,
            //     2: 0.0,
            // },
        };
        body.accelerate(rotation);
    }

    fn reset_world(&mut self) {
        self.world_wrapper = WorldWrapper::new();
    }

    fn teleport_potato(
        &mut self,
        objs: &mut Vec<(&mut SphereCollider, &mut Position, &mut Rotation)>,
    ) {
        for (potato, _a, _b) in objs {
            let body = self.world_wrapper.get_body(potato.body_index);
            if body.center_of_mass().1 < -20.0 {
                body.move_to(Vec3 {
                    0: 0.0,
                    1: 10.0,
                    2: 0.0,
                });
                body.scale_velocity(0.0);
            }
        }
    }

    fn fry_pan_score_increase(
        &mut self,
        position: &mut Position,
        fry_assignment: &mut FryAssignment,
        potatoes: &Vec<usize>,
    ) {
        let fry_pan_radius = 8.0;
        for potato_body_index in potatoes {
            let body = self.world_wrapper.get_body(*potato_body_index);
            if body.center_of_mass().1 < -20.0 {
                println!(
                    "potato x:{:?} z:{:?}",
                    body.center_of_mass().0,
                    body.center_of_mass().2
                );
                println!("x:{:?} z:{:?}", position.x, position.z);
                let x_dif = body.center_of_mass().0 - position.x;
                let z_dif = body.center_of_mass().2 - position.z;
                // let distance = x_dif * x_dif + z_dif * z_dif
                // println!(distance);
                if x_dif * x_dif + z_dif * z_dif < fry_pan_radius * fry_pan_radius {
                    fry_assignment.score += 1;
                    println!("SCOREEE");
                }
            }
        }
    }
}
