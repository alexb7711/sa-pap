//===============================================================================
// External Crates
use yaml_rust::Yaml;

//===============================================================================
// Import modules
use crate::util::fileio::yaml_loader;

//===============================================================================
/// Structure to consolidate the bus assignment information
///
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Assignment {
    pub t: (f32, f32),
    pub b: usize,
}

//===============================================================================
/// Structure to track charger information
//
#[allow(dead_code)]
pub struct Charger {
    // Public
    pub schedule: Vec<Vec<Assignment>>, // Lists of scheduled charge times

    // Private
    config: Yaml,
    free_time: Vec<Vec<(f32, f32)>>, // Lists of free times
}

//===============================================================================
/// Implementation of Charger
//
// BUG: The charger does not make sure that the times are within the time horizon
//
impl Charger {
    //---------------------------------------------------------------------------
    /// Constructor that returns a Charger object
    ///
    /// # Input
    /// * q: Optional number of chargers to create. Defaults to one.
    ///
    /// # Output
    /// * Return a charger object
    ///
    pub fn new(config_path: &str, q: Option<usize>) -> Charger {
        // Extract the number of queues
        let q: usize = q.unwrap_or(1 as usize);

        // Create a charger
        let mut c: Charger = Charger {
            schedule: Vec::new(),
            config: yaml_loader::load_yaml(config_path),
            free_time: Vec::new(),
        };

        // Create the number of queues specified
        c.add_chargers(q);

        return c;
    }

    //--------------------------------------------------------------------------
    /// Assigns a bus with an identification $id$ to a charger queue $q$ if the time slice $c$ is available.
    ///
    /// # Input
    /// * q: Charger queue index
    /// * c: Candidate time frame tuple
    /// * id: Identification number of the bus
    ///
    /// # Output
    /// * assigned: True if the bus was successfully assigned, false otherwise
    ///
    pub fn assign(self: &mut Charger, q: usize, c: (f32, f32), id: usize) -> bool {
        // Default the assigned to false
        let mut assigned: bool = false;

        // Check that the time slice is increasing
        if c.0 > c.1 {
            return assigned;
        }

        // If the space is available, assign the bus to that time slot
        if self.avail(&q, &c) {
            // Create Assignment
            let a: Assignment = Assignment { b: id, t: c };

            // Assign
            self.schedule[q].push(a);

            // Sort the schedule by the starting charge time
            // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
            self.schedule[q].sort_by(|a, b| a.t.0.partial_cmp(&b.t.0).unwrap());

            // Indicate that the assignment was successful
            assigned = true;
        }

        // Update the free time for the qth charger
        self.update_free_time(q);

        return assigned;
    }

    //--------------------------------------------------------------------------
    /// Remove the assignment of $c$ in charger queue $q$.
    ///
    /// # Input
    /// * q: Charger queue index
    /// * c: Candidate time frame tuple
    ///
    /// # Output
    /// * rem: True if the time slice was removed, false otherwise
    ///
    pub fn remove(self: &mut Charger, q: usize, c: (f32, f32)) -> bool {
        // Default to indicate that the time slice item was not removed
        let mut rem: bool = false;

        // Check if the time slice exists in the charger queue
        if self.exists(&q, &c) {
            // State that the item is being removed
            rem = true;

            // Remove the item
            self.schedule[q].retain(|s| s.t != c);
        }

        // Update the free time for the qth charger
        self.update_free_time(q);

        return rem;
    }

    //--------------------------------------------------------------------------
    /// The `avail` function checks if the given a charger queue and time frame is available for assignment.
    ///
    /// # Input
    /// * q: Charger queue index
    /// * c: Candidate time frame tuple
    ///
    /// # Output
    /// * avail: True if the time slice is available, false otherwise
    ///
    pub fn avail(self: &mut Charger, q: &usize, c: &(f32, f32)) -> bool {
        // If the queue is empty, return true
        if self.schedule[*q].len() == 0 {
            return true;
        }

        // Iterate through the schedule for charger q
        for it in self.schedule[*q].iter() {
            // Extract the iterator
            let ts = *it;

            // Compare the current scheduled time with the candidate
            // If the candidate time has any of the following properties:
            //
            // * the candidates initial and final times are not less than the current time slice's initial time or
            // * the candidates initial and final times are not greater than the current time slice's final time
            //
            if (c.0 < ts.t.0 && c.1 < ts.t.0) || (c.0 > ts.t.1 && c.1 > ts.t.1) {
                continue;
            } else {
                // Return that there is no availability
                println!("here: candidate: {:?}, time slice: {:?}", c, ts.t);
                return false;
            }
        }

        // Return that there is an available time
        return true;
    }

    //--------------------------------------------------------------------------
    /// The `exists` function checks if given candidate time slice exists in the current queue
    ///
    /// # Input
    /// * q: Charger queue index
    /// * c: Candidate time frame tuple
    ///
    /// # Output
    /// * exists: True if the time slice is in the queue, false otherwise
    ///
    pub fn exists(self: &mut Charger, q: &usize, c: &(f32, f32)) -> bool {
        // Iterate through the vector and check if the time slice does not exist
        if let Some(_) = self.schedule[*q].iter().find(|s| s.t == *c) {
            return true;
        };

        return false;
    }

    //--------------------------------------------------------------------------
    /// The `add_chargers' function adds charger queues.
    ///
    /// # Input
    /// * q: Number of chargers to add
    ///
    /// # Output
    /// * NONE
    ///
    pub fn add_chargers(self: &mut Charger, q: usize) {
        // Create the appropriate number of schedules and free time lists
        for _ in 0..q {
            self.schedule.push(Vec::new());
            self.free_time.push(Vec::new());
        }
    }

    //--------------------------------------------------------------------------
    /// The `update_free_time' function updates the times that charger q is available.
    ///
    /// # Input
    /// * q: Index of the charger to update free time
    ///
    /// # Output
    /// * NONE
    ///
    fn update_free_time(self: &mut Charger, q: usize) {
        // Extract the BOD and EOD
        let bod = self.config.clone()["time"]["BOD"].as_f64().unwrap() as f32;
        let eod = self.config.clone()["time"]["EOD"].as_f64().unwrap() as f32;
        let mut s_prev: std::option::Option<&Assignment> = None;

        // Create a new free time vector
        let mut ft: Vec<(f32, f32)> = vec![];

        // For each item in the schedule
        for it in self.schedule[q].iter() {
            let s = it;

            // If the iterator is the first item in the list
            if s == self.schedule[q].first().unwrap() {
                ft.push((bod, s.t.0));
            // Else if the iterator is the last item in the list
            } else if s == self.schedule[q].last().unwrap() {
                ft.push((s.t.1, eod));
            // Else the iterator is in the middle of the list
            } else {
                match s_prev {
                    Some(s_prev) => ft.push((s_prev.t.1, s.t.0)),
                    None => panic!("'s_prev' is empty: charger.rs"),
                }
            }

            // Update the previous iterator
            s_prev = Some(s);
        }
    }
}
