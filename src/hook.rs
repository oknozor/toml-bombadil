use anyhow::Result;
use colored::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hook {
    pub command: String,
}

impl Hook {
    pub(crate) fn run(&self) -> Result<()> {
        let command_display = format!("`{}`", &self.command.green());
        println!("Running install hook : {}", command_display);

        let args = self.split_args()?;
        let mut command = Hook::build_command(args);

        let mut child = command
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        BufReader::new(child.stdout.take().unwrap())
            .lines()
            .for_each(|line| println!("{}", line.unwrap_or_else(|_| "".into())));

        BufReader::new(child.stderr.take().unwrap())
            .lines()
            .for_each(|line| eprintln!("{}", line.unwrap_or_else(|_| "".into())));

        child.wait().map(|_| ()).map_err(|err| anyhow!(err))
    }

    fn build_command(args: Vec<&str>) -> Command {
        let mut command = Command::new(args[0]);
        let mut pos = 1;

        while let Some(arg) = args.get(pos) {
            command.arg(arg);
            pos += 1;
        }

        command
    }

    // TODO : use shell words
    fn split_args(&self) -> Result<Vec<&str>> {
        let mut indices: Vec<usize> = self
            .command
            .rmatch_indices('\"')
            .map(|(idx, _)| idx)
            .collect();

        if indices.is_empty() {
            return Ok(self.command.split(' ').collect());
        }

        if indices.len() % 2 != 0 {
            return Err(anyhow!("Missing matching \\\""));
        }

        let mut args = vec![];
        let mut cursor = 0;
        while let (Some(start_quote_idx), Some(end_quote_idx)) = (indices.pop(), indices.pop()) {
            let start = start_quote_idx + 1;
            let split_space: Vec<&str> = self.command[cursor..start]
                .split(' ')
                .filter(|split| !split.is_empty())
                .filter(|split| *split != "\"")
                .collect();

            args.extend_from_slice(&split_space);
            args.push(&self.command[start..end_quote_idx]);
            cursor = end_quote_idx + 1;
        }

        Ok(args)
    }
    pub fn new(command: &str) -> Self {
        let command = command.to_owned();
        Hook { command }
    }
}

#[cfg(test)]
mod tests {
    use crate::hook::Hook;

    #[test]
    fn should_run_command() {
        // Arrange
        let hook = Hook {
            command: "echo hello world".to_string(),
        };

        // Act
        let result = hook.run();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_to_run_invalid_command() {
        // Arrange
        let hook = Hook {
            command: "azmroih".to_string(),
        };

        // Act
        let result = hook.run();

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn should_split_args_with_quotes() {
        // Arrange
        let hook = Hook {
            command: "echo \"hello Tom\"".to_string(),
        };

        // Act
        let result = hook.split_args();

        // Assert
        assert_eq!(result.unwrap(), vec!["echo", "hello Tom"]);
    }

    #[test]
    fn should_split_args_with_multiple_quotes() {
        // Arrange
        let hook = Hook {
            command: "echo \"hello Tom\" || grep \"Toml\"".to_string(),
        };

        // Act
        let result = hook.split_args();

        // Assert
        assert_eq!(
            result.unwrap(),
            vec!["echo", "hello Tom", "||", "grep", "Toml"]
        );
    }

    #[test]
    fn should_split_args() {
        // Arrange
        let hook = Hook {
            command: "git commit -m init".to_string(),
        };

        // Act
        let result = hook.split_args();

        // Assert
        assert_eq!(result.unwrap(), vec!["git", "commit", "-m", "init"]);
    }
}
