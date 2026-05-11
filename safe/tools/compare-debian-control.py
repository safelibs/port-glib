#!/usr/bin/env python3
import argparse
from pathlib import Path

from common import compare_jsonish, normalize_deb822_value, parse_deb822_file, read_json


def baseline_expected_stanzas(baseline: dict) -> list[dict[str, str]]:
    expected = []
    for stanza in baseline["stanzas"]:
        expected.append({field: metadata["value"] for field, metadata in stanza["fields"].items()})
    return expected


def stanza_identity(stanza: dict[str, str]) -> str:
    return stanza.get("Package") or stanza.get("Source") or "<unnamed>"


def verify_runtime_relationships(runtime: dict[str, str], dev_bin: dict[str, str]) -> None:
    for field in ["Breaks", "Provides", "Replaces"]:
        if not normalize_deb822_value(runtime.get(field, "")):
            raise ValueError(f"Runtime stanza lost required non-empty {field}")
    provides = normalize_deb822_value(dev_bin.get("Provides", ""))
    if "libglib2.0-dev-bin-${local:DEB-HOST-ARCH-OS}" not in provides:
        raise ValueError("libglib2.0-dev-bin virtual provide is missing")


def verify_baseline(baseline_path: Path, control_path: Path) -> None:
    baseline = read_json(baseline_path)
    control_stanzas = parse_deb822_file(control_path)
    compare_jsonish(baseline_expected_stanzas(baseline), control_stanzas)

    source_stanza = baseline["stanzas"][0]
    runtime = next(stanza for stanza in baseline["stanzas"] if stanza["name"] == "libglib2.0-0t64")
    dev_bin = next(stanza for stanza in baseline["stanzas"] if stanza["name"] == "libglib2.0-dev-bin")
    verify_runtime_relationships(
        {field: metadata["value"] for field, metadata in runtime["fields"].items()},
        {field: metadata["value"] for field, metadata in dev_bin["fields"].items()},
    )
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


def compare_against_baseline(baseline_path: Path, control_path: Path) -> None:
    baseline = read_json(baseline_path)
    control_stanzas = parse_deb822_file(control_path)
    baseline_stanzas = baseline["stanzas"]
    if len(control_stanzas) < len(baseline_stanzas):
        raise ValueError("debian/control is missing required stanzas")

    expected_names = [stanza["name"] for stanza in baseline_stanzas]
    actual_names = [stanza_identity(stanza) for stanza in control_stanzas[: len(expected_names)]]
    if expected_names != actual_names:
        raise ValueError(f"Stanza order mismatch: expected {expected_names}, got {actual_names}")

    for baseline_stanza, actual_stanza in zip(baseline_stanzas, control_stanzas, strict=True):
        for field, metadata in baseline_stanza["fields"].items():
            mode = metadata["mode"]
            if field not in actual_stanza:
                raise ValueError(f"Missing field {field} in stanza {baseline_stanza['name']}")
            actual_value = normalize_deb822_value(actual_stanza[field])
            expected_value = normalize_deb822_value(metadata["value"])
            if mode == "exact-after-normalization":
                if actual_value != expected_value:
                    raise ValueError(
                        f"Field {field} in stanza {baseline_stanza['name']} changed: "
                        f"expected {expected_value!r}, got {actual_value!r}"
                    )
            elif mode == "regenerable-with-rationale":
                if not actual_value:
                    raise ValueError(f"Regenerable field {field} in stanza {baseline_stanza['name']} is empty")
            elif mode == "preserve-for-provenance":
                continue
            else:
                raise ValueError(f"Unknown preservation mode {mode} for {baseline_stanza['name']}:{field}")

    runtime = next(stanza for stanza in control_stanzas if stanza_identity(stanza) == "libglib2.0-0t64")
    dev_bin = next(stanza for stanza in control_stanzas if stanza_identity(stanza) == "libglib2.0-dev-bin")
    verify_runtime_relationships(runtime, dev_bin)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--control", type=Path, required=True)
    parser.add_argument("--verify-baseline", action="store_true")
    args = parser.parse_args()
    if args.verify_baseline:
        verify_baseline(args.baseline, args.control)
        return
    compare_against_baseline(args.baseline, args.control)


if __name__ == "__main__":
    main()
