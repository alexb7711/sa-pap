#[allow(non_snake_case)]
//===============================================================================
/// Module to output data to a CSV file in a format so LaTeX can plot it.
//
pub mod DataOutput {
    //==========================================================================
    // Standard library
    use chrono::{DateTime, Local};
    use csv::Writer;
    use std::fs;

    //==========================================================================
    // Import modules
    use crate::sa::charger::Charger;
    use crate::sa::data::Data;
    use crate::sa::Results;

    //==========================================================================
    // Static data
    static E_CELL: &str = "nan"; // Text to place in empty cell
    static STEP_CNT: usize = 1000; // Set static step count

    //===========================================================================
    // PUBLIC

    //---------------------------------------------------------------------------
    /// Output data in a format for LaTeX to be able to plot
    ///
    /// # Input:
    /// * fn: Base name of the file
    /// * r: Results structure
    /// * path: Path to output directory
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
            // Get the month and time strings
            let current_local: DateTime<Local> = Local::now();
            let directory = current_local.format("%m/%d/%H-%M-%S/").to_string();
            let directory = "data/".to_string() + directory.as_str();

            // Create Directories
            fs::create_dir_all(directory.clone()).unwrap();

            // Create file with score
            let fields: Vec<String> = Vec::new();
            let data: Vec<Vec<f32>> = Vec::new();
            save_to_file(&directory.clone(), &r.score.to_string(), &fields, data);

