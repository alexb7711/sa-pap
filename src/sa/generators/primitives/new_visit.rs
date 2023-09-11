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
        let mut ud: &(f32, f32) = &ae;

        // Index of the charging queue
        let mut q: usize;

        loop {
            // Select a random charger
            q = rand::thread_rng().gen_range(0..q_cnt);

            // Find a free time slice
            let ts: (f32, f32) = find_ts(ch, q);

            // Check if the arrival/departure fits in the time slice
            let (fits, ud) = ch.find_free_time(&ae, &ts);

            if true {
                break;
            }
        }

        return ch.assign(q, ud, b);
    }

    //--------------------------------------------------------------------------
    /// The `find_ts` function returns a random free time slice given the charger.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Charger queue index
    ///
    /// # Output
    /// * ts: Time slice of selected free time
    ///
    fn find_ts(ch: &mut Charger, q: usize) -> (f32, f32) {
        // Get the number of open time slots
        let ft_cnt: usize = ch.free_time[q].len();

        // Select a random time slot
        let ft_idx = rand::thread_rng().gen_range(0..ft_cnt);

        // Reserve the time and return the success
        return ch.free_time[q][ft_idx];
    }
}
