#!/usr/bin/env python3
import argparse
from pathlib import Path

from common import read_json, series_lines


def verify_manifest() -> None:
    baseline = read_json(Path("abi/debian-patches.json"))
    series = series_lines(Path("vendor/original/debian/patches/series"))
    manifest_patches = [entry["patch"] for entry in baseline["entries"]]
    if manifest_patches != series:
        raise ValueError("abi/debian-patches.json no longer matches debian/patches/series")
    allowed = {"upstream-source", "packaging", "workaround", "CVE-driven"}
    for entry in baseline["entries"]:
        if entry["category"] not in allowed:
            raise ValueError(f"Unexpected patch category: {entry['category']}")
        for field in ["expected_disposition", "active_queue", "regressions", "provenance"]:
            if field not in entry:
                raise ValueError(f"Patch manifest entry is missing field {field}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-manifest", action="store_true")
    args = parser.parse_args()
    if args.verify_manifest:
        verify_manifest()
        return
    raise SystemExit("Only --verify-manifest is implemented in bootstrap")


if __name__ == "__main__":
    main()

