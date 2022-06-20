// Public Crates
extern crate yaml_rust;

// Import Crates
use yaml_rust::Yaml;

// My modules

// Import modules
use crate::fileio::yaml_loader;

//===============================================================================
// Structure for ScheduleGenerator
pub struct RouteGenerator
{
    // pub config: Vec<Yaml>,
    pub config: Yaml,
}

//===============================================================================
// Implementation of ScheduleGenerator
impl RouteGenerator
{
    //===========================================================================
    // PUBLIC

    //---------------------------------------------------------------------------
    /// Returns a schedule generator
    ///
    /// # Input
    /// * `config_path`: Path to YAML schedule config
    ///
    /// # Output
    /// * `ScheduleGenerator`
    ///
    pub fn new(config_path: &str) -> RouteGenerator
    {
        // Create new RouteGenerator
        let rg = RouteGenerator
        {
            config: yaml_loader::load_yaml(config_path),
        };

        // Return Route Generator
        return rg;
    }

    //===========================================================================
    // PRIVATE

}
