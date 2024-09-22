---
 prev:
   text: 'Hooks'
   link: '/guide/imports'
   next: false
---

 # Encrypted secret

::: warning
To use encryption, you need to have gnupg installed, and a pair of gpg keys.

Encrypted value will be stored in your variable file, but once rendered, secret will be decrypted in `.dots/`
directory. Before going further ensure `.dots/` is in your dotfiles repository .gitignore.
:::

## Configuration

Add your gpg user id to `bombadil.toml`:

```toml
dotfile_dir = "bombadil-example"

# The gpg user associated with the key pair you want to use
gpg_user_id = "me@example.org"

vars = [ "vars.toml" ]
```

## Adding secrets

```bash
bombadil add-secret -k "server_password" -v "hunter2" -f vars.toml
```

Alternatively If you want to avoid having secrets in your shell history :

```bash
bombadil add-secret -k "server_password" -f vars.toml --ask
```

::: tip
Note that from now on bombadil will prompt for your GPG key password each time you link dot entries.
Make sure to configure the desired [pinentry](~/.gnupg/gpg-agent.conf) program in `~/.gnupg/gpg-agent.conf`.

```bash
# File: /home/okno/.gnupg/gpg-agent.conf
pinentry-program /usr/bin/pinentry-gnome3
# ...
```
:::
