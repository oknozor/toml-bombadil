use anyhow::Result;
use cmd_lib::{CmdResult, Process};
use colored::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hook {
    pub command: String,
}

impl Hook {
    pub(crate) fn run(&self) -> Result<()> {
        let command_display = format!("`{}`", &self.command.green());
        println!("Running post install hook : {}", command_display);

        Process::new(self.command.clone())
            .wait::<CmdResult>()
            .map_err(|err| anyhow!(err))
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
