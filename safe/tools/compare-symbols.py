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
            symbols.append(parts[-1].split("@", 1)[0])
    return sorted(symbols)


def expected_symbols(path: Path) -> tuple[list[str], set[str]]:
    result = []
    version_names = set()
    for line in path.read_text().splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("*") or stripped.startswith("lib"):
            continue
        symbol, _, rest = stripped.partition("@")
        result.append(symbol)
        if rest:
            version_names.add(rest.split()[0])
    return sorted(result), version_names


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--expected", type=Path, required=True)
    parser.add_argument("--library", type=Path, required=True)
    args = parser.parse_args()
    expected, version_names = expected_symbols(args.expected)
    actual = [symbol for symbol in exported_symbols(args.library) if symbol not in version_names]
    missing = [symbol for symbol in expected if symbol not in actual]
    extra = [symbol for symbol in actual if symbol not in expected]
    if missing or extra:
        details = []
        if missing:
            details.append(f"missing exported symbols ({len(missing)}): {missing[:20]}")
        if extra:
            details.append(f"extra exported symbols ({len(extra)}): {extra[:20]}")
        raise SystemExit("; ".join(details))

    print(f"symbol parity ok: {len(expected)} symbols match {args.library}")


if __name__ == "__main__":
    main()
