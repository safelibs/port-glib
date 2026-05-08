# GLib Rust Port Report

This report documents the currently checked-out `port-glib` safe workspace. It
is intentionally a living port document: subsequent phases must update this
existing file in place, preserve still-accurate sections, and revise only the
claims that drift as code, manifests, tests, or packaging change.

## High-level architecture

The safe port is a Rust workspace under `safe/` that builds Ubuntu 24.04 GLib
replacement shared libraries and helper binaries while preserving the upstream
GLib C ABI. The root workspace is declared in `safe/Cargo.toml:1`; it contains
`abi-support`, `safe-glib`, `safe-gthread`, `safe-gmodule`, `safe-gobject`,
`safe-gio`, and `safe-girepository` (`safe/Cargo.toml:2-11`). All workspace
crates use Rust 2021, LGPL-2.1-or-later, and workspace version `0.1.0`
(`safe/Cargo.toml:13-16`).

The public boundary is still the GLib/GThread/GModule/GObject/GIO/GIRepository
C ABI. Rust-owned entrypoints are exported as `pub unsafe extern "C" fn` with
either `#[unsafe(export_name = "...")]` in crates that use unsafe attributes,
such as `g_hash_table_new()` in `safe/crates/glib/src/hash/api.rs:551-552` and
`g_module_open()` in `safe/crates/gmodule/src/module_api.rs:24-25`, or plain
`#[export_name = "..."]` in GIRepository exports such as `gi_repository_new()`
at `safe/crates/girepository/src/exports.rs:52-53`. Generated translated
modules keep their C2Rust symbols under `safe_c2rust_*`; the GLib and GIO build
scripts scan the translated source, compare it with Rust-owned exports, and emit
`core::arch::global_asm!` alias stubs for remaining C ABI symbols
(`safe/crates/glib/build.rs:41-51`, `safe/crates/glib/build.rs:78-123`,
`safe/crates/gio/build.rs:40-50`, `safe/crates/gio/build.rs:79-124`). Version scripts under
`safe/abi/version-scripts/` constrain the exported symbol sets, and
`safe/tools/build-abi-shell.py` binds each Cargo static library to the upstream
SONAME and realname (`safe/tools/build-abi-shell.py:22-95`).

Cargo emits `rlib` and `staticlib` artifacts, not `cdylib`, for the six shipped
library crates (`safe/crates/glib/Cargo.toml:8-9`,
`safe/crates/gthread/Cargo.toml:8-9`, `safe/crates/gmodule/Cargo.toml:8-9`,
`safe/crates/gobject/Cargo.toml:8-9`, `safe/crates/gio/Cargo.toml:8-9`,
`safe/crates/girepository/Cargo.toml:8-9`). Shared objects are linked by
`safe/tools/build-abi-shell.py`, which supplies the SONAMEs, realnames, version
scripts, dependency ordering, native link libraries, and package helper tools
(`safe/tools/build-abi-shell.py:22-143`). I found no `cbindgen` or `bindgen`
usage in `safe/Cargo.toml`, `safe/Cargo.lock`, `safe/crates/`, `safe/tools/`,
`safe/debian/`, or `safe/meson.build`.

The primary data flow is:

1. A C caller links against the upstream-compatible symbol name from
   `libglib-2.0.so.0`, `libgobject-2.0.so.0`, `libgio-2.0.so.0`, and related
   libraries.
2. The symbol lands either in a Rust-owned ABI shim or in a generated alias to a
   C2Rust-translated `safe_c2rust_*` function.
3. The shim converts only enough of the C ABI state to operate: raw pointers,
   `GError **` out parameters, GLib callback function pointers, exported global
   variables, and C string vectors remain explicit ABI concerns.
4. The implementation either runs safe Rust-owned logic, dispatches through the
   Rust-owned compatibility layer, or calls translated modules that still mirror
   upstream C structure and unsafe pointer handling.
5. Packaging stages the resulting libraries, headers, pkg-config files, helper
   programs, Debian scripts, and preserved upstream assets into Ubuntu binary
   packages.

Crate layout:

| Crate | Role |
| --- | --- |
| `safe/crates/abi-support` | Shared C ABI aliases and structs, plus the `layout-probe` binary used by layout extraction (`safe/crates/abi-support/Cargo.toml:19-21`). |
| `safe/crates/glib` | Core GLib ABI plus Rust-owned modules for bytes, charset, file utilities, GVariant, hash tables, key files, markup, spawn, and compatibility dispatch; translated GLib modules remain under `safe/crates/glib/src/translated/`. |
| `safe/crates/gthread` | Deprecated GThread initialization shims and warning runtime. |
| `safe/crates/gmodule` | Rust-owned GModule ABI over the OS dynamic loader. |
| `safe/crates/gobject` | GObject ABI type definitions, callback/vtable types, partial Rust-owned modules, and translated upstream GObject modules. |
| `safe/crates/gio` | GIO ABI, translated upstream GIO modules, Rust-owned generated compatibility shims, and helper binaries `gapplication`, `gdbus`, `gdbus-codegen`, `gio`, `gio-launch-desktop`, `gio-querymodules`, `glib-compile-resources`, `glib-compile-schemas`, `gresource`, and `gsettings` (`safe/crates/gio/Cargo.toml:16-53`). |
| `safe/crates/girepository` | GIRepository parser/runtime/export layer plus `gi-compile-repository`, `gi-decompile-typelib`, and `gi-inspect-typelib` (`safe/crates/girepository/Cargo.toml:11-20`). |

Directory map:

