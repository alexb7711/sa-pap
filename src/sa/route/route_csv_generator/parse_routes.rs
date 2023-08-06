//=========================================================================
// Import Crates
use csv::ReaderBuilder;

//===============================================================================
// External Crates

//===========================================================================
// PUBLIC
//---------------------------------------------------------------------------
/// Returns a file handler to CSV file with routes
///
/// # Input
/// * `csv_path`: Path to CSV file
///
/// # Output
/// * `csv::Reader`
///
pub fn read_csv(csv_path: &str) -> csv::Reader<std::fs::File> {
    match ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .delimiter(b',')
        .from_path(csv_path)
    {
        Ok(reader) => return reader,
        Err(e) => panic!("{:?}", e),
    };
}

//---------------------------------------------------------------------------
/// Parses the CSV route file
///
/// # Input
/// * `csv_h`: File handler for the CSV file
///
/// # Output
/// *
///
pub fn parse_csv(_csv_h: csv::Reader<std::fs::File>) {}

//===========================================================================
// Private
