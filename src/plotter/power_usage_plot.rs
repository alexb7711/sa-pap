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
pub struct PowerUsage {}

//===============================================================================
/// Implementation of the plotting function for the total accumulated energy.
/// That it, this line graph tracks the total energy consumed by the system over
/// the working day.
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Accumulated Energy Plot
///
impl Plotter for PowerUsage {
    fn plot(&mut self, d: &mut Box<Data>) -> bool {
        // Variables
        let A = d.param.A;
        let N = d.param.N;
        let T = d.param.T;
        let K = d.param.K;
        let v = &d.dec.v;
        let u = &d.dec.u;
        let c = &d.dec.c;
        let delta = K as f32 / T;

        let mut slow: Vec<usize> = vec![0; d.param.slow];
        let mut fast: Vec<usize> = vec![0; d.param.fast];

        // Configure plot
        let name: String = String::from("charger_usage");
        let mut fg = Figure::new();
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
                if u[i] <= dt && c[i] >= dt {
                    // Check if the BEB is assigned to a slow charger
                    if v[i] >= A && v[i] < A + d.param.slow {
                        slow[k as usize] += 1;
                    // Check if the BEB is assigned to a fast charger
                    } else if v[i] >= A + d.param.slow && v[i] < A + d.param.slow + d.param.fast {
                        fast[k as usize] += 1;
                    }
                }
            }
        }

        // Configure the plot
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_y_label("Energy Usage [KWh]", &[])
            .lines(x.clone(), slow, &[]);

        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
            .set_x_label("Time [hr]", &[])
            .set_y_label("Energy Usage [KWh]", &[])
            .lines(x.clone(), fast, &[]);

        // Plot Figure
        fg.show().unwrap();

        return false;
    }
}
