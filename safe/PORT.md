# GLib Rust Port Report

This report documents the checked-out `port-glib` safe workspace for the
`impl-package-integration` phase. It covers the Rust implementation under
`safe/crates`, the C-compatible GLib-family ABI exported by the shared
libraries, and the Debian packaging and dependent-harness surfaces that make
the build a drop-in Ubuntu 24.04 replacement package.

## High-level architecture

The safe tree is a Cargo workspace with seven members:
`crates/abi-support`, `crates/glib`, `crates/gthread`, `crates/gmodule`,
`crates/gobject`, `crates/gio`, and `crates/girepository`
(`safe/Cargo.toml:1`). Workspace metadata sets Rust 2021,
`LGPL-2.1-or-later`, and version `0.1.0` (`safe/Cargo.toml:13`).

The public boundary is the GLib-family C ABI. Each library crate builds `rlib`
and `staticlib` artifacts rather than a Cargo `cdylib`
(`safe/crates/glib/Cargo.toml:8`, `safe/crates/gobject/Cargo.toml:8`,
`safe/crates/gio/Cargo.toml:8`, `safe/crates/gmodule/Cargo.toml:8`,
`safe/crates/gthread/Cargo.toml:8`, `safe/crates/girepository/Cargo.toml:8`).
`safe/tools/build-abi-shell.py` links those static archives into shared objects
with the upstream SONAMEs and version scripts: `libglib-2.0.so.0`,
`libgthread-2.0.so.0`, `libgmodule-2.0.so.0`, `libgobject-2.0.so.0`,
`libgio-2.0.so.0`, and `libgirepository-2.0.so.0`
(`safe/tools/build-abi-shell.py:22`). The version scripts currently define
1872 `libglib`, 2 `libgthread`, 10 `libgmodule`, 478 `libgobject`, 2107
`libgio`, and 231 `libgirepository` exported symbols
(`safe/abi/version-scripts/libglib.map:1`,
`safe/abi/version-scripts/libgthread.map:1`,
`safe/abi/version-scripts/libgmodule.map:1`,
`safe/abi/version-scripts/libgobject.map:1`,
`safe/abi/version-scripts/libgio.map:1`,
`safe/abi/version-scripts/libgirepository.map:1`).

`safe-glib` is the core utility library. Its module map exposes Rust-owned
wrappers for bytes, charset, file utilities, GVariant, hash tables, key files,
markup, spawn, threading, Unicode, URI, and other API groups, while still
including translated upstream modules under `translated`
(`safe/crates/glib/src/lib.rs:17`). `safe-gobject` exposes the type system,
object, signal, and value modules plus translated upstream GObject internals
(`safe/crates/gobject/src/lib.rs:17`). `safe-gio` layers action, application,
D-Bus, file, resource, settings, socket, stream, proxy, tool, and translated
GIO modules (`safe/crates/gio/src/lib.rs:17`). `safe-gmodule` and
`safe-gthread` are smaller ABI crates for module loading and deprecated thread
initialization shims (`safe/crates/gmodule/src/lib.rs:6`,
`safe/crates/gthread/src/lib.rs:6`). `safe-girepository` contains the Rust
GIRepository implementation plus the `gi-compile-repository`,
`gi-decompile-typelib`, and `gi-inspect-typelib` binaries
(`safe/crates/girepository/Cargo.toml:11`).

Data flows from C callers into exported C ABI functions, through Rust-owned
wrappers or translated upstream modules, and then to Rust state or OS/system
library calls. The GLib/GIO/GObject crates preserve C struct layouts in `abi`
modules (`safe/crates/glib/src/lib.rs:42`, `safe/crates/gobject/src/lib.rs:29`,
`safe/crates/gio/src/lib.rs:32`). `abi-support` imports those layouts for the
layout probe (`safe/crates/abi-support/Cargo.toml:8`). GIRepository keeps a
clear Rust-owned path: public `gi_*` exports in `exports.rs` forward raw C
pointers to repository state and parser code in `runtime.rs` and `parser.rs`
(`safe/crates/girepository/src/exports.rs:13`,
`safe/crates/girepository/src/runtime.rs:96`,
`safe/crates/girepository/src/parser/mod.rs:318`).

Build glue is split between Cargo build scripts and the ABI shell. The build
scripts retain the old `SAFE_LINK_SONAME` and `SAFE_LINK_VERSION_SCRIPT`
environment hooks but emit no direct `cdylib` linker arguments; they do emit
native link libraries such as `pcre2-8`, `glib-2.0`, `gobject-2.0`, `gio-2.0`,
`gmodule-2.0`, `ffi`, `dl`, `mount`, `selinux`, `z`, `pthread`, and `m`
(`safe/crates/glib/build.rs:147`, `safe/crates/gobject/build.rs:95`,
`safe/crates/gmodule/build.rs:19`, `safe/crates/gthread/build.rs:19`,
`safe/crates/girepository/build.rs:20`). `safe-glib` and `safe-gio` generate
assembly aliases from translated `safe_c2rust_*` symbols to the public names
not reimplemented by Rust-owned modules (`safe/crates/glib/build.rs:149`,
`safe/crates/gio/build.rs:161`, `safe/crates/glib/src/aliases.rs:1`,
`safe/crates/gio/src/exports.rs:4`). `safe-gobject` adds a build-time safety
gate that requires a nearby `SAFETY:` comment for non-translated unsafe
boundaries (`safe/crates/gobject/build.rs:28`).

