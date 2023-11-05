
use clap::{arg, command, Arg, Command, ArgMatches};

use crate::error::Error;
use crate::file::csv_system;
use crate::{action::Action, action::ValueParser, Result};
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

    fn remove(&self, task: &String, _path: &String) -> Result<()> {
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

// impl ValueParser for ArgMatches {
//     fn parse_arg(&self, action: &String, value: &String, path: Option<&String>) -> Result<&String> {
//         Ok(self.get_one::<String>(value).unwrap())
//     }

//     fn parse_arg_flag(&self, action: &String, value: &String, path: Option<&String>) -> Result<()> {
//         todo!()
//     }
// }

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
                    .about("Print all tasks")
                    .arg(Arg::new("path").short('p')),
            )
            .get_matches();

        let action = match_result.subcommand_name().unwrap();
        println!("{}", action);

        match match_result.subcommand_name() {
            Some("add") => {
                let task = match_result.get_one::<String>("task").unwrap();
                let path = match_result
                    .subcommand_matches(action)
                    .unwrap()
                    .get_one::<String>("path")
                    .unwrap();

                self.add(task, path)
            }
            Some("remove") => {
                let task = match_result.get_one::<String>("task").unwrap();
                let path = match_result
                    .subcommand_matches(action)
                    .unwrap()
                    .get_one::<String>("path")
                    .unwrap();

                self.remove(task, path)
            }
            Some("show") => {
                let path = match_result
                    .subcommand_matches(action)
                    .unwrap()
                    .get_one::<String>("path")
                    .unwrap();
                self.show(path)
            }
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
    }
}
