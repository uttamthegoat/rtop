#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"
PKGVER="0.1.0"
PROJECT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || realpath "$PWD/../..")"

echo "Creating source tarball rtop-$PKGVER.tar.gz ..."
cd "$PROJECT_ROOT"
git archive --format=tar.gz --output="dist/arch/rtop-$PKGVER.tar.gz" HEAD 2>/dev/null || \
  tar czf "dist/arch/rtop-$PKGVER.tar.gz" \
    --exclude=target \
    --exclude=dist \
    --exclude=.git \
    --transform="s|^\.|rtop-$PKGVER|" \
    -C "$PROJECT_ROOT" .

echo "Done: dist/arch/rtop-$PKGVER.tar.gz"
