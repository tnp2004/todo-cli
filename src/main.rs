use todo_cli::{command::Todo, config::{self, TConfig}};

fn main() {
    let todo = Todo::init();
    if let Err(e) = todo.run() {
        println!("{}", e);
    }

    let mut cfg = config::Cfg::init();
    cfg.load_config();
    println!("{}", cfg.get_path());
}
