#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

if [ ! -f "rtop-0.1.0.tar.gz" ]; then
  echo "Source tarball not found. Run build.sh first."
  exit 1
fi

if pacman -Qi rtop &>/dev/null; then
  echo "Removing existing rtop package..."
  sudo pacman -R --noconfirm rtop
fi

makepkg -si "$@"