| Path | Contents and contract |
| --- | --- |
| `safe/Cargo.toml` | Workspace membership and shared package metadata. |
| `safe/Cargo.lock` | Resolved Rust dependency versions used for this report. |
| `safe/meson.build` | Thin Meson wrapper for `cargo check`, ABI extraction, and the `abi-shell-build` custom target (`safe/meson.build:19-44`). |
| `safe/debian/` | Ubuntu packaging control files, scripts, patch metadata, tests, and the `debian/rules` build glue. The default package build appends `nodoc noinsttest nogir noudeb` unless `SAFE_FULL_PACKAGE_BUILD` is set (`safe/debian/rules:3-6`). |
| `safe/abi/` | ABI, layout, install, Debian-patch, link-compat, version-script, and test manifests consumed by the verification tools. |
| `safe/tests/` | Editable upstream test mirrors, manifest lists, CVE probes, package tests, and fixtures. The editable mirror contract is recorded in `safe/abi/test-source-path-map.json`. |
| `safe/tools/` | Build, ABI extraction, layout extraction, link-compat, unsafe-audit, package-baseline, Debian patch, CVE regression, and staging tools. |
| `safe/vendor/original/` | Preserved upstream source and installed interface assets used as current comparison/staging input. |
| `safe/vendor/build-check/` | Bootstrap comparison material. The GLib/GIO ABI-shell checks guard against specific retired backend/static-library fragments (`safe/tools/build-abi-shell.py:168-216`), but GObject still wires a local build-check module through `safe/crates/gobject/src/translated/mod.rs:1`; that remaining bootstrap surface is listed under remaining issues. |
| `safe/crates/*` | Rust crates listed above. |

The Debian build does not rely on upstream Meson compilation for the shipped
library objects. `safe/debian/rules` builds the ABI shell with
`python3 tools/build-abi-shell.py --build-root debian/build/deb --multiarch ...`
(`safe/debian/rules:74-75`) and stages the package tree with
`safe/tools/stage-package-tree.py` (`safe/debian/rules:77-78`). Staging resolves
`$SAFE_VENDOR_ORIGINAL` and `$BUILD_ROOT` placeholders from
`safe/abi/installed-files.json` (`safe/tools/stage-package-tree.py:29-56`) and
renders pkg-config metadata such as `glib-2.0`, `gmodule-2.0`, `gio-2.0`, and
`girepository-2.0` (`safe/tools/stage-package-tree.py:68-239`).

## Where the unsafe Rust lives

This bootstrap state still contains a large generated C translation surface.
The authoritative current unsafe inventory is encoded in
`safe/tools/check-unsafe-audit.py:7-38`, which expects 26,244 unsafe tokens
across 30 audit classes and explains each class in
`safe/tools/check-unsafe-audit.py:40-70`. Running it in this pass printed:
`unsafe audit ok: 26244 unsafe tokens covered by 30 audit classes`.

Counts below are token counts from `rg -n "\bunsafe\b" safe/crates --glob
"*.rs"` as normalized by `safe/tools/check-unsafe-audit.py`; they include
`unsafe extern`, `unsafe fn`, `unsafe impl`, unsafe blocks, and
`#[unsafe(...)]` attributes. Paths are repository-root relative.

