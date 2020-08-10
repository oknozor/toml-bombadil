use anyhow::Result;
use colored::Colorize;
use pest::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Parser)]
#[grammar = "template.pest"]
struct BombadilParser;

pub(crate) struct Variables {
    /// holds the values defined in template.toml
    pub variables: HashMap<String, String>,
}

impl Variables {
    /// Deserialize a toml file struct Variables
    pub(crate) fn from_toml(path: &Path) -> Result<Self> {
        let file = File::open(path);

        if let Err(err) = file {
            let warning = format!("Could not open var file {:?} : {}", path, err).red();
            eprintln!("{}", warning);
            Ok(Self::default())
        } else {
            let mut buf_reader = BufReader::new(file.unwrap());
            let mut contents = String::new();

            buf_reader
                .read_to_string(&mut contents)
                .map_err(|err| anyhow!("Cannot read var file {:?} : {}", &path, err))?;

            let variables: HashMap<String, String> = toml::from_str(&contents)
                .map_err(|err| anyhow!("parse error in {:?} :  {}", path, err))?;

            Ok(Self { variables })
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

        let pairs = BombadilParser::parse(Rule::file, &contents)
            .expect("Unable to parse template file")
            .next()
            .unwrap();

        let mut output = String::new();

        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::variable => {
                    let var_name = pair.into_inner().next().unwrap().as_str().trim();

                    let value = self.variables.get(var_name).cloned().unwrap_or_else(|| {
                        let err = format!("Undefined variable : {} in {:?}", var_name, path);
                        eprintln!("{}", err.yellow());
                        "undefined variable".to_string()
                    });

                    output.push_str(&value);
                }
                Rule::raw_content => output.push_str(pair.as_str()),
                _ => (),
            }
        }

        Ok(output)
    }

    pub(crate) fn extend(&mut self, template: Variables) {
        self.variables.extend(template.variables);
    }
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            variables: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::templating::Variables;
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn should_inject_variables() {
        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());

        let string = Variables { variables: map }
            .to_dot(Path::new("tests/dotfiles_simple/template"))
            .unwrap();

        assert_eq!(string, "color: red_value");
    }

    #[test]
    fn should_fail_on_non_utf8_file() {
        let content = Variables {
            variables: HashMap::new(),
        }
        .to_dot(Path::new("tests/dotfiles_non_utf8/ferris.png"));

        assert!(content.is_err());
    }
}
