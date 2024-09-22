---
prev:
  text: 'Templates'
  link: '/guide/templates'
next:
  text: 'Hooks'
  link: '/guide/hooks'
---

# Profiles

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

## Composable profiles

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

## List available profiles

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

## Dot overrides

As we saw in the previous section Bombadil's profile can be used to link new dot files, we will now
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
rofi = { source = "rofi/rofi.i3", target = ".config/rofi"}

# Sway profile
[settings.sway.dots]
rofi = { source = "rofi/rofi.sway" }
```

Notice on the `sway` profile we are redefining the rofi dot entry and only specifying the source attribute.

## Variable overrides

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
