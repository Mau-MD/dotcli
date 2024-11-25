use std::env;

pub mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a command");
        return;
    }
    command::command::execute(&args[1], args.clone());
}
