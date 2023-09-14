//===============================================================================
// External Crates
use rand::Rng;
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
pub struct Charger {
    // Public
    pub schedule: Vec<Vec<Assignment>>, // Lists of scheduled charge times
    pub free_time: Vec<Vec<(f32, f32)>>, // Lists of free times

    // Private
    config: Yaml,
}

//===============================================================================
/// Implementation of Charger
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
        if !self.check_in_bounds(&c) {
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
            if (c.0 <= ts.t.0 && c.1 <= ts.t.0) || (c.0 >= ts.t.1 && c.1 >= ts.t.1) {
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
    /// The `find_free_time` function checks if the arrival/departure time fits in the time slice. If it is smaller than
    /// the time slice, return itself. Otherwise, return the sub-time slice if available.
    ///
    /// # Input
    /// * ae: Arrival/departure times
    /// * ts: Available time slice
    ///
    /// # Output
    /// * (fits, ud) : The tuple indicating that the arrival/departure time fits and the charge start/stop charge times
    ///
    pub fn find_free_time(
        self: &mut Charger,
        ae: &(f32, f32),
        ts: &(f32, f32),
    ) -> (bool, (f32, f32)) {
        let lower = ts.0;
        let upper = ts.1;
        let a = ae.0;
        let e = ae.1;

        // Create random object
        let mut rng = rand::thread_rng();

        // Create start/stop charging tuple
        let mut fits = true;

        // Create charge start/stop buffers
        let mut u: f32 = a;
        let mut d: f32 = e;

        // If the time slice and the arrival/departure times don't match up, immediately return a fail
        if (a <= lower && e <= lower) || (a >= upper && e >= upper) {
            fits = false;
            return (fits, (*ae));
        }

        // The arrival/departure times are fully within the free time
        if lower <= a && upper >= e {
            u = rng.gen_range(a..e);
            d = rng.gen_range(u..e);
        // The departure time is fully within the free time and the arrival time is less than the lower bound
        } else if lower >= a && upper >= e {
            u = rng.gen_range(lower..e);
            d = rng.gen_range(u..e);
        // The arrival time is fully within the free time and the departure time is greater than the lower bound
        } else if lower <= a && upper <= e {
            u = rng.gen_range(a..upper);
            d = rng.gen_range(u..upper);
        // The arrival/departure times are less than and greater than the lower and upper bound, respectively
        } else if lower >= a && upper <= e {
            u = rng.gen_range(lower..upper);
            d = rng.gen_range(u..upper);
        } else {
            fits = false;
        }

        return (fits, (u, d));
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
        // Extract the BOD and EOD
        let bod = self.config.clone()["time"]["BOD"].as_f64().unwrap() as f32;
        let eod = self.config.clone()["time"]["EOD"].as_f64().unwrap() as f32;

        // Create the appropriate number of schedules and free time lists
        for _ in 0..q {
            self.schedule.push(Vec::new());
            self.free_time.push(vec![(bod, eod)]);
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

        // If there is nothing in the schedule the free time is the entire day
        if self.schedule[q].is_empty() {
            ft.push((bod, eod));
        }

        // For each item in the schedule
        for it in self.schedule[q].iter() {
            let s = it;

            // If the iterator is the first item in the list
            if s == self.schedule[q].first().unwrap() {
                ft.push((bod, s.t.0));
            // Else the iterator is in the middle of the list
            } else {
                match s_prev {
                    Some(s_prev) => ft.push((s_prev.t.1, s.t.0)),
                    None => panic!("'s_prev' is empty: charger.rs"),
                }
            }

            // If the iterator is the last item in the list
            if s == self.schedule[q].last().unwrap() {
                ft.push((s.t.1, eod));
            }

            // Update the previous iterator
            s_prev = Some(s);
        }

        // Update the free time vector
        self.free_time[q] = ft;
    }

    //--------------------------------------------------------------------------
    /// The `check_in_bounds' function checks if the specified times are within the time horizon
    ///
    /// # Input
    /// * q: Index of the charger to update free time
    ///
    /// # Output
    /// * bool: true if the values are within the time horizon, false otherwise
    ///
    fn check_in_bounds(self: &mut Charger, c: &(f32, f32)) -> bool {
        let bod = self.config.clone()["time"]["BOD"].as_f64().unwrap() as f32;
        let eod = self.config.clone()["time"]["EOD"].as_f64().unwrap() as f32;

        // Check the ordering
        //
        // - BOD <= c.0
        // - c.0 <= c.1
        // - c.1 <= EOD
        //
        if bod <= c.0 && c.0 <= c.1 && c.1 <= eod {
            return true;
        }

        return false;
    }
}
