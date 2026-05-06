#!/usr/bin/env python3
"""Retired GLib backend entry point.

The impl-glib-core phase builds libglib-2.0 from the Rust crate directly. This
script remains only as a stable tombstone for older automation that may try to
invoke the former helper.
"""

import sys


def main() -> None:
    sys.stderr.write("retired GLib helper: libglib core is built by the Rust crate\n")
    raise SystemExit(2)


if __name__ == "__main__":
    main()
