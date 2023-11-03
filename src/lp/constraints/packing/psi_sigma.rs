//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct PsiSigma {}

//===============================================================================
/// Implementation of `Constraint` for `PsiSigma` structure.
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
impl Constraint for PsiSigma {
    fn run(&mut self, d: &mut Data, i: usize, j: usize) -> bool {
        // Extract decision variables
        let psi = &d.dec.psi;
        let sig = &d.dec.sigma;

        // Constraints

        // Check the spatial ordering
        if !(psi[i][j] as usize + psi[j][i] as usize <= 1) {
            return false;
        }

        // Check the temporal ordering
        if !(sig[i][j] as usize + sig[j][i] as usize <= 1) {
            return false;
        }

        // Check the spatiotemporal ordering
        if !(psi[i][j] as usize + psi[j][i] as usize + sig[i][j] as usize + sig[j][i] as usize >= 1)
        {
            return false;
        }

        return true;
    }
}
