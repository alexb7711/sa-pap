
// Generators

// Public
pub mod route_csv_generator;
pub mod route_rand_generator;
pub mod route_event;                                                            // Keep public for testing

// Private
mod bus;

// Public imports
use std::boxed::Box;

// Developed imports
use crate::sa::data::Data;
use crate::sa::route::route_event::RouteEvent;                              // Keep public for testing

//===============================================================================
/// Trait to define `Route` interfaces
//
pub trait Route {
    fn run(&mut self);
    fn get_route_data(&self) -> Box<&Vec<RouteEvent>>;
    fn get_data(&self) -> Box<&Data>;
}
