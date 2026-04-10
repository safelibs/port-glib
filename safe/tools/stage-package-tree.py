#!/usr/bin/env python3
import argparse
import json
import os
import shutil
from pathlib import Path

from common import SAFE_ROOT, ensure_dir, run


MULTIARCH_HELPERS = (
    "gio-launch-desktop",
    "gio-querymodules",
    "glib-compile-schemas",
)

GIREPOSITORY_HELPERS = {
    "gi-compile-repository": "girepository/compiler/gi-compile-repository",
    "gi-decompile-typelib": "girepository/decompiler/gi-decompile-typelib",
    "gi-inspect-typelib": "girepository/inspector/gi-inspect-typelib",
}

def executable(path: Path, content: str) -> None:
    ensure_dir(path.parent)
    path.write_text(content)
    path.chmod(0o755)


def resolve_placeholder(source: str) -> Path:
    if source.startswith("$SAFE_VENDOR_ORIGINAL/"):
        return SAFE_ROOT / "vendor" / "original" / source[len("$SAFE_VENDOR_ORIGINAL/") :]
    if source == "$SAFE_VENDOR_ORIGINAL":
        return SAFE_ROOT / "vendor" / "original"
    if source.startswith("$SAFE_VENDOR_BUILD_CHECK/"):
        return SAFE_ROOT / "vendor" / "build-check" / source[len("$SAFE_VENDOR_BUILD_CHECK/") :]
    if source == "$SAFE_VENDOR_BUILD_CHECK":
        return SAFE_ROOT / "vendor" / "build-check"
    if source.startswith("$BUILD_ROOT/"):
        return SAFE_ROOT / "vendor" / "build-check" / source[len("$BUILD_ROOT/") :]
    if source == "$BUILD_ROOT":
        return SAFE_ROOT / "vendor" / "build-check"
    raise KeyError(f"Unhandled placeholder source: {source}")


def installed_source_map() -> dict[str, Path]:
    mapping: dict[str, Path] = {}
    payload = json.loads((SAFE_ROOT / "abi" / "installed-files.json").read_text())
    for entry in payload["entries"]:
        source = entry["source"]
        if not source.startswith("$"):
            continue
        resolved = resolve_placeholder(source)
        install_path = entry["install_path"].replace("/.//", "/")
        mapping[install_path] = resolved
        if install_path.startswith("/usr/local/"):
            mapping[install_path.replace("/usr/local/", "/usr/", 1)] = resolved
        gdb_prefix = "/usr/local/share/gdb/auto-load/usr/local/"
        if install_path.startswith(gdb_prefix):
            mapping[install_path.replace(gdb_prefix, "/usr/share/gdb/auto-load/usr/", 1)] = resolved
    return mapping


def lookup_installed_source(install_map: dict[str, Path], install_path: str) -> Path | None:
    direct = install_map.get(install_path)
    if direct is not None:
        return direct
    if install_path.startswith("/usr/"):
        return install_map.get("/usr/local/" + install_path[len("/usr/") :])
    return None


