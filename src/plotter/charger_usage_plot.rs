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
pub struct ChargerUsagePlot {}

//===============================================================================
/// Helper functions for the `ChargerUsagePlot` object.
///
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Charger Usage plot
///
///
impl ChargerUsagePlot {
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
        let A = dat.param.A;
        let N = dat.param.N;
        let T = dat.param.T;
        let K = dat.param.K;
        let v = &dat.dec.v;
        let u = &dat.dec.u;
        let d = &dat.dec.d;
        let delta = T / K as f32;

        let mut slow: Vec<usize> = vec![0; K as usize];
        let mut fast: Vec<usize> = vec![0; K as usize];

        // Configure plot
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
                if u[i] <= dt && d[i] >= dt {
                    // Check if the BEB is assigned to a slow charger
                    if v[i] >= A && v[i] < A + dat.param.slow {
                        slow[k as usize] += 1;
                    // Check if the BEB is assigned to a fast charger
                    } else if v[i] >= A + dat.param.slow
                        && v[i] < A + dat.param.slow + dat.param.fast
                    {
                        fast[k as usize] += 1;
                    }
                }
            }
        }

        // Configure the plot
        let name: String = String::from("Slow Charger Usage");
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .boxes(x.clone(), slow, &[]);

        let name: String = String::from("Fast Charger Usage");
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_x_range(Fix(0.0), Fix(24.0))
            .set_y_label("Energy Usage [KWh]", &[])
            .boxes(x.clone(), fast, &[]);
    }

    //--------------------------------------------------------------------------
    /// The `save
    ///
    /// # Input
    /// * fg: Figure for charger schedule
    ///
    /// # Output
    /// * NONE
    ///
    fn save_to_disk(fg: &Figure, p: &String) {
        // Save GNUPlot
        let name: String = String::from("charger-usage");
        fg.echo_to_file(&format!("{}.gnuplot", p.clone() + name.as_str()));
    }
}

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
    fn plot(display_plot: bool, dat: &mut Box<Data>, p: &String) {
        let mut fg = Figure::new();

        // Create plot
        ChargerUsagePlot::create_plot(dat, &mut fg);

        // Plot Figure
        if display_plot {
            fg.show().unwrap();
        }

        // Save to disk
        ChargerUsagePlot::save_to_disk(&fg, p);
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
        ChargerUsagePlot::create_plot(dat, fg);

        // Update plots
        fg.show_and_keep_running().unwrap();
    }
}
