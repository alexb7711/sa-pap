//=========================================================================
// Import modules
use crate::sa::data::Data;

//===============================================================================
// Declare modules
pub mod dynamic;
pub mod queue;

//===============================================================================
/// Trait to define `constraint` interfaces
//
pub trait Constraint {
    fn run(&mut self, d: &mut Data) -> bool;
}
