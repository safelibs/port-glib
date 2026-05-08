# GLib Rust Port Report

This report documents the checked-out `port-glib` safe workspace for the
`impl-girepository-rust` documentation phase. It focuses on the Rust
GIRepository implementation under `safe/crates/girepository`, its `gi-*`
tooling, and the shared ABI/packaging files that make it a drop-in Ubuntu 24.04
replacement for `libgirepository-2.0`.

## High-level architecture

The safe tree is a Cargo workspace with seven members:
`crates/abi-support`, `crates/glib`, `crates/gthread`, `crates/gmodule`,
`crates/gobject`, `crates/gio`, and `crates/girepository`
(`safe/Cargo.toml:1`). Workspace package metadata sets Rust 2021,
`LGPL-2.1-or-later`, and version `0.1.0` (`safe/Cargo.toml:13-16`).

`safe-girepository` is a library plus three helper binaries. The crate builds
`rlib` and `staticlib` artifacts (`safe/crates/girepository/Cargo.toml:9`) and
defines `gi-compile-repository`, `gi-decompile-typelib`, and
`gi-inspect-typelib` as Rust binaries (`safe/crates/girepository/Cargo.toml:11`).
The binary entrypoints only dispatch into `safe_girepository::tools`
(`safe/crates/girepository/src/bin/gi_compile_repository.rs:1`,
`safe/crates/girepository/src/bin/gi_decompile_typelib.rs:1`,
`safe/crates/girepository/src/bin/gi_inspect_typelib.rs:1`).

The public boundary is the upstream GIRepository C ABI. `exports.rs` contains
the exported `gi_*` surface: direct wrappers for repository path management and
stack-load functions, macro-generated wrappers for implemented metadata
queries, `*_get_type` functions, and a zero-return compatibility set
(`safe/crates/girepository/src/exports.rs:13`,
`safe/crates/girepository/src/exports.rs:83`,
`safe/crates/girepository/src/exports.rs:403`). The version script exports 231
`gi_*` symbols (`safe/abi/version-scripts/libgirepository.map:1`; confirmed with
`rg -n '^[[:space:]]+gi_.*;' abi/version-scripts/libgirepository.map | wc -l`),
and the rebuilt build-root shared object also exports 231 versioned `gi_*`
symbols (`nm -D --defined-only build-girepository/girepository/libgirepository-2.0.so.0.8000.0`).

Internally, `lib.rs` defines the Rust module map and C-compatible ABI layouts:
`GIBaseInfoStack`, `GIArgInfo`, `GITypeInfo`, `GIArgument`, `GITypeTag`,
`GIArrayType`, and `GIAttributeIter`
(`safe/crates/girepository/src/lib.rs:1`,
`safe/crates/girepository/src/lib.rs:12`). Those layouts are included in the
shared layout probe (`safe/crates/abi-support/src/bin/layout-probe.rs:369`).

Data flow is:

1. A C caller enters an exported `gi_*` symbol in `exports.rs`.
2. The wrapper converts raw C pointers and out-pointers into calls on
   `runtime.rs`.
3. `runtime.rs` manages repository state, document caches, GI info handles,
   GType registration, string-vector allocation, and metadata lookup
   (`safe/crates/girepository/src/runtime.rs:96`,
   `safe/crates/girepository/src/runtime.rs:363`,
   `safe/crates/girepository/src/runtime.rs:472`,
   `safe/crates/girepository/src/runtime.rs:626`).
4. Repository loading delegates to `parser.rs`, which can read GIR XML,
   parse upstream binary typelibs enough to recover namespace metadata, and
   compile a safe custom typelib wrapper that embeds GIR text
   (`safe/crates/girepository/src/parser/mod.rs:318`,
   `safe/crates/girepository/src/parser/mod.rs:385`,
   `safe/crates/girepository/src/parser/mod.rs:402`,
   `safe/crates/girepository/src/parser/mod.rs:880`).
5. Tool binaries call `tools.rs`: `gi-compile-repository` converts GIR to the
   safe custom typelib format, `gi-decompile-typelib` reconstructs GIR, and
   `gi-inspect-typelib` prints shared-library or dependency records
   (`safe/crates/girepository/src/tools/mod.rs:98`,
   `safe/crates/girepository/src/tools/mod.rs:117`,
   `safe/crates/girepository/src/tools/mod.rs:141`).

