use core::fmt;
use gamelib::game_state::changes::controls::Controls;
use gamelib::game_state::changes::motion::Motion;
use gamelib::plot::PlotsHolder;
use libc::fprintf;
use num::ToPrimitive;
use ogc_rs::input::*;
use ogc_rs::prelude::*;

pub struct InputManager {
    wii_mote_states: [WiiMoteState; 4],
    plots_holder: PlotsHolder,
}

impl InputManager {
    pub fn new() -> Self {
        // Setup the wiimote
        Input::init(ControllerType::Wii);
        // let mut wii_mote_states: [WiiMoteState; 4] = ;
        // let controller_ports = [
        //     ControllerPort::One,
        //     ControllerPort::Two,
        //     ControllerPort::Three,
        //     ControllerPort::Four,
        // ];
        let wii_mote_states = [
            create_wii_mote(ControllerPort::One),
            create_wii_mote(ControllerPort::Two),
            create_wii_mote(ControllerPort::Three),
            create_wii_mote(ControllerPort::Four),
        ];
        let plots_holder = PlotsHolder::new();
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
        return Controls {
            home_button_down: self.wii_mote_states[0]
                .wii_mote
                .is_button_down(Button::Home),
            one_button_down: self.wii_mote_states[0].wii_mote.is_button_down(Button::One),
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
    return WiiMoteState::new(wii_mote);
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
