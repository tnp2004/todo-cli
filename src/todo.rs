use clap::{arg, command, Command};

use crate::{action::Action, Result};
use crate::file::csv_system;
use crate::error::Error;
pub struct Todo {
    pub header_fields: Vec<String>,
}
// struct TodoHeader {
//     task: String,
//     done: bool,
//     created_at: String,
//     updated_at: String,
// }

impl Action for Todo {
    fn add(&self, task: String) -> Result<()> {
        csv_system::write(&task, &"todo_test.csv".to_string())?;
        println!("Add: {}", task);

        Ok(())
    }

    fn remove(&self, task: String) -> Result<()> {
        println!("Remove: {}", task);

        Ok(())
    }
}

impl Todo {
    pub fn init() -> Self {
        Self {
            header_fields: vec!["task".to_string(), "done".to_string()],
        }
    }

    pub fn run(&self) -> Result<()> {
        let match_result = command!()
            // Add
            .subcommand(
                Command::new("add")
                    .arg(arg!([task]))
                    .about("Insert a new task"),
            )
            // Remove
            .subcommand(
                Command::new("remove")
                    .arg(arg!([task]))
                    .about("Delete a task"),
            )
            .get_matches();

        let action = match_result.subcommand_name();
        let task = match_result.subcommand_matches(action.unwrap()).unwrap().get_one::<String>("task").unwrap().to_string();
        
        match action.unwrap() {
            "add" => self.add(task),
            "remove" => self.remove(task),
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
        
    }
}
