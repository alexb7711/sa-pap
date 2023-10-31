extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_constraints {

    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::lp::objectives::std_obj::StdObj;
    use super::sa_pap::lp::objectives::Objective;
    use super::sa_pap::sa::data::Data;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::route::Route;

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
    fn get_data() -> Data {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        return rg.data;
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_std_obj() {
        // Create objective and data object
        let mut o: StdObj = StdObj {};
        let mut d: Data = get_data();

        // Set some dummy values

        // w
        d.dec.w = vec![vec![false; d.param.Q]; d.param.N];

        // g
        d.dec.g = vec![vec![0.0; d.param.Q]; d.param.N];

        // Test 0 - test w terms
        d.dec.w[0][0] = true;
        d.dec.w[1][0] = true;
        let j = o.run(&mut d);
        assert_eq!(j, 2000.0);

        // Test 1 - test w terms
        d.dec.w[5][4] = true;
        let j = o.run(&mut d);
        assert_eq!(j, 7000.0);

        // Test 2 - reset w terms
        d.dec.w = vec![vec![false; d.param.Q]; d.param.N];
        let j = o.run(&mut d);
        assert_eq!(j, 0.0);

        // Test 3 - test w terms
        d.dec.w[0][1] = true;
        d.dec.w[1][1] = true;
        d.dec.w[5][6] = true;
        let j = o.run(&mut d);
        assert_eq!(j, 11000.0);

        // Reset w terms
        d.dec.w = vec![vec![false; d.param.Q]; d.param.N];

        // Test 4 - set g terms
        d.dec.g[0][0] = 1.0;
        let j = o.run(&mut d);
        assert_eq!(j, 100.0);

        // Test 4 - set g terms
        d.dec.g[10][8] = 1.0;
        let j = o.run(&mut d);
        assert_eq!(j, 500.0);

        // Test 4 - set g and w terms
        // Test 0 - test w terms
        d.dec.w[0][0] = true;
        d.dec.w[1][0] = true;
        let j = o.run(&mut d);
        assert_eq!(j, 2500.0);
    }
}
