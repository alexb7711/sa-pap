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
pub struct PowerUsagePlot {}

//===============================================================================
//
impl PowerUsagePlot {
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
        let N = dat.param.N;
        let T = dat.param.T;
        let K = dat.param.K;
        let v = &dat.dec.v;
        let u = &dat.dec.u;
        let d = &dat.dec.d;
        let r = &dat.param.r;
        let delta = T / K as f32;

        let mut power: Vec<f32> = vec![0.0; K as usize];

        // Create domain
        let x: Vec<f32> = (0..K).map(|x| x as f32 * delta).collect();

        // For each discrete time step
        for k in 0..K {
            // Calculate the discrete time
            let dt = k as f32 * delta;

            // For each visit
            for i in 0..N {
                // Check if the visit is in within the current discrete step
                if u[i] <= dt && d[i] >= dt {
                    // Add on the power
                    power[k as usize] += r[v[i]];
                }
            }
        }

        // Configure the plot
        let name: String = String::from("Power Usage");
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .boxes(x.clone(), power, &[]);
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
        let name: String = String::from("power-usage");
        fg.echo_to_file(&format!("{}.gnuplot", p.clone() + name.as_str()));
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
impl Plotter for PowerUsagePlot {
    fn plot(display_plot: bool, dat: &mut Box<Data>, p: &String) {
        // Configure plot
        let mut fg = Figure::new();

        // Create plot
        PowerUsagePlot::create_plot(dat, &mut fg);

        // Plot Figure
        if display_plot {
            fg.show().unwrap();
        }

        // Save to disk
        PowerUsagePlot::save_to_disk(&fg, p);
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
        PowerUsagePlot::create_plot(dat, fg);

        // Update plots
        fg.show_and_keep_running().unwrap();
    }
}
