#!/bin/sh
set -eu

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

pkg-config --exists girepository-2.0
[ -f /usr/include/glib-2.0/girepository/girepository.h ]

cc \
  $(pkg-config --cflags girepository-2.0) \
  /src/safe/tests/package/girepository-consumer.c \
  -o "$tmpdir/girepository-consumer" \
  $(pkg-config --libs girepository-2.0)

"$tmpdir/girepository-consumer"
