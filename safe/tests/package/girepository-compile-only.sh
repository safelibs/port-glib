#!/bin/sh
set -eu

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

pkg-config --exists girepository-2.0
[ -f /usr/include/glib-2.0/girepository/girepository.h ]

cc \
  $(pkg-config --cflags girepository-2.0) \
  "$script_dir/girepository-consumer.c" \
  -o "$tmpdir/girepository-consumer" \
  $(pkg-config --libs girepository-2.0)

"$tmpdir/girepository-consumer"
