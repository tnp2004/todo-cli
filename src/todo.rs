use clap::{arg, command, Arg, Command};

use crate::error::Error;
use crate::file::csv_system;
use crate::{action::Action, Result};
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
    fn add(&self, task: &String, path: &String) -> Result<()> {
        csv_system::write(task, path)?;
        println!("Add: {}", task);

        Ok(())
    }

    fn remove(&self, task: &String, path: &String) -> Result<()> {
        println!("Remove: {}", task);

        Ok(())
    }

    fn show(&self, path: &String) -> Result<()> {
        let records = csv_system::read(path)?;
        for record in records {
            for (i, field) in record.iter().enumerate() {
                println!("{}: {}", i, field);
            }
        }

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
            // Show
            .subcommand(
                Command::new("show")
                    .about("Show all tasks")
                    .arg(Arg::new("path").short('p')),
            )
            .get_matches();

        let action = match_result.subcommand_name();
        let task = match_result
            .subcommand_matches(action.unwrap())
            .unwrap()
            .get_one::<String>("task")
            .unwrap();
        let path = match_result
            .subcommand_matches(action.unwrap())
            .unwrap()
            .get_one::<String>("p")
            .unwrap();

        match action.unwrap() {
            "add" => self.add(task, path),
            "remove" => self.remove(task, path),
            "show" => self.show(&"todo_test.csv".to_string()),
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
    }
}
