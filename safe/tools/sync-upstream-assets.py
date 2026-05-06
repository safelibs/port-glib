#!/usr/bin/env python3
import argparse
import hashlib
import os
import shutil
from pathlib import Path

from common import SAFE_ROOT, VENDOR_ORIGINAL, path_map_entries, read_json


IGNORED_NAMES = {"__pycache__"}
IGNORED_SUFFIXES = {".pyc"}


def is_ignored_relpath(relpath: str) -> bool:
    parts = Path(relpath).parts
    return any(part in IGNORED_NAMES for part in parts) or any(relpath.endswith(suffix) for suffix in IGNORED_SUFFIXES)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def mirror_contract_entries(entry: dict[str, object]) -> list[dict[str, object]]:
    edits = entry.get("preserved_edits", [])
    if not isinstance(edits, list):
        raise ValueError(f"Invalid preserved_edits contract for {entry['editable_prefix']}")
    seen_paths: set[str] = set()
    for edit in edits:
        if not isinstance(edit, dict):
            raise ValueError(f"Invalid preserved edit entry for {entry['editable_prefix']}: {edit!r}")
        relpath_value = edit.get("path")
        if not isinstance(relpath_value, str) or not relpath_value:
            raise ValueError(f"Preserved edit is missing a relative path for {entry['editable_prefix']}")
        relpath = Path(relpath_value)
        if relpath.is_absolute() or ".." in relpath.parts:
            raise ValueError(f"Preserved edit path must stay inside the editable mirror: {relpath_value}")
        relpath_text = relpath.as_posix()
        if relpath_text in seen_paths:
            raise ValueError(f"Duplicate preserved edit path for {entry['editable_prefix']}: {relpath_text}")
        seen_paths.add(relpath_text)
        kind = edit.get("kind")
        if kind not in {"modified", "added"}:
            raise ValueError(f"Unknown preserved edit kind {kind!r} for {entry['editable_prefix']}")
        reason = edit.get("reason")
        if not isinstance(reason, str) or not reason.strip():
            raise ValueError(f"Preserved edit {relpath_text} must document why it diverges")
        if kind == "modified" and "canonical_sha256" not in edit:
            raise ValueError(f"Modified preserved edit {relpath_text} must pin canonical_sha256")
        if "editable_sha256" not in edit:
            raise ValueError(f"Preserved edit {relpath_text} must pin editable_sha256")
    return edits


def walk_tree(root: Path) -> tuple[set[str], set[str]]:
    dirs: set[str] = set()
    files: set[str] = set()
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [name for name in dirnames if name not in IGNORED_NAMES]
        rel_dir = Path(dirpath).relative_to(root)
        rel_dir_text = "" if rel_dir == Path(".") else rel_dir.as_posix()
        for name in dirnames:
            relpath = f"{rel_dir_text}/{name}" if rel_dir_text else name
            dirs.add(relpath)
        for name in filenames:
            relpath = f"{rel_dir_text}/{name}" if rel_dir_text else name
            if is_ignored_relpath(relpath):
                continue
            files.add(relpath)
    return dirs, files


def parent_dirs(relpath: str) -> set[str]:
    parents = set()
    for parent in Path(relpath).parents:
        if parent == Path("."):
            break
        parents.add(parent.as_posix())
    return parents


