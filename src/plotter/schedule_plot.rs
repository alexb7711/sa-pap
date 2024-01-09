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
pub struct SchedulePlot {}

//===============================================================================
/// Implementation of the plotting function to display the schedule of the BEBs.
/// The plot consists of
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Accumulated Energy Plot
///
///
impl Plotter for SchedulePlot {
    fn plot(d: &mut Box<Data>) -> bool {
        // Variables
        let N = d.param.N;
        let A = d.param.A;
        let G = &d.param.Gam;
        let a = &d.param.a;
        let c = &d.dec.c;
        let u = &d.dec.u;
        let v = &d.dec.v;

        // Create array buffers
        let mut aslow: Vec<f32> = Vec::new();
        let mut cslow: Vec<f32> = Vec::new();
        let mut uslow: Vec<f32> = Vec::new();
        let mut vslow: Vec<usize> = Vec::new();
        let mut afast: Vec<f32> = Vec::new();
        let mut cfast: Vec<f32> = Vec::new();
        let mut ufast: Vec<f32> = Vec::new();
        let mut vfast: Vec<usize> = Vec::new();

        //----------------------------------------------------------------------
        // Loop through each visit
        for i in 0..N {
            if v[i] >= A && v[i] < A + d.param.slow {
                aslow.push(a[i]);
                cslow.push(c[i]);
                uslow.push(u[i]);
                vslow.push(v[i]);
            } else {
                afast.push(a[i]);
                cfast.push(c[i]);
                ufast.push(u[i]);
                vfast.push(v[i]);
            }
        }

        //----------------------------------------------------------------------
        // Create slow and fast arrays
        let mut slow_x: Vec<f32> = Vec::new();
        let mut slow_err: Vec<f32> = Vec::new();
        let mut slow_y: Vec<usize> = Vec::new();
        let mut fast_x: Vec<f32> = Vec::new();
        let mut fast_err: Vec<f32> = Vec::new();
        let mut fast_y: Vec<usize> = Vec::new();

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create slow arrow vector data
        for b in G {
            // For each slow charger visit
            for i in 0..aslow.len() {
                // If the current visit is for bus b
                if *b == G[i] {
                    // Append the visit information to vectors
                    slow_x.push((cslow[i] - uslow[i]) / 2.0);
                    slow_err.push(cslow[i] - uslow[i]);
                    slow_y.push(vslow[i]);
                }
            }
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create fast arrow vector data
        for b in G {
            // For each fast charger visit
            for i in 0..afast.len() {
                // If the current visit is for bus b
                if *b == G[i] {
                    // Append the visit information to vectors
                    fast_x.push((cfast[i] - ufast[i]) / 2.0);
                    fast_err.push(cfast[i] - ufast[i]);
                    fast_y.push(vfast[i]);
                }
            }
        }

        //----------------------------------------------------------------------
        // Configure plot
        let mut fg_slow = Figure::new();
        let mut fg_fast = Figure::new();

        // Plot slow charges
        let name: String = String::from("Slow Schedule");
        fg_slow
            .axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .x_error_bars(slow_x.clone(), slow_y.clone(), slow_err, &[]);

        // Plot fast charges
        let name: String = String::from("Fast Schedule");
        fg_fast
            .axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .x_error_bars(fast_x.clone(), fast_y.clone(), fast_err.clone(), &[]);

        // Plot Figure
        fg_slow.show().unwrap();
        fg_fast.show().unwrap();

        // Get the month and time strings
        let current_local: DateTime<Local> = Local::now();
        let directory = current_local.format("%m/%d/%H-%M-%S/").to_string();
        let directory = "data/".to_string() + directory.as_str();

        // Create Directories
        fs::create_dir_all(directory.clone()).unwrap();

        // Save GNUPlot
        let name: String = String::from("Slow Schedule");
        fg_slow.echo_to_file(&format!("{}.gnuplot", directory.clone() + name.as_str()));

        let name: String = String::from("Fast Schedule");
        fg_fast.echo_to_file(&format!("{}.gnuplot", directory.clone() + name.as_str()));

        return false;
    }
}
