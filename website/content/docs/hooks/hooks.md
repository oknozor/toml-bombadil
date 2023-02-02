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
Bombadil's hooks are shell commands invoked before and after your dotfiles have been symlinked. They are useful if you need
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
prehooks = [ "echo \"Updating dots\""]
posthooks = [ "sway reload" ]
```

The default hooks will always run regardless of the activated profiles.

## Profile hooks

If you maintain a profile per window manager you might want to leave the default profile hooks empty and manage
per profile hooks :

```toml
[settings]
prehooks = [ "echo 'Updating dots'"]
posthooks = [ "echo 'Default profile'" ]

[profiles.sway]
# Use toml multiline string to escape quotes and special characters
prehooks = [ 
  """
  echo "Sway profile"
  """ 
]

posthooks = [ "sway reload" ]

[profiles.i3]
prehooks = [ "echo 'i3 profile'" ]
posthooks = [ "i3-msg reload" ]
```

### Limitations

- Hooks run in a sub-shell therefore, **command meant to change your current shell environment won't work** :

```toml
posthooks = [ "source /home/user/.zshrc" ] # This does not work !
```

That's it for hooks, in the next chapter we will see how to split your Bombadil config into multiple files.
