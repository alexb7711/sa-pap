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
pub struct ScorePlot {}

//===============================================================================
//
impl ScorePlot {
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
    fn create_plot(dat: &mut Box<Data>, fg: &mut Figure) {
        // Variables
        let score: &Vec<f64> = &dat.dec.J;

        // Create domain
        let x: Vec<f32> = (0..score.len()).map(|x| x as f32).collect();

        // Configure the plot
        let name: String = String::from("Score");
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_x_label("Iteration", &[])
            .set_y_label("Score", &[])
            .boxes(x.clone(), score, &[]);
    }

    //--------------------------------------------------------------------------
    /// The `save_do_disk` function outputs the results of the plot to disk.
    ///
    /// # Input
    /// * fg: Figure for charger schedule
    ///
    /// # Output
    /// * NONE
    ///
    fn save_to_disk(fg: &Figure) {
        // Get the month and time strings
        let current_local: DateTime<Local> = Local::now();
        let directory = current_local.format("%m/%d/%H-%M-%S/").to_string();
        let directory = "data/".to_string() + directory.as_str();

        // Create Directories
        fs::create_dir_all(directory.clone()).unwrap();

        // Save GNUPlot
        let name: String = String::from("slow-charger-power-usage");
        fg.echo_to_file(&format!("{}.gnuplot", directory.clone() + name.as_str()));
    }
}

//===============================================================================
/// Implementation of the plotting function for the total accumulated energy.
/// That it, this line graph tracks the total energy consumed by the system over
/// the working day.
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Power usage plot
///
impl Plotter for ScorePlot {
    fn plot(display_plot: bool, dat: &mut Box<Data>) {
        // Configure plot
        let mut fg = Figure::new();

        // Create plot
        ScorePlot::create_plot(dat, &mut fg);

        // Plot Figure
        if display_plot {
            fg.show().unwrap();
        }

        // Save to disk
        ScorePlot::save_to_disk(&fg);
    }

    //===============================================================================
    //
    fn real_time(_: bool, dat: &mut Box<Data>, fg: &mut Figure) {
        // Clear plots
        fg.clear_axes();

        // Create plot
        ScorePlot::create_plot(dat, fg);

        // Update plots
        fg.show_and_keep_running().unwrap();
    }
}