Debian packaging is source-owned under `safe/debian`. `debian/rules` defaults
to `nodoc noinsttest nogir noudeb` unless `SAFE_FULL_PACKAGE_BUILD` is set,
builds the ABI shell with `tools/build-abi-shell.py`, stages package payloads
with `tools/stage-package-tree.py`, generates cross-prefixed `gi-*` wrappers,
and intentionally skips build-time tests in favor of package and autopkgtest
surfaces (`safe/debian/rules:3`, `safe/debian/rules:45`,
`safe/debian/rules:70`, `safe/debian/rules:90`). `stage-package-tree.py`
renders pkg-config files for `glib-2.0`, `gthread-2.0`,
`gmodule-2.0`, `gobject-2.0`, `gio-2.0`, `gio-unix-2.0`, and
`girepository-2.0`, copies helper tools, stages introspection data, and patches
runpaths (`safe/tools/stage-package-tree.py:68`,
`safe/tools/stage-package-tree.py:474`).

The Debian package layout is governed by install files, link files, and JSON
baselines. Runtime packages install the GLib/GObject/GModule/GThread/GIO and
GIRepository shared objects plus runtime helper tools
(`safe/debian/libglib2.0-0t64.install:1`,
`safe/debian/libgirepository-2.0-0.install:1`). Development packages install
headers, static archives, linker symlinks, pkg-config metadata, developer
tools, `gi-*` helpers, and cross wrappers
(`safe/debian/libglib2.0-dev.install:1`,
`safe/debian/libglib2.0-dev-bin.install:1`,
`safe/debian/libglib2.0-dev-bin.links:1`,
`safe/debian/libgirepository-2.0-dev.install:1`). GIR and typelib packages are
split into GLib-family and GIRepository packages
(`safe/debian/gir1.2-glib-2.0.install:1`,
`safe/debian/gir1.2-glib-2.0-dev.install:1`,
`safe/debian/gir1.2-girepository-3.0.install:1`,
`safe/debian/gir1.2-girepository-3.0-dev.install:1`). The install-manifest
baselines cover `runtime`, `abi-shell`, `dev-package`, `doc-package`, `udeb`,
and `full` package profiles (`safe/abi/install-manifests/runtime.json:1`).
The postinst baseline records that the runtime package declares triggers for
GSettings schema compilation and GIO module cache generation, but expects those
generated files to remain absent in the isolated runtime manifest test
(`safe/debian/libglib2.0-0t64.triggers.in:1`,
`safe/abi/postinst-state/runtime.json:1`).

Package-level tests and the dependent harness are now package-oriented.
`test-original.sh` defaults to `GLIB_UNDER_TEST=safe` and `GLIB_TEST_SCOPE=all`,
copies the read-only `safe/` tree into the container, installs build
dependencies from `safe/debian/control`, builds `.deb` packages with
`dpkg-buildpackage`, installs those packages, and records the SHA-256 of the
runtime library extracted from the freshly built `libglib2.0-0t64` package
(`test-original.sh:12`, `test-original.sh:232`, `test-original.sh:273`,
`test-original.sh:315`, `test-original.sh:344`, `test-original.sh:349`,
`test-original.sh:360`). In safe mode, package installation itself is validated
by `assert_installed_safe_libglib`, which checks that
`/usr/lib/$multiarch/libglib-2.0.so.0.8000.0` exists, is owned by
`libglib2.0-0t64`, and matches that extracted package hash
(`test-original.sh:130`, `test-original.sh:135`, `test-original.sh:138`,
`test-original.sh:364`). Runtime binary resolution is a separate dependent-probe
assertion: `assert_binary_uses_test_glib` checks `ldd` output, and in safe mode
requires the probed binary to resolve `libglib-2.0.so.0` to the installed
`/usr/lib/$multiarch/libglib-2.0.so.0.8000.0` package path and matching hash
(`test-original.sh:103`, `test-original.sh:108`, `test-original.sh:115`,
`test-original.sh:118`, `test-original.sh:120`).

The same harness still supports an original-prefix comparison path, but only for
the dependent scope. In `GLIB_UNDER_TEST=original` mode it builds the upstream
tree under `GLIB_PREFIX=/opt/glib-original`, then prepends that prefix to
`PATH`, `LD_LIBRARY_PATH`, `PKG_CONFIG_PATH`, and `ACLOCAL_PATH` before
installing dependent packages (`test-original.sh:28`, `test-original.sh:77`,
`test-original.sh:214`, `test-original.sh:799`, `test-original.sh:894`). The
original-mode branch of `assert_binary_uses_test_glib` therefore expects probed
runtime binaries to load `libglib-2.0.so.0` from the prefix library directory,
not from the installed safe Debian package path (`test-original.sh:110`,
`test-original.sh:112`). The `package-smoke` and `debian-tests` scopes are
safe-only and reject `GLIB_UNDER_TEST=original` (`test-original.sh:811`,
`test-original.sh:883`). The `package-smoke` scope runs `debian/tests/build`,
`debian/tests/build-static`, and the two GIRepository package scripts
(`test-original.sh:811`). The `debian-tests` scope runs the declared
autopkgtest scripts (`test-original.sh:863`, `safe/debian/tests/control:1`).
The `dependents` scope installs the runtime dependent packages from
`dependents.json` and runs one focused probe per dependent plus a source build
of `budgie-artwork` to cover the compile-time-only `pocillo-icon-theme` path
(`test-original.sh:142`, `test-original.sh:741`, `test-original.sh:894`).

Short directory map:

