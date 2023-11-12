use clap::{arg, command, Arg, ArgMatches, Command};
use crate::config::{Cfg, TConfig};
use crate::error::Error;
use crate::file::csv_system;
use crate::{action::Action, action::ArgParser, Result};

pub struct Todo {
    pub header_fields: Vec<String>,
    pub config: Cfg
}

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
        print!("    ");
        self.header_fields
            .iter()
            .for_each(|field| print!("{} ", field));
        print!("\n");
        let mut counter = 1;
        for record in records {
            print!("{}: ", counter);
            for field in record.iter() {
                print!(" {}", field);
            }
            counter += 1;
            print!("\n")
        }

        Ok(())
    }
}

impl ArgParser for ArgMatches {
    fn parse_arg(&self, arg_name: &String) -> Result<Option<&String>> {
        let arg_value = self.get_one::<String>(arg_name);
      
        if arg_value.is_none() {
            return Err(Error::ArgumentNotFound);
        }

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
    pub fn init(cfg: Cfg) -> Self {
        Self {
            config: cfg,
            header_fields: vec!["task".to_string(), "done".to_string()],
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
                    .arg(arg!([task])),
            )
            // Show
            .subcommand(Command::new("show").about("Print all tasks"))
            .get_matches();

        let action = match_result.subcommand_name();
        match action {
            Some("add") => {
                let task = match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"task".to_string())?;
                let path = match_result.parse_arg(&"path".to_string())?;

                self.add(task, path.unwrap())
            }
            Some("remove") => {
                let task = match_result
                    .parse_sub_arg(&action.unwrap().to_string(), &"task".to_string())?;
                let path = match_result.parse_arg(&"path".to_string())?;

                self.remove(task, path.unwrap())
            }
            Some("show") => {
                match match_result.parse_arg(&"path".to_string()) {
                    Ok(path) => self.show(path.unwrap()),
                    Err(_) => {
                        let path = self.config.get_path();
                        self.show(path)
                    }
                }
            }
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
    }
}
