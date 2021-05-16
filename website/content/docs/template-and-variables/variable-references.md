+++
title = "Variable reference"
description = "Manage variable reference"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 2
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Manage variable reference"
toc = true
top = false
+++

### Variable references

Sometimes it could be handy to use different variables names for the same values.
For instance if you want to define a system wide color scheme, you could define the following variables references :

```toml
# bombadil.toml
[settings]
vars = [ "vars.toml", "theme_vars.toml", "alacritty_vars.toml", "sway_vars.toml" ]
# ... 
```

By prefixing a variable value with `%` you tell bombadil to look for a variable reference.
Here `%red`, `%black` and `%green` will be replaced with the actual `red`, `black` and `green` values.

```toml
# theme_vars.toml
red = "#ff0000"
black = "#000000"
green = "#008000"
```

```toml
# sway_vars.toml
sway_client_focused_background = "%black"
sway_client_focused_border = "#ffff00"
# ...
```

```toml
# alacritty_vars.toml
alacritty_background = "%black"
alacritty_cursor = "%green"
# ...
```
