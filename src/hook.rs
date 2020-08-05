use anyhow::Result;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hook {
    pub command: String,
}

impl Hook {
    pub(crate) fn run(&self) -> Result<()> {
        let args: Vec<&str> = self.command.split(' ').collect();
        if args.is_empty() {
            return Err(anyhow!("Cannot run empty hook"));
        }

        let mut command = Command::new(args[0]);

        let mut idx = 1;
        while let Some(arg) = args.get(idx) {
            command.arg(arg);
            idx += 1;
        }

        println!("Running post install hook : {}", &self.command);
        command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;

        Ok(())
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
