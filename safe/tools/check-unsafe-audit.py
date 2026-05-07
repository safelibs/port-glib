#!/usr/bin/env python3
import argparse
import re
from pathlib import Path


EXPECTED_UNSAFE_COUNTS = {
    "crates/abi-support/src/ffi.rs": 2,
    "crates/gio/build.rs": 2,
    "crates/gio/src/generated_compat.rs": 74,
    "crates/gio/src/lib.rs": 2,
    "crates/gio/src/translated": 19182,
    "crates/girepository/src/exports.rs": 102,
    "crates/girepository/src/runtime.rs": 136,
    "crates/glib/build.rs": 2,
    "crates/glib/src/bytes/api.rs": 3,
    "crates/glib/src/charset/api.rs": 20,
    "crates/glib/src/data.rs": 17,
    "crates/glib/src/fileutils/api.rs": 3,
    "crates/glib/src/gvariant/api.rs": 13,
    "crates/glib/src/hash/api.rs": 132,
    "crates/glib/src/keyfile/api.rs": 3,
    "crates/glib/src/legacy.rs": 30,
    "crates/glib/src/markup/api.rs": 14,
    "crates/glib/src/spawn/api.rs": 20,
    "crates/glib/src/support.rs": 17,
    "crates/glib/src/translated": 3851,
    "crates/gmodule/src/module_api.rs": 20,
    "crates/gmodule/src/runtime.rs": 13,
    "crates/gobject/build.rs": 7,
    "crates/gobject/src/object/mod.rs": 14,
    "crates/gobject/src/signal/mod.rs": 2,
    "crates/gobject/src/translated": 2539,
    "crates/gobject/src/type_system/mod.rs": 17,
    "crates/gobject/src/value/mod.rs": 1,
    "crates/gthread/src/compat.rs": 4,
    "crates/gthread/src/runtime.rs": 2,
}

AUDIT_CLASSES = {
    "crates/abi-support/src/ffi.rs": "C callback typedefs shared by exported ABI structs.",
    "crates/gio/build.rs": "Build-time alias parser string patterns only.",
    "crates/gio/src/generated_compat.rs": "Rust-owned replacements for generated GIO enum, portal, and D-Bus helper ABI symbols with raw C out-parameters.",
    "crates/gio/src/lib.rs": "C callback typedefs and translated-crate lint compatibility.",
    "crates/gio/src/translated": "Generated Rust translation of upstream C plus final Rust-owned file-monitor/settings polling fallbacks; build-check-only generated modules are excluded.",
    "crates/girepository/src/exports.rs": "Rust-owned GIRepository ABI entrypoints with raw C pointer ingress and out parameters.",
    "crates/girepository/src/runtime.rs": "Rust-owned GIRepository runtime over GObject type registration, C string vectors, stack-loaded GI structs, and GError out parameters.",
    "crates/glib/build.rs": "Build-time alias parser string patterns only.",
    "crates/glib/src/bytes/api.rs": "Rust-owned GLib byte-array ABI wrapper over translated storage.",
    "crates/glib/src/charset/api.rs": "Rust-owned charset ABI wrappers around process identity and environment-sensitive C calls.",
    "crates/glib/src/data.rs": "Exported global data symbols required by GLib ABI consumers.",
    "crates/glib/src/fileutils/api.rs": "Rust-owned filesystem ABI wrapper over translated canonicalization.",
    "crates/glib/src/gvariant/api.rs": "Rust-owned GVariant validation wrapper over serialized C ABI pointers.",
    "crates/glib/src/hash/api.rs": "Rust-owned GHashTable implementation; raw pointers model GLib ownership and callback ABI.",
    "crates/glib/src/keyfile/api.rs": "Rust-owned key-file ABI wrapper over translated parser state.",
    "crates/glib/src/legacy.rs": "Internal ABI dispatch layer preserving exported GLib symbol behavior.",
    "crates/glib/src/markup/api.rs": "Rust-owned markup parser ABI wrapper and GError out-parameter handling.",
    "crates/glib/src/spawn/api.rs": "Rust-owned spawning ABI wrapper for C argv/envp and pipe out-parameters.",
    "crates/glib/src/support.rs": "Process-global C runtime support symbols required by translated GLib modules.",
    "crates/glib/src/translated": "Generated Rust translation of upstream C plus final TAP subprocess output compatibility; retained only at the C ABI boundary.",
    "crates/gmodule/src/module_api.rs": "Exported GModule ABI entrypoints over dlopen/dlsym runtime state.",
    "crates/gmodule/src/runtime.rs": "OS dynamic-loader boundary and C callback invocation for GModule.",
    "crates/gobject/build.rs": "Build-time safety gate for hand-written GObject modules.",
    "crates/gobject/src/object/mod.rs": "GObject and GParamSpec class vtable callback typedefs.",
    "crates/gobject/src/signal/mod.rs": "GObject closure callback typedefs.",
    "crates/gobject/src/translated": "Generated Rust translation of upstream GObject; retained only at the C ABI boundary.",
    "crates/gobject/src/type_system/mod.rs": "GType lifecycle and plugin callback typedefs.",
    "crates/gobject/src/value/mod.rs": "GValue transform callback typedef.",
    "crates/gthread/src/compat.rs": "Deprecated exported thread initialization ABI shims.",
    "crates/gthread/src/runtime.rs": "GLib warning callback boundary for deprecated GThread entrypoints.",
}