| Purpose group | File or tree | Count | Unsafe forms | Justification |
| --- | ---: | ---: | --- | --- |
| Shared ABI callback typedefs | `safe/crates/abi-support/src/ffi.rs` | 2 | `unsafe extern` function-pointer typedefs at `safe/crates/abi-support/src/ffi.rs:34` and `safe/crates/abi-support/src/ffi.rs:36` | GLib ABI structs store C callback pointers that Rust cannot call safely without caller-specific contracts. |
| Build-time alias parsing | `safe/crates/glib/build.rs` | 2 | String patterns containing `unsafe extern` at `safe/crates/glib/build.rs:62-63` | Build script scans translated Rust exports; these tokens are not runtime unsafe Rust. |
| Build-time alias parsing | `safe/crates/gio/build.rs` | 2 | String patterns containing `unsafe extern` at `safe/crates/gio/build.rs:61-62` | Build script scans translated GIO exports; these tokens are not runtime unsafe Rust. |
| Build-time GObject safety gate | `safe/crates/gobject/build.rs` | 7 | String patterns for `unsafe` detection at `safe/crates/gobject/build.rs:9-14` and diagnostic text at `safe/crates/gobject/build.rs:62` | Build script enforces `SAFETY:` comments around hand-written GObject unsafe boundaries; these tokens are not runtime unsafe Rust. |
| GIO callback ABI typedefs | `safe/crates/gio/src/lib.rs` | 2 | `unsafe extern` callback typedefs at `safe/crates/gio/src/lib.rs:35-36` | `GActionEntry` exposes C callbacks in `repr(C)` ABI structs. |
| GIO generated compatibility shims | `safe/crates/gio/src/generated_compat.rs` | 74 | `unsafe impl`, `unsafe extern` imports, unsafe blocks, and `pub unsafe extern "C" fn` exports at representative lines `safe/crates/gio/src/generated_compat.rs:23-32`, `safe/crates/gio/src/generated_compat.rs:131-165`, `safe/crates/gio/src/generated_compat.rs:272-344`, and `safe/crates/gio/src/generated_compat.rs:486-750` | Provides Rust-owned replacements for generated GIO enum, portal, D-Bus, stdio-global, and LSan compatibility ABI symbols with raw C out-parameters and GObject calls. |
| GIO translated upstream modules | `safe/crates/gio/src/translated/` | 19,182 | Generated `extern "C"` declarations, callback casts, pointer arithmetic, raw static data, unsafe blocks, and `pub unsafe extern "C" fn` exports; representative files include `safe/crates/gio/src/translated/compat.rs:6-164`, `safe/crates/gio/src/translated/original/gio/gopenuriportal.rs:1`, and `safe/crates/gio/src/translated/original/gio/glocalfileinputstream.rs:2` | C2Rust translation of upstream GIO plus final Rust-owned file-monitor/settings polling fallbacks; retained to preserve the C ABI while functions are retired into safer Rust. |
| GIRepository exported ABI | `safe/crates/girepository/src/exports.rs` | 102 | Macro-generated and direct `pub unsafe extern "C" fn` exports plus unsafe runtime dispatch blocks at `safe/crates/girepository/src/exports.rs:14-24`, `safe/crates/girepository/src/exports.rs:53-80`, `safe/crates/girepository/src/exports.rs:108-148`, and `safe/crates/girepository/src/exports.rs:159-399` | Public GIRepository ABI accepts raw repository pointers, C strings, stack-loaded info structs, and out parameters. |
| GIRepository runtime | `safe/crates/girepository/src/runtime.rs` | 136 | `unsafe extern` imports, `unsafe impl Send/Sync`, unsafe blocks, unsafe helper functions, and one local `unsafe extern` callback at `safe/crates/girepository/src/runtime.rs:42-68`, `safe/crates/girepository/src/runtime.rs:123-127`, `safe/crates/girepository/src/runtime.rs:274-291`, `safe/crates/girepository/src/runtime.rs:626-888`, `safe/crates/girepository/src/runtime.rs:1312`, and `safe/crates/girepository/src/runtime.rs:1609-1636` | Runtime bridges GIRepository state to GObject type registration, GLib allocation/string APIs, C string vectors, and raw GI info out-parameters. |
| GLib byte array ABI | `safe/crates/glib/src/bytes/api.rs` | 3 | `unsafe extern` import and `g_byte_array_new_take` export at `safe/crates/glib/src/bytes/api.rs:7-12` | Raw buffer ownership enters through the C ABI and must be handed to GLib byte-array storage. |
| GLib charset ABI | `safe/crates/glib/src/charset/api.rs` | 20 | `unsafe extern` libc identity imports, unsafe env helpers, and charset exports at `safe/crates/glib/src/charset/api.rs:10`, `safe/crates/glib/src/charset/api.rs:59-99`, and `safe/crates/glib/src/charset/api.rs:113-175` | Mirrors environment-sensitive GLib charset APIs while sanitizing privileged-context environment use. |
| GLib exported globals | `safe/crates/glib/src/data.rs` | 17 | `unsafe impl Sync` for exported const pointers and `#[unsafe(export_name)]` globals at `safe/crates/glib/src/data.rs:6` and `safe/crates/glib/src/data.rs:111-141` | ABI consumers expect process-global data symbols such as `g_ascii_table`, `g_utf8_skip`, and GLib version globals. |
| GLib filesystem wrapper | `safe/crates/glib/src/fileutils/api.rs` | 3 | `unsafe extern` import and `g_canonicalize_filename` export at `safe/crates/glib/src/fileutils/api.rs:8` and `safe/crates/glib/src/fileutils/api.rs:65-66` | Raw C strings and ownership flow through the public file utility ABI. |
| GLib GVariant wrapper | `safe/crates/glib/src/gvariant/api.rs` | 13 | `unsafe extern` imports, unsafe serialized-layout helpers, and GVariant exports at `safe/crates/glib/src/gvariant/api.rs:10`, `safe/crates/glib/src/gvariant/api.rs:207-216`, and `safe/crates/glib/src/gvariant/api.rs:229-281` | Serialized `GVariant` input and borrowed byte/data ownership are represented as C pointers at the ABI. |
| GLib hash table implementation | `safe/crates/glib/src/hash/api.rs` | 132 | Callback typedefs, `unsafe extern` imports, unsafe table helpers, unsafe blocks, and many exported hash-table functions at `safe/crates/glib/src/hash/api.rs:11-14`, `safe/crates/glib/src/hash/api.rs:49-56`, `safe/crates/glib/src/hash/api.rs:250-504`, and `safe/crates/glib/src/hash/api.rs:524-1278` | `GHashTable` is Rust-owned but its keys, values, destroy functions, iterators, and fallback ABI all remain raw GLib C pointers/callbacks. |
| GLib key-file wrapper | `safe/crates/glib/src/keyfile/api.rs` | 3 | `unsafe extern` import and `g_key_file_load_from_data` export at `safe/crates/glib/src/keyfile/api.rs:10` and `safe/crates/glib/src/keyfile/api.rs:18-19` | C buffer and `GError **` contracts enter through the GLib key-file ABI. |
| GLib legacy dispatch | `safe/crates/glib/src/legacy.rs` | 30 | ABI callback typedef and internal `pub(crate) unsafe fn` dispatch wrappers at `safe/crates/glib/src/legacy.rs:18-22`, `safe/crates/glib/src/legacy.rs:126-174`, and `safe/crates/glib/src/legacy.rs:197-387` | Centralizes calls from Rust-owned exports back into translated fallback implementations while preserving ABI behavior. |
| GLib markup wrapper | `safe/crates/glib/src/markup/api.rs` | 14 | `unsafe extern` import, unsafe `GError **` helper, and markup exports at `safe/crates/glib/src/markup/api.rs:15`, `safe/crates/glib/src/markup/api.rs:47`, and `safe/crates/glib/src/markup/api.rs:57-167` | Markup parser ownership, references, parser state, and error out-parameters are C ABI objects. |
| GLib spawn wrapper | `safe/crates/glib/src/spawn/api.rs` | 20 | Callback typedef, `unsafe extern` imports, local unsafe blocks for argv/envp/pipes, and spawn exports at `safe/crates/glib/src/spawn/api.rs:13-15`, `safe/crates/glib/src/spawn/api.rs:33-101`, and `safe/crates/glib/src/spawn/api.rs:140-316` | Spawn APIs consume `char **`, optional child setup callbacks, process environment, and pipe/Fd out-parameters from C. |
| GLib C runtime support | `safe/crates/glib/src/support.rs` | 17 | `unsafe extern` imports, exported C global shims, init-array linkage, LSan shims, and locale exports at `safe/crates/glib/src/support.rs:3-20`, `safe/crates/glib/src/support.rs:30-41`, and `safe/crates/glib/src/support.rs:49-59` | Translated GLib modules need address-compatible stdio/environ globals, locale charset helpers, and sanitizer symbols. |
| GLib translated upstream modules | `safe/crates/glib/src/translated/` | 3,851 | Generated `extern "C"` declarations, unsafe helper functions, pointer arithmetic, static initializer functions, unsafe blocks, and `safe_c2rust_*` exports; representative lines include `safe/crates/glib/src/translated/compat.rs:6-155`, `safe/crates/glib/src/translated/gfileutils.rs:1-463`, and `safe/crates/glib/src/translated/gutf8.rs:1-560` | C2Rust translation of upstream GLib retained at the ABI boundary while Rust-owned modules replace selected APIs. |
| GModule public ABI | `safe/crates/gmodule/src/module_api.rs` | 20 | `pub unsafe extern "C" fn` exports at `safe/crates/gmodule/src/module_api.rs:19-30`, `safe/crates/gmodule/src/module_api.rs:47-65`, and `safe/crates/gmodule/src/module_api.rs:92-144` | GModule APIs expose raw `GModule *`, C strings, symbol out-pointers, and GLib errors. |
| GModule runtime | `safe/crates/gmodule/src/runtime.rs` | 13 | `unsafe extern` loader imports, unsafe helpers, and function-pointer transmutes at `safe/crates/gmodule/src/runtime.rs:22`, `safe/crates/gmodule/src/runtime.rs:43-47`, `safe/crates/gmodule/src/runtime.rs:85-149`, `safe/crates/gmodule/src/runtime.rs:171-236`, and `safe/crates/gmodule/src/runtime.rs:303-332` | Implements GModule over `dlopen`, `dlsym`, `dlclose`, plugin init/unload callbacks, and GLib error/string allocation. |
| GObject callback ABI | `safe/crates/gobject/src/object/mod.rs` | 14 | `unsafe extern` vtable callback typedefs at `safe/crates/gobject/src/object/mod.rs:37-56` | GObject and GParamSpec class structs store C callbacks. |
| GObject signal callback ABI | `safe/crates/gobject/src/signal/mod.rs` | 2 | `unsafe extern` closure callback typedefs at `safe/crates/gobject/src/signal/mod.rs:5-7` | GObject closures invoke C marshalling and destroy callbacks. |
| GObject translated upstream modules | `safe/crates/gobject/src/translated/` | 2,539 | Generated `extern "C"` declarations, callback casts, unsafe blocks, and `safe_c2rust_*` exports; representative root is `safe/crates/gobject/src/translated/original/` | C2Rust translation of upstream GObject retained at the C ABI boundary. |
| GObject type-system callbacks | `safe/crates/gobject/src/type_system/mod.rs` | 17 | `unsafe extern` lifecycle, value-table, interface, and plugin callback typedefs at `safe/crates/gobject/src/type_system/mod.rs:4-18` and `safe/crates/gobject/src/type_system/mod.rs:97-102` | GType registration and plugins expose C lifecycle callback slots. |
| GObject value transform callback | `safe/crates/gobject/src/value/mod.rs` | 1 | `unsafe extern` transform typedef at `safe/crates/gobject/src/value/mod.rs:44` | `GValue` transforms are callback-driven ABI slots. |
| GThread compatibility shims | `safe/crates/gthread/src/compat.rs` | 4 | `#[unsafe(export_name)]` and `pub unsafe extern "C" fn` exports at `safe/crates/gthread/src/compat.rs:3-14` | Deprecated thread initialization symbols must remain exported for old consumers. |
| GThread warning runtime | `safe/crates/gthread/src/runtime.rs` | 2 | `unsafe extern` GLib warning callback and unsafe helper at `safe/crates/gthread/src/runtime.rs:5-14` | Deprecated GThread entrypoints report through GLib's C warning path. |

