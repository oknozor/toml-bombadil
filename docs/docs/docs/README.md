## Dotfile templates
Toml Bombadil uses the tera templating engine. Turning dotfiles into templates is meant to make theme 
editing, environment managment and ricing smoother.

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

### Render templates

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

### Explicitly declare dot entry variables

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

## Encrypted secret

::: warning
To use encryption, you need to have gnupg installed, and a pair of gpg keys.

Encrypted value will be stored in your variable file, but once rendered, secret will be in clear in `.dots/` 
directory. Before going further ensure `.dots/` is in your dotfiles repository .gitignore.
::: 

### Configuration

Add your gpg user id to `bombadil.toml`: 

```toml
dotfile_dir = "bombadil-example"

# The gpg user associated with the key pair you want to use
gpg_user_id = "me@example.org" 

vars = [ "vars.toml" ]
```

### Adding secrets

```bash
bombadil add-secret -k "server_password" -v "hunter2" -f vars.toml
```

Alternatively If you want to avoid having secrets in your shell history :

```bash
bombadil add-secret -k "server_password" -f vars.toml --ask
```

::: tip
Note that from now on bombadil will prompt for your GPG key password each time you link dot entries. 
Make sure to configure the desired [pinentry](~/.gnupg/gpg-agent.conf) program in `~/.gnupg/gpg-agent.conf`.

```bash
# File: /home/okno/.gnupg/gpg-agent.conf
pinentry-program /usr/bin/pinentry-gnome3
# ...
```
:::

## Profiles

As we saw Bombadil allows to define a default profile. 
For some programs you might want to set an alternate configuration.

Bombadil allow you two do this in several ways :

- override dot entries source and/or target value.
- add new dot entries.
- add or overriding variables.
- add hooks to the profile.

Before going further, let's take a look at a real life example. In the following config, we have defined some 
post-install hooks and dot entries for two profiles : `sway` and `i3`.

```toml
dotfiles_dir = "dotfiles"

[settings.dots]
# Dots linked with the default profiles, alacritty will always be linked
alacritty = { source = "alacritty", target = ".config/alacritty" }

[profiles.sway]
# Sway profile hook : running `bombadil link -p sway` will exectute `sway reload`
posthooks = ["sway reload"]

# Sway profile dot entries
[profiles.sway.dots]
sway = { source = "sway/wm", target = ".config/sway" }
wofi = { source = "sway/wofi", target = ".config/wofi" }


# i3 profile hook : running `bombadil link -p i3` will exectute `i3-msg reload`
[profiles.i3]
posthooks = ["i3-msg reload"]

# i3 profile dot entries
[profiles.i3.dots]
i3 =  { source = "i3/wm", target = ".config/i3" }
polybar =  { source = "i3/rofi", target = ".config/rofi" }
```

From there we could link either `sway` profile or `i3` running one of the following: 

```bash
bombadil link -p sway
bombadil link -p i3
```

It is also possible to combine multiple profile : 
```bash
bombadil link -p sway i3
```

### Meta profiles

When using multiple profile you will often find yourself typing command like this one: 
```bash
bombadil link -p sway rofi solarized material-icons
```

To turn this into a single profile you can use the `extra_profiles` attribute : 

```toml
[profiles.cool-sway]
extra_profiles = [ "sway", "rofi", "solarized", "material-icons"]
```

Now you can simply run: 
```bash
bombadil -p cool-sway
```

### List available profiles

To list available profiles you can run `bombadil link --help` :

```bash
❯ bombadil link --help      
Symlink a copy of your dotfiles and inject variables according to bombadil.toml settings

Usage: bombadil link [OPTIONS]

Options:
  -p, --profiles [<PROFILES>...]  A list of comma separated profiles to activate [possible values: i3, sway, leftwm, cool-sway, default-sway, ironbar, wofi, eww, waybar, hipster-sway, oldtimer-sway, onagre]
  -h, --help                      Print help
```

Alternatively the `bombadil get profiles` produce a one profiles per line output, suited for shell scripting :

```bash
❯ bombadil get profiles     
default
wofi
cool-sway
waybar
ironbar
eww
onagre
i3
hipster-sway
default-sway
oldtimer-sway
leftwm
sway
```


### Dot overrides

