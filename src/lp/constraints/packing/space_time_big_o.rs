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
        // Extract parameters
        let Q = d.param.Q;
        let S = d.param.S;

        // Extract decision variables
        let psi = &mut d.dec.psi;
        let sig = &mut d.dec.sigma;
        let v = &d.dec.v;
        let s = &d.dec.s;
        let u = &d.dec.u;

        // Constraint

        // If `i == j` set sigma and psi to false
        if i == j {
            psi[i][j] = false;
            sig[i][j] = false;
            return true;
        }

        let psi_big_o =
            (v[i] as i32 - v[j] as i32 - S as i32 - (psi[i][j] as i32 - S as i32) * Q as i32)
                as i32
                >= 0;
        let sig_big_o = u[i] - u[j] - s[j] - (f32::from(sig[i][j]) - 1.0) * Q as f32 >= 0.0;

        if i != j {
            // Check if constraint is valid with psi_ij = false
            if !(psi_big_o) {
                // If the constraint failed with psi_ij = false, update psi_ij = true
                psi[i][j] = false;
            }

            // Check if constraint is valid with sigma_ij = false
            if !(sig_big_o) {
                // If the constraint failed with sigma_ij = false, update sigma_ij = true
                sig[i][j] = false;
            }
        }

        return true;
    }
}