`invoke/mod.rs` is intentionally narrow at this phase. It only records public
tool/invoke constants such as `GI_TYPELIB_PATH`, `--output`,
`--shared-library`, and the default inspected namespace/version
(`safe/crates/girepository/src/invoke/mod.rs:1`).

Build and packaging are driven by the ABI shell rather than by Cargo `cdylib`.
`build.rs` keeps old linker hook environment variables but emits no cdylib args;
it does emit link libraries for GLib, GObject, Gio, GModule, libffi, libdl,
libmount, libselinux, zlib, and libm (`safe/crates/girepository/build.rs:3`,
`safe/crates/girepository/build.rs:19`). `safe/tools/build-abi-shell.py` links
the static archive into `libgirepository-2.0.so.0.8000.0`, with SONAME
`libgirepository-2.0.so.0`, static archive `libgirepository-2.0.a`, version
script `safe/abi/version-scripts/libgirepository.map`, safe GLib/GObject/Gio/
GModule dependencies, and native link libraries
(`safe/tools/build-abi-shell.py:83`). The same script builds the three Rust
helper tools and stages them under `girepository/compiler`,
`girepository/decompiler`, and `girepository/inspector`
(`safe/tools/build-abi-shell.py:139`,
`safe/tools/build-abi-shell.py:598`).

Package staging installs the library, headers, pkg-config file, tools, GIR, and
typelib artifacts. `girepository-2.0.pc` declares `girdir`,
`typelibdir`, `Requires: glib-2.0, gobject-2.0`, and
`Requires.private: gmodule-no-export-2.0, gio-2.0, libffi >= 3.0.0`
(`safe/tools/stage-package-tree.py:219`). The runtime library package installs
`libgirepository-2.0.so.0*` (`safe/debian/libgirepository-2.0-0.install:1`);
the development package installs headers, static archive, unversioned linker
symlink, and pkg-config metadata (`safe/debian/libgirepository-2.0-dev.install:1`);
the GIRepository GIR and typelib packages install `GIRepository-3.0.gir` and
`GIRepository-3.0.typelib`
(`safe/debian/gir1.2-girepository-3.0-dev.install:1`,
`safe/debian/gir1.2-girepository-3.0.install:1`). Debian rules also synthesize
cross-prefixed wrappers for the `gi-*` tools
(`safe/debian/rules:45`), and `libglib2.0-dev-bin.links` exposes the staged
helpers from `/usr/bin` (`safe/debian/libglib2.0-dev-bin.links:1`).

Short directory map:

```text
safe/
  Cargo.toml                                workspace members and package defaults
  crates/girepository/Cargo.toml            safe-girepository lib/staticlib and gi-* bins
  crates/girepository/build.rs              Cargo link-library hints for ABI-shell builds
  crates/girepository/src/lib.rs            module map and public C layout types
  crates/girepository/src/exports.rs        exported GIRepository C ABI wrappers
  crates/girepository/src/runtime.rs        repository state, GType registration, info queries
  crates/girepository/src/parser/mod.rs     GIR parser, binary typelib metadata reader, typelib writer
  crates/girepository/src/tools/mod.rs      gi-compile/decompile/inspect command behavior
  crates/girepository/src/invoke/mod.rs     current invoke/tool constants
  crates/abi-support/src/ffi.rs             shared GLib-family primitive aliases
  crates/abi-support/src/bin/layout-probe.rs GIRepository layout probe entries
  abi/version-scripts/libgirepository.map   exported symbol contract
  abi/link-compat/girepository.json         link/run/test contract for this phase
  tests/manifests/girepository.txt          selected upstream GIRepository tests
  tests/package/girepository-*.sh           installed-package acceptance scripts
  tools/build-abi-shell.py                  staticlib-to-shared-object and tool staging
  tools/stage-package-tree.py               Debian package tree renderer
```

## Where the unsafe Rust lives

Workspace-wide unsafe is still dominated by older translated GLib/GObject/GIO
code. The current inventory command,
`cd safe && rg -n "\bunsafe\b" crates --glob '*.rs'`, reports these source-line
match counts by crate: `gio` 19260, `glib` 4123, `gobject` 2579,
`girepository` 238, `gmodule` 33, `gthread` 6, and `abi-support` 2. The
GIRepository phase owns the `girepository` and shared `abi-support` entries;
their complete line-level inventory is below and matches
`rg -n "\bunsafe\b" crates/girepository crates/abi-support`.

Public ABI callback types:

