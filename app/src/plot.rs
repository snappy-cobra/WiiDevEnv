use alloc::format;
use alloc::vec::Vec;
// use ogc_rs::input::*;
use ogc_rs::prelude::*;

pub fn createPlot(title: &str, labels: &Vec<&str>, measurements: &Vec<Vec<f32>>) {
    startPlot();
    for measurement in measurements {
        printPlotMeasurement(title, labels, measurement);
    }
    stopPlot();
}

fn startPlot() {
    println!("START_PLOT");
}

fn printPlotMeasurement(title: &str, labels: &Vec<&str>, measurement: &Vec<f32>) {
    let mut formatted_measurement: Vec<String> = Vec::new();
    for (i, label) in labels.iter().enumerate() {
        let m = format!("{}={:.5}", label, measurement[i]);
        formatted_measurement.push(m);
    }
    println!("P_{}_{}", title, formatted_measurement.join(","));
}

fn stopPlot() {
    println!("STOP_PLOT");
}

pub struct Plot {
    title: String,
    labels: Vec<String>,
    measurements: Vec<Vec<f32>>,
}

impl Plot {
    pub fn new(title: &str, labels: Vec<&str>) -> Plot {
        let titleString = title.to_string();
        let labelsString: Vec<String> = labels.iter().map(|s| s.to_string()).collect();
        Plot {
            title: titleString,
            labels: labelsString,
            measurements: vec![],
        }
    }

    pub fn addMeasurement(&mut self, measurement: Vec<f32>) {
        self.measurements.push(measurement)
    }

    fn resetMeasurements(&mut self) {
        self.measurements = vec![];
    }

    pub fn plotToLogs(&mut self) {
        let title = self.title.as_str();
        let labels = self.labels.iter().map(|s| s.as_str()).collect();
        createPlot(title, &labels, &self.measurements);
        self.resetMeasurements();
    }
}

// pub struct PlotsHolder {
//     plots
// }
