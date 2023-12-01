extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_primitive_generators {

    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::data::Data;
    use super::sa_pap::sa::generators::primitives::new_charger::*;
    use super::sa_pap::sa::generators::primitives::new_visit::*;
    use super::sa_pap::sa::generators::primitives::new_window::*;
    use super::sa_pap::sa::generators::primitives::purge::*;
    use super::sa_pap::sa::generators::primitives::slide_visit::*;
    use super::sa_pap::sa::generators::primitives::wait::*;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
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
    fn get_data() -> Box<Data> {
        // Create empty `RouteEvent` vector
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(schedule_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        return rg.get_data();
    }

    //--------------------------------------------------------------------------
    // Returns if the given time slice exists in the current chargers schedule
    //
    fn time_slice_exists(charger: &Charger, q: &usize, c: &(f32, f32)) -> bool {
        if let Some(_) = charger.schedule[*q].iter().find(|s| s.t == *c) {
            return true;
        };

        return false;
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_new_visit() {
        // Get route data
        let mut rd = get_data();

        // Create charger
        let mut charger: Charger = Charger::new(schedule_path(), false, None, None);

        // Queue index
        let q: usize = 0;

        // Bus ID
        let id: usize = 0;

        // Test 0 - Ensure that the free time is (BOD, EOD)
        assert_eq!(charger.free_time[0][q], (0.0, 24.0));

        // Test 1 - Ensure the size of free times is 1
        assert_eq!(charger.free_time[q].len(), 1);

        // Test 2 - Create a new visit in an empty schedule
        assert!(
            new_visit::run(&mut rd, 0, &mut charger, id, &(0.01, 0.09)),
            "Could not create new visit."
        );
        assert_eq!(rd.dec.v[0], 0);
        assert_eq!(rd.dec.w[0][0], true);

        // Test 3 - Ensure the size of free times is now 2
        assert_eq!(charger.free_time[q].len(), 2);

        // Create a new charger
        let mut charger: Charger = Charger::new(schedule_path(), false, None, None);

        // Assign some visits
        let c: (f32, f32) = (0.1, 0.2);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.2, 0.5);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.6, 0.7);
        charger.assign(q, c, id);

        // Test 4 - Check the set free time
        assert_eq!(charger.free_time[q][0], (0.0, 0.1));
        assert_eq!(charger.free_time[q][1], (0.2, 0.2));
        assert_eq!(charger.free_time[q][2], (0.5, 0.6));
        assert_eq!(charger.free_time[q][3], (0.7, 24.0));
        assert_eq!(charger.free_time[q].len(), 4);

        // Test 5 - Assign a new bus to be charged in a busy schedule
        assert!(new_visit::run(&mut rd, 0, &mut charger, id, &(0.7, 1.0)));
        assert_eq!(charger.free_time[q].len(), 5);
        assert_eq!(rd.dec.v[0], id);
        assert_eq!(rd.dec.w[0][0], true);

        // Test 6 - Assign a bus to be charged with an invalid time
        assert_eq!(
            new_visit::run(&mut rd, 0, &mut charger, id, &(0.2, 0.5)),
            false
        );
        assert_eq!(charger.free_time[q].len(), 5);

        // Test 7 - Assign a new bus to be charged in a busy schedule
        for idx in 0..10 {
            if new_visit::run(&mut rd, 0, &mut charger, id, &(0.0, 0.1)) {
                assert!(true);
                break;
            } else if idx + 1 == 10 {
                assert!(false);
            }
        }
        assert_eq!(charger.free_time[q].len(), 6);

        // Test 8 - Assign two buses to be charged close to each other
        assert!(new_visit::run(&mut rd, 0, &mut charger, id, &(0.5, 1.0)));
        assert!(new_visit::run(&mut rd, 0, &mut charger, id, &(1.0, 1.5)));
        assert_eq!(charger.free_time[q].len(), 8);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_wait_purge() {
        // Get route data
        let mut rd = get_data();

        // Create charger
        let mut charger: Charger = Charger::new(schedule_path(), true, Some(rd.param.A), None);

        // Create a simple schedule
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);

        let mut id: usize = 0;
        let mut i: usize = 0;
        for idx in 0..rd.param.N {
            if rd.param.Gam[idx] != 0 {
                id = rd.param.Gam[idx] as usize;
                i = idx;
                break;
            }
        }

        // Assign the charger
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.0, 0.02);

        // Assign the charger
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.3, 0.5);

        // Assign the charger
        charger.assign(q, c, id);

        // Make sure they are all there
        assert!(time_slice_exists(&charger, &q, &(0.1, 0.2)));
        assert!(time_slice_exists(&charger, &q, &(0.0, 0.02)));
        assert!(time_slice_exists(&charger, &q, &(0.3, 0.5)));
        assert_eq!(charger.schedule[q].len(), 3);

        // Test 1
        assert!(wait::run(&mut rd, i, &mut charger, q, id, &(0.1, 0.2)));
        assert_eq!(rd.dec.v[i], id);
        assert_eq!(rd.dec.w[i][rd.dec.v[i]], true);
        assert_eq!(time_slice_exists(&charger, &id, &(0.1, 0.2)), true);
        assert_eq!(charger.schedule[id].len(), 1);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 2
        assert!(purge::run(&mut rd, i, &mut charger, id, &(0.1, 0.2)));
        assert_eq!(rd.dec.v[i], id);
        assert_eq!(rd.dec.w[i][rd.dec.v[i]], true);
        assert_eq!(time_slice_exists(&charger, &id, &(0.1, 0.2)), false);
        assert_eq!(charger.schedule[id].len(), 0);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 2
        println!("{:?}", charger.schedule[0]);
        assert_eq!(
            wait::run(&mut rd, i, &mut charger, q, id, &(0.1, 0.2)),
            false
        );
        assert_eq!(time_slice_exists(&charger, &q, &(0.1, 0.2)), false);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 3
        assert!(wait::run(&mut rd, 0, &mut charger, q, id, &(0.0, 0.02)));
        assert_eq!(time_slice_exists(&charger, &id, &(0.0, 0.02)), true);
        assert_eq!(charger.schedule[id].len(), 1);
        assert_eq!(charger.schedule[q].len(), 1);

        // Test 4
        assert!(purge::run(&mut rd, 0, &mut charger, q, &(0.3, 0.5)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.3, 0.5)), false);
        assert_eq!(charger.schedule[q].len(), 0);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_new_window() {
        // Create charger
        let mut charger: Charger = Charger::new(schedule_path(), false, None, None);

        // Get route data
        let mut rd = get_data();

        // Create a simple schedule
        let q: usize = 0;
        let id: usize = 3;

        let c: (f32, f32) = (0.1, 0.2);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.0, 0.02);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.3, 0.5);
        charger.assign(q, c, id);

        // Test 1 - Check the number of assignments
        assert_eq!(charger.schedule[q].len(), 3);

        // Un-assign and reassign bus
        assert_eq!(
            new_window::run(&mut rd, 0, &mut charger, q, &(0.1, 0.2), &(0.1, 0.2)),
            true
        );
        assert_eq!(charger.schedule[q].len(), 3);

        // Ensure MILP data updates
        // Queue
        assert_eq!(rd.dec.w[0][0], true);
        assert_eq!(rd.dec.v[0], 0);

        // Initial charge time
        let mut found_visit = false;
        for u in rd.dec.u.clone() {
            if u >= 0.1 && u <= 0.2 {
                found_visit = true;
            }
        }
        assert!(found_visit);

        // Final charge time
        let mut found_visit = false;
        for c in rd.dec.c.clone() {
            if c >= 0.1 && c <= 0.2 {
                found_visit = true;
            }
        }
        assert!(found_visit);

        // Un-assign and reassign bus
        assert_eq!(
            new_window::run(&mut rd, 0, &mut charger, q, &(0.3, 0.5), &(0.3, 0.5)),
            true
        );
        assert_eq!(charger.schedule[q].len(), 3);
        assert_eq!(charger.exists(&q, &(0.3, 0.5)), false);

        // Ensure MILP data updates
        // Queue
        assert_eq!(rd.dec.w[0][0], true);
        assert_eq!(rd.dec.v[0], 0);

        // Initial charge time
        let mut found_visit = false;
        for u in rd.dec.u.clone() {
            if u >= 0.3 && u <= 0.5 {
                found_visit = true;
            }
        }
        assert!(found_visit);

        // Final charge time
        let mut found_visit = false;
        for c in rd.dec.c.clone() {
            if c >= 0.3 && c <= 0.5 {
                found_visit = true;
            }
        }
        assert!(found_visit);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_slide() {
        // Create charger
        let mut charger: Charger = Charger::new(schedule_path(), false, None, None);

        // Get route data
        let mut rd = get_data();

        // Create a simple schedule
        let q: usize = 0;
        let id: usize = 3;

        let c: (f32, f32) = (0.1, 0.2);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.0, 0.02);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.3, 0.5);
        charger.assign(q, c, id);

        // Test 1 - Check the number of assignments
        assert_eq!(charger.schedule[q].len(), 3);

        // Test 2 - Un-assign and reassign bus
        assert!(
            new_window::run(&mut rd, 0, &mut charger, q, &(0.1, 0.2), &(0.1, 0.2)),
            "Failed to find new window."
        );
        assert_eq!(charger.schedule[q].len(), 3);
        assert!(rd.dec.u[0] >= 0.1 && rd.dec.u[0] <= 0.2);
        assert!(rd.dec.c[0] >= rd.dec.u[0] && rd.dec.c[0] <= 0.2);

        // Un-assign and reassign bus
        assert!(
            new_window::run(&mut rd, 0, &mut charger, q, &(0.3, 0.5), &(0.3, 0.5),),
            "Failed to find new window."
        );
        assert_eq!(charger.schedule[q].len(), 3);
        assert_eq!(charger.exists(&q, &(0.3, 0.5)), false);
        assert_eq!(charger.exists(&q, &(0.3, 0.5)), false);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_new_charger() {
        // Get route data
        let mut rd = get_data();

        // Create charger
        let mut charger: Charger = Charger::new(schedule_path(), true, Some(rd.param.A), None);

        // Create a simple schedule
        let q: usize = 1;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;
        charger.assign(q, c, id);

        let q: usize = 0;
        let id: usize = 1;
        let c: (f32, f32) = (0.0, 0.02);
        charger.assign(q, c, id);

        let q: usize = 1;
        let id: usize = 2;
        let c: (f32, f32) = (0.3174, 0.5);
        charger.assign(q, c, id);

        // Test 1 - Check the number of assignments
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 2 - Change charger
        assert!(new_charger::run(
            &mut rd,
            0,
            &mut charger,
            1,
            3,
            &(0.1, 0.2)
        ));
        assert_ne!(rd.dec.v[0], 1);
    }
}
