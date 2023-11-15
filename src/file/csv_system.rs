use std::f32::consts::E;

use crate::{error::Error, status};
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
    writer.write_record(&[id.to_string(), task.to_string(), status::Status::Holding.to_string(), time::current_time(), "no update".to_string()])?;
    writer.flush()?;

    Ok(())
}

pub fn delete(header_fields: &Vec<String>, n: i32, path: &String) -> Result<(), Error> {
    let prev_records = read(path)?;
    let mut writer = Writer::from_path(path)?;
    // header
    writer.write_record(header_fields)?;
    // rewrite previous records
    for (i, record) in prev_records.iter().enumerate() {
        
        // skip the record to be deleted
        if i as i32 + 1 == n {
            continue;
        }

        writer.write_record(record)?;
    }
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

pub fn update_status(header_fields: &Vec<String>, n: i32, status: status::Status, path: &String) -> Result<(), Error> {
    let prev_records = read(path)?;
    let mut writer = Writer::from_path(path)?;
    // header
    writer.write_record(header_fields)?;
    // rewrite previous records
    for (i, record) in prev_records.iter().enumerate() {
        
        // update specific record
        if i as i32 + 1 == n {
            let new_status = if status::is_ok(&status.to_string()) {
                status.to_string()
            } else {
                return Err(Error::StatusNotFound);
            };

            writer.write_record(&[record[0].to_string(), record[1].to_string(), new_status, record[3].to_string(), time::current_time()])?;
            continue;
        }

        writer.write_record(record)?;
    }
    writer.flush()?;

    Ok(())
}