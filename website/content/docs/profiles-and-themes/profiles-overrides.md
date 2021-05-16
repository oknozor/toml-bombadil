+++
title = "Dot overrides"
description = "Manage profiles dot entries"
date = 2021-05-16
updated = 2021-05-16
draft = false
weight = 2
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Manage profiles dot entries"
toc = true
top = false
+++

### Overriding dot entries

Let's say you are using [maven](https://maven.apache.org/) for several java projects, some of them are open source,
and some of them uses a corporate repository :


let's assume your dotfiles have the following structure :

```shell script
~/bombadil-example
├── bombadil.toml
└── maven
    ├── settings.corporate.xml
    └── settings.xml
```

Your bombadil config contains a single dot entry with an alternate profile :

```toml
# bombadil.toml
dotfile_dir = "bombadil-example"
[settings.dots]
maven = { source = "maven/settings.xml", target = ".m2/settings.xml"}

[profiles.corporate.dots]
maven = { source = "maven/settings.corporate.xml" }
```

When overriding a default dot entry under a new profile `source` and `target` property are optional,
the default profile value will be used if not specified. You can also define a new dot entry in which case `source`
and `target` are required.

If you now run `bombadil link --help` you should notice a new profile value is available :

```
USAGE:
    bombadil link

OPTIONS:
    -p, --profiles <PROFILES>...    A list of comma separated profiles to activate [possible values: corporate]
    -h, --help                      Prints help information
```

`bombadil link` would produce the following link :
```shell script
❯ bombadil link
"/home/okno/dotfiles/.dots/maven/settings.xml" => "/home/okno/.m2/settings.xml"
```

Linking with the `corporate` profile would use the alternate source for `.m2/settings.xml` :
```shell script
❯ bombadil link -p corporate
"/home/okno/dotfiles/.dots/maven/settings.corporate.xml" => "/home/okno/.m2/settings.xml"
```