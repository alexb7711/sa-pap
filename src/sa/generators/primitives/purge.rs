//==============================================================================
/// The `purge` primitive is used to assign a bus to an available charger.
//
pub mod purge {

    // Import modules
    use crate::sa::charger::Charger;

    //--------------------------------------------------------------------------
    /// The run function executes the `purge` module. Given the set queue and
    /// start/stop charging times, purge that scheduled time from the charger queue.
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
        return ch.remove(q, *ud);
    }
}
