//===============================================================================
// Import developed modules
use crate::lp::constraints::packing::service_time::ServiceTime;
use crate::lp::constraints::Constraint;
use crate::sa::charger::Charger;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct ValidInitDepEndTimes {}

//===============================================================================
/// Implementation of `Constraint` for `ValidInitDepEndTimes` structure.
///
/// # Input
/// * d: Data for the current model
/// * i: index of the visit
/// * j: index for the queue
///
/// # Output
/// * bool: Constraint successfully applied and is true
///
#[allow(non_snake_case)]
impl Constraint for ValidInitDepEndTimes {
    fn run(dat: &mut Data, _ch: &mut Charger, i: usize, _j: usize) -> bool {
        // Extract parameters
        let T = dat.param.T;
        let a = &dat.param.a;
        let e = &dat.param.e;

        // Extract decision variables
        let d = &dat.dec.d;
        let u = &dat.dec.u;
        let s = &dat.dec.s;

        // Constraint

        // Ensure the arrival time is before the attach time
        if !(a[i] <= u[i]) {
            println!("Visit {}", i);
            println!("{} > {}", a[i], u[i]);
            println!("valid_init_dep_end-time.rs: a[i] > u[i]");
            return false;
        }

        // Ensure the detach time is before the departure time
        if !(d[i] <= e[i]) {
            println!("Visit {}", i);
            println!("{} > {}", d[i], e[i]);
            println!("valid_init_dep_end-time.rs: d[i] > e[i]");
            return false;
        }

        // Ensure the initial time is early enough such that the service time does not exceed the time horizon
        if !(u[i] <= T - s[i]) {
            println!("Visit {}", i);
            println!("{} > {} - {}", u[i], T, s[i]);
            println!("valid_init_dep_end-time.rs: u[i] > T - s[i]");
            return false;
        }

        return true;
    }
}

//==============================================================================
/// Implementation of helper functions for `ValidInitDepEndTimes`
//
impl ValidInitDepEndTimes {
    //--------------------------------------------------------------------------
    /// The `update_dec_var` function updates the decision variables associated
    /// with the `ValidInitDepEndTimes` constraints.
    ///
    /// # Input
    /// * data: Simulated annealing data object.
    ///
    /// # Output
    /// * NONE
    ///
    fn _update_dec_var(data: &mut Data, ch: &mut Charger, i: usize, j: usize) {
        // Update service time
        ServiceTime::run(data, ch, i, j);
    }
}
