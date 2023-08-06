//===============================================================================
// Declare submodules
pub mod parse_routes;

//===============================================================================
// External Crates
use csv;
use yaml_rust::Yaml;

//===============================================================================
// Import Crates
use crate::sa::route::Route;
use crate::util::fileio::yaml_loader;

//===============================================================================
// Import modules

//===============================================================================
// Implementation of ScheduleGenerator
#[allow(dead_code)]
pub struct RouteCSVGenerator {
    // PUBLIC

    // PRIVATE
    config: Yaml,
    csv_h: csv::Reader<std::fs::File>,
}

//===============================================================================
// Implementation of ScheduleCSVGenerator
impl RouteCSVGenerator {
    //===========================================================================
    // PUBLIC

    //---------------------------------------------------------------------------
    /// Constructor that returns a CSV schedule generator
    ///
    /// # Input
    /// * `config_path`   : Path to YAML schedule config
    /// * `csv_path`      : Path to CSV file
    ///
    /// # Output
    /// * `ScheduleGenerator`
    ///
    pub fn new(config_path: &str) -> RouteCSVGenerator {
        // Create new RouteGenerator
        let rg = RouteCSVGenerator {
            config: yaml_loader::load_yaml(config_path),
            csv_h: parse_routes::read_csv(config_path),
        };

        // Return Route Generator
        return rg;
    }
}

//===============================================================================
//
impl Route for RouteCSVGenerator {
    //---------------------------------------------------------------------------
    /// Generate or load route
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `route_schedule`: The routes that the buses must adhere to
    ///
    fn run(self: &mut RouteCSVGenerator) {}
}

//===============================================================================
// TEST PRIVATE METHODS IN ROUTE GENERATOR
#[cfg(test)]
mod priv_test_route_gen {
    //use super::{RouteCSVGenerator,Route};

    //---------------------------------------------------------------------------
    //
    // fn create_object() -> RouteCSVGenerator
    // {
    //     return RouteCSVGenerator::new(false, "./src/config/schedule-test.yaml");
    // }
}
