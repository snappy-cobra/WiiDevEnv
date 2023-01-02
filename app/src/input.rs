use crate::controller::plot_wiimote_movement;
use crate::plot::PlotsHolder;
use gamelib::Controls;
use ogc_rs::prelude::{Button, ControllerPort, ControllerType, Input, WPadDataFormat};

pub struct InputManager {
    wii_mote: Input,
    plots_holder: PlotsHolder,
}

impl InputManager {
    pub fn new() -> Self {
        // Setup the wiimote
        Input::init(ControllerType::Wii);
        let wii_mote = Input::new(ControllerType::Wii, ControllerPort::One);
        wii_mote
            .as_wpad()
            .set_data_format(WPadDataFormat::ButtonsAccelIR);
        let plots_holder = PlotsHolder::new();
        Self {
            wii_mote,
            plots_holder,
        }
    }

    pub fn update(&mut self) -> Controls {
        Input::update(ControllerType::Wii);
        plot_wiimote_movement(&mut self.plots_holder, &self.wii_mote);
        Controls {
            home_button_down: self.wii_mote.is_button_down(Button::Home),
            one_button_down: self.wii_mote.is_button_down(Button::One),
        }
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}
