use crate::error::Error;
use csv::Writer;

pub fn write(task: &String, path: &String) -> Result<(), Error> {
    let mut writer = Writer::from_path(path)?;
    writer.write_record(&["task", "done"])?;
    writer.write_record(&[task.to_string(), "false".to_string()])?;
    writer.flush()?;

    Ok(())
}
