//==============================================================================
/// The `wait` primitive is used to move the bus to its waiting queue
//
pub mod wait {

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::generators::primitives::purge::*;
    use crate::sa::route::route_event::RouteEvent;

    //--------------------------------------------------------------------------
    /// The run function executes the `wait` module. This module moves a queued
    /// bus to its waiting queue
    ///
    /// # Input
    /// * r: Vector of `RouteEvents`
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
        r: &mut Vec<RouteEvent>,
        i: usize,
        ch: &mut Charger,
        q: usize,
        b: usize,
        ud: &(f32, f32),
    ) -> bool {
        // Remove the visit, return false if unsuccessful
        if !purge::run(r, i, ch, q, ud) {
            return false;
        }

        // Extract the number of chargers
        let q: usize = b;

        // Return true/false if assignment succeeded/failed
        if ch.assign(q, *ud, b) {
            // Update route data
            if r.len() > 0 {
                r[i].queue = q as u16; // Update queue to wait queue
                r[i].attach_time = ud.0; // Update attach
                r[i].detach_time = ud.1; // Update detach time
            }

            return true;
        } else {
            return false;
        }
    }
}
