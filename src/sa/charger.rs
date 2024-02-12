//===============================================================================
// External Crates
use rand::Rng;
use yaml_rust::Yaml;

//===============================================================================
// Import modules
use crate::sa::data::Data;
use crate::sa::generators::primitives;
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
///
#[derive(Clone)]
pub struct Charger {
    // Public
    pub schedule: Vec<Vec<Assignment>>, // Lists of scheduled charge times
    pub free_time: Vec<Vec<(f32, f32)>>, // Lists of free times
    pub charger_count: (usize, usize, usize), // Charger counts (wait, slow, fast)
    pub charger_speed: (f32, f32, f32), // Charger speeds (wait, slow, fast)

    // Private
    config: Yaml,
}

//===============================================================================
/// Implementation of Charger
//
impl Charger {
    /////////////////////////////////////////////////////////////////////////////
    // PUBLIC
    /////////////////////////////////////////////////////////////////////////////

    //---------------------------------------------------------------------------
    /// Constructor that returns a Charger object
    ///
    /// # Input
    /// * config_path: String path to the configuration file
    /// * load_c_from_yaml: flag to indicate whether to load charger counts from
    /// * the configuration file
    /// * a_force: Then number of wait chargers to force
    /// * q_force: The total number of chargers to force
    ///
    /// # Output
    /// * Return a charger object
    ///
    pub fn new(
        config_path: &str,
        load_c_from_yaml: bool,
        a_force: Option<usize>,
        q_force: Option<usize>,
    ) -> Charger {
        // Create a charger
        let mut c: Charger = Charger {
            schedule: Vec::new(),
            free_time: Vec::new(),
            charger_count: (0, 0, 1),
            charger_speed: (0.0, 30.0, 910.0),
            config: yaml_loader::load_yaml(config_path),
        };

        // Extract the number of queues
        let mut q: usize = q_force.unwrap_or(1 as usize);

        // Set the charger count
        c.charger_count = (0, 0, q);

        // Load chargers file if specified
        let q_wait: usize;
        if load_c_from_yaml {
            // Extract the number of queues from YAML
            if let Some(a) = a_force {
                q_wait = a;
            } else {
                q_wait = c.config.clone()["buses"]["num_bus"].as_i64().unwrap() as usize;
            }

            let q_slow: usize = c.config.clone()["chargers"]["slow"]["num"]
                .as_i64()
                .unwrap() as usize;

            let q_fast: usize = c.config.clone()["chargers"]["fast"]["num"]
                .as_i64()
                .unwrap() as usize;

            // Update charger count
            q = q_wait + q_slow + q_fast;

            // Set the charger count
            c.charger_count = (q_wait, q_slow, q_fast);

            // Set charger speeds
            let slow_c = c.config.clone()["chargers"]["slow"]["rate"]
                .as_f64()
                .unwrap() as f32;
            let fast_c = c.config.clone()["chargers"]["fast"]["rate"]
                .as_f64()
                .unwrap() as f32;
            c.charger_speed = (0.0, slow_c, fast_c);
        }

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

            // Update the free time for the qth charger
            self.update_free_time(q);
        }

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
            // Store length
            let l_bef = self.schedule[q].len();

            // Remove the item
            self.schedule[q].retain(|s| s.t != c);

            // State that the item is being removed
            rem = l_bef > self.schedule[q].len();

