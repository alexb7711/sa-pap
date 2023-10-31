//===============================================================================
// Declare submodules
use self::temp_func::TempFunc;
use crate::sa::charger::Charger;
use crate::sa::generators::Generator;
use crate::sa::route::Route;

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
        let sa: SA = SA {
            gsol,
            gsys,
            gtweak,
            charger: Box::new(Charger::new(config_path, true, None)),
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
    pub fn run(self: &mut SA<'a>, lff: bool) -> Option<Results> {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Initialize

        // Generate new solution
        self.gsys.run();

        // Set local search iteration count
        let k = 1000;

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Execute SA

        // While the temperature function is cooling down
        while let Some(_t) = self.tf.step() {
            // Generate new solution
            self.gsys.run();

            // Calculate objective function

            // Compare the objective functions
            self.cmp_obj_fnc();

            // Iterate though local search
            for _ in 0..k {
                // Tweak the schedule
                self.gtweak.run(&mut self.gsys, &mut self.charger);

                // Calculate objective function

                // Compare the objective functions
                self.cmp_obj_fnc();
            }
        }

        return None;
    }

    //---------------------------------------------------------------------------
    /// Compare objective functions and return the kept result.
    ///
    /// # Input
    /// * j1: Previous objective function
    /// * j2: New Objective function
    /// * t : Temperature
    ///
    /// # Output
    /// * `Results`: Output of SA algorithm
    ///
    fn cmp_obj_fnc(self: &mut SA<'a>) -> bool {
        return false;
    }
}
