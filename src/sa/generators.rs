// Generators
pub mod primitives;
pub mod schedule_generator; // Create charge schedule
pub mod tweak_schedule; // Alter a charge schedule // Pool of all the SA generator primitives

//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::route::Route;

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Generator {
    fn run(&mut self, s: &mut dyn Route, c: &mut Charger);
}
