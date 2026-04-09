#!/usr/bin/env python3
import argparse
from pathlib import Path

from common import PathNormalizer, REPO_ROOT, compare_jsonish, read_json


def normalize_entries(path: Path) -> list[dict[str, object]]:
    data = read_json(path)
    if isinstance(data, dict) and "entries" in data:
        return sorted(data["entries"], key=lambda item: (item.get("install_path", ""), item.get("source", "")))
    entries = []
    if isinstance(data, dict):
        normalizer = PathNormalizer(
            build_root=REPO_ROOT / "build-check",
            original_root=REPO_ROOT / "original",
        )
        for source, install_value in data.items():
            install_paths = install_value if isinstance(install_value, list) else [install_value]
            for install_path in install_paths:
                entries.append({"source": normalizer.normalize_token(str(source)), "install_path": install_path})
        return sorted(entries, key=lambda item: (item["install_path"], item["source"]))
    raise TypeError(f"Unsupported installed-file payload in {path}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--expected", type=Path, required=True)
    parser.add_argument("--actual", type=Path)
    parser.add_argument("--debian-files", type=Path)
    args = parser.parse_args()

    expected = normalize_entries(args.expected)
    if args.actual:
        actual = normalize_entries(args.actual)
        compare_jsonish(expected, actual)
        return
    if args.debian_files:
        raise SystemExit("debian/files comparison is implemented by verify-package-baselines.py during bootstrap generation")
    raise SystemExit("Either --actual or --debian-files is required")


if __name__ == "__main__":
    main()
