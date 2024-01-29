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
use gnuplot::Figure;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use yaml_rust::Yaml;

//==============================================================================
// Import modules
use self::temp_func::TempFunc;
use crate::lp::constraints::constraints;
use crate::lp::objectives::std_obj::StdObj;
use crate::lp::objectives::Objective;
use crate::plotter::schedule_plot::SchedulePlot;
use crate::plotter::Plotter;
use crate::sa::charger::Charger;
use crate::sa::data::Data;
use crate::sa::generators::Generator;
use crate::sa::route::Route;
use crate::util::fileio::yaml_loader;

//==============================================================================
/// Results from simulated annealing
/// TODO: Remove `#[allow(dead_code)]
//
#[allow(dead_code)]
#[derive(Clone)]
pub struct Results {
    pub score: f64,
    pub data: Box<Data>,
    pub charger: Box<Charger>,
}

//==============================================================================
/// Structure for simulated annealing
//
#[allow(dead_code)]
pub struct SA<'a> {
    gsol: Box<dyn Generator>,   // Solution generator
    gsys: Box<dyn Route>,       // Route generator
    gtweak: Box<dyn Generator>, // Solution modifier
    charger: Box<Charger>,      // Charge schedule keeper
    tf: &'a mut Box<TempFunc>,  // Cooling Schedule
    config_path: &'a str,       // Path to simulation configuration file
    sol_found: bool,            // Indicates whether a solution was found
    pb: &'a ProgressBar,        // Progress Bar for this thread
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
    /// * `config_path` : String of relative path to configuration file
    /// * `gsol`        : Solution generator
    /// * `gsys`        : Route generator
    /// * `gtweak`      : Tweak schedule
    /// * `tf`          : The temperature function to use
    ///
    /// # Output
    /// * `Some(Results)`: Results of simulation (if there is any)
    ///
    pub fn new(
        config_path: &'a str,
        gsol: Box<dyn Generator>,
        mut gsys: Box<dyn Route>,
        gtweak: Box<dyn Generator>,
        tf: &'a mut Box<TempFunc>,
        pb: &'a mut ProgressBar,
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
            tf,
            config_path,
            sol_found: false,
            pb,
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
    pub fn run(self: &mut SA<'a>, rtp: bool, _lff: bool) -> Option<Results> {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Initialize

        // Create real time figures
        let mut fg_slow = Figure::new();
        let mut fg_fast = Figure::new();

        // Create progress bar and set style
        self.pb
            .set_length(self.tf.get_temp_vec().unwrap().len() as u64);
        self.pb
            .set_style(ProgressStyle::with_template("{prefix}|{wide_bar} {pos}/{len}").unwrap());

        // Extract solution sets
        let sol_orig = *self.gsys.get_data();
        let mut sol_best = *self.gsys.get_data();
        let mut sol_current = *self.gsys.get_data();
        let mut sol_new;

        // Set local search iteration count
        let config: Yaml = yaml_loader::load_yaml(self.config_path);
        let k = config["time"]["K"].clone().into_i64().unwrap();

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

        // While the temperature function is cooling down
        for t in self.tf.get_temp_vec().unwrap() {
            // Set the prefix depending on whether a solution has been found or not
            if self.sol_found {
                self.pb.set_prefix(format!("✓"));
            } else {
                self.pb.set_prefix(format!("×"));
            }

            // Print solution found indicator
            self.pb.inc(1);

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

            // Plot schedule in real time
            SchedulePlot::real_time(
                rtp,
                &mut Box::new(sol_current.clone()),
                &mut fg_slow,
                &mut fg_fast,
            );
        }

        // Check if the data has been changed
        let result: Option<Results>;
        if sol_orig.dec != sol_best.dec {
            // Create result object
            result = Some(Results {
                score: StdObj::run(&mut sol_best.clone()),
                data: Box::new(sol_best.clone()),
                charger: self.charger.clone(),
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
        // Check if the data is valid

        // If the constraint check failed
        if !constraints::run(sol_new) {
            // Bail
            return;
        } else {
            // Indicate a solution was found
            self.sol_found = true;
        }

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
        if jbest == 0.0 || jbest - *j0 > 0.0 {
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
