use anyhow::Result;
use colored::*;
use dirs::home_dir;
use std::path::PathBuf;

/// Represent a link between a `source` dotfile in the user defined dotfiles directory
/// and the XDG `target` path where it should be linked
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dot {
    /// Path relative to user defined dotfile
    pub source: PathBuf,
    /// Target path either relative to $HOME or absolute
    pub target: PathBuf,
}

/// Same as dot but source and target are optionals
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DotOverride {
    /// Path relative to user defined dotfile
    pub source: Option<PathBuf>,
    /// Target path either relative to $HOME or absolute
    pub target: Option<PathBuf>,
}

impl Dot {
    /// Return the target path of a dot entry either absolute or relative to $HOME
    pub fn target(&self) -> Result<PathBuf> {
        if self.target.is_absolute() {
            Ok(self.target.clone())
        } else {
            home_dir()
                .map(|home| home.join(&self.target))
                .ok_or_else(|| {
                    let err = format!("Unable to find dot path : {:?}", &self.target).red();
                    anyhow!(err)
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dots::Dot;
    use std::path::PathBuf;

    #[test]
    fn should_get_target_path() {
        // Arrange
        let home = env!("HOME");

        let dot = Dot {
            source: Default::default(),
            target: PathBuf::from(".config/sway"),
        };

        // Act
        let result = dot.target();

        // Assert
        assert!(result.is_ok());
        let expected = PathBuf::from(home).join(".config").join("sway");

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn should_get_absolute_target_path() {
        // Arrange
        let dot = Dot {
            source: Default::default(),
            target: PathBuf::from("/etc/profile"),
        };

        // Act
        let result = dot.target();

        // Assert
        assert!(result.is_ok());

        let expected = PathBuf::from("/etc/profile");
        assert_eq!(result.unwrap(), expected);
    }
}
