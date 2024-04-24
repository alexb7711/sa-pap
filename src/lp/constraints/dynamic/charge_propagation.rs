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
    //==========================================================================
    /// The `update_lin_charge` function adjusts the charge time so that the
    /// BEB is not over charged using the linear battery dynamics model.
    ///
    /// # Input
    /// * dat: Data object
    /// * ch: Charger object
    /// * i: Visit index
    ///
    /// # Output
    /// * charge: Liner battery dynamics SOC estimation
    ///
    #[allow(non_snake_case)]
    fn update_lin_charge(dat: &mut Data, _ch: &mut Charger, i: usize) -> f32 {
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

    //==========================================================================
    /// The `linear_model` function determines the amount of charge supplied to
    /// the BEB for visit `i` using a linear battery dynamics model.
    ///
    /// # Input
    /// * dat: Data object
    /// * ch: Charger object
    /// * i: Visit index
    ///
    /// # Output
    /// * charge: Liner battery dynamics SOC estimation
    ///
    #[allow(non_snake_case)]
    fn linear_model(dat: &mut Data, ch: &mut Charger, i: usize) -> f32 {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Constraint

        // Extract parameters
        let Gam = &dat.param.Gam;
        let kappa = &dat.param.k;
        let r = &dat.param.r;
        let s = &dat.dec.s;

        // Calculate charge amount
        let mut charge: f32 = r[dat.dec.v[i]] * s[i];

        // Ensure the charge does not exceed the battery limit
        if !(dat.dec.eta[i] + charge <= kappa[Gam[i] as usize]) {
            charge = ChargePropagate::update_lin_charge(dat, ch, i);
        }

        return charge;
    }

    //==========================================================================
    /// The `nonlinear_model` function determines the amount of charge supplied
    /// to the BEB for visit `i` using a non-linear battery dynamics.
    ///
    /// # Input
    /// * dat: Data object
    /// * ch: Charger object
    /// * i: Visit index
    ///
    /// # Output
    /// * charge: Non-linear battery dynamics SOC estimation
    ///
    #[allow(non_snake_case)]
    fn nonlinear_model(dat: &mut Data, _ch: &mut Charger, i: usize) -> f32 {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Constraint

        // Extract parameters
        let Gam = &dat.param.Gam;
        let dt = dat.dec.s[i] * 3600.0;
        let eta = &dat.dec.eta;
        let kappa = &dat.param.k;
        let r = &dat.param.conv;
        let v = &dat.dec.v;

        // Calculate model parameters
        let abar = f32::exp(-r[v[i]] * dt);
        let bbar = abar - 1.0;

        // Calculate charge amount
        let mut soc: f32 = eta[i];

        // Calculate the new SOC
        soc = soc * abar - bbar * kappa[Gam[i] as usize];

        // println!("{} @ {}: {}", r[v[i]], dt, soc - eta[i]);

        return soc - eta[i];
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
    fn run(dat: &mut Data, ch: &mut Charger, i: usize, _j: usize) -> bool {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Constraint
        let charge: f32;

        // If the linear model is to be used
        if dat.param.model == "linear" {
            charge = ChargePropagate::linear_model(dat, ch, i);
        // Otherwise use the non-linear model
        } else {
            charge = ChargePropagate::nonlinear_model(dat, ch, i);
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
    fn _update_dec_var(data: &mut Data, ch: &mut Charger, i: usize, j: usize) {
        // Update the initial charge time
        InitFinalCharge::run(data, ch, i, j);
    }
}