Unsafe that is not required by the intended public GLib-compatible ABI/API
boundary is limited to build-time code scanning and audit enforcement in
`safe/crates/glib/build.rs`, `safe/crates/gio/build.rs`, and
`safe/crates/gobject/build.rs`; those files contain the word `unsafe` in string
patterns and diagnostics rather than runtime unsafe operations. Runtime unsafe
beyond a direct exported ABI shim is documented in the next section: the most
important cases are the GModule OS dynamic-loader boundary, the C runtime global
bridges, and GIRepository cross-library calls into GObject/GLib.

## Remaining unsafe FFI beyond the original ABI/API boundary

The intended boundary is the upstream GLib-family C ABI itself: exported
`g_*`, `g_module_*`, `g_object_*`, `g_type_*`, `g_dbus_*`, `gi_*`, callback
typedefs, `repr(C)` structs/unions, C string vectors, `GError **`, and global
ABI data symbols. The following FFI surfaces are additional or adjacent to that
boundary.

| Surface | Symbols or library | Evidence | Why needed | Plausible safe-Rust replacement |
| --- | --- | --- | --- | --- |
| OS dynamic loader | `dlopen`, `dlclose`, `dlsym`, `dlerror` from glibc/libdl | Imported at `safe/crates/gmodule/src/runtime.rs:43-47`; used at `safe/crates/gmodule/src/runtime.rs:122-125`, `safe/crates/gmodule/src/runtime.rs:149`, `safe/crates/gmodule/src/runtime.rs:198`, `safe/crates/gmodule/src/runtime.rs:227`, and `safe/crates/gmodule/src/runtime.rs:332` | GModule's purpose is loading process modules and resolving symbols. | Keep a small audited wrapper; a pure safe replacement would still need OS loader FFI or a dependency that wraps it. |
| C runtime globals and locale | `stderr`, `stdin`, `stdout`, `environ`, and `libc::nl_langinfo(libc::CODESET)` | `safe/crates/glib/src/support.rs:3-7`, `safe/crates/glib/src/support.rs:19-20`, `safe/crates/glib/src/support.rs:30-41` | Translated modules and GLib charset APIs need address-compatible C runtime globals and locale codeset semantics. | Some environment and locale behavior could move to Rust `std`, but exported/address-compatible globals and exact locale ABI likely require FFI. |
| GIRepository to GLib/GObject cross-library C calls | `g_object_get_type`, `g_object_new_with_properties`, `g_type_class_ref`, `g_type_name`, `g_type_query`, `g_type_register_static_simple`, `g_malloc`, `g_strdup`, `g_file_error_quark`, `g_quark_to_string`, `g_set_error_literal` | Declared at `safe/crates/girepository/src/runtime.rs:42-68`; used through runtime functions such as `register_type()` at `safe/crates/girepository/src/runtime.rs:274-291`, object type queries at `safe/crates/girepository/src/runtime.rs:804-888`, and `make_strv()` at `safe/crates/girepository/src/runtime.rs:1609-1622` | GIRepository has to register and describe GObject types and allocate GLib-owned string vectors for ABI callers. | Replace with crate-internal Rust APIs once the GLib/GObject/GIRepository crates share a stable internal Rust boundary instead of linking through C ABI symbols. |
| Translated upstream interop | Numerous `extern "C"` blocks and callback casts inside `safe/crates/glib/src/translated/`, `safe/crates/gio/src/translated/`, and `safe/crates/gobject/src/translated/` | Representative entries: `safe/crates/glib/src/translated/gfileutils.rs:1`, `safe/crates/gio/src/translated/original/gio/gopenuriportal.rs:1`, and `safe/crates/gio/src/translated/original/gio/glocalfileinputstream.rs:2` | Generated translations preserve upstream C module structure, libc calls, GLib cross-calls, vtables, and callback shapes. | Retire translated modules into Rust-owned modules with safe internal data structures and narrow ABI shims. |
| Native libraries linked into the shipped ABI shell | `pcre2-8`, `quadmath`, `dl`, `pthread`, `m`, `ffi`, `z`, `mount`, `selinux` | Library native link lists in `safe/tools/build-abi-shell.py:22-95`; pkg-config metadata in `safe/tools/stage-package-tree.py:68-239`; Debian build deps in `safe/debian/control:7-32` | Required by current translated behavior, public pkg-config compatibility, GObject FFI, compression, mount/SELinux integration, dynamic loading, threading, math, and `f128`/quadmath support. | Some functionality can move to Rust crates over time. Public pkg-config compatibility and system integration may still require native libraries. |
| Build/test process FFI and compiler tooling | `pkg-config`, `gcc`/`cc`, `readelf`, `nm`, `ar`, `patchelf`, plus an LD_PRELOAD-style C shim in package-baseline verification | Tool calls appear in `safe/tools/build-abi-shell.py:168-193`, `safe/tools/build-abi-shell.py:489-497`, `safe/tools/link-compat.py:364-366`, and `safe/tools/run-cve-regressions.py:182-188`; Debian declares `patchelf`, `pkgconf`, `rustc`, `meson`, and related tools in `safe/debian/control:7-32` | Needed to link/version shared libraries, compile compatibility probes, inspect symbols, stage packages, and verify install behavior. | Build tooling can be wrapped more tightly, but compiler/linker and ELF inspection are expected for a C ABI replacement. These are not shipped runtime FFI surfaces. |
| Direct `libc` crate use | `libc` crate `0.2.186` | Direct dependency in `safe/crates/glib/Cargo.toml:11-16` and `safe/crates/gio/Cargo.toml:11-14`; resolved in `safe/Cargo.lock:131-133` | Provides C types and libc calls for translated modules and ABI-compatible support. | Replace module-by-module as translated code is retired; ABI shims will still need C type definitions. |

