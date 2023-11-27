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
    pub fn run(d: &mut Data, i: usize, ch: &mut Charger, q: usize, ud: &(f32, f32)) -> bool {
        if ch.remove(q, *ud) {
            // Update route data
            if d.len() > 0 {
                d[i].queue = d[i].id; // Put BEB in wait queue
                d[i].attach_time = d[i].arrival_time; // Attach time is arrival time
                d[i].detach_time = d[i].departure_time; // Detach time is departure time
            }
            return true;
        } else {
            return false;
        }
    }
}
