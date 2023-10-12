//===============================================================================
// Import modules
use crate::sa::charger::Charger;
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
    fn run(self: &mut ScheduleGenerator, c: &mut Charger, d: &mut Data) {}
}
