use crate::config::{Cfg, TConfig};
use crate::error::Error;
use crate::file::csv_system;
use crate::status::Status;
use crate::{action::Action, action::ArgParser, Result};
use clap::{arg, command, Arg, ArgMatches, Command};

pub struct Todo {
    pub header_fields: Vec<String>,
    pub config: Cfg,
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
            // Set default path
            .subcommand(
                Command::new("setpath")
                    .about("Set default path")
                    .arg(arg!([path])),
            )
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
            // Update status
            .subcommand(
                Command::new("update")
                    .about("Update status of a task")
                    .arg(arg!([number]))
                    .arg(arg!([status])),
            )
            // Show
            .subcommand(Command::new("show").about("Print all tasks"))
             // Export csv file
             .subcommand(
                Command::new("export")
                    .about("Export csv file")
                    .arg(arg!([path]))
            )
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

            // UPDATE STATUS
            Some("update") => {
                let path = match match_result.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => self.config.get_path(),
                };
                let n = match match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"number".to_string())
                {
                    Some(n) => n.parse::<i32>().unwrap(),
                    None => return Err(Error::TaskNotFound),
                };
                let status = match match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"status".to_string())
                {
                    Some(stat) => stat,
                    None => return Err(Error::StatusNotFound),
                };

                self.update_status(n, status, path)
            }

            Some("setpath") => {
                let path = match match_result.parse_sub_arg(&action.unwrap().to_string(),&"path".to_string()) {
                    Some(path) => path,
                    None => return Err(Error::ArgumentNotFound),
                };

                self.config.set_path(path.to_string())?;

                Ok(())
            }

            Some("export") => {
                let path = match match_result.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => self.config.get_path(),
                };
                let export_path = match match_result.parse_sub_arg(&action.unwrap().to_string(),&"path".to_string()) {
                    Some(path) => path,
                    None => return Err(Error::ArgumentNotFound),
                };

                println!("Exporting csv file to {}", export_path);

                Ok(())
            }
            
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
    }
}
