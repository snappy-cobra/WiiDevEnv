use alloc::vec;
use core::fmt;
use gamelib::game_state::changes::controls::{Controls, Direction, MotionControl, WiiMoteControl};
use gamelib::plot::PlotsHolder;
use micromath::F32Ext;
use num::ToPrimitive;
use ogc_rs::input::*;
use ogc_rs::prelude::*;
use ogc_rs::print;
use ogc_rs::println;

pub struct InputManager {
    wii_mote_states: [WiiMoteState; 4],
    plots_holder: PlotsHolder,
}

impl InputManager {
    pub fn new() -> Self {
        // Setup the wiimote
        Input::init(ControllerType::Wii);
        let controller_ports = [
            ControllerPort::One,
            ControllerPort::Two,
            ControllerPort::Three,
            ControllerPort::Four,
        ];
        let wii_mote_states = controller_ports.map(create_wii_mote);
        let plots_holder = PlotsHolder::new(false);
        Self {
            wii_mote_states,
            plots_holder,
        }
    }

    pub fn update(&mut self) -> Controls {
        Input::update(ControllerType::Wii);
        self.wii_mote_states
            .iter_mut()
            .for_each(|x| x.update_motion(&mut self.plots_holder));
        let wii_mote_controls = self
            .wii_mote_states
            .iter()
            .map(|x| x.to_wii_mote_control())
            .collect::<Vec<WiiMoteControl>>();
        // let a = wii_mote_controls.try_into<[WiiMoteControl;4]>();
        return Controls {
            wii_mote_control: wii_mote_controls,
        };
    }
}

fn create_wii_mote(controller_port: ControllerPort) -> WiiMoteState {
    let wii_mote = Input::new(ControllerType::Wii, controller_port);
    wii_mote
        .as_wpad()
        // .set_data_format(WPadDataFormat::ButtonsAccel); Maybe usefull if wiimote does not have high enough latency
        .set_data_format(WPadDataFormat::ButtonsAccelIR);
    // wii_mote.as_wpad().set_motion_plus(true);
    WiiMoteState::new(wii_mote)
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WiiMoteState {
    wii_mote: Input,
    motion: Option<Motion>,
    _prev_gforce: Vec<(f32, f32, f32)>,
}

impl WiiMoteState {
    pub fn new(input_wii_mote: Input) -> Self {
        WiiMoteState {
            wii_mote: input_wii_mote,
            motion: None,
            _prev_gforce: Vec::new(),
        }
    }

    pub fn update_motion(&mut self, plots_holder: &mut PlotsHolder) {
        const measurement_lenght: usize = 3;
        const min_neutral_lenght: usize = 3;
        const max_neutral_length: usize = 9;

        let cur_gforce = self.wii_mote.as_wpad().gforce();

        self._prev_gforce.push(cur_gforce);
        if self._prev_gforce.len() > max_neutral_length + measurement_lenght {
            self._prev_gforce.drain(..1);
        }

        if self._prev_gforce.len() < min_neutral_lenght + measurement_lenght {
            // Not enough measurements yet to do anything usefull
            return;
        }
        let (neutral_gforce_measurements, movement_gforce_measurements) = self
            ._prev_gforce
            .split_at(self._prev_gforce.len() - measurement_lenght);

        let neutral_gforce = find_average(neutral_gforce_measurements);
        let movement_gforce = find_average(movement_gforce_measurements);

        match self.motion {
            None => {
                self.motion =
                    Motion::create_if_needed(neutral_gforce, movement_gforce, plots_holder)
            }
            Some(ref mut motion) => {
                if motion.ended {
                    self.motion = None;
                    self._prev_gforce = Vec::new();
                } else {
                    motion.update(neutral_gforce, movement_gforce, plots_holder);
                }
            }
        }
    }

    fn to_wii_mote_control(&self) -> WiiMoteControl {
        let motion_control: Option<MotionControl>;
        match &self.motion {
            None => motion_control = None,
            Some(m) => motion_control = Some(m.to_motion_control()),
        }

        return WiiMoteControl {
            motion: motion_control,
            home_button_down: self.wii_mote.is_button_down(Button::Home),
            one_button_down: self.wii_mote.is_button_down(Button::One),
        };
    }
}

fn find_average(gforce_vec: &[(f32, f32, f32)]) -> (f32, f32, f32) {
    let mut x_sum: f32 = 0.0;
    let mut y_sum: f32 = 0.0;
    let mut z_sum: f32 = 0.0;
    for (x, y, z) in gforce_vec {
        x_sum += x;
        y_sum += y;
        z_sum += z;
    }
    match gforce_vec.len().to_f32() {
        Some(len) => (x_sum / len, y_sum / len, z_sum / len),
        None => (0.0, 0.0, 0.0),
    }
}

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
            if plots_holder.motion_plot {
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
            }
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
            if plots_holder.motion_plot {
                plots_holder.plots_to_logs()
            }
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
