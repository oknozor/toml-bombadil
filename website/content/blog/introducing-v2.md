+++
title = "Introducing toml Bombadil v2"
description = "Introducing Toml Bombadil, a dotfile manager written in rust, helping you manage your dotfiles across several machines"
date = 2021-05-25
updated = 2021-05-25
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["Oknozor"]

[extra]
lead = """
It's been a few months since I last worked on Toml Bombadil, I was working on other project and add few time to
make improvements here. Recently I had some spare time to work again on the next version. It is now ready to be released ! 
"""
+++

If you don't know what Toml Bombadil is you might want to read the [introduction](../../docs/getting-started/introduction/) and
[quick start guide](../../docs/getting-started/quick-start/).

For those who uses the previous version, here are the key feature coming with this version. 

## Breaking changes

- GPG secret are now stored alongside variables. If you previously defined bombadil's secret the API has been completely
  rewritten from scratch, You might want to check the [documentation](../../docs/template-and-variables/gpg-secret/)
  
## Features

- `bombadil unlink` command was added to completely remove symlinks.
  
- Bombadil now has a `get` command to display dots, hooks, path, profiles, vars and secrets. These commands are useful 
  if you want to write external script like the wofi theme switcher written by [@DSpeckhals](https://github.com/DSpeckhals).
    
- Shell completion can now be generated using `bombadil generate-completions`.

- Dot entries can have their separate scoped variable file.

## Tests

- We now have a dockerized [bats](https://github.com/bats-core/bats-core) integration test suite, this should enable us
  to quickly reproduce bug in the future.
  
## Documentation

As you can see, we now have a beautiful website powered by [zola](https://www.getzola.org/) and the 
[adidoks](https://github.com/aaranxu/adidoks) theme.


## Thanks

Toml Bombadil is slowly but surely gaining popularity thanks to every people who contributed to Toml Bombadil :
- [@DSpeckhals](https://github.com/DSpeckhals)
- [@mrkajetanp](https://github.com/mrkajetanp)

Special thanks to [lucas-dclrcq](https://github.com/lucas-dclrcq) who has been helping me design Toml Bombadil from the start.





