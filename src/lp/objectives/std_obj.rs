//===============================================================================
// Import developed modules
use crate::lp::constraints::constraints;
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
    fn AC(dat: &mut Data, i: usize) -> f64 {
        // Extract input parameters
        let G = &dat.param.Gam;
        let ep = &dat.param.ep;
        let r = &dat.param.r;
        let nu = dat.param.nu;
        let k = &dat.param.k;

        // Extract decision variables
        let v = dat.dec.v[i];
        let eta = &dat.dec.eta;

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
        return (ep[v] * r[v]) as f64 + phi;
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
    fn UC(_dat: &mut Data, _i: usize) -> f64 {
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
    fn run(dat: &mut Data, run_constr: bool) -> (bool, f64) {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Extract input parameters
        let N = dat.param.N;
        let mut J: f64 = 0.0;
        let mut val_sched: bool = false;

        for i in 0..N {
            if run_constr {
                for j in 0..N {
                    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Calculate constraints
                    val_sched = constraints::run(dat, i, j);
                }
            }

            //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Calculate the objective function
            J += StdObj::AC(dat, i) + StdObj::UC(dat, i);
        }
        return (val_sched, J);
    }
}
