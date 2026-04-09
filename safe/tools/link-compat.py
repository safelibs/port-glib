#!/usr/bin/env python3
import argparse
from pathlib import Path

from common import REPO_ROOT, read_json


def verify_manifests() -> None:
    catalog = read_json(Path("abi/link-compat/catalog.json"))
    entries = catalog["entries"]
    kinds = {entry["kind"] for entry in entries}
    required_kinds = {
        "generated_abi_consumer",
        "debian_build_smoke",
        "upstream_test_target",
    }
    if not required_kinds.issubset(kinds):
        raise ValueError(f"Catalog is missing required entry kinds: {required_kinds - kinds}")

    entry_ids = {entry["id"] for entry in entries}
    for phase in ["abi-shell", "glib-core", "glib-advanced", "gobject", "gio", "girepository", "full"]:
        manifest = read_json(Path(f"abi/link-compat/{phase}.json"))
        missing = [entry_id for entry_id in manifest["entry_ids"] if entry_id not in entry_ids]
        if missing:
            raise ValueError(f"{phase}.json references unknown link-compat entries: {missing}")

    abi_shell = read_json(Path("abi/link-compat/abi-shell.json"))["entry_ids"]
    for soname in [
        "generated-smoke:libglib-2.0.so.0",
        "generated-smoke:libgthread-2.0.so.0",
        "generated-smoke:libgmodule-2.0.so.0",
        "generated-smoke:libgobject-2.0.so.0",
        "generated-smoke:libgio-2.0.so.0",
        "generated-smoke:libgirepository-2.0.so.0",
        "generated-smoke:libgirepository-2.0.so.0:package",
        "debian-build-smoke:build",
        "debian-build-smoke:build-static",
    ]:
        if soname not in abi_shell:
            raise ValueError(f"abi-shell.json is missing required smoke entry {soname}")

    disallowed_fragments = [
        str(REPO_ROOT / "original"),
        str(REPO_ROOT / "build-check"),
        "original/",
        "build-check/",
    ]
    for entry in entries:
        serialized = str(entry)
        if str(REPO_ROOT) in serialized:
            raise ValueError(f"Catalog entry retains a workspace-absolute path: {entry['id']}")
        if "repo-root" in serialized:
            raise ValueError(f"Catalog entry retains repo-root wording instead of normalized placeholders: {entry['id']}")
        if (
            ("original/" in serialized or "build-check/" in serialized)
            and "$SAFE_VENDOR_ORIGINAL" not in serialized
            and "$SAFE_VENDOR_BUILD_CHECK" not in serialized
            and "$BUILD_ROOT" not in serialized
        ):
            raise ValueError(f"Catalog entry retains unnormalized repo paths: {entry['id']}")

    unmatched = read_json(Path("abi/link-compat/unmatched-symbols.json"))
    allowed = {
        "not declared in installed public headers",
        "tooling parser limitation with tracked blocker",
    }
    if any(entry["reason"] not in allowed for entry in unmatched["entries"]):
        raise ValueError("Found unmatched symbol with an unapproved reason")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-manifests", action="store_true")
    args = parser.parse_args()
    if args.verify_manifests:
        verify_manifests()
        return
    raise SystemExit("Only --verify-manifests is implemented in bootstrap")


if __name__ == "__main__":
    main()

