//===============================================================================
// Import modules
use crate::sa::data::Data;

//===============================================================================
// Declare modules

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Plotter {
    fn plot(&mut self, name: String, d: &mut Box<Data>) -> bool;
}
