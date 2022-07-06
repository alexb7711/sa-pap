// Public Crates
extern crate yaml_rust;

// Import Crates
use yaml_rust::Yaml;
use std::cell::RefCell;

// Import modules
pub use crate::util::traits::Generator;
use crate::util::fileio::yaml_loader;
use crate::util::rand_utils;

//===============================================================================
// Structure for buses
#[allow(dead_code)]
#[derive(Default,Clone)]
/// Defines the structure that contains the bus data
pub struct Bus
{
    bat_capacity   : f32,
    initial_charge : f32,
    discharge_rate : f32,
    final_charge   : f32,
}

//===============================================================================
// Structure for route
#[allow(dead_code)]
#[derive(Default)]
/// Defines the structure that contains the route data
pub struct RouteEvent
{
    // Parameters
    arrival_time   : f32,
    bus            : Bus,
    departure_time : f32,
    discharge      : f32,
    id             : u16,
    route_time     : f32,

    // Decision variables
    attach_time    : f32,
    detatch_time   : f32,
    queue          : u16,
}

//===============================================================================
// Structure for RouteGenerator
#[allow(dead_code)]
/// Defines the structure that contains data for RouteGenerator to run
pub struct RouteGenerator
{
    // PUBLIC
    pub route: RefCell<Vec<RouteEvent>>,
    pub buses: RefCell<Vec<Bus>>,

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
            route: RefCell::new(Vec::new()),
            buses: RefCell::new(Vec::new()),

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
    fn create_buffers(self: &mut RouteGenerator)
    {
        // Variables
        let num_bus: usize = self.config["buses"]["num_bus"].as_i64().unwrap() as usize;
        let visits : usize = self.config["buses"]["num_visit"].as_i64().unwrap() as usize;

        // Reserve memory for all buses
        self.buses.borrow_mut().reserve_exact(num_bus);

        // Reserve memory for all events
        self.route.borrow_mut().reserve_exact(visits);
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
        let num_bus   : u16 = self.config["buses"]["num_bus"].as_i64().unwrap() as u16;
        let num_visit : u16 = self.config["buses"]["num_visit"].as_i64().unwrap() as u16;

        // Generate number of routes (events) for each bus
        let route_count: Vec<u16> = rand_utils::rand_route_count(num_bus, num_visit);

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
                     id        : u16,
                     event_cnt : u16)
    {
        // Variables
        let mut arrival_new : f32 = 0.0; /* Arrival time of next visit [hr]     */
        let mut arrival_old : f32;       /* Arrival time of previous visit [hr] */
        let mut depart      : f32;       /* Departure time [hr]                 */
        let mut discharge   : f32;       /* Discharge of current route [KWH]    */

        // Loop through each event
        for i in 0..event_cnt
        {
            // Store arrival time
            arrival_old = arrival_new;

            // Check for final visit
            let final_visit : bool = if i == event_cnt-1 {true} else {false};

            // Select departure time (based off old arrival)
            depart = self.next_depart(arrival_old, final_visit);

            // Select new arrival time
            arrival_new = self.next_arrival(i, event_cnt);

            // Calculate the amount of discharge
            discharge = self.calc_discharge(id, arrival_old, depart);

            // Append bus data
            self.append_bus_data(i as usize, id as usize, arrival_old, depart, discharge);
        }
    }

    //---------------------------------------------------------------------------
    /// Generate a random rest time between `min_rest` and `max_rest`. This
    /// method also adds on the arrival time to return the next departure time
    /// in [hr].
    ///
    /// # Input
    /// * `arrival`     : Arrival time for bus
    /// * `final_visit` : Flag to indicate bus's last visit
    ///
    /// # Output
    /// * `depart` : Departure time
    ///
    fn next_depart(self: &mut RouteGenerator, arrival: f32, final_visit: bool) -> f32
    {
        // Variables
        let depart: f32;

        if final_visit
        {
            // Set the final departure time as the time horizon
            depart = self.config["time_horizon"].as_i64().unwrap() as f32;
        }
        else
        {
            let min_rest: f32 = self.config["buses"]["min_rest"].as_f64().unwrap() as f32;
            let max_rest: f32 = self.config["buses"]["max_rest"].as_f64().unwrap() as f32;

            // Randomly select a value between min_rest and max_rest
            depart = arrival + rand_utils::rand_range(min_rest, max_rest);
        }

        return depart;
    }

