use clap::{arg, command, Arg, ArgMatches, Command};
use crate::arg_parser::ArgParser;
use crate::Error;
use crate::Result;
use crate::config::TConfig;
use crate::todo::Todo;
use crate::action::Action;

pub trait Comd {
    fn match_command(&self, todo: &Todo) -> Result<()>;
}

pub fn read() -> ArgMatches {
    command!()
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
                .arg(arg!([path])),
        )
        .subcommand(
            Command::new("import")
                .about("Import csv file")
                .arg(arg!([path])),
        )
        .get_matches()
}

impl Comd for ArgMatches {
    fn match_command(&self, todo: &Todo) -> Result<()> {
        let action = self.subcommand_name();
        match action {
            // ADD
            Some("add") => {
                let task = match self
                    .parse_sub_arg(&action.unwrap().to_string(), &"task".to_string())
                {
                    Some(task) => task,
                    None => return Err(Error::TaskNotFound),
                };

                let path = match self.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => todo.config.get_path(),
                };

                todo.add(task, path)
            }

            // REMOVE
            Some("remove") => {
                let number = match self
                    .parse_sub_arg(&action.unwrap().to_string(), &"number".to_string())
                {
                    Some(n) => n.parse::<i32>().unwrap(),
                    None => return Err(Error::TaskNotFound),
                };

                let path = match self.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => todo.config.get_path(),
                };

                todo.remove(number, path)
            }

            // SHOW
            Some("show") => {
                let path = match self.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    // default path
                    None => todo.config.get_path(),
                };

                todo.show(path)
            }

            // UPDATE STATUS
            Some("update") => {
                let path = match self.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => todo.config.get_path(),
                };
                let n = match self
                    .parse_sub_arg(&action.unwrap().to_string(), &"number".to_string())
                {
                    Some(n) => n.parse::<i32>().unwrap(),
                    None => return Err(Error::TaskNotFound),
                };
                let status = match self
                    .parse_sub_arg(&action.unwrap().to_string(), &"status".to_string())
                {
                    Some(stat) => stat,
                    None => return Err(Error::StatusNotFound),
                };

                todo.update_status(n, status, path)
            }

            Some("setpath") => {
                let path = match self.parse_sub_arg(&action.unwrap().to_string(),&"path".to_string()) {
                    Some(path) => path,
                    None => return Err(Error::ArgumentNotFound),
                };

                todo.config.set_path(path.to_string())?;

                Ok(())
            }

            Some("export") => {
                let path = match self.parse_arg(&"path".to_string()) {
                    Some(path) => path,
                    None => todo.config.get_path(),
                };
                let export_path = match self.parse_sub_arg(&action.unwrap().to_string(),&"path".to_string()) {
                    Some(path) => path,
                    None => return Err(Error::ArgumentNotFound),
                };

                todo.export(path, export_path)?;

                Ok(())
            }

            Some("import") => {
                let import_path = match self.parse_sub_arg(&action.unwrap().to_string(),&"path".to_string()) {
                    Some(path) => path,
                    None => return Err(Error::ArgumentNotFound),
                };
                // let output_path = match self.parse_arg(&"path".to_string()) {
                //     Some(path) => path,
                //     None => todo.config.get_path(),
                // };

                todo.import(import_path, &".".to_string())?;

                Ok(())
            }
            
            _ => Err(Error::CommandNotFound),
        }?;

        Ok(())
    }
}