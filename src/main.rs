use sa_pap::sa::generators::Generator;
//------------------------------------------------------------------------------
// Import standard library
use yaml_rust::Yaml;

//------------------------------------------------------------------------------
// Import developed modules
use sa_pap::sa::generators::gen_new_visits::GenNewVisits;
use sa_pap::sa::generators::gen_wait_queue::GenWaitQueue;
use sa_pap::sa::generators::tweak_schedule::TweakSchedule;
use sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
use sa_pap::sa::route::route_rand_generator::RouteRandGenerator;
use sa_pap::sa::route::Route;
use sa_pap::sa::temp_func::{
    CoolSchedule::Exponential, CoolSchedule::Geometric, CoolSchedule::Linear, TempFunc,
};
use sa_pap::sa::SA;
use sa_pap::util::bool_util;
use sa_pap::util::data_output::DataOutput;
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

    // Load in schedule YAML
    let schedule_config: Yaml = yaml_loader::load_yaml(schedule_path());

    // Determine schedule type
    let schedule_type = gen_config["schedule"].clone().into_string().unwrap();

    // Determine solution generator
    let sol_gen = gen_config["solution_gen"].clone().into_string().unwrap();

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

    // Get parameters
    let temperature_func = schedule_config["temp"]["type"]
        .clone()
        .into_string()
        .unwrap();
    let init_temp = schedule_config["temp"]["init"].clone().into_f64().unwrap() as f32;
    let delta = schedule_config["temp"]["delta"].clone().into_f64().unwrap() as f32;

    // Create temperature function
    let mut tf: Box<TempFunc>;
    if temperature_func == "Geometric" {
        tf = Box::new(TempFunc::new(Geometric, init_temp, delta, true));
    } else if temperature_func == "Exponential" {
        tf = Box::new(TempFunc::new(Exponential, init_temp, delta, true));
    } else if temperature_func == "Linear" {
        tf = Box::new(TempFunc::new(Linear, init_temp, delta, true));
    } else {
        panic!("Invalid temperature schedule provided!!!");
    }

    // Create solver
    let gsol: Box<dyn Generator>;
    if sol_gen == "wait" {
        gsol = Box::new(GenWaitQueue::new());
    } else {
        gsol = Box::new(GenNewVisits::new());
    }

    // Create tweaker
    let gtweak: Box<TweakSchedule> = Box::new(TweakSchedule::new());

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Create SA object and run SA

    // Pass schedule generator, temperature function, solution generator, and solution tweaker into the SA module
    let mut sa: SA = SA::new(schedule_path(), gsol, gsys, gtweak, &mut tf);

    // Run simulated annealing simulation
    if let Some(res) = sa.run(load_from_file) {
        // Export results to CSV
        DataOutput::output_data(String::from("sa"), res, None);

        // Plot results
        if plot {}
    } else {
        panic!("No result was generated!!!");
    }
}
