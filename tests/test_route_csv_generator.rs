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
    fn schedule_path() -> &'static str {
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
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

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
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

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
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Test the route counts

        // Find the first instance for BEB 0 arrival
        let mut idx = 0;
        for i in 0..rg.data.param.N {
            if rg.route[i].id == 0 {
                idx = i;
                break;
            }
        }

        assert_eq!(
            rg.route[idx].arrival_time, 0.0,
            "Initial arrival time was not at BOD."
        );
        assert_ne!(
            rg.route[idx].departure_time, 0.0,
            "The departure time for should equal to the BOD."
        );

        // Find the first instance for BEB 35 arrival
        let mut idx = 0;
        for i in 0..rg.data.param.N {
            if rg.route[i].id == 35 {
                idx = i;
                break;
            }
        }

        assert_eq!(rg.route[idx].arrival_time, 0.0);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_route_sort() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

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

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charge_rate_vector() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Ensure the arrival times are increasing
        for i in 0..rg.data.param.r.len() {
            let r: f32;
            if i < 35 {
                r = 0.0;
            } else if i < 42 {
                r = 100.0
            } else {
                r = 400.0
            }
            assert_eq!(rg.data.param.r[i], r);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charge_assignment_vector() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Ensure the arrival times are increasing
        for i in 0..rg.data.param.r.len() {
            assert_eq!(rg.data.param.r[i], rg.data.param.ep[i]);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_assignment_cost() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Ensure the arrival times are increasing
        for i in 0..rg.data.param.m.len() {
            let m: usize;
            if i < 35 {
                m = 0;
            } else {
                m = 1000 * (i - 34);
            }

            assert_eq!(rg.data.param.m[i], m);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_route_visit_index() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Check the index of the routes increases
        for i in 0..rg.route.len() {
            assert_eq!(rg.route[i].visit, i);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_milp_data_update() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

        // Get a copy of the MILP data
        let data_cpy = rg.get_data().clone();

        // Load the CSV schedule
        rg.run();

        // Change some things in the route data
        rg.get_route_events()[0].arrival_time = 10.0;

        let id: u16;
        if rg.get_route_events()[30].id != 16 {
            id = 16;
            rg.get_route_events()[30].id = id;
        } else {
            id = 17;
            rg.get_route_events()[30].id = id;
        }

        rg.get_route_events()[8].departure_time = 70.0;
        rg.get_route_events()[5].detach_time = 4.0;
        rg.get_route_events()[16].attach_time = 12.0;

        // Assert that charges have been made to the route data
        {
            let milp = rg.get_data().clone();
            let re = rg.get_route_events();

            // Assert that route data is different than MILP data
            assert_ne!(re[0].arrival_time, milp.param.a[0]);
            assert_ne!(re[30].id, milp.param.Gam[30]);
            assert_ne!(re[8].departure_time, milp.param.e[8]);
            assert_ne!(re[5].detach_time, milp.dec.c[5]);
            assert_ne!(re[16].attach_time, milp.dec.u[16]);

            // Assert that that the data was changed to what was expected
            assert_eq!(re[0].arrival_time, 10.0);
            assert_eq!(re[30].id, id);
            assert_eq!(re[8].departure_time, 70.0);
            assert_eq!(re[5].detach_time, 4.0);
            assert_eq!(re[16].attach_time, 12.0);
        }

        // Update milp data
        rg.update_milp_data();

        // Assert that the MILP data matches the route data
        let milp = rg.get_data().clone();
        let re = rg.get_route_events();

        for i in 0..milp.param.A {
            assert_eq!(re[i].visit, i);
            assert_eq!(re[i].id, milp.param.Gam[i]);
            assert_eq!(re[i].arrival_time, milp.param.a[i]);
            assert_eq!(re[i].departure_time, milp.param.e[i]);
            assert_eq!(re[i].attach_time, milp.dec.u[i]);
            assert_eq!(re[i].detach_time, milp.dec.c[i]);
        }

        // Assert that the data has changed
        assert_ne!(data_cpy, milp);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_route_data_update() {
        let mut rg: Box<dyn Route> = Box::new(RouteCSVGenerator::new(schedule_path(), csv_path()));

        // Load the CSV schedule
        rg.run();

        // Change some things in MILP data. Note `get_data` returns a copy of the MILP data, not a reference.
        rg.get_data().param.a[0] = 10.0;

        // Make sure we never have a failure
        let id;
        if rg.get_data().param.Gam[10] != 32 {
            id = 32;
            rg.get_data().param.Gam[10] = id;
        } else {
            id = 33;
            rg.get_data().param.Gam[10] = id;
        }

        rg.get_data().param.e[8] = 70.0;

        // Assert that charges have not been made to the route data
        {
            let milp = rg.get_data().clone();
            let re = rg.get_route_events();

            // Assert that route data is the same as MILP data
            assert_eq!(re[0].arrival_time, milp.param.a[0]);
            assert_eq!(re[10].id, milp.param.Gam[10]);
            assert_eq!(re[8].departure_time, milp.param.e[8]);

            // Assert that that the data not was changed to what was expected
            assert_ne!(milp.param.a[0], 10.0);
            assert_ne!(milp.param.Gam[10], id);
            assert_ne!(milp.param.e[8], 70.0);
        }
    }
}