| Site | Form | Justification |
| --- | --- | --- |
| `safe/crates/abi-support/src/ffi.rs:34` | unsafe extern fn pointer alias | `GenericFn` represents C callback/vtable slots exposed by GLib-family public ABI structs. |
| `safe/crates/abi-support/src/ffi.rs:36` | unsafe extern fn pointer alias | `GDestroyNotify` stores C destroy callbacks supplied by callers. |

External GLib/GObject calls:

| Site | Form | Justification |
| --- | --- | --- |
| `safe/crates/girepository/src/runtime.rs:42` | `unsafe extern "C"` block | Declares GLib/GObject functions used to register GI info GTypes, allocate GLib-owned arrays/strings, query quarks, and set `GError`. |

Thread-safety marker impls:

| Site | Form | Justification |
| --- | --- | --- |
| `safe/crates/girepository/src/runtime.rs:123` | `unsafe impl Sync` | `TypeRegistry` stores process-global immutable GType ids after `OnceLock` initialization. |
| `safe/crates/girepository/src/runtime.rs:124` | `unsafe impl Send` | `TypeRegistry` contains scalar GType ids and is moved/shared only through the `OnceLock` registry. |
| `safe/crates/girepository/src/runtime.rs:369` | `unsafe impl Send` | `InfoEntry` is stored behind a `Mutex<HashMap<...>>`; its raw-pointer-facing identity is tracked by address keys while owned data is cloned Rust metadata. |

GType registration and layout-backed info storage:

| Site | Form | Justification |
| --- | --- | --- |
| `safe/crates/girepository/src/runtime.rs:127` | unsafe block | Initializes the global GI GType registry by calling GObject registration/query functions. |
| `safe/crates/girepository/src/runtime.rs:274` | unsafe fn | `register_type` wraps `g_type_register_static_simple`, which consumes raw C strings and nullable class/instance init callbacks. |
| `safe/crates/girepository/src/runtime.rs:275` | unsafe block | Calls `g_type_register_static_simple` with C ABI arguments. |
| `safe/crates/girepository/src/runtime.rs:288` | unsafe fn | `type_query` fills a C `GTypeQuery` out-struct. |
| `safe/crates/girepository/src/runtime.rs:290` | unsafe block | Passes an uninitialized out-pointer to `g_type_query`. |
| `safe/crates/girepository/src/runtime.rs:291` | unsafe block | Assumes `g_type_query` initialized the `GTypeQuery` struct. |
| `safe/crates/girepository/src/runtime.rs:410` | unsafe block | Calls `g_type_class_ref` to populate `GIBaseInfoStack.parent_instance.g_class`. |
| `safe/crates/girepository/src/runtime.rs:435` | unsafe block | Calls `g_type_class_ref` for stack-allocated GI info. |
| `safe/crates/girepository/src/runtime.rs:436` | unsafe block | Clears and writes a caller-provided `GIBaseInfoStack` out-struct. |

Exported public ABI wrappers in `exports.rs`:

