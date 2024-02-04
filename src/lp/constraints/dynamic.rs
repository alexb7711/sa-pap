//===============================================================================
// Declare modules
pub mod charge_propagation;
pub mod init_final_charge;
pub mod scalar_to_vector_queue;

//==============================================================================
/// Module that runs all the dynamic constraints
//
pub mod dynamic {
    //==========================================================================
    // Import modules
    use crate::lp::constraints::dynamic::charge_propagation::ChargePropagate;
    use crate::lp::constraints::dynamic::scalar_to_vector_queue::ScalarToVectorQueue;
    use crate::lp::constraints::Constraint;
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;

    //--------------------------------------------------------------------------
    /// Run the packing constraints. If they all pass, return true. Otherwise
    /// return false.
    ///
    /// # Input
    /// * d: Data for the current model
    /// * i: index of the visit
    /// * j: index for the queue
    ///
    /// # Output
    /// * bool: Constraints successfully/unsuccessfully applied
    ///
    pub fn run(d: &mut Data, ch: &mut Charger, i: usize, j: usize) -> bool {
        if !ChargePropagate::run(d, ch, i, j) {
            return false;
        }

        if !ScalarToVectorQueue::run(d, ch, i, j) {
            return false;
        }

        // Indicate success
        return true;
    }
}
