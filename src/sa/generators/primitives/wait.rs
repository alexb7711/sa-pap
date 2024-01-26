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

        // Extract the number of chargers
        let q: usize = b;

        // Return true/false if assignment succeeded/failed
        if ch.assign(q, *ae, b) {
            // Update route data
            if d.param.N > 0 {
                // Update queue to wait queue
                d.dec.v[i] = d.param.Gam[i] as usize;
                d.dec.w[i][q] = false;
                d.dec.w[i][d.dec.v[i]] = true;

                // Update attach
                d.dec.u[i] = ae.0;

                // Update detach time
                d.dec.c[i] = ae.1;
            }

            return true;
        } else {
            return false;
        }
    }
}