| Site | Form | Justification |
| --- | --- | --- |
| `safe/crates/girepository/src/exports.rs:14` | macro-generated unsafe extern fn | `abi_ret!` emits C ABI functions that receive raw pointers and return scalar/pointer values. |
| `safe/crates/girepository/src/exports.rs:24` | macro-generated unsafe extern fn | `abi_void!` emits C ABI functions with raw pointer arguments and no return value. |
| `safe/crates/girepository/src/exports.rs:34` | macro-generated unsafe extern fn | `abi_get_type!` emits `*_get_type` C ABI functions. |
| `safe/crates/girepository/src/exports.rs:45` | macro-generated unsafe extern fn | `abi_zero_arg_symbols!` emits compatibility symbols that currently return `0`. |
| `safe/crates/girepository/src/exports.rs:53,58,63,71,76` | unsafe extern fn plus unsafe runtime call | Repository construction and path APIs expose the intended GIRepository C ABI and forward raw C pointers to runtime helpers. |
| `safe/crates/girepository/src/exports.rs:108-138` | `abi_ret!` invocations with unsafe blocks | Repository load, namespace, dependency, and info lookup exports call raw-pointer runtime functions. |
| `safe/crates/girepository/src/exports.rs:142` | unsafe extern fn plus unsafe block | `gi_repository_get_object_gtype_interfaces` writes C out-parameters and forwards raw arrays. |
| `safe/crates/girepository/src/exports.rs:159,164` | unsafe extern fn plus unsafe runtime call | Base-info clear/unref consume raw GI info pointers from C callers. |
| `safe/crates/girepository/src/exports.rs:162,167,170,173` | `abi_ret!` invocations with unsafe blocks | Base-info ref/name/namespace/attribute exports operate on raw info handles. |
| `safe/crates/girepository/src/exports.rs:177-203` | `abi_ret!`/direct unsafe extern entries | Arg-info exports read metadata and write stack `GITypeInfo` out-structs. |
| `safe/crates/girepository/src/exports.rs:207-240` | `abi_ret!`/direct unsafe extern entries | Callable-info exports expose raw argument/return metadata and stack-load outputs. |
| `safe/crates/girepository/src/exports.rs:245-258` | `abi_ret!` invocations with unsafe blocks | Enum and field info exports return raw GI info handles. |
| `safe/crates/girepository/src/exports.rs:262-274` | `abi_ret!`/`abi_void!` invocations | Function-info and invoker exports expose GIRepository function metadata and current invoke stubs. |
| `safe/crates/girepository/src/exports.rs:276-307` | `abi_ret!` invocations with unsafe blocks | Interface/object lookup exports receive C strings and return raw GI info handles. |
| `safe/crates/girepository/src/exports.rs:311-324` | `abi_ret!` invocations with unsafe blocks | Registered-type and signal exports bridge metadata to GType/signal fields. |
| `safe/crates/girepository/src/exports.rs:328-366` | `abi_ret!` invocations with unsafe blocks | Struct, type, and typelib exports expose raw field/type metadata. |
| `safe/crates/girepository/src/exports.rs:369-399` | `abi_ret!` invocations with unsafe blocks | Union and vfunc exports return raw metadata handles or out-parameter values. |
| `safe/crates/girepository/src/exports.rs:403-511` | macro-generated unsafe extern fn exports | 107 compatibility symbols remain present for link ABI but return `0` via the macro at line 45. |

Runtime public unsafe functions:

| Site | Form | Justification |
| --- | --- | --- |
| `safe/crates/girepository/src/runtime.rs:626` | unsafe fn plus unsafe block | Allocates a `GIRepository` object with `g_object_new_with_properties`. |
| `safe/crates/girepository/src/runtime.rs:644,668` | unsafe fn plus `CStr::from_ptr` | Search/library path mutators copy caller-owned C strings. |
| `safe/crates/girepository/src/runtime.rs:657,681` | unsafe fn plus out-pointer writes | Search/library path getters return stable GLib-style string vectors and write counts. |
| `safe/crates/girepository/src/runtime.rs:692,712,723,733,743,755,762,776` | unsafe fn group | Repository require, version enumeration, loaded namespace, c-prefix, dependency, count, info, and name lookup functions consume C strings and raw repository handles. |
| `safe/crates/girepository/src/runtime.rs:804,824` | unsafe fn plus unsafe C calls | GType and error-domain lookup call `g_type_name` and `g_quark_to_string`. |
| `safe/crates/girepository/src/runtime.rs:840` | unsafe fn plus out-pointer writes and `g_malloc` | Interface lookup allocates a C array with GLib allocation and writes caller out-parameters. |
| `safe/crates/girepository/src/runtime.rs:891,900,923` | unsafe fn plus `Box::from_raw`/zeroing | Base-info lifetime and stack cleanup consume raw handles that may own boxed ABI structs. |
| `safe/crates/girepository/src/runtime.rs:942,957,975` | unsafe fn group | Base-info metadata access returns leaked C strings or null pointers through C ABI. |
| `safe/crates/girepository/src/runtime.rs:992-1040` | unsafe fn group | Arg-info getters and stack-load helpers write C out-parameters and create raw info handles. |
| `safe/crates/girepository/src/runtime.rs:1047-1097` | unsafe fn group | Callable-info getters and stack-load helpers return raw metadata handles. |
| `safe/crates/girepository/src/runtime.rs:1103-1129` | unsafe fn group | Enum/value/field helpers return raw metadata handles. |
| `safe/crates/girepository/src/runtime.rs:1136-1172` | unsafe fn group plus unsafe block | Function-info helpers expose symbols; `function_invoke` only sets a `GError` for the `g_file_read_link` path and otherwise returns failure. |
| `safe/crates/girepository/src/runtime.rs:1176-1298` | unsafe fn group plus out-pointer writes | Interface/object method, vfunc, signal, property, and interface-declarer lookups return raw metadata handles. |
| `safe/crates/girepository/src/runtime.rs:1312` | unsafe extern fn | `local_ref_func` is a C ABI placeholder returned as a ref-function pointer when metadata expects one. |
| `safe/crates/girepository/src/runtime.rs:1314-1355` | unsafe fn group plus unsafe C call | Registered-type and signal helpers return GType/type-name/type-init/signal metadata; `g_object_get_type` is used for that exact type-init symbol. |
| `safe/crates/girepository/src/runtime.rs:1361-1455` | unsafe fn group plus out-pointer writes | Struct/type/typelib helpers expose field counts, sizes, array metadata, interfaces, pointer flags, and raw typelib handles. |
| `safe/crates/girepository/src/runtime.rs:1459-1501` | unsafe fn group plus out-pointer writes | Union/vfunc helpers expose methods, fields, discriminator offsets, sizes, and invokers. |
| `safe/crates/girepository/src/runtime.rs:1609` | unsafe fn plus unsafe blocks | `make_strv` allocates a GLib-compatible null-terminated `char **` with `g_malloc` and `g_strdup`. |
| `safe/crates/girepository/src/runtime.rs:1636` | unsafe block | `ptr_string` copies caller-owned C strings using `CStr::from_ptr`. |

