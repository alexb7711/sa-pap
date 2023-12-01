#![allow(non_snake_case)]

//==============================================================================
// Declare submodules
pub mod charger; // Parameters and decision variables
pub mod data; // Parameters and decision variables
pub mod generators; // Pool of all the SA generators
pub mod route; // Pool of all the route generators
pub mod temp_func; // Temperature functions

//==============================================================================
// Import standard library
use indicatif::ProgressIterator;
use rand::{thread_rng, Rng};

//==============================================================================
// Import modules
use self::temp_func::TempFunc;
use crate::lp::objectives::std_obj::StdObj;
use crate::lp::objectives::Objective;
use crate::sa::charger::Charger;
use crate::sa::data::Data;
use crate::sa::generators::Generator;
use crate::sa::route::Route;

//==============================================================================
/// Results from simulated annealing
/// TODO: Remove `#[allow(dead_code)]
//
#[derive(Default)]
#[allow(dead_code)]
pub struct Results {
    data: Box<Data>,
}

//==============================================================================
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

//==============================================================================
/// Implementation of SA
//
impl<'a> SA<'a> {
    //==========================================================================
    // PUBLIC
    //==========================================================================

    //--------------------------------------------------------------------------
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
        mut gsys: Box<dyn Route>,
        gtweak: Box<dyn Generator>,
        tf: &'a mut Box<TempFunc>,
    ) -> SA<'a> {
        // Generate new solution
        gsys.run();

        // Extract BEB count
        let A = Some(gsys.get_data().param.A);

        // Create SA object
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

    //--------------------------------------------------------------------------
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

        // Extract solution sets
        let sol_orig = *self.gsys.get_data();
        let mut sol_best = *self.gsys.get_data();
        let mut sol_current = *self.gsys.get_data();
        let mut sol_new;

        // Set local search iteration count
        let k = 10;

        // Initialize objective function variables
        let mut J0: f64;
        let mut J1: f64 = 99999999.0; // Initialize to some obscene value

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Execute SA

        // Generate new solution
        self.gsol.run(&mut self.gsys, &mut self.charger);

        // Extract new data set
        sol_new = *self.gsys.get_data();

        // Calculate objective function
        J0 = StdObj::run(&mut sol_current);

        // Compare the objective functions
        if self.cmp_obj_fnc(J0, J1, self.tf.get_temp(None).unwrap()) {
            self.update_current_values(&mut sol_current, &mut sol_new);
        }

        // Make the world a little more pretty
        println!("Executing SA:");

        // While the temperature function is cooling down
        for t in self.tf.get_temp_vec().unwrap().into_iter().progress() {
            // Generate new solution
            self.gsol.run(&mut self.gsys, &mut self.charger);

            // Extract new data set
            sol_new = *self.gsys.get_data();

            // Calculate objective function
            J1 = StdObj::run(&mut sol_new);

            // Update data sets
            self.update_data_sets(
                &mut sol_best,
                &mut sol_current,
                &mut sol_new,
                &mut J0,
                &mut J1,
                t,
            );

            // Iterate though local search
            for _ in 0..k {
                // Tweak the schedule
                self.gtweak.run(&mut self.gsys, &mut self.charger);

                // Extract new data set
                sol_new = *self.gsys.get_data();

                // Calculate objective function
                J1 = StdObj::run(&mut sol_new);

                // Update data sets
                self.update_data_sets(
                    &mut sol_best,
                    &mut sol_current,
                    &mut sol_new,
                    &mut J0,
                    &mut J1,
                    t,
                );
            }
        }

        // Check if the data has been changed
        let result: Option<Results>;
        if sol_orig.dec != sol_best.dec {
            // Create result object
            result = Some(Results {
                data: Box::new(sol_best.clone()),
            });
        } else {
            result = None;
        }

        return result;
    }

    //==========================================================================
    // PRIVATE
    //==========================================================================

    //--------------------------------------------------------------------------
    /// Update current data sets. Three data sets are provided: best, current,
    /// and new. The logic goes as follows:
    ///
    /// - Check whether to update current data set with new data set with either:
    ///     - Probability 1 if new data set has a lesser objective score
    ///     - Probability $e^{-\frac{J_{old} - J_{new}}{T}}$ if new data has a
    ///       greater objective score
    /// - Check whether to update current data set with best data set
    ///
    /// # Input
    /// * sol_best: The best known solution data set
    /// * sol_current: The current solution data set
    /// * sol_new: The new solution data set
    /// * j0: Previous objective function
    /// * j1: New Objective function
    /// * t : Temperature
    ///
    /// # Output
    /// * NONE
    ///
    fn update_data_sets(
        self: &mut SA<'a>,
        sol_best: &mut Data,
        sol_current: &mut Data,
        sol_new: &mut Data,
        j0: &mut f64,
        j1: &mut f64,
        t: f32,
    ) {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Compare current data with new data

        // Compare the objective functions
        if self.cmp_obj_fnc(*j0, *j1, t) {
            // Update the current solution with the new data set
            self.update_current_values(sol_current, sol_new);

            // Update J0
            *j0 = *j1;
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Compare current data with best data
        let jbest = StdObj::run(sol_best);

        // If the current solution is strictly better than the current best
        println!("{} - {} = {}", jbest, *j0, jbest - *j0);
        if jbest == 0.0 || jbest - *j0 > 0.0 {
            println!("Update best..");
            // Update the best to match the current data set
            self.update_current_values(sol_best, sol_current);
        }
    }

    //--------------------------------------------------------------------------
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
        if delta_e > 0.0 {
            // Indicate that new data, `j_1`, is replacing old data, `j_0`
            return true;
        // Otherwise, the new data, `j_1`, has a larger objective function
        } else {
            // Calculate the coefficient
            let coef: f64 = delta_e / (30.0 * t as f64);

            // Calculate `e^coef`
            let e: f64 = coef.exp();

            // Generate a number between 0 and 1
            let prob = thread_rng().gen_range(0.0..=1.0);

            // Return whether to keep the new data.
            // - if e <= prob: keep new data
            // - if e > prob: keep old data
            return e <= prob;
        }
    }

    //--------------------------------------------------------------------------
    /// Update old data with new
    ///
    /// # Input
    /// * sol_current: Current solution data set
    /// * sol_new: New solution data set
    ///
    /// # Output
    /// * NONE
    ///
    fn update_current_values(self: &mut SA<'a>, sol_current: &mut Data, sol_new: &mut Data) {
        *sol_current = sol_new.clone();
    }
}
