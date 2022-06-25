// Public Crates
extern crate yaml_rust;

// Import Crates
use yaml_rust::Yaml;

// Import modules
pub use crate::util::traits::Generator;
use crate::util::fileio::yaml_loader;
use crate::util::rand_utils;

//===============================================================================
// Structure for route
#[allow(dead_code)]
#[derive(Default)]
/// Defines the structure that contains the route data
pub struct RouteEvent
{
    arrival_time   : u64,
    attach_time    : u64,
    departure_time : u64,
    detatch_time   : u64,
    discharge_rate : u64,
    id             : u64,
    queue          : u64,
    route_time     : u64
}

//===============================================================================
// Structure for RouteGenerator
#[allow(dead_code)]
/// Defines the structure that contains data for RouteGenerator to run
pub struct RouteGenerator
{
    // PUBLIC
    pub route: Vec<RouteEvent>,

    // PRIVATE
    config: Yaml,
    load_from_file: bool,
}

//===============================================================================
// Implementation of ScheduleGenerator
impl RouteGenerator
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
    pub fn new(load_from_file: bool, config_path: &str) -> RouteGenerator
    {
        // Create new RouteGenerator
        let rg = RouteGenerator
        {
            route: Vec::new(),

            config: yaml_loader::load_yaml(config_path),
            load_from_file: load_from_file,
        };

        // Return Route Generator
        return rg;
    }

    //===========================================================================
    // PRIVATE

    //---------------------------------------------------------------------------
    /// Create the `Route` structure to store route data
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn create_route_struct(self: &mut RouteGenerator)
    {
        // Variables
        let visits: usize = self.config["buses"]["num_visit"].as_i64().unwrap() as usize;

        // Reserve memory for all events
        self.route.reserve_exact(visits);
    }

    //---------------------------------------------------------------------------
    /// Generates the `Route` structure data and populates it
    ///
    /// # Input
    /// * config: Parsed route YAML config file
    ///
    /// # Output
    /// * `route_schedule`: The routes that the buses must adhere to
    ///
    fn generate_routes(self: &mut RouteGenerator)
    {
        // Variables
        let num_bus: i64   = self.config["buses"]["num_bus"].as_i64().unwrap();
        let num_visit: i64 = self.config["buses"]["num_visit"].as_i64().unwrap();

        // Generate number of routes for each bus
        let route_count: Vec<i64> = rand_utils::rand_route_count(num_bus, num_visit);
    }
}

//===============================================================================
//
impl Generator for RouteGenerator
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
    fn run(self: &mut RouteGenerator)
    {
        // If load from file
        if self.load_from_file
        {}
        // Otherwise generate new route
        else
        {
            // Create buffer
            RouteGenerator::create_route_struct(self);

            // Generate
            RouteGenerator::generate_routes(self);
        }
    }
}
