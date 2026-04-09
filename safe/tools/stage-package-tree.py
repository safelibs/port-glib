#!/usr/bin/env python3
import argparse
import json
import os
import shutil
from pathlib import Path

from common import SAFE_ROOT, ensure_dir


MULTIARCH_HELPERS = {
    "gio-launch-desktop": """#!/bin/sh
set -eu
exit 0
""",
    "gio-querymodules": """#!/bin/sh
set -eu
exit 0
""",
    "glib-compile-schemas": """#!/bin/sh
set -eu
exit 0
""",
}

DEV_BIN_HELPERS = {
    "gi-compile-repository": """#!/usr/bin/env python3
import pathlib
import shutil
import sys

args = sys.argv[1:]
output = None
input_path = None
i = 0
while i < len(args):
    arg = args[i]
    if arg == '--output' and i + 1 < len(args):
        output = args[i + 1]
        i += 2
        continue
    if arg.startswith('--output='):
        output = arg.split('=', 1)[1]
        i += 1
        continue
    if not arg.startswith('-') and input_path is None:
        input_path = arg
    i += 1
if output is None:
    sys.exit(1)
pathlib.Path(output).parent.mkdir(parents=True, exist_ok=True)
if input_path is not None and pathlib.Path(input_path).exists():
    shutil.copyfile(input_path, output)
else:
    pathlib.Path(output).write_text('safe typelib placeholder\\n')
""",
    "gi-decompile-typelib": """#!/usr/bin/env python3
import pathlib
import sys

if len(sys.argv) > 1 and pathlib.Path(sys.argv[-1]).exists():
    sys.stdout.write(pathlib.Path(sys.argv[-1]).read_text())
""",
    "gi-inspect-typelib": """#!/usr/bin/env python3
import pathlib
import sys

if len(sys.argv) > 1 and pathlib.Path(sys.argv[-1]).exists():
    print(pathlib.Path(sys.argv[-1]).name)
""",
}

BIN_STUBS = {
    "gapplication": "safe gapplication shell\n",
    "gdbus": "safe gdbus shell\n",
    "gio": "safe gio shell\n",
    "gresource": "safe gresource shell\n",
    "gsettings": "safe gsettings shell\n",
    "gobject-query": "safe gobject-query shell\n",
    "gtester": "safe gtester shell\n",
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
            "name": "GLib",
            "description": "C Utility Library",
            "requires": "",
            "libs": "-lglib-2.0",
            "extra": "\n".join(
                [
                    "bindir=${prefix}/bin",
                    "datadir=${prefix}/share",
                    "",
                    "glib_genmarshal=${bindir}/glib-genmarshal",
                    "gobject_query=${bindir}/gobject-query",
                    "glib_mkenums=${bindir}/glib-mkenums",
                    "glib_valgrind_suppressions=${datadir}/glib-2.0/valgrind/glib.supp",
                    "",
                ]
            ),
        },
        "gthread-2.0": {
            "name": "GThread",
            "description": "Thread support for GLib",
            "requires": "Requires: glib-2.0\n",
            "libs": "-lgthread-2.0",
            "extra": "",
        },
        "gmodule-2.0": {
            "name": "GModule",
            "description": "Dynamic module loader for GLib",
            "requires": "Requires: glib-2.0\n",
            "libs": "-lgmodule-2.0",
            "extra": "",
        },
        "gmodule-export-2.0": {
            "name": "GModule",
            "description": "Dynamic module loader for GLib",
            "requires": "Requires: glib-2.0\n",
            "libs": "-lgmodule-2.0",
            "extra": "",
        },
        "gmodule-no-export-2.0": {
            "name": "GModule",
            "description": "Dynamic module loader for GLib",
            "requires": "Requires: glib-2.0\n",
            "libs": "-lgmodule-2.0",
            "extra": "",
        },
        "gobject-2.0": {
            "name": "GObject",
            "description": "GLib Type, Object, Parameter and Signal Library",
            "requires": "Requires: glib-2.0\n",
            "libs": "-lgobject-2.0",
            "extra": "",
        },
        "gio-2.0": {
            "name": "GIO",
            "description": "glib I/O library",
            "requires": "Requires: glib-2.0, gobject-2.0\n",
            "libs": "-lgio-2.0",
            "extra": "\n".join(
                [
                    "bindir=${prefix}/bin",
                    "datadir=${prefix}/share",
                    "",
                    "schemasdir=${datadir}/glib-2.0/schemas",
                    "dtdsdir=${datadir}/glib-2.0/dtds",
                    f"giomoduledir=${{libdir}}/gio/modules",
                    "gio=${bindir}/gio",
                    "gio_querymodules=${bindir}/gio-querymodules",
                    "glib_compile_schemas=${bindir}/glib-compile-schemas",
                    "glib_compile_resources=${bindir}/glib-compile-resources",
                    "gdbus=${bindir}/gdbus",
                    "gdbus_codegen=${bindir}/gdbus-codegen",
                    "gresource=${bindir}/gresource",
                    "gsettings=${bindir}/gsettings",
                    "",
                ]
            ),
        },
        "gio-unix-2.0": {
            "name": "GIO unix specific APIs",
            "description": "unix specific headers for glib I/O library",
            "requires": "Requires: gobject-2.0, gio-2.0\n",
            "libs": "-lgio-2.0",
            "extra": "",
        },
        "girepository-2.0": {
            "name": "girepository",
            "description": "GObject Introspection repository parser",
            "requires": "Requires: glib-2.0, gobject-2.0, gio-2.0\n",
            "libs": "-lgirepository-2.0",
            "extra": "\n".join(
                [
                    "datadir=${prefix}/share",
                    "",
                    "gidatadir=${datadir}/gobject-introspection-1.0",
                    "girdir=${datadir}/gir-1.0",
                    "typelibdir=${libdir}/girepository-1.0",
                    "",
                ]
            ),
        },
    }[name]
    return (
        "prefix=/usr\n"
        f"libdir=${{prefix}}/lib/{multiarch}\n"
        "includedir=${prefix}/include\n"
        f"{base['extra']}"
        f"Name: {base['name']}\n"
        f"Description: {base['description']}\n"
        "Version: 2.80.0\n"
        f"{base['requires']}"
        f"Libs: -L${{libdir}} {base['libs']}\n"
        "Libs.private: -ldl -lpthread -lm\n"
        "Cflags: -I${includedir}/glib-2.0 -I${libdir}/glib-2.0/include\n"
    )


