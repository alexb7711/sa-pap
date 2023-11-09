//===============================================================================
// Import developed modules
use crate::lp::constraints::packing::service_time::ServiceTime;
use crate::lp::constraints::Constraint;
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
    fn run(d: &mut Data, i: usize, j: usize) -> bool {
        // Update decision variables
        ValidInitDepEndTimes::update_dec_var(d, i, j);

        // Extract parameters
        let T = d.param.T;
        let a = &d.param.a;
        let e = &d.param.e;

        // Extract decision variables
        let c = &d.dec.c;
        let u = &d.dec.u;
        let s = &d.dec.s;

        // Constraint

        // Ensure the arrival time is before the attach time
        if !(a[i] <= u[i]) {
            println!("Invalid arrival time.");
            return false;
        }

        // Ensure the detach time is before the departure time
        if !(c[i] <= e[i]) {
            println!("Invalid detach time.");
            return false;
        }

        // Ensure the initial time is early enough such that the service time does not exceed the time horizon
        if !(u[i] <= T - s[i]) {
            println!("Invalid attach time.");
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
    fn update_dec_var(data: &mut Data, i: usize, j: usize) {
        // Update sigma/psi decision variables
        ServiceTime::run(data, i, j);
    }
}
