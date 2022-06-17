
// Public Crates

// My modules
pub mod plotter;            // Simulated annealing algorithm
pub mod sa;                 // Simulated annealing algorithm
pub mod route_generator;    // Create random routes
pub mod schedule_generator; // Generate charging schedule
pub mod solution_tweaker;   // Tweak a given solution

// Import Modules
use crate::route_generator::RouteGenerator;

//===============================================================================
//
fn main()
{
    // Create simulated annealing module
    //   - Select initial temperature
    //   - Select cooling schedule

    // Create schedule generator
    let rg: RouteGenerator = route_generator::RouteGenerator::new("./src/yaml/schedule.yaml");

    println!("{:?}", rg.config[0]["time_horizon"].as_i64().unwrap())

    // Create solution generator and tweaker

    // Pass schedule generator, solution generator, and solution tweaker
    // into the SA module

    // Run simulated annealing simulation

    // Plot results
}
