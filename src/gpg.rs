use crate::templating::Variables;
use anyhow::Result;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const PGP_HEADER: &str = "-----BEGIN PGP MESSAGE-----\n\n";
const PGP_FOOTER: &str = "\n-----END PGP MESSAGE-----";

pub struct Gpg {
    pub user_id: String,
}

impl Gpg {
    pub fn new(user_id: &str) -> Self {
        Gpg {
            user_id: user_id.to_string(),
        }
    }

    pub fn push_secret<S: AsRef<Path> + ?Sized>(
        &self,
        key: &str,
        value: &str,
        var_file: &S,
    ) -> Result<()> {
        let mut vars = Variables::from_toml(var_file.as_ref(), Some(&self))?;
        println!("Added {}:{}", key, value);
        let encrypted = self.encrypt(value)?;
        let encrypted = encrypted.replace(PGP_HEADER, "");
        let encrypted = encrypted.replace(PGP_FOOTER, "");

        let encrypted = format!("gpg:{}", encrypted);
        vars.insert(key.to_string(), encrypted);

        let toml = toml::to_string(&vars.variables)?;
        std::fs::write(&var_file, toml)?;

        Ok(())
    }

    fn encrypt(&self, content: &str) -> Result<String> {
        let mut child = Command::new("gpg")
            .arg("--encrypt")
            .arg("--armor")
            .arg("-r")
            .arg(&self.user_id)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("error calling gpg command, is gpg installed ?");

        {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(content.as_bytes())?;
        }

        child
            .wait_with_output()
            .map(|result| String::from_utf8(result.stdout).expect("Error getting encrypted value"))
            .map_err(|err| anyhow!("Error encrypting content : {}", err))
    }

    pub(crate) fn decrypt(&self, content: &str) -> Result<String> {
        let mut child = Command::new("gpg")
            .arg("--decrypt")
            .arg("--armor")
            .arg("-r")
            .arg(&self.user_id)
            .arg("-q")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("error calling gpg command, is gpg installed ?");

        let pgp_message = format!("{}{}{}", PGP_HEADER, content, PGP_FOOTER);
        {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(pgp_message.as_bytes())?;
        }

        child
            .wait_with_output()
            .map(|result| String::from_utf8(result.stdout).expect("Error getting encrypted value"))
            .map_err(|err| anyhow!("Error encrypting content : {}", err))
    }
}
