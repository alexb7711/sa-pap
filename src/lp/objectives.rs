//=========================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::data::Data;

//===============================================================================
// Declare modules
pub mod std_obj;

//===============================================================================
/// Trait to define `objective` interfaces
//
pub trait Objective {
    fn run(dat: &mut Data, ch: &mut Charger) -> (bool, f64);
}
