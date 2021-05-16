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
lead = "Manage variable scope"
toc = true
top = false
+++

### Variable scopes

Although it is perfectly fine to use only var files using `settings.vars`, managing themes and profile can be a tedious
process. To provide isolation between dotfiles vars, you can use the dot `var` attribute :

By default, bombadil will look for a file named `vars.toml` in every dot entry dir.
For this example the var file is explicitly named`wofi_vars.toml` but `my_dot_dir/vars.toml` would be automatically picked.


```
dotfiles/wofi
├── config
├── solarized.toml
├── style.css
└── wofi_vars.toml
```

Here we define variables that will only be resolved when rendering `wofi` template dots.
- Global vars defined under `settings.vars` will still be accessible in wofi dotfiles.
- Global vars defined under `settings.vars` will be overridden by colliding local variables.

```toml
[settings.dots]
wofi = { source = "wofi", target = ".config/wofi", vars = "wofi_vars.toml" }
``` 

A common pattern to organize your themes and profiles would be to maintain a global variable file for each variant,
and a corresponding local variable file for each dot entry :

```toml
dotfiles_dir = "my_dotfiles"

[settings]
vars = [ "themes/default.toml" ]

[settings.dots]
wofi = { source = "wofi", target = ".config/wofi", vars = "wofi_vars.toml" }

[profiles.solarized]
# Our solarized theme overrides the default vars
vars = [ "themes/solarized.toml" ]

[[dots]]
# Override the local var path with solarized theme
wofi = { vars = "solarized.toml" }
```
