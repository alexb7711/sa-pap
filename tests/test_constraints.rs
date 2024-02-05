#![allow(non_snake_case)]

extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_packing_constraints {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::route::Route;
    use sa_pap::lp::constraints::packing::psi_sigma::PsiSigma;
    use sa_pap::lp::constraints::packing::service_time::ServiceTime;
    use sa_pap::lp::constraints::packing::space_time_big_o::SpaceTimeBigO;
    use sa_pap::lp::constraints::packing::valid_init_dep_end_time::ValidInitDepEndTimes;
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
    fn test_space_time_big_o_psi() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

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
                SpaceTimeBigO::run(&mut rg.data, &mut charger, i, j);
            }
        }
        {
            // Get variables
            let psi = &rg.data.dec.psi;

            // Check output
            assert_eq!(psi[1][0], false);
            assert_eq!(psi[2][0], false);
            assert_eq!(psi[2][1], false);
            assert_eq!(psi[3][0], false);
            assert_eq!(psi[3][1], false);
            assert_eq!(psi[3][2], false);
            assert_eq!(psi[3][4], false);
            assert_eq!(psi[2][3], true);
            assert_eq!(psi[0][0], false);
            assert_eq!(psi[0][1], true);
            assert_eq!(psi[2][3], true);
        }
    }

    //---------------------------------------------------------------------------
    // Test sigma constraint
    #[test]
    fn test_space_time_big_o_sigma() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Set some visit queues
        {
            let u = &mut rg.data.dec.u;
            let d = &mut rg.data.dec.d;
            u[0] = 0.0;
            d[0] = 0.5;
            u[1] = 1.0;
            d[1] = 1.5;
            u[2] = 2.0;
            d[2] = 2.5;
            u[3] = 3.0;
            d[3] = 3.5;
        }

        // Run the `psi_sigma` constraint
        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                SpaceTimeBigO::run(&mut rg.data, &mut charger, i, j);
            }
        }
        {
            // Get variables
            let sigma = &rg.data.dec.sigma;

            // Check output
            assert_eq!(sigma[1][0], false);
            assert_eq!(sigma[2][0], false);
            assert_eq!(sigma[2][1], false);
            assert_eq!(sigma[3][0], false);
            assert_eq!(sigma[3][1], false);
            assert_eq!(sigma[3][2], false);
            assert_eq!(sigma[3][4], false);
            assert_eq!(sigma[2][3], true);
            assert_eq!(sigma[0][0], false);
            assert_eq!(sigma[0][1], true);
            assert_eq!(sigma[2][3], true);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_service_time() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Extract variables
        let n = rg.data.param.N.clone();

        // Update initial and final charge times
        {
            let c = &mut rg.data.dec.d;
            let u = &mut rg.data.dec.u;

            for i in 0..n {
                u[i] = i as f32 * i as f32 * 1.1;
                c[i] = c[i] + i as f32;
            }
        }

        // Run constraint
        for i in 0..n {
            for j in 0..n {
                ServiceTime::run(&mut rg.data, &mut charger, i, j);
            }
        }

        // Check constraints
        let d = &mut rg.data.dec.d;
        let u = &mut rg.data.dec.u;
        let s = &mut rg.data.dec.s;

        for i in 0..n {
            assert_eq!(s[i], d[i] - u[i]);
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_psi_sigma() {
        // Test 0 - Obvious case
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Set some visit queues
        {
            let d = &mut rg.data.dec.d;
            let u = &mut rg.data.dec.u;
            let v = &mut rg.data.dec.v;

            for i in 0..rg.data.param.N {
                let idx = i % rg.data.param.Q;
                u[i] = i as f32;
                d[i] = 0.01 + i as f32;
                v[i] = idx;
            }
        }

        // Run the `psi_sigma` constraint and assert every result is true
        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                assert!(
                    PsiSigma::run(&mut rg.data, &mut charger, i, j),
                    "Failing the obvious case `test_psi_sigma`"
                );
            }
        }

        // Test 1 - All time
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Set some visit queues
        {
            let d = &mut rg.data.dec.d;
            let u = &mut rg.data.dec.u;

            for i in 0..rg.data.param.N {
                u[i] = i as f32;
                d[i] = 0.01 + i as f32;
            }
        }

        // Run the `psi_sigma` constraint and assert every result is true
        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                assert!(
                    PsiSigma::run(&mut rg.data, &mut charger, i, j),
                    "Failing the 'all time' case `test_psi_sigma`"
                );
            }
        }

        // Test 2 - All queue
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());

        // Load the CSV schedule
        rg.run();

        // Set some visit queues
        {
            let v = &mut rg.data.dec.v;
            let d = &mut rg.data.dec.d;
            let u = &mut rg.data.dec.u;
            let s = &mut rg.data.dec.s;

            for i in 0..rg.data.param.N {
                // Set some queue time
                u[i] = 0.0;
                d[i] = 0.1;
                s[i] = d[i] - u[i];

                // All BEBs get a unique charger
                v[i] = i;
            }
        }

        // Run the `psi_sigma` constraint and assert every result is true
        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                assert!(
                    PsiSigma::run(&mut rg.data, &mut charger, i, j),
                    "Failing the 'all queue' case `test_psi_sigma`"
                );
            }
        }
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_valid_init_dep_end_time() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Extract variables
        let n = rg.data.param.N.clone();

        // Update the time horizon
        rg.data.param.T = (n * 10) as f32;

        // Update initial and final charge times
        {
            let a = &mut rg.data.param.a;
            let e = &mut rg.data.param.e;

            let d = &mut rg.data.dec.d;
            let u = &mut rg.data.dec.u;

            for i in 0..n {
                u[i] = a[i];
                d[i] = e[i];
            }
        }

        // Run constraint
        for i in 0..n {
            for j in 0..n {
                assert!(
                    ValidInitDepEndTimes::run(&mut rg.data, &mut charger, i, j),
                    "BEB time constraints did not pass."
                );
            }
        }
    }
}

