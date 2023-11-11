//==============================================================================
// Import modules
use crate::sa::data::Data;

//==============================================================================
// Declare modules
pub mod dynamic;
pub mod packing;

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
    fn run(d: &mut Data, i: usize, j: usize) -> bool;
}
