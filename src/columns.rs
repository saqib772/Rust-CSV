use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    

    // Extracting Specific Columns:
    for result in reader.records() {
        let record = result?;
        let id = &record[0];
        let name = &record[1];
        let location = &record[2];
        println!("ID: {}, Name: {}, Location: {}", id, name, location);
    }
    


    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("user.csv"){
        eprintln!("{}", e);
    }
}