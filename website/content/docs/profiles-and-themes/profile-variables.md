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
lead = "Manage Bombadil Profile variables"
toc = true
top = false
+++

### Adding variables

Here is an example bombadil config :

```shell script
~/bombadil-example
├── bashrc
├── bombadil.toml
├── java10-vars.toml
└── vars.toml
```

Adding or overriding variables can be done this way :

```toml
# bombadil.toml
dotfile_dir = "bombadil-example"
[settings]
vars = [ "vars.toml" ]

[settings.dots]
bashrc = { source = "bashrc", target = ".bashrc"}

[profiles.corporate]
vars = [ "java10-vars.toml" ]
``` 

```shell script
# ~/bombadil-example/bashrc
export JAVA_HOME=__[java_home]__
# ...
```

```shell script
# ~/bombadil-example/vars.toml
java_home = "/etc/java-openjdk"
# ...
```

```shell script
# ~/bombadil-example/java10-vars.toml
java_home = "/etc/java10-openjdk"
# ...
```

Running `bombadil link -p corporate` would produce the following `.bashrc` :
```shell script
export JAVA_HOME=/etc/java10-openjdk
```