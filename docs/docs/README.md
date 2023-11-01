---
home: true
heroImage: logo.png
tagline: A Dotfile manager written in Rust
actionText: Getting Started →
actionLink: /quickstart/
features:
  - title: Dotfiles Template️
    details: Inject variables in your dotfiles to manage your dotfile state in one place.
  - title: GPG encryption
    details: Encrypt your SSH keys, passwords, corporate configs via GPG and safely commit them to your public dotfile repository.
  - title: Themes and profiles
    details: Organize your dotfiles, switch themes and working environment on the fly.
  - title: Installation hooks.
    details: Live reload your window manager components with pre and post installation hooks.
footer: MIT Licensed | Copyright © 2020 Paul Delafosse
---

### What are dotfiles anyway?

If you don't know what dotfiles are, you probably want to read this.


## Why Another dotfile manager

I wrote Toml Bombadil because I kept changing my desktop environment : switching from i3 to sway, 
from sway to xfce, from xfce to gnome and back to sway. When you keep changing your working 
environment like this you end up with several problems :

- Some symlinks will end up orphans.
- Not every program you use support Xresources and you will most probably have to manually edit some themes/config.
- When starting a fresh installation you will very likely need to adapt your existing dotfiles to your new machine.
- It is a mess!

Toml Bombadil try to solve this with a simple addition to the symlink method used by other tools: 
instead of creating a symlink from a dotfile to the actual config path of a program, it will create 
a copy of it and symlink the copy. This additional step allow to use your original dotfile as a 
template and inject variables in the copy. You can have multiple value files in the same dotfile 
repository and change color scheme, or any value on the fly.
While Toml Bombadil has all those features available you could start using it only to generate 
symlinks and templatize your dot file progressively.

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/toml-bombadil.svg)](https://repology.org/project/toml-bombadil/versions)

### Archlinux

```shell
pacman -S toml-bombadil
```

### Cargo

```shell
cargo install toml-bombadil
```


## Alternatives

The awesome-dotfiles repo maintain a list of dotfile managers.
Before writing Toml Bombadil, I was using [dotbot](https://github.com/anishathalye/dotbot), it's easy to configure and simple to use.
I wanted something a bit more configurable but [chezmoi](https://www.chezmoi.io/) felt way too complicated.
If you want the heavy artillery, chezmoi is probably what you are looking for.



## Contributing

Find out how to contribute to Toml Bombadil [-> Contributing](../contributing.md)

