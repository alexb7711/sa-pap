// Generators

// Public
pub mod route_csv_generator;
pub mod route_event;
pub mod route_rand_generator; // Keep public for testing

// Private
mod bus;

// Public imports
use std::boxed::Box;

// Developed imports
use crate::sa::data::Data;
use crate::sa::route::route_event::RouteEvent; // Keep public for testing

//===============================================================================
/// Trait to define `Route` interfaces
//
pub trait Route {
    fn run(&mut self);

    // Getters
    fn get_route_events(&mut self) -> Box<&mut Vec<RouteEvent>>;
    fn get_data(&mut self) -> Box<Data>;

    // Setters
    fn set_route_events(&mut self, b: Box<&mut Vec<RouteEvent>>);
    fn set_data(&mut self, d: Box<Data>);

    // Update
    fn update_route_events(&mut self);
    fn update_milp_data(&mut self);
}
