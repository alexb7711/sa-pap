extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_packing_constraints {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::route::Route;
    use sa_pap::lp::constraints::packing::space_time_big_o::SpaceTimeBigO;
    use sa_pap::lp::constraints::Constraint;

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
    // Test psi constraint
    #[test]
    fn test_psi() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Set some visit queues
        {
            let v = &mut rg.data.dec.v;
            v[0] = 0;
            v[1] = 1;
            v[2] = 2;
            v[3] = 3;
        }

        // Run the `psi_sigma` constraint
        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                SpaceTimeBigO::run(&mut rg.data, i, j);
            }
        }
        {
            // Get variables
            let psi = &rg.data.dec.psi;

            // Check output
            assert_eq!(psi[1][0], true);
            assert_eq!(psi[2][0], true);
            assert_eq!(psi[2][1], true);
            assert_eq!(psi[3][0], true);
            assert_eq!(psi[3][1], true);
            assert_eq!(psi[3][2], true);
            assert_eq!(psi[3][4], true);
            assert_eq!(psi[2][3], false);
            assert_eq!(psi[0][0], false);
            assert_eq!(psi[0][1], false);
            assert_eq!(psi[2][3], false);
        }
    }

    //---------------------------------------------------------------------------
    // Test sigma constraint
    #[test]
    fn test_sigma() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Set some visit queues
        {
            let u = &mut rg.data.dec.u;
            u[0] = 0.0;
            u[1] = 1.0;
            u[2] = 2.0;
            u[3] = 3.0;
        }

        // Run the `psi_sigma` constraint
        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                SpaceTimeBigO::run(&mut rg.data, i, j);
            }
        }
        {
            // Get variables
            let sigma = &rg.data.dec.sigma;

            // Check output
            assert_eq!(sigma[1][0], true);
            assert_eq!(sigma[2][0], true);
            assert_eq!(sigma[2][1], true);
            assert_eq!(sigma[3][0], true);
            assert_eq!(sigma[3][1], true);
            assert_eq!(sigma[3][2], true);
            assert_eq!(sigma[3][4], true);
            assert_eq!(sigma[2][3], false);
            assert_eq!(sigma[0][0], false);
            assert_eq!(sigma[0][1], false);
            assert_eq!(sigma[2][3], false);
        }
    }

    //---------------------------------------------------------------------------
    // Invalid paths should cause a panic
    #[test]
    fn test_service_time() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();
    }

    //---------------------------------------------------------------------------
    // Invalid paths should cause a panic
    #[test]
    fn test_space_time_big_o() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();
    }

    //---------------------------------------------------------------------------
    // Invalid paths should cause a panic
    #[test]
    fn test_valit_init_dep_end_time() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();
    }
}
