//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct SpaceTimeBigO {}

//===============================================================================
/// Implementation of `Constraint` for `SpaceTimeBigO` structure.
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
impl Constraint for SpaceTimeBigO {
    fn run(&mut self, d: &mut Data, i: usize, j: usize) -> bool {
        // Extract parameters
        let Q = d.param.Q;
        let S = d.param.S;

        // Extract decision variables
        let psi = d.dec.psi;
        let sig = d.dec.sigma;
        let v = d.dec.v;
        let s = d.dec.s;
        let u = d.dec.u;

        // Constraint

        if i != j {
            // Check the spatial allocations
            if !(v[i] - v[j] - S - (psi[i][j] as usize - S) * Q >= 0) {
                return false;
            }

            // Check the temporal time allocations
            if !(u[i] - u[j] - s[j] - (f32::from(sig[i][j]) - 1.0) * Q as f32 >= 0.0) {
                return false;
            }
        }

        return true;
    }
}
