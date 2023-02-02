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
Toml Bombadil uses the <a href="https://tera.netlify.app/">tera templating engine</a>.
Turning dotfiles into templates is meant to make theme editing, environment managment and ricing smoother. 
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

A Bombadil var files is a normal toml file.  

For example, you have the following file in `{dotfiles_dir}/vars.toml`.

```toml
[apps]
terminal = "alacritty"
bar = "waybar"

[theme.colors]
background = "#292C3E"
foreground = "#EBEBEB"
text = "#FF261E"
cursor = "#FF261E"
black = "#0d0d0d"
red = "#FF301B"
green = "#A0E521"
yellow = "#FFC620"
blue = "#1BA6FA"

[theme.settings]
set_cursor_color = true
```

Given the following dot entry : 
```toml
[settings.dots]
alacritty = { source = "alacritty.yml", target = ".config/alacritty/alacritty.yml" }
```

The `source` attributes point to a template dotfile named `alacritty.yaml`.
We can use the previously defined variables using the `{{variable_name}}` syntax :

```yaml
colors:
   primary:
       background: "{{theme.colors.background}}"
       foreground: "{{theme.colors.foreground}}"
{%- if theme.settings.set_cursor_color == "true" %}
   cursor:
       text: "{{theme.colors.text}}"
       cursor: "{{theme.colors.cursor}}"
{% endif -%}
```

### Rendering

To inject your variables, simply run `bombadil link`.
Templates will be rendered to the `.dots` directory, then symlinked according to your dots config.


In the previous example the output file actually linked to alacritty's config would look like this:

```yaml
colors:
  primary:
    background: "#292C3E"
    foreground: "#EBEBEB"
  cursor:
    text: "#FF261E"
    cursor: "#FF261E"
```

In the next section we will see how to organize our variables to make reusable structured themes using variable references. 
