//=========================================================================
// Import Crates
use std::cmp::Ordering;

//===============================================================================
/// Structure for buses
///
/// Defines the structure that contains the bus data.
///
/// * [Sorting structs](rust-lang-nursery.github.iol/rust-cookbook/algorithms/sorting.html)
/// * [Implementation for partial equality](rosettacode.org/wiki/Sort_an_array_of_composite_structures#Rust)
///
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Bus {
    pub bat_capacity: f32,
    pub initial_charge: f32,
    pub discharge_rate: f32,
    pub final_charge: f32,
}

//===============================================================================
//
impl PartialEq for Bus {
    //--------------------------------------------------------------------------
    /// Tests for `self` and `other` values to be equal by using `==` operator.
    ///
    /// # INPUT
    /// * `other`: Object to be comparing to
    ///
    /// # OUTPUT
    /// * `bool`: True if all items match
    ///
    fn eq(&self, other: &Self) -> bool {
        return (self.initial_charge == other.initial_charge)
            && (self.discharge_rate == other.discharge_rate);
    }

    //--------------------------------------------------------------------------
    /// Tests for `self` and `other` values to be not equal by using `==` operator.
    ///
    /// # INPUT
    /// * `other`: Object to be comparing to
    ///
    /// # OUTPUT
    /// * `bool`: True if no items match
    ///
    fn ne(&self, other: &Self) -> bool {
        return (self.initial_charge != other.initial_charge)
            && (self.discharge_rate != other.discharge_rate);
    }
}

//===============================================================================
//
impl Eq for Bus {}

//===============================================================================
//
impl PartialOrd for Bus {
    //--------------------------------------------------------------------------
    /// Returns an ordering between `self` and `other` values if one exists.
    ///
    /// # INPUT
    /// * `other`: Object to be comparing to
    ///
    /// # OUTPUT
    /// * `Option<Ordering>`: An ordering of the objects if one exists
    ///
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(&other));
    }
}

//===============================================================================
//
impl Ord for Bus {
    //--------------------------------------------------------------------------
    /// Returns an `Ordering` between `self` and `other`
    ///
    /// # INPUT
    /// * `other`: Object to be comparing to
    ///
    /// # OUTPUT
    /// * `Option<Ordring>`: An ordering of the objects if one exists
    ///
    fn cmp(&self, other: &Self) -> Ordering {
        return self
            .initial_charge
            .partial_cmp(&other.initial_charge)
            .unwrap();
    }
}
