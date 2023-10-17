// Generators
pub mod gen_new_visits; // Create new charge schedule with new_visits primitive
pub mod gen_wait_queue; // Create new charge schedule by placing in waiting queues
pub mod primitives;
pub mod tweak_schedule; // Alter a charge schedule // Pool of all the SA generator primitives

//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::route::Route;

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Generator {
    fn run(&mut self, s: &mut dyn Route, c: &mut Charger) -> bool;
}
