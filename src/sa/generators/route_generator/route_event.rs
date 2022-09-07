//=========================================================================
// Import Crates
use std::cmp::Ordering;

//=========================================================================
// Import modules
pub use crate::sa::generators::route_generator::bus::Bus;

//===============================================================================
/// Structure for route
///
/// Defines the structure that contains the route data
///
/// * [Sorting structs](rust-lang-nursery.github.iol/rust-cookbook/algorithms/sorting.html)
/// * [Implementation for partial equality](rosettacode.org/wiki/Sort_an_array_of_composite_structures#Rust)
///
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct RouteEvent
{
    // Parameters
    pub arrival_time   : f32,
    pub bus            : Bus,
    pub departure_time : f32,
    pub discharge      : f32,
    pub id             : u16,
    pub route_time     : f32,

    // Decision variables
    pub attach_time    : f32,
    pub detatch_time   : f32,
    pub queue          : u16,
}

//===============================================================================
//
impl PartialEq for RouteEvent
{
    //--------------------------------------------------------------------------
    /// Tests for `self` and `other` values to be equal by using `==` operator.
    ///
    /// # INPUT
    /// * `other`: Object to be comparing to
    ///
    /// # OUTPUT
    /// * `bool`: True if all items match
    ///
    fn eq(&self, other: &Self) -> bool
    {
        return self.arrival_time == other.arrival_time;
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
    fn ne(&self, other: &Self) -> bool
    {
        return self.arrival_time != other.arrival_time;
    }
}

//===============================================================================
//
impl Eq for RouteEvent
{}

//===============================================================================
//
impl PartialOrd for RouteEvent
{
    //--------------------------------------------------------------------------
    /// Returns an ordering between `self` and `other` values if one exists.
    ///
    /// # INPUT
    /// * `other`: Object to be comparing to
    ///
    /// # OUTPUT
    /// * `Option<Ordring>`: An ordering of the objects if one exists
    ///
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        return Some(self.cmp(&other));
    }
}

//===============================================================================
//
impl Ord for RouteEvent
{
    //--------------------------------------------------------------------------
    /// Returns an `Ordering` between `self` and `other`
    ///
    /// # INPUT
    /// * `other`: Object to be comparing to
    ///
    /// # OUTPUT
    /// * `Option<Ordring>`: An ordering of the objects if one exists
    ///
    fn cmp(&self, other: &Self) -> Ordering
    {
        return self.arrival_time.partial_cmp(&other.arrival_time).unwrap();
    }
}
