use clap::{arg, command, Arg, ArgMatches, Command};

use crate::error::Error;
use crate::file::csv_system;
use crate::{action::Action, action::ArgParser, Result};
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

impl ArgParser for ArgMatches {
    fn parse_arg(&self, arg_name: &String) -> Result<&String> {
        let arg_value = self.get_one::<String>(arg_name).unwrap();

        Ok(arg_value)
    }

    fn parse_sub_arg(&self, action: &String, arg_name: &String) -> Result<&String> {
        let arg_flag_value = self
            .subcommand_matches(action)
            .unwrap()
            .get_one::<String>(arg_name)
            .unwrap();

        Ok(arg_flag_value)
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
                    .about("Insert a new task")
                    .arg(arg!([task]))
                    .arg(Arg::new("path").short('p')),
            )
            // Remove
            .subcommand(
                Command::new("remove")
                    .about("Delete a task")
                    .arg(arg!([task]))
                    .arg(Arg::new("path").short('p')),
            )
            // Show
            .subcommand(
                Command::new("show")
                    .about("Print all tasks")
                    .arg(Arg::new("path").short('p')),
            )
            .get_matches();

        let action = match_result.subcommand_name();
        match action {
            Some("add") => {
                let task = match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"task".to_string())?;
                let path = match_result.parse_sub_arg(&"add".to_string(), &"path".to_string())?;

                self.add(task, path)
            }
            Some("remove") => {
                let task = match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"task".to_string())?;
                let path =
                    match_result.parse_sub_arg(&"remove".to_string(), &"path".to_string())?;

                self.remove(task, path)
            }
            Some("show") => {
                let path = match_result.parse_sub_arg(&"show".to_string(), &"path".to_string())?;

                self.show(path)
            }
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
    }
}
