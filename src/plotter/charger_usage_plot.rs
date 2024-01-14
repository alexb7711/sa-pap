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
pub struct ChargerUsagePlot {}

//===============================================================================
/// Implementation of the plotting function for the slow/fast charger usage.
/// It is a box graph that is merely tracking how many fast/slow chargers are
/// being utilized at any given discrete time.
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Charge usage plot
///
impl Plotter for ChargerUsagePlot {
    fn plot(display_plot: bool, d: &mut Box<Data>) {
        // Variables
        let A = d.param.A;
        let N = d.param.N;
        let T = d.param.T;
        let K = d.param.K;
        let v = &d.dec.v;
        let u = &d.dec.u;
        let c = &d.dec.c;
        let delta = T / K as f32;

        let mut slow: Vec<usize> = vec![0; K as usize];
        let mut fast: Vec<usize> = vec![0; K as usize];

        // Configure plot
        let name: String = String::from("Charger Usage");
        let mut fg = Figure::new();
        fg.set_multiplot_layout(2, 1);

        // Create domain
        let x: Vec<f32> = (0..K).map(|x| x as f32 * delta).collect();

        // For each discrete time step
        for k in 0..K {
            // Calculate the discrete time
            let dt = k as f32 * delta;

            // For each visit
            for i in 0..N {
                // Check if the visit is in within the current discrete step
                if u[i] <= dt && c[i] >= dt {
                    // Check if the BEB is assigned to a slow charger
                    if v[i] >= A && v[i] < A + d.param.slow {
                        slow[k as usize] += 1;
                    // Check if the BEB is assigned to a fast charger
                    } else if v[i] >= A + d.param.slow && v[i] < A + d.param.slow + d.param.fast {
                        fast[k as usize] += 1;
                    }
                }
            }
        }

        // Configure the plot
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .boxes(x.clone(), slow, &[]);

        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .boxes(x.clone(), fast, &[]);

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
        let name: String = String::from("charger-usage");
        fg.echo_to_file(&format!("{}.gnuplot", directory + name.as_str()));
    }
}