```text
safe/
  Cargo.toml                         workspace members and package defaults
  crates/abi-support/                shared primitive ABI aliases and layout probe
  crates/glib/                       GLib ABI, Rust-owned API groups, translated core modules
  crates/gthread/                    deprecated GThread compatibility exports
  crates/gmodule/                    GModule ABI and dynamic-loader runtime
  crates/gobject/                    GObject type, object, signal, and value ABI
  crates/gio/                        GIO ABI, helper binaries, translated GIO modules
  crates/girepository/               GIRepository ABI, parser/runtime, gi-* binaries
  abi/version-scripts/               exported symbol maps for every shared library
  abi/install-manifests/             Debian package content baselines by profile
  abi/postinst-state/                trigger-state expectations
  abi/debian-control-preservation.json Debian control-field preservation contract
  debian/                            package control, rules, install lists, tests, triggers
  tests/package/                     installed-package smoke tests
  tools/build-abi-shell.py           staticlib-to-shared-object linker/stager
  tools/stage-package-tree.py        Debian package tree renderer
  tools/check-unsafe-audit.py        unsafe inventory guard
```

## Where the unsafe Rust lives

Unsafe is still substantial because the port preserves a C ABI and contains
mechanically translated GLib/GObject/GIO modules. The current inventory guard
passes:

```text
cd safe && python3 tools/check-unsafe-audit.py
unsafe audit ok: 26244 unsafe tokens covered by 30 audit classes
```

`rg -n "\bunsafe\b" safe --glob '*.rs'` produced 26241 line matches in this
documentation pass; the three-token difference is from lines that contain more
than one `unsafe` token. The authoritative class list and counts are in
`safe/tools/check-unsafe-audit.py:5`, and each class has a purpose string in
`safe/tools/check-unsafe-audit.py:38`.

Complete audited unsafe classes:

| Site | Count | Forms | Why it exists |
| --- | ---: | --- | --- |
| `safe/crates/abi-support/src/ffi.rs:34,36` | 2 | unsafe extern fn pointer aliases | C callback typedefs shared by exported ABI structs. |
| `safe/crates/gio/build.rs:61,62` | 2 | string patterns only | Build script scans translated exports by text; these are not executable unsafe code. |
| `safe/crates/gio/src/generated_compat.rs:23` | 74 | unsafe extern blocks, unsafe exports, unsafe blocks | Rust-owned GIO compatibility exports for enum, portal, D-Bus helper, allocation, and out-parameter ABI symbols. |
| `safe/crates/gio/src/lib.rs:35,36` | 2 | unsafe extern fn pointer aliases | GIO public ABI structs store caller-supplied callbacks. |
| `safe/crates/gio/src/translated/**` | 19182 | unsafe extern blocks, unsafe fns, unsafe blocks, statics | Generated Rust translation of upstream GIO C plus file-monitor/settings/GVDB helpers; all raw pointers model GLib/GIO ABI and translated C ownership. |
| `safe/crates/girepository/src/exports.rs:14` | 102 | macro-generated unsafe exports and unsafe blocks | Rust-owned GIRepository public `gi_*` ABI shims receive raw pointers and out-parameters. |
| `safe/crates/girepository/src/runtime.rs:42` | 136 | unsafe extern block, unsafe impls, unsafe fns, unsafe blocks | GIRepository runtime bridges GObject type registration, C strings, GLib allocation, GError, and stack-loaded GI structs. |
| `safe/crates/glib/build.rs:62,63` | 2 | string patterns only | Build script scans translated exports by text; these are not executable unsafe code. |
| `safe/crates/glib/src/bytes/api.rs:7` | 3 | unsafe extern block and unsafe wrapper | Rust-owned byte-array ABI wrapper delegates to translated GLib storage. |
| `safe/crates/glib/src/charset/api.rs:10` | 20 | unsafe extern block and unsafe exports | Charset ABI wrappers consume C strings and query process environment state. |
| `safe/crates/glib/src/data.rs:6` | 17 | unsafe attributes/statics | Exported global data symbols such as GLib compatibility variables. |
| `safe/crates/glib/src/fileutils/api.rs:8` | 3 | unsafe extern block and unsafe wrapper | Filesystem ABI wrapper delegates to translated canonicalization. |
| `safe/crates/glib/src/gvariant/api.rs:10` | 13 | unsafe extern block and unsafe wrappers | GVariant validation ABI consumes serialized C pointers. |
| `safe/crates/glib/src/hash/api.rs:11` | 132 | unsafe extern blocks, unsafe callbacks, unsafe methods | Rust-owned GHashTable keeps GLib raw-pointer ownership and hash/equality/destroy callback ABI. |
| `safe/crates/glib/src/keyfile/api.rs:10` | 3 | unsafe extern block and unsafe wrapper | Key-file ABI wrapper exposes translated parser state. |
| `safe/crates/glib/src/legacy.rs:18` | 30 | unsafe exported dispatch wrappers | Internal compatibility layer for exported GLib symbols still served by translated code. |
| `safe/crates/glib/src/markup/api.rs:15` | 14 | unsafe extern block and unsafe wrappers | Markup parser ABI handles raw parser contexts and `GError` out-parameters. |
| `safe/crates/glib/src/spawn/api.rs:13` | 20 | unsafe extern block and unsafe wrappers | Spawn ABI consumes C `argv`/`envp`, working-directory strings, and pipe out-parameters. |
| `safe/crates/glib/src/support.rs:3` | 17 | unsafe attributes, extern statics, unsafe exports | Process-global C runtime support symbols required by translated GLib modules. |
| `safe/crates/glib/src/translated/**` | 3851 | unsafe extern blocks, unsafe fns, unsafe blocks, statics | Generated Rust translation of upstream GLib C; retained for ABI coverage not yet rewritten into Rust-owned modules. |
| `safe/crates/gmodule/src/module_api.rs:19` | 20 | unsafe exported C ABI fns | GModule public ABI receives raw module handles, strings, and out-parameters. |
| `safe/crates/gmodule/src/runtime.rs:22` | 13 | unsafe callback alias, unsafe extern block, unsafe fns, transmute | Dynamic-loader runtime calls `dlopen`/`dlsym`, invokes optional module callbacks, and writes GLib errors. |
| `safe/crates/gobject/build.rs:9` | 7 | string patterns only | Build-time safety gate searches for unsafe boundaries lacking `SAFETY:` comments. |
| `safe/crates/gobject/src/object/mod.rs:37` | 14 | unsafe extern fn pointer aliases | GObject and GParamSpec class vtables store C callbacks. |
| `safe/crates/gobject/src/signal/mod.rs:5,7` | 2 | unsafe extern fn pointer aliases | GObject closures store C marshal/notify callbacks. |
| `safe/crates/gobject/src/translated/**` | 2539 | unsafe extern blocks, unsafe fns, unsafe blocks, statics | Generated Rust translation of upstream GObject; retained for C ABI behavior. |
| `safe/crates/gobject/src/type_system/mod.rs:4` | 17 | unsafe extern fn pointer aliases | GType lifecycle and plugin callbacks are C ABI callbacks. |
| `safe/crates/gobject/src/value/mod.rs:44` | 1 | unsafe extern fn pointer alias | `GValue` transform callback typedef. |
| `safe/crates/gthread/src/compat.rs:3,13` | 4 | unsafe exported C ABI fns | Deprecated thread-initialization exports keep ABI compatibility and warn. |
| `safe/crates/gthread/src/runtime.rs:5,14` | 2 | unsafe extern block and unsafe helper | Calls GLib `g_log` for deprecated GThread warnings. |

