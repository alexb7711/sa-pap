extern crate sa_pap;

//===============================================================================
/// TEST PUBLIC INTERFACES OF ROUTE_GENERATOR

//===============================================================================
//
#[cfg(test)]
mod test_route_rand_generator {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::route::route_rand_generator::RouteRandGenerator;
    use super::sa_pap::sa::route::Route;
    use sa_pap::util::fileio::yaml_loader;

    //---------------------------------------------------------------------------
    //
    fn get_route_size() -> usize {
        return yaml_loader::load_yaml(yaml_path())["buses"]["num_visit"]
            .as_i64()
            .unwrap() as usize;
    }

    //---------------------------------------------------------------------------
    //
    fn get_bus_size() -> usize {
        return yaml_loader::load_yaml(yaml_path())["buses"]["num_bus"]
            .as_i64()
            .unwrap() as usize;
    }

    //---------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str {
        return "./src/config/schedule-test.yaml";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_valid_load_yaml() {
        let mut rg: RouteRandGenerator = RouteRandGenerator::new(false, yaml_path());

        rg.run();

        assert_eq!(rg.route.capacity(), get_route_size());
        assert_eq!(rg.buses.capacity(), get_bus_size());
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_valid_bus_data() {
        let mut rg: RouteRandGenerator = RouteRandGenerator::new(false, yaml_path());

        rg.run();

        let e = rg.route;

        // Test 1: Make sure all buses start with an arrival time of 0
        assert!(e[0].arrival_time == e[1].arrival_time);
        assert!(e[1].arrival_time == e[5].arrival_time);
        assert!(e[9].arrival_time == e[8].arrival_time);
        assert!(e[0].arrival_time == e[2].arrival_time);

        // Test 2: Arrival times are sorted from least to greatest
        assert!(e[0].arrival_time < e[30].arrival_time);
        assert!(e[3].arrival_time <= e[20].arrival_time);
        assert!(e[15].arrival_time < e[70].arrival_time);
        assert!(e[100].arrival_time <= e[101].arrival_time);
        assert!(e[160].arrival_time < e[199].arrival_time);
        assert!(e[5].arrival_time < e[30].arrival_time);
        assert!(e[35].arrival_time < e[88].arrival_time);
    }
}
