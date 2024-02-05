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
/// * run_constr: Boolean to run packing constraints. If false, only update the
///               charge propagation
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
    pub fn run(run_constr: bool, dat: &mut Data, ch: &mut Charger, i: usize, j: usize) -> bool {
        // If packing constraints fail
        if run_constr && !packing::run(dat, ch, i, j) {
            println!("Packing");
            return false;
        }

        // If dynamic constraints fail
        if !dynamic::run(dat, ch, i, j) {
            println!("Dynamic");
            return false;
        }

        // Success
        return true;
    }
}
