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
/// Module to run the plots
//
pub mod plot {
    //--------------------------------------------------------------------------
    // Import modules
    use crate::plotter::accumulated_energy_usage_plot::AccumulatedEnergyUsagePlot;
    use crate::plotter::charge_plot::ChargePlot;
    use crate::plotter::charger_usage_plot::ChargerUsagePlot;
    use crate::plotter::power_usage_plot::PowerUsagePlot;
    use crate::plotter::schedule_plot::SchedulePlot;
    use crate::plotter::Plotter;
    use crate::sa::data::Data;

    //---------------------------------------------------------------------------
    /// Runs all of the plotting code.
    ///
    /// # Input
    /// * should_plot: flag to indicate whether to run the plotting functions
    /// * d: Boxed data
    ///
    /// # Output
    /// * None
    ///
    pub fn run(should_plot: bool, d: &mut Box<Data>) {
        // If the flag is `false`, return early
        if !should_plot {
            return;
        }

        // Indicate the plots are being drown
        println!("Drawing plots...");

        // Execute plots
        AccumulatedEnergyUsagePlot::plot(d);
        ChargePlot::plot(d);
        ChargerUsagePlot::plot(d);
        PowerUsagePlot::plot(d);
        SchedulePlot::plot(d);
    }
}

//===============================================================================
/// Trait to define `Generator` interfaces
//
pub trait Plotter {
    fn plot(d: &mut Box<Data>) -> bool;
}
