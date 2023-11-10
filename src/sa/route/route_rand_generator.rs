//===============================================================================
// Declare submodules
// pub mod bus;
// pub mod route_event;

//===============================================================================
// External Crates
extern crate yaml_rust;

//===============================================================================
// Import Crates
use crate::sa::data::Data;
pub use std::boxed::Box;
use yaml_rust::Yaml;

//===============================================================================
// Import modules
use crate::sa::route::bus::Bus;
pub use crate::sa::route::route_event::RouteEvent; // Keep public for testing
use crate::sa::route::Route;
use crate::util::fileio::yaml_loader;
use crate::util::rand_utils;

//===============================================================================
/// Defines the structure that contains data for RouteGenerator to run
//
#[allow(dead_code)]
pub struct RouteRandGenerator {
    // PUBLIC
    pub route: Box<Vec<RouteEvent>>,
    pub data: Box<Data>,
    pub buses: Vec<Bus>,

    // PRIVATE
    config: Yaml,
    load_from_file: bool,
}

//===============================================================================
// Implementation of ScheduleGenerator
impl RouteRandGenerator {
    //===========================================================================
    // PUBLIC

    //---------------------------------------------------------------------------
    /// Returns a schedule generator
    ///
    /// # Input
    /// * `load_from_file`: Boolean that indicates to load previous schedule from file
    /// * `config_path`   : Path to YAML schedule config
    ///
    /// # Output
    /// * `ScheduleGenerator`
    ///
    pub fn new(load_from_file: bool, config_path: &str) -> RouteRandGenerator {
        // Create new RouteGenerator
        let rg = RouteRandGenerator {
            route: Box::new(Vec::new()),
            data: Box::new(Default::default()),
            buses: Vec::new(),

            config: yaml_loader::load_yaml(config_path),
            load_from_file,
        };

        // Return Route Generator
        return rg;
    }

