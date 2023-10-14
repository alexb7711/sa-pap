//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::route::Route;
use crate::sa::data::Data;
use crate::sa::generators::Generator;

//===============================================================================
/// Structure defining the information to create a charge schedule
pub struct ScheduleGenerator {}

//===============================================================================
/// Implementation of `ScheduleGenerator`
//
impl ScheduleGenerator {
    //---------------------------------------------------------------------------
    /// Initialize the `ScheduleGenerator` object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `ScheduleGenerator`: Simulated annealing structure
    ///
    pub fn new() -> ScheduleGenerator {
        return ScheduleGenerator {};
    }
}

//===============================================================================
/// Implementation of `Generator` for `ScheduleGenerator`
//
impl Generator for ScheduleGenerator {

    //---------------------------------------------------------------------------
    /// The `run` function for `ScheduleGenerator` creates a schedule by first
    /// assigning all the buses to fast chargers. If all the fast chargers are
    /// are utilized, then assign buses to slow chargers.
    ///
    /// # Input
    /// * c: Charger object
    /// * d: Data object
    ///
    /// # Output
    /// * Updated charger object
    ///
    fn run(self: &mut ScheduleGenerator, _s: &mut dyn Route, _c: &mut Charger, _d: &mut Data) {
        
    }
}
