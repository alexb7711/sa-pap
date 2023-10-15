extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_primitive_generators {

    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::generators::Generator;
    use super::sa_pap::sa::generators::schedule_generator::ScheduleGenerator;

    //---------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str {
        return "./src/config/schedule-test.yaml";
    }

   //---------------------------------------------------------------------------
    //
    fn csv_path() -> &'static str {
        return "./src/config/routes.csv";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_schedule_generator() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None);

        // Create CSV generator
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Create the generator
        let mut sg = ScheduleGenerator::new();

        // Run the generator
        sg.run(&mut rg, &mut charger);

        // Queue index
        let _q: usize = 0;

        // Bus ID
        let _id: usize = 3;

        // Test 0 - Ensure that the free time is (BOD, EOD)
        // assert_eq!(charger.free_time[q][0], (0.0, 10.0));

        // Test 1 - Ensure the size of free times is 1
        // assert_eq!(charger.free_time[q].len(), 1);
    }
}
