use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    //Calculating Statistics:
    let mut total_salary = 0;
    let mut num_records = 0;

    for result in reader.records() {
        let record = result?;
        let salary: i32 = record[3].parse()?; // Assuming salary is in the fourth column
        total_salary += salary;
        num_records += 1;
    }

    if num_records > 0 {
        let average_salary = total_salary / num_records;
        println!("Average Salary: {}", average_salary);
    } else {
        println!("No data found in the CSV.");
    }


    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("user.csv"){
        eprintln!("{}", e);
    }
}