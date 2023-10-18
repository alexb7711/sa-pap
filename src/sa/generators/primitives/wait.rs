//==============================================================================
/// The `wait` primitive is used to move the bus to its waiting queue
//
pub mod wait {

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::generators::primitives::purge::*;

    //--------------------------------------------------------------------------
    /// The run function executes the `wait` module. This module moves a queued
    /// bus to its waiting queue
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Charger queue index
    /// * b: Bus id
    /// * ud: Start/stop charging times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(ch: &mut Charger, q: usize, b: usize, ud: &(f32, f32)) -> bool {
        // Remove the visit, return false if unsuccessful
        if !purge::run(ch, q, ud) {
            return false;
        }

        // Extract the number of chargers
        let q: usize = b;

        // Return true/false if assignment succeeded/failed
        return ch.assign(q, *ud, b);
    }
}
