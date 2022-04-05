use crate::gpg::{Gpg, GPG_PREFIX};
use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub(crate) struct Variables {
    /// holds the values defined in template.toml
    pub variables: HashMap<String, String>,
    /// Store decrypted secret value
    /// this might be empty if the var is deserialized without gpg option
    pub secrets: HashMap<String, String>,
}

impl Variables {
    pub(crate) fn from_paths(
        base_path: &Path,
        var_paths: &[PathBuf],
        gpg: Option<&Gpg>,
    ) -> Result<Self> {
        let mut out = Self::default();
        for path in var_paths {
            let variables = Self::from_toml(&base_path.join(path), gpg)?;
            out.extend(variables);
        }

        Ok(out)
    }

    /// Deserialize a toml file struct Variables
    pub(crate) fn from_toml(path: &Path, gpg: Option<&Gpg>) -> Result<Self> {
        let file = File::open(path);

        if let Err(err) = file {
            eprintln!("{} {:?} : {}", "Could not open var file".red(), path, err);
            Ok(Self::default())
        } else {
            let mut buf_reader = BufReader::new(file.unwrap());
            let mut contents = String::new();

            buf_reader
                .read_to_string(&mut contents)
                .map_err(|err| anyhow!("Cannot read var file {:?} : {}", &path, err))?;

            let variables: HashMap<String, String> = toml::from_str(&contents)
                .map_err(|err| anyhow!("parse error in {:?} :  {}", path, err))?;

            let vars = if let Some(gpg) = gpg {
                let secrets = Variables::decrypt_values(&variables, gpg)?;
                Variables { variables, secrets }
            } else {
                Variables {
                    variables,
                    secrets: HashMap::default(),
                }
            };

            Ok(vars)
        }
    }

    /// Read file in the given path and return its content
    /// with variable replaced by their values.
    pub(crate) fn to_dot(&self, path: &Path) -> Result<String> {
        // Read file content
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        // Create the tera context from variables and secrets.
        let mut context = tera::Context::new();
        for (name, value) in self.variables.iter() {
            context.insert(name, value);
        }

        self.secrets.iter().for_each(|(k, v)| {
            context.insert(k.to_owned(), v);
        });

        tera::Tera::one_off(&contents, &context, false)
            .context("Failed to apply templating to file:")
    }

    pub(crate) fn resolve_ref(&mut self) {
        // Collect variable references
        let entries: Vec<(String, Option<String>)> = self
            .variables
            .iter()
            .filter(|(_, value)| value.starts_with('%'))
            .map(|(key, value)| (key, &value[1..value.len()]))
            .map(|(key, ref_key)| (key.clone(), self.variables.get(ref_key).cloned()))
            .collect();

        // insert value in place of references
        entries.iter().for_each(|(key, opt_value)| match opt_value {
            Some(value) => {
                let _ = self.variables.insert(key.to_string(), value.to_string());
            }
            None => {
                let warning = format!("Reference ${} not found in config", &key).yellow();
                eprintln!("{}", warning);
            }
        });
    }

    pub(crate) fn extend(&mut self, vars: Variables) {
        self.variables.extend(vars.variables);
        self.secrets.extend(vars.secrets);
    }

    pub(crate) fn insert(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    fn decrypt_values(
        vars: &HashMap<String, String>,
        gpg: &Gpg,
    ) -> Result<HashMap<String, String>> {
        let encrypted_vars = vars
            .iter()
            .filter(|(_, value)| value.starts_with(GPG_PREFIX));

        let mut secrets = HashMap::new();

        for (key, value) in encrypted_vars {
            let value = value.strip_prefix(GPG_PREFIX).unwrap();
            let value = gpg.decrypt_secret(value)?;
            let _ = secrets.insert(key.clone(), value);
        }

        Ok(secrets)
    }
}

#[cfg(test)]
mod test {
    use crate::templating::Variables;
    use anyhow::Result;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use speculoos::prelude::*;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    #[test]
    fn should_inject_variables() {
        let mut variables = HashMap::new();
        variables.insert("red".to_string(), "red_value".to_string());

        let dot = Variables {
            variables,
            secrets: Default::default(),
        }
        .to_dot(Path::new("tests/dotfiles_simple/template.css"))
        .unwrap();

        assert_eq!(
            dot,
            indoc! {
                    ".class {
                        color: red_value
                    }
                    "
            }
        );
    }

    #[test]
    fn should_inject_secret_variables() {
        let mut variables = HashMap::new();
        variables.insert("red".to_string(), "red_value".to_string());
        variables.insert("pass".to_string(), "encrypted with gpg".to_string());

        let mut secrets = HashMap::new();
        secrets.insert("pass".to_string(), "hunter2".to_string());

        let dot_content = Variables { variables, secrets }
            .to_dot(Path::new("tests/dotfiles_with_secret/template"))
            .unwrap();

        assert_that!(dot_content).contains("color: red_value");
        assert_that!(dot_content).contains("secret: hunter2");
    }

    #[test]
    fn should_fail_on_non_utf8_file() {
        let content = Variables {
            variables: HashMap::new(),
            secrets: Default::default(),
        }
        .to_dot(Path::new("tests/dotfiles_non_utf8/ferris.png"));

        assert_that!(content).is_err();
    }

    #[test]
    fn should_get_vars_from_toml() -> Result<()> {
        let vars = Variables::from_toml(&Path::new("tests/dotfiles_with_meta/vars.toml"), None)?;

        assert_eq!(vars.variables.get("red"), Some(&"%meta_red".to_string()));
        assert_eq!(vars.variables.get("black"), Some(&"#000000".to_string()));
        assert_eq!(vars.variables.get("green"), Some(&"#008000".to_string()));
        Ok(())
    }

    #[test]
    fn should_get_vars_multiple_path() -> Result<()> {
        let vars = Variables::from_paths(
            &Path::new("tests/dotfiles_with_meta/"),
            &[PathBuf::from("vars.toml"), PathBuf::from("meta_vars.toml")],
            None,
        )?;

        assert_eq!(vars.variables.get("red"), Some(&"%meta_red".to_string()));
        assert_eq!(vars.variables.get("black"), Some(&"#000000".to_string()));
        assert_eq!(vars.variables.get("green"), Some(&"#008000".to_string()));
        assert_eq!(vars.variables.get("meta_red"), Some(&"#FF0000".to_string()));
        Ok(())
    }

    #[test]
    fn extend_should_overwrite_vars() {
        let mut variables = HashMap::new();
        variables.insert("white".to_string(), "#000000".to_string());

        let mut secrets = HashMap::new();
        secrets.insert("password".to_string(), "hunter2".to_string());

        let mut extends = HashMap::new();
        extends.insert("white".to_string(), "#FFFFFF".to_string());

        let mut extends_secrets = HashMap::new();
        extends_secrets.insert("password".to_string(), "hunter3".to_string());

        let mut vars = Variables { variables, secrets };

        let extends = Variables {
            variables: extends,
            secrets: extends_secrets,
        };

        vars.extend(extends);

        assert_eq!(vars.variables.len(), 1);
        assert_eq!(vars.secrets.len(), 1);
        assert_eq!(vars.variables.get("white"), Some(&"#FFFFFF".to_string()));
        assert_eq!(vars.secrets.get("password"), Some(&"hunter3".to_string()));
    }
}
