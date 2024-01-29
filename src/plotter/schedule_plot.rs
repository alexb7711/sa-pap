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
/// Helper functions for the `SchedulePlot` object.
///
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Schedule plot
///
///
impl SchedulePlot {
    //--------------------------------------------------------------------------
    ///
    /// # Input
    /// * : Boxed data
    ///
    /// # Output
    /// * None
    ///
    ///
    fn create_plot(d: &mut Box<Data>, fg_slow: &mut Figure, fg_fast: &mut Figure) {
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
            } else if v[i] >= A + d.param.slow && v[i] < A + d.param.slow + d.param.fast {
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
                    slow_x.push((cslow[i] + uslow[i]) / 2.0);
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
                    fast_x.push((cfast[i] + ufast[i]) / 2.0);
                    fast_err.push(cfast[i] - ufast[i]);
                    fast_y.push(vfast[i]);
                }
            }
        }

        //----------------------------------------------------------------------
        // Configure plot
        // Plot slow charges
        let name: String = String::from("Slow Schedule");
        fg_slow
            .axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Queue", &[])
            .set_y_range(Fix(0.0), Fix(d.param.slow as f64))
            .x_error_bars(slow_x.clone(), slow_y.clone(), slow_err, &[]);

        // Plot fast charges
        let name: String = String::from("Fast Schedule");
        fg_fast
            .axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Queue", &[])
            .set_y_range(Fix(0.0), Fix(d.param.fast as f64))
            .x_error_bars(fast_x.clone(), fast_y.clone(), fast_err.clone(), &[]);
    }

    //--------------------------------------------------------------------------
    /// The `save_do_disk` function outputs the results of the plot to disk.
    ///
    /// # Input
    /// * fg_slow: Figure for slow charger schedule
    /// * fg_fast: Figure for fast charger schedule
    ///
    /// # Output
    /// * NONE
    ///
    fn save_to_disk(fg_slow: &Figure, fg_fast: &Figure) {
        // Get the month and time strings
        let current_local: DateTime<Local> = Local::now();
        let directory = current_local.format("%m/%d/%H-%M-%S/").to_string();
        let directory = "data/".to_string() + directory.as_str();

        // Create Directories
        fs::create_dir_all(directory.clone()).unwrap();

        // Save GNUPlot
        let name: String = String::from("slow-schedule");
        fg_slow.echo_to_file(&format!("{}.gnuplot", directory.clone() + name.as_str()));

        let name: String = String::from("fast-schedule");
        fg_fast.echo_to_file(&format!("{}.gnuplot", directory.clone() + name.as_str()));
    }
}

//===============================================================================
/// Implementation of the plotting function to display the schedule of the BEBs.
/// The plot consists of
impl Plotter for SchedulePlot {
    //--------------------------------------------------------------------------
    ///
    /// # Input
    /// * d: Boxed data
    ///
    /// # Output
    /// * Schedule plot
    ///
    ///
    fn plot(display_plot: bool, d: &mut Box<Data>) {
        // Create object
        let mut fg_slow = Figure::new();
        let mut fg_fast = Figure::new();

        // Create plot
        SchedulePlot::create_plot(d, &mut fg_slow, &mut fg_fast);

        // Plot Figure
        if display_plot {
            fg_slow.show().unwrap();
            fg_fast.show().unwrap();
        }

        // Save to disk
        SchedulePlot::save_to_disk(&fg_slow, &fg_fast);
    }

    //--------------------------------------------------------------------------
    //
    fn real_time(
        display_plot: bool,
        d: &mut Box<Data>,
        fg_slow: &mut Figure,
        fg_fast: &mut Figure,
    ) {
        // Create plot
        SchedulePlot::create_plot(d, fg_slow, fg_fast);

        // Plot Figure
        if display_plot {
            fg_slow.show_and_keep_running().unwrap();
            fg_fast.show_and_keep_running().unwrap();
        }
    }
}
