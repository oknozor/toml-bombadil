use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub const STORE_PATH: &str = "secret_store.gpg";

pub struct Gpg {
    pub user_id: String,
}

impl Gpg {
    pub fn new(user_id: &str) -> Self {
        Gpg {
            user_id: user_id.to_string(),
        }
    }

    pub fn push_secret(self, key: &str, value: &str) -> Result<()> {
        let mut secrets = if Path::new(STORE_PATH).exists() {
            self.decrypt()?
        } else {
            HashMap::new()
        };

        secrets.insert(key.to_string(), value.to_string());

        let toml = toml::to_string(&secrets)?;
        println!("{}", toml);
        let encrypted = self.encrypt(&toml)?;
        std::fs::write(STORE_PATH, encrypted)?;

        Ok(())
    }

    pub fn remove_secret(&self, key: &str) -> Result<()> {
        let mut secrets = if Path::new(STORE_PATH).exists() {
            self.decrypt()?
        } else {
            return Err(anyhow!("Secret store not found"));
        };

        if secrets.contains_key(key) {
            secrets.remove(key);
        } else {
            return Err(anyhow!("Given key wasn't found in the secret store"));
        }

        let toml = toml::to_string(&secrets)?;
        println!("{}", toml);
        let encrypted = self.encrypt(&toml)?;
        std::fs::write(STORE_PATH, encrypted)?;

        Ok(())
    }

    pub fn encrypt(&self, content: &str) -> Result<Vec<u8>> {
        let mut child = Command::new("gpg")
            .arg("--encrypt")
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
            .map(|result| result.stdout)
            .map_err(|err| anyhow!("Error encrypting content : {}", err))
    }

    pub fn decrypt(&self) -> Result<HashMap<String, String>> {
        let output = Command::new("gpg")
            .arg("--decrypt")
            .arg("-r")
            .arg(&self.user_id)
            .arg(&STORE_PATH)
            .output()?;

        let content = String::from_utf8(output.stdout)?;
        toml::from_str::<HashMap<String, String>>(&content)
            .map_err(|err| anyhow!("Failed to decrypt secret store {}", err))
    }
}