FORBIDDEN_BOOTSTRAP_PATTERNS = (
    "$SAFE_VENDOR_BUILD_CHECK",
    "vendor/build-check",
    "build-glib-backend.py",
    "/".join(("safe", "vendor", "build_check")),
    "safe_" + "vendor_build_check",
)


def audit_key(path: Path) -> str:
    relative = path.as_posix()
    for translated_root in (
        "crates/gio/src/translated/",
        "crates/glib/src/translated/",
        "crates/gobject/src/translated/",
    ):
        if relative.startswith(translated_root):
            return translated_root.rstrip("/")
    return relative


def unsafe_count(path: Path) -> int:
    return len(re.findall(r"\bunsafe\b", path.read_text(errors="ignore")))


def bootstrap_leftovers() -> list[str]:
    matches = []
    for path in Path("crates").rglob("*.rs"):
        text = path.read_text(errors="ignore")
        for pattern in FORBIDDEN_BOOTSTRAP_PATTERNS:
            if pattern in text:
                matches.append(f"{path}: contains {pattern}")
    return matches


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.parse_args()

    observed: dict[str, int] = {}
    for path in Path("crates").rglob("*.rs"):
        count = unsafe_count(path)
        if count == 0:
            continue
        key = audit_key(path)
        observed[key] = observed.get(key, 0) + count

    unexpected = sorted(set(observed) - set(EXPECTED_UNSAFE_COUNTS))
    missing = sorted(set(EXPECTED_UNSAFE_COUNTS) - set(observed))
    changed = sorted(
        key
        for key, expected in EXPECTED_UNSAFE_COUNTS.items()
        if key in observed and observed[key] != expected
    )
    undocumented = sorted(key for key in observed if key not in AUDIT_CLASSES)
    bootstrap_matches = bootstrap_leftovers()

    errors = []
    if unexpected:
        errors.append("unexpected unsafe-bearing files:\n" + "\n".join(unexpected))
    if missing:
        errors.append("expected unsafe-bearing files disappeared:\n" + "\n".join(missing))
    if changed:
        errors.append(
            "unsafe counts changed; update this audit with a justification:\n"
            + "\n".join(
                f"{key}: expected {EXPECTED_UNSAFE_COUNTS[key]}, observed {observed[key]}"
                for key in changed
            )
        )
    if undocumented:
        errors.append("unsafe-bearing files lack audit class text:\n" + "\n".join(undocumented))
    if bootstrap_matches:
        errors.append(
            "bootstrap build-check translated modules are still wired into crates:\n"
            + "\n".join(bootstrap_matches)
        )

    if errors:
        raise SystemExit("\n\n".join(errors))

    print(f"unsafe audit ok: {sum(observed.values())} unsafe tokens covered by {len(observed)} audit classes")


if __name__ == "__main__":
    main()