def render_install_pc(name: str, multiarch: str) -> str:
    base = {
        "glib-2.0": {
            "variables": "\n".join(
                [
                    "bindir=${prefix}/bin",
                    "datadir=${prefix}/share",
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                    "glib_genmarshal=${bindir}/glib-genmarshal",
                    "gobject_query=${bindir}/gobject-query",
                    "glib_mkenums=${bindir}/glib-mkenums",
                    "glib_valgrind_suppressions=${datadir}/glib-2.0/valgrind/glib.supp",
                    "",
                ]
            ),
            "name": "GLib",
            "description": "C Utility Library",
            "requires": "",
            "requires_private": "Requires.private: libpcre2-8 >= 10.32\n",
            "libs": "Libs: -L${libdir} -lglib-2.0\n",
            "libs_private": "Libs.private: -lm -pthread\n",
            "cflags": "Cflags: -I${includedir}/glib-2.0 -I${libdir}/glib-2.0/include\n",
        },
        "gthread-2.0": {
            "variables": "\n".join(
                [
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                ]
            ),
            "name": "GThread",
            "description": "Thread support for GLib",
            "requires": "Requires: glib-2.0\n",
            "requires_private": "",
            "libs": "Libs: -L${libdir} -lgthread-2.0 -pthread\n",
            "libs_private": "",
            "cflags": "Cflags: -I${includedir} -pthread\n",
        },
        "gmodule-2.0": {
            "variables": "\n".join(
                [
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                    "gmodule_supported=true",
                    "",
                ]
            ),
            "name": "GModule",
            "description": "Dynamic module loader for GLib",
            "requires": "Requires: gmodule-no-export-2.0, glib-2.0\n",
            "requires_private": "",
            "libs": "Libs: -Wl,--export-dynamic\n",
            "libs_private": "",
            "cflags": "Cflags: -I${includedir}\n",
        },
        "gmodule-export-2.0": {
            "variables": "\n".join(
                [
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                    "gmodule_supported=true",
                    "",
                ]
            ),
            "name": "GModule",
            "description": "Dynamic module loader for GLib",
            "requires": "Requires: gmodule-no-export-2.0, glib-2.0\n",
            "requires_private": "",
            "libs": "Libs: -Wl,--export-dynamic\n",
            "libs_private": "",
            "cflags": "Cflags: -I${includedir}\n",
        },
        "gmodule-no-export-2.0": {
            "variables": "\n".join(
                [
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                    "gmodule_supported=true",
                    "",
                ]
            ),
            "name": "GModule",
            "description": "Dynamic module loader for GLib",
            "requires": "Requires: glib-2.0\n",
            "requires_private": "",
            "libs": "Libs: -L${libdir} -lgmodule-2.0 -pthread\n",
            "libs_private": "",
            "cflags": "Cflags: -I${includedir} -pthread\n",
        },
        "gobject-2.0": {
            "variables": "\n".join(
                [
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                ]
            ),
            "name": "GObject",
            "description": "GLib Type, Object, Parameter and Signal Library",
            "requires": "Requires: glib-2.0\n",
            "requires_private": "Requires.private: libffi >= 3.0.0\n",
            "libs": "Libs: -L${libdir} -lgobject-2.0\n",
            "libs_private": "",
            "cflags": "Cflags: -I${includedir}\n",
        },
        "gio-2.0": {
            "variables": "\n".join(
                [
                    "bindir=${prefix}/bin",
                    "datadir=${prefix}/share",
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                    "schemasdir=${datadir}/glib-2.0/schemas",
                    "dtdsdir=${datadir}/glib-2.0/dtds",
                    "giomoduledir=${libdir}/gio/modules",
                    "gio=${bindir}/gio",
                    "gio_querymodules=${libdir}/glib-2.0/gio-querymodules",
                    "glib_compile_schemas=${libdir}/glib-2.0/glib-compile-schemas",
                    "glib_compile_resources=${bindir}/glib-compile-resources",
                    "gdbus=${bindir}/gdbus",
                    "gdbus_codegen=${bindir}/gdbus-codegen",
                    "gresource=${bindir}/gresource",
                    "gsettings=${bindir}/gsettings",
                    "",
                ]
            ),
            "name": "GIO",
            "description": "glib I/O library",
            "requires": "Requires: glib-2.0, gobject-2.0\n",
            "requires_private": "Requires.private: gmodule-no-export-2.0, zlib, mount >= 2.23, libselinux >= 2.2\n",
            "libs": "Libs: -L${libdir} -lgio-2.0\n",
            "libs_private": "",
            "cflags": "Cflags: -I${includedir}\n",
        },
        "gio-unix-2.0": {
            "variables": "includedir=${prefix}/include\n\n",
            "name": "GIO unix specific APIs",
            "description": "unix specific headers for glib I/O library",
            "requires": "Requires: gobject-2.0, gio-2.0\n",
            "requires_private": "",
            "libs": "",
            "libs_private": "",
            "cflags": "Cflags: -I${includedir}/gio-unix-2.0\n",
        },
        "girepository-2.0": {
            "variables": "\n".join(
                [
                    "datadir=${prefix}/share",
                    "includedir=${prefix}/include",
                    f"libdir=${{prefix}}/lib/{multiarch}",
                    "",
                    "gidatadir=${datadir}/gobject-introspection-1.0",
                    "girdir=${datadir}/gir-1.0",
                    "typelibdir=${libdir}/girepository-1.0",
                    "",
                ]
            ),
            "name": "girepository",
            "description": "GObject Introspection repository parser",
            "requires": "Requires: glib-2.0, gobject-2.0\n",
            "requires_private": "Requires.private: gmodule-no-export-2.0, gio-2.0, libffi >= 3.0.0\n",
            "libs": "Libs: -L${libdir} -lgirepository-2.0\n",
            "libs_private": "Libs.private: -lm\n",
            "cflags": "Cflags: -I${includedir}\n",
        },
    }[name]
    return (
        "prefix=/usr\n"
        f"{base['variables']}"
        f"Name: {base['name']}\n"
        f"Description: {base['description']}\n"
        "Version: 2.80.0\n"
        f"{base['requires']}"
        f"{base['requires_private']}"
        f"{base['libs']}"
        f"{base['libs_private']}"
        f"{base['cflags']}"
    )


