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
pub struct SA {
    gsol: Box<dyn Generator>,   // Solution generator
    gsys: Box<dyn Route>,       // Route generator
    gtweak: Box<dyn Generator>, // Solution perterber
    charger: Box<Charger>,      // Charge schedule keeper
    r: Results,                 // Results
    tf: Box<TempFunc>,          // Cooling Schedule
}

//===============================================================================
/// Implementation of SA
//
impl SA {
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
        gsol: Box<dyn Generator>,
        gsys: Box<dyn Route>,
        gtweak: Box<dyn Generator>,
        tf: Box<TempFunc>,
    ) -> SA {
        let sa: SA = SA {
            gsol,
            gsys,
            gtweak,
            charger: Box::new(Charger::new(None)),
            r: Default::default(),
            tf,
        };

        return sa;
    }

    //---------------------------------------------------------------------------
    /// Run the SA algorithm
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `Results`: Output of SA algorithm
    ///
    pub fn run(self: &mut SA) -> Option<Results> {
        return None;
    }
}
