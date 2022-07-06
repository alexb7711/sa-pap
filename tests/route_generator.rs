extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_route_generator
{
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::route_generator::{RouteGenerator, Generator};
    use sa_pap::util::fileio::yaml_loader;

    //---------------------------------------------------------------------------
    //
    fn get_size() -> usize
    {
        return yaml_loader::load_yaml(yaml_path())["buses"]["num_visit"].as_i64().unwrap() as usize;
    }

    //---------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str
    {
        return "./src/yaml/schedule.yaml";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_valid_load_yaml()
    {
        let mut rg: RouteGenerator = RouteGenerator::new(false, yaml_path());

        rg.run();

        assert_eq!(rg.route.get_mut().capacity(), get_size());
    }
}
