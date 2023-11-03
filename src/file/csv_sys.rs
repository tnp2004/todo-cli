use csv::Writer;

struct TodoHeader {
    task: String,
    done: bool,
}

pub fn write(task: &String, path: &String) -> Result<(), Box<csv::Error>> {
    let writer_result = Writer::from_path(path);
    let mut writer = match writer_result {
        Ok(writer) => writer,
        Err(err) => return Err(Box::new(err)),
    };
    writer.write_record(&["task", "done"])?;

    match writer.write_record(&[task.to_string(), "false".to_string()]) {
        Ok(record) => record,
        Err(err) => return Err(Box::new(err)),
    };

    Ok(())
}
