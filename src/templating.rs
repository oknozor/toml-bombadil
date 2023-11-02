use crate::gpg::Gpg;
use crate::settings::GPG;
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json_merge::{Dfs, Merge};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tera::{Context, Map, Tera, Value};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Variables {
    /// holds the values defined in template.toml
    pub(crate) inner: Value,
}

impl Variables {
    pub(crate) fn has_secrets(&self) -> bool {
        self.inner
            .get("secrets")
            .and_then(|vars| vars.as_object())
            .map(|secrets| !secrets.is_empty())
            .unwrap_or(false)
    }

    pub(crate) fn without_secrets(&self) -> Value {
        let mut vars = self.inner.clone();
        if let Some(vars) = vars.as_object_mut() {
            vars.remove("secrets");
        }

        vars
    }

    pub(crate) fn get_secrets(&self) -> Result<Map<String, Value>> {
        let secrets = self
            .inner
            .get("secrets")
            .and_then(|value| value.as_object());

        let Some(secrets) = secrets else {
            return Ok(Map::new());
        };

        let Some(gpg) = GPG.as_ref() else {
            return Err(anyhow!("Cannot decrypt secrets, no GPG user id configured"));
        };

        let mut decrypted_secrets = serde_json::Map::new();
        for (key, value) in secrets {
            let decrypted = gpg.decrypt_secret(
                value
                    .as_str()
                    .expect("Secret value mut be a gpg encrypted string"),
            )?;
            decrypted_secrets.insert(key.clone(), Value::String(decrypted));
        }

        Ok(decrypted_secrets)
    }

    pub(crate) fn push_secret(&mut self, key: &str, encrypted: &str) {
        match self.get_secrets_mut() {
            None => {
                let mut secrets = tera::Map::new();
                secrets.insert(key.to_string(), Value::String(encrypted.to_string()));
                let Some(vars) = self.inner.as_object_mut() else {
                    panic!("Variables should be a Value::Object");
                };

                vars.insert("secrets".to_string(), Value::Object(secrets));
            }
            Some(secrets) => {
                secrets.insert(key.to_string(), Value::String(encrypted.to_string()));
            }
        };
    }

    pub(crate) fn from_paths(base_path: &Path, var_paths: &[PathBuf]) -> Result<Self> {
        let mut out = Self::default();
        for path in var_paths {
            let variables = Self::from_path(&base_path.join(path))?;
            out.extend(variables);
        }

        Ok(out)
    }

    /// Deserialize a toml file struct Variables
    pub(crate) fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
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

            let variables: tera::Value = toml::from_str(&contents)
                .map_err(|err| anyhow!("parse error in {:?} :  {}", path, err))?;

            let vars = if let Some(gpg) = GPG.as_ref() {
                let secrets = variables
                    .get("secrets")
                    .and_then(|secrets| secrets.as_object());

                // Replace secrets with their decrypted values
                if let Some(secrets) = secrets {
                    let secrets = Variables::decrypt_values(secrets, gpg)?;
                    variables.get("secrets").replace(&secrets);
                };

                Variables { inner: variables }
            } else {
                Variables { inner: variables }
            };

            Ok(vars)
        }
    }

    /// Read file in the given path and return its content
    /// with variable replaced by their values.
    pub(crate) fn to_dot(&self, path: &Path) -> tera::Result<String> {
        // Read file content
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        // Create the tera context from variables and secrets.
        let mut context = tera::Context::new();
        let variable_context = Context::from_serialize(self.inner.clone())?;
        context.extend(variable_context);
        let mut tera = Tera::default();
        let filename = path.as_os_str().to_str().expect("Non UTF8 filename");

        tera.add_raw_template(filename, &contents)?;
        tera.render(filename, &context)
    }

    pub(crate) fn extend(&mut self, other: Variables) {
        self.inner.merge_recursive::<Dfs>(&other.inner);
    }

    pub(crate) fn with_secrets(&mut self, secrets: Map<String, Value>) {
        let Some(vars) = self.inner.as_object_mut() else {
            panic!("Variable should be a Value::Object")
        };

        vars.insert("secrets".to_string(), Value::Object(secrets));
    }

    fn decrypt_values(vars: &Map<String, Value>, gpg: &Gpg) -> Result<Value> {
        let mut decrypted = tera::Map::new();
        for (key, value) in vars {
            let value = value
                .as_str()
                .expect("'[secrets]' values must be a encrypted type string");
            let value = gpg.decrypt_secret(value)?;
            decrypted.insert(key.clone(), tera::Value::String(value));
        }

        Ok(Value::Object(decrypted))
    }

    fn get_secrets_mut(&mut self) -> Option<&mut Map<String, Value>> {
        self.inner
            .get_mut("secrets")
            .and_then(|value| value.as_object_mut())
    }

    pub(crate) fn with_os(mut self) -> Self  {
        let Some(vars) = self.inner.as_object_mut() else {
            panic!("Variables should be a Value::Object");
        };

        vars.insert("os".to_string(), Value::String(std::env::consts::OS.to_string()));
        self
    }
}

