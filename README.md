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
    - [Meta variables](#meta-variables)
 - [Switching profile](#switching-profile)
    - [Switching source](#switching-source)
    - [Switching variables](#switching-variables)
 - [Hooks](#hooks)
    - [Limitations](#limitations)
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

By default Bombadil will look for a toml config file named `bombadil.toml`.

```shell script
git clone https://github.com/my_org/my_dotfiles
cd my_dotfiles && touch bombadil.toml
```

If you are using git you might want to add `.dots` to your `.gitignore`. 

**2. Configuration :**

```toml
# {dotfiles}/bombadil.toml

# Path to your dotfiles relative to your $HOME directory
dotfiles_dir = "my_dotfiles"

# A dot entry representing a symlink, `source` is relative to `dotfiles_dir` 
# and target shall be relative to $HOME directory or absolute.
[[dot]]
source = "sway"
target = ".config/sway"

# You can have as many dot entry as you want, linking files or directories
[[dot]]
source = "alacritty"
target = ".config/alacritty/alacritty.yml"

# Var hold the path to a toml file containing the variables to inject in your templatize dotfiles
# You can have multiple var files as long as variable names does not colide. 
[[var]] # Optional
path = "vars.toml"

# Meta vars holds the definitive values for aliased variables. It allows to reuse and group variables.
[[meta]] # Optional
path = "meta-vars.toml"

# Post install commands
[[hook]] # Optional
command = "sway reload"
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

Now that your dot files are symlinked with Bombadil you can define some variables A Bombadil var files
is a valid toml file containing only key with string values : 

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
[[var]]
path = "vars.toml"
```

Let's say you have the following dot entry : 
```toml
[[dot]]
source = "alacritty"
target = ".config/alacritty/alacritty.yml"
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

### Meta variables 

Sometimes it could be handy to use different variables names for the same values.
For instance if you want to define a system wide color scheme, you could define the following meta variables : 

```toml
# bombadil.toml
[[meta]] 
path = "meta_vars.toml"

[[var]]
path = "alacritty_vars.toml" 

[[var]]
path = "sway_vars.toml" 

# ... 
```

A meta variable configuration looks exactly like a variables configuration file. The only difference is that meta vars
are intended to be used in other var files : 

```toml
# meta_vars.toml
meta_red = "#ff0000"
meta_black = "#000000"
meta_green = "#008000"
```

```toml
# sway_vars.toml
sway_client_focused_background = "meta_black"
sway_client_focused_border = "#ffff00"
# ...
```

```toml
# alacritty_vars.toml
alacritty_background = "meta_black"
alacritty_cursor = "meta_green"
# ...
```

## Switching profile

As we saw Bombadil allows to define global variables for all your dotfiles. For some programs you might want to 
set alternate configurations for a single config file and change it without reloading the whole bombadil config.
Bombadil allow you two do this in two different way : by changing the source file linked against user configuration or 
by overriding variables only for this specific file.  

### Switching source

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
dotile_dir = "bombadil-example"

[[dot]]
 name = "maven" # dot entry with profiles require to be named in order to generate bombadil command
 source = "settings.xml" 
 target = ".m2/settings.xml"
    [[dot.profile]] # A profile entry for the maven dotfile (you can define as many as you want) 
     name = "corporate" # name of the profile, required to generate bombadil command
     switch.source = "settings.corporate.xml" # we are going to use alternate dot source to "settings.corporate.xml"
     hook = "echo changed mvn env" # post install hook
```

If you now run `bombadil --help` you should notice a new subcommand named after your dot entry name as been generated : 

```
USAGE:
    bombadil <SUBCOMMAND>

... 

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    install    Link a given bombadil config to XDG_CONFIG_DIR/bombadil.toml
    link       Symlink a copy of your dotfiles  and inject variables according to bombadil.toml config
    maven      User defined profile command
```

Let's now run `bombadil maven --help` : 

```
User defined profile command

USAGE:
    bombadil maven --set-profile <PROFILE>

OPTIONS:
    -s, --set-profile <PROFILE>    Switch to a valid profile defined in your bombadil config [possible values:
                                   corporate, default]
    -h, --help                     Prints help information
```

As you can see the possible values for the `--set-profile` flag contains the profile we defined and a default profile which correspond to the default source. 

We can now switch profile like so :  

```shell script
bombadil maven --set-env corporate
```

Or using the short flag version: 

```shell script
bombadil maven -s corporate
```

This will update the file linked to `$HOME/.m2/settings.xml` to use `settings.corporate.xml`. If you defined template 
variables in that file they will be replaced as if you ran the `link` command.

To revert to the default profile you can run `bomadil maven -s default`. Using `bomadil link` will also reset all defined
profiles to their default values. 

### Switching variables

Switching variables is done the same way as switching source : 

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

```shell script
# ~/bombadil-example/bashrc
export JAVA_HOME=__[java_home]__
# ...
```

```toml
# vars.toml
java_home = "/etc/java-openjdk"
```

```toml
# java10-vars.toml
java_home = "/etc/java10-openjdk"
```

Switching profile by var would be done like this : 

```toml
# bombadil.toml
dotfiles_dir = "bombadil-example"

[[dot]] 
name = "bash"
source = "bashrc"
target = ".bashrc"
[[dot.profile]]
    name = "java10"
    switch.vars = "java10-vars.toml" # This is the only difference with the switch source method  
```

We could now override the variable with our `java10` profile by running the following :

```shell script
bombadil bash -s java10
```

And switch back to default : 

```shell script
bombadil bash -s default
```

## Hooks

So far we have no talked about hooks, as we saw they can be invoked as an entry in the config : 

```toml
[[hook]]
command = "sway reload"
```

This will invoke the `sway reload` command after `bombadil link` has updated your dotfiles.

You can also define post install hook for custom profile :

```toml
dotfiles_dir = "bombadil-example"

[[dot]] 
name = "alacritty"
source = "allacritty"
target = ".config/alacritty"
[[dot.profile]]
    name = "nord"
    switch.vars = "nord-colors.toml" 
    hook = "neofetch"
```

This will run  [neofetch](https://github.com/dylanaraps/neofetch) after updating your alacritty color scheme with the 
nord color palette.

### Limitations

- Hook are run in a sub-shell therefore, command meant to change your current shell environment won't work :

```toml
[[hook]]
command = "source /home/user/.zshrc" # This does not work ! 
```

- Environment variable won't be expanded unless you explicitly call a sub-shell : 

```toml
[[hook]]
command = "echo $HOME" # This will print "$HOME" unexpanded
```

```toml
[[hook]]
command = "zsh -c \"echo $HOME\"" # This works
```

## Example repositories

If you use Bombadil please submit an issue or a PR to update this section, we will be happy to reference your dotfiles here !
 
- [https://github.com/oknozor/dotfiles](https://github.com/oknozor/dotfiles)
  
## Contributing

Found a bug, have a suggestion for a new feature ? Please submit an [issue](https://github.com/oknozor/toml-bombadil/issues). 

## License

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.




