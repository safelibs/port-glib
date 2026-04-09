#!/usr/bin/env python3
import argparse
import filecmp
import shutil
from pathlib import Path

from common import SAFE_ROOT, VENDOR_ORIGINAL, path_map_entries, read_json


def sync_once(entry: dict[str, str]) -> None:
    src = SAFE_ROOT.parent / entry["canonical_prefix"]
    dst = SAFE_ROOT.parent / entry["editable_prefix"]
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src, dst, symlinks=True)


def verify_map(path: Path) -> None:
    actual = read_json(path)
    expected = path_map_entries()
    if actual != expected:
        raise ValueError("abi/test-source-path-map.json does not match the fixed seven-root contract")
    for entry in actual:
        src = SAFE_ROOT.parent / entry["canonical_prefix"]
        dst = SAFE_ROOT.parent / entry["editable_prefix"]
        if not src.exists():
            raise FileNotFoundError(src)
        if not dst.exists():
            raise FileNotFoundError(dst)
        comparison = filecmp.dircmp(src, dst)
        if comparison.left_only or comparison.right_only or comparison.diff_files:
            raise ValueError(f"Editable mirror diverged from vendored source: {entry['editable_prefix']}")
    vendored_only = VENDOR_ORIGINAL / "tests"
    if not vendored_only.exists():
        raise FileNotFoundError(vendored_only)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-map", type=Path)
    args = parser.parse_args()
    if args.verify_map:
        verify_map(args.verify_map)
        return
    for entry in path_map_entries():
        sync_once(entry)


if __name__ == "__main__":
    main()

