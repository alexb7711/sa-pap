//==============================================================================
/// The `new_charger` primitive is used to assign a bus to an available charger.
//
pub mod new_visit {
    // Import standard library
    use rand::Rng;

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
    pub fn run(ch: &mut Charger, b: usize, ae: (f32, f32)) -> bool {
        // Extract the number of chargers
        let q_cnt: usize = ch.schedule.len();

        // Set the attach/detach time as the arrival/departure time
        let mut ud: (f32, f32);

        // Indicate that the desired schedule reservation does not fit
        let mut fits: bool;

        // Indicate that the desired schedule reservation does not fit
        let mut checked_indices: Vec<bool> = vec![false; b];

        // Index of the charging queue
        let mut q: usize;

        // Flag to indicate whether all the queues have been checked
        let mut exhausted: bool;

        loop {
            (exhausted, q) = get_rand_queue(q_cnt, &mut checked_indices);

            // If all the options have been exhausted, return that the assignment could not be made
            if exhausted {
                return false;
            }

            // Find a free time slice
            let ts: (f32, f32) = get_rand_ts(ch, q);

            // Check if the arrival/departure fits in the time slice
            (fits, ud) = ch.find_free_time(&ae, &ts);

            // If the arrival/exit fits in the time slice, exit
            if fits {
                break;
            }
        }

        return ch.assign(q, ud, b);
    }

    //--------------------------------------------------------------------------
    /// The `get_rand_ts` function returns a random free time slice given the charger.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Charger queue index
    ///
    /// # Output
    /// * ts: Time slice of selected free time
    ///
    fn get_rand_ts(ch: &mut Charger, q: usize) -> (f32, f32) {
        // Get the number of open time slots
        let ft_cnt: usize = ch.free_time[q].len();

        // Select a random time slot
        let ft_idx = rand::thread_rng().gen_range(0..ft_cnt);

        // Reserve the time and return the success
        return ch.free_time[q][ft_idx];
    }

    //--------------------------------------------------------------------------
    /// The `gen_rand_queue` function generates a new random number in the range of the queues available. If all the
    /// queues have been checked, return that all the routes have been exhausted
    ///
    /// # Input
    /// * cnt: Number of charger queues
    /// * tested_indices: Vector of indices that have been tested
    ///
    /// # Output
    /// * ts: Time slice of selected free time
    ///
    fn get_rand_queue(cnt: usize, tested_index: &mut Vec<bool>) -> (bool, usize) {
        // Flag to indicate that all the queues have been searched
        let mut exhausted = false;

        // Index of the queue found
        let mut q: usize;

        loop {
            // Generate random number
            q = rand::thread_rng().gen_range(0..cnt);

            // If the queue has not been tested
            if !tested_index[q] {
                // Update list of checked indices
                tested_index[q] = true;

                // Exit loop
                break;
            // If all the routes have been exhausted
            } else if tested_index.iter().all(|x| *x) {
                // Indicate all the routes have been checked with no success
                exhausted = true;

                // Exit the loop
                break;
            }
        }
        return (exhausted, q);
    }
}
