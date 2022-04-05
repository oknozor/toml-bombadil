use anyhow::{anyhow, Result};
use colored::*;
use serde_derive::{Deserialize, Serialize};
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

        let mut child = Command::new("sh")
            .args(["-c", &self.command])
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        BufReader::new(child.stdout.take().unwrap())
            .lines()
            .for_each(|line| println!("{}", line.unwrap_or_else(|_| "".into())));

        BufReader::new(child.stderr.take().unwrap())
            .lines()
            .for_each(|line| eprintln!("{}", line.unwrap_or_else(|_| "".into())));

        child
            .wait()
            .map(|exit_code| {
                if exit_code.success() {
                    Ok(())
                } else {
                    Err(anyhow!("Hook run failed with status {}", exit_code))
                }
            })
            .unwrap()
    }

    pub fn new(command: &str) -> Self {
        let command = command.to_owned();
        Hook { command }
    }
}

#[cfg(test)]
mod tests {
    use crate::hook::Hook;
    use speculoos::prelude::*;

    #[test]
    fn should_run_command() {
        // Arrange
        let hook = Hook {
            command: "echo hello world".to_string(),
        };

        // Act
        let result = hook.run();

        // Assert
        assert_that!(result).is_ok();
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
        assert_that!(result).is_err();
    }
}
