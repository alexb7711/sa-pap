//==============================================================================
/// The `new_charger` primitive is used to assign a bus to an available charger.
//
mod new_visit {
    // Import standard library
    use rand::Rng;

    // Import modules
    use crate::sa::charger::Charger;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_charger` module. Given the set of routes and a bus ID and visit, the run
    /// function shall assign a bus to an available charger and return the new schedule. Return false if assignment
    // failed and true if successful.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q_cnt: Number of queues available
    /// * ts: Time slice to assign to queue
    /// * b: Bus id
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    #[allow(dead_code)]
    pub fn run(ch: &mut Charger, q_cnt: usize, b: usize) -> bool {
        // Select a random charger
        let q: usize = rand::thread_rng().gen_range(0..(q_cnt - 1));

        // Find a free time slice
        let ts: (f32, f32) = find_free_time(ch, q);

        return ch.assign(q, ts, b);
    }

    //--------------------------------------------------------------------------
    /// The `find_free_time` function returns a random free time given the charger.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Charger queue index
    ///
    /// # Output
    /// * ts: Time slice of selected free time
    ///
    fn find_free_time(ch: &mut Charger, q: usize) -> (f32, f32) {
        // Get the number of open time slots
        let ft_cnt: usize = ch.free_time[q].len();

        // Select a random time slot
        let ft_idx = rand::thread_rng().gen_range(0..ft_cnt);

        // Reserve the time and return the success
        return ch.free_time[q][ft_idx];
    }
}
