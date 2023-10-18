//==============================================================================
/// The `remove` primitive is used to assign a bus to an available charger.
//
pub mod remove {

    // Import modules
    use crate::sa::charger::Charger;

    //--------------------------------------------------------------------------
    /// The run function executes the `remove` module. Given the set queue and
    /// start/stop charging times, remove that scheduled time from the charger
    /// queue and place it in its waiting queue.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Charger queue index
    /// * ud: Start/stop charging times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(ch: &mut Charger, q: usize, id: usize, ud: &(f32, f32)) -> bool {
        // Data
        let mut removed = false;

        // If the bus was successfully removed
        if ch.remove(q, (*ud).clone()) {
            // And if the bus was successfully added to its waiting queue
            if ch.assign(q, (*ud).clone(), id) {
                // Indicate that the bus was removed successfully
                removed = true;
            }
        }

        return removed;
    }
}