    //---------------------------------------------------------------------------
    /// Returns a schedule generator
    ///
    /// # Input
    /// * `config_path`: Path to YAML schedule config
    ///
    /// # Output
    /// * `ScheduleGenerator`
    ///
    pub fn print_route(self) {
        for i in 0..self.route.len() {
            println!(
                "({}) ID: {} - Arrival: {} - Depart: {}",
                i, self.route[i].id, self.route[i].arrival_time, self.route[i].departure_time
            );
        }
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
    fn create_buffers(self: &mut RouteRandGenerator) {
        // Variables
        let num_bus: usize = self.config["buses"]["num_bus"].as_i64().unwrap() as usize;
        let visits: usize = self.config["buses"]["num_visit"].as_i64().unwrap() as usize;

        // Reserve memory for all buses
        self.buses.resize(num_bus, Bus::default());

        // Reserve memory for all events
        self.route.resize(visits, RouteEvent::default());
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
    fn generate_routes(self: &mut RouteRandGenerator) {
        // Variables
        let num_bus: u16 = self.config["buses"]["num_bus"].as_i64().unwrap() as u16;
        let num_visit: u16 = self.config["buses"]["num_visit"].as_i64().unwrap() as u16;
        let mut route_idx: u16 = 0;

        // Generate number of routes (events) for each bus
        let route_count: Vec<u16> = rand_utils::rand_route_count(num_bus, num_visit);

        // Loop through each bus
        for id in 0..num_bus {
            // Create event
            self.create_events(id, route_count[id as usize], route_idx);

            // Update route index. Minus one to make zero indexed
            route_idx += route_count[id as usize];
        }

        // Sort array of events by arrival time
        self.route.sort();
    }

    //---------------------------------------------------------------------------
    /// Creates a sequence of events for bus i.
    ///
    /// # Input
    /// * `id`        : ID of bus being attended to
    /// * `event_cnt` : Number of events by bus `id`
    /// * `route_idx` : Index to start appending to in route vector
    ///
    /// # Output
    /// * NONE
    ///
    fn create_events(self: &mut RouteRandGenerator, id: u16, event_cnt: u16, route_idx: u16) {
        // Variables
        let mut arrival_new: f32 = 0.0; /* Arrival time of next visit [hr]     */
        let mut arrival_old: f32; /* Arrival time of previous visit [hr] */
        let mut depart: f32; /* Departure time [hr]                 */
        let mut discharge: f32; /* Discharge of current route [KWH]    */

        // Loop through each event
        for iter in std::iter::zip(route_idx..event_cnt + route_idx, 1..=event_cnt) {
            // Extract iter (TODO: specify the type)
            let (i, j) = iter;

            // Store arrival time
            arrival_old = arrival_new.clone();

            // Check for final visit
            let final_visit: bool = if j == event_cnt { true } else { false };

            // Select departure time (based off old arrival)
            depart = self.next_depart(arrival_old, final_visit);

            // Select new arrival time
            arrival_new = self.next_arrival(j, event_cnt);

            // Calculate the amount of discharge
            discharge = self.calc_discharge(id, arrival_old, depart);

            // Append bus data
            self.add_bus_data(i as usize, id as usize, arrival_old, depart, discharge);
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
    fn next_depart(self: &mut RouteRandGenerator, arrival: f32, final_visit: bool) -> f32 {
        // Variables
        let depart: f32;

        if final_visit {
            // Set the final departure time as the time horizon
            depart = self.config["time"]["EOD"].as_f64().unwrap() as f32;
        } else {
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
    /// * `next_arrival` : Next arrival time for bus `b`
    ///
    fn next_arrival(self: &mut RouteRandGenerator, current_visit: u16, event_cnt: u16) -> f32 {
        // Variables
        let time_horizon: f32 = self.config["time"]["EOD"].as_f64().unwrap() as f32;
        let chunk: f32 = time_horizon / (event_cnt as f32);
        let next_arr: f32 = (current_visit as f32) * chunk;

        return next_arr;
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
    fn calc_discharge(
        self: &mut RouteRandGenerator,
        id: u16,
        prev_depart: f32,
        next_arrival: f32,
    ) -> f32 {
        // Variables
        let dis_rat: f32 = self.buses[id as usize].discharge_rate;

        return dis_rat * (next_arrival - prev_depart);
    }

    //---------------------------------------------------------------------------
    /// Add bus data to the route event vector
    ///
    /// # Input
    /// * `event`     : Event number
    /// * `id`        : Id of the bus
    /// * `arrival`   : Arrival time for bus `b`
    /// * `depart`    : Depart time for bus `b`
    /// * `discharge` : Amount of discharge for next route
    ///
    /// # Output
    /// * NONE
    ///
    fn add_bus_data(
        self: &mut RouteRandGenerator,
        event: usize,
        id: usize,
        arrival: f32,
        depart: f32,
        discharge: f32,
    ) {
        // Variables
        let b: &Bus = &self.buses[id];

        // Populate
        self.route[event].arrival_time = arrival;
        self.route[event].bus = b.clone();
        self.route[event].departure_time = depart;
        self.route[event].discharge = discharge;
        self.route[event].id = id as u16;
        self.route[event].route_time = depart - arrival;
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
    fn create_buses(self: &mut RouteRandGenerator) {
        // Variables
        let bat_capacity: f32 = self.config["buses"]["bat_capacity"].as_f64().unwrap() as f32;
        let dis_rat: f32 = self.config["buses"]["dis_rate"].as_f64().unwrap() as f32;
        let fc: f32 = self.config["final_charge"].as_f64().unwrap() as f32;
        let ic_ub: f32 = self.config["initial_charge"]["max"].as_f64().unwrap() as f32;
        let ic_lb: f32 = self.config["initial_charge"]["min"].as_f64().unwrap() as f32;
        let num_bus: u16 = self.config["buses"]["num_bus"].as_i64().unwrap() as u16;

        for b in 0..num_bus as usize {
            self.buses[b].bat_capacity = bat_capacity;
            self.buses[b].discharge_rate = dis_rat;
            self.buses[b].final_charge = fc;
            self.buses[b].initial_charge = rand_utils::rand_range(ic_lb, ic_ub);
        }
    }
}

//===============================================================================
//
impl Route for RouteRandGenerator {
    //---------------------------------------------------------------------------
    /// Generate or load route
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `route_schedule`: The routes that the buses must adhere to
    ///
    fn run(self: &mut RouteRandGenerator) {
        // If load from file
        if self.load_from_file {
        }
        // Otherwise generate new route
        else {
            // Create buffers
            self.create_buffers();

            // Create buses
            self.create_buses();

            // Generate
            self.generate_routes();
        }
    }

    //---------------------------------------------------------------------------
    /// Return the route data
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `route: Vector of route data
    ///
    fn get_route_events(self: &RouteRandGenerator) -> Box<Vec<RouteEvent>> {
        return self.route.clone();
    }

    //---------------------------------------------------------------------------
    /// Return the data object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `data`: Data object
    ///
    fn get_data(self: &RouteRandGenerator) -> Box<Data> {
        return self.data.clone();
    }

    //---------------------------------------------------------------------------
    /// Update the route data
    ///
    /// # Input
    /// * `route: Vector of route data
    ///
    /// # Output
    /// * NONE
    ///
    fn set_route_events(self: &mut RouteRandGenerator, r: Box<Vec<RouteEvent>>) {
        self.route = r;
    }

    //---------------------------------------------------------------------------
    /// Update the data object
    ///
    /// # Input
    /// * `data`: Data object
    ///
    /// # Output
    /// * NONE
    ///
    fn set_data(self: &mut RouteRandGenerator, d: Box<Data>) {
        self.data = d;
    }
}

//===============================================================================
// TEST PRIVATE METHODS IN ROUTE GENERATOR
#[cfg(test)]
mod priv_test_route_gen {
    use super::{Route, RouteRandGenerator};

    //---------------------------------------------------------------------------
    //
    fn create_object() -> RouteRandGenerator {
        return RouteRandGenerator::new(false, "./src/config/schedule-test.yaml");
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_next_depart() {
        // Variables
        let mut rg: RouteRandGenerator = create_object();
        let mut arrival: f32 = 1.0;
        let time_horizon: f32 = rg.config["time"]["EOD"].as_f64().unwrap() as f32;

        // Test 1
        let mut depart: f32 = rg.next_depart(arrival, false);
        assert_eq!(depart, arrival + 0.1);

        // Test 2
        arrival = 2.0;
        depart = rg.next_depart(arrival, false);
        assert_eq!(depart, arrival + 0.1);

        // Test 3
        arrival = 5.0;
        depart = rg.next_depart(arrival, false);
        assert_eq!(depart, arrival + 0.1);

        // Test 4
        arrival = 1.0;
        depart = rg.next_depart(arrival, true);
        assert_eq!(depart, time_horizon);

        // Test 5
        arrival = 5.0;
        depart = rg.next_depart(arrival, true);
        assert_eq!(depart, time_horizon);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_next_arrival() {
        let mut rg: RouteRandGenerator = create_object();

        // Test 1
        let mut next_arrival: f32 = rg.next_arrival(1, 2);
        assert_eq!(next_arrival, 12.0);

        // Test 2
        next_arrival = rg.next_arrival(2, 2);
        assert_eq!(next_arrival, 24.0);

        // Test 3
        next_arrival = rg.next_arrival(1, 5);
        assert_eq!(next_arrival, 4.8);

        // Test 4
        next_arrival = rg.next_arrival(3, 5);
        assert_eq!(next_arrival, 14.400001);

        // Test 5
        next_arrival = rg.next_arrival(4, 5);
        assert_eq!(next_arrival, 19.2);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_calc_discharge() {
        let mut rg: RouteRandGenerator = create_object();
        let dis_rat: f32 = rg.config["buses"]["dis_rate"].as_f64().unwrap() as f32;

        rg.run();

        // Test 1
        let mut discharge: f32 = rg.calc_discharge(0, 0.0, 1.0);
        assert_eq!(discharge, dis_rat * (1.0));

        // Test 2
        discharge = rg.calc_discharge(0, 1.0, 4.0);
        assert_eq!(discharge, dis_rat * (3.0));

        // Test 3
        discharge = rg.calc_discharge(1, 1.0, 4.0);
        assert_eq!(discharge, dis_rat * (3.0));

        // Test 4
        discharge = rg.calc_discharge(1, 6.0, 10.0);
        assert_eq!(discharge, dis_rat * (4.0));
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_add_bus_data() {
        let mut rg: RouteRandGenerator = create_object();
        let num_event: usize = rg.config["buses"]["num_visit"].as_i64().unwrap() as usize;
        let bat_capacity: f32 = rg.config["buses"]["bat_capacity"].as_f64().unwrap() as f32;

        rg.run();

        // Test 1
        rg.add_bus_data(0, 0, 0.0, 10.0, 15.0);
        assert_eq!(rg.route[0].arrival_time, 0.0);
        assert_eq!(rg.route[0].departure_time, 10.0);
        assert_eq!(rg.route[0].discharge, 15.0);
        assert_eq!(rg.route[0].bus.bat_capacity, bat_capacity);
        assert_eq!(rg.route.len(), num_event);

        // Test 2
        rg.add_bus_data(100, 0, 0.0, 10.0, 15.0);
        assert_eq!(rg.route[100].arrival_time, 0.0);
        assert_eq!(rg.route[100].departure_time, 10.0);
        assert_eq!(rg.route[100].discharge, 15.0);
        assert_eq!(rg.route[100].bus.bat_capacity, bat_capacity);
        assert_eq!(rg.route.len(), num_event);

        // Test 3
        rg.add_bus_data(30, 0, 0.0, 2.0, 15.4);
        assert_eq!(rg.route[30].arrival_time, 0.0);
        assert_eq!(rg.route[30].departure_time, 2.0);
        assert_eq!(rg.route[30].discharge, 15.4);
        assert_eq!(rg.route[30].bus.bat_capacity, bat_capacity);
        assert_eq!(rg.route.len(), num_event);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_create_buses() {
        let mut rg: RouteRandGenerator = create_object();
        let bat_capacity: f32 = rg.config["buses"]["bat_capacity"].as_f64().unwrap() as f32;
        let dis_rat: f32 = rg.config["buses"]["dis_rate"].as_f64().unwrap() as f32;
        let fc: f32 = rg.config["final_charge"].as_f64().unwrap() as f32;

        rg.create_buffers();
        rg.create_buses();

        // Test 1
        assert_eq!(rg.buses[0].bat_capacity, bat_capacity);
        assert_eq!(rg.buses[0].discharge_rate, dis_rat);
        assert_eq!(rg.buses[0].final_charge, fc);

        // Test 2
        assert_eq!(rg.buses[5].bat_capacity, bat_capacity);
        assert_eq!(rg.buses[5].discharge_rate, dis_rat);
        assert_eq!(rg.buses[5].final_charge, fc);

        // Test 3
        assert_eq!(rg.buses[9].bat_capacity, bat_capacity);
        assert_eq!(rg.buses[9].discharge_rate, dis_rat);
        assert_eq!(rg.buses[9].final_charge, fc);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_create_buffers() {
        let mut rg: RouteRandGenerator = create_object();

        rg.create_buffers();

        let visit_len: usize = rg.config["buses"]["num_visit"].as_i64().unwrap() as usize;
        let bus_len: usize = rg.config["buses"]["num_bus"].as_i64().unwrap() as usize;

        assert_eq!(rg.route.len(), visit_len);
        assert_eq!(rg.buses.len(), bus_len);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_generate_routes() {
        let mut rg: RouteRandGenerator = create_object();

        rg.create_buffers();
        rg.generate_routes();

        let visit_len: usize = rg.config["buses"]["num_visit"].as_i64().unwrap() as usize;

        assert_eq!(rg.route.len(), visit_len);

        // Test 1
        assert!(rg.route[0].departure_time != 0.0);
        assert!(rg.route[5].departure_time != 0.0);
        assert!(rg.route[100].departure_time != 0.0);

        // Test 2
        assert!(rg.route[0].queue == 0);
        assert!(rg.route[5].queue == 0);
        assert!(rg.route[100].queue == 0);

        // Test 3
        assert!(rg.route[0].route_time > 0.0);
        assert!(rg.route[5].route_time > 0.0);
        assert!(rg.route[100].route_time > 0.0);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_create_events() {
        let mut rg: RouteRandGenerator = create_object();

        rg.create_buffers();
        rg.create_buses();

        // Test 1
        rg.create_events(0, 1, 0);
        assert_eq!(rg.route[0].id, 0);
        assert!(rg.route[0].departure_time > 0.0);

        // Test 2
        rg.create_events(1, 2, 1);
        assert_eq!(rg.route[1].id, 1);
        assert!(rg.route[0].departure_time > 0.0);

        // Test 2
        rg.create_events(7, 1, 56);
        assert_eq!(rg.route[56].id, 7);
        assert!(rg.route[0].departure_time > 0.0);
    }
}