The translated directories are the only places where the document does not list
every individual line inline; they contain 362 Rust files and 25569 unsafe-line
matches. Their file-level inventory is still mechanically reproducible with:

```bash
cd safe
rg -n "\bunsafe\b" crates/gio/src/translated crates/glib/src/translated crates/gobject/src/translated --glob '*.rs'
```

Unsafe that is not strictly required by the public C ABI/API boundary:
`safe/crates/girepository/src/runtime.rs:123`,
`safe/crates/girepository/src/runtime.rs:124`, and
`safe/crates/girepository/src/runtime.rs:369` are internal `unsafe impl`
assertions for process-global or mutex-protected state. The build-script unsafe
matches in `safe/crates/gio/build.rs`, `safe/crates/glib/build.rs`, and
`safe/crates/gobject/build.rs` are text patterns used by audit/generation
logic, not unsafe Rust execution. `safe/crates/girepository/src/runtime.rs:1312`
is a C ABI-shaped local reference-function placeholder returned through
metadata, not an exported symbol.

GIRepository-specific unsafe remains as documented in the previous phase:
`exports.rs` defines macro-generated `unsafe extern "C"` entrypoints at
`safe/crates/girepository/src/exports.rs:14`,
`safe/crates/girepository/src/exports.rs:24`,
`safe/crates/girepository/src/exports.rs:34`, and
`safe/crates/girepository/src/exports.rs:45`; direct repository/path/base-info
exports appear at `safe/crates/girepository/src/exports.rs:53`,
`safe/crates/girepository/src/exports.rs:142`, and
`safe/crates/girepository/src/exports.rs:159`. Runtime unsafe functions cover
GObject allocation, repository search paths, namespace loading, GI info handle
lifetime, stack loading, metadata accessors, string-vector allocation, and
`GError` handling (`safe/crates/girepository/src/runtime.rs:626`,
`safe/crates/girepository/src/runtime.rs:644`,
`safe/crates/girepository/src/runtime.rs:692`,
`safe/crates/girepository/src/runtime.rs:891`,
`safe/crates/girepository/src/runtime.rs:992`,
`safe/crates/girepository/src/runtime.rs:1136`,
`safe/crates/girepository/src/runtime.rs:1609`).

## Remaining unsafe FFI beyond the original ABI/API boundary

The intended public boundary is the GLib-family C ABI exported from the six
shared libraries and described by `safe/abi/version-scripts/*.map`. Additional
unsafe FFI beyond that public ABI is needed for OS integration, dynamic loading,
compression, regex support, and links to the GLib-family libraries while the
crates are built as separate static archives.

