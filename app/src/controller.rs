use crate::plot::PlotsHolder;
use alloc::vec;
use ogc_rs::prelude::*;

/// Currently only for plot controller movement, but will later contain gesture recognition functions
pub fn plot_wiimote_movement(plotHolder: &mut PlotsHolder, wii_mote: &Input) {
    if wii_mote.is_button_held(Button::B) {
        let gforce = wii_mote.as_wpad().raw().gforce;
        plotHolder.add_measurement(
            "gforce",
            vec!["x", "y", "z"],
            vec![gforce.x, gforce.y, gforce.z],
        );
        let accel = wii_mote.as_wpad().raw().accel;
        plotHolder.add_measurement(
            "accel",
            vec!["x", "y", "z"],
            vec![accel.x as f32, accel.y as f32, accel.z as f32],
        );
        let orient = wii_mote.as_wpad().raw().orient;
        plotHolder.add_measurement(
            "orient",
            vec!["roll", "pitch", "yaw"],
            vec![orient.roll, orient.pitch, orient.yaw],
        );
    }
    if wii_mote.is_button_up(Button::B) {
        // plot::create_plot("gforce", &gforce_plot_vec);
        plotHolder.plots_to_logs()
    }
}
