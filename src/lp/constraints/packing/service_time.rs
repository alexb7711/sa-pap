//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::charger::Charger;
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
    fn run(dat: &mut Data, _ch: &mut Charger, i: usize, _: usize) -> bool {
        // Extract decision variables
        let d = &dat.dec.d;
        let u = &dat.dec.u;
        let s = &mut dat.dec.s;

        // Constraint

        s[i] = d[i] - u[i];

        return true;
    }
}
