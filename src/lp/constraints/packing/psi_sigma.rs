//==============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::data::Data;

//==============================================================================
/// Structure defining the information to calculate service time
//
pub struct PsiSigma {}

//==============================================================================
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
        // Update decision variables
        self.update_dec_var(d);

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

//==============================================================================
/// Implementation of helper functions for `PsiSigma`
//
impl PsiSigma {
    //--------------------------------------------------------------------------
    /// The `update_dec_var` function updates the decision variables associated
    /// with the `PsiSigma` constraints.
    ///
    /// # Input
    /// * data: Simulated annealing data object.
    ///
    /// # Output
    /// * NONE
    ///
    fn update_dec_var(self: &mut PsiSigma, data: &mut Data) {
        // Variables
        let psi = &data.dec.psi;
        let sig = &data.dec.sigma;

        // For every visit `i`
        for i in 0..data.param.N {
            // For every visit `j`
            for j in i..data.param.N {
                // Indicate that visit `i` arrives before `j`

                // Indicate that visit `i` is below ``
            }
        }
    }
}
