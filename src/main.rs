// Public Crates
// use yaml_rust::Yaml;

// Import Modules
// use sa_pap::sa::route::route_rand_generator::RouteRandGenerator;
use sa_pap::sa::generators::gen_wait_queue::GenWaitQueue;
use sa_pap::sa::generators::tweak_schedule::TweakSchedule;
use sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
use sa_pap::sa::temp_func::{CoolSchedule::Geometric, TempFunc};
use sa_pap::sa::SA;
use sa_pap::util::bool_util;
use sa_pap::util::fileio::yaml_loader;
use yaml_rust::Yaml;

//===============================================================================
//
fn main() {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Initialize

    // Load in general YAML
    let gen_config: Yaml = yaml_loader::load_yaml("./src/config/general.yaml");

    // Decide to load previous run solution
    let load_from_file: bool =
        bool_util::i64_to_bool(gen_config["load_from_file"].clone().into_i64().unwrap());

    // Decide whether to plot results or not
    let _plot: bool = bool_util::i64_to_bool(gen_config["plot"].clone().into_i64().unwrap());

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Generate schedule

    // Create schedule generator
    // let gsys: Box<RouteRandGenerator> = Box::new(RouteRandGenerator::new(
    //     load_from_file,
    //     "./src/config/schedule.yaml",
    // ));

    let gsys = Box::new(RouteCSVGenerator::new(
        "./src/config/schedule.yaml",
        "./src/config/routes.csv",
    ));

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Create solution temperature function, generator and tweaker

    let tf: &mut Box<TempFunc> = &mut Box::new(TempFunc::new(Geometric, 500.0, 0.995, true));
    let gsol: Box<GenWaitQueue> = Box::new(GenWaitQueue::new());
    let gtweak: Box<TweakSchedule> = Box::new(TweakSchedule::new());

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Create SA object and run SA

    // Pass schedule generator, temperature function, solution generator, and solution tweaker into the SA module
    let mut sa: SA = SA::new("./src/config/schedule.yaml", gsol, gsys, gtweak, tf);

    // Run simulated annealing simulation
    sa.run(load_from_file);

    // Plot results
}
