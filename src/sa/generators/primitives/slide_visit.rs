//==============================================================================
/// The `slide_visit` primitive is used to assign a bus to an available charger.
//
pub mod slide_visit {

    // Standard lib modules
    use crate::util::rand_utils;

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::generators::primitives::remove::*;

    //--------------------------------------------------------------------------
    /// The run function executes the `slide_visit` module. This modules attempts to allocate a different charge time in
    /// the same queue.
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
    pub fn run(ch: &mut Charger, b: usize, q: usize, ae: &(f32, f32), ud: &(f32, f32)) -> bool {
        // Remove the visit, return false if unsuccessful
        if !remove::run(ch, q, ud) {
            return false;
        }

        // Create a list of time slices and shuffle them
        let mut time_slice = ch.free_time[q].clone();
        time_slice = rand_utils::shuffle_vec(&time_slice);

        // Iterate through the shuffled time slices
        for ts in time_slice.iter() {
            // Check if the arrival/departure fits in the time slice
            let (fits, ud) = ch.find_free_time(ae, ts);

            // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
            // times
            if fits {
                return ch.assign(q, ud, b);
            }
        }

        return false;
    }
}
