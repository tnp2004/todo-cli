use clap::{arg, command, Command};

pub struct Todo;

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

        println!("{:?}", match_result.subcommand_name().unwrap());
        println!(
            "{:?}",
            match_result
                .subcommand_matches("add")
                .unwrap()
                .get_one::<String>("task")
                .unwrap()
        )
    }
}
