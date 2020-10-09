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

    pub(crate) fn push_secret<S: AsRef<Path> + ?Sized>(
        &self,
        key: &str,
        value: &str,
        var_file: &S,
    ) -> Result<()> {
        let mut vars = Variables::from_toml(var_file.as_ref(), Some(&self))?;
        println!("Added {} : {}", key, value);
        let encrypted = self.encrypt(value)?;
        let encrypted = encrypted.replace(PGP_HEADER, "");
        let encrypted = encrypted.replace(PGP_FOOTER, "");

        let encrypted = format!("gpg:{}", encrypted);
        vars.insert(key.to_string(), encrypted);

        let toml = toml::to_string(&vars.variables)?;
        std::fs::write(&var_file, toml)?;

        Ok(())
    }

    pub(crate) fn decrypt_secret(&self, content: &str) -> Result<String> {
        let pgp_message = format!("{}{}{}", PGP_HEADER, content, PGP_FOOTER);
        self.decrypt(&pgp_message)
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

        let output = child.wait_with_output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).expect("Error getting encrypted value"))
                } else {
                    Err(anyhow!(
                        String::from_utf8(output.stdout).expect("Error getting encrypted value")
                    ))
                }
            }
            Err(err) => Err(anyhow!("Error encrypting content : {}", err)),
        }
    }

    fn decrypt(&self, content: &str) -> Result<String> {
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

        {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(content.as_bytes())?;
        }

        let output = child.wait_with_output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).expect("Error decrypting content"))
                } else {
                    Err(anyhow!(
                        String::from_utf8(output.stdout).expect("Error getting decrypting value")
                    ))
                }
            }
            Err(err) => Err(anyhow!("{}", err)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::gpg::Gpg;
    use anyhow::Result;
    use std::process::Command;

    const GPG_ID: &str = "test@toml.bombadil.org";

    fn import_keys() -> Result<()> {
        Command::new("gpg")
            .arg("--import")
            .arg("tests/gpg/public.gpg")
            .output()?;

        Command::new("gpg")
            .arg("--import")
            .arg("tests/gpg/private.gpg")
            .output()?;

        Ok(())
    }

    #[test]
    fn should_encrypt() {
        import_keys().unwrap();
        let gpg = Gpg::new(GPG_ID);

        let result = gpg.encrypt("test");

        assert!(result.is_ok())
    }

    #[test]
    fn should_not_encrypt_unkown_gpg_user() {
        let gpg = Gpg::new("unknown.user ");

        let result = gpg.encrypt("test");

        assert!(result.is_err())
    }

    #[test]
    fn should_decrypt() -> Result<()> {
        import_keys().unwrap();
        let gpg = Gpg::new(GPG_ID);

        let encrypted = gpg.encrypt("value")?;
        let decrypted = gpg.decrypt(&encrypted);

        assert!(decrypted.is_ok());
        assert_eq!(decrypted?, "value");
        Ok(())
    }
}