| Surface | Symbols/providers | Evidence | Why needed | Possible safe replacement |
| --- | --- | --- | --- | --- |
| POSIX/libc filesystem, process, environment, polling, sockets, and low-level runtime calls | `stat`, `open`, `getenv`, `poll`, `socket`, `sendmsg`, `statx`, spawn/process APIs and many libc-style functions from translated modules | Examples: `safe/crates/glib/src/charset/api.rs:15`, `safe/crates/glib/src/translated/gfileutils.rs:5`, `safe/crates/glib/src/translated/gfileutils.rs:57`, `safe/crates/glib/src/translated/gpoll.rs:2`, `safe/crates/glib/src/translated/gmessages.rs:80`, `safe/crates/gio/src/translated/original/gio/gdbusauthmechanismsha1.rs:797` | GLib/GIO are OS abstraction libraries; their public APIs require Unix filesystem, process, socket, and environment behavior. | Individual modules can be rewritten to safe `std`, `nix`, or `rustix` wrappers, but the C ABI still requires raw file descriptors and pointer-shaped data. |
| GLib/GObject cross-library calls from Rust-owned crates | `g_log`, allocator/string functions, GType/GObject functions, `GError` and `GQuark` helpers | `safe/crates/gthread/src/runtime.rs:5`, `safe/crates/gmodule/src/runtime.rs:49`, `safe/crates/girepository/src/runtime.rs:42`, `safe/crates/gio/src/generated_compat.rs:27` | Staticlib crates are linked into separate shared objects and still need compatible GLib allocation, logging, error, and type-system behavior across library boundaries. | Central safe wrappers in `abi-support` could reduce repeated unsafe blocks, but inter-library ABI calls remain necessary. |
| Dynamic loading | `dlopen`, `dlclose`, `dlsym`, `dlerror` from libdl; optional `g_module_check_init` and `g_module_unload` callbacks from loaded modules | `safe/crates/gmodule/src/runtime.rs:43`, `safe/crates/gmodule/src/runtime.rs:149`, `safe/crates/gmodule/src/runtime.rs:198`, `safe/crates/gmodule/src/runtime.rs:215`, `safe/crates/gmodule/src/runtime.rs:236` | GModule is a dynamic-module API; it must load arbitrary shared objects and call their C ABI entrypoints. | A wrapper crate such as `libloading` could hide some `dl*` calls, but symbol loading and callback invocation would remain unsafe. |
| Linux file monitoring | `inotify_init`, `inotify_init1`, `inotify_add_watch`, `inotify_rm_watch` | `safe/crates/gio/src/translated/original/gio/inotify/inotify_kernel.rs:69` | GIO file-monitor APIs need Linux inotify behavior. | Could be rewritten through a safe notify/inotify crate, with raw file-descriptor exposure audited at the boundary. |
| zlib compression | `deflate*` and `inflate*` from zlib | `safe/crates/gio/src/translated/original/gio/gzlibcompressor.rs:99`, `safe/crates/gio/src/translated/original/gio/gzlibdecompressor.rs:99` | GIO exposes zlib compressor/decompressor types and keeps Ubuntu GLib link behavior. | A pure Rust compressor could replace the internals only if ABI-compatible zlib error/header semantics are preserved. |
| PCRE2 regex | `libpcre2-8` link dependency | `safe/crates/glib/build.rs:147`, `safe/tools/build-abi-shell.py:33`, `safe/tools/stage-package-tree.py:88` | GLib regex APIs are PCRE2-compatible and the Debian development metadata exposes `libpcre2-8` as a private dependency. | A Rust regex engine would not be a drop-in semantic replacement for PCRE2. |
| libffi, libmount, libselinux, zlib, quadmath, pthread, libm, libdl link surface | Native link libraries on ABI-shell shared objects and pkg-config metadata | `safe/tools/build-abi-shell.py:33`, `safe/tools/build-abi-shell.py:45`, `safe/tools/build-abi-shell.py:57`, `safe/tools/build-abi-shell.py:69`, `safe/tools/build-abi-shell.py:81`, `safe/tools/build-abi-shell.py:93`, `safe/tools/stage-package-tree.py:88`, `safe/tools/stage-package-tree.py:174`, `safe/tools/stage-package-tree.py:204`, `safe/tools/stage-package-tree.py:235` | Mirrors Ubuntu GLib's runtime/development link contract and supports FFI closure, mount, SELinux, compression, floating-point, thread, and math behavior. | Some unused links might be pruned after ABI/link-compat review, but package metadata and dependent builds would need to change together. |
| Baseline verifier interposition | `dlsym(RTLD_NEXT, ...)` in generated C snippets inside a Python verifier | `safe/tools/verify-package-baselines.py:115` | The package baseline verifier intercepts filesystem calls while checking package content. | This is test tooling, not shipped runtime; no runtime replacement is needed. |

The grep evidence for raw FFI is intentionally large:
`rg -n '^\s*(pub\s+)?(unsafe\s+)?extern "C"\s*\{' safe/crates --glob '*.rs'`
finds 358 extern blocks, and
`rg -n '^\s*fn [A-Za-z_].*;' safe/crates/glib/src/translated safe/crates/gio/src/translated safe/crates/gobject/src/translated --glob '*.rs'`
finds 10555 translated foreign-function declaration lines. Those translated
FFI declarations are current implementation debt, not new public API surface.

## Remaining issues

Unsafe code remains the largest technical limitation. The current source has
26244 unsafe tokens across 30 audited classes (`safe/tools/check-unsafe-audit.py:5`).
Most are in translated GLib/GObject/GIO modules under
`safe/crates/glib/src/translated`, `safe/crates/gobject/src/translated`, and
`safe/crates/gio/src/translated`. The port is packaged as a drop-in ABI
replacement, but it is not yet a fully safe-Rust implementation internally.

Build-time upstream tests are skipped by Debian rules for this ABI-shell
package path (`safe/debian/rules:90`). The package test surface is instead the
autopkgtest and harness surface under `safe/debian/tests`,
`safe/tests/package`, and `test-original.sh`. The full package verifier
commands from `.plan/phases/07-package-integration.md` were not rerun during
this documentation pass because a separate long-running package/build shell was
already active in the checkout; the residual risk is that this report records
the configured surfaces and lightweight checks, not a fresh end-to-end
`dpkg-buildpackage` plus container test result.

