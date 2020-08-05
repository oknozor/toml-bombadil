use anyhow::Result;
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
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader
            .read_to_string(&mut contents)
            .map_err(|err| anyhow!("Cannot find var file {:?} : {}", &path, err))?;

        let variables: HashMap<String, String> = toml::from_str(&contents)
            .map_err(|err| anyhow!("parse error in {:?} :  {}", path, err))?;

        Ok(Self { variables })
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

                    let value = self
                        .variables
                        .get(var_name)
                        .unwrap_or_else(|| panic!("Undefined variable : {}", var_name));

                    output.push_str(value);
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
    fn test() {
        let mut map = HashMap::new();
        map.insert("red".to_string(), "red_value".to_string());

        let string = Variables { variables: map }
            .to_dot(Path::new("tests/dotfiles_simple/template"))
            .unwrap();

        assert_eq!(string, "color: red_value");
    }
}
