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
/// * `routes`: Tuple of that contains the vector of bus IDs and vector of routes
///
// BUG: https://levelup.gitconnected.com/working-with-csv-data-in-rust-7258163252f8
//      The deserialize is part of SERDE which needs some extra goodies to make it work
pub fn parse_csv(csv_h: &mut csv::Reader<std::fs::File>) -> (Vec<u16>, Vec<Vec<u16>>) {
    // Variables
    let mut routes: (Vec<u16>, Vec<Vec<u16>>) = (Vec::new(), Vec::from(Vec::new()));

    // Executable code

    let mut iter = csv_h.deserialize();

    // Loop through each row in the CSV file
    for result in iter.next() {
        // Unpack the row if possible
        let r: Vec<u16> = match result {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        };

        // Append the ID
        routes.0.push(r[0]);

        // Append the routes
        routes.1.push(r[1..].to_vec());
    }

    return routes;
}

//===========================================================================
// Private
