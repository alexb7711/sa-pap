// Public Crates

// Import Modules
use sa_pap::route_generator::RouteGenerator;

//===============================================================================
//
fn main()
{
    // Create simulated annealing module
    //   - Select initial temperature
    //   - Select cooling schedule

    // Create schedule generator
    let rg: RouteGenerator = RouteGenerator::new("./src/yaml/schedule.yaml");

    println!("{:?}", rg.config["time_horizon"].as_i64().unwrap())

    // Create solution generator and tweaker

    // Pass schedule generator, solution generator, and solution tweaker
    // into the SA module

    // Run simulated annealing simulation

    // Plot results
}
