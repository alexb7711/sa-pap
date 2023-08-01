//===============================================================================
// Declare submodules
pub use crate::sa::route::route_rand_generator::route_event::RouteEvent;        // Keep public for testing
use crate::sa::route::Route;
use crate::util::fileio::yaml_loader;
use yaml_rust::Yaml;

//===============================================================================
// External Crates

//===============================================================================
// Import Crates

//===============================================================================
// Import modules


//===============================================================================
// Implementation of ScheduleGenerator
#[allow(dead_code)]
pub struct RouteCSVGenerator
{
    // PUBLIC

    // PRIVATE
    config: Yaml,
    load_from_file: bool,
}

//===============================================================================
// Implementation of ScheduleCSVGenerator
impl RouteCSVGenerator
{
    //===========================================================================
    // PUBLIC

    //---------------------------------------------------------------------------
    /// Returns a schedule generator
    ///
    /// # Input
    /// * `config_path`: Path to YAML schedule config
    ///
    /// # Output
    /// * `ScheduleGenerator`
    ///
    pub fn new(load_from_file: bool, config_path: &str) -> RouteCSVGenerator
    {
        // Create new RouteGenerator
        let rg = RouteCSVGenerator
        {
            config: yaml_loader::load_yaml(config_path),
            load_from_file,
        };

        // Return Route Generator
        return rg;
    }
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
