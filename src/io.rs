use std::fs;
use csv;

#[allow(dead_code)]
pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents.split("\n").map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn read_from_csv(path: &str) -> Vec<Vec<f64>> {
    // Creates a new csv 'Reader' from a file
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(path).expect("Failed to open CSV!");
    // Retrieve and print header record
    match reader.headers(){
        Ok(header) => {
            println!("Header: {:?}", header);
        }
        Err(_) => {
            println!("No Header found...")
        }
    }
    let mut contents = vec![];
    for result in reader.records() {
        let record = result.unwrap();
        //println!("{:?}", record);
        let mut row = vec![];
        for cell in &record {
             match cell.parse::<f64>() {
                 Ok(value) => {
                     row.push(value);
                 }
                 Err(err) => {
                     println!("Error {err} parsing value {row:?}");
                 }
             }
        }
        contents.push(row);
    }
    contents
}