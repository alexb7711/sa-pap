//=========================================================================
// Import Crates
use csv::ReaderBuilder;
use yaml_rust::Yaml;

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
pub fn parse_csv(
    csv_h: &mut csv::Reader<std::fs::File>,
    config: &Yaml,
) -> (Vec<u16>, Vec<Vec<f32>>) {
    // Stores the route data
    let mut routes: (Vec<u16>, Vec<Vec<f32>>) = (Vec::new(), Vec::from(Vec::new()));
    let mut b: u16 = 0;
    let ignore: Vec<usize> = config["ignore"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap() as usize)
        .collect();

    // Loop through each row in the CSV file
    for result in csv_h.records() {
        // Reset the ith route vector
        let mut route_i: Vec<f32> = Vec::new();

        // Unpack the row if possible
        let r = match result {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        };

        // Check if the ID is in the ignore list
        let id: u16 = r[0].parse::<u16>().unwrap();

        // If the id is in the ignore list, don't include the route data
        if ignore.contains(&(id as usize)) {
            continue;
        }

        // Append the ID
        routes.0.push(b);
        b += 1;

        // Append the routes, skip the first element (queue index)
        for s in r.iter().skip(1) {
            // Convert the jth variable to float
            let f: f32 = s.trim().parse::<f32>().unwrap();

            // Append the float to the ith route vector
            route_i.push(f);
        }

        // Append route vector
        routes.1.push(route_i);
    }

    return routes;
}

//===========================================================================
// Private
