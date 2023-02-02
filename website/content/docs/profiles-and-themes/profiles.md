+++
title = "Profiles"
description = "Manage Bombadil profiles"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 1
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Manage Bombadil profiles"
toc = true
top = false
+++

## Profile configuration

As we saw Bombadil allows to define a default profile. For some programs you might want to
set alternative configurations.

Bombadil allow you two do this in several ways :
- override dot entries `source` and/or `target` value.
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

## List profiles

To list available profiles you can run `bombadil link --help` :

```
❯ bombadil link --help
bombadil-link
Symlink a copy of your dotfiles and inject variables according to bombadil.toml config

USAGE:
    bombadil link

OPTIONS:
    -h, --help                      Prints help information
    -p, --profiles <profiles>...    A list of comma separated profiles to activate [possible values: sway, i3]
```

Alternatively the `bombadil get profiles` produce a one profiles per line output, suited for shell scripting :
```
❯ bombadil get profiles
sway
i3
```

## Combine profiles 

If you manage many profiles, linking a specific combination can be tedious. 
Assuming you have a `i3`, `corporate`, `solarized-theme` profiles you want to link together you would have to type 
the following command each time you want to link your dotfiles :
```
❯ bombadil link -p i3 corporate solarized-theme
```

To avoid this you can define a combined profile like so : 
```toml
[profiles.workstation]
extra_profiles = ["i3", "corporate", "solarized-theme"]
```

Then you will just need to run the following : 
```
❯ bombadil link -p workstation
```

## Switch profile

We can switch profile by running the following :
- Link the default profile (alacritty dot only) : `bombadil link`.
- Link the default and sway config `bombadil link -p sway`.
- Link the default and  i3 config `bombadil link -p i3`.

This allows us to define per profile dot entries. In the next chapter we will see how to alter dot entries existing in
the default profile.
