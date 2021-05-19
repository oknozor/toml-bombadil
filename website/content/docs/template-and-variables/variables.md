+++
title = "Dotfile templates"
description = "Setup variables and dotfile templates"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 1
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = """
Toml Bombadil has a tiny template engine. Turning dotfiles into templates is meant to make theme editing, environment managment
and ricing smoother. 
"""
toc = true
top = false
+++

### Source variables

To use variables in bombadil you need to source your variable files in bombadil's config :

```toml
[settings]
vars = [ "vars.toml" ]
```

You can split variables into multiple files : 

```toml
[settings]
vars = [ "colors.toml", "env_vars.toml" ]
```

### Declare variables

A Bombadil var files is a toml file containing key with string values only.

For example, you have the following file in `{dotfiles_dir}/vars.toml`.

```toml
terminal = "alacritty"
background = "#292C3E"
foreground = "#EBEBEB"
text = "#FF261E"
cursor = "#FF261E"
black = "#0d0d0d"
red = "#FF301B"
green = "#A0E521"
yellow = "#FFC620"
blue = "#1BA6FA"
```

Given the following dot entry : 
```toml
[settings.dots]
alacritty = { source = "alacritty.yml", target = ".config/alacritty/alacritty.yml" }
```

The `source` attributes point to a template dotfile named `alacritty.yaml`. We can use the previously defined variables
using the `__[variable_name]__` syntax :

```yaml
# {dotfiles}/alacritty.yml
colors:
   primary:
       background: "__[background]__"
       foreground: "__[foreground]__"
   cursor:
       text: "__[text]__"
       cursor: "__[cursor]__"
```

### Rendering

To inject your variables, simply run `bombadil link`. Templates will be rendered to the `.dots` directory, then symlinked
according to your dots config.


In the previous example the output file actually linked to alacritty's config would look like this :

```yaml
...
# {dotfiles}/.dots/alacritty.yml
colors:
  primary:
    background: "#292C3E"
    foreground: "#EBEBEB"
  cursor:
    text: "#FF261E"
    cursor: "#FF261E"
# ...
```

In the next section we will see how to organize our variables to make reusable structured themes using variable references. 
