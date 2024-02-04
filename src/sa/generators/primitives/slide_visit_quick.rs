//==============================================================================
/// The `slide_visit` primitive is used to assign a bus to an available charger.
//
pub mod slide_visit_quick {
    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::generators::primitives::purge::*;

    //--------------------------------------------------------------------------
    /// The run function executes the `slide_visit` module. This modules attempts
    /// to allocate a different charge time in the same queue.
    ///
    /// # Input
    /// * d: MILP data object
    /// * ch: Charger object
    /// * b: Bus id
    /// * q: Queue index
    /// * ae: Arrive/departure times for the BEB
    /// * ud: Start/stop charge times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(
        d: &mut Data,
        i: usize,
        ch: &mut Charger,
        b: usize,
        q: usize,
        ae: &(f32, f32),
        ud: &(f32, f32),
    ) -> bool {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Remove the visit, return false if unsuccessful
        if !purge::run(d, i, ch, q, ud) {
            return false;
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Attempt to assign the visit

        // Get the time slice of interest
        let ts = ch.get_ts(&q, ae);

        // Check if the arrival/departure fits in the time slice
        let (fits, ud_new) = ch.find_free_time(ae, &ts);

        // If the selected time slice arrival/departure fits in the time slice, assign the start/stop charge
        // times
        if fits && ch.assign(q, ud_new, b) {
            // Update initial/final charge times
            d.dec.u[i] = ud_new.0;
            d.dec.d[i] = ud_new.1;
            d.dec.s[i] = ud.1 - ud.0;

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
