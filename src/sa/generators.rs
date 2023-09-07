// Generators
pub mod schedule_generator;                                                   // Create charge schedule
pub mod tweak_schedule;                                                       // Alter a charge schedule
pub mod primitives;                                                           // Pool of all the SA generator primitives

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Generator
{
    fn run(&mut self);
}
