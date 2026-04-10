#!/usr/bin/env python3
import argparse
import shutil
from pathlib import Path

from common import (
    BUILD_RELATIVE_ROOTS,
    REPO_ROOT,
    SAFE_ROOT,
    VENDOR_BUILD_CHECK,
    VENDOR_ORIGINAL,
    clean_subprocess_env,
    ensure_dir,
    read_json,
    run,
    write_json,
)


LIBRARIES = [
    {
        "crate": "safe-glib",
        "cargo_stem": "safe_glib",
        "out_dir": "glib",
        "soname": "libglib-2.0.so.0",
        "realname": "libglib-2.0.so.0.8000.0",
        "static": "libglib-2.0.a",
        "link_name": "libglib-2.0.so",
        "version_script": SAFE_ROOT / "abi" / "version-scripts" / "libglib.map",
    },
    {
        "crate": "safe-gthread",
        "cargo_stem": "safe_gthread",
        "out_dir": "gthread",
        "soname": "libgthread-2.0.so.0",
        "realname": "libgthread-2.0.so.0.8000.0",
        "static": "libgthread-2.0.a",
        "link_name": "libgthread-2.0.so",
        "version_script": SAFE_ROOT / "abi" / "version-scripts" / "libgthread.map",
    },
    {
        "crate": "safe-gmodule",
        "cargo_stem": "safe_gmodule",
        "out_dir": "gmodule",
        "soname": "libgmodule-2.0.so.0",
        "realname": "libgmodule-2.0.so.0.8000.0",
        "static": "libgmodule-2.0.a",
        "link_name": "libgmodule-2.0.so",
        "version_script": SAFE_ROOT / "abi" / "version-scripts" / "libgmodule.map",
    },
    {
        "crate": "safe-gobject",
        "cargo_stem": "safe_gobject",
        "out_dir": "gobject",
        "soname": "libgobject-2.0.so.0",
        "realname": "libgobject-2.0.so.0.8000.0",
        "static": "libgobject-2.0.a",
        "link_name": "libgobject-2.0.so",
        "version_script": SAFE_ROOT / "abi" / "version-scripts" / "libgobject.map",
    },
    {
        "crate": "safe-gio",
        "cargo_stem": "safe_gio",
        "out_dir": "gio",
        "soname": "libgio-2.0.so.0",
        "realname": "libgio-2.0.so.0.8000.0",
        "static": "libgio-2.0.a",
        "link_name": "libgio-2.0.so",
        "version_script": SAFE_ROOT / "abi" / "version-scripts" / "libgio.map",
    },
    {
        "crate": "safe-girepository",
        "cargo_stem": "safe_girepository",
        "out_dir": "girepository",
        "soname": "libgirepository-2.0.so.0",
        "realname": "libgirepository-2.0.so.0.8000.0",
        "static": "libgirepository-2.0.a",
        "link_name": "libgirepository-2.0.so",
        "version_script": SAFE_ROOT / "abi" / "version-scripts" / "libgirepository.map",
    },
]
AUTHORITATIVE_BUILD_ROOT = REPO_ROOT / "build-check"
AUTHORITATIVE_ORIGINAL_ROOT = REPO_ROOT / "original"


def replace_path(path: Path) -> None:
    if path.is_symlink() or path.is_file():
        path.unlink()
    elif path.is_dir():
        shutil.rmtree(path)


def symlink(path: Path, target: Path | str) -> None:
    if path.exists() or path.is_symlink():
        replace_path(path)
    path.symlink_to(target)


