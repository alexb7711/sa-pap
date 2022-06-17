// Public Crates
extern crate yaml_rust;

// Import Crates
use std::fs::File;
use std::io::Read;
use yaml_rust::{YamlLoader, Yaml};

//===============================================================================
// Structure for ScheduleGenerator
#[allow(dead_code)]
pub struct RouteGenerator
{
    pub config: Vec<Yaml>,
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
            config: RouteGenerator::load_yaml(config_path),
        };

        // Return Route Generator
        return rg;
    }

    //===========================================================================
    // PRIVATE

    //---------------------------------------------------------------------------
    /// Returns a `String` from a specified file
    ///
    /// # Input
    /// * `file_path`: Path to YAML schedule config
    ///
    /// # Output
    /// * `text`: String object with text from `file_path`
    ///
    fn create_yaml_string(file_path: &str) -> Result<String, std::io::Error>
    {
        // Set up file handler and empty buffer
        let mut file = File::open(file_path)?;
        let mut text = String::new();

        // Read file into text as a string
        file.read_to_string(&mut text)?;

        return Ok(text);
    }

    //---------------------------------------------------------------------------
    /// Creates an object from YAML file
    ///
    /// # Input
    /// * `config_path`: Path to YAML schedule config
    ///
    /// # Output
    /// * `yaml`: Vec<Yaml> object
    ///
    fn load_yaml(config_path: &str) -> Vec<Yaml>
    {
        // Parse file as String
        let text: std::io::Result<String> = RouteGenerator::create_yaml_string(&config_path);

        // Parse YAML
        let yaml = match text
        {
            Ok(text)   => YamlLoader::load_from_str(&text).unwrap(),
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        return yaml;
    }
}
