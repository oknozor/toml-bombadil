+++
title = "GPG secret"
description = "Manage GPG secret"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 4
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "How to setup GPG encryption with Toml Bombadil"
toc = true
top = false
+++


### Requirement

To use encryption this you need to have [gnupg](https://gnupg.org/) installed, and a pair of gpg keys.

⚠️ Encrypted value will be stored in your variable file, but once rendered, secret will be in clear in `.dots/`  directory.
Before going further with this ensure `.dots` is in your dotfiles repository `.gitignore`.

### Configuration

1. Add your gpg user id to bombadil's config :

    ```toml
    dotfile_dir = "bombadil-example"
    # The gpg user ids associated with the key pairs you want to use
    gpg_user_ids = [ "me@example.org", "my-colleague@example.org" ]

    vars = [ "vars.toml" ]

    [settings.dots]
    maven = { source = "maven/settings.xml", target = ".m2/settings.xml"}
    ```

### Adding secret

```bash
bombadil add-secret -k "server_password" -v "hunter2" -f vars.toml
```

Alternatively If you want to avoid having secrets in your shell history :

```bash
 bombadil add-secret -k "server_password" -f vars.toml --ask
```

### Use secrets

Once you secret has been added to a variable file you can use it as a normal variable :

```xml
    <server>
      <id>my.server</id>
      <username>Tom</username>
      <password>{{server_password}}</password>
    </server>
```

### Final steps

Make sure the secret has been written and encrypted :

   1. Get the decrypted value :

   ```bash
  bombadil get secrets
   ```

   2. Get the raw encrypted value :
   ```bash
  bombadil get vars | grep server_password
   ```

   3. Relink your dotfile to inject the secret value :
   ```bash
   bombadil link
   ```

That's it ! In the next chapter we will take a look at Bombadil profiles and themes.
