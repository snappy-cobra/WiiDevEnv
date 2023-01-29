use alloc::vec;
use ogc_rs::println;
use ogc_rs::print;
use crate::plot::PlotsHolder;
use micromath::F32Ext;
use crate::game_state::changes::controls::MotionControl;

pub struct Motion {
    pub direction: Direction,
    pub started: bool,
    pub ended: bool,
    _minimal_steps: usize,
}

impl Motion {
    pub fn new(direction: Direction) -> Self {
        Motion {
            direction,
            started: true, // true only at the first iteration of the motion
            ended: false,  // true only at the last iteration of the motion
            _minimal_steps: 7,
        }
    }

    pub fn create_if_needed(
        neutral_gforce: (f32, f32, f32),
        movement_gforce: (f32, f32, f32),
        plots_holder: &mut PlotsHolder,
    ) -> Option<Motion> {
        let (total_gforce, corrected_gforce) = process_gforce(neutral_gforce, movement_gforce);
        return if total_gforce >= 1.75 {
            let dir = find_direction(corrected_gforce);
            println!(
                "Motion started: {:?} {:?} {}",
                dir, corrected_gforce, total_gforce
            );
            plots_holder.add_measurement(
                "movement",
                vec!["x", "y", "z", "total"],
                vec![
                    corrected_gforce.0,
                    corrected_gforce.1,
                    corrected_gforce.2,
                    total_gforce,
                ],
            );
            let m = Motion::new(dir);
            Some(m)
        } else {
            None
        };
    }

    pub fn update(
        &mut self,
        neutral_gforce: (f32, f32, f32),
        movement_gforce: (f32, f32, f32),
        plots_holder: &mut PlotsHolder,
    ) {
        self.started = false;
        let (total_gforce, corrected_gforce) = process_gforce(neutral_gforce, movement_gforce);
        plots_holder.add_measurement(
            "movement",
            vec!["x", "y", "z", "total"],
            vec![
                corrected_gforce.0,
                corrected_gforce.1,
                corrected_gforce.2,
                total_gforce,
            ],
        );
        if self._minimal_steps > 0 {
            self._minimal_steps -= 1;
            return;
        }
        if total_gforce < 1.0 {
            self.ended = true;
            println!("Motion ended: {:?} {}", self.direction, total_gforce);
            plots_holder.plots_to_logs()
        }
    }

    pub fn to_motion_control(&self) -> MotionControl {
        MotionControl {
            direction: self.direction,
            started: self.started,
            ended: self.ended,
        }
    }

}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Xp,
    Xn,
    Yp,
    Yn,
    Zp,
    Zn,
}

fn process_gforce(
    neutral_gforce: (f32, f32, f32),
    movement_gforce: (f32, f32, f32),
) -> (f32, (f32, f32, f32)) {
    let neutral_factor: f32 = 0.8;
    let corrected_gforce = (
        movement_gforce.0 - neutral_gforce.0 * neutral_factor,
        movement_gforce.1 - neutral_gforce.1 * neutral_factor,
        movement_gforce.2 - neutral_gforce.2 * neutral_factor,
    );

    let total_gforce =
        (corrected_gforce.0.powi(2) + corrected_gforce.1.powi(2) + corrected_gforce.2.powi(2))
            .sqrt();
    // (corrected_gforce.0.powi(2) + corrected_gforce.2.powi(2)).sqrt();
    return (total_gforce, corrected_gforce);
}

fn find_direction(gforce: (f32, f32, f32)) -> Direction {
    let x = gforce.0;
    let y = gforce.1;
    let z = gforce.2;
    let x_abs = x.abs();
    let y_abs = y.abs();
    let z_abs = z.abs();
    // if x_abs > z_abs {
    if x_abs > y_abs && x_abs > z_abs {
        return if x > 0.0 {
            Direction::Xp
        } else {
            Direction::Xn
        };
    } else if y_abs > z_abs {
        return if y > 0.0 {
            Direction::Yp
        } else {
            Direction::Yn
        };
    } else {
        return if z > 0.0 {
            Direction::Zp
        } else {
            Direction::Zn
        };
    }
}