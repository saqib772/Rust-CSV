use std::error::Error;
use csv::Reader;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        let location = &record[2];
        if location == "NYC" {
            println!("{:?}", record);
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("user.csv") {
        eprintln!("{}", e);
    }
}
