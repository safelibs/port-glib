#!/bin/sh
set -eu

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

gir="/usr/share/gir-1.0/GIRepository-3.0.gir"
typelib="$tmpdir/GIRepository-3.0.typelib"

gi-compile-repository "$gir" --output "$typelib"
gi-inspect-typelib "$typelib" >/dev/null
gi-decompile-typelib "$typelib" >/dev/null
