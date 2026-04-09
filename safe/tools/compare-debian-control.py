#!/usr/bin/env python3
import argparse
from pathlib import Path

from common import compare_jsonish, parse_deb822_file, read_json


def verify_baseline(baseline_path: Path, control_path: Path) -> None:
    baseline = read_json(baseline_path)
    control_stanzas = parse_deb822_file(control_path)
    expected = []
    for stanza in baseline["stanzas"]:
        expected.append(
            {
                field: metadata["value"]
                for field, metadata in stanza["fields"].items()
            }
        )
    compare_jsonish(expected, control_stanzas)

    source_stanza = baseline["stanzas"][0]
    runtime = next(stanza for stanza in baseline["stanzas"] if stanza["name"] == "libglib2.0-0t64")
    dev_bin = next(stanza for stanza in baseline["stanzas"] if stanza["name"] == "libglib2.0-dev-bin")
    for field in ["Breaks", "Provides", "Replaces"]:
        if not runtime["fields"].get(field, {}).get("value"):
            raise ValueError(f"Runtime stanza lost required non-empty {field}")
    provides = dev_bin["fields"].get("Provides", {}).get("value", "")
    if "libglib2.0-dev-bin-${local:DEB-HOST-ARCH-OS}" not in provides:
        raise ValueError("libglib2.0-dev-bin virtual provide is missing from the frozen manifest")
    for field, metadata in source_stanza["fields"].items():
        mode = metadata["mode"]
        if field in {"Build-Depends", "Build-Depends-Arch", "Build-Depends-Indep"}:
            if mode != "regenerable-with-rationale":
                raise ValueError(f"Source field {field} must remain regenerable-with-rationale")
        elif field in {
            "Source",
            "Section",
            "Priority",
            "Maintainer",
            "XSBC-Original-Maintainer",
            "Uploaders",
            "Rules-Requires-Root",
            "Standards-Version",
            "Homepage",
            "Vcs-Browser",
            "Vcs-Git",
        } and mode != "exact-after-normalization":
            raise ValueError(f"Source field {field} lost exact-after-normalization mode")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--control", type=Path, required=True)
    parser.add_argument("--verify-baseline", action="store_true")
    args = parser.parse_args()
    if args.verify_baseline:
        verify_baseline(args.baseline, args.control)
        return
    raise SystemExit("Only --verify-baseline is implemented in bootstrap")


if __name__ == "__main__":
    main()

