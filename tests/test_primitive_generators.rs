extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_primitive_generators {

    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::generators::primitives::new_visit::*;

    //---------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str {
        return "./src/config/schedule-test.yaml";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_new_visit() {
        // Create charger
        let mut charger: Charger = Charger::new(yaml_path(), None);

        // Queue index
        let q: usize = 0;

        // Bus ID
        let id: usize = 3;

        // Test 0 - Ensure that the free time is (BOD, EOD)
        assert_eq!(charger.free_time[q][0], (0.0, 10.0));

        // Test 1 - Ensure the size of free times is 1
        assert_eq!(charger.free_time[q].len(), 1);

        // Test 2 - Create a new visit in an empty schedule
        assert!(new_visit::run(&mut charger, id, (0.01, 0.09)));

        // Test 3 - Ensure the size of free times is now 2
        assert_eq!(charger.free_time[q].len(), 2);

        // Create a new charger
        let mut charger: Charger = Charger::new(yaml_path(), None);

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
        assert!(new_visit::run(&mut charger, id, (0.7, 1.0)));
        assert_eq!(charger.free_time[q].len(), 5);
    }
}
