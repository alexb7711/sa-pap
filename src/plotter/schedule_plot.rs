#![allow(non_snake_case)]

use core::f32;
use std::f32;
use std::simd::usizex1;

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
    fn plot(&mut self, d: &mut Box<Data>) -> bool {
        // Variables
        let K = d.param.K as usize;
        let N = d.param.N;
        let T = d.param.T;
        let A = d.param.A;
        let G = d.param.Gam;
        let a = &d.param.a;
        let c = &d.dec.c;
        let r = &d.param.r;
        let u = &d.dec.u;
        let v = &d.dec.v;
        let delta = K as f32 / T;

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
        let mut slow_x1: Vec<Vec<f32>> = Vec::new();
        let mut slow_x2: Vec<Vec<f32>> = Vec::new();
        let mut slow_y: Vec<Vec<usize>> = Vec::new();
        let mut fast_x1: Vec<Vec<f32>> = Vec::new();
        let mut fast_x2: Vec<Vec<f32>> = Vec::new();
        let mut fast_y: Vec<Vec<usize>> = Vec::new();

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create slow arrow vector data
        for b in G {
            let mut x1: Vec<f32> = Vec::new();
            let mut x2: Vec<f32> = Vec::new();
            let mut y: Vec<usize> = Vec::new();
            for i in 0..aslow.len() {
                if b == G[i] {
                    x1.push(uslow[i]);
                    x2.push(cslow[i]);
                    y.push(vslow[i]);
                }
            }
            slow_x1.push(x1);
            slow_x2.push(x2);
            slow_y.push(y);
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create fast arrow vector data
        for b in G {
            let mut x1: Vec<f32> = Vec::new();
            let mut x2: Vec<f32> = Vec::new();
            let mut y: Vec<usize> = Vec::new();
            for i in 0..afast.len() {
                if b == G[i] {
                    x1.push(ufast[i]);
                    x2.push(cfast[i]);
                    y.push(vfast[i]);
                }
            }
            fast_x1.push(x1);
            fast_x2.push(x2);
            fast_y.push(y)
        }

        //----------------------------------------------------------------------
        // Configure plot
        let name: String = String::from("schedule");
        let mut fg_slow = Figure::new();
        let mut fg_fast = Figure::new();

        // Configure the plot
        for b in 0..A {
            fg_slow
                .axes2d()
                .set_title(name.as_str(), &[])
                .set_legend(gnuplot::Graph(0.0), gnuplot::Graph(1.0), &[], &[])
                .set_x_label("Time [hr]", &[])
                .set_y_label("Energy Usage [KWh]", &[])
                .boxes_set_width(slow_x1[b], slow_y[b], slow_x2[b], &[]);
        }

        // Plot Figure
        fg_slow.show().unwrap();

        return false;
    }
}
