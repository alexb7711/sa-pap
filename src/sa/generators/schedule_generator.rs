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
        // Get information about the route
        let mut route = r.get_route_events();

        // Get information about the route
        let data = r.get_data();

        // Determine the amount of BEBs
        let A: usize = data.param.A;

        // For each bus
        for b in 0..A {
            // For each visit
            for i in route.iter_mut() {
                // If the bus id matches `b`
                if i.id == b as u16 {

                    // Set the start/stop charge times
                    let ad = (i.arrival_time, i.departure_time);

                    // Check if the bus can be assigned, assign the bus wait queue
                    if c.assign(b.clone(), ad.clone(), b.clone()) {
                        // Update route event
                        i.attach_time = ad.0.clone();
                        i.detatch_time = ad.1.clone();
                        i.queue = b.clone() as u16;
                    }
                }
            }
        }

        // Update route and charger
        r.set_data(data);
        r.set_route_events(route);
    }
}
