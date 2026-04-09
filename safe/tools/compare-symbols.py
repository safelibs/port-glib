#!/usr/bin/env python3
import argparse
import subprocess
from pathlib import Path


def exported_symbols(library: Path) -> list[str]:
    output = subprocess.run(
        ["nm", "-D", "--defined-only", str(library)],
        check=True,
        text=True,
        capture_output=True,
    ).stdout.splitlines()
    symbols = []
    for line in output:
        parts = line.split()
        if parts:
            symbols.append(parts[-1])
    return sorted(symbols)


def expected_symbols(path: Path) -> list[str]:
    result = []
    for line in path.read_text().splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("*") or stripped.startswith("lib"):
            continue
        result.append(stripped.split("@", 1)[0])
    return sorted(result)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--expected", type=Path, required=True)
    parser.add_argument("--library", type=Path, required=True)
    args = parser.parse_args()
    expected = expected_symbols(args.expected)
    actual = exported_symbols(args.library)
    missing = [symbol for symbol in expected if symbol not in actual]
    if missing:
        raise SystemExit(f"Missing exported symbols: {missing[:20]}")


if __name__ == "__main__":
    main()

