//==============================================================================
/// The `wait` primitive is used to move the bus to its waiting queue
//
pub mod wait {

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::generators::primitives::purge::*;

    //--------------------------------------------------------------------------
    /// The run function executes the `wait` module. This module moves a queued
    /// bus to its waiting queue
    ///
    /// # Input
    /// * d: MILP data object
    /// * i: Index of current visit
    /// * ch: Charger object
    /// * q: Charger queue index
    /// * b: Bus id
    /// * ae: Arrival/exit times
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
        // Remove the visit, return false if unsuccessful
        if !purge::run(d, i, ch, q, ud) {
            return false;
        }

        // Return true/false if assignment succeeded/failed
        if ch.assign(b, *ae, b) {
            // Update queue to wait queue
            d.dec.v[i] = b;
            d.dec.w[i].fill(false);
            d.dec.w[i][b] = true;

            // Update attach/detach time
            d.dec.u[i] = ae.0;
            d.dec.d[i] = ae.1;
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
