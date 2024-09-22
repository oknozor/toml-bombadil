---
prev:
  text: 'Profiles'
  link: '/guide/profiles'
next:
  text: 'Imports'
  link: '/guide/imports'
---

# Hooks

Bombadil's hooks are shell commands invoked before and after your dotfiles have been symlinked.
They are useful if you need to reload some component manually after updating your dotfiles.

## Default hooks

Hooks are defined under the default profile section in `bombadil.toml`. In the example above,
sway reload will run when running bombadil link to update any changes made to sway UI.

```toml
dotfiles_dir = "bombadil-example.toml"

[settings]
prehooks = [ "echo \"Updating dots\""]
posthooks = [ "sway reload" ]
```

::: tip
Default hooks will always run regardless of the activated profiles.
:::

## Per profile hooks

If you maintain a profile per window manager you might want to leave the default profile
hooks empty and manage per profile hooks :

```toml
[settings]
prehooks = [ "echo \"Updating dots\""]
posthooks = [ "echo \"Default profile\"" ]

[profiles.sway]
prehooks = [ "echo \"Sway profile\"" ]
posthooks = [ "sway reload" ]

[profiles.i3]
prehooks = [ "echo \"i3 profile\"" ]
posthooks = [ "i3-msg reload" ]
```