def render_pc_file(name: str, build_root: Path, multiarch: str) -> str:
    original = VENDOR_ORIGINAL
    common_cflags = " ".join(
        [
            f"-I{original}",
            f"-I{original / 'glib'}",
            f"-I{original / 'gmodule'}",
            f"-I{original / 'gobject'}",
            f"-I{original / 'gio'}",
            f"-I{original / 'girepository'}",
            f"-I{build_root}",
            f"-I{build_root / 'glib'}",
            f"-I{build_root / 'gmodule'}",
            f"-I{build_root / 'gio'}",
            f"-I{build_root / 'girepository'}",
        ]
    )
    libdir = {
        "glib-2.0": build_root / "glib",
        "gthread-2.0": build_root / "gthread",
        "gmodule-2.0": build_root / "gmodule",
        "gmodule-export-2.0": build_root / "gmodule",
        "gmodule-no-export-2.0": build_root / "gmodule",
        "gobject-2.0": build_root / "gobject",
        "gio-2.0": build_root / "gio",
        "gio-unix-2.0": build_root / "gio",
        "girepository-2.0": build_root / "girepository",
    }[name]
    libs = {
        "glib-2.0": "-lglib-2.0",
        "gthread-2.0": "-lgthread-2.0",
        "gmodule-2.0": "-lgmodule-2.0",
        "gmodule-export-2.0": "-lgmodule-2.0",
        "gmodule-no-export-2.0": "-lgmodule-2.0",
        "gobject-2.0": "-lgobject-2.0",
        "gio-2.0": "-lgio-2.0",
        "gio-unix-2.0": "-lgio-2.0",
        "girepository-2.0": "-lgirepository-2.0",
    }[name]
    requires = {
        "glib-2.0": "",
        "gthread-2.0": "Requires: glib-2.0\n",
        "gmodule-2.0": "Requires: glib-2.0\n",
        "gmodule-export-2.0": "Requires: glib-2.0\n",
        "gmodule-no-export-2.0": "Requires: glib-2.0\n",
        "gobject-2.0": "Requires: glib-2.0\n",
        "gio-2.0": "Requires: glib-2.0, gobject-2.0\n",
        "gio-unix-2.0": "Requires: gobject-2.0, gio-2.0\n",
        "girepository-2.0": "Requires: glib-2.0, gobject-2.0, gio-2.0\n",
    }[name]
    variables = ""
    if name == "glib-2.0":
        variables = "\n".join(
            [
                f"glib_genmarshal={SAFE_ROOT / 'vendor' / 'original' / 'gobject' / 'glib-genmarshal.in'}",
                f"gobject_query={build_root / 'gobject' / 'gobject-query'}",
                f"glib_mkenums={SAFE_ROOT / 'vendor' / 'original' / 'gobject' / 'glib-mkenums.in'}",
                f"glib_valgrind_suppressions={SAFE_ROOT / 'vendor' / 'original' / 'tools' / 'glib.supp'}",
                "",
            ]
        )
    elif name == "gio-2.0":
        variables = "\n".join(
            [
                "schemasdir=/usr/share/glib-2.0/schemas",
                "dtdsdir=/usr/share/glib-2.0/dtds",
                f"giomoduledir=/usr/lib/{multiarch}/gio/modules",
                f"gio={build_root / 'gio' / 'gio'}",
                f"gio_querymodules={build_root / 'gio' / 'gio-querymodules'}",
                f"glib_compile_schemas={build_root / 'gio' / 'glib-compile-schemas'}",
                f"glib_compile_resources={build_root / 'gio' / 'glib-compile-resources'}",
                f"gdbus={build_root / 'gio' / 'gdbus'}",
                f"gdbus_codegen={build_root / 'gio' / 'gdbus-2.0' / 'codegen' / 'gdbus-codegen'}",
                f"gresource={build_root / 'gio' / 'gresource'}",
                f"gsettings={build_root / 'gio' / 'gsettings'}",
                "",
            ]
        )
    elif name == "girepository-2.0":
        variables = "\n".join(
            [
                "gidatadir=/usr/share/gobject-introspection-1.0",
                "girdir=/usr/share/gir-1.0",
                f"typelibdir=/usr/lib/{multiarch}/girepository-1.0",
                "",
            ]
        )
    description = {
        "glib-2.0": ("GLib", "C Utility Library"),
        "gthread-2.0": ("GThread", "Thread support for GLib"),
        "gmodule-2.0": ("GModule", "Dynamic module loader for GLib"),
        "gmodule-export-2.0": ("GModule", "Dynamic module loader for GLib"),
        "gmodule-no-export-2.0": ("GModule", "Dynamic module loader for GLib"),
        "gobject-2.0": ("GObject", "GLib Type, Object, Parameter and Signal Library"),
        "gio-2.0": ("GIO", "glib I/O library"),
        "gio-unix-2.0": ("GIO unix specific APIs", "unix specific headers for glib I/O library"),
        "girepository-2.0": ("girepository", "GObject Introspection repository parser"),
    }[name]
    return (
        f"prefix={SAFE_ROOT}\n"
        f"libdir={libdir}\n"
        f"includedir={original}\n"
        f"generated_includedir={build_root}\n"
        f"glibconfigdir={build_root / 'glib'}\n"
        f"{variables}"
        f"Name: {description[0]}\n"
        f"Description: {description[1]}\n"
        "Version: 2.80.0\n"
        f"{requires}"
        f"Libs: -L${{libdir}} {libs}\n"
        "Libs.private: -ldl -lpthread -lm\n"
        f"Cflags: {common_cflags}\n"
    )


def write_pkgconfig(build_root: Path, multiarch: str) -> None:
    pkg_dir = build_root / "pkgconfig"
    ensure_dir(pkg_dir)
    for name in [
        "glib-2.0",
        "gthread-2.0",
        "gmodule-2.0",
        "gmodule-export-2.0",
        "gmodule-no-export-2.0",
        "gobject-2.0",
        "gio-2.0",
        "gio-unix-2.0",
        "girepository-2.0",
    ]:
        (pkg_dir / f"{name}.pc").write_text(render_pc_file(name, build_root, multiarch))


def overlay_runtime_env(build_root: Path) -> dict[str, str]:
    library_dirs = [
        build_root / "glib",
        build_root / "gthread",
        build_root / "gmodule",
        build_root / "gobject",
        build_root / "gio",
        build_root / "girepository",
    ]
    return clean_subprocess_env(
        updates={
            "PKG_CONFIG_PATH": str(build_root / "pkgconfig"),
            "LD_LIBRARY_PATH": ":".join(str(path) for path in library_dirs),
        }
    )


