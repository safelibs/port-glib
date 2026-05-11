#!/usr/bin/env python3
import argparse
import io
import subprocess
import tarfile
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


def parse_debian_files(path: Path) -> list[Path]:
    path = path.resolve()
    artifacts: list[Path] = []
    for line in path.read_text().splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        artifact = stripped.split()[0]
        candidates = [
            (path.parent / artifact),
            (path.parent.parent / artifact),
            (path.parent.parent.parent / artifact),
        ]
        for candidate in candidates:
            if candidate.exists():
                artifacts.append(candidate.resolve())
                break
        else:
            artifacts.append(candidates[0].resolve())
    return artifacts


def package_name(archive: Path) -> str:
    result = subprocess.run(
        ["dpkg-deb", "-f", str(archive), "Package"],
        check=True,
        text=True,
        capture_output=True,
    )
    return result.stdout.strip()


def tar_members(command: list[str], package: str, member_kind: str) -> list[dict[str, object]]:
    result = subprocess.run(command, check=True, capture_output=True)
    archive = tarfile.open(fileobj=io.BytesIO(result.stdout))
    entries: list[dict[str, object]] = []
    for member in archive.getmembers():
        name = member.name.lstrip("./")
        if not name:
            continue
        if member_kind == "control":
            path = f"control/{name}"
            entry = {"package": package, "member_kind": "control", "path": path}
        elif member.isdir():
            entry = {"package": package, "member_kind": "directory", "path": "/" + name.rstrip("/")}
        elif member.issym():
            entry = {
                "package": package,
                "member_kind": "symlink",
                "path": "/" + name,
                "target": member.linkname,
            }
        elif member.isfile():
            entry = {"package": package, "member_kind": "file", "path": "/" + name}
        else:
            continue
        entries.append(entry)
    return entries


def deb_entries(archive: Path) -> list[dict[str, object]]:
    package = package_name(archive)
    control_entries = tar_members(["dpkg-deb", "--ctrl-tarfile", str(archive)], package, "control")
    data_entries = tar_members(["dpkg-deb", "--fsys-tarfile", str(archive)], package, "data")
    return control_entries + data_entries


def compare_debian_archives(expected_manifest: Path, debian_files: Path) -> None:
    manifest = read_json(expected_manifest)
    expected_packages = set(manifest["packages"])
    actual_archives = parse_debian_files(debian_files)
    actual_by_package = {}
    for archive in actual_archives:
        if archive.suffix not in {".deb", ".udeb"}:
            continue
        pkg = package_name(archive)
        if pkg in expected_packages:
            actual_by_package[pkg] = archive
    missing = sorted(expected_packages - set(actual_by_package))
    if missing:
        raise ValueError(f"debian/files is missing expected package artifacts: {missing}")

    actual_entries: list[dict[str, object]] = []
    for package in sorted(expected_packages):
        actual_entries.extend(deb_entries(actual_by_package[package]))
    actual_entries.sort(key=lambda item: (item["package"], item["member_kind"], item["path"], item.get("target", "")))

    expected_entries = sorted(
        manifest["entries"],
        key=lambda item: (item["package"], item["member_kind"], item["path"], item.get("target", "")),
    )
    compare_jsonish(expected_entries, actual_entries)


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
        compare_debian_archives(args.expected, args.debian_files)
        return
    raise SystemExit("Either --actual or --debian-files is required")


if __name__ == "__main__":
    main()
