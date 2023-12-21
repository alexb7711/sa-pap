#[allow(non_snake_case)]
//===============================================================================
/// Module to output data to a CSV file in a format so LaTeX can plot it.
//
pub mod DataOutput {
    //==========================================================================
    // Standard library
    use csv::Writer;

    //==========================================================================
    // Import modules
    use crate::sa::data::Data;
    use crate::sa::Results;

    //==========================================================================
    // Static data
    static E_CELL: &str = "null";

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
            fp = String::from("data/");
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
    fn charge_out(file_name: &String, d: &Data, path: &String) {
        // Variables
        let name = file_name.to_owned() + &"-charge";
        let N = d.param.N;
        let A = d.param.A;
        let G = &d.param.Gam;
        let eta = &d.dec.eta;
        let u = &d.dec.u;
        let c = &d.dec.c;
        let v = &d.dec.v;
        let r = &d.param.r;
        let s = &d.dec.s;
        let mut data: Vec<Vec<f32>> = vec![vec![-1.0; 2 * A]; 2 * N];

        // Create top row of CSV file
        let fields: Vec<Vec<String>> = (0..A)
            .map(|b| {
                vec![
                    String::from("time").to_owned() + &b.to_string(),
                    String::from("eta").to_owned() + &b.to_string(),
                ]
            })
            .collect();
        let fields: Vec<String> = fields.into_iter().flatten().collect();

        // For every bus
        for b in 0..A {
            let mut t_i: usize = 0;

            // Fro every visit
            for i in 0..N {
                // If the current visit is for the BEB of interest
                if G[i] as usize == b {
                    // Append the charge on arrival
                    data[t_i][b * 2 + 0] = u[i];
                    data[t_i][b * 2 + 1] = eta[i];

                    // Append the charge on departure
                    data[t_i + 1][b * 2 + 0] = c[i];
                    data[t_i + 1][b * 2 + 1] = eta[i] + s[i] * r[v[i]];

                    // Update the index
                    t_i += 2;
                }
            }
        }

        // Write data to disk
        save_to_file(path, &name, &fields, data);
    }

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

    //---------------------------------------------------------------------------
    /// Write data to CSV file
    ///
    /// # Input:
    /// * path   : Path to output directory
    /// * name   : Name of the file
    /// * fields : Title each column
    /// * data   : Matrix of data
    ///
    /// # Output:
    /// * CSV file located at 'PATH/NAME' with DATA as content
    fn save_to_file(path: &String, name: &String, fields: &Vec<String>, data: Vec<Vec<f32>>) {
        // Variables
        let file_name = path.to_owned() + name + &".csv";

        // Convert data to strings
        let mut data_s: Vec<Vec<String>> = Vec::new();
        for d in data {
            let d_tmp: Vec<String> = d.into_iter().map(|i| i.to_string()).collect();
            data_s.push(d_tmp);
        }

        // For each row
        let mut empty_rows: Vec<usize> = Vec::new();
        let mut idx: usize = 0;
        for row in data_s.iter_mut() {
            // For each item in the row
            for i in 0..row.len() {
                // If the row item is a '-1.0', replace it
                if row[i] == "-1.0" {
                    row[i] = String::from(E_CELL);
                }
            }

            // If the row is only commas, clear it
            if row.iter().all(|i| i == E_CELL) {
                empty_rows.push(idx);
            }

            // Update index
            idx += 1;
        }

        // Clear out empty rows
        for i in empty_rows {
            data_s.remove(i);
        }

        //  Save data to disk
        if let Ok(mut wtr) = Writer::from_path(file_name.clone()) {
            // Write each row to disk
            println!("Saving to disk");
            // fields.iter().for_each(|f| wtr.write_record(f).unwrap());
            wtr.write_record(fields).unwrap();
            data_s.iter().for_each(|row| wtr.write_record(row).unwrap());
        } else {
            panic!("Could not write to the file: {}", file_name);
        }
    }
}
