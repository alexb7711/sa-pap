//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct BilinearLinearization {}

//===============================================================================
/// Implementation of `Constraint` for `BilinearLinearization` structure.
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
impl Constraint for BilinearLinearization {
    fn run(&mut self, d: &mut Data, i: usize, j: usize) -> bool {
        // Extract parameters
        let Gam = d.param.Gam;
        let M = d.param.T;
        let Q = d.param.Q;
        let gam = d.param.gam;

        // Extract decision variables
        let w = d.dec.w;
        let s = d.dec.s;
        let g = d.dec.g;

        // Constraint

        // Iterate through each charger queue
        for q in 0..Q {
            if !(g[i][q] <= s[i]) {
                return false;
            }

            if !(g[i][q] >= s[i] - (1.0 - f32::from(w[i][q])) * M) {
                return false;
            }

            if !(g[i][q] <= M * f32::from(w[i][q])) {
                return false;
            }

            if !(g[i][q] >= 0.0) {
                return false;
            }
        }

        return true;
    }
}
