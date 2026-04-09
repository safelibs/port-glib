#!/usr/bin/env python3
import argparse
import shutil
from pathlib import Path

from common import REPO_ROOT, SAFE_ROOT, VENDOR_BUILD_CHECK, VENDOR_ORIGINAL, clean_subprocess_env, read_json, run


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
    for entry_id in [
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
        if entry_id not in abi_shell:
            raise ValueError(f"abi-shell.json is missing required smoke entry {entry_id}")

    unmatched = read_json(Path("abi/link-compat/unmatched-symbols.json"))
    allowed = {
        "not declared in installed public headers",
        "tooling parser limitation with tracked blocker",
    }
    if any(entry["reason"] not in allowed for entry in unmatched["entries"]):
        raise ValueError("Found unmatched symbol with an unapproved reason")


def resolve_placeholder(value: str, build_root: Path) -> str:
    if value.startswith("$BUILD_ROOT/"):
        return str(build_root / value[len("$BUILD_ROOT/") :])
    if value == "$BUILD_ROOT":
        return str(build_root)
    if value.startswith("$SAFE_VENDOR_ORIGINAL/"):
        return str(VENDOR_ORIGINAL / value[len("$SAFE_VENDOR_ORIGINAL/") :])
    if value == "$SAFE_VENDOR_ORIGINAL":
        return str(VENDOR_ORIGINAL)
    if value.startswith("$SAFE_VENDOR_BUILD_CHECK/"):
        return str(VENDOR_BUILD_CHECK / value[len("$SAFE_VENDOR_BUILD_CHECK/") :])
    if value == "$SAFE_VENDOR_BUILD_CHECK":
        return str(VENDOR_BUILD_CHECK)
    return value


def library_path_env(build_root: Path) -> str:
    parts = [
        build_root / "glib",
        build_root / "gthread",
        build_root / "gmodule",
        build_root / "gobject",
        build_root / "gio",
        build_root / "girepository",
    ]
    return ":".join(str(part) for part in parts)


def compile_generated(entry: dict, build_root: Path, run_binary: bool) -> None:
    workdir = build_root / "link-compat"
    source_path = workdir / entry["translation_unit"]["path"]
    source_path.parent.mkdir(parents=True, exist_ok=True)
    source_text = entry["translation_unit"]["source"]
    header_include = entry.get("header", {}).get("include", "")
    if header_include.startswith("glib/") and header_include != "glib.h":
        source_text = "#include <glib.h>\n" + source_text
    elif header_include.startswith("gobject/") and header_include != "glib-object.h":
        source_text = "#include <glib-object.h>\n" + source_text
    elif header_include.startswith("gio/") and header_include != "gio/gio.h":
        source_text = "#include <gio/gio.h>\n" + source_text
    elif header_include.startswith("girepository/") and header_include != "girepository/girepository.h":
        source_text = "#include <girepository/girepository.h>\n" + source_text
    source_path.write_text(source_text)
    runnable = bool(entry.get("runnable"))
    output = source_path.with_suffix("" if runnable else ".so")

    cmd = ["gcc", str(source_path), "-o", str(output)]
    if not runnable:
        cmd.extend(["-shared", "-fPIC"])
    for flag in entry.get("compile", {}).get("cflags", []):
        cmd.append(flag)
    include_dirs = [resolve_placeholder(value, build_root) for value in entry.get("compile", {}).get("include_dirs", [])]
    include_dirs.extend(
        [
            str(VENDOR_ORIGINAL / "glib"),
            str(VENDOR_ORIGINAL / "gmodule"),
            str(VENDOR_ORIGINAL / "gobject"),
            str(VENDOR_ORIGINAL / "gio"),
            str(VENDOR_ORIGINAL / "girepository"),
            str(VENDOR_BUILD_CHECK / "glib"),
            str(VENDOR_BUILD_CHECK / "gmodule"),
            str(VENDOR_BUILD_CHECK / "gio"),
            str(VENDOR_BUILD_CHECK / "girepository"),
        ]
    )
    for include_dir in include_dirs:
        cmd.extend(["-I", include_dir])

    env = clean_subprocess_env(
        updates={
            "PKG_CONFIG_PATH": str(build_root / "pkgconfig"),
            "LD_LIBRARY_PATH": library_path_env(build_root),
        }
    )
    pkg_config = entry.get("compile", {}).get("pkg_config", [])
    if pkg_config:
        result = run(["pkg-config", "--cflags", "--libs", *pkg_config], cwd=SAFE_ROOT, env=env, capture=True)
        cmd.extend(result.stdout.strip().split())
    else:
        for expected in entry.get("link", {}).get("expected", []):
            cmd.append(resolve_placeholder(expected, build_root))
    run(cmd, cwd=SAFE_ROOT, env=env)
    if run_binary and runnable:
        run([str(output)], cwd=SAFE_ROOT, env=env)


def run_debian_smoke(entry: dict, build_root: Path) -> None:
    env = clean_subprocess_env(
        updates={
            "PKG_CONFIG_PATH": str(build_root / "pkgconfig"),
            "LD_LIBRARY_PATH": library_path_env(build_root),
        }
    )
    script = resolve_placeholder(entry["script"], build_root)
    run([script], cwd=SAFE_ROOT, env=env)


def execute_phase(phase: str, build_root: Path, run_binaries: bool) -> None:
    catalog = {entry["id"]: entry for entry in read_json(Path("abi/link-compat/catalog.json"))["entries"]}
    manifest = read_json(Path(f"abi/link-compat/{phase}.json"))
    workdir = build_root / "link-compat"
    if workdir.exists():
        shutil.rmtree(workdir)
    workdir.mkdir(parents=True, exist_ok=True)

    for entry_id in manifest["entry_ids"]:
        entry = catalog[entry_id]
        if entry["kind"] == "generated_abi_consumer":
            compile_generated(entry, build_root, run_binaries)
        elif entry["kind"] == "debian_build_smoke":
            run_debian_smoke(entry, build_root)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-manifests", action="store_true")
    parser.add_argument("--phase")
    parser.add_argument("--build-root", type=Path)
    parser.add_argument("--compile-original-objects", action="store_true")
    parser.add_argument("--run", action="store_true")
    args = parser.parse_args()
    if args.verify_manifests:
        verify_manifests()
        return
    if args.phase and args.build_root:
        execute_phase(args.phase, args.build_root.resolve(), args.run)
        return
    raise SystemExit("Use --verify-manifests or provide --phase and --build-root")


if __name__ == "__main__":
    main()
