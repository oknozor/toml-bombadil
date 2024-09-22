---
prev:
  text: 'Hooks'
  link: '/guide/hooks'
next:
  text: 'GPG secrets'
  link: '/guide/secrets'
---

# Imports

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
