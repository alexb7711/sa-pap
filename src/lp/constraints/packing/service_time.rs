//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct ServiceTime {}

//===============================================================================
/// Implementation of `Constraint` for `ServiceTime` structure.
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
impl Constraint for ServiceTime {
    fn run(&mut self, d: &mut Data, i: usize, j: usize) -> bool {
        // Extract decision variables
        let c = &d.dec.c;
        let u = &d.dec.u;
        let s = &mut d.dec.s;

        // Constraint

        s[i] = c[i] - u[i];

        return true;
    }
}
