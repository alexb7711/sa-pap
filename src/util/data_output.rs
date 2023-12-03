//===============================================================================
/// Module to output data to a CSV file in a format so LaTeX can plot it.
//
pub mod DataOutput {
    //==========================================================================
    // Standard library
    use csv;

    //==========================================================================
    // Import modules
    use crate::sa::data::Data;
    use crate::sa::Results;

    //===========================================================================
    // PUBLIC

    //---------------------------------------------------------------------------
    /// Output data in a format for LaTeX to be able to plot
    ///
    /// # Input:
    /// * fn : Base name of the file
    /// * dm : Data manager
    /// * str: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    pub fn output_data(file_name: String, r: Results, path: Option<String>) {
        // Extract path string
        let fp: String;
        if let Some(p) = path {
            fp = p;
        } else {
            fp = String::from("../data/");
        }

        // Extract data
        let d = r.data;

        // Create Plots
        charge_out(&file_name, &d, &fp);
        usage_out(&file_name, &d, &fp);
        power_out(&file_name, &d, &fp);
        acc_energy_out(&file_name, &d, &fp);
        schedule_out(&file_name, &d, &fp);
    }

    //===========================================================================
    // PRIVATE

    //---------------------------------------------------------------------------
    /// Output charge plot data
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn charge_out(file_name: &String, d: &Data, path: &String) {}

    //---------------------------------------------------------------------------
    /// Output charger usage data
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn usage_out(file_name: &String, d: &Data, path: &String) {}

    //---------------------------------------------------------------------------
    /// Output power usage data
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn power_out(file_name: &String, d: &Data, path: &String) {}

    //---------------------------------------------------------------------------
    /// Accumulated output power usage data
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn acc_energy_out(file_name: &String, d: &Data, path: &String) {}

    //---------------------------------------------------------------------------
    /// Output schedule data
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn schedule_out(file_name: &String, d: &Data, path: &String) {}
}
