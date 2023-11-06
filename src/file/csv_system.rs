use crate::error::Error;
use csv::{Writer, Reader, StringRecord};

pub fn write(task: &String, path: &String) -> Result<(), Error> {
    let prev_records = read(path)?;
    let mut writer = Writer::from_path(path)?;
    // header
    writer.write_record(&["task", "done"])?;
    // rewrite previous records
    for record in prev_records {
        writer.write_record(&record)?;
    }
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
    }

    Ok(records)
}