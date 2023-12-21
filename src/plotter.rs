//===============================================================================
// Import modules
use crate::sa::data::Data;

//===============================================================================
// Declare modules
pub mod accumulated_energy_usage_plot;
pub mod charge_plot;
pub mod charger_usage_plot;
pub mod power_usage_plot;
pub mod schedule_plot;

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Plotter {
    fn plot(&mut self, name: String, d: &mut Box<Data>) -> bool;
}
