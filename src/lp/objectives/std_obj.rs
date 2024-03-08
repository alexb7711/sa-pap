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
    /// * dat: Data object containing the current charge schedule
    /// * i: Visit of interest
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
            let C: f32 = 9000.0;

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
        return 100.0 * (r * s) as f64;
    }

    //--------------------------------------------------------------------------
    /// Calculates the demand cost for the usage cost
    ///
    /// # Input
    /// * dat: Data structure for candidate schedule
    /// * ch : Charger availability object for candidate schedule
    ///
    /// # Output
    /// * pmax : Demand cost of the system
    fn demand_cost(dat: &mut Data, ch: &Charger) -> f64 {
        // Calculate vector of power consumption
        let p: Vec<f64> = StdObj::calc_power_vec(dat, ch);

        // Calculate the p15 and return the value
        return StdObj::calc_p15(&p);
    }

    //--------------------------------------------------------------------------
    /// Calculate the power vector over the time horizon.
    ///
    /// # Input
    /// * dat: Data structure for candidate schedule
    /// * ch : Charger availability object for candidate schedule
    ///
    /// # Output
    /// * p: Vector of power consumption at each discrete point
    ///
    fn calc_power_vec(dat: &Data, ch: &Charger) -> Vec<f64> {
        // Variables
        let dt = 1.0 / 60.0; // Step size of one minute
        let H = (dat.param.T / dt) as usize; // Get the time horizon divided by the step size
        let mut p: Vec<f64> = vec![0.0; H]; // Track the power consumption at each discrete point

        // For each charger queue
        for (i, q) in ch
            .schedule
            .iter()
            .enumerate()
            .skip_while(|x| x.0 < ch.charger_count.0)
        {
            // Get the charge rate
            let rate: f32 = ch.get_charge_rate(i);

            // For every time slice in the schedule for charger for `q`
            for ts in q {
                // Calculate the number of steps to take
                let n: usize = ((ts.t.1 - ts.t.0) / dt) as usize;

                // Create a vector of discrete time steps
                //
                // t = k*dt
                // k = t/dt
                //
                for k in linspace::<f64>(ts.t.0 as f64, ts.t.1 as f64, n).map(|x| x / dt as f64) {
                    p[k as usize] += rate as f64;
                }
            }
        }

        return p;
    }

    //--------------------------------------------------------------------------
    /// Calculate the p15 given the vector of discrete power consumption.
    ///
    /// # Input
    /// * p: Vector of discrete power consumption
    ///
    /// # Output
    /// * p15: Peak 15 over the time horizon
    ///
    fn calc_p15(p: &Vec<f64>) -> f64 {
        // Calculate p15
        let mut pmax: f64 = 0.0; // Maximum cost

        // For each visit that is after 15 minutes into the working day
        for (i, _) in p.iter().enumerate().skip_while(|x| x.0 < 15) {
            // Extract 15 minutes worth of power consumption and sum it
            let slice: f64 = p[i - 15..i].into_iter().sum();

            // If the slice is greater than pmax, update pmax
            if slice > pmax {
                pmax = slice;
            }
        }

        return pmax;
    }

    //--------------------------------------------------------------------------
    /// The run all constraints function does an exhaustive run of all the
    /// constraints. This function exists to ensure for debugging purposes.
    ///
    /// # Input
    /// * dat: Data object containing the current charge schedule
    /// * i: Visit of interest
    /// * run_constr: Flag to indicate whether to run all of the constraints
    ///
    /// # Output
    /// * (bool, f64): Flag to indicate success and the objective function score
    ///
    fn run_all_constr(dat: &mut Data, ch: &mut Charger, run_constr: bool) -> (bool, f64) {
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

    //--------------------------------------------------------------------------
    /// The run limited constraint function executes only the required constraints
    /// for the SA algorithm to function properly.
    ///
    /// # Input
    /// * dat: Data object containing the current charge schedule
    /// * i: Visit of interest
    /// * run_constr: Flag to indicate whether to run all of the constraints
    ///
    /// # Output
    /// * (bool, f64): Flag to indicate success and the objective function score
    ///
    fn run_lim_constr(dat: &mut Data, ch: &mut Charger, run_constr: bool) -> (bool, f64) {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Extract input parameters
        let N = dat.param.N;
        let mut J: f64 = 0.0;
        let mut val_sched: bool = false;

        for i in 0..N {
            //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Calculate constraints
            val_sched = constraints::run(run_constr, dat, ch, i, 0);

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
    /// * ch: Charger object
    /// * run_constr: Flag to indicate whether to run all of the constraints
    ///
    /// # Output
    /// * J: Objective function cost
    ///
    fn run(dat: &mut Data, ch: &mut Charger, run_constr: bool) -> (bool, f64) {
        // Variables
        let J: f64;
        let val_sched: bool;

        // If all the constraints are to be ran
        if run_constr {
            (val_sched, J) = StdObj::run_all_constr(dat, ch, run_constr);
        // Otherwise only a limited number of the constraints are required
        } else {
            (val_sched, J) = StdObj::run_lim_constr(dat, ch, run_constr);
        }

        return (val_sched, J);
    }
}
