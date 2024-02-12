//===============================================================================
// Import developed modules
use crate::lp::constraints::constraints;
use crate::lp::objectives::Objective;
use crate::sa::charger::Charger;
use crate::sa::data::Data;

//===============================================================================
// Import external crate
use itertools_num::linspace;

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
        let c_dif = eta[i] - (nu * k[G[i] as usize]) as f32;
        if c_dif < 0.0 {
            // Calculate the penalty
            let C: f32 = 500.0;

            phi = (C * f32::powf(c_dif, 2.0)) as f64;
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
    fn UC(dat: &mut Data, i: usize) -> f64 {
        // Extract decision variables
        let s = dat.dec.s[i];
        let v = dat.dec.v[i];

        // Extract input parameters
        let r = dat.param.r[v];

        // Calculate the consumption cost
        let cc = (r * s) as f64;

        // Calculate the assignment cost
        return cc;
    }

    //--------------------------------------------------------------------------
    /// Calculates the demand cost for the usage cost
    ///
    /// # Input
    /// * dat: Data structure for candidate schedule
    /// * ch : Charger availability object for candidate schedule
    /// i    : Index of the visit of interest
    ///
    /// # Output
    /// * pmax : Demand cost of the system
    fn demand_cost(dat: &mut Data, ch: &Charger) -> f64 {
        // Variables
        let dt = 0.15; // Step size of p15
        let H = (dat.param.T / dt) as usize; // Get the time horizon divided by the step size
        let pfix: f64 = 20.0; // TODO: Placeholder, get this data from data
        let mut p: Vec<f64> = vec![0.0; H - 15]; // Create a `p' vector of size H - 15
        let pmax: f64 = 0.0; // Maximum cost

        // Calculate p vector
        for (i, q) in ch.schedule.iter().enumerate() {
            // Get the charge for charger `q`
            let mut rate: f32 = 0.0;
            if i < ch.charger_count.0 {
                continue;
            } else if i >= ch.charger_count.0 && i < ch.charger_count.0 + ch.charger_count.1 {
                rate = ch.charger_speed.1;
            } else if i >= ch.charger_count.0 + ch.charger_count.1
                && i < ch.charger_count.0 + ch.charger_count.1 + ch.charger_count.2
            {
                rate = ch.charger_speed.2;
            }

            // For every time slice in the charge schedule for `q`
            for ts in q {
                // Calculate the number of steps to take
                let n: usize = ((ts.t.1 - ts.t.0) / dt) as usize;

                // Create a vector of discrete time steps
                for j in linspace::<f64>(ts.t.0 as f64, dt as f64, n).map(|x| x / dt as f64) {
                    p[j as usize] += rate as f64;
                }
            }
        }

        // Calculate p15
        for _val in p.iter() {}

        return pmax;
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
    fn run(dat: &mut Data, ch: &mut Charger, run_constr: bool) -> (bool, f64) {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Extract input parameters
        let N = dat.param.N;
        let mut J: f64 = 0.0;
        let mut val_sched: bool = false;

        for i in 0..N {
            for j in 0..N {
                //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                // Calculate constraints
                val_sched = constraints::run(run_constr, dat, ch, i, j);
            }

            //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Calculate the objective function
            J += StdObj::AC(dat, i) + StdObj::UC(dat, i);
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Calculate the demand cost
        J += StdObj::demand_cost(dat, ch);

        return (val_sched, J);
    }
}
