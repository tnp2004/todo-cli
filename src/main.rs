use todo_cli::command::Todo;

fn main() {
    let todo = Todo::init();
    if let Err(e) = todo.run() {
        println!("{}", e);
    }
}
