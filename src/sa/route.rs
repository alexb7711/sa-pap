// Generators

// Public
pub mod route_csv_generator;
pub mod route_rand_generator;
pub mod route_event;                                                            // Keep public for testing

// Private
mod bus;

//===============================================================================
/// Trait to define `Route` interfaces
//
pub trait Route {
    fn run(&mut self);
}
