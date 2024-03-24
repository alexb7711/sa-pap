#![allow(non_snake_case)]

//===============================================================================
// Declare submodules
pub mod parse_routes;

//===============================================================================
// Standard library
use csv;
use std::boxed::Box;
use std::collections::HashMap;
use yaml_rust::Yaml;

//===============================================================================
// Import modules
use crate::sa::data::Data;
use crate::sa::route::bus::Bus;
use crate::sa::route::route_event::RouteEvent;
use crate::sa::route::Route;
use crate::util::array_util::arry_util::{first, last};
use crate::util::fileio::yaml_loader;

//===============================================================================
// Import modules

//===============================================================================
// Implementation of RouteCSVGenerator
#[allow(dead_code)]
pub struct RouteCSVGenerator {
    // PUBLIC
    pub csv_schedule: (Vec<u16>, Vec<Vec<f32>>),
    pub data: Data,
    pub route: Vec<RouteEvent>,

    // PRIVATE
    g_config: Yaml,
    s_config: Yaml,
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
    /// * `schedule_path` : Path to YAML schedule configuration file
    /// * `general_path`  : Path to YAML general configuration file
    /// * `csv_path`      : Path to CSV file
    ///
    /// # Output
    /// * `RouteCSVGenerator`
    ///
    pub fn new(schedule_path: &str, general_path: &str, csv_path: &str) -> RouteCSVGenerator {
        // Create new RouteGenerator
        let rg = RouteCSVGenerator {
            csv_schedule: (Vec::new(), Vec::new()),
            data: Default::default(),
            route: Vec::new(),
            g_config: yaml_loader::load_yaml(general_path),
            s_config: yaml_loader::load_yaml(schedule_path),
            csv_h: parse_routes::read_csv(csv_path),
        };

        // Return Route Generator
        return rg;
    }

    //---------------------------------------------------------------------------
    /// Synchronize the `route` data with `data`.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    pub fn update_route(self: &mut RouteCSVGenerator) {
        // Empty out the routes
        self.route = Vec::new();

        // For every visit
        for i in 0..self.data.param.N {
            let a = &self.data.param.a;
            let e = self.data.param.e[i];
            let Gam = self.data.param.Gam[i];
            let gam = self.data.param.gam[i];
            let mut rt: f32 = 0.0;
            let u = self.data.dec.u[i];
            let d = self.data.dec.d[i];
            let v = self.data.dec.v[i];

            // If the BEB has another visit
            if gam >= 0 {
                rt = e - a[gam as usize];
            }

            // Create RouteEvent structure
            let r: RouteEvent = RouteEvent {
                visit: 0,
                arrival_time: a[i],
                bus: self.gen_bus(),
                departure_time: e,
                discharge: self.data.param.l[i],
                id: Gam,
                route_time: rt,
                attach_time: u,
                detach_time: d,
                queue: v as u16,
            };

            // Add route event to route
            self.route.push(r)
        }

