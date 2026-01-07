#!/bin/bash

set -e

THEME="${1:-Papirus-Dark}"
ACTION="$2"
COLOR="$3"

case "$ACTION" in
    list)
        papirus-folders -l --theme "$THEME"
        exit 0
        ;;
    set)
        if [[ -z "$COLOR" ]]; then
            echo "Error: No color provided" >&2
            exit 1
        fi
        papirus-folders -t "$THEME" -c "$COLOR"
        exit 0
        ;;
    *)
        echo "Usage:"
        echo "  $0 THEME list"
        echo "  $0 THEME set COLOR"
        exit 1
        ;;
esac