Unsafe that is not strictly required by the public GIRepository C ABI/API:
`TypeRegistry`/`InfoEntry` marker impls (`runtime.rs:123`, `runtime.rs:124`,
`runtime.rs:369`) are internal synchronization assertions, not C ABI shims.
The `local_ref_func` placeholder (`runtime.rs:1312`) is C ABI-shaped, but it is
an internal stand-in returned by `object_get_ref_function_pointer`, not an
exported GIRepository symbol.

## Remaining unsafe FFI beyond the original ABI/API boundary

The intended public boundary is `libgirepository-2.0`'s exported `gi_*` C ABI in
`exports.rs` and `libgirepository.map`. Additional FFI beyond that boundary is
limited to GLib/GObject integration calls in `runtime.rs` and link-library
configuration:

| Symbols | Provider | Why needed | Safe Rust replacement path |
| --- | --- | --- | --- |
| `g_object_get_type`, `g_object_new_with_properties`, `g_type_class_ref`, `g_type_name`, `g_type_query`, `g_type_register_static_simple` (`safe/crates/girepository/src/runtime.rs:43`) | GLib/GObject, linked through the safe workspace and loaded as `libglib-2.0.so.0`/`libgobject-2.0.so.0` in the ABI-shell object (`objdump -p build-girepository/girepository/libgirepository-2.0.so.0.8000.0`) | GIRepository info objects are GObject-shaped and must report compatible GTypes/classes to C callers. | A future safe GObject registration API in `safe-gobject` could wrap these calls, but the ABI still requires GType interop. |
| `g_malloc`, `g_strdup` (`safe/crates/girepository/src/runtime.rs:62`) | GLib allocator/string APIs | C callers expect GLib-owned string vectors from APIs such as version and dependency enumeration. | A safe wrapper over the GLib allocator could centralize allocation, but returning GLib-freeable memory remains necessary. |
| `g_file_error_quark`, `g_quark_to_string`, `g_set_error_literal` (`safe/crates/girepository/src/runtime.rs:64`) | GLib error/quark APIs | `find_by_error_domain` and the current `function_invoke` fallback need GLib quark/error semantics. | Safe wrapper functions around `GQuark`/`GError` would reduce localized unsafe but not remove the ABI dependency. |
| `glib-2.0`, `gobject-2.0`, `gio-2.0`, `gmodule-2.0`, `ffi`, `dl`, `mount`, `selinux`, `z`, `m` (`safe/crates/girepository/build.rs:19`) | System/safe GLib-family libraries plus native system libraries | Cargo/build-shell link configuration mirrors the Ubuntu GLib dependency set and allows static archive linking into the shared ABI shell. | Unused libraries can be pruned only if the package link contract and pkg-config metadata are also changed. |

No other Rust source in `safe/crates/girepository` declares foreign calls:
`rg -n 'extern "C"' crates/girepository crates/abi-support` finds only the
public exports, shared callback type aliases, the one runtime foreign block,
and `local_ref_func`.

## Remaining issues

The 231-symbol link ABI is complete, but not every symbol has complete runtime
semantics. The 107 functions generated by `abi_zero_arg_symbols!` return `0`
(`safe/crates/girepository/src/exports.rs:403`). This includes loader and
metadata surfaces such as `gi_repository_load_typelib`,
`gi_repository_is_registered`, `gi_repository_get_version`,
`gi_typelib_validate`, field offset/size accessors, closure/invoke helpers, and
several object/interface/property/vfunc accessors
(`safe/crates/girepository/src/exports.rs:409`,
`safe/crates/girepository/src/exports.rs:419`,
`safe/crates/girepository/src/exports.rs:472`,
`safe/crates/girepository/src/exports.rs:501`).

