#!/usr/bin/env python3
import argparse
import re
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.parse_args()
    hits = []
    for path in Path("crates").rglob("*.rs"):
        text = path.read_text()
        for match in re.finditer(r"\bunsafe\b", text):
            hits.append(f"{path}:{match.start()}")
    if hits:
        raise SystemExit("Unexplained unsafe usage remains in the bootstrap crates")


if __name__ == "__main__":
    main()

