//===============================================================================
// Declare submodules

//===============================================================================
// External Crates

//===============================================================================
// Import Crates
pub use crate::sa::route::route_rand_generator::route_event::RouteEvent;     // Keep public for testing
use crate::sa::route::Route;

//===============================================================================
// Import modules


//===============================================================================
// Implementation of ScheduleGenerator
#[allow(dead_code)]
pub struct RouteCSVGenerator
{
    // PUBLIC

    // PRIVATE
}

//===============================================================================
//
impl Route for RouteCSVGenerator
{
    //---------------------------------------------------------------------------
    /// Generate or load route
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `route_schedule`: The routes that the buses must adhere to
    ///
    fn run(self: &mut RouteCSVGenerator)
    {
    }
}

//===============================================================================
// TEST PRIVATE METHODS IN ROUTE GENERATOR
#[cfg(test)]
mod priv_test_route_gen
{
    //use super::{RouteCSVGenerator,Route};

    //---------------------------------------------------------------------------
    //
    // fn create_object() -> RouteCSVGenerator
    // {
    //     return RouteCSVGenerator::new(false, "./src/yaml/schedule-test.yaml");
    // }
}
