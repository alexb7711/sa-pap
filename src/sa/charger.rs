//===============================================================================
// Import modules

//===============================================================================
/// Structure to track charger information
//
#[allow(dead_code)]
pub struct Charger {
    q: Vec<Vec<f32>>, // Lists of scheduled charge times
}

//===============================================================================
/// Implementation of SA
//
impl Charger {
    //---------------------------------------------------------------------------
    /// Constructor that returns a Charger object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    pub fn new() -> Charger {
        // Create new Charger
        let c = Charger { q: Vec::new() };

        return c;
    }

    //--------------------------------------------------------------------------
    //
    pub fn assign(self: &mut Charger) {}

    //--------------------------------------------------------------------------
    //
    pub fn remove() {}

    //--------------------------------------------------------------------------
    //
    pub fn get_schedule() {}
}
