use std::fs::File;
use crate::{error::Error, status};
use crate::utils::{time, self};
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

pub fn export(header_fields: &Vec<String>,path: &String, export_path: &String) -> Result<(), Error> {
    let mut reader = Reader::from_path(path)?;
    let filename = format!("{}/Export-{}",export_path, utils::filename::get_file_name());
    File::create(&filename)?;
    let mut writer = Writer::from_path(filename)?;
     // header
     writer.write_record(&[header_fields[1].clone(), header_fields[2].clone()])?;
    for result in reader.records() {
        let record = result?;
        writer.write_record(&[record[1].to_string(), record[2].to_string()])?;
    }
    writer.flush()?;

    Ok(())
}

pub fn import(header_fields: &Vec<String>, import_path: &String, output_path: &String) -> Result<(), Error> {
    let import_records = match read(import_path) {
        Ok(recs) => recs,
        Err(_) => Err(Error::ReadFileError)?,
    };
    let filename = format!("{}/Import-{}",output_path, utils::filename::get_file_name());
    File::create(&filename)?;
    let mut writer = Writer::from_path(filename)?;
    
    writer.write_record(header_fields)?;
    let id = Uuid::new_v4();

    for record in import_records {
        writer.write_record(&[id.to_string(), record[1].to_string(), record[2].to_string(), time::current_time(), "no update".to_string()])?;
    }
    writer.flush()?;

    Ok(())
}