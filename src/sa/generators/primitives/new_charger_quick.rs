//==============================================================================
/// The `new_charger_quick` primitive is used to remove then add a bus back in.
//
pub mod new_charger_quick {

    // Import standard lib
    use crate::util::rand_utils;

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::generators::primitives::purge::*;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_charger_quick` module. This module
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
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Remove the visit, return false if unsuccessful
        if !purge::run(d, i, ch, q, ud) {
            return false;
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Random selection

        // Select a random charger queue
        let q_new = rand_utils::rand_range(0, ch.schedule.len() - 1);

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Attempt to assign the visit

        // Check if the arrival/departure fits in the time slice
        // Note that this line is what differentiates this function from `new_visit` by applying the same
        // start/stop charge time as before, just on a new charger.
        let (fits, _) = ch.find_free_time(ud, ud);

        // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
        // times
        if fits && ch.assign(q_new, *ud, b) {
            // Update route data
            if d.dec.w.len() > q_new {
                // Update queue
                d.dec.v[i] = q_new;

                // Update vector representation
                d.dec.w[i].fill(false);
                d.dec.w[i][q_new] = true;
            }
            return true;
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Place the original visit back in the queue availability matrix

        if !ch.assign(q, *ud, b) {
            panic!("Lost a visit!");
        };

        return false;
    }
}
