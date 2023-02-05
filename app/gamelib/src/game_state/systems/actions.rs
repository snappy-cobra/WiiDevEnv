use ogc_rs::println;
use ogc_rs::print;
use crate::game_state::GameState;
use crate::game_state::components::motion::Velocity;
use crate::game_state::components::game::*;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use crate::game_state::changes::controls::Direction;
use crate::game_state::components::physics::SphereCollider;
use crate::game_state::systems::system_name::SystemName::BounceBounds;
use crate::game_states::GameStateName;


/**
 * Stop the game from running if the home buttons is pressed.
 */
pub fn system_exit_action(state: &mut GameState) {
    if state.is_running {
        state.is_running = !state.changes.controls.wii_mote_controls[0].home_button_down;
    }
}

/**
 * Sets all velocities to 0 if 'one' is pressed.
 */
pub fn system_stop_action(state: &mut GameState) {
    for (_id, (velocity, controller_id)) in state.world.query_mut::<(&mut Velocity, & ControllerAssignment)>() {
        let controller_state = state.changes.controls.get_wii_mote_control(controller_id);
        if controller_state.one_button_down {
            velocity.x = 0.0;
            velocity.y = 0.0;
            velocity.z = 0.0;
        }
    }
}

/**
 * Checks if a motion has been made with controller One if this is the cases all velocities are increased in that direction.
 */
pub fn system_shake_action(state: &mut GameState) {
    for (_id, (velocity, controller_id)) in state.world.query_mut::<(&mut Velocity, & ControllerAssignment)>() {
        let controller_state = state.changes.controls.get_wii_mote_control(controller_id);
        match &controller_state.motion {
            None => (),
            Some(motion) => {
                if motion.started {
                    let mut small_rng = SmallRng::seed_from_u64(10u64);
                    let c = small_rng.next_u32() as f32 / u32::MAX as f32 * 0.5 - 0.1;
                    match motion.direction {
                        Direction::Zp => velocity.y += c,
                        Direction::Zn => velocity.y -= c,
                        Direction::Xp => velocity.x -= c,
                        Direction::Xn => velocity.x += c,
                        Direction::Yp => velocity.z += c,
                        Direction::Yn => velocity.z -= c,
                    };
                }
            }
        }
    }
}

pub fn system_control_potato(state: &mut GameState) {
    for (_id, (obj, controller_id)) in state.world.query_mut::<(&mut SphereCollider, & ControllerAssignment)>() {
        // let controller_state = state.changes.controls.get_wii_mote_control(controller_id);
        for i in 0..3 {
            let player = ControllerAssignment { id: i };
            match &state.changes.controls.get_wii_mote_control(&player).motion {
                None => (),
                Some(motion) => {
                    if motion.started {
                        let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
                        server_provider.render_server.apply_movement(obj, motion.direction)
                    }
                }
            }
        }
    }
}

pub fn system_reset_level(state: &mut GameState) {
    for wii_mote_control in state.changes.controls.wii_mote_controls.iter() {
        if wii_mote_control.one_button_down {
            println!("RESETTTTT");
            let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
            server_provider.render_server.reset_world();
            state.next_state = Some(GameStateName::BouncingCubes);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_state::GameState;

    #[test]
    fn test_exit() {
        // Doing nothing should not exit the game.
        let mut state = GameState::new();
        super::system_exit_action(&mut state);
        assert_eq!(state.is_running, true);

        // Pressing the home button should exit the game.
        state.changes.controls.wii_mote_controls[0].home_button_down = true;
        assert_eq!(state.is_running, true);
        super::system_exit_action(&mut state);
        assert_eq!(state.is_running, false);
    }
}