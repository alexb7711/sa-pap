#![allow(non_snake_case)]

//===============================================================================
// Declare submodules
pub mod parse_routes;

//===============================================================================
// External Crates
use csv;
use yaml_rust::Yaml;

//===============================================================================
// Import Crates
use crate::sa::data::Data;
use crate::sa::route::Route;
use crate::util::fileio::yaml_loader;

//===============================================================================
// Import modules

//===============================================================================
// Implementation of ScheduleGenerator
#[allow(dead_code)]
pub struct RouteCSVGenerator {
    // PUBLIC
    pub csv_schedule: (Vec<u16>, Vec<Vec<f32>>),
    pub data: Data,

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
    pub fn new(config_path: &str, csv_path: &str) -> RouteCSVGenerator {
        // Create new RouteGenerator
        let rg = RouteCSVGenerator {
            csv_schedule: (Vec::new(), Vec::new()),
            data: Data {
                ..Default::default()
            },
            config: yaml_loader::load_yaml(config_path),
            csv_h: parse_routes::read_csv(csv_path),
        };

        // Return Route Generator
        return rg;
    }

    //===========================================================================
    // PRIVATE

    //---------------------------------------------------------------------------
    //
    /// Allocates space for the start,stop set of routes to be generated.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn buffer_attributes(self: &mut RouteCSVGenerator) {
        let csv: &(Vec<u16>, Vec<Vec<f32>>) = &self.csv_schedule;
        let bod: f32 = self.config["time"]["BOD"].as_f64().unwrap() as f32;
        let eod: f32 = self.config["time"]["EOD"].as_f64().unwrap() as f32;

        self.data.param.A = csv.0.len();
        self.data.param.N = self.count_visits(&self.config, &csv);
        self.data.param.T = eod - bod;
        self.data.param.K = self.config["time"]["K"].as_i64().unwrap() as u16;

        let A = self.data.param.A;
        let N = self.data.param.N;

        self.data.param.a.reserve(N);
        self.data.param.e.reserve(N);
        self.data.param.D.reserve(N);
        self.data.param.g.reserve(N);
        self.data.param.G.reserve(N);

        let slow_c = [self.config["chargers"]["slow"]["rate"].as_f64().unwrap() as f32]
            .repeat(self.config["chargers"]["slow"]["num"].as_i64().unwrap() as usize);
        let fast_c = [self.config["chargers"]["fast"]["rate"].as_f64().unwrap() as f32]
            .repeat(self.config["chargers"]["fast"]["num"].as_i64().unwrap() as usize);
        self.data.param.Q = slow_c.len() + fast_c.len();

        self.data.param.alpha.reserve(A);
        self.data.param.beta.reserve(A);

        let T = self.data.param.T;
        let K = self.data.param.K;
        self.data.param.dt = T / K as f32;

        self.data.param.r = [slow_c.clone(), fast_c.clone()].concat();
        self.data.param.ep = self.data.param.r.clone();

        self.data.param.k =
            [self.config["buses"]["bat_capacity"].as_f64().unwrap() as f32].repeat(A);

        let Q = self.data.param.Q;
        self.data.param.m = (0..Q).map(|x| 1000*x).collect();

        self.data.param.nu = self.config["buses"]["min_charge"].as_f64().unwrap() as f32;
        self.data.param.D = [self.config["buses"]["dis_rate"].as_f64().unwrap() as f32].repeat(A);

        self.data.param.slow = slow_c.len();
        self.data.param.fast = fast_c.len();

    }

    //---------------------------------------------------------------------------
    //
    /// Counts the number of bus visits from the routes matrix.
    ///
    /// # Input
    ///
    /// * config: Initialization parameters from YAML
    /// * csv: Tuple containing the start/stop route information
    ///
    /// # Output
    ///
    /// * N : Number of visits
    ///
    fn count_visits(
        self: &RouteCSVGenerator,
        config: &Yaml,
        csv: &(Vec<u16>, Vec<Vec<f32>>),
    ) -> usize {
        let mut N: usize = 0;

        let bod: f32 = config["time"]["BOD"].as_f64().unwrap() as f32;
        let eod: f32 = config["time"]["EOD"].as_f64().unwrap() as f32;

        // for each bus
        for r in &csv.1 {
            // For start/stop pair, there is one visit
            N += (r.len() / 2) as usize;

            // If the bus does not go on route immediately after the working day has
            // begun
            if r.first().unwrap() > &bod {
                N += 1 // Increment the visit counter
            }
            // If the bus arrives before the end of the working day
            if r.last().unwrap() < &eod {
                N += 1 // Increment the visit counter
            }
        }

        return N;
    }

    //---------------------------------------------------------------------------
    //
    fn convert_route_to_visit(self: &RouteCSVGenerator) {}

    //---------------------------------------------------------------------------
    //
    fn calc_discharge(self: &RouteCSVGenerator) {}

    //---------------------------------------------------------------------------
    //
    fn generate_schedule_params(self: &RouteCSVGenerator) {}
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
        self.csv_schedule = parse_routes::parse_csv(&mut self.csv_h);

        // Buffer Attributes
        self.buffer_attributes();

        // Convert routes to visits
        self.convert_route_to_visit();

        // Estimate discharge over routes
        self.calc_discharge();

        // Generate schedule parameters
        self.generate_schedule_params();
    }
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