The staged installed-test payload is intentionally shallow. When the build
profile includes installed tests, `stage_autopkgtest_installed_tests` writes
TAP scripts that emit a single passing test for each named installed-test case
(`safe/tools/stage-package-tree.py:365`, `safe/tools/stage-package-tree.py:413`).
The Debian scripts still exercise the installed-test runner and wrappers
(`safe/debian/tests/installed-tests:17`,
`safe/debian/tests/closure-refcount:13`), but they do not execute the original
upstream installed-test binaries.

GIRepository link ABI is broad, but several exports still have placeholder
semantics. `abi_zero_arg_symbols!` defines many zero-return compatibility
symbols, including repository loading/query, typelib, closure/invoke, field,
property, interface, object, struct, type, and vfunc helpers
(`safe/crates/girepository/src/exports.rs:403`). `function_invoke` ignores the
argument arrays, only special-cases `g_file_read_link` to set a
`G_FILE_ERROR_NOENT`, and otherwise returns failure
(`safe/crates/girepository/src/runtime.rs:1147`). The parser reads GIR XML and
selected binary typelib metadata, and emits a safe custom typelib wrapper
around embedded GIR text rather than an upstream-identical typelib writer
(`safe/crates/girepository/src/parser/mod.rs:385`,
`safe/crates/girepository/src/parser/mod.rs:880`,
`safe/crates/girepository/src/parser/mod.rs:990`).

Some runtime/default paths remain multiarch-specific. GIRepository repository
lookup still includes x86_64 paths in Rust runtime/tool defaults
(`safe/crates/girepository/src/runtime.rs:24`,
`safe/crates/girepository/src/tools/mod.rs:68`,
`safe/crates/girepository/src/tools/mod.rs:83`). Debian packaging passes the
host multiarch to build and staging (`safe/debian/rules:56`,
`safe/tools/stage-package-tree.py:219`), so package layout is multiarch-aware,
but non-x86_64 runtime defaults should be rechecked before claiming full
cross-architecture equivalence.

Package smoke coverage is focused, not exhaustive. `debian/tests/build`
compiles and runs small dynamic/static consumers for `glib-2.0`, `gobject-2.0`,
`gio-2.0`, `gio-unix-2.0`, `gmodule-2.0`, and `gthread-2.0`, skipping static
GIO/GIO-Unix because libmount does not support static linking
(`safe/debian/tests/build:117`). The GIRepository smoke scripts verify
`pkg-config`, the installed header, `gi_repository_new`, `/usr/bin` tool links,
GIR/typelib locations, `gi-compile-repository`, `gi-inspect-typelib`, and
`gi-decompile-typelib` (`safe/tests/package/girepository-compile-only.sh:8`,
`safe/tests/package/girepository-installed.sh:17`). They do not cover every
exported GLib/GIO/GObject/GIRepository symbol.

Dependent coverage is representative and harness-driven. `dependents.json`
tracks 12 packages: 11 compile-time-and-runtime dependents
(`qemu-system-x86`, `network-manager`, `bluez`, `flatpak`, `modemmanager`,
`fwupd`, `gvfs-daemons`, `gstreamer1.0-tools`, `libvirt-daemon`, `udisks2`,
`tracker-miner-fs`) plus the compile-time-only `pocillo-icon-theme` coverage
through a `budgie-artwork` source build (`dependents.json:1`,
`test-original.sh:149`, `test-original.sh:171`). The probes verify library
resolution and basic daemon/tool behavior such as QEMU QMP startup,
NetworkManager/nmcli, bluetoothd startup, Flatpak local repository operations,
ModemManager/mmcli, fwupd remotes, GVfs D-Bus, a GStreamer fakesrc pipeline,
libvirtd/virsh, udisksctl, Tracker miner D-Bus, and GLib build tools used by
budgie-artwork (`test-original.sh:399`, `test-original.sh:434`,
`test-original.sh:464`, `test-original.sh:478`, `test-original.sh:499`,
`test-original.sh:527`, `test-original.sh:556`, `test-original.sh:579`,
`test-original.sh:584`, `test-original.sh:617`, `test-original.sh:646`,
`test-original.sh:674`). These are not full upstream test suites for the
dependent packages.

Bootstrap/oracle assets still exist for ABI and test tooling. The package
stager now stages payloads from the ABI-shell build root and original metadata,
but `safe/vendor/build-check` remains present and is still referenced by ABI
extraction/link-compat tools and by the container harness source-copy assertion
(`safe/tools/common.py:16`, `safe/tools/build-abi-shell.py:96`,
`safe/tools/link-compat.py:145`, `test-original.sh:268`). That is acceptable as
test/build evidence, but it should not be confused with a shipped runtime
payload.

`relevant_cves.json` lists 15 non-memory-corruption CVE classes across
filesystem permissions, algorithmic complexity, parser state handling,
network-policy bypass, environment/privilege boundaries, numeric API
semantics, symlink semantics, deserialization validation, origin
authentication, and platform-specific DoS (`relevant_cves.json:1`). The port
does not yet claim full mitigation for those classes because many relevant
surfaces remain translated or compatibility-focused: file copy/replace paths,
hashing, GVariant, GDBus, charset/environment, and spawn behavior still need
semantic verification against those CVEs.

## Dependencies and other libraries used

Direct Cargo dependencies from `safe/Cargo.toml` and member manifests, with
resolved versions from `cargo tree -e normal,build,dev`:

