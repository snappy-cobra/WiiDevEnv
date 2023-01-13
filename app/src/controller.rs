use crate::plot::PlotsHolder;
use alloc::vec;
use micromath::F32Ext;
use ogc_rs::prelude::*;

/// Currently only for plot controller movement, but will later contain gesture recognition functions
pub fn plot_wiimote_movement(plotHolder: &mut PlotsHolder, wii_mote: &Input) {
    let gforce = wii_mote.as_wpad().gforce();
    let total_gforce = (gforce.0.powi(2) + gforce.1.powi(2) + gforce.2.powi(2)).sqrt();
    // if total_gforce >= 3.0 {
    //     println!("movementsss: {}", total_gforce);
    // }
    if wii_mote.is_button_held(Button::B) {
        plotHolder.add_measurement(
            "gforce",
            vec!["x", "y", "z", "total"],
            vec![gforce.0, gforce.1, gforce.2, total_gforce],
        );
        // let accel = wii_mote.as_wpad().raw().accel;
        // plotHolder.add_measurement(
        //     "accel",
        //     vec!["x", "y", "z"],
        //     vec![accel.x as f32, accel.y as f32, accel.z as f32],
        // );
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
