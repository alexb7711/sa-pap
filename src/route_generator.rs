// Public Crates
extern crate yaml_rust;

// Import Crates
use yaml_rust::Yaml;

// My modules

// Import modules
use crate::util::fileio::yaml_loader;
pub use crate::util::traits::Generator;

//===============================================================================
// Structure for route
#[allow(dead_code)]
#[derive(Default)]
/// Defines the structure that contains the route data
pub struct RouteEvent
{
    id: i16,
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

        // Reserve memory for all visits
        self.route.reserve_exact(visits);
        println!("Visits: {}\nCapacity: {}", visits, self.route.capacity());
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
    {}
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
