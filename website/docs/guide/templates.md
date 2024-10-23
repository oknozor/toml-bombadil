---
next:
  text: 'Profiles'
  link: '/guide/profiles'
---

  # Dotfile templates
Toml Bombadil uses the tera templating engine. Turning dotfiles into templates is meant to make theme
editing, environment managment and ricing smoother.

## Source variables

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

## Declare variables

A Bombadil var files cant contains any value. Those will be used to generate template contexts:

```toml
[desktop]
wallpaper = "$HOME/dotfiles/wallpapers/sea.jpg"
monitor = "HDMI-A-1"

[theme]
font = "Fira Code 10"
background = "#002b36"
foreground = "#839496"
text = "#002b36"
cursor = "#839496"
black = "#073642"
red = "#dc322f"
green = "#859900"
yellow = "#b58900"
blue = "#268bd2"
magenta = "#d33682"
cyan = "#2aa198"
white = "#eee8d5"

[alacritty]
set_cursor = true
```

Variable files declared in your `bombadil.toml` config will be used to populate the template context:

```toml
[settings.dots]
alacritty = { source = "alacritty.yml", target = ".config/alacritty/alacritty.yml" }
```

The source attributes point to a template dotfile named `alacritty.yaml`.
We can use the previously defined variables using [tera syntax](https://keats.github.io/tera/docs/#introduction):

```yaml
# ~/dotfiles/alacritty.yml
colors:
   primary:
       background: "{{theme.background}}"
       foreground: "{{theme.foreground}}"
  {%- if alacritty.set_cursor == "true" %}
   cursor:
       text: "{{text}}"
       cursor: "{{cursor}}"
  {% endif -%}
```

## Default variables

By default, Bombadil automatically add some variable to your template context.

Using the `bombadil get vars` command to see your current template context, you should see the following output:

```json
{
  "os": "macos",
  "arch": "aarch64"
}
```

Here are the list of possible value for those default variables:
- `os` : https://doc.rust-lang.org/std/env/consts/constant.OS.html
- `arch` : https://doc.rust-lang.org/std/env/consts/constant.ARCH.html


## Render templates

To render and link templates, simply run `bombadil link`. Templates will be rendered to the `.dots` directory,
then symlinked according to your `bombadil.toml` config.

In the previous example the output file actually linked to alacritty's config would look like this:

```yaml
# ~/dotfiles/alacritty.yml
colors:
  primary:
    background: "#002b36"
    foreground: "#002b36"
  cursor:
    text: "#002b36"
    cursor: "#839496"
```
### Variable co-location

It is perfectly fine to use only var files using `[settings.vars]` to manage themes and profile.
But, as your dotfiles repository grow, this can quickly become a tedious process.
To provide isolation between dotfiles vars, you can use the dot `var` attribute.

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

Running `bombadil link` will render variables defined in `wofi/vars.toml` only for templates that resides in `wofi/`.

## Explicitly declare dot entry variables

The previous example works fine as long as our dotfile source is a directory.
Bombadil will look for a variable file named `vars.toml` in the source directory and everything
will be rendered accordingly when running bombadil link.

What if we are linking a file directly ?

```toml
zsh = { source = "zsh/zshrc", target = ".zshrc" }
```

If we are linking a single file instead of a directory, looking
for a default variable file here would be confusing and might collide
with other dotfile entries residing in the `zsh/` source directory.

To solve this we can explicitly declare a variable file for our dot entry :

```toml
zsh = { source = "zsh/zshrc", target = ".zshrc", vars = "shell_vars.toml" }
```

::: tip
Note that if you prefer having every single variable files named, this also works for dotfile directory.
:::
