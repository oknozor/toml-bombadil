+++
title = "Manage imports"
description = "Manage imports"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = """
As your config grow bigger you will probably want to split it into multiple files.
"""
toc = true
top = false
+++



## Config imports

Instead of having all your configs defined in a single toml file, you can split it into multiple file :

```toml
[settings.dots]
alacritty = { source = "alacritty", target = ".config/alacritty" }
zsh = { source = "zsh/zshrc", target = ".zshrc" }

[[import]]
path = "sway/sway.toml"

[[import]]
path = "i3/i3.toml"
```

Alternatively you can define multiple imports at once: 
```toml
dotfiles_dir = "dotfiles"

import = [
  { path = "sway/sway.toml" },
  { path = "i3/i3.toml" },
]

[settings.dots]
alacritty = { source = "terminals/alacritty", target = ".config/alacritty" }
zsh = { source = "zsh/zshrc", target = ".zshrc" }
# ...
```

In this example we have defined our `i3` and `sway` profile in separate files :

```toml
# {dotfile_dir}/i3/i3.toml
[profiles.i3]
posthooks = ["i3-msg reload"]

[profiles.i3.dots]
i3 =  { source = "i3/wm", target = ".config/i3" }
polybar =  { source = "i3/polybar", target = ".config/polybar" }
rofi =  { source = "i3/rofi", target = ".config/rofi" }
```
