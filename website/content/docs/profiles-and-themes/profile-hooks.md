+++
title = "Profile hooks"
description = "Manage Bombadil profiles hooks"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 4
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Manage Bombadil profiles hooks"
toc = true
top = false
+++

### Adding hooks

To add hooks for a profile simply add them under the `profiles.{profile_name}` section. Note that the default ones will
always be run.

```toml
# bombadil.toml
dotfile_dir = "bombadil-example"
[settings]
hooks = [ "echo \"default profile\"" ]

[profiles.corporate]
hooks = [ "echo \"corporate profile\"" ]
```
