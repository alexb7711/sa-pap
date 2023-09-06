// Generators
pub mod schedule_generator;                     // Create charge schedule
pub mod tweak_schedule;                         // Alter a charge schedule

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Generator
{
    fn run(&mut self);
}
