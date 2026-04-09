#!/usr/bin/env python3
import argparse
import os
import shutil
import subprocess
from pathlib import Path

from common import clean_subprocess_env, compare_jsonish, ensure_dir, read_json, run, write_json


ABI_SHELL_PACKAGES = {
    "libglib2.0-0t64",
    "libglib2.0-bin",
    "libglib2.0-data",
    "libglib2.0-dev",
    "libglib2.0-dev-bin",
    "libgirepository-2.0-0",
    "libgirepository-2.0-dev",
}
RUNTIME_PACKAGES = {
    "libglib2.0-0t64",
    "libglib2.0-bin",
    "libglib2.0-data",
    "libgirepository-2.0-0",
}
DEV_PACKAGES = {
    "libglib2.0-dev",
    "libglib2.0-dev-bin",
    "libgirepository-2.0-dev",
    "gir1.2-glib-2.0",
    "gir1.2-glib-2.0-dev",
    "gir1.2-girepository-3.0",
    "gir1.2-girepository-3.0-dev",
}
DOC_PACKAGES = {"libglib2.0-doc"}
UDEB_PACKAGES = {"libglib2.0-udeb"}

PATH_PRELOAD_SOURCE = r"""
#define _GNU_SOURCE
#include <dlfcn.h>
#include <dirent.h>
#include <fcntl.h>
#include <linux/stat.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

static const char *fake_root(void) {
    return getenv("DPKG_ROOT");
}

static bool should_map(const char *path) {
    if (!path || path[0] != '/')
        return false;
    return strcmp(path, "/usr/lib") == 0
        || strncmp(path, "/usr/lib/", 9) == 0
        || strcmp(path, "/usr/share") == 0
        || strncmp(path, "/usr/share/", 11) == 0
        || strcmp(path, "/var/lib/dpkg") == 0
        || strncmp(path, "/var/lib/dpkg/", 14) == 0;
}

static char *map_path(const char *path) {
    const char *root = fake_root();
    if (!root || !*root || !should_map(path))
        return NULL;
    size_t size = strlen(root) + strlen(path) + 1;
    char *mapped = malloc(size);
    if (!mapped)
        return NULL;
    snprintf(mapped, size, "%s%s", root, path);
    return mapped;
}

static const char *chosen_path(const char *path, char **mapped_out) {
    char *mapped = map_path(path);
    *mapped_out = mapped;
    return mapped ? mapped : path;
}

int access(const char *path, int mode) {
    static int (*real_fn)(const char *, int) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "access");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped), mode);
    free(mapped);
    return result;
}

int faccessat(int dirfd, const char *path, int mode, int flags) {
    static int (*real_fn)(int, const char *, int, int) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "faccessat");
    char *mapped = NULL;
    int result = real_fn(dirfd, chosen_path(path, &mapped), mode, flags);
    free(mapped);
    return result;
}

int stat(const char *path, struct stat *st) {
    static int (*real_fn)(const char *, struct stat *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "stat");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped), st);
    free(mapped);
    return result;
}

int lstat(const char *path, struct stat *st) {
    static int (*real_fn)(const char *, struct stat *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "lstat");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped), st);
    free(mapped);
    return result;
}

int stat64(const char *path, struct stat64 *st) {
    static int (*real_fn)(const char *, struct stat64 *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "stat64");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped), st);
    free(mapped);
    return result;
}

int __xstat(int ver, const char *path, struct stat *st) {
    static int (*real_fn)(int, const char *, struct stat *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "__xstat");
    char *mapped = NULL;
    int result = real_fn(ver, chosen_path(path, &mapped), st);
    free(mapped);
    return result;
}

int __xstat64(int ver, const char *path, struct stat64 *st) {
    static int (*real_fn)(int, const char *, struct stat64 *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "__xstat64");
    char *mapped = NULL;
    int result = real_fn(ver, chosen_path(path, &mapped), st);
    free(mapped);
    return result;
}

int __lxstat(int ver, const char *path, struct stat *st) {
    static int (*real_fn)(int, const char *, struct stat *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "__lxstat");
    char *mapped = NULL;
    int result = real_fn(ver, chosen_path(path, &mapped), st);
    free(mapped);
    return result;
}

int __lxstat64(int ver, const char *path, struct stat64 *st) {
    static int (*real_fn)(int, const char *, struct stat64 *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "__lxstat64");
    char *mapped = NULL;
    int result = real_fn(ver, chosen_path(path, &mapped), st);
    free(mapped);
    return result;
}

int fstatat(int dirfd, const char *path, struct stat *st, int flags) {
    static int (*real_fn)(int, const char *, struct stat *, int) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "fstatat");
    char *mapped = NULL;
    int result = real_fn(dirfd, chosen_path(path, &mapped), st, flags);
    free(mapped);
    return result;
}

int fstatat64(int dirfd, const char *path, struct stat64 *st, int flags) {
    static int (*real_fn)(int, const char *, struct stat64 *, int) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "fstatat64");
    char *mapped = NULL;
    int result = real_fn(dirfd, chosen_path(path, &mapped), st, flags);
    free(mapped);
    return result;
}

int __fxstatat(int ver, int dirfd, const char *path, struct stat *st, int flags) {
    static int (*real_fn)(int, int, const char *, struct stat *, int) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "__fxstatat");
    char *mapped = NULL;
    int result = real_fn(ver, dirfd, chosen_path(path, &mapped), st, flags);
    free(mapped);
    return result;
}

int __fxstatat64(int ver, int dirfd, const char *path, struct stat64 *st, int flags) {
    static int (*real_fn)(int, int, const char *, struct stat64 *, int) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "__fxstatat64");
    char *mapped = NULL;
    int result = real_fn(ver, dirfd, chosen_path(path, &mapped), st, flags);
    free(mapped);
    return result;
}

int statx(int dirfd, const char *path, int flags, unsigned int mask, struct statx *stx) {
    static int (*real_fn)(int, const char *, int, unsigned int, struct statx *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "statx");
    char *mapped = NULL;
    int result = real_fn(dirfd, chosen_path(path, &mapped), flags, mask, stx);
    free(mapped);
    return result;
}

int open(const char *path, int flags, ...) {
    static int (*real_fn)(const char *, int, ...) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "open");
    mode_t mode = 0;
    int has_mode = (flags & O_CREAT) != 0;
    if (has_mode) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, int);
        va_end(args);
    }
    char *mapped = NULL;
    int result = has_mode
        ? real_fn(chosen_path(path, &mapped), flags, mode)
        : real_fn(chosen_path(path, &mapped), flags);
    free(mapped);
    return result;
}

int open64(const char *path, int flags, ...) {
    static int (*real_fn)(const char *, int, ...) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "open64");
    mode_t mode = 0;
    int has_mode = (flags & O_CREAT) != 0;
    if (has_mode) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, int);
        va_end(args);
    }
    char *mapped = NULL;
    int result = has_mode
        ? real_fn(chosen_path(path, &mapped), flags, mode)
        : real_fn(chosen_path(path, &mapped), flags);
    free(mapped);
    return result;
}

int openat(int dirfd, const char *path, int flags, ...) {
    static int (*real_fn)(int, const char *, int, ...) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "openat");
    mode_t mode = 0;
    int has_mode = (flags & O_CREAT) != 0;
    if (has_mode) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, int);
        va_end(args);
    }
    char *mapped = NULL;
    int result = has_mode
        ? real_fn(dirfd, chosen_path(path, &mapped), flags, mode)
        : real_fn(dirfd, chosen_path(path, &mapped), flags);
    free(mapped);
    return result;
}

int openat64(int dirfd, const char *path, int flags, ...) {
    static int (*real_fn)(int, const char *, int, ...) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "openat64");
    mode_t mode = 0;
    int has_mode = (flags & O_CREAT) != 0;
    if (has_mode) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, int);
        va_end(args);
    }
    char *mapped = NULL;
    int result = has_mode
        ? real_fn(dirfd, chosen_path(path, &mapped), flags, mode)
        : real_fn(dirfd, chosen_path(path, &mapped), flags);
    free(mapped);
    return result;
}

DIR *opendir(const char *path) {
    static DIR *(*real_fn)(const char *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "opendir");
    char *mapped = NULL;
    DIR *result = real_fn(chosen_path(path, &mapped));
    free(mapped);
    return result;
}

int mkdir(const char *path, mode_t mode) {
    static int (*real_fn)(const char *, mode_t) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "mkdir");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped), mode);
    free(mapped);
    return result;
}

int mkdirat(int dirfd, const char *path, mode_t mode) {
    static int (*real_fn)(int, const char *, mode_t) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "mkdirat");
    char *mapped = NULL;
    int result = real_fn(dirfd, chosen_path(path, &mapped), mode);
    free(mapped);
    return result;
}

int unlink(const char *path) {
    static int (*real_fn)(const char *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "unlink");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped));
    free(mapped);
    return result;
}

int unlinkat(int dirfd, const char *path, int flags) {
    static int (*real_fn)(int, const char *, int) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "unlinkat");
    char *mapped = NULL;
    int result = real_fn(dirfd, chosen_path(path, &mapped), flags);
    free(mapped);
    return result;
}

int rmdir(const char *path) {
    static int (*real_fn)(const char *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "rmdir");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped));
    free(mapped);
    return result;
}

int remove(const char *path) {
    static int (*real_fn)(const char *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "remove");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped));
    free(mapped);
    return result;
}

int rename(const char *oldpath, const char *newpath) {
    static int (*real_fn)(const char *, const char *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "rename");
    char *old_mapped = NULL;
    char *new_mapped = NULL;
    int result = real_fn(chosen_path(oldpath, &old_mapped), chosen_path(newpath, &new_mapped));
    free(old_mapped);
    free(new_mapped);
    return result;
}

int chdir(const char *path) {
    static int (*real_fn)(const char *) = NULL;
    if (!real_fn)
        *(void **) (&real_fn) = dlsym(RTLD_NEXT, "chdir");
    char *mapped = NULL;
    int result = real_fn(chosen_path(path, &mapped));
    free(mapped);
    return result;
}
"""


