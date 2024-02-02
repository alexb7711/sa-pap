//==============================================================================
/// The `new_visit` primitive is used to assign a bus to an available charger.
//
pub mod new_visit_quick {
    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::generators::primitives::purge::*;
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
    /// * q: Charger queue index
    /// * b: Bus id
    /// * ae: Arrive/Exit times of the bus
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
        ae: &(f32, f32),
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

        // Select random time slice availability
        let ts_idx = rand_utils::rand_range(0, ch.free_time[q_new].len() - 1);

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Attempt to assign the visit

        // Check if the arrival/departure fits in the time slice
        let (fits, ud_new) = ch.find_free_time(ae, &ch.free_time[q_new][ts_idx].clone());

        // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
        // times
        if fits && ch.assign(q_new, ud_new, b) {
            // Update queue
            d.dec.v[i] = q_new;

            // Update vector representation
            d.dec.w[i].fill(false);
            d.dec.w[i][q_new] = true;

            // Update initial/final charge times
            d.dec.u[i] = ud_new.0;
            d.dec.c[i] = ud_new.1;

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
