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
        let mut slow_x: Vec<Vec<f32>> = Vec::new();
        let mut slow_err: Vec<Vec<f32>> = Vec::new();
        let mut slow_y: Vec<Vec<usize>> = Vec::new();
        let mut fast_x: Vec<Vec<f32>> = Vec::new();
        let mut fast_err: Vec<Vec<f32>> = Vec::new();
        let mut fast_y: Vec<Vec<usize>> = Vec::new();

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create slow arrow vector data
        for b in G {
            // Temporary variables
            let mut x: Vec<f32> = Vec::new();
            let mut err: Vec<f32> = Vec::new();
            let mut y: Vec<usize> = Vec::new();

            // For each slow charger visit
            for i in 0..aslow.len() {
                // If the current visit is for bus b
                if *b == G[i] {
                    // Append the visit information to temporary vectors
                    x.push((cslow[i] - uslow[i]) / 2.0);
                    err.push(cslow[i] - uslow[i]);
                    y.push(vslow[i]);
                }
            }

            // Append temporary vectors to plotting vectors
            slow_x.push(x);
            slow_err.push(err);
            slow_y.push(y);
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create fast arrow vector data
        for b in G {
            // Temporary variables
            let mut x: Vec<f32> = Vec::new();
            let mut err: Vec<f32> = Vec::new();
            let mut y: Vec<usize> = Vec::new();

            // For each fast charger visit
            for i in 0..afast.len() {
                // If the current visit is for bus b
                if *b == G[i] {
                    // Append the visit information to temporary vectors
                    x.push(ufast[i]);
                    err.push(cfast[i]);
                    y.push(vfast[i]);
                }
            }

            // Append temporary vectors to plotting vectors
            fast_x.push(x);
            fast_err.push(err);
            fast_y.push(y)
        }

        //----------------------------------------------------------------------
        // Configure plot
        let name: String = String::from("schedule");
        let mut fg_slow = Figure::new();
        let mut fg_fast = Figure::new();

        // Plot slow charges
        for b in 0..A {
            fg_slow
                .axes2d()
                .set_title(name.as_str(), &[])
                .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
                .set_x_label("Time [hr]", &[])
                .set_y_label("Energy Usage [KWh]", &[])
                .x_error_bars(
                    slow_x[b].clone(),
                    slow_y[b].clone(),
                    slow_err[b].clone(),
                    &[],
                );
        }

        // Plot fast charges
        for b in 0..A {
            fg_fast
                .axes2d()
                .set_title(name.as_str(), &[])
                .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
                .set_x_label("Time [hr]", &[])
                .set_y_label("Energy Usage [KWh]", &[])
                .x_error_bars(
                    fast_x[b].clone(),
                    fast_y[b].clone(),
                    fast_err[b].clone(),
                    &[],
                );
        }

        // Plot Figure
        fg_slow.show().unwrap();
        fg_fast.show().unwrap();

        return false;
    }
}
