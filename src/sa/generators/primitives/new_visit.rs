//==============================================================================
/// The `new_charger` primitive is used to assign a bus to an available charger.
//
pub mod new_visit {
    // Import standard library
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    // Import modules
    use crate::sa::charger::Charger;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_charger` module. Given the set of routes and a bus ID and visit, the run
    /// function shall assign a bus to an available charger and return the new schedule. Return false if assignment
    /// failed and true if successful.
    ///
    /// # Input
    /// * ch: Charger object
    /// * b: Bus id
    /// * ae: Arrive/Exit times of the bus
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(ch: &mut Charger, b: usize, ae: &(f32, f32)) -> bool {
        // Extract the number of chargers
        let q_cnt: usize = ch.schedule.len();

        // Create a list of queue indices and shuffle them
        let mut queues: Vec<usize> = (0..q_cnt).collect();
        queues.shuffle(&mut thread_rng());

        // Iterate the shuffled queue indices
        for q in queues.into_iter() {
            // Create a list of time slices and shuffle them
            let mut time_slice = ch.free_time[q].clone();
            time_slice.shuffle(&mut thread_rng());

            // Iterate through the shuffled time slices
            for ts in time_slice.iter() {
                // Check if the arrival/departure fits in the time slice
                let (fits, ud) = ch.find_free_time(ae, ts);

                // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
                // times
                if fits {
                    println!("Here");
                    return ch.assign(q, ud, b);
                }
            }
        }

        return false;
    }
}
