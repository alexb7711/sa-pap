//==============================================================================
/// The `remove` primitive is used to assign a bus to an available charger.
//
pub mod remove {

    // Import modules
    use crate::sa::charger::Charger;

    //--------------------------------------------------------------------------
    /// The run function executes the `remove` module. Given the set queue and start/stop charging times, remove that
    /// scheduled time from the charger queue.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Charger queue index
    /// * ud: Start/stop charging times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(ch: &mut Charger, q: usize, ud: &(f32, f32)) -> bool {
        return ch.remove(q, (*ud).clone());
    }
}
