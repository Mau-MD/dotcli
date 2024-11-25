use std::{
    fmt::{self, Display},
    fs, io,
};

use crate::command::command::Command;

use super::shell::find_shell_config_path;

pub struct AliasCommand {
    args: Vec<String>,
    available_commands: Vec<&'static str>,
    shell_config_path: String,
    aliases: Vec<Alias>,
}

struct Alias {
    id: String,
    name: String,
    value: String,
}

impl Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} = {}", self.id, self.name, self.value)
    }
}

impl AliasCommand {
    pub fn new() -> Self {
        Self {
            args: vec![],
            available_commands: vec!["add", "list"],
            shell_config_path: "".to_string(),
            aliases: vec![],
        }
    }
}

impl Command for AliasCommand {
    fn init(&mut self, args: Vec<String>) -> Result<(), String> {
        self.args = args;
        self.shell_config_path = find_shell_config_path()?;
        self.aliases = self.find_existing_aliases()?;
        Ok(())
    }

    fn execute(&self) -> Result<(), String> {
        if self.args.len() < 3 {
            return Err(format!(
                "Invalid command, please use one of the following: {:?}",
                self.available_commands
            ));
        }

        match self.args[2].as_str() {
            "add" => self.execute_add().map_err(|e| e.to_string()).map(|_| ()),
            "list" => self.execute_list(),
            _ => Err(format!(
                "Invalid command, please use one of the following: {:?}",
                self.available_commands
            )),
        }
    }
}

impl AliasCommand {
    fn execute_add(&self) -> io::Result<String> {
        if self.args.len() < 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid command, please provide a name and value to add",
            ));
        }

        let name = self.args[3].clone();
        let value = self.args[4].clone();

        let existing_aliases = self
            .find_existing_aliases()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("{}", e)))?;
        if existing_aliases.iter().any(|a| a.name == name) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias already exists",
            ));
        }

        let mut shell_config = fs::read_to_string(self.shell_config_path.clone())?;

        shell_config.push_str(&format!(
            "\n# Added by dotcli\nalias {}=\"{}\"\n",
            name, value
        ));
        fs::write(self.shell_config_path.clone(), shell_config)?;

        println!("Added alias to {}", self.shell_config_path);
        println!(
            "Note: Please run `source {}` to apply the changes",
            self.shell_config_path
        );
        Ok("".to_string())
    }

    fn execute_list(&self) -> Result<(), String> {
        println!(
            "{}",
            self.aliases
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
        Ok(())
    }

    fn find_existing_aliases(&self) -> Result<Vec<Alias>, String> {
        let shell_config = fs::read_to_string(self.shell_config_path.clone())
            .map_err(|e| format!("{} {}", self.shell_config_path, e))?;
        let lines = shell_config.lines();
        let mut aliases: Vec<Alias> = vec![];

        let mut count = 0;

        for line in lines {
            if line.contains("alias") {
                count += 1;
                let line_without_keyword = line
                    .split(" ")
                    .into_iter()
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" ");
                let name = line_without_keyword.split("=").nth(0).unwrap();
                let value = line_without_keyword.split("=").nth(1).unwrap();

                aliases.push(Alias {
                    id: count.to_string(),
                    name: name.to_string(),
                    value: value.to_string(),
                });
            }
        }
        Ok(aliases)
    }
}
