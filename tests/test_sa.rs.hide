extern crate sa_pap;

//==============================================================================
//
#[cfg(test)]
mod test_sa {

    //--------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::generators::gen_wait_queue::GenWaitQueue;
    use super::sa_pap::sa::generators::tweak_schedule::TweakSchedule;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::temp_func::{CoolSchedule::Geometric, TempFunc};
    use super::sa_pap::sa::SA;
    use super::sa_pap::util::bool_util;
    use super::sa_pap::util::fileio::yaml_loader;
    use yaml_rust::Yaml;

    //--------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str {
        return "./src/config/schedule-test.yaml";
    }

    //--------------------------------------------------------------------------
    //
    fn csv_path() -> &'static str {
        return "./src/config/routes.csv";
    }

    //--------------------------------------------------------------------------
    //
    fn general_path() -> &'static str {
        return "./src/config/general.yaml";
    }

    //--------------------------------------------------------------------------
    //
    #[test]
    fn test_sa_result() {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Initialize

        // Load in general YAML
        let gen_config: Yaml = yaml_loader::load_yaml(general_path());

        // Decide to load previous run solution
        let load_from_file: bool =
            bool_util::i64_to_bool(gen_config["load_from_file"].clone().into_i64().unwrap());

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Generate schedule
        let gsys = Box::new(RouteCSVGenerator::new(yaml_path(), csv_path()));

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create solution temperature function, generator and tweaker

        let tf: &mut Box<TempFunc> = &mut Box::new(TempFunc::new(Geometric, 500.0, 0.995, true));
        let gsol: Box<GenWaitQueue> = Box::new(GenWaitQueue::new());
        let gtweak: Box<TweakSchedule> = Box::new(TweakSchedule::new());

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create SA object and run SA

        // Pass schedule generator, temperature function, solution generator, and solution tweaker into the SA module
        let mut sa: SA = SA::new(yaml_path(), gsol, gsys, gtweak, tf);

        // Run simulated annealing simulation
        if let Some(_res) = sa.run(load_from_file) {
            assert!(true);
        } else {
            assert!(false, "No result was generated!!!");
        }
    }
}
