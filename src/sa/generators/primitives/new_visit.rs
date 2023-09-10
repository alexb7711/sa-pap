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
        // Create random object
        let mut rand = rand::thread_rng();

        // Select a random charger
        let q: usize = rand.gen_range(0..(q_cnt - 1));

        let ts: (f32, f32) = (0.1, 0.2);

        return ch.assign(q, ts, b);
    }
}