//===============================================================================
//
#[cfg(test)]
mod test_dynamic_constraints {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Charger;
    use super::sa_pap::sa::route::route_csv_generator::RouteCSVGenerator;
    use super::sa_pap::sa::route::Route;
    use sa_pap::lp::constraints::dynamic::charge_propagation::ChargePropagate;
    use sa_pap::lp::constraints::dynamic::init_final_charge::InitFinalCharge;
    use sa_pap::lp::constraints::dynamic::scalar_to_vector_queue::ScalarToVectorQueue;
    use sa_pap::lp::constraints::packing::valid_init_dep_end_time::ValidInitDepEndTimes;
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
    // Test charge propagation constraint
    #[test]
    fn test_charge_propagation() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Update initial and final charge times
        {
            let a = &mut rg.data.param.a;
            let e = &mut rg.data.param.e;
            let d = &mut rg.data.dec.d;
            let u = &mut rg.data.dec.u;
            let v = &mut rg.data.dec.v;

            for i in 0..rg.data.param.N {
                u[i] = a[i] + 0.001;
                d[i] = e[i] - 0.001;
                v[i] = rg.data.param.Gam[i] as usize;
            }
        }

        // Run constraint
        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                ValidInitDepEndTimes::run(&mut rg.data, &mut charger, i, j);
                assert!(
                    ChargePropagate::run(&mut rg.data, &mut charger, i, j),
                    "Charge did not propagate appropriately."
                );
            }
        }
    }

    //---------------------------------------------------------------------------
    // Test initial/final charge constraint
    #[test]
    fn test_init_final_charge_propagation() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Run constraint

        // Test 0 - check initial charges

        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                // Ensure initial times are updated correctly
                InitFinalCharge::run(&mut rg.data, &mut charger, i, j);
            }
        }

        {
            let alpha = &rg.data.param.alpha;
            let eta = &rg.data.dec.eta;

            for i in 0..rg.data.param.N {
                if alpha[i] > 0.0 {
                    assert_eq!(eta[i], alpha[i] * 387.78);
                }
            }
        }

        // Test 1 - Check final charges

        {
            let eta = &mut rg.data.dec.eta;
            let gam = &rg.data.param.gam;

            for i in 0..rg.data.param.N {
                if gam[i] == -1 {
                    eta[i] = 387.78;
                }
            }
        }

        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                // Ensure final times are updated correctly
                assert!(
                    InitFinalCharge::run(&mut rg.data, &mut charger, i, j),
                    "Charges did not initialize/finalize correctly."
                );
            }
        }
    }

    //---------------------------------------------------------------------------
    // Test initial charge constraint
    #[test]
    fn test_init_charge_propagation() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Run constraint

        // Test 0 - check initial charges

        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                // Ensure initial times are updated correctly
                InitFinalCharge::run(&mut rg.data, &mut charger, i, j);
            }
        }

        {
            let alpha = &rg.data.param.alpha;
            let eta = &rg.data.dec.eta;

            for i in 0..rg.data.param.N {
                if alpha[i] > 0.0 {
                    assert_eq!(eta[i], alpha[i] * 387.78);
                }
            }
        }
    }

    //---------------------------------------------------------------------------
    // Test final charge constraint failure
    #[test]
    #[should_panic]
    fn test_final_charge_insufficient_charge() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Run constraint

        // Check final charges

        // Update initial and final charge times
        {
            let eta = &mut rg.data.dec.eta;
            let gam = &rg.data.param.gam;

            for i in 0..rg.data.param.N {
                if gam[i] == -1 {
                    eta[i] = 387.78 * 0.8;
                }
            }
        }

        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                // Ensure initial/final times are updated correctly
                assert!(
                    InitFinalCharge::run(&mut rg.data, &mut charger, i, j),
                    "Final charge should have panicked."
                );
            }
        }
    }

    //---------------------------------------------------------------------------
    // Test scalar to vector queue constraint
    #[test]
    fn test_scalar_to_vector_queue_propagation() {
        let mut rg: RouteCSVGenerator = RouteCSVGenerator::new(yaml_path(), csv_path());
        let mut charger: Charger = Charger::new(yaml_path(), true, None, None);

        // Load the CSV schedule
        rg.run();

        // Run constraint

        // Update charge queues
        {
            let v = &mut rg.data.dec.v;
            let w = &mut rg.data.dec.w;
            let Gam = &rg.data.param.Gam;

            for i in 0..rg.data.param.N {
                v[i] = Gam[i] as usize;
                w[i][v[i]] = true;
            }
        }

        for i in 0..rg.data.param.N {
            for j in 0..rg.data.param.N {
                // Ensure initial/final times are updated correctly
                assert!(
                    ScalarToVectorQueue::run(&mut rg.data, &mut charger, i, j),
                    "w[i][j].sum() > 1."
                );
            }
        }
    }
}
