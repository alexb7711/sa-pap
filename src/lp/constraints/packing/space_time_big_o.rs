//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::charger::Charger;
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
    fn run(dat: &mut Data, _ch: &mut Charger, i: usize, j: usize) -> bool {
        // Extract decision variables
        let psi = &mut dat.dec.psi;
        let sig = &mut dat.dec.sigma;

        // If `i == j` set sigma and psi to false
        if i == j {
            psi[i][j] = false;
            sig[i][j] = false;
            return true;
        // Otherwise `i != j`
        } else {
            // Extract parameters
            let Q = dat.param.Q as i32;
            let T = dat.param.T;
            let S = dat.param.S as i32;

            // Extract decision variables
            let vi = dat.dec.v[i] as i32;
            let vj = dat.dec.v[j] as i32;
            let di = dat.dec.d[i];
            let uj = dat.dec.u[j];

            // Default to true
            psi[i][j] = true;
            sig[i][j] = true;

            // Calculate constraints
            // If the constraint failed with psi_ij = false, update psi_ij = true
            if !((vj - vi - S - (psi[i][j] as i32 - S) * Q) >= 0) {
                psi[i][j] = false;
            }

            // If the constraint failed with sigma_ij = false, update sigma_ij = true
            if !(uj - di - (f32::from(sig[i][j]) - 1.0) * T >= 0.0) {
                sig[i][j] = false;
            }
        }

        return true;
    }
}
