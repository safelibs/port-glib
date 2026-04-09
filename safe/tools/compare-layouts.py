#!/usr/bin/env python3
import argparse
from pathlib import Path

from common import read_json


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", action="append", type=Path, required=True)
    parser.add_argument("--build-root", type=Path, required=True)
    args = parser.parse_args()
    rust_layout_dir = args.build_root / "rust-layouts"
    if not rust_layout_dir.exists():
        raise SystemExit(f"Rust layout directory not found: {rust_layout_dir}")
    for baseline in args.baseline:
        expected = read_json(baseline)
        actual = read_json(rust_layout_dir / baseline.name)
        if expected != actual:
            raise SystemExit(f"Layout mismatch for {baseline.name}")


if __name__ == "__main__":
    main()

