extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_generators {

    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::route::Route;
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
        let mut charger: Charger = Charger::new(yaml_path(), true, None);

        // Create CSV generator
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        rg.run();

        // Create the generator
        let mut sg = ScheduleGenerator::new();

        // Run the generator
        sg.run(&mut rg, &mut charger);

        // Bus ID
        let _id: usize = 0;

        // Test 0 - Check first index of a few chargers
        assert_eq!(charger.schedule[0][0].t, (0.0,0.0));
        assert_eq!(charger.schedule[0][0].b, 0);

        assert_eq!(charger.schedule[1][0].t, (0.0,0.0));
        assert_eq!(charger.schedule[1][0].b, 1);

        assert_eq!(charger.schedule[2][0].t, (0.0,0.0));
        assert_eq!(charger.schedule[2][0].b, 2);

        assert_eq!(charger.schedule[9][0].t, (0.0,0.0));
        assert_eq!(charger.schedule[9][0].b, 9);

        // Test 1 - Test first index of a few chargers
        println!("{:?}", charger.schedule);
        assert_eq!(charger.schedule[0][1].t, (5.3333335, 5.3333335));
        assert_eq!(charger.schedule[0][1].b, 0);

        assert_eq!(charger.schedule[1][1].t, (5.6666665, 5.6666665));
        assert_eq!(charger.schedule[1][1].b, 1);

        assert_eq!(charger.schedule[2][1].t, (6.0,6.0));
        assert_eq!(charger.schedule[2][1].b, 2);

        assert_eq!(charger.schedule[18][1].t, (6.0,6.375));
        assert_eq!(charger.schedule[18][1].b, 18);

    }
}