def rebuild_test_overlays(build_root: Path) -> None:
    overlays = [
        (
            SAFE_ROOT / "tests" / "upstream" / "glib" / "markup-collect.c",
            build_root / "glib" / "tests" / "markup-collect",
            ["glib-2.0"],
        ),
    ]
    env = overlay_runtime_env(build_root)
    for source, output, pkg_modules in overlays:
        output.parent.mkdir(parents=True, exist_ok=True)
        pkg_config = run(
            ["pkg-config", "--cflags", "--libs", *pkg_modules],
            cwd=SAFE_ROOT,
            env=env,
            capture=True,
        ).stdout.strip()
        cmd = ["gcc", str(source), "-o", str(output)]
        if pkg_config:
            cmd.extend(pkg_config.split())
        run(cmd, cwd=SAFE_ROOT, env=env)


def build_libraries(build_root: Path, target_dir: Path) -> None:
    for library in LIBRARIES:
        env = clean_subprocess_env(
            updates={
                "SAFE_LINK_SONAME": library["soname"],
                "SAFE_LINK_VERSION_SCRIPT": str(library["version_script"]),
            }
        )
        run(
            ["cargo", "build", "-p", library["crate"], "--target-dir", str(target_dir)],
            cwd=SAFE_ROOT,
            env=env,
        )
        cargo_root = target_dir / "debug"
        out_dir = build_root / library["out_dir"]
        ensure_dir(out_dir)
        cargo_so = cargo_root / f"lib{library['cargo_stem']}.so"
        cargo_a = cargo_root / f"lib{library['cargo_stem']}.a"
        realname = out_dir / library["realname"]
        link_name = out_dir / library["link_name"]
        soname = out_dir / library["soname"]
        static_lib = out_dir / library["static"]
        realname.write_bytes(cargo_so.read_bytes())
        static_lib.write_bytes(cargo_a.read_bytes())
        symlink(soname, library["realname"])
        symlink(link_name, library["realname"])


def rewrite_paths(value: object, *, build_root: Path) -> object:
    if isinstance(value, str):
        return (
            value.replace(str(AUTHORITATIVE_BUILD_ROOT / ".." / "original"), str(VENDOR_ORIGINAL))
            .replace(str(AUTHORITATIVE_ORIGINAL_ROOT), str(VENDOR_ORIGINAL))
            .replace(str(AUTHORITATIVE_BUILD_ROOT), str(build_root))
        )
    if isinstance(value, list):
        return [rewrite_paths(item, build_root=build_root) for item in value]
    if isinstance(value, dict):
        return {
            key: rewrite_paths(item, build_root=build_root)
            for key, item in value.items()
        }
    return value


def stage_directory_copy(build_root: Path, relative: str) -> None:
    source = VENDOR_BUILD_CHECK / relative
    if not source.exists():
        return
    destination = build_root / relative
    if destination.exists() or destination.is_symlink():
        replace_path(destination)
    shutil.copytree(source, destination, symlinks=False)


def stage_meson_private(build_root: Path) -> None:
    source_dir = VENDOR_BUILD_CHECK / "meson-private"
    destination_dir = build_root / "meson-private"
    ensure_dir(destination_dir)
    for pc_file in source_dir.glob("*.pc"):
        shutil.copy2(pc_file, destination_dir / pc_file.name)


def stage_intro_tests(build_root: Path) -> None:
    intro_tests = rewrite_paths(
        read_json(VENDOR_BUILD_CHECK / "meson-info" / "intro-tests.json"),
        build_root=build_root,
    )
    ensure_dir(build_root / "meson-info")
    write_json(build_root / "meson-info" / "intro-tests.json", intro_tests)


def stage_authoritative_build(build_root: Path) -> None:
    # Copy the frozen test/tool baseline into the build root so helper binaries
    # keep their expected layout while still resolving local safe-built libraries.
    for relative in sorted(BUILD_RELATIVE_ROOTS - {"meson-info", "meson-private"}):
        stage_directory_copy(build_root, relative)
    stage_meson_private(build_root)
    stage_intro_tests(build_root)


def export_layouts(build_root: Path) -> None:
    run(
        ["python3", "tools/export-rust-layouts.py", "--output-dir", str(build_root / "rust-layouts")],
        cwd=SAFE_ROOT,
    )


def touch_stamp(stamp: Path) -> None:
    ensure_dir(stamp.parent)
    stamp.write_text("ok\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--build-root", type=Path, required=True)
    parser.add_argument("--multiarch", required=True)
    parser.add_argument("--stamp", type=Path)
    args = parser.parse_args()

    build_root = args.build_root.resolve()
    ensure_dir(build_root)
    target_dir = build_root / "cargo-target"
    stage_authoritative_build(build_root)
    build_libraries(build_root, target_dir)
    write_pkgconfig(build_root, args.multiarch)
    rebuild_test_overlays(build_root)
    export_layouts(build_root)
    if args.stamp:
        touch_stamp(args.stamp)


if __name__ == "__main__":
    main()