There is no evidence of additional `cbindgen`, `bindgen`, or dynamically loaded
third-party plugin interfaces outside GModule's documented dynamic-loader
semantics. The search used was `rg -n "extern \"C\"|extern \"system\"|libc::|pkg_config|pkg-config|build.rs|crate-type" Cargo.toml crates tools debian meson.build`
from `safe/`, with translated trees treated as the generated upstream C
translation boundary above.

## Remaining issues

The largest remaining issue is that this is still a bootstrap Rust port with
substantial generated unsafe translation. `safe/tools/check-unsafe-audit.py`
currently accepts 26,244 unsafe tokens across `safe/crates/`, and the three
translated roots account for most of them: GIO 19,182, GLib 3,851, and GObject
2,539 (`safe/tools/check-unsafe-audit.py:7-38`). This is not yet a fully
safe-Rust internal implementation. The current memory-safety story depends on
the public ABI shims, the translated code audit bucket, and the verification
commands below, not on `#![forbid(unsafe_code)]`.

`cargo check --workspace` passed in this pass, but it emitted many warnings from
generated code. The observed build output included 612 warnings for `safe-glib`
and 4,480 warnings for `safe-gio`, dominated by generated style, unused items,
and clashing extern declaration warnings. Those warnings do not currently fail
the build.

