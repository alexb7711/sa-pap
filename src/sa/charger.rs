//===============================================================================
// Import modules

//===============================================================================
/// Structure to consolidate the bus assignment information
///
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Assignment {
    t: (f32, f32),
    b: usize,
}

//===============================================================================
/// Structure to track charger information
//
#[allow(dead_code)]
pub struct Charger {
    schedule: Vec<Vec<Assignment>>, // Lists of scheduled charge times
}

//===============================================================================
/// Implementation of SA
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
    pub fn new() -> Charger {
        return Charger {
            schedule: Vec::new(),
        };
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
        if self.avail(&q, &c) {
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
        // Default to indicate that the time slice item was not removed
        let mut avail: bool = false;

        // Check if the time slice exists in the charger queue
        if let Some(_) = self.schedule[*q].iter().find(|s| s.t == *c) {
            // State that the item is being removed
            avail = true;
        };

        return avail;
    }
}
