//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::generators::Generator;
use crate::sa::route::Route;

//===============================================================================
/// Structure defining the information to create a charge schedule
pub struct GenWaitQueue {}

//===============================================================================
/// Implementation of `GenWaitQueue`
//
impl GenWaitQueue {
    //---------------------------------------------------------------------------
    /// Initialize the `GenWaitQueue` object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `GenWaitQueue`: Simulated annealing structure
    ///
    pub fn new() -> GenWaitQueue {
        return GenWaitQueue {};
    }
}

//===============================================================================
/// Implementation of `Generator` for `GenWaitQueue`
//
impl Generator for GenWaitQueue {
    #[allow(non_snake_case)]
    //---------------------------------------------------------------------------
    /// The `run` function for `GenWaitQueue` creates a schedule by first
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
    fn run(self: &mut GenWaitQueue, r: &mut Box<dyn Route>, c: &mut Charger) -> bool {
        // Get information about the route
        let mut route = r.get_route_events().clone();

        // Get information about the route
        let mut data = r.get_data();

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
                        i.detach_time = ad.1.clone();
                        i.queue = b.clone() as u16;

                        // Update MILP data
                        data.dec.u[i.visit] = ad.0.clone();
                        data.dec.c[i.visit] = ad.1.clone();
                        data.dec.v[i.visit] = b;
                        data.dec.w[i.visit][b] = true;
                    }
                }
            }
        }

        // Update route and charger
        r.set_route_events(Box::new(&mut route));
        r.set_data(data);

        return true;
    }
}