        // Assign visit indices
        for i in 0..self.route.len() {
            self.route[i].visit = i;
        }
    }

    //===========================================================================
    // PRIVATE

    //---------------------------------------------------------------------------
    /// Allocates buffers for input parameters.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn buffer_input_parameters(self: &mut RouteCSVGenerator, visits: &HashMap<u16, Vec<Vec<f32>>>) {
        // Misc Variables
        let csv: &(Vec<u16>, Vec<Vec<f32>>) = &self.csv_schedule;
        let bod: f32 = self.s_config["time"]["BOD"].as_f64().unwrap() as f32;
        let eod: f32 = self.s_config["time"]["EOD"].as_f64().unwrap() as f32;

        // Constants
        self.data.param.A = csv.0.len();
        self.data.param.N = self.count_visits(visits);
        self.data.param.T = eod - bod;
        self.data.param.K = self.s_config["time"]["K"].as_i64().unwrap() as u16;
        self.data.param.S = 1;
        self.data.param.ts = 0.001;

        // Quality of life variables
        let A = self.data.param.A;
        let N = self.data.param.N;

        // Allocate space for input parameters
        self.data.param.a = vec![0.0; N];
        self.data.param.e = vec![0.0; N];
        self.data.param.D = vec![0.0; N];
        self.data.param.gam = vec![-1; N];
        self.data.param.Gam = vec![0; N];
        self.data.param.alpha = vec![0.0; N];
        self.data.param.beta = vec![0.0; N];

        // Create charge rate vector
        self.data.param.r = self.create_charge_rate_vector();

        // Discretise the system
        let T = self.data.param.T;
        let K = self.data.param.K;
        self.data.param.dt = T / K as f32;

        // Battery capacity
        self.data.param.k =
            [self.s_config["buses"]["bat_capacity"].as_f64().unwrap() as f32].repeat(N);

        // Create assignment cost
        let Q = self.data.param.Q;
        self.data.param.ep = vec![0.0; A];
        let mut charge_queue: Vec<f32> = (0..(Q - A)).map(|x| 100.0 * (x as f32 + 1.0)).collect();
        self.data.param.ep.append(&mut charge_queue);

        self.data.param.nu = self.s_config["buses"]["min_charge"].as_f64().unwrap() as f32;
        self.data.param.D = [self.s_config["buses"]["dis_rate"].as_f64().unwrap() as f32].repeat(A);

        self.data.param.zeta =
            [self.s_config["buses"]["dis_rate"].as_f64().unwrap() as f32].repeat(A);
    }

    //---------------------------------------------------------------------------
    /// This function returns a vector of charge rates. The values are dependent
    /// on the `bat_model` value in `general.yaml`. The rates can either be set
    /// for a linear or non-linear battery dynamics model.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * r: Vector of charge rates
    ///
    fn create_charge_rate_vector(self: &mut RouteCSVGenerator) -> Vec<f32> {
        // Variables
        let wait_c: Vec<f32> = vec![0.0; self.data.param.A];
        let slow_c: Vec<f32>;
        let fast_c: Vec<f32>;

        // Set the model type
        self.data.param.model = self.g_config["bat_model"].as_str().unwrap().to_string();

        // If the system is utilizing the linear model
        if self.data.param.model == "linear" {
            // Create parts of charge rate vector
            slow_c = [self.s_config["chargers"]["slow"]["rate"].as_f64().unwrap() as f32]
                .repeat(self.s_config["chargers"]["slow"]["num"].as_i64().unwrap() as usize);
            fast_c = [self.s_config["chargers"]["fast"]["rate"].as_f64().unwrap() as f32]
                .repeat(self.s_config["chargers"]["fast"]["num"].as_i64().unwrap() as usize);

            // Create charge rate vector
            self.data.param.r = vec![wait_c.clone(), slow_c.clone(), fast_c.clone()].concat();
        }
        // Otherwise the system us utilizing the non-linear model
        else {
            slow_c = [self.s_config["chargers"]["slow"]["conv"].as_f64().unwrap() as f32]
                .repeat(self.s_config["chargers"]["slow"]["num"].as_i64().unwrap() as usize);
            fast_c = [self.s_config["chargers"]["fast"]["conv"].as_f64().unwrap() as f32]
                .repeat(self.s_config["chargers"]["fast"]["num"].as_i64().unwrap() as usize);
        }

        // Store charger count
        self.data.param.Q = wait_c.len() + slow_c.len() + fast_c.len();
        self.data.param.slow = slow_c.len();
        self.data.param.fast = fast_c.len();

        return vec![wait_c, slow_c, fast_c].concat();
    }

    //---------------------------------------------------------------------------
    /// Allocates buffers for decision variables.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn buffer_decision_variables(self: &mut RouteCSVGenerator) {
        // Variables
        let Q = self.data.param.Q;
        let N = self.data.param.N;

        // Generate decision variable buffers
        self.data.dec.u = vec![0.0; N];
        self.data.dec.v = vec![0; N];
        self.data.dec.d = vec![0.0; N];
        self.data.dec.s = vec![0.0; N];
        self.data.dec.g = vec![vec![0.0; N]; Q];
        self.data.dec.eta = vec![0.0; N];
        self.data.dec.w = vec![vec![false; Q]; N];
        self.data.dec.sigma = vec![vec![true; N]; N];
        self.data.dec.psi = vec![vec![true; N]; N];
    }

    //---------------------------------------------------------------------------
    /// Counts the number of bus visits from the routes matrix.
    ///
    /// # Input
    /// * config: Initialization parameters from YAML
    /// * csv: Tuple containing the start/stop route information
    ///
    /// # Output
    /// * N : Number of visits
    ///
    fn count_visits(self: &RouteCSVGenerator, visits: &HashMap<u16, Vec<Vec<f32>>>) -> usize {
        let mut N: usize = 0;

        // For each BEB
        for it in visits {
            // Extract tuple
            let (_, r) = it;

            // Add routes
            N += r.len();
        }

        return N;
    }

    //---------------------------------------------------------------------------
    //
    /// Convert the start/stop representation to a arrival/departure representation
    /// of the route schedule.
    ///
    /// # Input:
    /// * init  : Initialization parameters from YAML
    /// * routes: CSV route data in start/stop route form
    ///
    /// # Output:
    /// * routes: CSV route data in arrival/departure form
    ///
    fn convert_route_to_visit(self: &RouteCSVGenerator) -> HashMap<u16, Vec<Vec<f32>>> {
        // Variables
        let bod: f32 = self.s_config["time"]["BOD"].as_f64().unwrap() as f32;
        let eod: f32 = self.s_config["time"]["EOD"].as_f64().unwrap() as f32;
        let mut route_visit: HashMap<u16, Vec<Vec<f32>>> = HashMap::new();

        // Generate set of visit/departures

        // For each bus/route
        for i in 0..self.csv_schedule.0.len() {
            // Variables
            let b: u16 = self.csv_schedule.0[i];
            let r: Vec<f32> = self.csv_schedule.1[i].clone();
            let mut tmp_route: Vec<Vec<f32>> = Vec::new();
            let J = r.len();
            let mut arrival_c: f32 = r[1];
            let mut arrival_n: f32;

            // For each start/stop route pair
            for j in (0..J).step_by(2) {
                // Update the times
                let departure: f32 = r[j];
                arrival_n = r[j + 1];

                // If the first visit is at the BOD
                if j == 0 && r[j] > bod {
                    // The first arrival time is at BOD
                    tmp_route.push(vec![bod, departure]);
                }
                // Else if the first visit is after the BOD
                else if j == 0 && r[j] == bod {
                    // Place a dummy visit to propagate discharge
                    tmp_route.push(vec![bod, bod]);
                }
                // Else append the arrival/departure time normally
                else {
                    tmp_route.push(vec![arrival_c, departure]);
                }

                // if the final visit is not at the EOD
                if j == J - 2 && r[j + 1] < eod {
                    tmp_route.push(vec![arrival_n, eod]);
                }

                // Update the current arrival
                arrival_c = arrival_n;
            }

            // Update the route
            route_visit.insert(b, tmp_route);
        }

        return route_visit;
    }

    //---------------------------------------------------------------------------
    /// Calculate the discharge for each route
    ///
    /// Input:
    ///   - self  : Scheduler object
    ///   - route : Bus routes in start/stop form
    ///
    /// Output:
    ///   - discharge : Hash map of bus IDs with discharge vector
    ///
    fn calc_discharge(self: &RouteCSVGenerator) -> HashMap<usize, Vec<f32>> {
        let mut discharge: HashMap<usize, Vec<f32>> = HashMap::new();
        let eod: f32 = self.s_config["time"]["EOD"].as_f64().unwrap() as f32;
        let routes = &self.csv_schedule;

        // For each set of routes for bus b
        for b in &routes.0 {
            let J: usize = routes.1[*b as usize].len();
            let r = routes.1[*b as usize].clone();
            let mut discharge_tmp: Vec<f32> = Vec::new();

            // For each route for bus b
            for j in (0..J).step_by(2) {
                discharge_tmp.push(self.data.param.zeta[*b as usize] * (r[j + 1] - r[j]));

                // If the final visit is not at the end of the day
                if j == J - 2 && r[j + 1] < eod {
                    // The bus has no more routes
                    discharge_tmp.push(0.0);
                }
            }

            // Append the list of discharges
            discharge.insert(*b as usize, discharge_tmp);
        }

        return discharge;
    }

    //---------------------------------------------------------------------------
    /// Converts the route information into a vector of RouteEvents.
    ///
    /// # Input
    /// * visit: Hash map of visit information
    /// * dis  : Vector of route discharges
    ///
    /// # Output
    /// * route: Vector of RouteEvents consolidating the input parameters in order
    ///          of arrival time.
    ///
    fn populate_route_events(
        self: &RouteCSVGenerator,
        visit: &HashMap<u16, Vec<Vec<f32>>>,
        discharge: &HashMap<usize, Vec<f32>>,
    ) -> Vec<RouteEvent> {
        // Allocate route buffer space
        let mut route: Vec<RouteEvent> = Vec::new();

        // Loop through each visit/discharge
        for it in visit.into_iter() {
            // Extract visit and discharge
            let vis = it;

            // Extract the bus ID and visit
            let b: usize = *vis.0 as usize;
            let vis: &Vec<Vec<f32>> = vis.1;

            // Loop through each start/stop pair
            for it in vis.into_iter().zip(&discharge[&b]) {
                // Extract iterator
                let (v, d) = it;

                // Shadow `v` so that it can be mutable
                let mut v = v.clone();

                // If the start/stop times are the same, apply an epsilon so `rand`
                // does not yell at me :(
                if v[0] == v[1] {
                    v[1] += self.data.param.ts;
                }

                // Create RouteEvent structure
                let r: RouteEvent = RouteEvent {
                    visit: 0,
                    arrival_time: v[0],
                    bus: self.gen_bus(),
                    departure_time: v[1],
                    discharge: *d,
                    id: b as u16,
                    route_time: v[1] - v[0],
                    ..Default::default()
                };

                // Add route event to route
                route.push(r)
            }
        }

        // Sort visits by arrival time
        route.sort();

        // Assign visit indices
        for i in 0..route.len() {
            route[i].visit = i;
        }

        return route;
    }

    //---------------------------------------------------------------------------
    /// Generate information about the bus from the YAML file.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * Bus: Information about bus b
    ///
    fn gen_bus(self: &RouteCSVGenerator) -> Bus {
        let bat_capacity = self.s_config["buses"]["bat_capacity"].as_f64().unwrap() as f32;
        let alpha = self.s_config["initial_charge"]["max"].as_f64().unwrap() as f32;
        let beta = self.s_config["final_charge"].as_f64().unwrap() as f32;

        return Bus {
            bat_capacity,
            initial_charge: alpha * bat_capacity,
            final_charge: beta * bat_capacity,
            discharge_rate: self.s_config["buses"]["dis_rate"].as_f64().unwrap() as f32,
        };
    }

    //---------------------------------------------------------------------------
    /// Populate all the input parameters with the data provided by the route
    /// events.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn generate_schedule_params(self: &mut RouteCSVGenerator) {
        // Determine Gamma array
        self.gen_visit_id();

        // Determine gamma array
        self.find_next_visit();

        // Assign initial charges
        self.determine_initial_charges();

        // Assign final charges
        self.determine_final_charges();

        // Assign arrival times to arrival array
        self.assign_arrival_times();

        // Assign departure times to departure array
        self.assign_departure_times();

        // Assign discharge quantities to discharge array
        self.assign_discharge();
    }

    //---------------------------------------------------------------------------
    /// Create a list of BEB ids in order of arrival.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn gen_visit_id(self: &mut RouteCSVGenerator) {
        self.data.param.Gam = self
            .route
            .iter()
            .map(|i| i.id)
            .collect::<Vec<u16>>()
            .clone();
    }

    //---------------------------------------------------------------------------
    /// Create a list indices that indicate the next arrival index for bus
    /// Gamma[i].
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn find_next_visit(self: &mut RouteCSVGenerator) {
        // Local variables
        let A = self.data.param.A;
        let Gam = &mut self.data.param.Gam;

        // Populate gamma buffer with "no next visit" value
        let gam = &mut self.data.param.gam;

        // Keep track of the previous index each BEB has arrived at
        let mut next_idx: Vec<usize> = (0..A).map(|x| last(&Gam, x as u16).unwrap()).collect();

        // Keep track of the last instance each bus arrives
        let last_idx = next_idx.clone();

        // Loop through each BEB visit
        for i in (0..self.data.param.N).rev() {
            // Make sure that the index being checked is greater than the first
            // visit. If it is, set the previous index value equal to the current.
            // In other words, index i's value indicates the next index the bus
            // will visit.
            if i < last_idx[Gam[i] as usize] {
                // Update `gamma` array
                gam[i] = next_idx[Gam[i] as usize] as i16;

                // Update `next_idx`
                next_idx[Gam[i] as usize] = i;
            }
        }
    }

    //---------------------------------------------------------------------------
    /// Assign initial charges to all BEBs.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn determine_initial_charges(self: &mut RouteCSVGenerator) {
        // Local variables
        let init_charge = self.s_config["initial_charge"]["max"]
            .clone()
            .into_f64()
            .unwrap() as f32;
        let Gam = &self.data.param.Gam;

        // Loop through each BEB
        for a in 0..self.data.param.A {
            // Assign the initial charge for BEB `a`
            self.data.param.alpha[first(Gam, a as u16).unwrap()] = init_charge;
        }

        for i in 0..self.data.param.N {
            if self.data.param.alpha[i] > 0.0 {
                // Assign the initial charge
                self.data.dec.eta[i] =
                    self.data.param.alpha[i] * self.data.param.k[Gam[i] as usize];
            }
        }
    }

    //---------------------------------------------------------------------------
    /// Assign final charges to all BEBs.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn determine_final_charges(self: &mut RouteCSVGenerator) {
        // Local variables
        let final_charge = self.s_config["final_charge"].clone().into_f64().unwrap() as f32;
        let gam = &self.data.param.gam;
        let beta = &mut self.data.param.beta;

        // Loop through each BEB
        for i in 0..gam.len() {
            if gam[i] == -1 {
                beta[i as usize] = final_charge;
            }
        }
    }

    //---------------------------------------------------------------------------
    /// Create a list of arrival times for all visits in order.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn assign_arrival_times(self: &mut RouteCSVGenerator) {
        self.data.param.a = (0..self.route.len())
            .map(|x| self.route[x].arrival_time)
            .collect();
    }

    //---------------------------------------------------------------------------
    /// Create a list of departure times for all visits in order.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn assign_departure_times(self: &mut RouteCSVGenerator) {
        self.data.param.e = (0..self.route.len())
            .map(|x| self.route[x].departure_time)
            .collect();
    }

    //---------------------------------------------------------------------------
    /// Create a list of discharge quantities for all visits in order.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn assign_discharge(self: &mut RouteCSVGenerator) {
        self.data.param.l = (0..self.route.len())
            .map(|x| self.route[x].discharge)
            .collect();
    }

    //---------------------------------------------------------------------------
    /// Update the MILP decision variables given the current `route` vector.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * None
    ///
    fn update_milp_dec_var(self: &mut RouteCSVGenerator) {
        for i in 0..self.data.param.N {
            self.data.dec.u[i] = self.route[i].attach_time;
            self.data.dec.d[i] = self.route[i].detach_time;
            self.data.dec.v[i] = self.route[i].queue as usize;
        }
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
    fn run(self: &mut RouteCSVGenerator) {
        // Parse CSV
        self.csv_schedule = parse_routes::parse_csv(&mut self.csv_h, &self.s_config);

        // Convert routes to visits
        let visits = self.convert_route_to_visit();

        // Buffer input parameters
        self.buffer_input_parameters(&visits);

        // Buffer decision variables
        self.buffer_decision_variables();

        // Estimate discharge over routes
        let dis = self.calc_discharge();

        // Populate route data
        self.route = self.populate_route_events(&visits, &dis);

        // Generate schedule parameters
        self.generate_schedule_params();
    }

    //---------------------------------------------------------------------------
    /// Return the route data
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `route`: Vector of route data
    ///
    fn get_route_events(self: &mut RouteCSVGenerator) -> Box<&mut Vec<RouteEvent>> {
        return Box::new(&mut self.route);
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
    fn get_data(self: &mut RouteCSVGenerator) -> Box<Data> {
        return Box::new(self.data.clone());
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
    fn set_route_events(self: &mut RouteCSVGenerator, r: Box<&mut Vec<RouteEvent>>) {
        self.route = r.clone();
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
    fn set_data(self: &mut RouteCSVGenerator, dat: Box<Data>) {
        self.data = *dat;
    }

    //---------------------------------------------------------------------------
    /// Update the route events based on the data object
    ///
    /// # Input
    /// * `data`: Data object
    ///
    /// # Output
    /// * NONE
    ///
    fn update_route_events(self: &mut RouteCSVGenerator) {
        self.update_route();
    }

    //---------------------------------------------------------------------------
    /// Update the MILP data based on the route events object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn update_milp_data(self: &mut RouteCSVGenerator) {
        self.generate_schedule_params();
        self.update_milp_dec_var();
    }
}

//==============================================================================
// TEST PRIVATE METHODS IN ROUTE GENERATOR
#[cfg(test)]
mod priv_test_route_gen {
    //==========================================================================
    // Import modules
    use super::{Route, RouteCSVGenerator, RouteEvent};

    //--------------------------------------------------------------------------
    //
    fn create_object() -> RouteCSVGenerator {
        return RouteCSVGenerator::new(
            "./src/config/schedule-test.yaml",
            "./src/config/routes.csv",
        );
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_convert_route_to_visit() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Get the route visits
        let route = rg.convert_route_to_visit();

        // Test that there is something
        assert_eq!(route.is_empty(), false);

        // Test routes
        let r = match route.get(&0) {
            Some(r) => r.clone(),
            None => vec![],
        };

        assert_eq!(
            r[0],
            vec![0.0, 0.0],
            "The route route does not match the vector."
        );
        assert_eq!(
            r[1],
            vec![5.3333335, 5.3333335],
            "The route route does not match the vector."
        );
        assert_eq!(
            r[2],
            vec![6.016667, 8.075],
            "The route route does not match the vector."
        );

        let r = match route.get(&10) {
            Some(r) => r.clone(),
            None => vec![],
        };

        assert_eq!(
            r[0],
            vec![0.0, 0.0],
            "The route route does not match the vector."
        );
        assert_eq!(
            r[1],
            vec![6.0, 11.208333],
            "The route route does not match the vector."
        );
        assert_eq!(
            r[2],
            vec![11.683333, 13.783334],
            "The route route does not match the vector."
        );
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_discharge() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Get the route visits
        let _route = rg.convert_route_to_visit();
        let dis = rg.calc_discharge();

        // Test 1
        let j: usize = 0;
        let b: usize = 0;
        let r = rg.csv_schedule.1[b].clone();
        let l_dis = rg.data.param.zeta[b] * (r[j + 1] - r[j]);
        assert_eq!(dis[&b][j / 2 as usize], l_dis);

        // Test 2
        let j: usize = 4;
        let b: usize = 2;
        let r = rg.csv_schedule.1[b].clone();
        let l_dis = rg.data.param.zeta[b] * (r[j + 1] - r[j]);
        assert_eq!(dis[&b][j / 2 as usize], l_dis);

        // Test 3
        let j: usize = 6;
        let b: usize = 8;
        let r = rg.csv_schedule.1[b].clone();
        let l_dis = rg.data.param.zeta[b] * (r[j + 1] - r[j]);
        assert_eq!(dis[&b][j / 2 as usize], l_dis);

        // Test 4
        let j: usize = 10;
        let b: usize = 15;
        let r = rg.csv_schedule.1[b].clone();
        let l_dis = rg.data.param.zeta[b] * (r[j + 1] - r[j]);
        assert_eq!(dis[&b][j / 2 as usize], l_dis);
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_visit() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Get the route visits
        let visit = rg.convert_route_to_visit();
        let dis = rg.calc_discharge();

        // Get the RouteEvents
        let re = rg.populate_route_events(&visit, &dis);

        // Test 1
        let r: RouteEvent = RouteEvent {
            arrival_time: visit[&0][0][0],
            bus: rg.gen_bus(),
            departure_time: visit[&0][0][0],
            discharge: dis[&0][0],
            id: 0,
            route_time: visit[&0][0][0] - visit[&0][0][0],
            ..Default::default()
        };

        // Search for the matching ID
        let mut idx = 0;
        for i in 0..rg.data.param.N {
            if re[i].id == 0 {
                idx = i;
                break;
            }
        }

        assert_eq!(re[idx], r);
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_gen_visit_id() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Get the visit identifiers
        let Gam = rg.data.param.Gam.clone();

        // Check the visits
        for i in 0..Gam.len() {
            // Ensure sure the IDs match
            assert_eq!(
                Gam[i], rg.route[i].id,
                "The IDs do match route in Gamma and RouteEvents."
            );
        }
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_find_next_visit() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Get the initial visit and the next visit indices
        let Gam = rg.data.param.Gam.clone();
        let gam = rg.data.param.gam.clone();

        // Check the visits
        for i in 0..Gam.len() {
            // If the BEB has another visit
            if gam[i] > 0 {
                // Ensure the next visit has the same ID
                assert_eq!(
                    Gam[i], rg.route[gam[i] as usize].id,
                    "The ID of the current visit and next visit do not match."
                );
            }
        }

        assert_eq!(*gam.last().unwrap(), -1);
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_determine_initial_charges() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Get the charge percentage and the battery capacity
        let alpha = rg.data.param.alpha.clone();
        let kap = rg.data.param.k;

        // Count the number of initial charges
        let mut cnt = 0;

        // Check initial charge
        for i in 0..alpha.len() {
            // If visit `i` is an initial visit
            if alpha[i] > 0.0 {
                // Increment the counter
                cnt += 1;

                // Ensure that the initial charge is the expected value
                assert_eq!(
                    kap[i] * alpha[i],
                    rg.route[i].bus.initial_charge,
                    "The initial charges do not match."
                );
            }
        }

        // Ensure the number of initial charges equals the number of BEBs
        assert_eq!(
            cnt, rg.data.param.A,
            "The number of initial charges and BEBs do not match."
        );
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_determine_final_charge() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Get the charge percentage and the battery capacity
        let beta = rg.data.param.beta.clone();
        let kap = rg.data.param.k;

        // Count the number of initial charges
        let mut cnt = 0;

        // Check initial charge
        for i in 0..beta.len() {
            // If visit `i` is an initial visit
            if beta[i] > 0.0 {
                // Increment the counter
                cnt += 1;

                // Ensure that the initial charge is the expected value
                assert_eq!(
                    kap[i] * beta[i],
                    rg.route[i].bus.initial_charge,
                    "The initial charges do not match."
                );
            }
        }

        // Ensure the number of initial charges equals the number of BEBs
        assert_eq!(
            cnt, rg.data.param.A,
            "The number of initial charges and BEBs do not match."
        );
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_assign_arrival_times() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Loop through each visit
        for i in 0..rg.data.param.a.len() {
            // Ensure the arrival times are the same as the routes
            assert_eq!(
                rg.data.param.a[i], rg.route[i].arrival_time,
                "The data arrival time does not match the route arrival time"
            );
        }
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_departure_times() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Loop through each visit
        for i in 0..rg.data.param.a.len() {
            // Ensure the departure times are the same as the routes
            assert_eq!(
                rg.data.param.e[i], rg.route[i].departure_time,
                "The data departure time does not match the route departure time"
            );
        }
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_assign_discharge() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Loop through each visit
        for i in 0..rg.data.param.a.len() {
            // Ensure the departure times are the same as the routes
            assert_eq!(
                rg.data.param.l[i], rg.route[i].discharge,
                "The data discharge quantity does not match the route departure time"
            );
        }
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_update_route() {
        // Create the CSV Generator object
        let mut rg: RouteCSVGenerator = create_object();

        // Run the generator
        rg.run();

        // Loop through each visit
        for i in 0..rg.data.param.N {
            // Set a dummy arrival time
            rg.route[i].arrival_time = i as f32;
        }

        // Update `RouteEvents` with data
        rg.update_route();

        // Make sure data and route events match
        for i in 0..rg.data.param.N {
            assert_eq!(rg.route[i].visit, i);
            assert_eq!(rg.route[i].arrival_time, rg.data.param.a[i]);
            assert_eq!(rg.route[i].departure_time, rg.data.param.e[i]);
            assert_eq!(rg.route[i].attach_time, rg.data.dec.u[i]);
            assert_eq!(rg.route[i].detach_time, rg.data.dec.d[i]);
        }
    }
}