    //---------------------------------------------------------------------------
    /// Generates and returns the next arrival time for bus `b`.
    ///
    /// # Input
    /// * `current_visit` : Represents the current visit number
    /// * `event_cnt`     : Represents total number of visits
    ///
    /// # Output
    /// *
    ///
    fn next_arrival(self          : &mut RouteGenerator,
                    current_visit : u16,
                    event_cnt     : u16) -> f32
    {
        // Variables
        let time_horizon : f32 = self.config["time_horizon"].as_i64().unwrap() as f32;
        let chunk        : f32 = time_horizon/(event_cnt as f32);

        return (current_visit as f32)*chunk;
    }

    //---------------------------------------------------------------------------
    /// Calculate the average discharge of a bus on route in [KWH]
    ///
    /// # Input
    /// * `id`           : Id of the bus
    /// * `prev_depart`  : Previous departure time [HR]
    /// * `next_arrival` : Next arrival time [HR]
    ///
    /// # Output
    /// * `discharge`: Average discharge of bus on route in [KWH]
    ///
    fn calc_discharge(self         : &mut RouteGenerator,
                      id           : u16,
                      prev_depart  : f32,
                      next_arrival : f32) -> f32
    {
        // Variables
        let bat_capacity: f32 = self.buses.get_mut()[id as usize].bat_capacity;

        return bat_capacity * (next_arrival - prev_depart);
    }

    //---------------------------------------------------------------------------
    /// TODO
    fn append_bus_data(self      : &mut RouteGenerator,
                       event     : usize,
                       id        : usize,
                       arrival   : f32,
                       depart    : f32,
                       discharge : f32)
    {
        // Variables
        let b : &Bus        = &self.buses.borrow_mut()[id];

        // Populate
        self.route.borrow_mut().insert(event, RouteEvent::default());
        self.route.borrow_mut()[event].arrival_time   = arrival;
        self.route.borrow_mut()[event].bus            = b.clone();
        self.route.borrow_mut()[event].departure_time = depart;
        self.route.borrow_mut()[event].discharge      = discharge;
        self.route.borrow_mut()[event].id             = id as u16;
        self.route.borrow_mut()[event].route_time     = depart - arrival;
    }

    //---------------------------------------------------------------------------
    /// Create the fleet of buses and assign some of their properties
    ///
    /// # Input
    /// * NONE
    ///
    /// # Ouptut
    /// * NONE
    ///
    fn create_buses(self : &mut RouteGenerator)
    {
        // Variables
        let bat_capacity : f32        = self.config["buses"]["bat_capacity"].as_f64().unwrap() as f32;
        let dis_rat      : f32        = self.config["buses"]["dis_rate"].as_f64().unwrap() as f32;
        let fc           : f32        = self.config["final_charge"].as_f64().unwrap() as f32;
        let ic           : &Vec<Yaml> = self.config["initial_charge"].as_vec().unwrap();
        let num_bus      : u16        = self.config["buses"]["num_bus"].as_i64().unwrap() as u16;

        for b in 0..num_bus as usize
        {
            self.buses.get_mut().insert(b, Bus::default());
            self.buses.get_mut()[b].bat_capacity   = bat_capacity;
            self.buses.get_mut()[b].discharge_rate = dis_rat;
            self.buses.get_mut()[b].final_charge   = fc;
            self.buses.get_mut()[b].initial_charge = rand_utils::rand_range(ic[0].as_f64().unwrap() as f32, ic[1].as_f64().unwrap() as f32);
        }
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
            // Create buffers
            self.create_buffers();

            // Create buses
            self.create_buses();

            // Generate
            self.generate_routes();
        }
    }
}
