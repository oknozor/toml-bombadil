+++
title = "Introduction"
description = "Toml Bombadil is a dotfile manager with templating written in Rust"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 1
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Toml Bombadil is a dotfile manager with templating written in Rust"
toc = true
top = false
+++

## What are dotfiles anyway?

If you don't know what dotfiles are, you probably what to read [this](https://www.anishathalye.com/2014/08/03/managing-your-dotfiles/).

## Why Another dotfile manager

I wrote Toml Bombadil because I kept changing my desktop environment : switching from i3 to sway, from sway to xfce,
from xfce to gnome and back to sway. When you keep changing your working environment like this you end up with several problems :
- Some symlinks will end up orphans.
- Not every program you use support Xresources and you will most probably have to manually edit some themes/config.
- When starting a fresh installation you will very likely need to adapt your existing dotfiles to your new machine.
- It is a mess!

Toml Bombadil try to solve this with a simple addition to the symlink method used by other tools:
instead of creating a symlink from a dotfile to the actual config path of a program, it will create a copy of it and
symlink the copy. This additional step allow to use your original dotfile as a template and inject variables in the copy.
You can have multiple value files in the same dotfile repository and change color scheme, or any value on the fly.

While Toml Bombadil has all those features available you could start using it only to generate symlinks and templatize
your dot file progressively.

## Alternatives

The [awesome-dotfiles](https://www.anishathalye.com/2014/08/03/managing-your-dotfiles/)
repo maintain a list of dotfile managers.

Before writing Toml Bombadil, I was using [dotbot](https://github.com/anishathalye/dotbot), it's easy to configure and simple to use.

I wanted something a bit more configurable but [chezmoi](https://www.chezmoi.io/) felt way too complicated.

If you want the heavy artillery, chezmoi is probably what you are looking for.

## Quick Start

One page summary of how to get Toml Bombadil up and running. [Quick Start →](../quick-start/)

## Contributing

Find out how to contribute to Toml Bombadil. [Contributing →](../../contributing/how-to-contribute/)
