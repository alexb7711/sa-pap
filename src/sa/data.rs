//=========================================================================
// Import Crates

//=========================================================================
// Import modules

//===============================================================================
/// Structure for route data
///
/// Defines the structure that contains the metadata to generate the start/stop routes from the csv file.
///
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Data {
    pub param: Parameter,
    pub dec: DecisionVar
}

//===============================================================================
/// Structure for route parameters
///
/// Defines the structure that contains the buffers for the SA parameters
///
struct Parameters {
    pub A     : Vec<u16>,                                        // Number of buses
    pub D     : Vec<f32>,                                        // Discharge of route i
    pub G     : Vec<u16>,                                        // Array of visit IDs
    pub K     : u16,                                             // Discrete number of steps in T
    pub N     : Vec<u16>,                                        // Number of total visits
    pub Q     : Vec<u16>,                                        // Number of chargers
    pub S     : u16,                                             // Length of a single charger
    pub T     : u16,                                             // Time horizon                                   [hr]
    pub a     : Vec<f32>,                                        // Arrival time of bus visit i                    [hr]
    pub alpha : f32,                                             // Initial charge percentage for bus a            [%]
    pub beta  : f32,                                             // Final charge percentage for bus a at T         [%]
    pub dt    : f32,                                             // Discrete time step                             [hr]
    pub e     : None,                                            // Exit time for bus visit i                      [hr]
    pub ep    : Vec<f32>,                                        // (epsilon) Cost of using charger q per unit time
    pub fast  : u16,                                             // Number of fast chargers
    pub g     : Vec<u16>,                                        // Array of values indicating the next index for bus i
    pub k     : Vec<f32>,                                        // (kappa) Battery capacity for bus i             [MJ]
    pub l     : Vec<f32>,                                        // (lambda) Discharge of bus visit over route i
    pub m     : Vec<f32>,                                        // Cost of bus i being assigned to charger q
    pub nu    : Vec<f32>,                                        // Minimum charge allowed on departure of visit i [%]
    pub r     : Vec<f32>,                                        // Charge rate for charger q                      [KWh]
    pub s     : u16,                                             // Length of a bus
    pub slow  : u16,                                             // Number of slow chargers
    pub tk    : Vec<f32>,                                        // Array of discrete times                        [hr]
 }

//===============================================================================
/// Structure for route decision variables
///
/// Defines the structure that contains the buffers for the decision variables
///
struct DecisionVar {
    pub c     : Vec<f32>,                                        //  Detach time for visit i                [hr]
    pub psi : Vec<Vec<f32>>,                                     //  Determines if i is "fully left" of j
    pub eta   : Vec<f32>,                                        //  Initial charge for bus visit i         [MJ]
    pub s     : Vec<f32>,                                        //  Time to charge for bus visit i         [hr]
    pub sigma : None,                                            //  Determines if i is "fully below" j
    pub u     : None,                                            //  Initial charge time for visit i        [hr]
    pub v     : None,                                            //  Assigned queue for visit i
    pub w     : None,                                            //  Matrix representation of bus charger assignments
}
