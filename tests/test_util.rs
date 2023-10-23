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

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_shuffle_vec() {
        let v: Vec<u16> = (0..10).collect();
        let v_shuffle = rand_utils::shuffle_vec(&v);

        // Test 1 - first shuffle
        let match_cnt = v
            .iter()
            .zip(v_shuffle.iter())
            .filter(|&(a, b)| a == b)
            .count();
        assert!(match_cnt < v.len());

        // Test 2 - Shuffle the shuffle
        let v: Vec<u16> = v_shuffle.clone();
        let v_shuffle = rand_utils::shuffle_vec(&v);

        let match_cnt = v
            .iter()
            .zip(v_shuffle.iter())
            .filter(|&(a, b)| a == b)
            .count();
        assert!(match_cnt < v.len());
    }
}

//===============================================================================
//
#[cfg(test)]
mod test_triangular_fuzzy_number {
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::util::triangle_fuzzy_number::TriangleFuzzyNumber;

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_add_fuzzy_number() {
        let tfn1: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);
        let tfn2: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(2, 3, 4);

        assert_eq!(tfn1 + tfn2, TriangleFuzzyNumber::new(3, 5, 7));
        assert_eq!(tfn2 + tfn1, TriangleFuzzyNumber::new(3, 5, 7));
        assert_eq!(tfn1 + tfn1, TriangleFuzzyNumber::new(2, 4, 6));
        assert_eq!(tfn2 + tfn2, TriangleFuzzyNumber::new(4, 6, 8));
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_subtract_fuzzy_number() {
        let tfn1: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);
        let tfn2: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(2, 3, 4);

        assert_eq!(tfn2 - tfn1, TriangleFuzzyNumber::new(1, 1, 1));
        assert_eq!(tfn1 - tfn2, TriangleFuzzyNumber::new(-1, -1, -1));
        assert_eq!(tfn1 - tfn1, TriangleFuzzyNumber::new(0, 0, 0));
        assert_eq!(tfn2 - tfn2, TriangleFuzzyNumber::new(0, 0, 0));
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_mult_fuzzy_number() {
        let tfn1: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);
        let tfn2: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(2, 3, 4);

        assert_eq!(tfn1 * tfn2, TriangleFuzzyNumber::new(2, 6, 12));
        assert_eq!(tfn2 * tfn1, TriangleFuzzyNumber::new(2, 6, 12));
        assert_eq!(tfn1 * tfn1, TriangleFuzzyNumber::new(1, 4, 9));
        assert_eq!(tfn2 * tfn2, TriangleFuzzyNumber::new(4, 9, 16));
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_idx_fuzzy_number() {
        let tfn: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);

        assert_eq!(tfn[0], 1);
        assert_eq!(tfn[1], 2);
        assert_eq!(tfn[2], 3);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    #[should_panic]
    fn test_idx_oob_fuzzy_number() {
        let tfn: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);

        tfn[4];
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_idx_mut_fuzzy_number() {
        let mut tfn: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);

        assert_eq!(tfn[0], 1);
        assert_eq!(tfn[1], 2);
        assert_eq!(tfn[2], 3);

        // Update tfn
        tfn[0] = 5;

        assert_eq!(tfn[0], 5);
        assert_eq!(tfn[1], 2);
        assert_eq!(tfn[2], 3);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    #[should_panic]
    fn test_idx_mut_oob_fuzzy_number() {
        let mut tfn: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);

        // Update tfn
        tfn[3] = 5;

        tfn[4];
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_ranking_function() {
        let tfn: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);
        assert_eq!(tfn.ranking_function(), 2.0);

        let tfn: TriangleFuzzyNumber<f32> = TriangleFuzzyNumber::new(2.0, 3.0, 4.0);
        assert_eq!(tfn.ranking_function(), 3.0);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_compare_fuzzy_numbers() {
        let tfn1: TriangleFuzzyNumber<i32> = TriangleFuzzyNumber::new(1, 2, 3);
        // let tfn2: TriangleFuzzyNumber<f32> = TriangleFuzzyNumber::new(2.0, 3.0, 4.0);

        // let ineq = tfn1.partial_cmp(&tfn2);
        //
        // assert!()
    }
}
