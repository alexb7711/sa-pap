extern crate sa_pap;

//==============================================================================
//
#[cfg(test)]
mod test_tweak_schedule {

    //--------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::generators::gen_new_visits::GenNewVisits;
    use super::sa_pap::sa::generators::tweak_schedule::TweakSchedule;
    use super::sa_pap::sa::generators::Generator;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::route::Route;

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
    #[test]
    fn test_tweak_schedule_result() {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Initialize
        // Load charger parameters from YAML file
        let mut charger: Charger = Charger::new(yaml_path(), true, Some(35), None);

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Generate schedule
        let mut gsys: Box<RouteCSVGenerator> =
            Box::new(RouteCSVGenerator::new(yaml_path(), csv_path()));
        gsys.run();
        let mut gsys: Box<dyn Route> = gsys;

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create solution generator and run it
        let mut gsol: Box<GenNewVisits> = Box::new(GenNewVisits::new());
        gsol.run(&mut gsys, &mut charger);

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Get a copy of the solution
        let sol_orig = gsys.get_data().dec;

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create tweaker
        let mut gtweak: Box<TweakSchedule> = Box::new(TweakSchedule::new());

        for _ in 0..10 {
            //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Tweak the original schedule and check for updates
            while !gtweak.run(&mut gsys, &mut charger) {}
            let sol_new = gsys.get_data().dec;

            assert_ne!(sol_orig, sol_new, "The old and new solution match");
        }
    }
}
