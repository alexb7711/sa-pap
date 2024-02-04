//===============================================================================
// Declare modules
pub mod psi_sigma;
pub mod service_time;
pub mod space_time_big_o;
pub mod valid_init_dep_end_time;

//==============================================================================
/// Module that runs all the packing constraints
//
pub mod packing {
    //==========================================================================
    // Import modules
    use crate::lp::constraints::packing::psi_sigma::PsiSigma;
    use crate::lp::constraints::packing::valid_init_dep_end_time::ValidInitDepEndTimes;
    use crate::lp::constraints::Constraint;
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;

    //--------------------------------------------------------------------------
    //
    pub fn run(dat: &mut Data, ch: &mut Charger, i: usize, j: usize) -> bool {
        if !PsiSigma::run(dat, ch, i, j) {
            return false;
        }

        if !ValidInitDepEndTimes::run(dat, ch, i, j) {
            return false;
        }

        // Indicate success
        return true;
    }
}
