//==============================================================================
/// The `new_charger` primitive is used to assign a bus to an available charger.
//
mod new_charger {

    // Import modules
    use crate::sa::charger::Charger;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_charger` module. Given the set of routes and a bus ID and visit, the run
    /// function shall assign a bus to an available charger and return the new schedule. Return false if assignment
    // failed and true if successful.
    ///
    /// # Input
    /// * ch: Charger object
    /// * q: Queue wanting to be assigned to
    /// * ts: Time slice to assign to queue
    /// * b: Bus id
    ///
    /// # Output
    /// * bool: Assignment failure/success
    ///
    #[allow(dead_code)]
    pub fn run(ch: &mut Charger, q: usize, ts: (f32, f32), b: usize) -> bool {
        ch.assign(q, ts, b)
    }
}
