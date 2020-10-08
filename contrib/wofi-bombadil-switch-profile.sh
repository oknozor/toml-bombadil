#!/usr/bin/env sh

# Allow switching profiles using the wofi launcher/menu program (https://hg.sr.ht/~scoopta/wofi).

if ! command -v bombadil &> /dev/null; then
    echo "bombadil could not be found" >&2
    exit 1
fi

if ! command -v wofi &> /dev/null; then
    echo "wofi could not be found" >&2
    exit 1
fi

SELECTED_PROFILE=$(bombadil get profiles | wofi -i -d -p "Switch Toml Bombadil Profile:")

if [ "$SELECTED_PROFILE" = "default" ]; then
    bombadil link
else
    bombadil link -p "$SELECTED_PROFILE"
fi
