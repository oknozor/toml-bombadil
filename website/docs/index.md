---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Toml Bombadil"
  text: "A dotfile manager written in Rust"
  image:
    src: /logo.png
    alt: Toml Bombadil logo
  actions:
    - theme: brand
      text: Quick start
      link: /quickstart
    - theme: alt
      text: User guide
      link: /guide/templates

features:
  - title: Dotfiles Template️
    details: Inject variables in your dotfiles to manage your dotfile state in one place.
  - title: GPG encryption
    details: Encrypt your SSH keys, passwords, corporate configs via GPG and safely commit them to your public dotfile repository.
  - title: Themes and profiles
    details: Organize your dotfiles, switch themes and working environment on the fly.
  - title: Installation hooks.
    details: Live reload your window manager components with pre and post installation hooks.
---
