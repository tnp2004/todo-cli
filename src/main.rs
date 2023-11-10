use todo_cli::{command::Todo, config::{self, TConfig}};

fn main() {
    let todo = Todo::init();
    config::Cfg::init().set_path("yolol123".to_string());
    config::config();
    if let Err(e) = todo.run() {
        println!("{}", e);
    }
}
