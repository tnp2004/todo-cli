use todo_cli::{command::Todo, config::{self}};

fn main() {
    let mut cfg = config::Cfg::init("Config".to_string());
    if let Err(e) = cfg.load_config() {
        println!("{}", e);
    }

    let todo = Todo::init(cfg);
    if let Err(e) = todo.run() {
        println!("{}", e);
    }
}
