//==============================================================================
/// The `slide_visit` primitive is used to assign a bus to an available charger.
//
pub mod slide_visit {

    // Standard lib modules
    use crate::util::rand_utils;

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::generators::primitives::{self, purge::*};
    use crate::sa::route::route_event::RouteEvent;

    //--------------------------------------------------------------------------
    /// The run function executes the `slide_visit` module. This modules attempts
    /// to allocate a different charge time in the same queue.
    ///
    /// # Input
    /// * ch: Charger object
    /// * b: Bus id
    /// * q: Queue index
    /// * ae: Arrive/Exit times of the bus
    /// * ud: Start/stop charge times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(
        r: &mut Vec<RouteEvent>,
        i: usize,
        ch: &mut Charger,
        b: usize,
        q: usize,
        ae: &(f32, f32),
        ud: &(f32, f32),
    ) -> bool {
        // Remove the visit, return false if unsuccessful
        if !purge::run(r, i, ch, q, ud) {
            return false;
        }

        // Create a list of time slices and shuffle them
        let mut time_slice = ch.free_time[q].clone();
        time_slice = rand_utils::shuffle_vec(&time_slice);

        // Filter out very small windows
        time_slice = time_slice
            .into_iter()
            .filter(|x| x.1 - x.0 >= primitives::EPSILON)
            .collect();

        // Iterate through the shuffled time slices
        for ts in time_slice.iter() {
            // Check if the arrival/departure fits in the time slice
            let (fits, ud) = ch.find_free_time(ae, ts);

            // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
            // times
            if fits && ch.assign(q, ud, b) {
                // Update route data
                if r.len() > 0 {
                    r[i].attach_time = ud.0; // Update attach time
                    r[i].detach_time = ud.1; // Update detach time
                }

                return true;
            }
        }

        return false;
    }
}
