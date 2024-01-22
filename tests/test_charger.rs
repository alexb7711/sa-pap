extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_charger {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Assignment;
    use super::sa_pap::sa::charger::Charger;

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
    fn test_charger_initilazation() {
        // Load charger parameters from YAML file
        let charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Test 0 - Ensure the correct amount of chargers have been created
        let cc = charger.charger_count.1 + charger.charger_count.2;
        assert_eq!(cc, 11);

        // Test 1 - include the non-charger spots
        let cc = charger.charger_count.0 + charger.charger_count.1 + charger.charger_count.2;
        assert_eq!(cc, 22);

        // Test 2 - charger speeds
        let cc = charger.charger_speed;
        assert_eq!(cc, (0.0, 100.0, 400.0));
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_addition() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Make sure we have an empty charger queue
        assert_eq!(charger.schedule.is_empty(), false);

        // Test 1
        assert_eq!(charger.schedule.len(), 1);

        // Test 2
        charger.add_chargers(1);
        assert_eq!(charger.schedule.len(), 2);

        // Test 3
        charger.add_chargers(2);
        assert_eq!(charger.schedule.len(), 4);

        // Test 4
        charger.add_chargers(5);
        assert_eq!(charger.schedule.len(), 9);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_assignment() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Add queues (four three chargers)
        charger.add_chargers(2);

        // Test 0
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;

        // Ensure that the charger space is available
        assert!(charger.avail(&q, &c));

        // Assign the charger
        charger.assign(q, c, id);

        assert_eq!(charger.schedule[0][0], Assignment { t: c, b: id });

        // Test 1
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;

        // Assign the charger
        charger.assign(q, c, id);

        assert_eq!(charger.schedule[q][0], Assignment { t: c, b: id });

        // Test 2
        let q: usize = 1;
        let c: (f32, f32) = (0.0, 0.5);
        let id: usize = 2;

        // Assign the charger
        charger.assign(q, c, id);

        assert_eq!(charger.schedule[q][0], Assignment { t: c, b: id });

        // Test 3
        let q: usize = 2;
        let c: (f32, f32) = (1.0, 2.5);
        let id: usize = 1;

        // Assign the charger
        charger.assign(q, c, id);

        assert_eq!(charger.schedule[q][0], Assignment { t: c, b: id });

        // Test 4
        let q: usize = 0;
        let c: (f32, f32) = (0.19, 0.2);
        let id: usize = 1;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), false);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), false);

        // Test 5
        let q: usize = 0;
        let c: (f32, f32) = (0.25, 0.5);
        let id: usize = 2;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), true);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), true);

        // Test 6
        let q: usize = 1;
        let c: (f32, f32) = (0.1, 0.6);
        let id: usize = 1;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), false);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), false);

        // Test 7
        let q: usize = 2;
        let c: (f32, f32) = (0.5, 1.1);
        let id: usize = 0;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), false);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), false);

        // Test 8
        let q: usize = 2;
        let c: (f32, f32) = (0.1, 0.9);
        let id: usize = 0;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), true);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), true);

        // Test 9
        let q: usize = 2;
        let c: (f32, f32) = (0.5, 1.1);
        let id: usize = 0;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), false);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), false);

        // Test 9
        let q: usize = 2;
        let c: (f32, f32) = (0.5, 1.1);
        let id: usize = 0;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), false);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), false);

        // Test 10
        let q: usize = 2;
        let c: (f32, f32) = (0.9, 2.0);
        let id: usize = 0;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), false);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), false);

        // Test 11 - Ordering
        let q: usize = 2;
        let c: (f32, f32) = (10.0, 5.0);
        let id: usize = 0;

        // Assign the charger
        assert_eq!(charger.assign(q, c, id), false);

        // Make sure that the time slice was not assigned
        assert_eq!(time_slice_exists(&charger, &q, &c), false);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_deletion() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Create a simple schedule
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;

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
        assert!(charger.remove(q, (0.1, 0.2)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.1, 0.2)), false);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 2
        println!("{:?}", charger.schedule[0]);
        assert_eq!(charger.remove(q, (0.1, 0.2)), false);
        assert_eq!(time_slice_exists(&charger, &q, &(0.1, 0.2)), false);
        assert_eq!(charger.schedule[q].len(), 2);

        // Test 3
        assert!(charger.remove(q, (0.0, 0.02)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.0, 0.02)), false);
        assert_eq!(charger.schedule[q].len(), 1);

        // Test 4
        assert!(charger.remove(q, (0.3, 0.5)));
        assert_eq!(time_slice_exists(&charger, &q, &(0.3, 0.5)), false);
        assert_eq!(charger.schedule[q].len(), 0);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_ordering() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Create a simple schedule
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.6, 0.7);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.0, 0.02);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.3, 0.5);
        charger.assign(q, c, id);

        // Get the first item in the schedule
        let mut s_prev = charger.schedule[q][0].clone();

        // Iterate through the schedule
        for s in charger.schedule[q].iter().skip(1) {
            // Ensure that the times are always increasing
            assert!(s_prev.t.0 < s_prev.t.1);
            assert!(s_prev.t.1 < s.t.0);
            assert!(s.t.0 < s.t.1);

            // Set the previous to the current
            s_prev = s.clone();
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_avail() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Add queues (four total chargers)
        charger.add_chargers(3);

        // Test 1 - Assign to empty queue
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;

        // Ensure that the charger space is available
        assert!(charger.avail(&q, &c));

        // Assign the charger
        charger.assign(q, c, id);

        // Test 2 - Check if previous assigned charger is still available
        assert_eq!(charger.avail(&q, &c), false);

        // Test 3 - Assign a few new time slices
        let q: usize = 0;
        let c: (f32, f32) = (0.21, 0.23);
        let id: usize = 3;
        assert!(charger.avail(&q, &c));
        charger.assign(q, c, id);

        let q: usize = 0;
        let c: (f32, f32) = (0.01, 0.09);
        let id: usize = 3;
        assert!(charger.avail(&q, &c));
        charger.assign(q, c, id);

        let q: usize = 0;
        let c: (f32, f32) = (0.4, 0.51);
        let id: usize = 3;
        assert!(charger.avail(&q, &c));
        charger.assign(q, c, id);

        // Test 4 - Assign a huge time
        let q: usize = 0;
        let c: (f32, f32) = (0.0, 1.0);
        assert_eq!(charger.avail(&q, &c), false);

        // Test 5 - Assign a small but unavailable time
        let q: usize = 0;
        let c: (f32, f32) = (0.02, 0.03);
        assert_eq!(charger.avail(&q, &c), false);

        // Test 6 - Assign a small but available time
        let q: usize = 0;
        let c: (f32, f32) = (0.331, 0.339);
        assert_eq!(charger.avail(&q, &c), true);

        // Test 7 - Assign a small but unavailable time
        let q: usize = 0;
        let c: (f32, f32) = (0.401, 0.403);
        assert_eq!(charger.avail(&q, &c), false);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_get_ts() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Add queues (four total chargers)
        charger.add_chargers(3);

        // Test 1 - Assign to empty queue
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;

        // Ensure that the charger space is available
        assert_ne!(charger.get_ts(&q, &c), (0.0, 0.0));

        // Assign the charger
        charger.assign(q, c, id);

        // Test 2 - Check if previous assigned charger is still available
        assert_eq!(charger.get_ts(&q, &c), (0.0, 0.0));

        // Test 3 - Assign a few new time slices
        let q: usize = 0;
        let c: (f32, f32) = (0.21, 0.23);
        let id: usize = 3;
        assert_ne!(charger.get_ts(&q, &c), (0.0, 0.0));
        charger.assign(q, c, id);

        let q: usize = 0;
        let c: (f32, f32) = (0.01, 0.09);
        let id: usize = 3;
        assert_ne!(charger.get_ts(&q, &c), (0.0, 0.0));
        charger.assign(q, c, id);

        let q: usize = 0;
        let c: (f32, f32) = (0.4, 0.51);
        let id: usize = 3;
        assert_ne!(charger.get_ts(&q, &c), (0.0, 0.0));
        charger.assign(q, c, id);

        // Test 4 - Assign a huge time
        let q: usize = 0;
        let c: (f32, f32) = (0.0, 1.0);
        assert_eq!(charger.get_ts(&q, &c), (0.0, 0.0));

        // Test 5 - Assign a small but unavailable time
        let q: usize = 0;
        let c: (f32, f32) = (0.02, 0.03);
        assert_eq!(charger.get_ts(&q, &c), (0.0, 0.0));

        // Test 6 - Assign a small but available time
        let q: usize = 0;
        let c: (f32, f32) = (0.331, 0.339);
        assert_ne!(charger.get_ts(&q, &c), (0.0, 0.0));

        // Test 7 - Assign a small but unavailable time
        let q: usize = 0;
        let c: (f32, f32) = (0.401, 0.403);
        assert_eq!(charger.get_ts(&q, &c), (0.0, 0.0));
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_free_time() {
        // Create charger
        let q: usize = 0;
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Test 0
        assert_eq!(charger.free_time[q].is_empty(), false);
        assert_eq!(charger.free_time[q][0], (0.0, 24.0));

        // Create a simple schedule
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.4, 0.5);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.7, 0.8);
        charger.assign(q, c, id);

        // Test 1
        assert_eq!(charger.free_time[q].is_empty(), false);

        // Test 2
        assert_eq!(charger.free_time[q][0], (0.0, 0.1));
        assert_eq!(charger.free_time[q][1], (0.2, 0.4));
        assert_eq!(charger.free_time[q][2], (0.5, 0.7));
        assert_eq!(charger.free_time[q][3], (0.8, 24.0));

        // Test 3
        let c: (f32, f32) = (0.1, 0.2);
        charger.remove(q, c);
        assert_eq!(charger.free_time[q][0], (0.0, 0.4));
        assert_eq!(charger.free_time[q][1], (0.5, 0.7));
        assert_eq!(charger.free_time[q][2], (0.8, 24.0));

        let c: (f32, f32) = (0.4, 0.5);
        charger.remove(q, c);
        assert_eq!(charger.free_time[q][0], (0.0, 0.7));
        assert_eq!(charger.free_time[q][1], (0.8, 24.0));

        let c: (f32, f32) = (0.7, 0.8);
        charger.remove(q, c);
        assert_eq!(charger.free_time[q][0], (0.0, 24.0));
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_find_free_time() {
        // Create charger
        let q: usize = 0;
        let mut charger: Charger = Charger::new(yaml_path(), false, None, None);

        // Create a simple schedule
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.4, 0.5);
        charger.assign(q, c, id);

        let c: (f32, f32) = (0.7, 0.8);
        charger.assign(q, c, id);

        // Test 1 - fully valid request
        let ts = charger.free_time[q][0];
        let (fits, ud) = charger.find_free_time(&(0.0, 0.1), &ts);
        if fits {
            assert!(fits);
            assert!(ud.0 >= 0.0);
            assert!(ud.1 <= 0.1);
        } else {
            assert!(!fits);
        }

        // Test 2 - lower bound overlap
        let ts = charger.free_time[q][1];
        let (fits, ud) = charger.find_free_time(&(0.19, 0.3), &ts);
        if fits {
            assert!(fits);
            assert!(ud.0 >= 0.2);
            assert!(ud.1 <= 0.3);
        } else {
            assert!(!fits);
        }

        // Test 3 - upper bound overlap
        let ts = charger.free_time[q][1];
        let (fits, ud) = charger.find_free_time(&(0.2, 0.51), &ts);
        if fits {
            assert!(fits);
            assert!(ud.0 >= 0.2);
            assert!(ud.1 <= 0.4);
        } else {
            assert!(!fits);
        }

        // Test 4 - lower/upper bound overlap
        let ts = charger.free_time[q][1];
        let (fits, ud) = charger.find_free_time(&(0.0, 0.51), &ts);
        if fits {
            assert!(fits);
            assert!(ud.0 >= 0.2);
            assert!(ud.1 <= 0.4);
        } else {
            assert!(!fits);
        }

        // Test 5 - times do not match up
        let ts = charger.free_time[q][1];
        let (fits, ud) = charger.find_free_time(&(0.11, 0.19), &ts);
        if fits {
            assert!(!fits);
            assert!(ud.0 == 0.11);
            assert!(ud.1 == 0.19);
        } else {
            assert!(!fits);
        }

        // Test 6 - invalid request
        let ts = charger.free_time[q][0];
        let (fits, ud) = charger.find_free_time(&(0.1, 0.2), &ts);
        if fits {
            assert!(!fits);
            assert!(ud.0 == 0.1);
            assert!(ud.1 == 0.2);
        } else {
            assert!(!fits);
        }
    }
}
