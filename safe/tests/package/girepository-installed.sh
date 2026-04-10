#!/bin/sh
set -eu

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

multiarch="$(dpkg-architecture -qDEB_HOST_MULTIARCH)"
girdir="$(pkg-config --variable=girdir girepository-2.0)"
typelibdir="$(pkg-config --variable=typelibdir girepository-2.0)"
gir="$girdir/GIRepository-3.0.gir"
installed_typelib="$typelibdir/GIRepository-3.0.typelib"
compiled_typelib="$tmpdir/GIRepository-3.0.typelib"
decompiled_gir="$tmpdir/GIRepository-3.0.decompiled.gir"
inspect_env="GI_TYPELIB_PATH=$tmpdir:$typelibdir${GI_TYPELIB_PATH:+:$GI_TYPELIB_PATH}"

[ "$girdir" = "/usr/share/gir-1.0" ]
[ "$typelibdir" = "/usr/lib/$multiarch/girepository-1.0" ]
[ -f "$gir" ]
[ -f "$installed_typelib" ]

for tool in gi-compile-repository gi-decompile-typelib gi-inspect-typelib; do
  resolved="$(command -v "$tool")"
  [ "$resolved" = "/usr/bin/$tool" ]
  real_path="$(readlink -f "$resolved")"
  [ "$real_path" = "/usr/lib/$multiarch/glib-2.0/$tool" ]
  case "$resolved:$real_path" in
    *"/src/"*)
      echo "$tool resolved into the source tree" >&2
      exit 1
      ;;
  esac
done

cc \
  $(pkg-config --cflags girepository-2.0) \
  /src/safe/tests/package/girepository-consumer.c \
  -o "$tmpdir/girepository-consumer" \
  $(pkg-config --libs girepository-2.0)

"$tmpdir/girepository-consumer"

gi-compile-repository "$gir" --output "$compiled_typelib"
[ -s "$compiled_typelib" ]

env $inspect_env gi-inspect-typelib --print-shlibs GIRepository --typelib-version=3.0 >"$tmpdir/inspect-shlibs.txt"
grep -qx 'shlib: libgirepository-2.0.so.0' "$tmpdir/inspect-shlibs.txt"

env $inspect_env gi-inspect-typelib --print-typelibs GIRepository --typelib-version=3.0 >"$tmpdir/inspect-typelibs.txt"
grep -qx 'typelib: GLib-2.0' "$tmpdir/inspect-typelibs.txt"
grep -qx 'typelib: GObject-2.0' "$tmpdir/inspect-typelibs.txt"
grep -qx 'typelib: Gio-2.0' "$tmpdir/inspect-typelibs.txt"
grep -qx 'typelib: GModule-2.0' "$tmpdir/inspect-typelibs.txt"

env $inspect_env gi-decompile-typelib --output "$decompiled_gir" "$compiled_typelib"
grep -q '<namespace name="GIRepository" version="3.0"' "$decompiled_gir"
