#![allow(non_snake_case)]

//===============================================================================
// Declare submodules
use self::temp_func::TempFunc;
use crate::lp::objectives::std_obj::StdObj;
use crate::lp::objectives::Objective;
use crate::sa::charger::Charger;
use crate::sa::generators::Generator;
use crate::sa::route::Route;

//===============================================================================
// Import standard library
use rand::{thread_rng, Rng};

//===============================================================================
// Import modules
pub mod charger; // Parameters and decision variables
pub mod data; // Parameters and decision variables
pub mod generators; // Pool of all the SA generators
pub mod route; // Pool of all the route generators
pub mod temp_func; // Temperature functions

//===============================================================================
/// Results from simulated annealing
//
#[derive(Default)]
pub struct Results {}

//===============================================================================
/// Structure for simulated annealing
//
#[allow(dead_code)]
pub struct SA<'a> {
    gsol: Box<dyn Generator>,   // Solution generator
    gsys: Box<dyn Route>,       // Route generator
    gtweak: Box<dyn Generator>, // Solution perterber
    charger: Box<Charger>,      // Charge schedule keeper
    r: Results,                 // Results
    tf: &'a mut Box<TempFunc>,  // Cooling Schedule
}

//===============================================================================
/// Implementation of SA
//
impl<'a> SA<'a> {
    //---------------------------------------------------------------------------
    /// Initialize the SA object
    ///
    /// # Input
    /// * `g` : Solution generator
    /// * `tf` : The temperature function to use
    /// * `ts` : Tweak schedule
    ///
    /// # Output
    /// * `sa`: Simulated annealing structure
    ///
    pub fn new(
        config_path: &'a str,
        gsol: Box<dyn Generator>,
        gsys: Box<dyn Route>,
        gtweak: Box<dyn Generator>,
        tf: &'a mut Box<TempFunc>,
    ) -> SA<'a> {
        let A = Some(gsys.get_data().param.A);
        let sa: SA = SA {
            gsol,
            gsys,
            gtweak,
            charger: Box::new(Charger::new(config_path, true, A, None)),
            r: Default::default(),
            tf,
        };

        return sa;
    }

    //---------------------------------------------------------------------------
    /// Initialize and run the SA algorithm
    ///
    /// # Input
    /// * lff: Load previous results from file
    ///
    /// # Output
    /// * `Results`: Output of SA algorithm
    ///
    pub fn run(self: &mut SA<'a>, _lff: bool) -> Option<Results> {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Initialize

        // Generate new solution
        self.gsys.run();

        // Set local search iteration count
        let k = 1000;

        // Initialize objective function variables
        let mut J0: f64;
        let mut J1: f64 = 0.0;

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Execute SA

        // While the temperature function is cooling down
        while let Some(t) = self.tf.step() {
            // Generate new solution
            self.gsys.run();

            // Calculate objective function
            J0 = StdObj::run(&mut self.gsys.get_data());

            // Compare the objective functions
            self.cmp_obj_fnc(J0, J1, t);

            // Iterate though local search
            for _ in 0..k {
                // Tweak the schedule
                self.gtweak.run(&mut self.gsys, &mut self.charger);

                // Calculate objective function
                J1 = StdObj::run(&mut self.gsys.get_data());

                // Compare the objective functions
                self.cmp_obj_fnc(J0, J1, t);
            }
        }

        return None;
    }

    //---------------------------------------------------------------------------
    /// Compare objective functions and return the kept result.
    ///
    /// # Input
    /// * j0: Previous objective function
    /// * j1: New Objective function
    /// * t : Temperature
    ///
    /// # Output
    /// * true if the data has been changed to `j_1`, false otherwise
    ///
    fn cmp_obj_fnc(self: &mut SA<'a>, j0: f64, j1: f64, t: f32) -> bool {
        let delta_e: f64 = j0 - j1;

        // If the new data has a smaller objective function value than the old
        if delta_e < 0.0 {
            // Indicate that new data, `j_1`, is replacing old data, `j_0`
            return true;
        // Otherwise, the new data, `j_1`, has a larger objective function
        } else {
            // Calculate the coefficient
            let coef: f64 = delta_e / (t as f64);

            // Calculate `e^coef`
            let e: f64 = coef.exp();

            // Generate a number between 1-100
            let prob = thread_rng().gen_range(0.0..=1.0);

            // Return whether to keep the new data.
            // - if e <= prob: keep new data
            // - if e > prob: keep old data
            return e <= prob;
        }
    }
}
