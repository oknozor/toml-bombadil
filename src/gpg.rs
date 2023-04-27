use crate::templating::Variables;
use anyhow::{anyhow, Result};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const PGP_HEADER: &str = "-----BEGIN PGP MESSAGE-----\n\n";
const PGP_FOOTER: &str = "\n-----END PGP MESSAGE-----";
pub(crate) const GPG_PREFIX: &str = "gpg:";

#[derive(Clone)]
pub struct Gpg {
    pub user_ids: Vec<String>,
}

impl Gpg {
    pub(crate) fn new(user_ids: Vec<String>) -> Self {
        Gpg { user_ids }
    }

    pub(crate) fn push_secret<S: AsRef<Path> + ?Sized>(
        &self,
        key: &str,
        value: &str,
        var_file: &S,
    ) -> Result<()> {
        let mut vars = Variables::from_toml(var_file.as_ref())?;
        let encrypted = self.encrypt(value)?;
        let encrypted = encrypted.replace(PGP_HEADER, "");
        let encrypted = encrypted.replace(PGP_FOOTER, "");

        let encrypted = format!("gpg:{}", encrypted);
        vars.insert(key, &encrypted);

        let toml = toml::to_string(&vars.variables)?;
        std::fs::write(var_file, toml)?;
        println!("Added {} : {}", key, value);

        Ok(())
    }

    pub(crate) fn decrypt_secret(&self, content: &str) -> Result<String> {
        let pgp_message = format!("{}{}{}", PGP_HEADER, content, PGP_FOOTER);
        self.decrypt(&pgp_message)
    }

    fn encrypt(&self, content: &str) -> Result<String> {
        let mut cmd_binding = Command::new("gpg");
        let cmd = cmd_binding.arg("--encrypt").arg("--armor");
        for user_id in &self.user_ids {
            cmd.arg("-r").arg(user_id);
        }
        let mut child = cmd
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
                    Ok(String::from_utf8(output.stdout)?)
                } else {
                    Err(anyhow!(String::from_utf8(output.stdout)?))
                }
            }
            Err(err) => Err(anyhow!("Error encrypting content : {}", err)),
        }
    }

    fn decrypt(&self, content: &str) -> Result<String> {
        let mut cmd_binding = Command::new("gpg");
        let cmd = cmd_binding.arg("--decrypt").arg("--armor").arg("-q");
        for user_id in &self.user_ids {
            cmd.arg("-r").arg(user_id);
        }
        let mut child = cmd
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
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::env;
    use toml::Value;

    const GPG_IDS: [&'static str; 2] = ["test@toml.bombadil.org", "me@ibotty.net"];

    fn gpg_setup() {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        run_cmd!(
            gpg --import $crate_dir/tests/gpg/public.gpg;
            gpg --import $crate_dir/tests/gpg/private.gpg;
            echo -e "5\ny\n" | gpg --no-tty --command-fd 0 --expert --edit-key test@toml.bombadil.org trust;
        ).unwrap();
    }

    #[sealed_test(before = gpg_setup())]
    fn should_encrypt() {
        let gpg = Gpg::new(GPG_IDS.map(String::from).to_vec());

        let result = gpg.encrypt("test");

        assert_that!(result).is_ok();
    }

    #[sealed_test(before = gpg_setup())]
    fn should_not_encrypt_unkown_gpg_user() {
        let gpg = Gpg::new(vec!("unknown.user".to_string()));

        let result = gpg.encrypt("test");

        assert_that!(result).is_err();
    }

    #[sealed_test(before = gpg_setup())]
    fn should_decrypt() -> Result<()> {
        let gpg = Gpg::new(GPG_IDS.map(String::from).to_vec());

        let encrypted = gpg.encrypt("value")?;
        let decrypted = gpg.decrypt(&encrypted);

        assert_that!(decrypted)
            .is_ok()
            .is_equal_to(&"value".to_string());

        Ok(())
    }

    #[sealed_test(before = gpg_setup())]
    fn should_push_to_var() -> Result<()> {
        let gpg = Gpg::new(GPG_IDS.map(String::from).to_vec());
        std::fs::write("vars.toml", "")?;
        gpg.push_secret("key", "value", "vars.toml")?;

        let result = std::fs::read_to_string("vars.toml")?;
        let toml: Value = toml::from_str(&result)?;
        let value = toml.get("key");
        assert_that!(value).is_some();
        assert_that!(value.unwrap().as_str().unwrap().starts_with("gpg:"));
        Ok(())
    }

    #[sealed_test(before = gpg_setup())]
    fn should_decrypt_from_file() -> Result<()> {
        let gpg = Gpg::new(GPG_IDS.map(String::from).to_vec());
        std::fs::write("vars.toml", "")?;
        gpg.push_secret("key", "value", "vars.toml")?;

        let result = std::fs::read_to_string("vars.toml")?;
        let toml: Value = toml::from_str(&result)?;
        let value = toml.get("key");
        let value = value.unwrap().as_str().unwrap();
        let value = value.strip_prefix("gpg:").unwrap();

        let decrypted = gpg.decrypt_secret(value)?;

        assert_eq!(decrypted, "value");
        Ok(())
    }
}
