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
lead = """
Variable references allows using different variable names for the same values.
Define system wide color schemes, global values for specific environments etc.
"""
toc = true
top = false
+++

### Declare variable references

A variable reference is just like any other, except the value should be a variable name prefixed with `%`.

```toml
a_variable = "42"
variable_ref = "%a_variable"
```

Here `%a_variable` points to `"42"`. Any template call to `__[variable_ref]__` will be replace by `42` when running
`bombadil link`.

### Layout example

The main idea behind variable references is to avoid repetition and inject values into multiple dotfiles.
As you will see later in this documentation, references are meant to be mixed with profiles and scoped variable.

For now let us define a simple layout. Assuming we are using sway as a window manager, and our terminal is allacritty, 
we are going to define three separate variables files : 
- A global variable file `theme_vars.toml`
- A variable file for alacritty `alacritty_vars.toml`
- A variable file for sway `sway_vars.toml`

```toml
# bombadil.toml
[settings]
vars = [ "theme_vars.toml", "alacritty_vars.toml", "sway_vars.toml" ]
# ... 
```

- `theme_vars.toml` contains all our system theme colors : 

    ```toml
    red = "#ff0000"
    black = "#000000"
    green = "#008000"
    ```
- `sway_vars.toml` contain sway specific variables, some are references, some are static. 
    ```toml
    sway_client_focused_background = "%black"
    sway_client_focused_border = "#ffff00"
    # ...
    ```

- `alacritty_vars.toml` uses references just like sway variables.

    ```toml
    alacritty_background = "%black"
    alacritty_cursor = "%green"
    # ...
    ```

In the next section we will see how to scope variables to a specific dot entry. 