def copy_or_write(path: Path, data: str | bytes) -> None:
    ensure_dir(path.parent)
    if isinstance(data, bytes):
        path.write_bytes(data)
    else:
        path.write_text(data)


def copy_executable(source: Path, target: Path) -> None:
    ensure_dir(target.parent)
    shutil.copy2(source, target)
    target.chmod(target.stat().st_mode | 0o111)


def is_elf_binary(path: Path) -> bool:
    if not path.is_file() or path.is_symlink():
        return False
    with path.open("rb") as handle:
        return handle.read(4) == b"\x7fELF"


def patch_installed_runpaths(destdir: Path, multiarch: str) -> None:
    elf_roots = {
        destdir / "usr" / "bin": f"$ORIGIN/../lib/{multiarch}",
        destdir / "usr" / "lib" / multiarch / "glib-2.0": "$ORIGIN/..",
    }

    for root, runpath in elf_roots.items():
        if not root.exists():
            continue
        for child in root.iterdir():
            if not is_elf_binary(child):
                continue
            run(["patchelf", "--set-rpath", runpath, str(child)], cwd=SAFE_ROOT)


def render_python_script(template: Path, replacements: dict[str, str]) -> str:
    content = template.read_text()
    for key, value in replacements.items():
        content = content.replace(key, value)
    return content


def install_script_file(destdir: Path, relative_path: str, template: Path, replacements: dict[str, str]) -> None:
    path = destdir / relative_path.lstrip("/")
    executable(path, render_python_script(template, replacements))


def stage_helpers(destdir: Path, build_root: Path, multiarch: str) -> None:
    helper_root = destdir / "usr/lib" / multiarch / "glib-2.0"
    ensure_dir(helper_root)
    for name in MULTIARCH_HELPERS:
        copy_executable(build_root / "gio" / name, helper_root / name)

    for name, relative_path in GIREPOSITORY_HELPERS.items():
        copy_executable(build_root / relative_path, helper_root / name)

    install_script_file(
        destdir,
        "/usr/bin/gdbus-codegen",
        SAFE_ROOT / "vendor/original/gio/gdbus-2.0/codegen/gdbus-codegen.in",
        {"@PYTHON@": "python3", "@DATADIR@": "/usr/share"},
    )
    install_script_file(
        destdir,
        "/usr/bin/glib-genmarshal",
        SAFE_ROOT / "vendor/original/gobject/glib-genmarshal.in",
        {"@PYTHON@": "python3", "@VERSION@": "2.80.0"},
    )
    install_script_file(
        destdir,
        "/usr/bin/glib-mkenums",
        SAFE_ROOT / "vendor/original/gobject/glib-mkenums.in",
        {"@PYTHON@": "python3", "@VERSION@": "2.80.0"},
    )
    install_script_file(
        destdir,
        "/usr/bin/glib-gettextize",
        SAFE_ROOT / "vendor/original/tools/glib-gettextize.in",
        {
            "@PACKAGE@": "glib2.0",
            "@VERSION@": "2.80.0",
            "@prefix@": "/usr",
            "@datarootdir@": "/usr/share",
            "@datadir@": "/usr/share",
        },
    )
    install_script_file(
        destdir,
        "/usr/bin/gtester-report",
        SAFE_ROOT / "vendor/original/glib/gtester-report.in",
        {"@PYTHON@": "python3"},
    )


def build_profiles() -> set[str]:
    return {profile for profile in os.environ.get("DEB_BUILD_PROFILES", "").split() if profile}


def stage_introspection_data(destdir: Path, build_root: Path, multiarch: str) -> None:
    if "nogir" in build_profiles():
        return

    introspection_dir = build_root / "girepository" / "introspection"
    if not introspection_dir.exists():
        return

    gir_dir = destdir / "usr/lib" / multiarch / "gir-1.0"
    typelib_dir = destdir / "usr/lib" / multiarch / "girepository-1.0"
    ensure_dir(gir_dir)
    ensure_dir(typelib_dir)

    for artifact in sorted(introspection_dir.iterdir()):
        if artifact.suffix == ".gir":
            shutil.copy2(artifact, gir_dir / artifact.name)
        elif artifact.suffix == ".typelib":
            shutil.copy2(artifact, typelib_dir / artifact.name)


