use crate::error::Error;
use csv::{Writer, Reader, StringRecord};

pub fn write(task: &String, path: &String) -> Result<(), Error> {
    let mut writer = Writer::from_path(path)?;
    writer.write_record(&["task", "done"])?;
    writer.write_record(&[task.to_string(), "false".to_string()])?;
    writer.flush()?;

    Ok(())
}

pub fn read(path: &String) -> Result<Vec<StringRecord>, Error> {
    let mut reader = Reader::from_path(path)?;
    let mut records: Vec<StringRecord> = Vec::new();
    for result in reader.records() {
        let record = result?;
        records.push(record);
        // get record
        // for (i, field) in record.iter().enumerate() {
        //     println!("{}: {}", i, field);
        // }
    }

    Ok(records)
}