def verify_editable_mirror(entry: dict[str, object], src: Path, dst: Path) -> None:
    canonical_dirs, canonical_files = walk_tree(src)
    editable_dirs, editable_files = walk_tree(dst)

    modified_files: set[str] = set()
    added_files: set[str] = set()
    added_dirs: set[str] = set()
    for edit in mirror_contract_entries(entry):
        relpath = str(edit["path"])
        kind = str(edit["kind"])
        if is_ignored_relpath(relpath):
            raise ValueError(f"Editable mirror contract should not track ignored noise: {relpath}")

        editable_file = dst / relpath
        if kind == "modified":
            canonical_file = src / relpath
            if relpath not in canonical_files:
                raise ValueError(f"Missing canonical file for preserved edit: {canonical_file}")
            if relpath not in editable_files:
                raise ValueError(f"Missing editable file for preserved edit: {editable_file}")
            canonical_sha256 = str(edit["canonical_sha256"])
            editable_sha256 = str(edit["editable_sha256"])
            actual_canonical_sha256 = sha256(canonical_file)
            actual_editable_sha256 = sha256(editable_file)
            if actual_canonical_sha256 != canonical_sha256:
                raise ValueError(
                    f"Canonical source drifted for preserved edit {relpath}: "
                    f"{actual_canonical_sha256} != {canonical_sha256}"
                )
            if actual_editable_sha256 != editable_sha256:
                raise ValueError(
                    f"Editable mirror drifted for preserved edit {relpath}: "
                    f"{actual_editable_sha256} != {editable_sha256}"
                )
            if actual_editable_sha256 == actual_canonical_sha256:
                raise ValueError(f"Preserved edit no longer diverges from canonical source: {relpath}")
            modified_files.add(relpath)
            continue

        if kind == "added":
            if relpath in canonical_files:
                raise ValueError(f"Preserved added file unexpectedly exists in canonical source: {src / relpath}")
            if relpath not in editable_files:
                raise ValueError(f"Missing preserved added file in editable mirror: {editable_file}")
            editable_sha256 = str(edit["editable_sha256"])
            actual_editable_sha256 = sha256(editable_file)
            if actual_editable_sha256 != editable_sha256:
                raise ValueError(
                    f"Editable mirror drifted for preserved added file {relpath}: "
                    f"{actual_editable_sha256} != {editable_sha256}"
                )
            added_files.add(relpath)
            added_dirs.update(parent_dirs(relpath))
            continue

        raise ValueError(f"Unknown preserved edit kind {kind!r} for {entry['editable_prefix']}")

    missing_dirs = sorted(canonical_dirs - editable_dirs)
    extra_dirs = sorted(editable_dirs - canonical_dirs - added_dirs)
    missing_files = sorted(canonical_files - editable_files)
    extra_files = sorted(editable_files - canonical_files - added_files)
    if missing_dirs or extra_dirs or missing_files or extra_files:
        details = []
        if missing_dirs:
            details.append(f"missing dirs: {', '.join(missing_dirs[:10])}")
        if extra_dirs:
            details.append(f"extra dirs: {', '.join(extra_dirs[:10])}")
        if missing_files:
            details.append(f"missing files: {', '.join(missing_files[:10])}")
        if extra_files:
            details.append(f"extra files: {', '.join(extra_files[:10])}")
        raise ValueError(f"Editable mirror shape drifted for {entry['editable_prefix']}: {'; '.join(details)}")

    for relpath in sorted(canonical_files & editable_files):
        if relpath in modified_files:
            continue
        if sha256(src / relpath) != sha256(dst / relpath):
            raise ValueError(f"Editable mirror diverged from vendored source: {entry['editable_prefix']}/{relpath}")


def stash_preserved_edits(dst: Path, entry: dict[str, object]) -> dict[str, tuple[int, bytes]]:
    stashed: dict[str, tuple[int, bytes]] = {}
    for edit in mirror_contract_entries(entry):
        relpath = str(edit["path"])
        path = dst / relpath
        stashed[relpath] = (path.stat().st_mode, path.read_bytes())
    return stashed


def restore_preserved_edits(dst: Path, preserved: dict[str, tuple[int, bytes]]) -> None:
    for relpath, (mode, data) in preserved.items():
        path = dst / relpath
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        path.chmod(mode & 0o777)


def ignore_copytree_noise(_dirpath: str, names: list[str]) -> list[str]:
    ignored = []
    for name in names:
        if name in IGNORED_NAMES or any(name.endswith(suffix) for suffix in IGNORED_SUFFIXES):
            ignored.append(name)
    return ignored


def sync_once(entry: dict[str, object]) -> None:
    src = SAFE_ROOT.parent / entry["canonical_prefix"]
    dst = SAFE_ROOT.parent / entry["editable_prefix"]
    preserved: dict[str, tuple[int, bytes]] = {}
    if dst.exists():
        verify_editable_mirror(entry, src, dst)
        preserved = stash_preserved_edits(dst, entry)
        shutil.rmtree(dst)
    elif mirror_contract_entries(entry):
        raise FileNotFoundError(
            f"Editable mirror {dst} is missing and preserved local edits cannot be reconstructed"
        )
    shutil.copytree(src, dst, symlinks=True, ignore=ignore_copytree_noise)
    restore_preserved_edits(dst, preserved)


def verify_map(path: Path) -> None:
    actual = read_json(path)
    expected = path_map_entries()
    if actual != expected:
        raise ValueError("abi/test-source-path-map.json does not match the fixed seven-root contract")
    for entry in actual:
        src = SAFE_ROOT.parent / entry["canonical_prefix"]
        dst = SAFE_ROOT.parent / entry["editable_prefix"]
        if not src.exists():
            raise FileNotFoundError(src)
        if not dst.exists():
            raise FileNotFoundError(dst)
        verify_editable_mirror(entry, src, dst)
    vendored_only = VENDOR_ORIGINAL / "tests"
    if not vendored_only.exists():
        raise FileNotFoundError(vendored_only)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-map", type=Path)
    args = parser.parse_args()
    if args.verify_map:
        verify_map(args.verify_map)
        return
    for entry in path_map_entries():
        sync_once(entry)


if __name__ == "__main__":
    main()
