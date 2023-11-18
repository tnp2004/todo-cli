use crate::error::Error;
use crate::file::csv_system;
use crate::todo::Todo;
use crate::status::Status;
use crate::Result;

pub trait Action {
    fn add(&self, task: &String, path: &String) -> Result<()>;
    fn remove(&self, number: i32, path: &String) -> Result<()>;
    fn show(&self, path: &String) -> Result<()>;
    fn update_status(&self, number: i32, status: &String, path: &String) -> Result<()>;
    fn export(&self, path: &String, export_path: &String) -> Result<()>;
}

impl Action for Todo {
    fn add(&self, task: &String, path: &String) -> Result<()> {
        csv_system::write(&self.header_fields, task, path)?;
        println!("Add: {}", task);

        Ok(())
    }

    fn remove(&self, n: i32, path: &String) -> Result<()> {
        csv_system::delete(&self.header_fields, n, path)?;
        println!("Remove: {}", n);

        Ok(())
    }

    fn show(&self, path: &String) -> Result<()> {
        let records = csv_system::read(path)?;
        print!("    ");
        println!("{} {}", self.header_fields[1], self.header_fields[2]);
        let mut counter = 1;
        for record in records {
            print!("{}: ", counter);
            print!(" {} {}", record[1].to_string(), record[2].to_string());
            counter += 1;
            print!("\n")
        }

        Ok(())
    }

    fn update_status(&self, number: i32, status: &String, path: &String) -> Result<()> {
        // check and convert status to enum
        let stat = match status as &str {
            "holding" => Status::Holding,
            "implement" => Status::Implement,
            "finished" => Status::Finished,
            "cancel" => Status::Cancelled,
            _ => return Err(Error::StatusNotFound),
        };

        csv_system::update_status(&self.header_fields, number, stat, path)?;
        println!("Update status: {}", status);

        Ok(())
    }

    fn export(&self, path: &String, export_path: &String) -> Result<()> {
        csv_system::export(&self.header_fields, path, export_path)?;
        println!("Export csv file to {}", export_path);

        Ok(())
    }
    
}