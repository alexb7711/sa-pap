extern crate sa_pap;

//===============================================================================
/// TEST PUBLIC INTERFACES OF ROUTE_CSV_GENERATOR

//===============================================================================
//
#[cfg(test)]
mod test_route_csv_generator {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::route::route_csv_generator::{parse_routes, RouteCSVGenerator};
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
    fn csv_path() -> &'static str {
        return "./src/config/routes.csv";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_load_yaml() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Test bus IDs
        assert_eq!(rg.csv_schedule.0[0] , 0);
        assert_eq!(rg.csv_schedule.0[10], 10);
        assert_eq!(rg.csv_schedule.0[15], 15);
        assert_eq!(rg.csv_schedule.0[20], 20);
        assert_eq!(rg.csv_schedule.0[30], 30);
        assert_eq!(rg.csv_schedule.0[42], 42);
        assert_eq!(rg.csv_schedule.0[58], 58);

        // Test route times
        assert_eq!(rg.csv_schedule.1[0][0]  , 0.0);
        assert_eq!(rg.csv_schedule.1[0][7]  , 39600.0);
        assert_eq!(rg.csv_schedule.1[0][15] , 82800.0);

        assert_eq!(rg.csv_schedule.1[1][0]  , 0.0);
        assert_eq!(rg.csv_schedule.1[1][5]  , 31830.0);
        assert_eq!(rg.csv_schedule.1[1][9]  , 49950.0);

        assert_eq!(rg.csv_schedule.1[10][0]  , 0.0);
        assert_eq!(rg.csv_schedule.1[10][10] , 78420.0);
        assert_eq!(rg.csv_schedule.1[10][15] , 86370.0);

        assert_eq!(rg.csv_schedule.1[36][0]  , 0.0);
        assert_eq!(rg.csv_schedule.1[36][6]  , 32610.0);
        assert_eq!(rg.csv_schedule.1[36][22] , 68880.0);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_csv_load() {
        let mut p = parse_routes::read_csv(csv_path());

        for r in p.records() {
            println!("{:?}", r);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    #[should_panic]
    fn test_csv_bad_path() {
        parse_routes::read_csv("./routes.csv");
    }
}