| Crate | Direct dependency | Resolved version | Purpose |
| --- | --- | --- | --- |
| workspace | none | n/a | Workspace membership and package metadata only (`safe/Cargo.toml:1`). |
| `abi-support` | `safe-gio`, `safe-girepository`, `safe-glib`, `safe-gmodule`, `safe-gobject`, `safe-gthread` path deps | `0.1.0` each | Imports each member's public ABI layout types for layout probing (`safe/crates/abi-support/Cargo.toml:8`). |
| `safe-glib` | `c2rust-asm-casts` | `0.20.0` | C2Rust support for translated cast/asm patterns (`safe/crates/glib/Cargo.toml:12`). |
| `safe-glib` | `c2rust-bitfields` | `0.20.0` | C bitfield layout support in translated GLib structs (`safe/crates/glib/Cargo.toml:13`). |
| `safe-glib` | `f128` | `0.2.9` | Long-double/quad precision compatibility for GLib translated code (`safe/crates/glib/Cargo.toml:14`). |
| `safe-glib` | `libc` | `0.2.186` | libc type and symbol bindings for translated GLib and ABI wrappers (`safe/crates/glib/Cargo.toml:15`). |
| `safe-glib` | `num-traits` | `0.2.19` | Numeric helpers used by translated/compatibility code (`safe/crates/glib/Cargo.toml:16`). |
| `safe-glib` build | `serde` with `derive` | `1.0.228` | Build-script parsing/serialization support (`safe/crates/glib/Cargo.toml:19`). |
| `safe-glib` build | `serde_json` | `1.0.149` | Build-script JSON parsing (`safe/crates/glib/Cargo.toml:20`). |
| `safe-glib` build | `shlex` | `1.3.0` | Build-script shell-token parsing (`safe/crates/glib/Cargo.toml:21`). |
| `safe-gobject` | `c2rust-asm-casts` | `0.20.0` | C2Rust support for translated GObject patterns (`safe/crates/gobject/Cargo.toml:12`). |
| `safe-gobject` | `c2rust-bitfields` | `0.20.0` | C bitfield layout support in translated GObject structs (`safe/crates/gobject/Cargo.toml:13`). |
| `safe-gio` | `c2rust-asm-casts` | `0.22.1` | C2Rust support for translated GIO patterns (`safe/crates/gio/Cargo.toml:12`). |
| `safe-gio` | `c2rust-bitfields` | `0.22.1` | C bitfield layout support in translated GIO structs (`safe/crates/gio/Cargo.toml:13`). |
| `safe-gio` | `libc` | `0.2.186` | libc type and symbol bindings for translated GIO (`safe/crates/gio/Cargo.toml:14`). |
| `safe-girepository` | none | n/a | Uses `std`, local ABI aliases via `#[path]`, and explicit GLib/GObject FFI (`safe/crates/girepository/Cargo.toml:1`). |
| `safe-gmodule` | none | n/a | Uses `std` plus direct dynamic-loader/GLib FFI (`safe/crates/gmodule/Cargo.toml:1`). |
| `safe-gthread` | none | n/a | Uses `std` plus direct GLib logging FFI (`safe/crates/gthread/Cargo.toml:1`). |

Transitive Rust dependencies visible in `cargo tree` include
`c2rust-bitfields-derive`, `proc-macro2`, `quote`, `syn`, `unicode-ident`,
`f128_input`, `f128_internal`, `cc`, `find-msvc-tools`, `autocfg`, `serde_core`,
`serde_derive`, `itoa`, `memchr`, and `zmij`. The unsafe-heavy direct
dependencies are the C2Rust support crates and `libc`; they are acceptable for
this phase because translated C ABI modules still require C layout and libc
interoperation. No workspace crate currently uses `#![forbid(unsafe_code)]`.
`cargo geiger` was not installed in this environment, so dependency unsafe
depth was checked by manifest/tree review and the local unsafe audit rather
than by Geiger.

Debian build dependencies are declared in `safe/debian/control`. The source
package needs `cargo`, `rustc`, `debhelper-compat (= 13)`,
`dh-sequence-gnome`, `dh-sequence-python3`, `dpkg-dev`, `meson`, `pkgconf`,
`patchelf`, Python, `gettext`, XML/doc tools, and development libraries for
DBus, ELF, libffi, libmount, PCRE2, SELinux, Linux headers, and zlib
(`safe/debian/control:7`). Architecture-specific build/test dependencies add
desktop-file-utils, `dh-sequence-gir`, `gobject-introspection`, locales,
Python D-Bus/GI, qemu-user for cross builds, shared-mime-info, tzdata, and
xterm (`safe/debian/control:33`). Documentation builds add `gi-docgen` and
gobject-introspection (`safe/debian/control:43`).

Runtime and development package dependencies are preserved from Ubuntu GLib's
control structure. `libglib2.0-dev` depends on runtime and tool packages plus
`libffi-dev`, `libmount-dev`, `libpcre2-dev`, `libselinux1-dev`, `pkgconf`,
Python/qemu alternatives, and `zlib1g-dev` (`safe/debian/control:138`).
`libgirepository-2.0-dev` depends on `libglib2.0-dev`
(`safe/debian/control:232`). `libglib2.0-tests` depends on D-Bus, installed
test, Python, and shared-mime support (`safe/debian/control:82`).

Native/system libraries linked or published through pkg-config:

