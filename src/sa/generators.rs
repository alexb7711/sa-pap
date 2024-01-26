//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::route::Route;

//===============================================================================
// Declare modules
pub mod gen_new_visits; // Create new charge schedule with new_visits primitive
pub mod gen_wait_queue; // Create new charge schedule by placing in waiting queues
pub mod primitives; // Pool of all the SA generator primitives
pub mod tweak_schedule; // Alter a charge schedule slower with more certainty
pub mod tweak_schedule_quick; // Alter a charge schedule quickly uncertainly

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Generator {
    fn run(&mut self, s: &mut Box<dyn Route>, c: &mut Charger) -> bool;
}
