#![allow(non_snake_case)]

//===============================================================================
// Standard library
use chrono::{DateTime, Local};
use gnuplot::*;
use std::fs;

//===============================================================================
// Import modules
use crate::plotter::Plotter;
use crate::sa::data::Data;

//===============================================================================
/// Structure for `accumulated_energy_usage_plot`
pub struct AccumulatedEnergyUsagePlot {}

//===============================================================================
/// Implementation of the plotting function for the total accumulated energy usage.
/// This plot is a simple line graph that tracks the total amount of accumulated
/// energy used to run the provided charge schedule.
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Accumulated Energy Plot
///
///
impl Plotter for AccumulatedEnergyUsagePlot {
    fn plot(display_plot: bool, d: &mut Box<Data>) -> bool {
        // Variables
        let K = d.param.K as usize;
        let N = d.param.N;
        let T = d.param.T;
        let c = &d.dec.c;
        let r = &d.param.r;
        let u = &d.dec.u;
        let v = &d.dec.v;
        let delta = T / K as f32;

        // Configure plot
        let name: String = String::from("Accumulated Energy Usage");
        let mut fg = Figure::new();

        // Create array to count usage
        let mut usage: Vec<f32> = vec![0.0; K];

        // For each discrete time step
        for k in 0..K {
            // Calculate the discrete time
            let dt = k as f32 * delta;

            // If the index is greater than 0
            if k > 0 {
                usage[k] = usage[k - 1];
            }

            // For each visit
            for i in 0..N {
                // If the discrete time is within the active time for visit i
                if u[i] <= dt && c[i] >= dt {
                    usage[k] += r[v[i]] * delta;
                }
            }
        }

        // Generate the domain
        let x: Vec<f32> = (0..K).map(|k| k as f32 * delta).collect();

        // Configure the plot
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .lines(x, usage, &[]);

        // Plot Figure
        if display_plot {
            fg.show().unwrap();
        }

        // Get the month and time strings
        let current_local: DateTime<Local> = Local::now();
        let directory = current_local.format("%m/%d/%H-%M-%S/").to_string();
        let directory = "data/".to_string() + directory.as_str();

        // Create Directories
        fs::create_dir_all(directory.clone()).unwrap();

        // Save GNUPlot
        let name: String = String::from("accumulated-energy-usage");
        fg.echo_to_file(&format!("{}.gnuplot", directory + name.as_str()));

        return false;
    }
}
