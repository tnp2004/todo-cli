use crate::config::{Cfg, TConfig};
use crate::error::Error;
use crate::file::csv_system;
use crate::{action::Action, action::ArgParser, Result};
use clap::{arg, command, Arg, ArgMatches, Command};

pub struct Todo {
    pub header_fields: Vec<String>,
    pub config: Cfg,
}

impl Action for Todo {
    fn add(&self, task: &String, path: &String) -> Result<()> {
        csv_system::write(&self.header_fields,task, path)?;
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

    fn update_status(&self, number: i32, status: crate::status::Status, path: &String) -> std::prelude::v1::Result<(), Error> {
        csv_system::update_status(&self.header_fields, number, status, path)?;
        println!("Update status: {}", number);

        Ok(())
    }

    
}

impl ArgParser for ArgMatches {
    fn parse_arg(&self, arg_name: &String) -> Option<&String> {
        match self.get_one::<String>(arg_name) {
            Some(arg_value) => Some(arg_value),
            None => None,
        }
    }

    fn parse_sub_arg(&self, action: &String, arg_name: &String) -> Option<&String> {
        match self
            .subcommand_matches(action)
            .unwrap()
            .get_one::<String>(arg_name)
        {
            Some(flag_arg_value) => Some(flag_arg_value),
            None => None,
        }
    }
}

impl Todo {
    pub fn init(cfg: Cfg) -> Self {
        Self {
            config: cfg,
            header_fields: vec![
                "id".to_string(),
                "task".to_string(),
                "status".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ],
        }
    }

    pub fn run(&self) -> Result<()> {
        let match_result = command!()
            // Path argument
            .arg(Arg::new("path").short('p'))
            // Add
            .subcommand(
                Command::new("add")
                    .about("Insert a new task")
                    .arg(arg!([task])),
            )
            // Remove
            .subcommand(
                Command::new("remove")
                    .about("Delete a task")
                    .arg(arg!([number])),
            )
            // Show
            .subcommand(Command::new("show").about("Print all tasks"))
            .get_matches();

        let action = match_result.subcommand_name();
        match action {
            // ADD
            Some("add") => {
                let task = match match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"task".to_string())
                {
                    Some(task) => task,
                    None => return Err(Error::TaskNotFound),
                };

                let path = match match_result.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => self.config.get_path(),
                };

                self.add(task, path)
            }

            // REMOVE
            Some("remove") => {
                let number = match match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"number".to_string())
                {
                    Some(n) => n.parse::<i32>().unwrap(),
                    None => return Err(Error::TaskNotFound),
                };

                let path = match match_result.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => self.config.get_path(),
                };

                self.remove(number, path)
            }

            // SHOW
            Some("show") => {
                let path = match match_result.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    // default path
                    None => self.config.get_path(),
                };

                self.show(path)
            }
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
    }
}
