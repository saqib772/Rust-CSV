use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;



    let mut record_count = 0;
    for result in reader.records() {
        let _ = result?; // Ignore the actual record for now
        record_count += 1;
    }

    println!("Total records: {}", record_count);

    


    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("user.csv"){
        eprintln!("{}", e);
    }
}