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
pub struct ChargePlot {}

//===============================================================================
/// Implementation of the helper functions for the `ChargePlot` class.
impl ChargePlot {
    //--------------------------------------------------------------------------
    /// # Input
    /// - data: Data object
    ///
    /// # Output
    /// - x : Array of incrementing values from 1 to N
    /// - y : Array of charges for each bus
    ///
    fn group_charge_results(d: &Data) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        // Variables
        let A = d.param.A;
        let N = d.param.N;
        let G = &d.param.Gam;
        let c = &d.dec.c;
        let r = &d.param.r;
        let u = &d.dec.u;
        let eta = &d.dec.eta;
        let v = &d.dec.v;
        let s = &d.dec.s;

        let mut charges: Vec<Vec<f32>> = Vec::new();
        let mut idx: Vec<Vec<f32>> = Vec::new();

        // For each BEB
        for b in 0..A {
            let mut tmpx: Vec<f32> = Vec::new();
            let mut tmpy: Vec<f32> = Vec::new();

            // For each visit
            for i in 0..N {
                if G[i] as usize == b {
                    // Append the charge on arrival
                    tmpx.push(u[i]);
                    tmpy.push(eta[i]);

                    // Append the charge on departure
                    tmpx.push(c[i]);
                    tmpy.push(eta[i] + s[i] * r[v[i]]);
                }
            }

            // Update the plot arrays
            idx.push(tmpx);
            charges.push(tmpy);
        }

        return (idx, charges);
    }
}

//===============================================================================
/// Implementation of the plotting function for plotting the individual charges
/// for each BEB. It is a simple line graph that tracks the SOC for each BEB over
/// the time horizon of the simulation.
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Accumulated Energy Plot
///
///
impl Plotter for ChargePlot {
    fn plot(d: &mut Box<Data>) -> bool {
        // Variables
        let A = d.param.A;

        // Configure plot
        let name: String = String::from("charge");
        let mut fg = Figure::new();
        let (x, y) = ChargePlot::group_charge_results(&d);

        // Plot each charge line
        for i in 0..A {
            // Configure the plot
            fg.axes2d()
                .set_title(name.as_str(), &[])
                .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
                .set_x_label("Time [hr]", &[])
                .set_x_range(Fix(0.0), Fix(24.0))
                .set_y_label("Energy Usage [KWh]", &[])
                .lines(x[i].clone(), y[i].clone(), &[]);
        }

        // Plot Figure
        fg.show().unwrap();

        // Get the month and time strings
        let current_local: DateTime<Local> = Local::now();
        let directory = current_local.format("%m/%d/%H-%M-%S/").to_string();
        let directory = "data/".to_string() + directory.as_str();

        // Create Directories
        fs::create_dir_all(directory.clone()).unwrap();

        // Save GNUPlot
        fg.echo_to_file(&format!("{}.gnuplot", directory + name.as_str()));

        return false;
    }
}