Invoke support is deliberately shallow. `gi_function_info_prep_invoker` returns
success (`safe/crates/girepository/src/runtime.rs:1172`), but
`function_invoke` ignores argument arrays, only special-cases `g_file_read_link`
to set a `G_FILE_ERROR_NOENT`, and otherwise returns `0`
(`safe/crates/girepository/src/runtime.rs:1147`). `invoke/mod.rs` contains only
tool constants (`safe/crates/girepository/src/invoke/mod.rs:1`).

The GIR parser is pragmatic rather than a full XML/GIR implementation. It uses
a small local XML parser and selected attribute extraction
(`safe/crates/girepository/src/parser/mod.rs:990`), guesses a few layout values
(`safe/crates/girepository/src/parser/mod.rs:872`), supports safe custom
typelibs by embedding GIR text (`safe/crates/girepository/src/parser/mod.rs:385`),
and reads only metadata from upstream binary typelibs
(`safe/crates/girepository/src/parser/mod.rs:880`). This is enough for the
current manifest but not bit-for-bit equivalent to upstream's full typelib
loader/writer.

Search paths contain x86_64-specific defaults. Runtime repository lookup starts
with `/usr/local/lib/x86_64-linux-gnu/girepository-1.0`
(`safe/crates/girepository/src/runtime.rs:24`), and the tool search path falls
back to `/usr/lib/x86_64-linux-gnu/girepository-1.0` and
`/usr/lib/x86_64-linux-gnu/gir-1.0`
(`safe/crates/girepository/src/tools/mod.rs:68`,
`safe/crates/girepository/src/tools/mod.rs:83`). Debian packaging passes the
host multiarch into staging and wrappers (`safe/debian/rules:56`,
`safe/tools/stage-package-tree.py:219`), but non-x86_64 runtime lookup should be
rechecked before claiming cross-architecture equivalence.

Package-facing acceptance is covered by scripts but not by installed packages
in this documentation run. `girepository-compile-only.sh` requires
`pkg-config --exists girepository-2.0`, the installed header, and a trivial C
consumer that calls `gi_repository_new`
(`safe/tests/package/girepository-compile-only.sh:8`,
`safe/tests/package/girepository-consumer.c:1`). `girepository-installed.sh`
also checks `/usr/bin` tool links, GIR/typelib install locations,
`gi-compile-repository`, `gi-inspect-typelib --print-shlibs`,
`gi-inspect-typelib --print-typelibs`, and `gi-decompile-typelib`
(`safe/tests/package/girepository-installed.sh:8`). These scripts are tied to
the Debian install/link files listed above, but they were not executed against a
newly installed `.deb` in this documentation pass.

The selected upstream GIRepository manifest has 11 rows:
`cmph-bdz`, `gthash`, `function-info`, `object-info`,
`registered-type-info`, `repository`, `repository-search-paths`,
`struct-info`, `throws`, `union-info`, and `autoptr-girepository`
(`safe/tests/manifests/girepository.txt:1`). The build-root manifest runner and
link-compat run both passed against `build-girepository`, but broader upstream
GIRepository tests outside this frozen manifest remain out of scope.

No `PLACEHOLDER_`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, or `panic!`
markers were found in `safe/crates/girepository`, the package scripts, or the
GIRepository install/link files by
`rg -n 'PLACEHOLDER_|TODO|FIXME|unimplemented!|todo!|panic!'`.

`dependents.json` records 12 representative GLib dependents, including
`qemu-system-x86`, `network-manager`, `bluez`, `flatpak`, `modemmanager`,
`fwupd`, `gvfs-daemons`, `gstreamer1.0-tools`, `libvirt-daemon`, `udisks2`,
`tracker-miner-fs`, and `pocillo-icon-theme` (`dependents.json:1`). These are
package-level GLib/GObject/GIO dependents; this GIRepository phase only proves
the selected GIRepository ABI and package-facing scripts, not every dependent's
introspection workflow.

`relevant_cves.json` lists 15 non-memory-corruption CVEs whose root causes could
survive a Rust rewrite (`relevant_cves.json:1`). Most are GLib/GIO filesystem,
GVariant, GDBus, hash, charset, or spawn semantics rather than GIRepository
metadata parsing. This phase does not add new CVE regressions for
GIRepository specifically, but the remaining parser and invoke limitations mean
the report should not be read as a full mitigation claim for all listed CVE
classes.

