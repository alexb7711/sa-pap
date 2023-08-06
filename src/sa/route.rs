// Generators
pub mod route_csv_generator;
pub mod route_rand_generator;

//===============================================================================
/// Trait to define `Route` interfaces
//
pub trait Route {
    fn run(&mut self);
}