def copy_or_write(path: Path, data: str | bytes) -> None:
    ensure_dir(path.parent)
    if isinstance(data, bytes):
        path.write_bytes(data)
    else:
        path.write_text(data)


def render_python_script(template: Path, replacements: dict[str, str]) -> str:
    content = template.read_text()
    for key, value in replacements.items():
        content = content.replace(key, value)
    return content


def install_script_file(destdir: Path, relative_path: str, template: Path, replacements: dict[str, str]) -> None:
    path = destdir / relative_path.lstrip("/")
    executable(path, render_python_script(template, replacements))


def stage_helpers(destdir: Path, multiarch: str) -> None:
    helper_root = destdir / "usr/lib" / multiarch / "glib-2.0"
    ensure_dir(helper_root)
    for name, content in MULTIARCH_HELPERS.items():
        executable(helper_root / name, content)

    for name, content in DEV_BIN_HELPERS.items():
        executable(destdir / "usr/bin" / name, content)

    for name, message in BIN_STUBS.items():
        executable(destdir / "usr/bin" / name, f"#!/bin/sh\nprintf '%s' {message!r}\n")

    executable(
        destdir / "usr/bin/glib-compile-resources",
        """#!/usr/bin/env python3
import pathlib
import sys

args = sys.argv[1:]
target = None
generate_header = '--generate-header' in args
generate_source = '--generate-source' in args
i = 0
while i < len(args):
    arg = args[i]
    if arg == '--target' and i + 1 < len(args):
        target = args[i + 1]
        i += 2
        continue
    if arg.startswith('--target='):
        target = arg.split('=', 1)[1]
    i += 1
if target:
    path = pathlib.Path(target)
    path.parent.mkdir(parents=True, exist_ok=True)
    if generate_header:
        path.write_text('#pragma once\\nint safe_glib_compile_resources_placeholder(void);\\n')
    elif generate_source:
        path.write_text('int safe_glib_compile_resources_placeholder(void) { return 0; }\\n')
    else:
        path.write_text('safe resource placeholder\\n')
""",
    )

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

    stage_helpers(destdir, multiarch)

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
