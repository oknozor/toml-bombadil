+++
title = "Scope"
description = "Manage variable scope"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 3
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = """
It is perfectly fine to use only var files using [settings.vars] to manage themes and profile. But, as your dotfiles 
repository grow, this can quickly become a tedious process. To provide isolation between dotfiles vars, you can use the 
dot `var` attribute. 
"""
toc = true
top = false
+++

### Variable scopes

By default, bombadil will look for a file named `vars.toml` in every dotfile entry source directory : 

Let us assume the following dotfiles directory : 

```
dotfiles
└── bombadil.toml
└── theme.toml
└── wofi
    ├── config
    ├── style.css
    └── vars.toml

```

With `bombadil.toml` containing the above dotfile entry : 

```toml
wofi = { source = "wofi", target = ".config/wofi" }
```

And some variables defined in `wofi/vars.toml` :

```toml
# Here we use variable reference from `theme.toml`
bg = "%blue"
input_bg = "%white"
input_color = "%light_blue"
input_focused_bg = "%blue"
```

Running `bombadil link` will render variables defined in `wofi/vars.toml` only for templates that resides in `wofi/`.

### Explicitly declare scoped variables

The previous example works fine as long as our dotfile source is a directory.
Indeed, Bombadil will look for a variable file named `vars.toml` in the source directory and everything will be rendered
accordingly when running `bombadil link`.  

What if we are linking a file directly ? 

```toml
gtk3 = { source = "gtk/gtk-3.0", target = ".config/gtk-3.0" }
```

Here we are directly rendering a file instead of a directory, looking for a variable file here would be confusing and
might collide with other dotfile  entries residing in the `gtk/` source directory. 

To solve this we can explicitly declare a variable file for our dot entry : 

```toml
gtk3 = { source = "gtk/gtk-3.0", target = ".config/gtk-3.0", vars = "gtk/gtk-arc.toml" }
```

Note that if you prefer having every variable files named, this also works for dotfile directory. 

In the next section we will see how to use GPG encryption to securely manage secret values in Bombadil variables. 

