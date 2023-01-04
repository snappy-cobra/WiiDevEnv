use crate::controller::plot_wiimote_movement;
use crate::plot::PlotsHolder;
use gamelib::Controls;
use micromath::F32Ext;
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
    prev_gforce: (f32, f32, f32),
}

impl WiiMote {
    pub fn new(input_wii_mote: Input) -> Self {
        let prev_gforce = input_wii_mote.as_wpad().gforce();
        WiiMote {
            input_wii_mote,
            motion: None,
            prev_gforce,
        }
    }

    pub fn update_motion(&mut self, plot_holder: &mut PlotsHolder) {
        let prev_gforce = self.prev_gforce;
        let cur_gforce = self.input_wii_mote.as_wpad().gforce();
        let correct_gforce = (
            cur_gforce.0 - prev_gforce.0,
            cur_gforce.1 - prev_gforce.1,
            cur_gforce.2 - prev_gforce.2,
        );
        // let total_gforce = (gforce.0.powi(2) + gforce.1.powi(2) + gforce.2.powi(2)).sqrt();
        let total_gforce =
            (correct_gforce.0.powi(2) + correct_gforce.1.powi(2) + correct_gforce.2.powi(2)).sqrt();
        // if total_gforce >= 3.0 {
        //     println!("movementsss: {}", total_gforce);
        // }
        if self.input_wii_mote.is_button_held(Button::A) {
            plot_holder.add_measurement(
                "gforce",
                vec!["x", "y", "z", "total"],
                vec![
                    correct_gforce.0,
                    correct_gforce.1,
                    correct_gforce.2,
                    total_gforce,
                ],
            );
        }
        if self.input_wii_mote.is_button_up(Button::A) {
            // plot::create_plot("gforce", &gforce_plot_vec);
            plot_holder.plots_to_logs()
        }

        match self.motion {
            None => self.motion = Motion::create_if_needed(&self.input_wii_mote),
            Some(ref mut motion) => {
                if motion.ended {
                    self.motion = None
                } else {
                    motion.update(&self.input_wii_mote);
                }
            }
        }
        self.prev_gforce = cur_gforce;
    }
}

pub struct Motion {
    direction: Direction,
    started: bool,
    ended: bool,
}

impl Motion {
    pub fn new() -> Self {
        Motion {
            direction: Direction::xp,
            started: true, // true only at the first iteration of the motion
            ended: false,  // true only at the last iteration of the motion
        }
    }

    pub fn create_if_needed(input_wii_mote: &Input) -> Option<Motion> {
        let gforce = input_wii_mote.as_wpad().raw().gforce;
        let total_gforce = (gforce.x.powi(2) + gforce.y.powi(2) + gforce.z.powi(2)).sqrt();

        return if total_gforce >= 3.0 {
            println!("Motion started: {}", total_gforce);
            let m = Motion::new();
            Some(m)
        } else {
            None
        };
    }

    pub fn update(&mut self, input_wii_mote: &Input) {
        self.started = false;
        // let gforce = ;
        let gforce = input_wii_mote.as_wpad().raw().gforce;
        let total_gforce = (gforce.x.powi(2) + gforce.y.powi(2) + gforce.z.powi(2)).sqrt();

        if total_gforce < 3.0 {
            self.ended = true;
            println!("Motion ended: {}", total_gforce);
        }
    }
}

pub enum Direction {
    xp,
    xn,
    yp,
    yn,
    zp,
    zn,
}
