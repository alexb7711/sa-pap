//===============================================================================
// Import developed modules
use crate::lp::constraints::dynamic::init_final_charge::InitFinalCharge;
use crate::lp::constraints::Constraint;
use crate::sa::charger::Charger;
use crate::sa::data::Data;
use crate::sa::generators::primitives::EPSILON;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct ChargePropagate {}

//===============================================================================
/// Implementation of `Constraint` for `ChargePropogation` structure.
///
impl ChargePropagate {
    #[allow(non_snake_case)]
    fn update_charge(dat: &mut Data, ch: &mut Charger, i: usize) -> f32 {
        // Extract parameters
        let Gam = &dat.param.Gam;
        let r = &dat.param.r;
        let kappa = &dat.param.k;

        // Extract decision variables
        let eta = &mut dat.dec.eta;
        let v = &dat.dec.v;
        let s = &mut dat.dec.s;
        let u = &mut dat.dec.u;
        let d = &mut dat.dec.d;

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Adjust charge times

        // Retrieve the charger speed
        // let charge_rate: f32 = (0..Q).map(|q| f32::from(w[i][q]) * r[q]).sum();
        let charge_rate: f32 = r[v[i]];

        // Store the original charge duration
        let l_s = s[i];

        // Adjust the charge time such that the BEB is at maximum charge
        // and the schedule does not fail
        //
        // Units: Kwh * (hr / Kwh) = hr
        s[i] = (kappa[Gam[i] as usize] - eta[i]) / charge_rate;

        // Ensure charge time is non-zero
        if s[i] == 0.0 {
            s[i] = EPSILON;
        }
        // Update initial and final charge times. Choose to move u and d
        // closer together by (s_old - s_new) / 2
        let s_diff = (l_s - s[i]) / 2.0;
        u[i] += s_diff;
        d[i] -= s_diff;

        // If the update causes the time ordering to flip
        if u[i] > d[i] {
            // Update so that the assignment is valid
            let utmp = u[i].clone();
            u[i] = d[i];
            d[i] = utmp;
        } else if u[i] == d[i] {
            d[i] += EPSILON;
        }

        // Update the charge
        return r[v[i]] * s[i];
    }
}

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
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Update parameters
        ChargePropagate::update_dec_var(dat, ch, i, j);

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Constraint

        // Extract parameters
        let Gam = &dat.param.Gam;
        let kappa = &dat.param.k;
        let r = &dat.param.r;
        let s = &dat.dec.s;

        // Calculate charge amount
        // let mut charge: f32 = (0..Q).map(|q| f32::from(w[i][q]) * r[q] * s[i]).sum();
        let mut charge: f32 = r[dat.dec.v[i]] * s[i];

        // Ensure the charge does not exceed the battery limit
        if !(dat.dec.eta[i] + charge <= kappa[Gam[i] as usize]) {
            charge = ChargePropagate::update_charge(dat, ch, i);
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Constraint

        // Extract parameters
        let gam = &dat.param.gam;
        let l = &dat.param.l;

        // Extract decision variables
        let eta = &mut dat.dec.eta;

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
