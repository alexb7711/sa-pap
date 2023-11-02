//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct ChargePropogation {}

//===============================================================================
/// Implementation of `Constraint` for `ChargePropogation` structure.
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
impl Constraint for ChargePropogation {
    fn run(&mut self, d: &mut Data, i: usize, j: usize) -> bool {
        // Extract parameters
        let Q = d.param.Q;
        let gam = &d.param.gam;
        let r = &d.param.r;
        let l = &d.param.l;

        // Extract decision variables
        let eta = &mut d.dec.eta;
        let w = &d.dec.w;

        // Constraint

        if gam[i] >= 0 {
            // Update the next charge
            let charge: f32 = (0..Q).map(|q| f32::from(w[i][q]) * r[q]).sum();
            eta[gam[i] as usize] = eta[i] + charge - l[i];
        }

        return true;
    }
}
