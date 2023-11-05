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
    fn test_csv_load() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Test bus IDs
        assert_eq!(rg.csv_schedule.0[0], 0);
        assert_eq!(rg.csv_schedule.0[10], 10);
        assert_eq!(rg.csv_schedule.0[15], 15);
        assert_eq!(rg.csv_schedule.0[20], 20);
        assert_eq!(rg.csv_schedule.0[28], 28);
        assert_eq!(rg.csv_schedule.0[30], 30);
        assert_eq!(rg.csv_schedule.0[32], 32);

        // Test route times
        assert_eq!(rg.csv_schedule.1[0][0], 0.0);
        assert_eq!(rg.csv_schedule.1[0][7], 11.0);
        assert_eq!(rg.csv_schedule.1[0][15], 23.0);

        assert_eq!(rg.csv_schedule.1[1][0], 0.0);
        assert_eq!(rg.csv_schedule.1[1][5], 8.841666);
        assert_eq!(rg.csv_schedule.1[1][9], 13.875);

        assert_eq!(rg.csv_schedule.1[10][0], 0.0);
        assert_eq!(rg.csv_schedule.1[10][10], 21.783333);
        assert_eq!(rg.csv_schedule.1[10][15], 23.991667);

        assert_eq!(rg.csv_schedule.1[12][0], 0.0);
        assert_eq!(rg.csv_schedule.1[12][6], 18.35);
        assert_eq!(rg.csv_schedule.1[12][10], 23.0);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    #[should_panic]
    fn test_csv_bad_path() {
        parse_routes::read_csv("./routes.csv");
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_visit_count() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Test the route counts
        assert_eq!(
            rg.data.param.N,
            rg.route.len(),
            "The amount of visits in N differ from the length of route data."
        );
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_route_data() {
        let hr2sec: f32 = 3600.0;
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Test the route counts
        assert_eq!(
            rg.route[0].arrival_time, 0.0,
            "Initial arrival time was not at BOD."
        );
        assert!(
            rg.route[0].departure_time == 0.0,
            "The departure time for should equal to the BOD."
        );
        assert_eq!(rg.route[35].arrival_time, 18000.0 / hr2sec);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_route_sort() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Ensure the arrival times are increasing
        for i in 0..rg.route.len() - 1 {
            assert!(
                rg.route[i].arrival_time <= rg.route[i + 1].arrival_time,
                "Arrival times are not increasing in time."
            );
        }
    }
}
