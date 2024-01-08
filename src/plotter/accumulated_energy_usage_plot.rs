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
/// Implementation of the plotting function for the total accumulated energy usage.
/// This plot is a simple line graph that tracks the total amount of accumulated
/// energy used to run the provided charge schedule.
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Accumulated Energy Plot
///
///
impl Plotter for AccumulatedEnergyUsagePlot {
    fn plot(&mut self, d: &mut Box<Data>) -> bool {
        // Variables
        let K = d.param.K as usize;
        let N = d.param.N;
        let T = d.param.T;
        let c = &d.dec.c;
        let r = &d.param.r;
        let u = &d.dec.u;
        let v = &d.dec.v;
        let delta = K as f32 / T;

        // Configure plot
        let name: String = String::from("energy_usage_plot");
        let mut fg = Figure::new();

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
                if u[i] <= dt && c[i] >= delta {
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
            .set_y_label("Energy Usage [KWh]", &[])
            .lines(x, usage, &[]);

        // Plot Figure
        fg.show().unwrap();

        return false;
    }
}
