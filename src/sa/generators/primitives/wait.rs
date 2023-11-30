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

        // Extract the number of chargers
        let q: usize = b;

        // Return true/false if assignment succeeded/failed
        if ch.assign(q, *ud, b) {
            // Update route data
            if d.param.N > 0 {
                d.dec.v[i] = q; // Update queue to wait queue
                d.dec.u[i] = ud.0; // Update attach
                d.dec.c[i] = ud.1; // Update detach time
            }

            return true;
        } else {
            return false;
        }
    }
}