## Dependencies and other libraries used

`safe/Cargo.toml` itself declares workspace members and package metadata, not
third-party dependencies (`safe/Cargo.toml:1`). Direct member dependencies from
the current manifests and resolved versions from `cargo tree`/`Cargo.lock` are:

| Crate | Direct dependencies | Purpose |
| --- | --- | --- |
| `safe-girepository` | none (`safe/crates/girepository/Cargo.toml:1`) | Uses only `std`, shared local ABI aliases via `#[path]`, and external GLib/GObject FFI. |
| `abi-support` | `safe-gio`, `safe-girepository`, `safe-glib`, `safe-gmodule`, `safe-gobject`, `safe-gthread` path deps (`safe/crates/abi-support/Cargo.toml:8`) | Layout-probe crate that imports public ABI layout types from each workspace member. |
| `safe-gio` | `c2rust-asm-casts` `0.22` resolved `0.22.1`, `c2rust-bitfields` `0.22` resolved `0.22.1`, `libc` `0.2` resolved `0.2.186` (`safe/crates/gio/Cargo.toml:11`) | Supports translated GIO code, C bitfields, inline-cast patterns, and libc bindings. |
| `safe-glib` | `c2rust-asm-casts` `0.20` resolved `0.20.0`, `c2rust-bitfields` `0.20` resolved `0.20.0`, `f128` `0.2` resolved `0.2.9`, `libc` `0.2` resolved `0.2.186`, `num-traits` `0.2` resolved `0.2.19`; build deps `serde` `1` resolved `1.0.228`, `serde_json` `1` resolved `1.0.149`, `shlex` `1` resolved `1.3.0` (`safe/crates/glib/Cargo.toml:11`) | Supports translated GLib code, long-double handling, numeric helpers, libc bindings, and build-script manifest parsing. |
| `safe-gobject` | `c2rust-asm-casts` `0.20` resolved `0.20.0`, `c2rust-bitfields` `0.20` resolved `0.20.0` (`safe/crates/gobject/Cargo.toml:11`) | Supports translated GObject code and C bitfield layouts. |
| `safe-gmodule` | none (`safe/crates/gmodule/Cargo.toml:1`) | Local staticlib/rlib for GModule ABI. |
| `safe-gthread` | none (`safe/crates/gthread/Cargo.toml:1`) | Local staticlib/rlib for GThread ABI. |

The unsafe-heavy Rust dependencies are the C2Rust support crates and `libc`.
They are not used by `safe-girepository` directly; they remain acceptable in
the workspace because older GLib/GObject/GIO crates still contain mechanically
translated C code and C ABI surfaces. No workspace crate currently uses
`#![forbid(unsafe_code)]`.

Build-time and runtime C/system libraries are declared in three places:

| Source | Libraries/tools | Purpose |
| --- | --- | --- |
| `safe/crates/girepository/build.rs:19` | `glib-2.0`, `gobject-2.0`, `gio-2.0`, `gmodule-2.0`, `ffi`, `dl`, `mount`, `selinux`, `z`, `m` | Cargo link hints for ABI-shell staticlib/shared-object builds. |
| `safe/tools/build-abi-shell.py:83` | safe GLib/GObject/Gio/GModule deps and native `ffi`, `dl`, `mount`, `selinux`, `z`, `quadmath`, `pthread`, `m` | Links `libgirepository-2.0.so.0.8000.0` from the Rust static archive. |
| `safe/tools/stage-package-tree.py:219` | pkg-config `Requires` and `Requires.private` for GLib/GObject/Gio/GModule/libffi plus `Libs.private: -lm` | Publishes the development link contract. |
| `safe/debian/control:7` | `cargo`, `rustc`, `libffi-dev`, `libmount-dev`, `libselinux1-dev`, `zlib1g-dev`, `meson`, `pkgconf`, `patchelf`, Python, and standard Debian helper packages | Debian build and package-staging toolchain. |

The built `libgirepository-2.0.so.0.8000.0` in `build-girepository` currently
has dynamic `NEEDED` entries for `libglib-2.0.so.0`, `libgobject-2.0.so.0`,
`libgcc_s.so.1`, `libc.so.6`, and the ELF interpreter, with SONAME
`libgirepository-2.0.so.0` (`objdump -p build-girepository/girepository/libgirepository-2.0.so.0.8000.0`).

