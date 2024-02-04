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
    fn run(d: &mut Data, i: usize, j: usize) -> bool {
        // Extract decision variables
        let psi = &mut d.dec.psi;
        let sig = &mut d.dec.sigma;

        // Constraint

        // If `i == j` set sigma and psi to false
        if i == j {
            psi[i][j] = false;
            sig[i][j] = false;
            return true;
        // Otherwise `i != j`
        } else {
            // Extract parameters
            let Q = d.param.Q as i32;
            let T = d.param.T;
            let S = d.param.S as i32;

            // Extract decision variables
            let vi = d.dec.v[i] as i32;
            let vj = d.dec.v[j] as i32;
            let si = d.dec.s[i];
            let ui = &d.dec.u[i];
            let uj = &d.dec.u[j];

            // Default to true
            psi[i][j] = true;
            sig[i][j] = true;

            // Calculate constraints
            if (vj - vi - S - (psi[i][j] as i32 - S) * Q) < 0 {
                // If the constraint failed with psi_ij = false, update psi_ij = true
                psi[i][j] = false;
            }

            if uj - ui - si - (f32::from(sig[i][j]) - 1.0) * T < 0.0 {
                // If the constraint failed with sigma_ij = false, update sigma_ij = true
                sig[i][j] = false;
            }
        }

        return true;
    }
}
