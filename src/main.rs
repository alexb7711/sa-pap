// Public Crates
use yaml_rust::Yaml;

// Import Modules
use sa_pap::route_generator::{RouteGenerator, Generator};
use sa_pap::util::bool_util;
use sa_pap::util::fileio::yaml_loader;

//===============================================================================
//
fn main()
{
    // Load in general YAML
    let gen_config: Yaml = yaml_loader::load_yaml("./src/yaml/general.yaml");

    // Create simulated annealing module
    //   - Select initial temperature
    //   - Select cooling schedule

    // Create schedule generator
    let load_from_file: bool = bool_util::i64_to_bool(gen_config["load_from_file"].clone().into_i64().unwrap());
    let mut rg: RouteGenerator = RouteGenerator::new(load_from_file, "./src/yaml/schedule.yaml");

    RouteGenerator::run(&mut rg);

    // Create solution generator and tweaker

    // Pass schedule generator, solution generator, and solution tweaker
    // into the SA module

    // Run simulated annealing simulation

    // Plot results
}
