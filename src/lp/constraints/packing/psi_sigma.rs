//==============================================================================
// Import developed modules
use crate::lp::constraints::packing::service_time::ServiceTime;
use crate::lp::constraints::packing::space_time_big_o::SpaceTimeBigO;
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
    fn run(dat: &mut Data, i: usize, j: usize) -> bool {
        // Update decision variables
        PsiSigma::update_dec_var(dat, i, j);

        // Extract decision variables
        let psi = &dat.dec.psi;
        let sig = &dat.dec.sigma;

        // Constraints

        // Ignore the cases where i == j
        if i == j {
            return true;
        }

        // Check the spatial ordering
        if !(psi[i][j] as usize + psi[j][i] as usize <= 1) {
            println!("Visit {}", i);
            println!("{} + {} > 1", psi[i][j], psi[j][i]);
            println!(
                "I: i: {}, Gam: {}, v: {}",
                i, dat.param.Gam[i], dat.dec.v[i]
            );
            println!("j: j: {} Gam: {} v: {}", j, dat.param.Gam[i], dat.dec.v[j]);
            println!("psi_sigma.rs: PSI > 1");
            return false;
        }

        // Check the temporal ordering
        if !(sig[i][j] as usize + sig[j][i] as usize <= 1) {
            println!("Visit {}", i);
            println!("{} + {} > 1", sig[i][j], sig[j][i]);
            println!(
                "I: i: {}, Gam: {},  u: {}, d: {}",
                i, dat.param.Gam[i], dat.dec.u[i], dat.dec.d[i]
            );
            println!(
                "j: j: {} Gam: {} u: {}, d: {}",
                j, dat.param.Gam[i], dat.dec.u[j], dat.dec.d[j]
            );
            println!("psi_sigma.rs: SIGMA > 1");
            return false;
        }

        // Check the spatiotemporal ordering
        if !(psi[i][j] as usize + psi[j][i] as usize + sig[i][j] as usize + sig[j][i] as usize >= 1)
        {
            println!("Visit {}", i);
            println!(
                "{} + {} + {} + {} <= 1",
                psi[i][j], psi[j][i], sig[i][j], sig[j][i]
            );
            println!(
                "I: i: {}, Gam: {}, v: {}, u: {}, d: {}",
                i, dat.param.Gam[i], dat.dec.v[i], dat.dec.u[i], dat.dec.d[i]
            );
            println!(
                "j: j: {} Gam: {} v: {}, u: {}, d: {}",
                j, dat.param.Gam[i], dat.dec.v[j], dat.dec.u[j], dat.dec.d[j]
            );
            println!("psi_sigma.rs: SIGMA+PSI < 1");
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
    fn update_dec_var(data: &mut Data, i: usize, j: usize) {
        // Update the service time
        ServiceTime::run(data, i, j);

        // Update sigma/psi decision variables
        SpaceTimeBigO::run(data, i, j);
        SpaceTimeBigO::run(data, j, i);
    }
}