Build-time Debian tests are intentionally skipped by the ABI-shell packaging
rules: `override_dh_auto_test-arch` and `override_dh_auto_test-indep` both print
"Skipping build-time tests for the ABI shell phase" (`safe/debian/rules:90-94`).
The package build also skips `dh_strip` and `dh_dwz` for patchelf-adjusted safe
binaries (`safe/debian/rules:127-131`). During this pass, `dpkg-buildpackage -b
-uc -us` succeeded, but emitted packaging warnings about more than one
`build.ninja`, skipped stripping/dwz, diverted package shlibdeps, an undefined
`${shlibs:Depends}` for `libgirepository-2.0-dev`, and unused GNOME substitution
variables. The default build profiles skip docs, installed tests, GIR packages,
and udeb packages unless `SAFE_FULL_PACKAGE_BUILD` is set
(`safe/debian/rules:3-6`).

The link-compat verifier passed for dynamic GLib, GObject, GIO, GIO Unix,
GModule, and GThread probes and for static GLib, GObject, GModule, and GThread
probes. Static GIO and GIO Unix probes were skipped by current link-compat
configuration, and static link steps emitted expected glibc warnings for
`getaddrinfo`, `getpwnam_r`, `getpwuid`, `getpwuid_r`, and `dlopen` in static
binaries. This leaves residual risk around static GIO consumers until those
profiles are enabled or covered elsewhere.

Some helper tools are intentionally minimal or delegating in this phase.
`safe/crates/gio/src/tools.rs` delegates to system tools when present
(`safe/crates/gio/src/tools.rs:17-34`, `safe/crates/gio/src/tools.rs:160-181`).
Fallback behavior writes simple placeholder-like outputs for schema compilation,
GIO module caches, resource compilation, and `gdbus-codegen`
(`safe/crates/gio/src/tools.rs:75-145`). These are package/tool compatibility
surfaces, not complete reimplementations of every upstream helper behavior.

Known source-level caveats found by the TODO/FIXME/placeholder scan:

| Issue | Evidence | Impact |
| --- | --- | --- |
| Unsupported `.la` module archives | `safe/crates/gmodule/src/runtime.rs:192` | `g_module_open()` reports libtool archive paths as unsupported instead of loading through libtool archive metadata. |
| Unsupported atomic widths panic in translated compatibility helpers | `safe/crates/glib/src/translated/compat.rs:31-155`, `safe/crates/gio/src/translated/compat.rs:31-164`, `safe/crates/gobject/src/translated/compat.rs:69-168` | If translated code requests an unhandled atomic size, the process panics. Current translated calls exercised by the verifier did not hit this. |
| Generated non-void fallthrough panics remain | `safe/crates/glib/src/translated/gfileutils.rs:2350`, `safe/crates/glib/src/translated/gutf8.rs:557`, and `safe/crates/gio/src/translated/original/gio/gcontenttype.rs:2079` | These are generated defensive fallbacks and should be retired or proven unreachable as modules are rewritten. |
| GObject still exposes a translated build-check module | `safe/crates/gobject/src/translated/mod.rs:1`, `safe/crates/gobject/src/translated/build_check/mod.rs`, and `safe/crates/gobject/src/translated/build_check/gobject/glib_enumtypes.rs` | This is a bootstrap leftover in the source tree. It should be removed, retired, or explicitly justified by a later implementation phase; this report no longer claims that every build-check generated module is guarded away. |
| Upstream editable test mirrors carry upstream FIXME/TODO comments | For example `safe/tests/upstream/gio/meson.build:53-64`, `safe/tests/upstream/gio/meson.build:89-122`, and `safe/tests/upstream/gio/testfilemonitor.c:11` | These mostly document upstream test caveats and editable mirror policy. Treat them as test-suite context, not necessarily safe-port defects. |
| Debian preinst retains an upstream TODO | `safe/debian/libglib2.0-0t64.preinst:12` | Packaging script question inherited from Debian/Ubuntu remains unresolved. |

The editable upstream test mirror contract is explicit. `safe/abi/test-source-path-map.json`
records preserved edits for GLib `meson.build`, GLib `markup-collect.c`,
GThread `meson.build`, GModule `meson.build`, GObject `meson.build`, and an added
GIO `x-content/win32-software/autorun.exe`; GIO/GIRepository/fuzzing mirrors
otherwise map back to `safe/vendor/original/`. Future phases must update that
manifest and this report together if the editable mirror meaning changes.

The current test manifests contain 370 rows in `safe/tests/manifests/full.txt`,
22 in `safe/tests/manifests/fuzzing.txt`, 133 in `safe/tests/manifests/gio.txt`,
11 in `safe/tests/manifests/girepository.txt`, 68 in
`safe/tests/manifests/glib-advanced.txt`, 83 in
`safe/tests/manifests/glib-core.txt`, and 62 in
`safe/tests/manifests/gobject.txt`. `safe/abi/tests.json` is present and was
included in the metadata inventory.

`dependents.json` is a representative Ubuntu 24.04 sample generated on
2026-04-09. It lists 12 dependent packages: `qemu-system-x86`,
`network-manager`, `bluez`, `flatpak`, `modemmanager`, `fwupd`,
`gvfs-daemons`, `gstreamer1.0-tools`, `libvirt-daemon`, `udisks2`,
`tracker-miner-fs`, and `pocillo-icon-theme`. This pass did not run
dependent-specific integration tests for those packages; coverage is through
ABI/link/package/upstream/CVE manifests and the verifiers listed below.

`relevant_cves.json` records 15 relevant non-memory-corruption CVEs out of 47
reviewed CVEs. `safe/docs/cve-matrix.md:5-19` marks all 15 rows implemented:
CVE-2009-3289, CVE-2012-0039, CVE-2018-16428, CVE-2019-12450,
CVE-2019-13012, CVE-2020-6750, CVE-2021-3800, CVE-2021-27218,
CVE-2021-28153, CVE-2023-29499, CVE-2023-32611, CVE-2023-32636,
CVE-2023-32665, CVE-2024-34397, and CVE-2025-4056. This documentation pass did
not rerun `python3 tools/run-cve-regressions.py --all --build-root build-final
--rebuild`; the residual risk is that the CVE status is based on the existing
matrix plus the ABI/package/link verifiers run here.

