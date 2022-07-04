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
    /// * `config`: Parsed route YAML config file
    ///
    /// # Output
    /// * `route_schedule`: The routes that the buses must adhere to
    ///
    fn generate_routes(self: &mut RouteGenerator)
    {
        // Variables
        let num_bus   : i64 = self.config["buses"]["num_bus"].as_i64().unwrap();
        let num_visit : i64 = self.config["buses"]["num_visit"].as_i64().unwrap();

        // Generate number of routes (events) for each bus
        let route_count: Vec<i64> = rand_utils::rand_route_count(num_bus, num_visit);

        // Loop through each bus
        for id in 0..num_bus
        {
            self.create_events(id, route_count[id as usize]);
        }

        // Sort array of events by arrival time
    }

    //---------------------------------------------------------------------------
    /// Creates a sequence of events for bus i.
    ///
    /// # Input
    /// * `id`        : ID of bus being attended to
    /// * `event_cnt` : Number of events by bus `id`
    ///
    /// # Output
    /// * ``
    ///
    fn create_events(self      : &mut RouteGenerator,
                     id        : i64,
                     event_cnt : i64)
    {
        // Variables
        let mut arrival_old : u16  = 0; /* Arrival time of previous visit [hr] */
        let mut arrival_new : u16  = 0; /* Arrival time of next visit [hr]     */
        let mut depart      : u16  = 0; /* Departure time [hr]                 */
        let mut discharge   : u16  = 0; /* Discharge of current route [KWH]    */

        for i in 0..event_cnt
        {
            // Store arrival time
            arrival_old = arrival_new;

            // Check for final visit
            let final_visit : bool = if i == event_cnt-1 {true} else {false};

            // Select departure time (based off old arrival)
            depart = self.next_depart(arrival_old, final_visit);

            // Select new arrival time
            arrival_new = self.next_arrival();

            // Calculate the amount of discharge
            discharge = self.calc_discharge(arrival_old, depart);

            // Append bus data
            self.append_bus_data(id, arrival_old, depart, discharge);
        }
    }

    //---------------------------------------------------------------------------
    /// TODO
    fn next_depart(self: &mut RouteGenerator, arrival: u16, final_visit: bool) -> u16
    {
        return 0;
    }

    //---------------------------------------------------------------------------
    /// TODO
    fn next_arrival(self: &mut RouteGenerator) -> u16
    {
        return 0;
    }

    //---------------------------------------------------------------------------
    /// TODO
    fn calc_discharge(self         : &mut RouteGenerator,
                      prev_depart  : u16,
                      next_arrival : u16) -> u16
    {
        return 0;
    }

    //---------------------------------------------------------------------------
    /// TODO
    fn append_bus_data(self      : &mut RouteGenerator,
                       id        : i64,
                       arrival   : u16,
                       depart    : u16,
                       discharge : u16)
    {
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
