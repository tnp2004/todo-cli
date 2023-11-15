use crate::{error::Error, status::Status};
use crate::utils::time;
use uuid::Uuid;
use csv::{Reader, StringRecord, Writer};

pub fn write(header_fields: &Vec<String>,task: &String, path: &String) -> Result<(), Error> {
    let prev_records = read(path)?;
    let mut writer = Writer::from_path(path)?;
    // header
    writer.write_record(header_fields)?;
    // rewrite previous records
    for record in prev_records {
        writer.write_record(&record)?;
    }
    let id = Uuid::new_v4();
    writer.write_record(&[id.to_string(), task.to_string(), Status::Holding.to_string(), time::current_time(), "no update".to_string()])?;
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
