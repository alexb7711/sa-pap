#![allow(non_snake_case)]

//===============================================================================
// Standard library
use gnuplot::*;

//===============================================================================
// Import modules
use crate::plotter::Plotter;
use crate::sa::data::Data;

//===============================================================================
/// Structure for `accumulated_energy_usage_plot`
pub struct AccumulatedEnergyUsagePlot {}

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
impl AccumulatedEnergyUsagePlot {
    //--------------------------------------------------------------------------
    /// Process the data for the figure.
    ///
    /// # Input
    /// * d : Boxed data
    /// * fg: Charger schedule figure
    /// * fg_fast: Fast charger schedule figure
    ///
    /// # Output
    /// * None
    ///
    fn create_plot(dat: &mut Box<Data>, fg: &mut Figure) {
        // Variables
        let K = dat.param.K as usize;
        let N = dat.param.N;
        let T = dat.param.T;
        let d = &dat.dec.d;
        let r = &dat.param.r;
        let u = &dat.dec.u;
        let v = &dat.dec.v;
        let delta = T / K as f32;

        // Configure plot
        let name: String = String::from("Accumulated Energy Usage");

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
                if u[i] <= dt && d[i] >= dt {
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
    fn save_to_disk(fg: &Figure, p: &String) {
        // Save GNUPlot
        let name: String = String::from("accumulated-energy-usage");
        fg.echo_to_file(&format!("{}.gnuplot", p.clone() + name.as_str()));
    }
}

//===============================================================================
/// Implementation of the plotting function for the total accumulated energy usage.
/// This plot is a simple line graph that tracks the total amount of accumulated
/// energy used to run the provided charge schedule.
///
/// # Input
/// * display_plot: Flag to indicate whether the plot is to be displayed
/// * d: Boxed data
/// * p: Base path to the plot will be saved
///
/// # Output
/// * Accumulated Energy Plot
///
///
impl Plotter for AccumulatedEnergyUsagePlot {
    fn plot(display_plot: bool, dat: &mut Box<Data>, p: &String) {
        let mut fg = Figure::new();

        // Create plot
        AccumulatedEnergyUsagePlot::create_plot(dat, &mut fg);

        // Plot Figure
        if display_plot {
            fg.show().unwrap();
        }

        // Save to disk
        AccumulatedEnergyUsagePlot::save_to_disk(&fg, p);
    }

    //===============================================================================
    //
    fn real_time(rpt: bool, dat: &mut Box<Data>, fg: &mut Figure) {
        // Determine whether to create the plot
        if !rpt {
            return;
        }

        // Clear plots
        fg.clear_axes();

        // Create plot
        AccumulatedEnergyUsagePlot::create_plot(dat, fg);

        // Update plots
        fg.show_and_keep_running().unwrap();
    }
}
