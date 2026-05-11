#!/usr/bin/env python3
import argparse
from pathlib import Path

from common import read_json


def verify_state(manifest_path: Path, root: Path, *, purged: bool) -> None:
    manifest = read_json(manifest_path)
    key = "expected_after_purge" if purged else "expected_after_install"
    for entry in manifest["entries"]:
        target = root / entry["path"].lstrip("/")
        expectation = entry[key]
        exists = target.exists()
        if expectation == "present" and not exists:
            raise FileNotFoundError(f"Expected generated runtime state is missing: {target}")
        if expectation == "absent" and exists:
            raise ValueError(f"Expected generated runtime state to be absent: {target}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--purged", action="store_true")
    args = parser.parse_args()
    verify_state(args.manifest, args.root, purged=args.purged)


if __name__ == "__main__":
    main()

