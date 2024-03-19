//==============================================================================
/// The `new_charger` primitive is used to remove then add a bus back in.
//
pub mod new_charger {

    // Import standard lib
    use rand::distributions::{Distribution, WeightedIndex};
    use rand::prelude::*;

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::generators::primitives::purge::*;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_charger` module. This module
    /// purges the visit from the schedule and places that same BEB schedule
    /// on a random queue.
    ///
    /// # Input
    /// * d: MILP data object
    /// * i: Visit index
    /// * ch: Charger object
    /// * q: Charger queue index
    /// * b: Bus id
    /// * ud: Start/stop charging times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(
        d: &mut Data,
        i: usize,
        ch: &mut Charger,
        q: usize,
        b: usize,
        ud: &(f32, f32),
    ) -> bool {
        // Remove the visit, return false if unsuccessful
        if !purge::run(d, i, ch, q, ud) {
            return false;
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Create random list of charger indices

        // Extract the number of chargers
        let q_cnt: usize = ch.schedule.len();

        // Determine the charger offset from waiting queues
        let offset: usize; // Offset for slow or fast chargers
        let w = [3, 1];
        let dist = WeightedIndex::new(&w).unwrap();
        let selection_vals = [0, 1];
        let mut rng = thread_rng();
        let charge_type: usize = selection_vals[dist.sample(&mut rng)];

        // If the charger selected is a slow charger
        if charge_type == 0 {
            // Set the offset to ignore waiting queues
            offset = ch.charger_count.0;
        } else {
            // Set the offset to ignore waiting and slow queues
            offset = ch.charger_count.0 + ch.charger_count.1;
        }

        // Create a vector with the bus wait queue and all the charger queues
        let queues: Vec<usize> = (offset..q_cnt).collect();

        // Iterate the shuffled queue indices
        for q_new in queues.into_iter() {
            // Retrieve the time slice of interest if it exists
            let ts = ch.get_ts(&q_new, &ud);

            // Check if the arrival/departure fits in the time slice
            // Note that this line is what differentiates this function from `new_visit` by applying the same
            // start/stop charge time as before, just on a new charger.
            let (fits, _) = ch.find_free_time(ud, &ts);

            // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
            // times
            if ts != (0.0, 0.0) && fits && ch.assign(q_new, *ud, b) {
                // Update route data
                // Update queue
                d.dec.v[i] = q_new;

                // Update vector representation
                d.dec.w[i].fill(false);
                d.dec.w[i][q_new] = true;

                return true;
            }
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Place the original visit back in the queue availability matrix
        if !ch.assign(q, *ud, b) {
            panic!("Lost a visit!");
        };

        return false;
    }
}
