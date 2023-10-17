//==============================================================================
/// The `new_charger` primitive is used to remove then add a bus back in.
//
pub mod new_charger {

    // Import standard lib
    use crate::util::rand_utils;

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::generators::primitives::remove::*;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_charger` module. This module encapsulates a `remove` then `new_visit`.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Charger queue index
    /// * b: Bus id
    /// * ud: Start/stop charging times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(ch: &mut Charger, q: usize, b: usize, ud: &(f32, f32)) -> bool {
        // Remove the visit, return false if unsuccessful
        if !remove::run(ch, q, b, ud) {
            return false;
        }

        // Extract the number of chargers
        let q_cnt: usize = ch.schedule.len();

        // Create a list of queue indices and shuffle them
        let mut queues: Vec<usize> = (0..q_cnt).collect();
        queues = rand_utils::shuffle_vec(&queues);

        // Iterate the shuffled queue indices
        for q in queues.into_iter() {
            // Create a list of time slices and shuffle them
            let mut time_slice = ch.free_time[q].clone();
            time_slice = rand_utils::shuffle_vec(&time_slice);

            // Iterate through the shuffled time slices
            for ts in time_slice.iter() {
                // Check if the arrival/departure fits in the time slice
                // Note that this line is what differentiates this function from `new_visit` by applying the same
                // start/stop charge time as before, just on a new charger.
                let (fits, _) = ch.find_free_time(ud, ts);

                // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
                // times
                if fits {
                    return ch.assign(q, *ud, b);
                }
            }
        }

        return false;
    }
}