## How this document was produced

Files consulted: `safe/PORT.md`, `.plan/phases/06-girepository-tools.md`,
`safe/Cargo.toml`, `safe/crates/abi-support/Cargo.toml`,
`safe/crates/gio/Cargo.toml`, `safe/crates/girepository/Cargo.toml`,
`safe/crates/glib/Cargo.toml`, `safe/crates/gmodule/Cargo.toml`,
`safe/crates/gobject/Cargo.toml`, `safe/crates/gthread/Cargo.toml`,
`safe/crates/girepository/build.rs`,
`safe/crates/girepository/src/lib.rs`,
`safe/crates/girepository/src/exports.rs`,
`safe/crates/girepository/src/runtime.rs`,
`safe/crates/girepository/src/parser/mod.rs`,
`safe/crates/girepository/src/repository/mod.rs`,
`safe/crates/girepository/src/invoke/mod.rs`,
`safe/crates/girepository/src/tools/mod.rs`,
`safe/crates/abi-support/src/ffi.rs`,
`safe/crates/abi-support/src/bin/layout-probe.rs`,
`safe/tools/build-abi-shell.py`, `safe/tools/stage-package-tree.py`,
`safe/abi/version-scripts/libgirepository.map`,
`safe/abi/link-compat/girepository.json`,
`safe/abi/installed-files.json`,
`safe/tests/manifests/girepository.txt`,
`safe/tests/package/girepository-compile-only.sh`,
`safe/tests/package/girepository-installed.sh`,
`safe/tests/package/girepository-consumer.c`,
`safe/debian/rules`, `safe/debian/libgirepository-2.0-0.install`,
`safe/debian/libgirepository-2.0-dev.install`,
`safe/debian/gir1.2-girepository-3.0.install`,
`safe/debian/gir1.2-girepository-3.0-dev.install`,
`safe/debian/libglib2.0-dev-bin.links`,
`original/girepository/`, `original/girepository/tests/`, `dependents.json`,
and `relevant_cves.json`.

Commands run:

```bash
cd safe && cargo tree -p safe-girepository -e normal,build,dev
cd safe && cargo metadata --format-version=1 --no-deps
cd safe && cargo tree -p safe-glib -e normal,build,dev --depth 2
cd safe && cargo tree -p safe-gio -e normal,build,dev --depth 2
cd safe && cargo tree -p safe-gobject -e normal,build,dev --depth 2
cd safe && rg -n "\bunsafe\b" crates/girepository crates/abi-support
cd safe && rg -n "\bunsafe\b" crates --glob '*.rs'
cd safe && rg -n 'extern "C"' crates/girepository crates/abi-support
cd safe && rg -n 'PLACEHOLDER_|TODO|FIXME|unimplemented!|todo!|panic!' crates/girepository tests/package debian/libgirepository-2.0-0.install debian/libgirepository-2.0-dev.install debian/gir1.2-girepository-3.0.install debian/gir1.2-girepository-3.0-dev.install debian/libglib2.0-dev-bin.links abi/link-compat/girepository.json
cd safe && rg -n '^[[:space:]]+gi_.*;' abi/version-scripts/libgirepository.map | wc -l
cd safe && jq '.entry_ids | length' abi/link-compat/girepository.json
cd safe && cargo check --workspace
cd safe && cargo geiger -p safe-girepository --all-targets
cd safe && python3 tools/build-abi-shell.py --build-root build-girepository --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-girepository/.stamp
cd safe && python3 tools/link-compat.py --phase girepository --build-root build-girepository --compile-original-objects --run
cd safe && python3 tools/run-meson-manifest.py --build-root build-girepository --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-girepository/meson-info/intro-tests.json --manifest tests/manifests/girepository.txt --print-errorlogs
cd safe && ! rg -n "PLACEHOLDER_" crates/girepository/src/exports.rs
cd safe && nm -D --defined-only build-girepository/girepository/libgirepository-2.0.so.0.8000.0
cd safe && objdump -p build-girepository/girepository/libgirepository-2.0.so.0.8000.0
```

`cargo check --workspace`, `tools/build-abi-shell.py`,
`tools/link-compat.py`, `tools/run-meson-manifest.py`, and the
`PLACEHOLDER_` absence check passed. `cargo geiger` was not installed in this
environment (`cargo` reported `no such command: geiger`), so the unsafe
inventory was produced with `rg` rather than Geiger.
