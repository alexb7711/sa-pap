//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct InitFinalCharge {}

//===============================================================================
/// Implementation of `Constraint` for `InitFinalCharge` structure.
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
impl Constraint for InitFinalCharge {
    fn run(d: &mut Data, i: usize, _: usize) -> bool {
        // Extract parameters
        let Gam = &d.param.Gam;
        let alpha = &d.param.alpha;
        let beta = &d.param.beta;
        let kappa = &d.param.k;

        // Extract decision variables
        let eta = &mut d.dec.eta;

        // Constraint

        // If the current visit is the initial visit for BEB `i`
        if alpha[i] > 0.0 {
            // Assign the initial charge
            eta[i] = alpha[i] * kappa[Gam[i] as usize];
        }

        // If the current visit is the final visit for BEB `i`
        if beta[i] > 0.0 {
            // Ensure that the final charge is above the specified threshold
            if !(eta[i] >= beta[i] * kappa[Gam[i] as usize]) {
                return false;
            }
        }

        return true;
    }
}