def should_expect_udeb(profiles: str | None) -> bool:
    if not profiles:
        return True
    return "noudeb" not in profiles.split()


def recover_udeb(source_copy: Path) -> Path:
    package_dir = source_copy / "debian" / "libglib2.0-udeb"
    if package_dir.exists():
        shutil.rmtree(package_dir)
    ensure_dir(package_dir)
    run(["dh_install", "-plibglib2.0-udeb"], cwd=source_copy)
    run(
        [
            "fakeroot",
            "sh",
            "-ec",
            "dh_shlibdeps -plibglib2.0-udeb || true; "
            "dh_installdeb -plibglib2.0-udeb; "
            "dh_gencontrol -plibglib2.0-udeb; "
            "dh_md5sums -plibglib2.0-udeb; "
            "dh_builddeb -plibglib2.0-udeb",
        ],
        cwd=source_copy,
    )
    matches = sorted(source_copy.parent.glob("libglib2.0-udeb_*.udeb"))
    if not matches:
        raise FileNotFoundError("libglib2.0-udeb was not emitted by the fallback packaging path")
    return matches[-1]


def build_packages(source_dir: Path, work_root: Path, label: str, profiles: str | None) -> dict[str, Path]:
    build_root = work_root / label
    source_copy = build_root / "src"
    if build_root.exists():
        shutil.rmtree(build_root)
    shutil.copytree(source_dir, source_copy, symlinks=True)
    env_updates = {"DEB_BUILD_OPTIONS": "nocheck"}
    if profiles:
        env_updates["DEB_BUILD_PROFILES"] = profiles
    env = clean_subprocess_env(updates=env_updates)
    run(["dpkg-buildpackage", "-us", "-uc", "-b"], cwd=source_copy, env=env)
    files = {}
    for line in (source_copy / "debian" / "files").read_text().splitlines():
        if not line.strip():
            continue
        artifact = line.split()[0]
        if not (artifact.endswith(".deb") or artifact.endswith(".udeb")):
            continue
        package_name = artifact.split("_", 1)[0]
        files[package_name] = source_copy.parent / artifact
    if should_expect_udeb(profiles) and "libglib2.0-udeb" not in files:
        files["libglib2.0-udeb"] = recover_udeb(source_copy)
    return files


