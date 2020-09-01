use anyhow::Result;
use colored::*;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hook {
    pub command: String,
}

impl Hook {
    pub(crate) fn run(&self) -> Result<()> {
        let command_display = format!("`{}`", &self.command.green());
        println!("Running post install hook : {}", command_display);
        let args: Vec<&str> = self.command.split(' ').collect();
        let mut command = Command::new(args[0]);
        let mut pos = 1;

        while let Some(arg) = args.get(pos) {
            command.arg(arg);
            pos += 1;
        }

        command.output().map_err(|err| anyhow!(err)).map(|_| ())
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
}
