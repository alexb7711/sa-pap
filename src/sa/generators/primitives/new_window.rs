//==============================================================================
/// The `new_window` primitive is used to remove then add a bus back in.
//
pub mod new_window {

    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::generators::primitives::new_visit::*;
    use crate::sa::generators::primitives::purge::*;
    use crate::sa::route::route_event::RouteEvent;

    //--------------------------------------------------------------------------
    /// The run function executes the `new_window` module. This module encapsulates
    /// a `purge` then `new_visit`.
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
        ad: &(f32, f32),
        ud: &(f32, f32),
    ) -> bool {
        // Remove the visit, return false if unsuccessful
        if !purge::run(r, i, ch, q, ud) {
            return false;
        }

        // Add the same bus back in as a new visit, return false if unsuccessful
        if !new_visit::run(ch, q, ad) {
            return false;
        }

        return true;
    }
}
