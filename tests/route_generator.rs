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
    use sa_pap::util::fileio::yaml_loader;
    use std::cell::Ref;
    use super::sa_pap::sa::generators::Generator;
    use super::sa_pap::sa::generators::route_generator::{RouteGenerator, RouteEvent};

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

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_valid_bus_data()
    {
        let mut rg: RouteGenerator = RouteGenerator::new(false, yaml_path());

        rg.run();

        let e: Ref<Vec<RouteEvent>> = rg.route.borrow();

        // Test 1: Make sure all buses start with an arrival time of 0
        assert!(e[0].arrival_time == e[1].arrival_time);
        assert!(e[1].arrival_time == e[5].arrival_time);
        assert!(e[10].arrival_time == e[15].arrival_time);
        assert!(e[0].arrival_time == e[20].arrival_time);

        // Test 2: Arrival times are sorted from least to greatest
        assert!(e[0].arrival_time < e[30].arrival_time);
        assert!(e[3].arrival_time <= e[20].arrival_time);
        assert!(e[15].arrival_time < e[70].arrival_time);
        assert!(e[100].arrival_time <= e[101].arrival_time);
        assert!(e[160].arrival_time < e[199].arrival_time);
        assert!(e[5].arrival_time < e[30].arrival_time);
        assert!(e[35].arrival_time < e[88].arrival_time);
    }
}
