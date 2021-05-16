+++
title = "GPG secret"
description = "Manage GPG secret"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "How to setup GPG encryption with Toml Bombadil"
toc = true
top = false
+++


### Manage secrets

**Before going further with this ensure `.dots` is in dotfiles repository's `.gitignore`!**

To use encryption this you need to have [gnupg](https://gnupg.org/) installed, and a pair of gpg keys.

1. Add your gpg user id to bombadil's config :

    ```toml
    dotfile_dir = "bombadil-example"
    # The gpg user associated with the key pair you want to use
    gpg_user_id = "me@example.org" 
   
    vars = [ "vars.toml" ]
    
    [settings.dots]
    maven = { source = "maven/settings.xml", target = ".m2/settings.xml"}
    ```

2. Add secret variable :

    ```
    bombadil add-secret -k "server_password" -v "hunter2" -f vars.toml
    ```
   or if you want to avoid having secrets in your shell history :
    ```
    bombadil add-secret -k "server_password" -f vars.toml --ask
    ```

3. Use the secret value in any dot entry :
    ```xml
        <server>
          <id>my.server</id>
          <username>Tom</username>
          <password>__[server_password]__</password>
        </server>
    ```
4. Make sure the secret has been written and encrypted :
    - Get the decrypted value :
   ```
   bombadil get secrets
   ```
    - Get the raw encrypted value :
   ```
   bombadil get vars | grep server_password
   ```

5. Relink your dotfile to inject the secret value :
    ```
    bombadil link
    ```

This is it