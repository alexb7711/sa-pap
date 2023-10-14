//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::route::Route;
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
    #[allow(non_snake_case)]

    //---------------------------------------------------------------------------
    /// The `run` function for `ScheduleGenerator` creates a schedule by first
    /// assigning all the buses to fast chargers. If all the fast chargers are
    /// are utilized, then assign buses to slow chargers.
    ///
    /// # Input
    /// * r: Route object
    /// * c: Charger object
    /// * d: Data object
    ///
    /// # Output
    /// * Updated charger object
    ///
    fn run(self: &mut ScheduleGenerator, r: &mut dyn Route, c: &mut Charger) {
        // // Calculate offset to fast charger index
        // let offset: usize = c.charger_count.0 + c.charger_count.1 - 1;
        // 
        // // Get solver data
        // let route_data = r.get_data();
        // let route_data = route_data.borrow();
        // 
        // // Get information about the route
        // let route = r.get_route_data();
        // let mut route = route.borrow_mut();
        // 
        // // Determine the amount of BEBs
        // let A: usize = route_data.param.A;
        // 
        // // For each bus
        // for b in 0..A {
        // // For each visit
        //     for i in route.iter_mut() {
        //         // If the bus id matches `b`
        //         if i.id == b as u16 {
        //             // Determine the index
        //             let q = b+offset;
        //             let ad = (i.arrival_time, i.departure_time);
        // 
        //             // Check if the bus can be assigned
        //             if c.assign(q, ad, b) {
        //                 i.attach_time = ad.0;
        //                 i.detatch_time = ad.1;
        //             }
        //         }
        //         // Assign bus to fast charger
        //     }
        // }
    }
}
