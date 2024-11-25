use crate::command::path::PathCommand;

use super::alias::AliasCommand;

pub trait Command {
    fn init(&mut self, args: Vec<String>) -> Result<(), String>;
    fn execute(&self) -> Result<(), String>;
}

pub fn execute(command: &str, args: Vec<String>) {
    let mut command = command_factory(command);
    if let Err(e) = run_command(&mut command, args) {
        panic!("{}", e);
    }
}

fn run_command(command: &mut Box<dyn Command>, args: Vec<String>) -> Result<(), String> {
    command
        .init(args)
        .map_err(|e| format!("Error initializing command: {}", e))?;
    command
        .execute()
        .map_err(|e| format!("Error executing command: {}", e))
}

fn command_factory(command: &str) -> Box<dyn Command> {
    match command {
        "path" => Box::new(PathCommand::new()),
        "alias" => Box::new(AliasCommand::new()),
        _ => panic!("Invalid command"),
    }
}
