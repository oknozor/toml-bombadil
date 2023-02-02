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
lead = """
As we saw on the previous chapter Bombadil's profile can be used to link new dot files, in this chapter we will
see how to alter existing dotfile entries.
"""
toc = true
top = false
+++

## Dot entry override

Let's say you are using [maven](https://maven.apache.org/) for several java projects, some of them are open source,
and some of them uses a corporate repository. Maven config typically resides in `$HOME/.m2/settings.xml`. The problem here
is that we want to use a different config depending on the project we are working on. 


To solve this we will define the following dotfiles :

```bash
~/bombadil-example
├── bombadil.toml
└── maven
    ├── settings.corporate.xml
    └── settings.xml
```

## Configuration

Bombadil's config contains a single dot entry with the alternate profile `corporate` :

```toml
# bombadil.toml
dotfile_dir = "bombadil-example"

[settings.dots]
maven = { source = "maven/settings.xml", target = ".m2/settings.xml"}

[profiles.corporate.dots]
maven = { source = "maven/settings.corporate.xml" }
```

Notice on the `corporate` profile we are redefining the `maven` dot entry and only specifying the `source` attribute.

## Linking 

Linking the default profile with `bombadil link`, will produce the following link :
```bash
bombadil link
[Created]
"/home/okno/dotfiles/.dots/maven/settings.xml" => "/home/okno/.m2/settings.xml"
```

Linking with the `corporate` profile will use the alternate source for `.m2/settings.xml` :

```bash
bombadil link -p corporate
[Created]
"/home/okno/dotfiles/.dots/maven/settings.corporate.xml" => "/home/okno/.m2/settings.xml"
```

When overriding a default dot entry under a new profile, `source` and `target` property are optional.
The default profile value will be used if not specified. 

As we saw in the previous chapter, you can also define a new
dot entry in which case `source` and `target` are required.

In the next chapter we will see how to override variables with profiles.


