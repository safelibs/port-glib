#!/bin/sh
set -eu

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cc \
  $(pkg-config --cflags girepository-2.0) \
  /src/safe/tests/package/girepository-consumer.c \
  -o "$tmpdir/girepository-consumer" \
  $(pkg-config --libs girepository-2.0)

"$tmpdir/girepository-consumer"

