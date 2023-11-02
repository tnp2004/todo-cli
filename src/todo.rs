use crate::action::Action;
use clap::{arg, command, Command};
pub struct Todo;

impl Action for Todo {
    fn add(&self, task: String) {
        println!("Add: {}", task);
    }

    fn remove(&self, task: String) {
        println!("Remove: {}", task);
    }
}

impl Todo {
    pub fn run(&self) {
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

        match match_result
            .subcommand_matches(action.unwrap())
            .unwrap()
            .get_one::<String>("task")
        {
            Some(task) => match action {
                Some("add") => self.add(task.to_string()),
                Some("remove") => self.remove(task.to_string()),
                _ => {
                    panic!("Command not found");
                }
            },
            None => panic!("Task not found"),
        };
    }
}
