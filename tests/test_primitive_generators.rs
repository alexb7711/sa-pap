extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_primitive_generators {

    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::generators::primitives::new_visit::*;
    use super::sa_pap::sa::generators::primitives::new_window::*;
    use super::sa_pap::sa::generators::primitives::purge::*;
    use super::sa_pap::sa::generators::primitives::wait::*;

    //---------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str {
        return "./src/config/schedule-test.yaml";
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
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None);

        // Queue index
        let q: usize = 0;

        // Bus ID
        let id: usize = 0;

        // Test 0 - Ensure that the free time is (BOD, EOD)
        assert_eq!(charger.free_time[q][0], (0.0, 10.0));

        // Test 1 - Ensure the size of free times is 1
        assert_eq!(charger.free_time[q].len(), 1);

        // Test 2 - Create a new visit in an empty schedule
        assert!(new_visit::run(&mut charger, id, &(0.01, 0.09)));

        // Test 3 - Ensure the size of free times is now 2
        assert_eq!(charger.free_time[q].len(), 2);

        // Create a new charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None);

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
        assert_eq!(charger.free_time[q][3], (0.7, 10.0));
        assert_eq!(charger.free_time[q].len(), 4);

        // Test 4 - Assign a new bus to be charged in a busy schedule
        assert!(new_visit::run(&mut charger, id, &(0.7, 1.0)));
        assert_eq!(charger.free_time[q].len(), 5);

        // Test 5 - Assign a bus to be charged with an invalid time
        assert_eq!(new_visit::run(&mut charger, id, &(0.2, 0.5)), false);
        assert_eq!(charger.free_time[q].len(), 5);

        // Test 6 - Assign a new bus to be charged in a busy schedule
        assert!(new_visit::run(&mut charger, id, &(0.0, 0.1)));
        assert_eq!(charger.free_time[q].len(), 6);

        // Test 7 - Assign two buses to be charged clone to each other
        assert!(new_visit::run(&mut charger, id, &(0.5, 0.55)));
        assert!(new_visit::run(&mut charger, id, &(0.55, 0.6)));
        assert_eq!(charger.free_time[q].len(), 8);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_wait_purge() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None);

        // Create a simple schedule
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 0;

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
        assert!(wait::run(&mut charger, q, id, &(0.1, 0.2)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.1, 0.2)), true);
        assert_eq!(charger.schedule[q].len(), 3);

        // Test 2
        assert!(purge::run(&mut charger, q, &(0.1, 0.2)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.1, 0.2)), false);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 2
        println!("{:?}", charger.schedule[0]);
        assert_eq!(wait::run(&mut charger, q, id, &(0.1, 0.2)), false);
        assert_eq!(time_slice_exists(&charger, &q, &(0.1, 0.2)), false);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 3
        assert!(wait::run(&mut charger, q, id, &(0.0, 0.02)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.0, 0.02)), true);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 4
        assert!(purge::run(&mut charger, q, &(0.3, 0.5)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.3, 0.5)), false);
        assert_eq!(charger.schedule[q].len(), 1);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_new_window() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None);

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
            new_window::run(&mut charger, q, &(0.1, 0.2), &(0.1, 0.2)),
            true
        );
        assert_eq!(charger.schedule[q].len(), 3);

        // Un-assign and reassign bus
        assert_eq!(
            new_window::run(&mut charger, q, &(0.3, 0.5), &(0.3, 0.5)),
            true
        );
        assert_eq!(charger.schedule[q].len(), 3);
        assert_eq!(charger.exists(&q, &(0.3, 0.5)), false);
        assert_eq!(charger.exists(&q, &(0.3, 0.5)), false);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_slide() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None);

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
        assert_eq!(
            new_window::run(&mut charger, q, &(0.1, 0.2), &(0.1, 0.2)),
            true
        );
        assert_eq!(charger.schedule[q].len(), 3);

        // Un-assign and reassign bus
        assert_eq!(
            new_window::run(&mut charger, q, &(0.3, 0.5), &(0.3, 0.5)),
            true
        );
        assert_eq!(charger.schedule[q].len(), 3);
        assert_eq!(charger.exists(&q, &(0.3, 0.5)), false);
        assert_eq!(charger.exists(&q, &(0.3, 0.5)), false);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_new_charger() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, Some(2));

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
        let c: (f32, f32) = (0.3, 0.5);
        charger.assign(q, c, id);

        // Test 1 - Check the number of assignments
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 2 - Un-assign and reassign bus

        // Un-assign and reassign bus
        if charger.exists(&1, &(0.3, 0.5)) {
            assert_eq!(charger.schedule[1].len(), 2);
            assert_eq!(charger.exists(&1, &(0.3, 0.5)), true);
        } else {
            assert_eq!(charger.schedule[0].len(), 1);
            assert_eq!(charger.exists(&0, &(0.3, 0.5)), true);
        }
    }
}