Debian quilt patch provenance is documented separately in
`safe/docs/debian-patch-provenance.md`. It states that `debian/source/format`
remains `3.0 (quilt)`, `safe/debian/patches/series` is intentionally empty, and
the original Ubuntu/Debian patch queue is classified as absorbed into safe
source/package/test policy or obsolete with rationale
(`safe/docs/debian-patch-provenance.md:1-7`). `python3
tools/verify-debian-patches.py --verify-manifest` passed in this pass.

## Dependencies and other libraries used

Root `safe/Cargo.toml` has workspace membership and package metadata only; direct
Rust dependencies live in member manifests. The direct Cargo dependencies and
the resolved versions observed with `cargo tree -e normal,build,dev` are:

| Dependency | Declared in | Declared version | Resolved version | Purpose |
| --- | --- | --- | --- | --- |
| `safe-gio`, `safe-girepository`, `safe-glib`, `safe-gmodule`, `safe-gobject`, `safe-gthread` | `safe/crates/abi-support/Cargo.toml:8-14` | Path dependencies | `0.1.0` | Lets `abi-support` and `layout-probe` reference all safe ABI layouts. |
| `c2rust-asm-casts` | `safe/crates/glib/Cargo.toml:11-16`, `safe/crates/gobject/Cargo.toml:11-13` | `0.20` | `0.20.0` | Supports C2Rust-generated cast patterns in translated GLib/GObject code. |
| `c2rust-bitfields` | `safe/crates/glib/Cargo.toml:11-16`, `safe/crates/gobject/Cargo.toml:11-13` | `0.20` | `0.20.0` | Provides bitfield support for translated C structs. |
| `f128` | `safe/crates/glib/Cargo.toml:11-16` | `0.2` | `0.2.9` | Provides 128-bit floating-point support required by translated GLib ABI code; pulls `cc` and quadmath-related build support. |
| `libc` | `safe/crates/glib/Cargo.toml:11-16`, `safe/crates/gio/Cargo.toml:11-14` | `0.2` | `0.2.186` | C types, libc symbols, and translated-code support. |
| `num-traits` | `safe/crates/glib/Cargo.toml:11-16` | `0.2` | `0.2.19` | Numeric traits used by translated or compatibility GLib code. |
| `serde` with `derive` | `safe/crates/glib/Cargo.toml:18-21` | `1` | `1.0.228` | Build-time parsing/serialization support for GLib alias generation. |
| `serde_json` | `safe/crates/glib/Cargo.toml:18-21` | `1` | `1.0.149` | Build-time JSON support for GLib alias generation. |
| `shlex` | `safe/crates/glib/Cargo.toml:18-21` | `1` | `1.3.0` | Build-time shell-style token parsing support. |
| `c2rust-asm-casts` | `safe/crates/gio/Cargo.toml:11-14` | `0.22` | `0.22.1` | Supports C2Rust-generated cast patterns in translated GIO code. |
| `c2rust-bitfields` | `safe/crates/gio/Cargo.toml:11-14` | `0.22` | `0.22.1` | Provides bitfield support for translated GIO structs. |

Transitive Rust dependencies observed in `safe/Cargo.lock` include `autocfg
1.5.0`, `c2rust-bitfields-derive 0.20.0` and `0.22.1`, `cc 1.2.61`,
`f128_input 0.2.1`, `f128_internal 0.2.2`, `find-msvc-tools 0.1.9`, `itoa
1.0.18`, `memchr 2.8.0`, `proc-macro2 1.0.103`, `quote 1.0.40`, `serde_core
1.0.228`, `serde_derive 1.0.228`, `syn 1.0.109` and `2.0.106`,
`unicode-ident 1.0.22`, and `zmij 1.0.21` (`safe/Cargo.lock:18-292`).

The unsafe-heavy dependencies are `libc`, `c2rust-asm-casts`,
`c2rust-bitfields`, and `f128`/its build support. They are acceptable for this
bootstrap state because they support the C ABI and C2Rust translation boundary
that has not yet been retired. `cargo geiger` was not installed in this
environment, so dependency unsafe density was not independently quantified in
this pass.

System and native link dependencies:

