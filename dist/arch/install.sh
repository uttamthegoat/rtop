#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

bash build.sh

if pacman -Qi rtop &>/dev/null; then
  echo "Removing existing rtop package..."
  sudo pacman -R --noconfirm rtop
fi

makepkg -sf

sudo pacman -U ./rtop-0.1.0-*.pkg.tar.zst
