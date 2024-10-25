---
prev:
  text: 'Home'
  link: '/'
next:
  text: 'User Guide'
  link: '/guide/templates'
---

# Setup

If you already have some dotfiles on a git repository, no need to start from scratch:

   ```bash
  git clone https://github.com/my_org/dotfiles
   ```

Add Toml Bombadadil config to your dotfiles:

```bash
cd my_dotfiles && touch bombadil.toml
```

Linking bombadil config:

For Bombadil to be able to run from any directory and use different config files we need to symlink 
its config to `$XDG_CONFIG_DIR/bombadil.toml`:

```bash
bombadil install my_dotfiles/
```

::: warning
Toml Bombadil will generate a copy of your dotfiles under `.dots`. If you use git to manage your dotfiles, 
you need to add `.dots` to your `.gitignore`.
:::
    
## Configuration

Toml Bombadil obviously uses the toml configuration format. 
here is a sample configuration:

```toml
# Path to your dotfile directory containing this config file (relative to $HOME). 
dotfiles_dir = "dotfiles"

# (Optional) GPG user ids for secret encryption/decryption.
gpg_user_ids = ["paul.delafosse@protonmail.com"]

# (Optional) list of bombadil config files to include in the configuration. 
import = [
   { path = "wm/sway/sway.toml" },
   { path = "wm/i3/i3.toml" },
]

[settings]
# An array of toml files paths containing the variables to inject in your templatized dotfiles.
vars = [ "vars.toml"]

# An array of post install shell commands
posthooks = [ "nvim --headless -c 'autocmd User PackerComplete quitall' -c 'PackerSync'" ]


# Dotfiles template with their respective `source` template and `target` directories.
[settings.dots]

# A dot entry representing a symlink, `source` is relative to `dotfiles_dir`
# and `target` shall be relative to $HOME directory or absolute.
alacritty = { source = "terminals/alacritty", target = ".config/alacritty" }
zsh = { source = "zsh/zshrc", target = ".zshrc" }
starship = { source = "zsh/starship.toml", target = ".config/starship.toml" }
gitconfig = { source = "git/gitconfig", target = ".gitconfig" }
```

## Linking files

Once you have written your config simply run: 

```bash
bombadil link
```

Alternatively you can use hotreload while editing templates: 
```bash
bombadil watch
```

## Workflow

Toml Bombadil behave slightly differently than other dotfiles managers: 
your dotfiles will not be directly symlinked to their target locations. 
Instead, Toml Bombadil will create a copy of your dotfiles under `.dots` 
and then symlink those copy.

The idea behind this is to inject variables into your dotfiles and allow 
you to compose various themes and profiles. 
Because of this, you will need to reload your dotfiles with `bombadil link` 
whenever you make change.

A convenient way to work with Toml Bombadil would be to add a keyboard shortcut 
for bombadil link in your window manager.

## Going further

So far we have covered the basic on how to install and symlink your dotfiles, 
but Toml Bombadil as many more features.
