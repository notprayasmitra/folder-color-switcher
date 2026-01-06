#!/bin/bash

THEME="${1:-Papirus-Dark}"

# List current color and all available colors
if [[ "$2" == "list" ]]; then
    papirus-folders -l --theme "$THEME"
fi

# Set a color
if [[ "$2" == "set" && -n "$3" ]]; then
    papirus-folders -t "$THEME" -c "$3"
fi