| Source | Libraries/tools | Purpose |
| --- | --- | --- |
| `safe/crates/glib/build.rs:147` | `pcre2-8` | GLib regex compatibility. |
| `safe/crates/gobject/build.rs:95` | `dl`, `ffi`, `m`, `pthread`, `glib-2.0`, `gthread-2.0`, `gmodule-2.0` | GObject FFI, math/thread support, and links to GLib-family libraries. |
| `safe/crates/gmodule/build.rs:19` | `glib-2.0`, `dl` | GModule logging/error and dynamic loading. |
| `safe/crates/gthread/build.rs:19` | `glib-2.0` | Deprecated GThread warning/logging. |
| `safe/crates/girepository/build.rs:20` | `glib-2.0`, `gobject-2.0`, `gio-2.0`, `gmodule-2.0`, `ffi`, `dl`, `mount`, `selinux`, `z`, `m` | GIRepository type, allocation, metadata, and link compatibility. |
| `safe/tools/build-abi-shell.py:22` | `pcre2-8`, `ffi`, `z`, `mount`, `selinux`, `quadmath`, `dl`, `pthread`, `m` plus internal GLib-family deps | Final shared-object linking for ABI shell libraries. |
| `safe/tools/stage-package-tree.py:68` | pkg-config metadata for GLib-family libraries | Development link contract consumed by downstream builds. |
| `safe/tools/build-abi-shell.py:640`, `safe/tools/link-compat.py:366`, `safe/tools/common.py:86`, `safe/tools/run-cve-regressions.py:183` | `pkg-config` command | Build/link/test discovery of compiler and linker flags. |

No `cbindgen` or `bindgen` invocation is used by the current safe package
build; headers are staged from vendored original GLib assets via the installed
file manifest and `stage-package-tree.py` (`safe/tools/stage-package-tree.py:41`).

## How this document was produced

Files consulted: `safe/PORT.md`, `.plan/phases/07-package-integration.md`,
`safe/Cargo.toml`, all member `Cargo.toml` files, member `build.rs` files,
`safe/crates/*/src/lib.rs`, `safe/crates/girepository/src/exports.rs`,
`safe/crates/girepository/src/runtime.rs`,
`safe/crates/girepository/src/parser/mod.rs`,
`safe/crates/gmodule/src/module_api.rs`,
`safe/crates/gmodule/src/runtime.rs`,
`safe/crates/gthread/src/compat.rs`,
`safe/crates/gthread/src/runtime.rs`,
`safe/tools/build-abi-shell.py`, `safe/tools/stage-package-tree.py`,
`safe/tools/check-unsafe-audit.py`,
`safe/tools/compare-debian-control.py`,
`safe/tools/verify-package-baselines.py`,
`safe/tools/compare-installed-files.py`,
`safe/tools/check-postinst-state.py`,
`safe/abi/version-scripts/*.map`, `safe/abi/install-manifests/*.json`,
`safe/abi/installed-files.json`, `safe/abi/postinst-state/runtime.json`,
`safe/abi/debian-control-preservation.json`, `safe/debian/control`,
`safe/debian/rules`, `safe/debian/*.install`, `safe/debian/*.links`,
`safe/debian/*.triggers.in`, `safe/debian/tests/control`,
`safe/debian/tests/*`, `safe/tests/package/*`, `dependents.json`,
`relevant_cves.json`, and `test-original.sh`.

Commands run:

```bash
cd safe && cargo metadata --format-version=1 > /tmp/port-glib-cargo-metadata.json
cd safe && cargo metadata --format-version=1 --no-deps >/tmp/port-glib-cargo-metadata-nodeps.json
cd safe && cargo tree -e normal,build,dev > /tmp/port-glib-cargo-tree.txt
cd safe && cargo tree -e normal,build,dev >/tmp/port-glib-cargo-tree-fresh.txt
cd safe && rg -n "Build-Depends|Depends|pkg-config|cbindgen|bindgen|cc::|println!\(\"cargo:rustc-link|cargo:rustc-link-lib|shlibdeps|dh_|autopkgtest" Cargo.toml debian crates tools meson.build || true
python3 -m json.tool dependents.json >/tmp/port-glib-dependents.json
cd safe && ls debian/tests tests/package abi/install-manifests abi/postinst-state
cd safe && python3 tools/check-unsafe-audit.py
cd safe && rg -n "\bunsafe\b" . --glob '*.rs' > /tmp/port-glib-unsafe-rg.txt
cd safe && rg -n '^\s*(pub\s+)?(unsafe\s+)?extern "C"\s*\{' crates --glob '*.rs' > /tmp/port-glib-extern-blocks.txt
cd safe && rg -n '^\s*fn [A-Za-z_].*;' crates/glib/src/translated crates/gio/src/translated crates/gobject/src/translated --glob '*.rs'
cd safe && python3 tools/compare-debian-control.py --baseline abi/debian-control-preservation.json --control debian/control
cd safe && python3 tools/check-postinst-state.py --help
cd safe && python3 tools/compare-installed-files.py --help
cd safe && python3 tools/verify-package-baselines.py --help
cd safe && cargo geiger -p safe-girepository --all-targets
```

`tools/check-unsafe-audit.py`, `cargo metadata`, `cargo tree`, JSON parsing of
`dependents.json`, package/test directory listing, and
`tools/compare-debian-control.py` passed. `cargo geiger` was unavailable
(`cargo` reported `no such command: geiger`). The heavyweight verifier commands
from `.plan/phases/07-package-integration.md` (`dpkg-buildpackage`,
`tools/verify-package-baselines.py`, `GLIB_TEST_SCOPE=package-smoke`,
`GLIB_TEST_SCOPE=debian-tests`, and `GLIB_TEST_SCOPE=dependents`) were not run
in this documentation pass because another long-running package/build command
was already active in the checkout.