            // Update the free time for the qth charger
            self.update_free_time(q);
        }

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
                return false;
            }
        }

        // Return that there is an available time
        return true;
    }

    //--------------------------------------------------------------------------
    /// The `get_ts` function checks if the given a charger queue and time frame
    /// is available for assignment and returns the time slice.
    ///
    /// # Input
    /// * q: Charger queue index
    /// * c: Candidate time frame tuple
    ///
    /// # Output
    /// * ts: returns the range [L,U] if a time slice is available. [0,0]
    /// is returned otherwise.
    ///
    pub fn get_ts(self: &mut Charger, q: &usize, c: &(f32, f32)) -> (f32, f32) {
        // If the queue is empty, return true
        if self.schedule[*q].len() == 0 {
            return self.free_time[*q][0];
        }

        // Iterate through the schedule for charger q
        for it in self.free_time[*q].iter() {
            // Extract the iterator
            let ts = *it;

            // Compare the current scheduled time with the candidate
            // If the candidate time has any of the following properties:
            //
            // * the candidates initial and final are greater than or equal to the lower free time
            // * the candidates initial and final times are less than or equal to the upper free time
            //
            // That is L <= c.0 <= c.1 <= U.
            //
            if (c.0 >= ts.0 && c.1 >= ts.0) && (c.0 <= ts.1 && c.1 <= ts.1) {
                // Return the matched time slice
                return ts;
            } else {
                // Continue searching for an availability
                continue;
            }
        }

        // Indicate that no time is available
        return (0.0, 0.0);
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

        // Create start/stop charging tuple
        let fits;
        let mut fits_u = false;
        let mut fits_d = false;

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
            (u, fits_u) = self.get_rand_range(None, Some(d), (a, e));
            (d, fits_d) = self.get_rand_range(Some(u), None, (u, e));
            // The departure time is fully within the free time and the arrival time is less than the lower bound
        } else if lower >= a && upper >= e {
            (u, fits_u) = self.get_rand_range(None, Some(d), (lower, e));
            (d, fits_d) = self.get_rand_range(Some(u), None, (u, e));
            // The arrival time is fully within the free time and the departure time is greater than the lower bound
        } else if lower <= a && upper <= e {
            (u, fits_u) = self.get_rand_range(None, Some(d), (a, upper));
            (d, fits_d) = self.get_rand_range(Some(u), None, (u, upper));
            // The arrival/departure times are less than and greater than the lower and upper bound, respectively
        } else if lower > a && upper <= e {
            (u, fits_u) = self.get_rand_range(None, Some(d), (lower, upper));
            (d, fits_d) = self.get_rand_range(Some(u), None, (u, upper));
        }

        // Keep the window above a certain threshold. This value should be bigger than `primitives::EPSILON`
        if d - u < primitives::EPSILON * 10.0 {
            fits = false;
        // Else it fits the threshold
        } else {
            // It fits only if upper and lower bound fit
            fits = fits_u && fits_d;
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
    /// Given MILP data `dat`, update the charge availability matrix.
    ///
    /// # Input
    /// * dat: MILP data object
    ///
    /// # Output
    /// * NONE
    ///
    pub fn milp_to_schedule(self: &mut Charger, dat: &Data) {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Empty the current schedule
        for q in self.schedule.iter_mut() {
            *q = Vec::new();
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Update the schedule with the MILP data
        for i in 0..dat.param.N {
            let a = Assignment {
                b: dat.param.Gam[i] as usize,
                t: (dat.dec.u[i], dat.dec.d[i]),
            };

            self.schedule[dat.dec.v[i]].push(a);
        }

        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Regenerate availability matrix
        for q in 0..dat.param.Q {
            self.update_free_time(q);
        }
    }

    //--------------------------------------------------------------------------
    /// Given the charger index, return the charge rate.
    ///
    /// # Inbox
    /// * q: Charger index
    ///
    /// # Output
    /// * r : Charge rate
    ///
    pub fn get_charge_rate(self: &Charger, q: usize) -> f32 {
        // Ensure the charger index is within bound
        if q >= self.schedule.len() {
            panic!("charger.rs: Charger index is not valid.")
        }

        // Buffer to store charge rate
        let rate: f32;

        // Waiting queue
        if q < self.charger_count.0 {
            rate = self.charger_speed.0;
        // Slow charger
        } else if q >= self.charger_count.0 && q < self.charger_count.0 + self.charger_count.1 {
            rate = self.charger_speed.1;
        // Fast charger
        } else {
            rate = self.charger_speed.2;
        }
        return rate;
    }

    /////////////////////////////////////////////////////////////////////////////
    // PRIVATE
    /////////////////////////////////////////////////////////////////////////////

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

    //--------------------------------------------------------------------------
    /// Given a arrival/departure time black, (a,e), return a random
    /// attach/detach time, (u,d), that has a non-zero time difference, i.e
    /// d - e != 0.0.
    ///
    /// # Input
    /// * u: Current attach time
    /// * d: Current detach times
    /// * lu: Lower/upper bound
    ///
    /// # Output
    /// * v: Random value.
    ///
    fn get_rand_range(
        self: &mut Charger,
        u: Option<f32>,
        d: Option<f32>,
        lu: (f32, f32),
    ) -> (f32, bool) {
        // Create charge start/stop buffers
        let mut v: f32;

        // Create random object
        let mut rng = rand::thread_rng();

        // Check if the window is large enough
        if lu.1 - lu.0 < primitives::EPSILON {
            // If it is not, return false
            return (0.0, false);
        }

        // While there is a zero-difference charge time
        loop {
            // Generate random attach/detach time
            v = rng.gen_range(lu.0..lu.1);

            // If the departure time is being updated
            if let Some(u) = u {
                // Make sure the new departure time does not create a zero-time visit
                if v - u > 0.0 {
                    break;
                }
            }

            // If the arrival time is being updated
            if let Some(d) = d {
                // Make sure the new arrival time does not create a zero-time visit
                if d - v > 0.0 {
                    break;
                }
            }
        }

        return (v, true);
    }
}
