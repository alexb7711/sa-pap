// Generators
pub mod route_rand_generator;                        // Create random routes
pub mod route_csv;                              // Create route from CSV file

//===============================================================================
/// Trait to define `Route` interfaces
//
pub trait Route
{
    fn run(&mut self);
}
