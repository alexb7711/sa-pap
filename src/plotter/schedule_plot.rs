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
    /// Process the data for the figure.
    ///
    /// # Input
    /// * d : Boxed data
    /// * fg_slow: Slow charger schedule figure
    /// * fg_fast: Fast charger schedule figure
    ///
    /// # Output
    /// * None
    ///
    fn create_plot(dat: &mut Box<Data>, fg_slow: &mut Figure, fg_fast: &mut Figure) {
        // Variables
        let N = dat.param.N;
        let A = dat.param.A;
        let d = &dat.dec.d;
        let u = &dat.dec.u;
        let v = &dat.dec.v;

        // Create array buffers
        let mut cslow: Vec<f32> = Vec::new();
        let mut uslow: Vec<f32> = Vec::new();
        let mut vslow: Vec<usize> = Vec::new();
        let mut cfast: Vec<f32> = Vec::new();
        let mut ufast: Vec<f32> = Vec::new();
        let mut vfast: Vec<usize> = Vec::new();

        //----------------------------------------------------------------------
        // Loop through each visit
        for i in 0..N {
            if v[i] >= A && v[i] < A + dat.param.slow {
                cslow.push(d[i]);
                uslow.push(u[i]);
                vslow.push(v[i]);
            } else if v[i] >= A + dat.param.slow && v[i] < A + dat.param.slow + dat.param.fast {
                cfast.push(d[i]);
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
        for i in 0..cslow.len() {
            // Append the visit information to vectors
            slow_x.push((cslow[i] + uslow[i]) / 2.0);
            slow_err.push((cslow[i] - uslow[i]) / 2.0);
            slow_y.push(vslow[i]);
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create fast arrow vector data
        for i in 0..cfast.len() {
            // Append the visit information to vectors
            fast_x.push((cfast[i] + ufast[i]) / 2.0);
            fast_err.push((cfast[i] - ufast[i]) / 2.0);
            fast_y.push(vfast[i]);
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
            .set_y_range(Fix(A as f64), Fix(A as f64 + dat.param.slow as f64))
            .x_error_bars(
                slow_x.clone(),
                slow_y.clone(),
                slow_err,
                &[PointSymbol('x'), Color("blue")],
            );

        // Plot fast charges
        let name: String = String::from("Fast Schedule");
        fg_fast
            .axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Queue", &[])
            .set_y_range(
                Fix(A as f64 + dat.param.slow as f64),
                Fix(A as f64 + dat.param.slow as f64 + dat.param.fast as f64),
            )
            .x_error_bars(
                fast_x.clone(),
                fast_y.clone(),
                fast_err.clone(),
                &[PointSymbol('x'), Color("red")],
            );
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
//
impl Plotter for SchedulePlot {
    //--------------------------------------------------------------------------
    /// Implementation of the plotting function to display the schedule of the BEBs.
    /// The plot consists of
    ///
    /// # Input
    /// * d: Boxed data
    ///
    /// # Output
    /// * Schedule plot
    ///
    ///
    fn plot(display_plot: bool, dat: &mut Box<Data>) {
        // Create object
        let mut fg_slow = Figure::new();
        let mut fg_fast = Figure::new();

        // Create plot
        SchedulePlot::create_plot(dat, &mut fg_slow, &mut fg_fast);

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
        dat: &mut Box<Data>,
        fg_slow: &mut Figure,
        fg_fast: &mut Figure,
    ) {
        if display_plot {
            // Clear plots
            fg_slow.clear_axes();
            fg_fast.clear_axes();

            // Create plot
            SchedulePlot::create_plot(dat, fg_slow, fg_fast);

            // Update plots
            fg_slow.show_and_keep_running().unwrap();
            fg_fast.show_and_keep_running().unwrap();
        }
    }
}
