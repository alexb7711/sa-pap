extern crate sa_pap;

//===============================================================================
/// TEST PUBLIC INTERFACES OF ROUTE_CSV_GENERATOR

//===============================================================================
//
#[cfg(test)]
mod test_route_csv_generator {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::route::route_csv_generator::{parse_routes, RouteCSVGenerator};
    use super::sa_pap::sa::route::Route;
    use sa_pap::util::fileio::yaml_loader;

    //---------------------------------------------------------------------------
    //
    fn get_route_size() -> usize {
        return yaml_loader::load_yaml(yaml_path())["buses"]["num_visit"]
            .as_i64()
            .unwrap() as usize;
    }

    //---------------------------------------------------------------------------
    //
    fn get_bus_size() -> usize {
        return yaml_loader::load_yaml(yaml_path())["buses"]["num_bus"]
            .as_i64()
            .unwrap() as usize;
    }

    //---------------------------------------------------------------------------
    //
    fn yaml_path() -> &'static str {
        return "./src/config/schedule-test.yaml";
    }

    //---------------------------------------------------------------------------
    //
    fn csv_path() -> &'static str {
        return "./src/config/routes.csv";
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_load_yaml() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path());

        rg.run();

        // assert_eq!(rg.route.get_mut().capacity(), get_route_size());
        // assert_eq!(rg.buses.get_mut().capacity(), get_bus_size());
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_csv_load() {
        let mut p = parse_routes::read_csv(csv_path());

        for r in p.records() {
            println!("{:?}", r);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    #[should_panic]
    fn test_csv_bad_path() {
        parse_routes::read_csv("./routes.csv");
    }
}