def archive_entries(package_name: str, package_path: Path) -> list[dict[str, str]]:
    temp_root = package_path.parent / f".extract-{package_name}"
    if temp_root.exists():
        shutil.rmtree(temp_root)
    data_dir = temp_root / "data"
    control_dir = temp_root / "control"
    ensure_dir(data_dir)
    ensure_dir(control_dir)
    run(["dpkg-deb", "-x", str(package_path), str(data_dir)])
    run(["dpkg-deb", "-e", str(package_path), str(control_dir)])
    entries = []
    for path in sorted(data_dir.rglob("*")):
        relpath = "/" + path.relative_to(data_dir).as_posix()
        if path.is_symlink():
            entries.append({"package": package_name, "member_kind": "symlink", "path": relpath, "target": os.readlink(path)})
        elif path.is_dir():
            entries.append({"package": package_name, "member_kind": "directory", "path": relpath})
        else:
            entries.append({"package": package_name, "member_kind": "file", "path": relpath})
    for path in sorted(control_dir.rglob("*")):
        if path.is_dir():
            continue
        relpath = "control/" + path.relative_to(control_dir).as_posix()
        entries.append({"package": package_name, "member_kind": "control", "path": relpath})
    shutil.rmtree(temp_root)
    return entries


def manifest_from_packages(name: str, packages: dict[str, Path], selected: set[str]) -> dict[str, object]:
    entries = []
    for package_name in sorted(selected):
        if package_name not in packages:
            raise FileNotFoundError(f"Expected package {package_name} is missing from the build output")
        entries.extend(archive_entries(package_name, packages[package_name]))
    entries.sort(key=lambda item: (item["package"], item["member_kind"], item["path"]))
    return {"manifest": name, "packages": sorted(selected), "entries": entries}


