# Toml Bombadil 
![CI](https://github.com/oknozor/toml-bombadil/workflows/CI/badge.svg?branch=master)
[![codecov](https://codecov.io/gh/oknozor/toml-bombadil/branch/master/graph/badge.svg)](https://codecov.io/gh/oknozor/toml-bombadil)
![crates.io](https://img.shields.io/crates/v/toml-bombadil.svg)
![aur](https://img.shields.io/aur/version/bombadil-bin)

Toml Bombadil is a dotfile manager written in rust. 

##  Why another dotfile manager ? 

I wrote Toml Bombadil because I kept changing my desktop environment : 
switching from i3 to sway, from sway to xfce, from xfce to gnome and back to sway.
When you keep changing your working environment like this you end up with several problems : 
- Some symlinks will end up orphans. 
- Not every program you use support Xresources and you will most probably have to manually edit some themes/config. 
- When starting a fresh installation you will very likely need to adapt your existing dotfiles to your new machine.
- It is a mess 

Toml Bombadil try to solve this with a simple addition to the symlink method used by other tools : instead of creating 
a symlink from a dotfile to the actual config path of a program, it will create a copy of it and symlink the copy. 
This additional step allow to use your original dotfile as a template and inject variables in the copy. 
You can have multiple value files in the same dotfile repository and change color scheme, or any value on the fly.

In addition this is completely optional, you could start using Toml Bombadil only to generate symlinks and templatize 
your dot file progressively. 

## Table of contents
 - [Installation](#Installation)
    - [Cargo](#using-cargo)
    - [Archlinux](#archlinux)
 - [Getting started](#getting-started)
 - [Dotfile Templates](#dotfile-templates)
    - [Variables](#variables)
    - [Variable references](#variable-references)
 - [Switching profile](#switching-profile)
    - [Overriding dot entries](#overriding-dot-entries)
    - [Adding variables](#adding-variables)
    - [Adding hooks](#adding-hooks)
 - [Hooks](#hooks)
    - [Limitations](#limitations)
 - [Managing imports](#managing-imports)
 - [Example repositories](#example-repositories)
 - [Contributing](#contributing)
 - [License](#license)

## Installation 

### Using [cargo](https://doc.rust-lang.org/cargo/)

```shell script
cargo install toml-bombadil
```

### Archlinux 
```shell script
yay -S bombadil-bin
```

## Getting started

**1. Setup :** 

```shell script
git clone https://github.com/my_org/dotfiles
cd my_dotfiles && touch bombadil.toml
```

If you are using git you might want to add `.dots` to your `.gitignore`. 

**2. Configuration :**

Toml bombadil needs a toml config file, along this readme we will call it `bombadil.toml`. 

```toml
# {dotfiles}/bombadil.toml

# Path to your dotfiles relative to your $HOME directory
dotfiles_dir = "my_dotfiles"
[settings]
# An array of toml files paths containing the variables to inject in your templatize dotfiles
# You can have multiple var files as long as variable names does not colide. 
vars = [ "vars.toml" ]

# An array of post install shell commands
hooks = [ "sway reload" ]
[settings.dots]
# A dot entry representing a symlink, `source` is relative to `dotfiles_dir` 
# and `target` shall be relative to $HOME directory or absolute.
sway = { source = "sway", target = ".config/sway" }
# You can have as many dot entry as you want, linking files or directories
alacritty = { source = "alacritty", target = ".config/alacritty/alacritty.yml" }
```

**3. Linking bombadil :**

For Bombadil to be able to run from any directory and use different config files we need to symlink bombadil config to 
`$XDG_CONFIG_DIR/bombadil.toml` : 

```shell script
bombadil install -c my_dotfiles/bombadil.toml
```

If you want to switch to another config simply run : 
```shell script
bombadil install -c my_dotfiles/bombadil-i3.toml
``` 

**4. Install template and symlink :**

```shell script
bombadil link
```

This command will do the following : 
- Remove `{dotfiles_dir}/.dots` and any symlink pointing to a sub directory/file.
- Inject variables (if you defined some) in a copy of dot entries listed in Bombadil config.
- Write the copies to `{dotfiles_dir}/.dots`.
- Symlink dot entries.
- Run post install hooks.

## Dotfile Templates

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

And the output file actually linked to alacritty config would be this :

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

To update variables and the current config simply run `bombadil link`.

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

## Switching profile

As we saw Bombadil allows to define a default profile. For some programs you might want to 
set an alternate configuration.

Bombadil allow you two do this in several ways : 
- overriding a dot entry `source` and/or `target` value.
- adding a new dot entry.
- adding new variables.
- overriding existing variable.
- adding hooks to the profile. 

### Overriding dot entries

Let's say you are using [maven](https://maven.apache.org/) for several java projects, some of them are open source 
and some of them uses a corporate repository : 


let's assume your dotfiles are the following : 

```shell script
~/bombadil-example
❯ tree
.
├── bombadil.toml
└── maven
    ├── settings.corporate.xml
    └── settings.xml
```

Your bombadil config contains a single dot entry with an alternate profile : 

```toml
# bombadil.toml
dotfile_dir = "bombadil-example"
[settings.dots]
maven = { source = "maven/settings.xml", target = ".m2/settings.xml"}

[profiles.corporate.dots]
maven = { source = "maven/settings.corporate.xml" }
```

When overriding a default dot entry under a new profile `source` and `target` property are optional,
the default profile value will be used if not specified. You can also define a new dot entry in which case `source`
and `target` are required. 

If you now run `bombadil link --help` you should notice a new profile value is available : 

```
USAGE:
    bombadil link

OPTIONS:
    -p, --profiles <PROFILES>...    A list of comma separated profiles to activate [possible values: corporate]
    -h, --help                      Prints help information
```

`bombadil link` would produce the following link : 
```shell script
❯ bombadil link
"/home/okno/dotfiles/.dots/maven/settings.xml" => "/home/okno/.m2/settings.xml"
```

Linking with the `corporate` profile would use the alternate source for `.m2/settings.xml` : 
```shell script
❯ bombadil link -p corporate
"/home/okno/dotfiles/.dots/maven/settings.corporate.xml" => "/home/okno/.m2/settings.xml"
```

### Adding variables 

Here is an example bombadil config : 

```shell script
~/bombadil-example
❯ tree
.
├── bashrc
├── bombadil.toml
├── java10-vars.toml
└── vars.toml
```

Adding or overriding variables can be done this way :

```toml
# bombadil.toml
dotfile_dir = "bombadil-example"
[settings]
vars = [ "vars.toml" ]

[settings.dots]
bashrc = { source = "bashrc", target = ".bashrc"}

[profiles.corporate]
vars = [ "java10-vars.toml" ]
``` 

```shell script
# ~/bombadil-example/bashrc
export JAVA_HOME=__[java_home]__
# ...
```

```shell script
# ~/bombadil-example/vars.toml
java_home = "/etc/java-openjdk"
# ...
```

```shell script
# ~/bombadil-example/java10-vars.toml
java_home = "/etc/java10-openjdk"
# ...
```

Running `bombadil link -p corporate` would produce the following `.bashrc` : 
```shell script
export JAVA_HOME=/etc/java10-openjdk
```

### Adding hooks 

To add hooks for a profile simply add them under the `profiles.{profile_name}` section. Note that the default ones will
alway be run. 

```toml
# bombadil.toml
dotfile_dir = "bombadil-example"
[settings]
hooks = [ "echo \"default profile\"" ]

[profiles.corporate]
hooks = [ "echo \"corporate profile\"" ]
``` 


## Hooks

So far we have no talked about hooks, as we saw they can be invoked as an entry in the config : 

```toml
dotfiles_dir = "bombadil-example.toml"
[settings]
hooks = [ "sway reload" ]
```

This will invoke the `sway reload` command after `bombadil link` has updated your dotfiles.

The default hooks will always be run regardless of the activated profiles.
You can also add hooks for a specific profile.

```toml
dotfiles_dir = "bombadil-example.toml"

[settings]
# This reside in the default profile an will always be executed after bombadil link
hooks = [ "sway reload", "echo 42" ]

[profiles.corporate]
# This will only be executed when activating the `corporate` 
hooks = [ "echo \"Welcome to evil corp\"" ]
```

### Limitations

- Hook are run in a sub-shell therefore, *command meant to change your current shell environment won't work* :

```toml
hooks = [ "source /home/user/.zshrc" ] # This does not work ! 
```

- Environment variable won't be expanded unless you explicitly call a sub-shell : 

```toml
hooks = [ "echo $HOME" ] # This will print "$HOME"
```

```toml
hooks = [ "zsh -c \"echo $HOME\"" ] # This works
```

## Managing imports

As your dotfiles configuration grows bigger, it might be useful to split it into multiple files. 
To achieve this you can use the `[[import]]` option in your bombadil config : 

```toml
# bombadil.toml
dotfiles_dir = "bombadil-example"

[settings]
hooks = ["sway reload"]

[settings.dots]
alacritty = { source = "alacritty", target = ".config/alacritty" }
wofi = { source = "wofi", target = ".config/wofi" }
sway = { source = "sway", target = ".config/sway" }
waybar = { source = "waybar", target = ".config/waybar" }

[[import]]
path = "bombadil-example/shell-config.toml"
```

Given the following imported file, both config will be merged before running any bombadil command.

```toml
# bombadil/shell-config.toml
[settings]

vars = [ "bombadil-example/vars.toml" ]

[settings.dots]
zsh = { source = "zsh/zshrc", target = ".zshrc" }
zsh_env = { source = "zsh/zshenv", target = ".zshenv" }
starship = { source = "zsh/starship.toml", target = ".config/starhip.toml" }
```

## Example repositories

If you use Bombadil please submit an issue or a PR to update this section, we will be happy to reference your dotfiles here !
 
- [https://github.com/oknozor/dotfiles](https://github.com/oknozor/dotfiles)
  
## Contributing

Found a bug, have a suggestion for a new feature ? 
Please read the [contribution guideline](CONTRIBUTING.md) and submit an [issue](https://github.com/oknozor/toml-bombadil/issues). 

## License

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.




