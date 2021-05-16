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
lead = "Install and configure Toml Bombadil"
toc = true
top = false
+++

## Variables


### Variables

Now that your dot files are symlinked with Bombadil you can define some variables.
A Bombadil var files is a valid toml file containing only key with string values :

For example you have the following file in `{dotfiles_dir}/vars.toml`.

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

You can use the var file by adding the following to your Bombadil config :
```toml
[settings]
vars = [ "vars.toml" ]
```

Let's say you have the following dot entry :
```toml
[settings.dots]
alacritty = { source = "alacritty", target = ".config/alacritty/alacritty.yml" }
```

`alacritty.yaml` color scheme could look like this :
```yaml
...
# {dotfiles}/alacritty.yml
colors:
   primary:
       background: "__[background]__"
       foreground: "__[foreground]__"
   cursor:
       text: "__[text]__"
       cursor: "__[cursor]__"
...
```

The output file actually linked to alacritty config would be this :

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
...
```

To update variables, and the current config simply run `bombadil link`.