def multiarch_triplet() -> str:
    return subprocess.run(
        ["dpkg-architecture", "-qDEB_HOST_MULTIARCH"],
        env=clean_subprocess_env(),
        check=True,
        text=True,
        capture_output=True,
    ).stdout.strip()


def compile_path_preload(work_root: Path) -> Path:
    work_root = work_root.resolve()
    ensure_dir(work_root)
    source = work_root / "dpkg-path-preload.c"
    library = work_root / "dpkg-path-preload.so"
    source.write_text(PATH_PRELOAD_SOURCE)
    run(["cc", "-shared", "-fPIC", "-O2", "-o", str(library), str(source), "-ldl"])
    return library


def init_dpkg_root(root: Path) -> None:
    root = root.resolve()
    if root.exists():
        shutil.rmtree(root)
    ensure_dir(root / "var/lib/dpkg/updates")
    ensure_dir(root / "var/log")
    ensure_dir(root / "tmp")
    ensure_dir(root / "usr/share")
    ensure_dir(root / "usr/lib")
    (root / "var/lib/dpkg" / "status").write_text("")


def dpkg_env(root: Path, preload: Path) -> dict[str, str]:
    root = root.resolve()
    preload = preload.resolve()
    return clean_subprocess_env(
        updates={
            "DPKG_ROOT": str(root),
            "DPKG_ADMINDIR": str(root / "var/lib/dpkg"),
            "LD_PRELOAD": str(preload),
        }
    )


def run_dpkg(root: Path, preload: Path, args: list[str]) -> None:
    root = root.resolve()
    run(
        [
            "dpkg",
            f"--root={root}",
            f"--admindir={root / 'var/lib/dpkg'}",
            "--force-not-root",
            "--force-script-chrootless",
            "--force-depends",
            *args,
        ],
        env=dpkg_env(root, preload),
    )


def dpkg_status_map(root: Path) -> dict[str, str]:
    status_file = root / "var/lib/dpkg/status"
    package_status = {}
    current_package = None
    current_status = None
    for line in status_file.read_text().splitlines():
        if not line:
            if current_package is not None and current_status is not None:
                package_status[current_package] = current_status
            current_package = None
            current_status = None
            continue
        if line.startswith("Package: "):
            current_package = line.removeprefix("Package: ").strip()
        elif line.startswith("Status: "):
            current_status = line.removeprefix("Status: ").strip()
    if current_package is not None and current_status is not None:
        package_status[current_package] = current_status
    return package_status


def assert_installed_status(root: Path, packages: set[str]) -> None:
    status_map = dpkg_status_map(root)
    for package in sorted(packages):
        status = status_map.get(package)
        if status != "install ok installed":
            raise ValueError(f"Package {package} was not fully installed in the dpkg proof root: {status!r}")


def assert_purged_status(root: Path, packages: set[str]) -> None:
    status_map = dpkg_status_map(root)
    for package in sorted(packages):
        status = status_map.get(package)
        if status is None:
            continue
        if status.endswith("not-installed"):
            continue
        raise ValueError(f"Package {package} was not purged from the dpkg proof root: {status!r}")


def tracked_runtime_state_entries(multiarch: str) -> list[dict[str, str]]:
    return [
        {
            "package": "libglib2.0-0t64",
            "path": "/usr/share/glib-2.0/schemas/gschemas.compiled",
            "responsible": f"/usr/lib/{multiarch}/glib-2.0/glib-compile-schemas",
        },
        {
            "package": "libglib2.0-0t64",
            "path": f"/usr/lib/{multiarch}/gio/modules/giomodule.cache",
            "responsible": f"/usr/lib/{multiarch}/glib-2.0/gio-querymodules",
        },
    ]


def observe_runtime_state(root: Path, entries: list[dict[str, str]], key: str) -> list[dict[str, str]]:
    observed = []
    for entry in entries:
        target = root / entry["path"].lstrip("/")
        item = dict(entry)
        item[key] = "present" if target.exists() else "absent"
        observed.append(item)
    return observed


