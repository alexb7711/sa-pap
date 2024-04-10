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
pub struct ChargePlot {}

//===============================================================================
/// Implementation of the helper functions for the `ChargePlot` class.
impl ChargePlot {
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

        // Configure plot
        let name: String = String::from("charge");
        let ax = fg.axes2d();
        let (x, y) = ChargePlot::group_charge_results(&dat);

        // Plot each charge line
        for i in 0..A {
            // Configure the plot
            ax.set_title(name.as_str(), &[])
                .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
                .set_x_label("Time [hr]", &[])
                .set_x_range(Fix(0.0), Fix(24.0))
                .set_y_label("Energy Usage [KWh]", &[])
                .lines(x[i].clone(), y[i].clone(), &[]);
        }
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
        let name: String = String::from("charge");
        fg.echo_to_file(&format!("{}.gnuplot", p.clone() + name.as_str()));
    }
    //--------------------------------------------------------------------------
    /// # Input
    /// - data: Data object
    ///
    /// # Output
    /// - x : Array of incrementing values from 1 to N
    /// - y : Array of charges for each bus
    ///
    fn group_charge_results(dat: &Data) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        // Variables
        let A = dat.param.A;
        let N = dat.param.N;
        let G = &dat.param.Gam;
        let d = &dat.dec.d;
        let r = &dat.param.r;
        let u = &dat.dec.u;
        let eta = &dat.dec.eta;
        let v = &dat.dec.v;
        let s = &dat.dec.s;

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
                    tmpx.push(d[i]);
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
/// * Charge plot
///
///
impl Plotter for ChargePlot {
    fn plot(display_plot: bool, dat: &mut Box<Data>, p: &String) {
        let mut fg = Figure::new();

        // Create plot
        ChargePlot::create_plot(dat, &mut fg);

        // Plot Figure
        if display_plot {
            fg.show().unwrap();
        }

        // Save to disk
        ChargePlot::save_to_disk(&fg, p);
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
        ChargePlot::create_plot(dat, fg);

        // Update plots
        fg.show_and_keep_running().unwrap();
    }
}
