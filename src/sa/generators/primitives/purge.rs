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
            if d.param.N > 0 {
                // Put BEB in wait queue
                d.dec.v[i] = d.param.Gam[i] as usize;
                d.dec.w[i][q] = false;
                d.dec.w[i][d.dec.v[i]] = true;

                // Attach time is arrival time
                d.dec.u[i] = d.param.a[i];

                // Detach time is departure time
                d.dec.c[i] = d.param.e[i];
            }
            return true;
        } else {
            return false;
        }
    }
}
