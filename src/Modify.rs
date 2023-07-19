use std::error::Error;
use csv;
use csv::{Reader, Writer};

fn read_and_write_modified_data(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(input_path)?;
    let mut writer = Writer::from_path(output_path)?;

    for result in reader.records() {
        let record = result?;
        let mut modified_record = record.clone();
        if let Some(name) = modified_record.get_mut(1) {
            *name = name.to_string().to_uppercase(); // Capitalize the name
        }
        writer.write_record(&modified_record)?;
    }

    writer.flush()?;
    Ok(())
}

fn main() {
    if let Err(e) = read_and_write_modified_data("user.csv", "modified_user.csv") {
        eprintln!("{}", e);
    }
}
