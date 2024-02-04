//===============================================================================
// Import developed modules
use crate::lp::constraints::dynamic::init_final_charge::InitFinalCharge;
use crate::lp::constraints::Constraint;
use crate::sa::charger::Charger;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct ChargePropagate {}

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
impl Constraint for ChargePropagate {
    fn run(dat: &mut Data, ch: &mut Charger, i: usize, j: usize) -> bool {
        // Update parameters
        ChargePropagate::update_dec_var(dat, ch, i, j);

        // Extract parameters
        let Q = dat.param.Q;
        let Gam = &dat.param.Gam;
        let gam = &dat.param.gam;
        let r = &dat.param.r;
        let kappa = &dat.param.k;
        let l = &dat.param.l;

        // Extract decision variables
        let eta = &mut dat.dec.eta;
        let w = &dat.dec.w;
        let s = &mut dat.dec.s;

        // Constraint

        // Calculate charge amount
        let mut charge: f32 = (0..Q).map(|q| f32::from(w[i][q]) * r[q] * s[i]).sum();

        // Ensure the charge does not exceed the battery limit
        if !(eta[i] + charge <= kappa[Gam[i] as usize]) {
            // Adjust charge times

            // Retrieve the charger speed
            let charge_rate: f32 = (0..Q).map(|q| f32::from(w[i][q]) * r[q]).sum();

            // Adjust the charge time such that the BEB is at maximum charge
            // and the schedule does not fail
            //
            // Units: Kwh * (hr / Kwh) = hr
            //
            s[i] = (kappa[Gam[i] as usize] - eta[i]) / charge_rate;

            // Update the charge
            charge = (0..Q).map(|q| f32::from(w[i][q]) * r[q] * s[i]).sum();
        }

        // If the BEB has another visit
        if gam[i] >= 0 {
            // Update the next charge
            eta[gam[i] as usize] = eta[i] + charge - l[i];
        }

        return true;
    }
}

//==============================================================================
/// Implementation of helper functions for `ChargePropogation`
//
impl ChargePropagate {
    //--------------------------------------------------------------------------
    /// The `update_dec_var` function updates the decision variables associated
    /// with the `ChargePropogation` constraints.
    ///
    /// # Input
    /// * data: Simulated annealing data object.
    /// * i: index of the visit
    /// * j: index for the queue
    ///
    /// # Output
    /// * NONE
    ///
    fn update_dec_var(data: &mut Data, ch: &mut Charger, i: usize, j: usize) {
        // Update the initial charge time
        InitFinalCharge::run(data, ch, i, j);
    }
}
