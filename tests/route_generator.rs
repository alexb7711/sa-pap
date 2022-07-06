extern crate sa_pap;

//===============================================================================
/// TEST PUBLIC INTERFACES OF ROUTE_GENERATOR

//===============================================================================
//
#[cfg(test)]
mod test_route_generator
{
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::route_generator::{RouteGenerator, Generator};
    use sa_pap::util::fileio::yaml_loader;

    //---------------------------------------------------------------------------
    //
    fn get_route_size() -> usize
    {
        return yaml_loader::load_yaml(yaml_path())["buses"]["num_visit"].as_i64().unwrap() as usize;
    }

    //---------------------------------------------------------------------------
    //
    fn get_bus_size() -> usize
    {
        return yaml_loader::load_yaml(yaml_path())["buses"]["num_bus"].as_i64().unwrap() as usize;
    }

    //---------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str
    {
        return "./src/yaml/schedule-test.yaml";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_valid_load_yaml()
    {
        let mut rg: RouteGenerator = RouteGenerator::new(false, yaml_path());

        rg.run();

        assert_eq!(rg.route.get_mut().capacity(), get_route_size());
        assert_eq!(rg.buses.get_mut().capacity(), get_bus_size());
    }
}
