//===============================================================================
// Declare submodules
use self::temp_func::TempFunc;
use crate::sa::generators::Generator;

//===============================================================================
// Import modules
pub mod generators;                                                          // Pool of all the generators
pub mod temp_func;                                                           // Temperature functions

//===============================================================================
/// Results from simulated annealing
//
#[derive(Default)]
pub struct Results
{}

//===============================================================================
/// Structure for simulated annealing
//
#[allow(dead_code)]
pub struct SA
{
    gsol   : Box <dyn Generator>,                                            // Solution generator
    gsys   : Box <dyn Generator>,                                            // System generator
    gtweak : Box <dyn Generator>,                                            // Solution perterber
    r      : Results,
    tf     : Box <TempFunc>,                                                 // Cooling Schedule
}

//===============================================================================
/// Implementation of SA
//
impl SA
{
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
    pub fn new(gsol   : Box<dyn Generator>,
               gsys   : Box<dyn Generator>,
               gtweak : Box<dyn Generator>,
               tf     : Box<TempFunc>) -> SA

    {
        let sa: SA = SA
        {
            gsol,
            gsys,
            gtweak,
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
    pub fn run(self: &mut SA) -> Option<Results>
    {
        return None;
    }

}