#[cfg(test)]
mod test {
    use crate::templating::Variables;
    use anyhow::Result;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Map, Value};
    use speculoos::prelude::*;
    use std::path::Path;

    #[test]
    fn should_inject_variables() {
        let mut variables = Map::new();
        variables.insert("red".to_string(), Value::String("red_value".to_string()));
        let variables = Value::Object(variables);

        let dot = Variables { inner: variables }
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
    fn should_replace_existing_secret() -> Result<()> {
        let mut variables = Map::new();
        variables.insert("red".to_string(), Value::String("red_value".to_string()));

        let mut variables = Variables {
            inner: Value::Object(variables),
        };

        variables.push_secret("pass", "hunter2");

        let dot_content = variables
            .to_dot(Path::new("tests/dotfiles_with_secret/template"))
            .unwrap();

        assert_eq!(
            dot_content,
            indoc! {
                r#"
            color: red_value
            secret: hunter2"#
            }
        );
        Ok(())
    }

    #[test]
    fn should_fail_on_non_utf8_file() {
        let content = Variables {
            inner: Value::Object(Map::new()),
        }
        .to_dot(Path::new("tests/dotfiles_non_utf8/ferris.png"));

        assert_that!(content).is_err();
    }

    #[test]
    fn should_get_vars_from_toml() -> Result<()> {
        let vars = Variables::from_path(&Path::new("tests/dotfiles_vars/vars.toml"))?;

        assert_eq!(
            vars.inner.get("red").and_then(Value::as_str),
            Some("#FF0000")
        );
        assert_eq!(
            vars.inner.get("black").and_then(Value::as_str),
            Some("#000000")
        );
        assert_eq!(
            vars.inner.get("green").and_then(Value::as_str),
            Some("#008000")
        );
        Ok(())
    }

    #[test]
    fn extend_should_overwrite_vars() -> Result<()> {
        // Arrange
        let mut variables: Variables = toml::from_str(indoc! {
            "
            white = \"#000000\"
            black = \"#000000\"

            [secrets]
            password = \"hunter2\"
            "
        })?;

        let overrides: Variables = toml::from_str(indoc! {
            "
            white = \"#FFFFFF\"
            other_var = 1

            [secrets]
            password = \"hunter3\"
            other_secret = \"secret\"
            "
        })?;

        // Act
        variables.extend(overrides);

        // Assert
        // Note: if you wonder why json is used as the output format here
        // Take a look at https://github.com/toml-rs/toml-rs/issues/142
        // Also since this is never serialized back to toml but used in tera
        // context only, comparison using json is not an issue
        assert_eq!(
            variables.inner,
            json! {
                {
                  "white": "#FFFFFF",
                  "black": "#000000",
                  "secrets": {
                    "password": "hunter3",
                    "other_secret": "secret"
                  },
                  "other_var": 1
                }
            }
        );

        Ok(())
    }

    #[test]
    fn should_have_secrets() -> Result<()> {
        // Arrange
        let variables: Variables = toml::from_str(indoc! {
            "
            white = \"#000000\"
            black = \"#000000\"

            [secrets]
            password = \"hunter2\"
            "
        })?;

        // Act + Assert
        assert_that!(variables.has_secrets()).is_true();

        Ok(())
    }

    #[test]
    fn should_not_have_secrets() -> Result<()> {
        // Arrange
        let variables: Variables = toml::from_str(indoc! {
            "
            white = \"#000000\"
            black = \"#000000\"
            "
        })?;

        // Act + Assert
        assert_that!(variables.has_secrets()).is_false();

        Ok(())
    }
}
