+++
title = "Hooks"
description = "Hooks"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Setup Bombadil post install hooks"
toc = true
top = false
+++



## Hooks

So far we have not talked about hooks, as we saw they can be declared as an entry in the config :

```toml
dotfiles_dir = "bombadil-example.toml"
[settings]
hooks = [ "sway reload" ]
```

This will invoke the `sway reload` command after `bombadil link` has updated your dotfiles.

The default hooks will always run regardless of the activated profiles.
You can also add hooks for a specific profile.

```toml
dotfiles_dir = "bombadil-example.toml"

[settings]
# This resides in the default profile an will always be executed after bombadil link
hooks = [ "sway reload", "echo 42" ]

[profiles.corporate]
# This will only be executed when activating the `corporate` 
hooks = [ "echo \"Welcome to evil corp\"" ]
```

### Limitations

- Hooks run in a sub-shell therefore, **command meant to change your current shell environment won't work** :

```toml
hooks = [ "source /home/user/.zshrc" ] # This does not work ! 
```

- Environment variables won't be expanded unless you explicitly call a sub-shell :

```toml
hooks = [ "echo $HOME" ] # This will print "$HOME"
```

```toml
hooks = [ "zsh -c \"echo $HOME\"" ] # This works
```
