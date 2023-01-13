use crate::controller::plot_wiimote_movement;
use crate::plot::PlotsHolder;
use core::fmt;
use gamelib::Controls;
use micromath::F32Ext;
use num::ToPrimitive;
use ogc_rs::input::*;
use ogc_rs::prelude::*;

pub struct InputManager {
    wii_mote: WiiMote, // Change later to a list to support multiple controllers.
    plots_holder: PlotsHolder,
}

impl InputManager {
    pub fn new() -> Self {
        // Setup the wiimote
        Input::init(ControllerType::Wii);
        let wii_mote = Input::new(ControllerType::Wii, ControllerPort::One);
        wii_mote
            .as_wpad()
            // .set_data_format(WPadDataFormat::ButtonsAccel); Maybe usefull if wiimote does not have high enough latency
            .set_data_format(WPadDataFormat::ButtonsAccelIR);
        // wii_mote.as_wpad().set_motion_plus(true);
        let cus_wii_mote = WiiMote::new(wii_mote);
        let plots_holder = PlotsHolder::new();
        Self {
            wii_mote: cus_wii_mote,
            plots_holder,
        }
    }

    pub fn update(&mut self) -> Controls {
        Input::update(ControllerType::Wii);
        self.wii_mote.update_motion(&mut self.plots_holder);
        plot_wiimote_movement(&mut self.plots_holder, &self.wii_mote.input_wii_mote);
        Controls {
            home_button_down: self.wii_mote.input_wii_mote.is_button_down(Button::Home),
            one_button_down: self.wii_mote.input_wii_mote.is_button_down(Button::One),
        }
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WiiMote {
    input_wii_mote: Input,
    motion: Option<Motion>,
    // TODO: Could be improved by using a rotating queue
    prev_gforce: Vec<(f32, f32, f32)>,
}

impl WiiMote {
    pub fn new(input_wii_mote: Input) -> Self {
        WiiMote {
            input_wii_mote,
            motion: None,
            prev_gforce: Vec::new(),
        }
    }

    pub fn update_motion(&mut self, plot_holder: &mut PlotsHolder) {
        let cur_gforce = self.input_wii_mote.as_wpad().gforce();
        let average_gforce = find_average(&self.prev_gforce);
        let corrected_gforce = (
            cur_gforce.0 - average_gforce.0,
            cur_gforce.1 - average_gforce.1,
            cur_gforce.2 - average_gforce.2,
        );

        let total_gforce =
            // (cur_gforce.0.powi(2) + cur_gforce.1.powi(2) + cur_gforce.2.powi(2)).sqrt();
            (corrected_gforce.0.powi(2) + corrected_gforce.1.powi(2) + corrected_gforce.2.powi(2)).sqrt();
        // (cur_gforce.0.powi(2) + (cur_gforce.2-1.0).powi(2)).sqrt();
        // // let total_gforce =
        // //     (correct_gforce.0.powi(2) + correct_gforce.1.powi(2) + correct_gforce.2.powi(2)).sqrt();
        // // if total_gforce >= 3.0 {
        // //     println!("movementsss: {}", total_gforce);
        // // }
        if self.input_wii_mote.is_button_held(Button::A) {
            plot_holder.add_measurement(
                "gforce_corrected",
                vec!["x", "y", "z", "total"],
                vec![
                    corrected_gforce.0,
                    corrected_gforce.1,
                    corrected_gforce.2,
                    total_gforce,
                ],
            );
        }
        if self.input_wii_mote.is_button_up(Button::A) {
            plot_holder.plots_to_logs()
        }
        match self.motion {
            None => self.motion = Motion::create_if_needed(total_gforce, cur_gforce),
            Some(ref mut motion) => {
                if motion.ended {
                    self.motion = None;
                    self.prev_gforce = Vec::new();
                } else {
                    motion.update(&self.input_wii_mote);
                }
            }
        }
        match self.motion {
            None => {
                self.prev_gforce.push(cur_gforce);
                if self.prev_gforce.len() > 9 {
                    self.prev_gforce.drain(..1);
                }
            }
            Some(ref mut motion) => 
        }
        if self.motion == None {

        }
    }
}

fn find_average(gforce_vec: &Vec<(f32, f32, f32)>) -> (f32, f32, f32) {
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
    direction: Direction,
    started: bool,
    ended: bool,
}

impl Motion {
    pub fn new(direction: Direction) -> Self {
        Motion {
            direction,
            started: true, // true only at the first iteration of the motion
            ended: false,  // true only at the last iteration of the motion
        }
    }

    pub fn create_if_needed(
        total_gforce: f32,
        corrected_gforce: (f32, f32, f32),
    ) -> Option<Motion> {
        return if total_gforce >= 3.0 {
            let dir = find_direction(corrected_gforce);
            println!(
                "Motion started: {:?} {:?} {}",
                dir, corrected_gforce, total_gforce
            );
            let m = Motion::new(dir);
            Some(m)
        } else {
            None
        };
    }

    pub fn update(&mut self, input_wii_mote: &Input) {
        self.started = false;
        // let gforce = ;
        let gforce = input_wii_mote.as_wpad().gforce();
        let total_gforce = (gforce.0.powi(2) + gforce.1.powi(2) + gforce.2.powi(2)).sqrt();

        if input_wii_mote.is_button_down(Button::Minus) {
            self.ended = true;
            println!("Motion ended");
        }
        // if total_gforce < 3.0 {
        //     self.ended = true;
        //     println!("Motion ended: {:?} {}", self.direction, total_gforce);
        // }
    }
}

#[derive(Debug)]
pub enum Direction {
    xp,
    xn,
    yp,
    yn,
    zp,
    zn,
}

pub fn find_direction(gforce: (f32, f32, f32)) -> Direction {
    // TODO: try to add gravity compensation on gforce measurement
    let x = gforce.0;
    let y = gforce.1;
    let z = gforce.2;
    let x_abs = x.abs();
    let y_abs = y.abs();
    let z_abs = z.abs();
    if x_abs > z_abs {
        // if x_abs > y_abs && x_abs > z_abs {
        return if x > 0.0 {
            Direction::xp
        } else {
            Direction::xn
        };
    // } else if y_abs > z_abs {
    //     return if y > 0.0 {
    //         Direction::yp
    //     } else {
    //         Direction::yn
    //     };
    } else {
        return if z > 0.0 {
            Direction::zp
        } else {
            Direction::zn
        };
    }
}
