+++
title = "Introducing Toml Bombadil v3"
description = "Toml Bombadil, a dotfile manager written in rust, helping you manage your dotfiles across several machines"
date = 2022-02-24
updated = 2022-02-24
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["Nukesor"]

[extra]
lead = """
"""
+++

If you don't know what Toml Bombadil is you might want to read the [introduction](../../docs/getting-started/introduction/) and
[quick start guide](../../docs/getting-started/quick-start/).

For those who used the previous version, here are the key feature coming with this version. 


## Features

Toml Bombadil now uses the [tera](https://tera.netlify.app/) engine for a full-featured templating experience.

`tera` is inspired by [Jinja](https://jinja.palletsprojects.com/en/3.0.x/) and thereby also uses the well-known `{{ }}` syntax for variable substitution and `{% %}` for logical expressions.

You're now able to write dotfiles with proper in-file templating logic.
For instance, you can now write configs like this:

```toml
...
[[block]]
block = "hueshift"
step = 50
hue_shifter = "redshift"

{% if profile == "laptop" %}
# Only include the battery status in the laptop config.
[[block]]
block = "battery"
device = "BAT0"
driver = "sysfs"
format = "Bat0: {percentage}% {time}"
{% endif %}

[[block]]
block = "sound"
step_width = 2
...
```

Read the `tera` documentation to get a better understanding of what's now possible.

## Breaking changes

The old templating syntax `__[]__` doesn't work any longer.
We know that this is a huge breaking change, but we assumed that this'll be a net positive for the project in the long run.
 
## Thanks

Toml Bombadil is slowly but surely gaining popularity thanks to every person that contributed to Toml Bombadil:
- [@Svenstaro](https://github.com/svenstaro)
- [@Nukesor](https://github.com/nukesor)
- [@dtolnay](https://github.com/dtolnay)
- [@HaoZeke](https://github.com/HaoZeke)
- [@kakawait](https://github.com/kakawait)
- [@DSpeckhals](https://github.com/DSpeckhals)
- [@mrkajetanp](https://github.com/mrkajetanp)

Special thanks to the creator and maintainer of Toml-Bombadil [oknozor](https://github.com/oknozor). It has been a lot of fun working on this project.
