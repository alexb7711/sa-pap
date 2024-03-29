//==============================================================================
/// The `new_visit` primitive is used to assign a bus to an available charger.
//
pub mod new_visit {
    // Import standard lib
    use rand::distributions::{Distribution, WeightedIndex};
    use rand::prelude::*;

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::generators::primitives;
    use crate::util::rand_utils;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_visit` module. Given the set of
    /// routes and a bus ID and visit, the run function shall assign a bus to an
    /// available charger and return the new schedule. Return false if assignment
    /// failed and true if successful.
    ///
    /// # Input
    /// * d: MILP data object
    /// * i: Visit index
    /// * ch: Charger object
    /// * b: Bus id
    /// * ae: Arrival/exit times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(
        dat: &mut Data,
        i: usize,
        ch: &mut Charger,
        q: usize,
        b: usize,
        ae: &(f32, f32),
        ud: &(f32, f32),
    ) -> bool {
        // Extract the number of chargers
        let q_cnt: usize = ch.schedule.len();

        // Determine the charger offset from waiting queues
        let offset;
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
            // Create a list of time slices and shuffle them
            let mut time_slice = ch.free_time[q_new].clone();
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
                if fits && ch.assign(q_new, ud, b) {
                    // Update queue
                    dat.dec.v[i] = q_new;

                    // Update vector representation
                    dat.dec.w[i].fill(false);
                    dat.dec.w[i][q_new] = true;

                    // Update initial/final charge times
                    dat.dec.u[i] = ud.0;
                    dat.dec.d[i] = ud.1;
                    dat.dec.s[i] = ud.1 - ud.0;

                    // Indicate success
                    return true;
                }
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
