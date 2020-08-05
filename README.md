# Toml Bombadil 
![CI](https://github.com/oknozor/toml-bombadil/workflows/CI/badge.svg?branch=master)
[![codecov](https://codecov.io/gh/oknozor/toml-bombadil/branch/master/graph/badge.svg)](https://codecov.io/gh/oknozor/toml-bombadil)
! This is a work in progress.

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

## Getting started

1. Installation : 

By default Bombadil will look for a toml config file named `bombadil.toml`.

```sh
git clone https://github.com/my_org/my_dotfiles
cd my_dotfiles && touch bombadil.toml
```

If you are using git you might want to add `.dots` to your `.gitignore`. 

2. Configuration : 

```toml
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
[[var]]
path = "vars.toml"

# Post install commands
[[hook]]
command = "sway reload"
```

3. Linking bombadil : 

For Bombadil to be able to run from any directory and use different config files we need to symlink bombadil config to 
`$XDG_CONFIG_DIR/bombadil.toml` : 

```shell script
bombadil install -c my_dotfiles/bombadil.toml
```

If you want to switch to another config simply run : 
```shell script
bombadil install -c my_dotfiles/bombadil-i3.toml
```

4. Install template and symlink : 

```shell script
bombadil link
```

This command will do the following : 
- Remove {dotfiles_dir}/.dots and any symlink pointing to a sub directory/file
- Inject variables (if you defined some) in a copy of dot entries listed in Bombadil config
- Write the copies to {dotfiles_dir}/.dots
- Symlink dot entries
- Run post install hooks

## Templatize you dotfiles

Now that your dot files are symlinked with Bombadil you can define some variables to inject. A Bombadil var files
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
```
[[dot]]
source = "alacritty"
target = ".config/alacritty/alacritty.yml"
```

`alacritty.yaml` color scheme could look like this : 
```yaml
...
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
colors:
  primary:
    background: "#292C3E"
    foreground: "#EBEBEB"
  cursor:
    text: "#FF261E"
    cursor: "#FF261E"
...
```

To update the current config simply run `bombadil link`.