def maybe_copy_metadata_file(entry_path: str) -> bool:
    return entry_path.startswith("/usr/share/doc/") or entry_path.startswith("/usr/share/man/")


def build_artifact_map(build_root: Path, multiarch: str) -> dict[str, Path]:
    return {
        f"/usr/lib/{multiarch}/libglib-2.0.so.0.8000.0": build_root / "glib/libglib-2.0.so.0.8000.0",
        f"/usr/lib/{multiarch}/libglib-2.0.a": build_root / "glib/libglib-2.0.a",
        f"/usr/lib/{multiarch}/libgthread-2.0.so.0.8000.0": build_root / "gthread/libgthread-2.0.so.0.8000.0",
        f"/usr/lib/{multiarch}/libgthread-2.0.a": build_root / "gthread/libgthread-2.0.a",
        f"/usr/lib/{multiarch}/libgmodule-2.0.so.0.8000.0": build_root / "gmodule/libgmodule-2.0.so.0.8000.0",
        f"/usr/lib/{multiarch}/libgmodule-2.0.a": build_root / "gmodule/libgmodule-2.0.a",
        f"/usr/lib/{multiarch}/libgobject-2.0.so.0.8000.0": build_root / "gobject/libgobject-2.0.so.0.8000.0",
        f"/usr/lib/{multiarch}/libgobject-2.0.a": build_root / "gobject/libgobject-2.0.a",
        f"/usr/lib/{multiarch}/libgio-2.0.so.0.8000.0": build_root / "gio/libgio-2.0.so.0.8000.0",
        f"/usr/lib/{multiarch}/libgio-2.0.a": build_root / "gio/libgio-2.0.a",
        f"/usr/lib/{multiarch}/libgirepository-2.0.so.0.8000.0": build_root / "girepository/libgirepository-2.0.so.0.8000.0",
        f"/usr/lib/{multiarch}/libgirepository-2.0.a": build_root / "girepository/libgirepository-2.0.a",
    }


def stage_files(destdir: Path, build_root: Path, multiarch: str) -> None:
    install_map = installed_source_map()
    artifacts = build_artifact_map(build_root, multiarch)
    manifest = json.loads((SAFE_ROOT / "abi/install-manifests/abi-shell.json").read_text())

    for entry in manifest["entries"]:
        member_kind = entry["member_kind"]
        path = entry["path"]
        target = destdir / path.lstrip("/")
        if member_kind == "control":
            continue
        if member_kind == "directory":
            ensure_dir(target)
            continue
        if member_kind == "symlink":
            if path in {
                "/usr/bin/gi-compile-repository",
                "/usr/bin/gi-decompile-typelib",
                "/usr/bin/gi-inspect-typelib",
            }:
                continue
            ensure_dir(target.parent)
            if target.exists() or target.is_symlink():
                target.unlink()
            target.symlink_to(entry["target"])
            continue
        if member_kind != "file":
            raise ValueError(f"Unhandled manifest member kind: {member_kind}")
        if maybe_copy_metadata_file(path):
            continue
        if path in artifacts:
            ensure_dir(target.parent)
            shutil.copy2(artifacts[path], target)
            continue
        if path.startswith(f"/usr/lib/{multiarch}/pkgconfig/"):
            copy_or_write(target, render_install_pc(Path(path).name[:-3], multiarch))
            continue
        source = lookup_installed_source(install_map, path)
        if source is not None:
            ensure_dir(target.parent)
            shutil.copy2(source, target)
            continue

    stage_helpers(destdir, build_root, multiarch)
    stage_introspection_data(destdir, build_root, multiarch)
    patch_installed_runpaths(destdir, multiarch)

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--build-root", type=Path, required=True)
    parser.add_argument("--destdir", type=Path, required=True)
    parser.add_argument("--multiarch", required=True)
    args = parser.parse_args()

    destdir = args.destdir.resolve()
    if destdir.exists():
        shutil.rmtree(destdir)
    ensure_dir(destdir)
    stage_files(destdir, args.build_root.resolve(), args.multiarch)


if __name__ == "__main__":
    main()
