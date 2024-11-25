use std::{
    env,
    fmt::{self, Display},
    fs, io,
};

use crate::command::command::Command;

use super::shell::{find_shell_config_path, is_absolute_path};

pub struct PathCommand {
    args: Vec<String>,
    available_commands: Vec<&'static str>,
    shell_config_path: String,
    paths: Vec<PathReference>,
}

struct PathReference {
    path: String,
    id: String,
}

impl Display for PathReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.id, self.path)
    }
}

impl PathCommand {
    pub fn new() -> Self {
        Self {
            args: vec![],
            available_commands: vec!["add", "list"],
            shell_config_path: "".to_string(),
            paths: vec![],
        }
    }
}

impl Command for PathCommand {
    fn init(&mut self, args: Vec<String>) -> Result<(), String> {
        self.args = args;
        self.shell_config_path = find_shell_config_path()?;
        self.paths = self.find_existing_paths()?;
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

impl PathCommand {
    fn execute_add(&self) -> io::Result<String> {
        if self.args.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid command, please provide a path to add",
            ));
        }

        let mut path = self.args[3].clone();
        let mut shell_config = fs::read_to_string(self.shell_config_path.clone())?;

        if !is_absolute_path(&path) {
            let current_dir = env::current_dir()?;
            let abs_path = current_dir.join(&path).canonicalize()?;
            if abs_path.try_exists()? {
                path = abs_path.to_str().unwrap().to_string();
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Path {} does not exist", path),
                )
                .into());
            }
        }

        shell_config.push_str(&format!(
            "\n# Added by dotcli\nexport PATH=\"$PATH:{}\"\n",
            path
        ));
        fs::write(self.shell_config_path.clone(), shell_config)?;

        println!("Added path to {}", self.shell_config_path);
        println!("Executing `source {}`", self.shell_config_path);

        std::process::Command::new("source")
            .arg(self.shell_config_path.clone())
            .spawn()?;

        println!("Done");
        Ok("".to_string())
    }

    fn execute_list(&self) -> Result<(), String> {
        println!(
            "{}",
            self.paths
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
        Ok(())
    }

    fn find_existing_paths(&self) -> Result<Vec<PathReference>, String> {
        let shell_config = fs::read_to_string(self.shell_config_path.clone())
            .map_err(|e| format!("{} {}", self.shell_config_path, e))?;
        let lines = shell_config.lines();
        let mut paths: Vec<PathReference> = vec![];

        let mut count = 0;

        for line in lines {
            if line.contains("export PATH=") {
                count += 1;
                paths.push(PathReference {
                    path: line.split("=").nth(1).unwrap().to_string(),
                    id: count.to_string(),
                });
            }
        }
        Ok(paths)
    }
}
