//------------------------------------------------------------------------------
// Import standard library
use yaml_rust::Yaml;

//------------------------------------------------------------------------------
// Import developed modules
use sa_pap::sa::generators::gen_wait_queue::GenWaitQueue;
use sa_pap::sa::generators::tweak_schedule::TweakSchedule;
use sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
use sa_pap::sa::route::route_rand_generator::RouteRandGenerator;
use sa_pap::sa::route::Route;
use sa_pap::sa::temp_func::{CoolSchedule::Geometric, TempFunc};
use sa_pap::sa::SA;
use sa_pap::util::bool_util;
use sa_pap::util::fileio::yaml_loader;

//===============================================================================
// FUNCTIONS

//------------------------------------------------------------------------------
//
fn schedule_path() -> &'static str {
    return "./src/config/schedule.yaml";
}

//--------------------------------------------------------------------------
//
fn csv_path() -> &'static str {
    return "./src/config/routes.csv";
}

//------------------------------------------------------------------------------
//
fn general_path() -> &'static str {
    return "./src/config/general.yaml";
}

//===============================================================================
// MAIN
fn main() {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Initialize

    // Load in general YAML
    let gen_config: Yaml = yaml_loader::load_yaml(general_path());

    // Determine schedule type
    let schedule_type = gen_config["schedule"].clone().into_string().unwrap();

    // Decide to load previous run solution
    bool_util::i64_to_bool(gen_config["load_from_file"].clone().into_i64().unwrap());

    // Decide to load previous run solution
    let load_from_file: bool =
        bool_util::i64_to_bool(gen_config["load_from_file"].clone().into_i64().unwrap());

    // Decide whether to plot results or not
    let plot: bool = bool_util::i64_to_bool(gen_config["plot"].clone().into_i64().unwrap());

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Generate schedule

    // Create buffer for general system generator
    let gsys: Box<dyn Route>;

    // Run the schedule that was specified
    if schedule_type == "csv" {
        gsys = Box::new(RouteCSVGenerator::new(schedule_path(), csv_path()));
    } else if schedule_type == "rand" {
        // Create schedule generator
        gsys = Box::new(RouteRandGenerator::new(load_from_file, schedule_path()));
    } else {
        panic!("Unknown route generator specified in `general.yaml`!!!");
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Create solution temperature function, generator and tweaker

    let tf: &mut Box<TempFunc> = &mut Box::new(TempFunc::new(Geometric, 500.0, 0.995, true));
    let gsol: Box<GenWaitQueue> = Box::new(GenWaitQueue::new());
    let gtweak: Box<TweakSchedule> = Box::new(TweakSchedule::new());

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Create SA object and run SA

    // Pass schedule generator, temperature function, solution generator, and solution tweaker into the SA module
    let mut sa: SA = SA::new(schedule_path(), gsol, gsys, gtweak, tf);

    // Run simulated annealing simulation
    if let Some(_res) = sa.run(load_from_file) {
        // Export results to CSV
        // Plot results
        if plot {}
    } else {
        panic!("No result was generated!!!");
    }
}
