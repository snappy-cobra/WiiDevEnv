use alloc::format;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use crate::alloc::string::ToString;
use hashbrown::HashMap;
use ogc_rs::println;
use ogc_rs::print;


/// Allow for easy creation of plots to the logs.
/// these logs can than be parsed using the plot_logs.py which will create the actual plots.
/// plot_logs.py can be found in the python_support folder.
pub fn create_plot(title: &str, labels: &Vec<&str>, measurements: &Vec<Vec<f32>>) {
    for measurement in measurements {
        print_plot_measurement(title, labels, measurement);
    }
}

fn start_plot_group() {
    println!("START_PLOTS");
}

fn stop_plot_group() {
    println!("STOP_PLOTS");
}

fn print_plot_measurement(title: &str, labels: &[&str], measurement: &[f32]) {
    let mut formatted_measurement: Vec<String> = Vec::new();
    for (i, label) in labels.iter().enumerate() {
        let m = format!("{}={:.5}", label, measurement[i]);
        formatted_measurement.push(m);
    }
    println!("P_{}_{}", title, formatted_measurement.join(","));
}

/// Stores all the information needed for a single plot
/// These measurements increase over time, and are reset when a plot is "logged"
pub struct Plot {
    title: String,
    labels: Vec<String>,
    measurements: Vec<Vec<f32>>,
}

impl Plot {
    pub fn new(title: &str, labels: Vec<&str>) -> Plot {
        let title_string = title.to_string();
        let labels_string: Vec<String> = labels.iter().map(|s| s.to_string()).collect();
        Plot {
            title: title_string,
            labels: labels_string,
            measurements: vec![],
        }
    }

    pub fn add_measurement(&mut self, measurement: Vec<f32>) {
        self.measurements.push(measurement)
    }

    fn reset_measurements(&mut self) {
        self.measurements = vec![];
    }

    pub fn to_logs(&mut self) {
        if self.measurements.is_empty() {
            return;
        }
        let title = self.title.as_str();
        let labels = self.labels.iter().map(|s| s.as_str()).collect();
        create_plot(title, &labels, &self.measurements);
        self.reset_measurements();
    }
}

/// Holds all the plots which are being build up.
/// Allows for clean tracking of the logs being created.
pub struct PlotsHolder {
    plots: HashMap<String, Plot>,
}

impl PlotsHolder {
    pub fn new() -> PlotsHolder {
        PlotsHolder {
            plots: HashMap::new(),
        }
    }

    pub fn add_measurement(&mut self, title: &str, labels: Vec<&str>, measurement: Vec<f32>) {
        match self.plots.get_mut(title) {
            Some(plot) => plot.add_measurement(measurement),
            None => {
                let new_plot = Plot::new(title, labels);
                self.plots.insert(title.to_string(), new_plot);
            }
        };
    }

    pub fn plots_to_logs(&mut self) {
        start_plot_group();
        for plot in self.plots.values_mut() {
            plot.to_logs();
        }
        stop_plot_group();
    }
}
