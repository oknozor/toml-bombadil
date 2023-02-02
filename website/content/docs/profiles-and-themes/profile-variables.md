+++
title = "Profile variables"
description = "Manage Bombadil Profile variables"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 3
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "In addition to dotfiles overriding, Bombadil profiles supports variable overrides."
toc = true
top = false
+++

### Create a dotfile template

Let's assume you have the following in your `.bashrc` dotfile:

```bash
# ~/bombadil-example/bashrc
export JAVA_HOME={{java_home}}
```

And your default profiles variable look like this:

```toml
# ~/bombadil-example/vars.toml
[java]
home = "/etc/java-openjdk"
```

Here is our bombadil config:
```toml
dotfile_dir = "bombadil-example"

[settings]
vars = [ "vars.toml" ]

[settings.dots]
bashrc = { source = "bashrc", target = ".bashrc" }
```

So far we have defined a variable for `$JAVA_HOME` and we are using it once.
Not very useful.

### Override default variable

Now that we have declared some variables in the default profile, we can override it using a new profile:

```toml
dotfile_dir = "bombadil-example"

[settings]
vars = [ "vars.toml" ]

[settings.dots]
bashrc = { source = "bashrc", target = ".bashrc" }

[profiles.corporate]
vars = [ "java10-vars.toml" ]
```

The profile variable file will be loaded after the default one and any matching variable name will override the default:

```toml
[java]
home = "/etc/java10-openjdk"
```

Running `bombadil link -p corporate` would now produce the following `.bashrc` :
```bash
export JAVA_HOME=/etc/java10-openjdk
```

This concludes the chapter on profiles and themes, in the next chapter we will talk about hooks.
