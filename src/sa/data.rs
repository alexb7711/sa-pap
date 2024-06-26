#![allow(non_snake_case)]

//=========================================================================
// Import Crates

//=========================================================================
// Import modules

//===============================================================================
/// Structure for route data
///
/// Defines the structure that contains the metadata to generate the start/stop routes from the csv file.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Data {
    pub param: Parameter,
    pub dec: DecisionVar,
}

//===============================================================================
/// Structure for route parameters
///
/// Defines the structure that contains the buffers for the SA parameters
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Parameter {
    pub A: usize,        // Number of buses
    pub D: Vec<f32>,     // Discharge of route i
    pub Gam: Vec<u16>,   // Array of visit IDs
    pub K: u16,          // Discrete number of steps in T
    pub N: usize,        // Number of total visits
    pub Q: usize,        // Number of chargers
    pub S: usize,        // Length of a single bus
    pub T: f32,          // Time horizon                                   [hr]
    pub a: Vec<f32>,     // Arrival time of bus visit i                    [hr]
    pub alpha: Vec<f32>, // Initial charge percentage for bus a            [%]
    pub beta: Vec<f32>,  // Final charge percentage for bus a at T         [%]
    pub dt: f32,         // Discrete time step                             [hr]
    pub e: Vec<f32>,     // Exit time for bus visit i                      [hr]
    pub ep: Vec<f32>,    // (epsilon) Cost of using charger q per unit time
    pub fast: usize,     // Number of fast chargers
    pub gam: Vec<i16>,   // Array of values indicating the next index for bus i
    pub k: Vec<f32>,     // (kappa) Battery capacity for bus i             [MJ]
    pub l: Vec<f32>,     // (lambda) Discharge of bus visit over route i
    pub model: String,   // Linear/non-linear battery dynamics model
    pub nu: f32,         // Minimum charge allowed on departure of visit i [%]
    pub r: Vec<f32>,     // Charge rate for charger q                      [KWh]
    pub conv: Vec<f32>,  // Convergence rate for charger q
    pub slow: usize,     // Number of slow chargers
    pub tk: Vec<f32>,    // Array of discrete times                        [hr]
    pub ts: f32,         // Amount of offset applied                       [hr]
    pub zeta: Vec<f32>,  // Discharge rate of bus b
}

//===============================================================================
/// Structure for route decision variables
///
/// Defines the structure that contains the buffers for the decision variables
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DecisionVar {
    pub Jb: Vec<f64>,          //  Best objective function score
    pub Jc: Vec<f64>,          //  Current objective function score
    pub Jn: Vec<f64>,          //  New objective function score
    pub d: Vec<f32>,           //  Detach time for visit i                [hr]
    pub g: Vec<Vec<f32>>,      //  Detach time for visit i                [hr]
    pub psi: Vec<Vec<bool>>,   //  Determines if i is "fully left" of j
    pub eta: Vec<f32>,         //  Initial charge for bus visit i         [MJ]
    pub s: Vec<f32>,           //  Time to charge for bus visit i         [hr]
    pub sigma: Vec<Vec<bool>>, //  Determines if i is "fully below" j
    pub u: Vec<f32>,           //  Initial charge time for visit i        [hr]
    pub v: Vec<usize>,         //  Assigned queue for visit i
    pub w: Vec<Vec<bool>>,     //  Matrix representation of bus charger assignments
}
