use std::error::Error;
use std::io;
// use std::process;

pub fn read() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // Iterator yields Result<StringRecord, Error>
        // so check the error here
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}