//==============================================================================
/// The `purge` primitive is used to assign a bus to an available charger.
//
pub mod purge {

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::route::route_event::RouteEvent;

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
    pub fn run(
        r: &mut Vec<RouteEvent>,
        i: usize,
        ch: &mut Charger,
        q: usize,
        ud: &(f32, f32),
    ) -> bool {
        if ch.remove(q, *ud) {
            // Update route data
            if r.len() > 0 {
                r[i].queue = r[i].id; // Put BEB in wait queue
                r[i].attach_time = r[i].arrival_time; // Attach time is arrival time
                r[i].detach_time = r[i].departure_time; // Detach time is departure time
            }
            return true;
        } else {
            return false;
        }
    }
}
