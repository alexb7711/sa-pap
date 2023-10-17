//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::generators::primitives::new_visit::*;
use crate::sa::generators::Generator;
use crate::sa::route::Route;

//===============================================================================
/// Structure defining the information to create a charge schedule
pub struct GenNewVisits {}

//===============================================================================
/// Implementation of `GenNewVisits`
//
impl GenNewVisits {
    //---------------------------------------------------------------------------
    /// Initialize the `GenNewVisits` object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `GenNewVisits`: Simulated annealing structure
    ///
    pub fn new() -> GenNewVisits {
        return GenNewVisits {};
    }
}

//===============================================================================
/// Implementation of `Generator` for `GenNewVisits`
//
impl Generator for GenNewVisits {
    #[allow(non_snake_case)]
    //---------------------------------------------------------------------------
    /// The `run` function for `GenNewVisits` creates a schedule by running the
    /// `new_visit` generator for each visit. It is important to note that not
    /// every visit may be assigned due to the random nature of the assignments.
    ///
    /// # Input
    /// * r: Route object
    /// * c: Charger object
    /// * d: Data object
    ///
    /// # Output
    /// * Updated charger object
    ///
    fn run(self: &mut GenNewVisits, r: &mut dyn Route, c: &mut Charger) -> bool {
        // Get information about the route
        let mut route = r.get_route_events();

        // Get information about the route
        let data = r.get_data();

        // For each visit
        for i in route.iter_mut() {
            // Set the start/stop charge times
            let ae = &(i.arrival_time, i.departure_time);

            // Check if the bus can be assigned, assign the bus wait queue
            if new_visit::run(c, i.id as usize, ae) {
                // Update route event
                i.attach_time = ae.0;
                i.detatch_time = ae.1;
                i.queue = i.id;
            }
        }

        // Update route and charger
        r.set_data(data);
        r.set_route_events(route);

        return true;
    }
}
