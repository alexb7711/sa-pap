//===============================================================================
// Import modules

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
    pub schedule: Vec<Vec<Assignment>>, // Lists of scheduled charge times
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
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    pub fn new(q: Option<usize>) -> Charger {
        // Extract the number of queues
        let q = q.unwrap_or(1 as usize);

        // Create a charger
        let mut c: Charger = Charger {
            schedule: Vec::new(),
        };

        // Create the number of queues specified
        c.add_chargers(q);

        // println!("{:?}", c.schedule);

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
        for _ in 0..q {
            self.schedule.push(Vec::new())
        }
    }
}
