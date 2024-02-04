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
        // Update sigma/psi decision variables
        SpaceTimeBigO::run(dat, i, j);
        SpaceTimeBigO::run(dat, j, i);

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
    fn _update_dec_var(dat: &mut Data, i: usize, j: usize) {
        // Update the service time
        ServiceTime::run(dat, i, j);

        // Update sigma/psi decision variables
        SpaceTimeBigO::run(dat, i, j);
        SpaceTimeBigO::run(dat, j, i);
    }
}
