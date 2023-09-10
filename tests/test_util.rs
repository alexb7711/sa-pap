extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_yaml_loader {
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::util::fileio::yaml_loader;

    //---------------------------------------------------------------------------
    // Invalid paths should cause a panic
    #[test]
    #[should_panic]
    fn test_invalid_load_yaml() {
        yaml_loader::load_yaml("bad_file.yaml");
    }

    //---------------------------------------------------------------------------
    // Valid paths should not cause a panic
    #[test]
    fn test_valid_load_yaml() {
        yaml_loader::load_yaml("./src/config/schedule-test.yaml");
    }

    //---------------------------------------------------------------------------
    // Test that the content is what is expected
    #[test]
    fn test_contents() {
        let yaml = yaml_loader::load_yaml("./src/config/schedule-test.yaml");

        assert_eq!(yaml["time"]["EOD"].as_f64().unwrap(), 10.0);
        assert_eq!(yaml["final_charge"].as_f64().unwrap(), 0.95);
        assert_eq!(yaml["initial_charge"]["min"].as_f64().unwrap(), 0.90);
        assert_eq!(yaml["initial_charge"]["max"].as_f64().unwrap(), 0.95);
    }
}

//===============================================================================
//
#[cfg(test)]
mod test_bool_util {
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::util::bool_util;

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_bool_util() {
        // True
        assert!(bool_util::i64_to_bool(1));
        assert!(bool_util::i64_to_bool(5));
        assert!(bool_util::i64_to_bool(-1));

        // False
        assert!(!bool_util::i64_to_bool(0));
    }
}

//===============================================================================
//
#[cfg(test)]
mod test_rand_utils {
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::util::rand_utils;

    //---------------------------------------------------------------------------
    //
    fn get_vec_size(vec: &Vec<u16>) -> usize {
        return vec.len();
    }

    //---------------------------------------------------------------------------
    //
    fn get_vec_count(vec: &Vec<u16>) -> u16 {
        return vec.iter().sum();
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_rand_route_count() {
        // Test objects
        let a = rand_utils::rand_route_count(1, 10);
        let b = rand_utils::rand_route_count(10, 100);
        let c = rand_utils::rand_route_count(30, 400);

        // Test each object length
        assert_eq!(get_vec_size(&a), 1);
        assert_eq!(get_vec_size(&b), 10);
        assert_eq!(get_vec_size(&c), 30);

        // Test each object sum
        assert_eq!(get_vec_count(&a), 10);
        assert_eq!(get_vec_count(&b), 100);
        assert_eq!(get_vec_count(&c), 400);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_range() {
        let lower: f32 = 1.0;
        let upper: f32 = 100.0;

        let mut v: f32 = rand_utils::rand_range(lower, upper);
        assert!(v >= lower && v <= upper);

        v = rand_utils::rand_range(lower, upper);
        assert!(v >= lower && v <= upper);

        v = rand_utils::rand_range(lower, upper);
        assert!(v >= lower && v <= upper);
    }
}
