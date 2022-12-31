use ogc_rs::prelude::{Button, ControllerPort, ControllerType, Input, WPadDataFormat};
use rust_wii_lib::Controls;

pub struct InputManager {
    wii_mote: Input,
}

impl InputManager {
    pub fn new() -> Self {
        // Setup the wiimote
        Input::init(ControllerType::Wii);
        let wii_mote = Input::new(ControllerType::Wii, ControllerPort::One);
        wii_mote
            .as_wpad()
            .set_data_format(WPadDataFormat::ButtonsAccelIR);
        Self { wii_mote }
    }

    pub fn update(&mut self) -> Controls {
        Input::update(ControllerType::Wii);
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