def install_and_purge_runtime_state(runtime_packages: dict[str, Path], work_root: Path) -> dict[str, object]:
    multiarch = multiarch_triplet()
    work_root = work_root.resolve()
    root = work_root / "runtime-root"
    preload = compile_path_preload(work_root)
    init_dpkg_root(root)
    package_paths = [str(runtime_packages[name]) for name in sorted(runtime_packages)]
    run_dpkg(root, preload, ["-i", *package_paths])
    run_dpkg(root, preload, ["--triggers-only", "-a"])
    assert_installed_status(root, set(runtime_packages))
    entries = observe_runtime_state(root, tracked_runtime_state_entries(multiarch), "expected_after_install")

    run_dpkg(root, preload, ["--purge", *sorted(runtime_packages)])
    run_dpkg(root, preload, ["--triggers-only", "-a"])
    assert_purged_status(root, set(runtime_packages))
    entries = observe_runtime_state(root, entries, "expected_after_purge")
    return {"multiarch": multiarch, "entries": entries}


def generate_all(source_dir: Path, work_root: Path, abi_shell_profiles: str) -> tuple[dict[str, object], dict[str, object], dict[str, object], dict[str, object], dict[str, object], dict[str, object], dict[str, object]]:
    shell_packages = build_packages(source_dir, work_root, "shell", abi_shell_profiles)
    full_packages = build_packages(source_dir, work_root, "full", "")
    abi_shell = manifest_from_packages("abi-shell", shell_packages, ABI_SHELL_PACKAGES & shell_packages.keys())
    runtime = manifest_from_packages("runtime", full_packages, RUNTIME_PACKAGES)
    dev_package = manifest_from_packages("dev-package", full_packages, DEV_PACKAGES)
    doc_package = manifest_from_packages("doc-package", full_packages, DOC_PACKAGES)
    udeb = manifest_from_packages("udeb", full_packages, UDEB_PACKAGES)
    full = manifest_from_packages("full", full_packages, set(full_packages))
    runtime_state = install_and_purge_runtime_state(
        {name: full_packages[name] for name in sorted(RUNTIME_PACKAGES)},
        work_root,
    )
    return abi_shell, runtime, dev_package, doc_package, udeb, full, runtime_state


def write_manifests(manifest_dir: Path, postinst_manifest: Path, generated: tuple[dict[str, object], ...]) -> None:
    abi_shell, runtime, dev_package, doc_package, udeb, full, runtime_state = generated
    write_json(manifest_dir / "abi-shell.json", abi_shell)
    write_json(manifest_dir / "runtime.json", runtime)
    write_json(manifest_dir / "dev-package.json", dev_package)
    write_json(manifest_dir / "doc-package.json", doc_package)
    write_json(manifest_dir / "udeb.json", udeb)
    write_json(manifest_dir / "full.json", full)
    write_json(postinst_manifest, runtime_state)


def verify(manifest_dir: Path, postinst_manifest: Path, generated: tuple[dict[str, object], ...]) -> None:
    abi_shell, runtime, dev_package, doc_package, udeb, full, runtime_state = generated
    compare_jsonish(read_json(manifest_dir / "abi-shell.json"), abi_shell)
    compare_jsonish(read_json(manifest_dir / "runtime.json"), runtime)
    compare_jsonish(read_json(manifest_dir / "dev-package.json"), dev_package)
    compare_jsonish(read_json(manifest_dir / "doc-package.json"), doc_package)
    compare_jsonish(read_json(manifest_dir / "udeb.json"), udeb)
    compare_jsonish(read_json(manifest_dir / "full.json"), full)
    compare_jsonish(read_json(postinst_manifest), runtime_state)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--work-root", type=Path, required=True)
    parser.add_argument("--abi-shell-profiles", required=True)
    parser.add_argument("--install-manifests", type=Path, required=True)
    parser.add_argument("--postinst-manifest", type=Path, required=True)
    parser.add_argument("--write-manifests", action="store_true")
    args = parser.parse_args()
    generated = generate_all(
        args.source.resolve(),
        args.work_root.resolve(),
        args.abi_shell_profiles,
    )
    if args.write_manifests:
        write_manifests(args.install_manifests.resolve(), args.postinst_manifest.resolve(), generated)
    else:
        verify(args.install_manifests.resolve(), args.postinst_manifest.resolve(), generated)


if __name__ == "__main__":
    main()
