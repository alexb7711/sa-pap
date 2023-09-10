extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_charger {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Assignment;
    use super::sa_pap::sa::charger::Charger;

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
    fn test_charger_addition() {
        // Create charger
        let mut charger: Charger = Charger::new(None);

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
        let mut charger: Charger = Charger::new(None);

        // Add queues (four three chargers)
        charger.add_chargers(2);

        // Test 1
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
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_deletion() {}

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_ordering() {}

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_charger_avail() {
        // Create charger
        let mut charger: Charger = Charger::new(None);

        // Add queues (four total chargers)
        charger.add_chargers(3);

        // Test 1
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;

        // Ensure that the charger space is available
        assert!(charger.avail(&q, &c));

        // Assign the charger
        charger.assign(q, c, id);
    }
}
