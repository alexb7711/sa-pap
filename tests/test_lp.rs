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
        let mut d: Data = get_data();

        // Set some dummy values

        // w
        d.dec.w = vec![vec![false; d.param.Q]; d.param.N];

        // s
        d.dec.s = vec![0.0; d.param.N];

        // Test w terms

        // Test 0
        d.dec.w[0][0] = true;
        d.dec.w[1][0] = true;
        let j = StdObj::run(&mut d);
        assert_eq!(j, 0.0);

        // Test 1
        d.dec.w[0][35] = true;
        let j = StdObj::run(&mut d);
        assert_eq!(j, 1000.0);

        // Test 2
        d.dec.w = vec![vec![false; d.param.Q]; d.param.N];
        d.dec.w[0][35] = true;
        d.dec.w[3][38] = true;
        let j = StdObj::run(&mut d);
        assert_eq!(j, 5000.0);

        // Test 3
        d.dec.w[2][35] = true;
        d.dec.w[1][42] = true;
        d.dec.w[5][37] = true;
        let j = StdObj::run(&mut d);
        assert_eq!(j, 17000.0);

        // Reset w terms
        d.dec.w = vec![vec![false; d.param.Q]; d.param.N];

        // Test both terms

        // Test 4
        d.dec.s[0] = 1.0;
        d.dec.w[0][35] = true;
        let j = StdObj::run(&mut d);
        assert_eq!(j, 1100.0);

        // Test 5
        d.dec.w[3][36] = true;
        d.dec.s[3] = 3.0;
        let j = StdObj::run(&mut d);
        assert_eq!(j, 3400.0);

        // Test 6
        d.dec.w[5][40] = true;
        d.dec.s[5] = 9.0;
        d.dec.w[1][35] = true;
        d.dec.s[1] = 1.0;
        let j = StdObj::run(&mut d);
        assert_eq!(j, 11400.0);
    }
}