| Library/tool | Evidence | Purpose |
| --- | --- | --- |
| `pcre2-8` | Linked for GLib in `safe/tools/build-abi-shell.py:22-34`; exposed in pkg-config metadata at `safe/tools/stage-package-tree.py:68-91`; build dependency `libpcre2-dev` at `safe/debian/control:20` | GLib regular-expression compatibility. |
| `quadmath`, `m` | Listed in native libs in `safe/tools/build-abi-shell.py:22-95` | Floating-point/math support for translated GLib-family code and `f128`. |
| `dl` | Listed in `safe/tools/build-abi-shell.py:22-95`; explicitly imported by GModule runtime at `safe/crates/gmodule/src/runtime.rs:43-47` | Dynamic module loading and loader error reporting. |
| `pthread` | Listed in `safe/tools/build-abi-shell.py:22-95`; pkg-config metadata uses `-pthread` in `safe/tools/stage-package-tree.py:93-161` | GLib/GThread/GModule/GObject/GIO thread ABI compatibility. |
| `ffi` | Native dependency for GObject/GIO/GIRepository in `safe/tools/build-abi-shell.py:59-94`; Debian build dependency `libffi-dev` at `safe/debian/control:18` | GObject/GIRepository ABI and introspection support. |
| `z` | Native dependency for GIO/GIRepository in `safe/tools/build-abi-shell.py:72-94`; Debian build dependency `zlib1g-dev` at `safe/debian/control:32` | Compression/resource compatibility. |
| `mount` | Native dependency for GIO/GIRepository in `safe/tools/build-abi-shell.py:72-94`; Debian build dependency `libmount-dev` at `safe/debian/control:19` | Unix mount integration exposed by GIO. |
| `selinux` | Native dependency for GIO/GIRepository in `safe/tools/build-abi-shell.py:72-94`; Debian build dependency `libselinux1-dev` at `safe/debian/control:21` | SELinux-aware file/security behavior. |
| Build tools: `cargo`, `rustc`, `meson`, `pkgconf`, `patchelf`, `gcc`/`cc`, `dpkg-dev`, `debhelper`, `python3`, `docbook-*`, `gettext`, `xsltproc`, `libxml2-utils`, `linux-libc-dev`, `dbus-daemon` and test tools | Debian build dependencies in `safe/debian/control:7-45`; Meson finds `cargo`, `python3`, and `dpkg-architecture` in `safe/meson.build:8-12`; build scripts call `pkg-config` and compiler/symbol tools in `safe/tools/build-abi-shell.py` and `safe/tools/link-compat.py` | Build, link, package, and verify the ABI-compatible Debian artifacts. |

## How this document was produced

Commands run from `/home/yans/safelibs/pipeline/ports/port-glib` unless noted:

```bash
test -f safe/PORT.md || true
find safe -maxdepth 3 -type f | sort | sed -n "1,200p"
cd safe && cargo metadata --format-version=1 --no-deps
cd safe && cargo tree -e normal,build,dev
cd safe && rg -n "\bunsafe\b" crates --glob "*.rs" --stats
cd safe && python3 tools/check-unsafe-audit.py
cd safe && cargo geiger --version
cd safe && rg -n "extern \"C\"|extern \"system\"|libc::|pkg_config|pkg-config|build.rs|crate-type" Cargo.toml crates tools debian meson.build || true
cd safe && rg -n "cbindgen|bindgen" Cargo.toml Cargo.lock crates tools debian meson.build || true
cd safe && rg -n "TODO|FIXME|XXX|HACK|placeholder|unimplemented!|todo!|panic!|stub|unsupported" crates tools debian tests docs abi meson.build Cargo.toml
cd safe && cargo check --workspace
cd safe && python3 tools/extract_abi.py --verify
cd safe && python3 tools/extract_layouts.py --verify
cd safe && python3 tools/link-compat.py --verify-manifests
cd safe && python3 tools/verify-debian-patches.py --verify-manifest
cd safe && python3 tools/build-abi-shell.py --build-root build-abi-shell --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-abi-shell/.stamp
cd safe && python3 tools/link-compat.py --phase abi-shell --build-root build-abi-shell --compile-original-objects --run
cd safe && dpkg-buildpackage -b -uc -us
cd safe && python3 tools/verify-package-baselines.py --source . --work-root build-package-baselines --abi-shell-profiles "nodoc noinsttest nogir noudeb" --install-manifests abi/install-manifests --postinst-manifest abi/postinst-state/runtime.json
cd safe && wc -l tests/manifests/*.txt abi/tests.json
git rev-parse --short HEAD
```

The raw command `cd safe && grep -RIn "\bunsafe\b" . || true` was started but
stopped because generated build/package directories made the output too noisy
for source inventory. The source-scoped `rg` command and
`tools/check-unsafe-audit.py` were used instead. `cargo geiger` was unavailable:
Cargo reported `error: no such command: geiger`.

Files consulted:

| File or directory | Use |
| --- | --- |
| `.plan/phases/01-safe-bootstrap.md` | Source phase map for artifact scope and verifier commands. |
| `safe/Cargo.toml`, `safe/Cargo.lock`, `safe/crates/*/Cargo.toml`, `safe/crates/*/src/` | Workspace, dependency, crate, module, ABI, unsafe, and FFI inventory. |
| `safe/meson.build`, `safe/debian/rules`, `safe/debian/control`, `safe/debian/patches/series` | Build and packaging architecture. |
| `safe/tools/build-abi-shell.py`, `safe/tools/stage-package-tree.py`, `safe/tools/check-unsafe-audit.py`, `safe/tools/extract_abi.py`, `safe/tools/extract_layouts.py`, `safe/tools/link-compat.py`, `safe/tools/verify-debian-patches.py`, `safe/tools/verify-package-baselines.py`, `safe/tools/run-cve-regressions.py` | ABI shell, staging, unsafe audit, verification, and CVE command evidence. |
| `safe/abi/tests.json`, `safe/abi/test-source-path-map.json`, `safe/abi/link-compat/*.json`, `safe/abi/layout-manifests/*.json`, `safe/abi/install-manifests/*.json`, `safe/abi/debian-patches.json`, `safe/abi/version-scripts/*.map` | Test, editable mirror, ABI, layout, install, patch, and symbol export manifests. |
| `safe/tests/upstream/*`, `safe/tests/manifests/*`, `safe/tests/cve/*`, `safe/docs/cve-matrix.md`, `safe/docs/debian-patch-provenance.md` | Test coverage, known caveats, CVE status, and Debian patch provenance. |
| `dependents.json`, `relevant_cves.json`, `test-original.sh`, `original/` | Dependent package context, CVE scope, final upstream test harness reference, and original source comparison context. |

Heavyweight commands not run in this pass: `python3 tools/run-cve-regressions.py
--all --build-root build-final --rebuild` and `GLIB_UNDER_TEST=safe
GLIB_TEST_SCOPE=all ../test-original.sh`. The residual risk is recorded in the
remaining-issues section above.
