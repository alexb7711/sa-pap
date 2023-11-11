#[allow(rustdoc::invalid_html_tags)]

/// module: yaml_loader
///
/// Description:
/// This module is used to load in a YAML file and parse it.
///
pub mod yaml_loader {
    //---------------------------------------------------------------------------
    // Public Crates
    extern crate yaml_rust;

    //---------------------------------------------------------------------------
    // Import Crates
    use std::fs::File;
    use std::io::Read;
    use yaml_rust::{Yaml, YamlLoader};

    //---------------------------------------------------------------------------
    /// Returns a `String` from a specified file
    ///
    /// # Input
    /// * `file_path`: Path to YAML schedule config
    ///
    /// # Output
    /// * `text`: String object with text from `file_path`
    ///
    fn create_yaml_string(file_path: &str) -> Result<String, std::io::Error> {
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
    pub fn load_yaml(config_path: &str) -> Yaml {
        // Parse file as String
        let text: std::io::Result<String> =
            crate::util::fileio::yaml_loader::create_yaml_string(&config_path);

        // Parse YAML
        let yaml = match text {
            Ok(text) => YamlLoader::load_from_str(&text).unwrap(),
            Err(error) => panic!(
                "Problem opening the file: {:?}\nError: {:?}:",
                config_path, error
            ),
        };

        return yaml[0].clone();
    }
}
