extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_yaml_loader
{
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::util::fileio::yaml_loader;

    //---------------------------------------------------------------------------
    // Invalid paths should cause a panic
    #[test]
    #[should_panic]
    fn test_invalid_load_yaml()
    {
        yaml_loader::load_yaml("bad_file.yaml");
    }

    //---------------------------------------------------------------------------
    // Valid paths should not cause a panic
    #[test]
    fn test_valid_load_yaml()
    {
        yaml_loader::load_yaml("./src/yaml/schedule.yaml");
    }

    //---------------------------------------------------------------------------
    // Test that the content is what is expected
    #[test]
    fn test_contents()
    {
        let yaml = yaml_loader::load_yaml("./src/yaml/schedule.yaml");

        assert_eq!(yaml["time_horizon"].as_i64().unwrap(), 24);
        assert_eq!(yaml["final_charge"].as_f64().unwrap(), 0.95);
        assert_eq!(yaml["initial_charge"]["max"].as_f64().unwrap(), 0.95);
        assert_eq!(yaml["initial_charge"]["min"].as_f64().unwrap(), 0.90);
    }
}

//===============================================================================
//
#[cfg(test)]
mod test_bool_util
{
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::util::bool_util;

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_bool_util()
    {
        // True
        assert!(bool_util::i64_to_bool(1));
        assert!(bool_util::i64_to_bool(5));
        assert!(bool_util::i64_to_bool(-1));

        // False
        assert!(!bool_util::i64_to_bool(0));
    }
}
