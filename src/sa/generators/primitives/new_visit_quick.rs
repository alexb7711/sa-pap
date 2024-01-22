//==============================================================================
/// The `new_visit` primitive is used to assign a bus to an available charger.
//
pub mod new_visit {
    // Import standard lib
    use crate::util::rand_utils;

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::generators::primitives;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_visit` module. Given the set of
    /// routes and a bus ID and visit, the run function shall assign a bus to an
    /// available charger and return the new schedule. Return false if assignment
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
    pub fn run(d: &mut Data, i: usize, ch: &mut Charger, b: usize, ae: &(f32, f32)) -> bool {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Remove the visit, return false if unsuccessful
        if !purge::run(d, i, ch, q, ud) {
            return false;
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Random selection

        // Select a random charger queue
        let q_new = rand_range(0, ch.charger_count);

        // Select random time slice availability
        let ts_idx = rand_range(0, ch.free_time[q_new].len());

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Attempt to assign the visit

        // Check if the arrival/departure fits in the time slice
        let (fits, ud_new) = ch.find_free_time(ae, ch.free_time[q_new][ts_idx]);

        // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
        // times
        if fits && ch.assign(q_new, ud_new, b) {
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
