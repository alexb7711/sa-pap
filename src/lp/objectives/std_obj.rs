//===============================================================================
// Import developed modules
use crate::lp::objectives::Objective;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the data required to calculate the standard objective
/// function for SA PAP
//
pub struct StdObj {}

//===============================================================================
/// Implementation of `Objective` for `StdObj` structure.
//
#[allow(non_snake_case)]
impl Objective for StdObj {
    fn run(&mut self, d: &mut Data) -> f64 {
        // Variables
        let mut J: f64 = 0.0;

        // Extract input parameters
        let N = d.param.N;
        let Q = d.param.Q;
        let m = &d.param.m;
        let ep = &d.param.ep;

        // Extract decision variables
        let w = &d.dec.w;
        let g = &d.dec.g;

        // Calculate the objective function
        for i in 0..N {
            for q in 0..Q {
                J += f64::from(w[i][q]) * m[q] as f64 + g[i][q] as f64 * ep[q] as f64;
            }
        }
        return J;
    }
}
