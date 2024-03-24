extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_generators {

    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::generators::gen_wait_queue::GenWaitQueue;
    use super::sa_pap::sa::generators::Generator;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::route::Route;

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

    //------------------------------------------------------------------------------
    //
    fn general_path() -> &'static str {
        return "./src/config/general.yaml";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_schedule_generator() {
        // Create CSV generator
        let rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), general_path(), csv_path());
        let mut rg: Box<dyn Route> = Box::new(rg);
        rg.run();

        // Create charger
        let mut charger: Charger = Charger::new(
            yaml_path(),
            true,
            Some(rg.get_data().param.A as usize),
            None,
        );

        // Create the generator
        let mut sg = GenWaitQueue::new();

        // Epsilon time shift
        let ep = rg.get_data().param.ts;

        // Run the generator
        sg.run(&mut rg, &mut charger);

        // Test 0 - Check first index of a few chargers
        assert_eq!(charger.schedule[0][0].t, (0.0, 0.0 + ep));
        assert_eq!(charger.schedule[0][0].b, 0);

        assert_eq!(charger.schedule[1][0].t, (0.0, 0.0 + ep));
        assert_eq!(charger.schedule[1][0].b, 1);

        assert_eq!(charger.schedule[2][0].t, (0.0, 0.0 + ep));
        assert_eq!(charger.schedule[2][0].b, 2);

        assert_eq!(charger.schedule[9][0].t, (0.0, 0.0 + ep));
        assert_eq!(charger.schedule[9][0].b, 9);

        // Test 1 - Check second index of a few chargers
        assert_eq!(charger.schedule[0][1].t, (5.3333335, 5.3333335 + ep));
        assert_eq!(charger.schedule[0][1].b, 0);

        assert_eq!(charger.schedule[1][1].t, (5.6666665, 5.6666665 + ep));
        assert_eq!(charger.schedule[1][1].b, 1);

        assert_eq!(charger.schedule[2][1].t, (6.0, 6.0 + ep));
        assert_eq!(charger.schedule[2][1].b, 2);

        assert_eq!(charger.schedule[9][1].t, (5.6666665, 10.875));
        assert_eq!(charger.schedule[9][1].b, 9);

        // Test 2 - Test third index of a few chargers
        println!("{:?}", charger.schedule);
        assert_eq!(charger.schedule[0][2].t, (6.016667, 8.075));
        assert_eq!(charger.schedule[0][2].b, 0);

        assert_eq!(charger.schedule[1][2].t, (6.35, 8.408334));
        assert_eq!(charger.schedule[1][2].b, 1);

        assert_eq!(charger.schedule[2][2].t, (6.6833334, 8.741667));
        assert_eq!(charger.schedule[2][2].b, 2);

        assert_eq!(charger.schedule[18][2].t, (6.8083334, 7.358333));
        assert_eq!(charger.schedule[18][2].b, 18);
    }
}
