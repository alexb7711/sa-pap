//==============================================================================
// Declare modules
pub mod dynamic;
pub mod packing;

//==============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::data::Data;

//==============================================================================
/// Trait to define `constraint` interfaces
///
/// # Input
/// * d: Data for the current model
/// * i: index of the visit
/// * j: index for the queue
///
/// # Output
/// * bool: Constraint successfully applied and is true
///
pub trait Constraint {
    fn run(dat: &mut Data, ch: &mut Charger, i: usize, j: usize) -> bool;
}

//==============================================================================
/// Module that runs all the constraints
//
pub mod constraints {
    //==============================================================================
    // Import modules
    use crate::lp::constraints::dynamic::dynamic;
    use crate::lp::constraints::packing::packing;
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;

    //--------------------------------------------------------------------------
    //
    pub fn run(dat: &mut Data, ch: &mut Charger) -> bool {
        for i in 0..dat.param.N {
            for j in 0..dat.param.N {
                // If packing constraints fail
                if !packing::run(dat, ch, i, j) {
                    return false;
                }

                // If dynamic constraints fail
                if !dynamic::run(dat, ch, i, j) {
                    return false;
                }
            }
        }
        // Success
        return true;
    }
}
