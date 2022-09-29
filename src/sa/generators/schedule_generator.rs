//===============================================================================
// Import modules
use crate::sa::generators::Generator;

//===============================================================================
/// Structure defining the information to create a charge schedule
//
#[derive(Default)]
pub struct ScheduleGenerator
{}

//===============================================================================
/// Implementation of `ScheduleGenerator`
//
impl ScheduleGenerator
{
    //---------------------------------------------------------------------------
    /// Initialize the `ScheduleGenerator` object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `ScheduleGenerator`: Simulated annealing structure
    ///
    pub fn new() -> ScheduleGenerator
    {
        return ScheduleGenerator {};
    }
}

//===============================================================================
/// Implementation of `Generator` for `ScheduleGenerator`
//
impl Generator for ScheduleGenerator
{
    fn run(self: &mut ScheduleGenerator)
    {}
}
