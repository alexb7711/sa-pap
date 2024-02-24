//==============================================================================
/// The `purge` primitive is used to assign a bus to an available charger.
//
pub mod purge {

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;

    //--------------------------------------------------------------------------
    /// The run function executes the `purge` module. Given the set queue and
    /// start/stop charging times, purge that scheduled time from the charger queue.
    ///
    /// # Input
    /// * d: MILP data object
    /// * i: Visit index
    /// * ch: Charger object
    /// * q: Charger queue index
    /// * ud: Start/stop charging times
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    pub fn run(_: &mut Data, _: usize, ch: &mut Charger, q: usize, ud: &(f32, f32)) -> bool {
        if ch.remove(q, *ud) {
            return true;
        }

        return false;
    }
}
