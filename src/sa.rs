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
    tf: &'a Box<TempFunc>,      // Cooling Schedule
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
        tf: &'a Box<TempFunc>,
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
    /// * NONE
    ///
    /// # Output
    /// * `Results`: Output of SA algorithm
    ///
    pub fn run(self: &mut SA<'a>) -> Option<Results> {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Initialize

        // Generate schedule
        self.gsys.run();

        // Create initial solution (`gen_new_visit` or `gen_wait_queue`)
        let _sol_init = self.gsol.run(&mut self.gsys, &mut self.charger);

        // Select temperature schedule (T) and initialize temperature (t_k)
        let _t = self.tf;
        let _tk: u32 = 1000;

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Execute SA
        return self.execute();
    }

    //---------------------------------------------------------------------------
    /// Execute the SA algorithm
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `Results`: Output of SA algorithm
    ///
    fn execute(self: &SA<'a>) -> Option<Results> {
        return None;
    }
}
