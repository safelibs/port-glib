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


def verify_phase_state(manifest_path: Path, source_format: Path, series_path: Path, provenance_doc: Path) -> None:
    manifest = read_json(manifest_path)
    original_series = series_lines(Path("vendor/original/debian/patches/series"))
    manifest_patches = [entry["patch"] for entry in manifest["entries"]]
    if manifest_patches != original_series:
        raise ValueError("Patch manifest patch order no longer matches the original Debian series")
    if source_format.read_text().strip() != "3.0 (quilt)":
        raise ValueError("debian/source/format must remain 3.0 (quilt)")

    allowed_dispositions = {
        "carried-verbatim",
        "rewritten-for-safe-packaging",
        "absorbed-into-safe-source",
        "obsolete-with-rationale",
    }
    active_series = series_lines(series_path)
    expected_active = [entry["patch"] for entry in manifest["entries"] if entry["active_queue"]]
    if active_series != expected_active:
        raise ValueError(
            f"debian/patches/series does not match manifest active_queue flags: "
            f"expected {expected_active}, got {active_series}"
        )

    provenance_text = provenance_doc.read_text()
    for entry in manifest["entries"]:
        disposition = entry.get("expected_disposition")
        if disposition not in allowed_dispositions:
            raise ValueError(f"Patch {entry['patch']} uses unsupported disposition {disposition!r}")
        if not isinstance(entry.get("active_queue"), bool):
            raise ValueError(f"Patch {entry['patch']} must use a boolean active_queue flag")
        if not entry.get("provenance"):
            raise ValueError(f"Patch {entry['patch']} is missing safe provenance paths")
        if not entry.get("regressions"):
            raise ValueError(f"Patch {entry['patch']} is missing regression/test references")
        if entry["patch"] not in provenance_text or disposition not in provenance_text:
            raise ValueError(f"Provenance document does not mention {entry['patch']} with {disposition}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-manifest", action="store_true")
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--source-format", type=Path)
    parser.add_argument("--series", type=Path)
    parser.add_argument("--provenance-doc", type=Path)
    args = parser.parse_args()
    if args.verify_manifest:
        verify_manifest()
        return
    if args.manifest and args.source_format and args.series and args.provenance_doc:
        verify_phase_state(args.manifest, args.source_format, args.series, args.provenance_doc)
        return
    raise SystemExit("Use --verify-manifest or provide --manifest/--source-format/--series/--provenance-doc")


if __name__ == "__main__":
    main()