            // Store handle to directory
            fp = String::from(directory);
        }

        // Extract data
        let d = r.data;
        let c = r.charger;

        // Create Plots
        charge_out(&file_name, &d, &c, &fp);
        charger_count_out(&file_name, &d, &c, &fp);
        power_out(&file_name, &d, &c, &fp);
        acc_energy_out(&file_name, &d, &c, &fp);
        schedule_out(&file_name, &d, &c, &fp);
    }

    //===========================================================================
    // PRIVATE

    //---------------------------------------------------------------------------
    /// Output charge data for each BEB over time
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * char: Charger object
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn charge_out(file_name: &String, dat: &Data, char: &Charger, path: &String) {
        let slow = char.charger_count.0;
        if dat.param.conv[slow] > 0.0 {
            nonlinear_soc(file_name, dat, char, path);
        } else {
            linear_soc(file_name, dat, char, path);
        }
    }

    //---------------------------------------------------------------------------
    /// Output the charger count over the time horizon
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * char: Charger object
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn charger_count_out(file_name: &String, dat: &Data, char: &Charger, path: &String) {
        // Variables
        let K: usize = STEP_CNT;
        let N: usize = dat.param.N;
        let T: f32 = dat.param.T;
        let dt: f32 = T as f32 / K as f32;
        let u: &Vec<f32> = &dat.dec.u;
        let w: &Vec<Vec<bool>> = &dat.dec.w;
        let v: &Vec<usize> = &dat.dec.v;
        let d: &Vec<f32> = &dat.dec.d;

        // Table variables
        let name = file_name.to_owned() + &"-charge-cnt";
        let wait: usize = char.charger_count.0;
        let slow: usize = char.charger_count.1;
        let fields: Vec<String> = vec![
            String::from("visit"),
            String::from("wait"),
            String::from("slow"),
            String::from("fast"),
        ];
        let mut data: Vec<Vec<f32>> = vec![vec![0.0; 4]; K];

        // For each time step
        for k in 0..K {
            // Calculate time slice
            let t: f32 = k as f32 * dt;
            data[k as usize][0] = t;

            // For each visit
            for i in 0..N {
                // Ignore the very early times as it breaks GNUPlot filling
                if t < 4.0 {
                    continue;
                }

                // if the time step is between the current visit and the BEB has
                // assigned
                if u[i] <= t && d[i] >= t && w[i][v[i]] {
                    // If the BEB is in a waiting queue
                    if v[i] < wait {
                        data[k as usize][1] += 1.0;
                    // Else if the BEB is in a slow charging queue
                    } else if v[i] >= wait && v[i] < wait + slow {
                        data[k as usize][2] += 1.0;
                    // Else if the BEB is in a fast charging queue
                    } else if v[i] >= wait + slow {
                        data[k as usize][3] += 1.0;
                    }
                }
            }
        }

        // Write data to disk
        save_to_file(path, &name, &fields, data);
    }

    //---------------------------------------------------------------------------
    /// Output power usage data
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * char: Charger object
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn power_out(file_name: &String, dat: &Data, _char: &Charger, path: &String) {
        // Variables
        let K: usize = STEP_CNT;
        let N: usize = dat.param.N;
        let T: f32 = dat.param.T;
        let d: &Vec<f32> = &dat.dec.d;
        let dt: f32 = T as f32 / K as f32;
        let r: &Vec<f32> = &dat.param.r;
        let u: &Vec<f32> = &dat.dec.u;
        let v: &Vec<usize> = &dat.dec.v;
        let w: &Vec<Vec<bool>> = &dat.dec.w;

        // Table variables
        let name = file_name.to_owned() + &"-power-usage";
        let mut data: Vec<Vec<f32>> = vec![vec![0.0; 2]; K];
        let fields: Vec<String> = vec![String::from("time"), String::from("power")];

        // For each time step
        for k in 0..K {
            // Calculate time slice
            let t: f32 = k as f32 * dt;
            data[k as usize][0] = t;

            // Ignore early times as it breaks GNUPlot
            if t < 4.0 {
                continue;
            }

            // For each visit
            for i in 0..N {
                // if the time step is between the current visit and the BEB has
                // assigned
                if w[i][v[i]] && u[i] <= t && d[i] >= t {
                    // Add on the accumulated power for the current discrete time slice
                    data[k as usize][1] += r[v[i]];
                }
            }
        }

        // Write data to disk
        save_to_file(path, &name, &fields, data);
    }

    //---------------------------------------------------------------------------
    /// Accumulated energy data output
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * char: Charger object
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn acc_energy_out(file_name: &String, dat: &Data, _char: &Charger, path: &String) {
        // Variables
        let K: usize = STEP_CNT;
        let N: usize = dat.param.N;
        let T: f32 = dat.param.T;
        let d: &Vec<f32> = &dat.dec.d;
        let dt: f32 = T as f32 / K as f32;
        let r: &Vec<f32> = &dat.param.r;
        let u: &Vec<f32> = &dat.dec.u;
        let v: &Vec<usize> = &dat.dec.v;
        let w: &Vec<Vec<bool>> = &dat.dec.w;

        // Table variables
        let name = file_name.to_owned() + &"-acc-energy-usage";
        let mut data: Vec<Vec<f32>> = vec![vec![0.0; 2]; K];
        let fields: Vec<String> = vec![String::from("time"), String::from("power")];

        // For each time step
        for k in 0..K {
            // Calculate time slice
            let t: f32 = k as f32 * dt;
            data[k as usize][0] = t;

            // If the index is greater than zero
            if k > 0 {
                // Use the previous data
                data[k][1] = data[k - 1][1];
            }

            // For each visit
            for i in 0..N {
                // if the time step is between the current visit and the BEB has
                // assigned
                if u[i] <= t && d[i] >= t && w[i][v[i]] {
                    // Add on the accumulated energy for visit `i`
                    data[k as usize][1] += r[v[i]] * dt;
                }
            }
        }

        // Write data to disk
        save_to_file(path, &name, &fields, data);
    }

    //---------------------------------------------------------------------------
    /// Output schedule data
    ///
    /// # Input:
    /// * file_name : Base name of the file
    /// * d : Data manager
    /// * char: Charger object
    /// * path: Path to output directory
    ///
    /// # Output:
    /// * Data files
    ///
    fn schedule_out(file_name: &String, dat: &Data, char: &Charger, path: &String) {
        // Variables
        let A: usize = dat.param.A;
        let N: usize = dat.param.N;
        let G: &Vec<u16> = &dat.param.Gam;
        let u: &Vec<f32> = &dat.dec.u;
        let v: &Vec<usize> = &dat.dec.v;
        let s: &Vec<f32> = &dat.dec.s;
        let w: &Vec<Vec<bool>> = &dat.dec.w;
        let wait: usize = char.charger_count.0;

        // Table variables
        let name = file_name.to_owned() + &"-schedule";
        let mut data: Vec<Vec<f32>> = vec![vec![-1.0; 3 * A]; N];
        let fields: Vec<Vec<String>> = (0..A)
            .map(|b| {
                vec![
                    String::from("charger").to_owned() + &b.to_string(),
                    String::from("u").to_owned() + &b.to_string(),
                    String::from("s").to_owned() + &b.to_string(),
                ]
            })
            .collect();
        let fields: Vec<String> = fields.into_iter().flatten().collect();

        // For each visit
        for i in 0..N {
            // If the current visit is scheduled, the charge duration is of some significant time, and the BEB was
            // placed on a non-waiting queue.
            if w[i][v[i]] && s[i] > 0.001 && v[i] >= wait {
                // Include the information in the data buffer
                data[i][G[i] as usize * 3 + 0] = (v[i] - wait) as f32;
                data[i][G[i] as usize * 3 + 1] = u[i];
                data[i][G[i] as usize * 3 + 2] = s[i];
            }
        }

        // Write data to disk
        save_to_file(path, &name, &fields, data);
    }

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

        // BUG: this seems to break output
        // let mut empty_rows: Vec<usize> = Vec::new();
        // let mut idx: usize = 0;

        // For each row
        for row in data_s.iter_mut() {
            // For each item in the row
            for i in 0..row.len() {
                // If the row item is a '-1.0', replace it
                if row[i] == "-1" {
                    row[i] = String::from(E_CELL);
                }
            }

            // BUG: this seems to break output
            // // If the row is only commas, clear it
            // if row.iter().all(|i| i == E_CELL) {
            //     empty_rows.push(idx);
            // }
            // Update index
            // idx += 1;
        }

        // BUG: this seems to break output
        // // Clear out empty rows
        // for i in empty_rows {
        //     data_s.remove(i);
        // }

        //  Save data to disk
        if let Ok(mut wtr) = Writer::from_path(file_name.clone()) {
            // Write each row to disk
            wtr.write_record(fields).unwrap();
            data_s.iter().for_each(|row| wtr.write_record(row).unwrap());
        } else {
            panic!("Could not write to the file: {}", file_name);
        }
    }

    fn linear_soc(file_name: &String, dat: &Data, _char: &Charger, path: &String) {
        // Variables
        let name = file_name.to_owned() + &"-charge";
        let N = dat.param.N;
        let A = dat.param.A;
        let G = &dat.param.Gam;
        let eta = &dat.dec.eta;
        let u = &dat.dec.u;
        let d = &dat.dec.d;
        let v = &dat.dec.v;
        let r = &dat.param.r;
        let s = &dat.dec.s;
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

            // For every visit
            for i in 0..N {
                println!("LINEAR");
                // If the current visit is for the BEB of interest
                if G[i] as usize == b {
                    // Append the charge on arrival
                    data[t_i][b * 2 + 0] = u[i];
                    data[t_i][b * 2 + 1] = eta[i];

                    // Append the charge on departure
                    data[t_i + 1][b * 2 + 0] = d[i];
                    data[t_i + 1][b * 2 + 1] = eta[i] + s[i] * r[v[i]];

                    // Update the index
                    t_i += 2;
                }
            }
        }

        // Write data to disk
        save_to_file(path, &name, &fields, data);
    }

    fn nonlinear_soc(file_name: &String, dat: &Data, _char: &Charger, path: &String) {
        // Variables
        let name = file_name.to_owned() + &"-charge";
        let N = dat.param.N;
        let A = dat.param.A;
        let G = &dat.param.Gam;
        let eta = &dat.dec.eta;
        let kappa = &dat.param.k;
        let u = &dat.dec.u;
        let d = &dat.dec.d;
        let v = &dat.dec.v;
        let r = &dat.param.conv;
        let s = &dat.dec.s;
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

            // For every visit
            for i in 0..N {
                // If the current visit is for the BEB of interest
                if G[i] as usize == b {
                    // Calculate model parameters
                    let dt = s[i] * 3600.0;
                    let abar = f32::exp(-r[v[i]] * dt);
                    let bbar = abar - 1.0;

                    // Append the charge on arrival
                    data[t_i][b * 2 + 0] = u[i];
                    data[t_i][b * 2 + 1] = eta[i];

                    // Append the charge on departure
                    data[t_i + 1][b * 2 + 0] = d[i];
                    data[t_i + 1][b * 2 + 1] = eta[i] * abar - bbar * kappa[G[i] as usize];

                    if eta[i] * abar - bbar * kappa[G[i] as usize] > 388.0 {
                        println!("OUTPUT ISSUES")
                    }

                    // Update the index
                    t_i += 2;
                }
            }
        }

        // Write data to disk
        save_to_file(path, &name, &fields, data);
    }
}
