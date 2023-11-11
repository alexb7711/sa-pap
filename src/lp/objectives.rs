//=========================================================================
// Import modules
use crate::sa::data::Data;

//===============================================================================
// Declare modules
pub mod std_obj;

//===============================================================================
/// Trait to define `objective` interfaces
//
pub trait Objective {
    fn run(d: &mut Data) -> f64;
}
