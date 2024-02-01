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
/// Implementation of the `StdObj` object. Contains helper functions for
/// calculating the objective function.
///
#[allow(non_snake_case)]
impl StdObj {
    //--------------------------------------------------------------------------
    /// Calculates the assignment cost for the objective function
    ///
    /// # Input
    /// * d: Data object containing the current charge schedule
    /// * i: Visit of interest
    /// * q: Charger queue of interest
    ///
    /// # Output
    /// * AC: Assignment cost for the provided schedule
    ///
    fn AC(d: &mut Data, i: usize, q: usize) -> f64 {
        // Extract input parameters
        let G = &d.param.Gam;
        let ep = &d.param.ep;
        let nu = d.param.nu;
        let k = &d.param.k;

        // Extract decision variables
        let w = &d.dec.w;
        let wiq = f64::from(w[i][q]);
        let eta = &d.dec.eta;

        // Calculate the penalty
        let mut phi: f64 = 0.0;

        // If the charge goes below the threshold
        if (eta[i] - (nu * k[G[i] as usize]) as f32) < 0.0 {
            // Calculate the penalty
            let c_dif = eta[i] - (nu * k[G[i] as usize]) as f32;
            let C: f32 = 500.0;

            phi = (0.5 * C * f32::powf(c_dif, 2.0)) as f64;
        }

        // Calculate the assignment cost
        return wiq as f64 * ep[q] as f64 + phi;
    }

    //--------------------------------------------------------------------------
    /// Calculates the utility cost for the objective function
    ///
    /// # Input
    /// * d: Data object containing the current charge schedule
    /// * i: Visit of interest
    /// * q: Charger queue of interest
    ///
    /// # Output
    /// * UC: Assignment cost for the provided schedule
    ///
    fn UC(_d: &mut Data, _i: usize, _q: usize) -> f64 {
        // Extract input parameters

        // Extract decision variables

        // Calculate the assignment cost
        return 0.0;
    }
}

//===============================================================================
/// Implementation of `Objective` for `StdObj` structure.
//
#[allow(non_snake_case)]
impl Objective for StdObj {
    //--------------------------------------------------------------------------
    /// Calculates the objective function for the provided schedule.
    ///
    /// # Input
    /// * d: Data object containing the current charge schedule
    ///
    /// # Output
    /// * J: Objective function cost
    ///
    fn run(d: &mut Data) -> f64 {
        // Variables
        let mut J: f64 = 0.0;

        // Extract input parameters
        let N = d.param.N;
        let Q = d.param.Q;

        // Calculate the objective function
        for i in 0..N {
            for q in 0..Q {
                J += StdObj::AC(d, i, q) + StdObj::UC(d, i, q);
            }
        }
        return J;
    }
}
