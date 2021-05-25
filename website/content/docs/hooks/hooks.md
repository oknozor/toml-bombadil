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
lead = """
Bombadil's hooks are shell commands invoked after your dotfiles have been symlinked. They are useful if you need
to reload some component manually after updating your dotfiles. 
"""
toc = true
top = false
+++



## Default profile hook

Hooks are defined under the default profile section in Bombadil's configuration. In the example above, 
`sway reload` will run when running `bombadil link` to update any changes made to sway UI.

```toml
dotfiles_dir = "bombadil-example.toml"

[settings]
hooks = [ "sway reload" ]
```

The default hooks will always run regardless of the activated profiles.

## Profile hooks

If you maintain a profile per window manager you might want to leave the default profile hooks empty and manage 
per profile hooks : 

```toml
[settings]
hooks = [ "echo \"Default profile\"" ]

[profiles.sway]
hooks = [ "sway reload", "echo \"Sway profile\"" ]

[profiles.i3]
hooks = [ "i3-msg reload", "echo \"i3 profile\"" ]
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

That's it for hooks, in the next chapter we will see how to split your Bombadil config into multiple files. 
