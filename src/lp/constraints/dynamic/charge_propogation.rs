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
    fn run(d: &mut Data, i: usize, _: usize) -> bool {
        // Extract parameters
        let Q = d.param.Q;
        let Gam = &d.param.Gam;
        let gam = &d.param.gam;
        let nu = &d.param.nu;
        let r = &d.param.r;
        let kappa = &d.param.k;
        let l = &d.param.l;

        // Extract decision variables
        let eta = &mut d.dec.eta;
        let w = &d.dec.w;

        // Constraint

        // Calculate charge amount
        let charge: f32 = (0..Q).map(|q| f32::from(w[i][q]) * r[q]).sum();

        // Ensure the charge does not exceed the battery limit
        if !(eta[i] + charge <= kappa[Gam[i] as usize]) {
            return false;
        }

        // Ensure the charge does not go below the minimum allowed threshold
        if !(eta[i] + charge - l[i] >= nu * kappa[Gam[i] as usize]) {
            return false;
        }

        // If the BEB has another visit
        if gam[i] >= 0 {
            // Update the next charge
            eta[gam[i] as usize] = eta[i] + charge - l[i];
        }

        return true;
    }
}