As we saw in the previous section Bombadil's profile can be used to link new dot files, in this chapter we will 
see how to alter existing dotfile entries.

Let's assume you use [rofi](https://github.com/davatorium/rofi) and want to use a different configuration when running
on `i3` or `sway`. 

```
~/dotfiles
├── bombadil.toml
└── rofi
    ├── rofi.i3
    └── rofi.sway
```

```toml
# Default profile dot entries
[settings.dots]
maven = { source = "rofi/rofi.i3", target = ".config/rofi"}

# Sway profile
[settings.sway.dots]
maven = { source = "rofi/rofi.sway" }
```

Notice on the `sway` profile we are redefining the rofi dot entry and only specifying the source attribute. 

### Variable overrides

Like for dotfiles variable can be overridden by profile variables: 

```bash
dotfile_dir = "dotfiles"

[settings]
vars = [ "vars.toml" ]

[settings.dots]
bashrc = { source = "bashrc", target = ".bashrc"}

[profiles.darcula]
vars = [ "darcula.toml" ]
```

With the default `vars.toml` containing the following:
```toml
# ~/dotfiles/vars.toml
[theme]
black = "#073642"
red = "#dc322f"
green = "#859900"
yellow = "#b58900"
blue = "#268bd2"
magenta = "#d33682"
cyan = "#2aa198"
white = "#eee8d5"
```

And our `darcula.toml` variables:
```toml
[theme]
black = "#000000"
red = "#ff5555"
green = "#50fa7b"
yellow = "#f1fa8c"
blue = "#caa9fa"
magenta = "#ff79c6"
cyan = "#8be9fd"
white = "#bfbfbf"
```

We can now switch between color themes: 
```bash
bombadil link # default theme
bombadil link -p darcula # darcula theme enabled
```

::: tip
Using this approach you can combine several profiles to achieve any combination. 

```bash
bombadil link -p sway darcula
bombadil link -p i3 solarized material-icon
```
::: 

### Imports

Instead of having all your configs defined in a single toml file, you can split it into multiple file:

```toml
dotfiles_dir = "dotfiles"
gpg_user_id = "me@example.org"

import = [
  { path = "wm/sway/sway.toml" },
  { path = "wm/leftwm/leftwm.toml" },
  { path = "wm/i3/i3.toml" },
  { path = "bars/bars.toml" },
  { path = "launchers/launchers.toml" },
]

[settings]
vars = [ "vars.toml"]

[settings.dots]
alacritty = { source = "terminals/alacritty", target = ".config/alacritty" }
zsh = { source = "zsh/zshrc", target = ".zshrc" }
starship = { source = "zsh/starship.toml", target = ".config/starship.toml" }
gitconfig = { source = "git/gitconfig", target = ".gitconfig" }
nvim = { source = "editors/neovim", target = ".config/nvim" }
```

Additional config will be merged with `bombadil.toml`:
```toml
# ~/dotfiles/i3/i3.toml
[profiles.i3]
posthooks = ["i3-msg reload"]

[profiles.i3.dots]
i3 =  { source = "i3/wm", target = ".config/i3" }
polybar =  { source = "i3/polybar", target = ".config/polybar" }
rofi =  { source = "i3/rofi", target = ".config/rofi" }
```


## Hooks

Bombadil's hooks are shell commands invoked before and after your dotfiles have been symlinked.
They are useful if you need to reload some component manually after updating your dotfiles.

### Default hooks

Hooks are defined under the default profile section in `bombadil.toml`. In the example above, 
sway reload will run when running bombadil link to update any changes made to sway UI.

```toml
dotfiles_dir = "bombadil-example.toml"

[settings]
prehooks = [ "echo \"Updating dots\""]
posthooks = [ "sway reload" ]
```

::: tip
Default hooks will always run regardless of the activated profiles.
:::

### Per profile hooks

If you maintain a profile per window manager you might want to leave the default profile 
hooks empty and manage per profile hooks :

```toml
[settings]
prehooks = [ "echo \"Updating dots\""]
posthooks = [ "echo \"Default profile\"" ]

[profiles.sway]
prehooks = [ "echo \"Sway profile\"" ]
posthooks = [ "sway reload" ]

[profiles.i3]
prehooks = [ "echo \"i3 profile\"" ]
posthooks = [ "i3-msg reload" ]
```