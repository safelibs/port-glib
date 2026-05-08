# GLib Rust Port Report

This report documents the checked-out `port-glib` safe workspace for the
`impl-gio-rust` documentation phase. The code inspected here is the current
Rust GIO implementation under `safe/crates/gio`, plus the shared ABI support,
package staging, test, CVE, and manifest files that define the installed
Ubuntu 24.04 replacement package behavior.

## High-level architecture

The safe port is a Rust workspace under `safe/` that builds GLib-family
replacement shared libraries while preserving the upstream C ABI. The workspace
members are `crates/abi-support`, `crates/glib`, `crates/gthread`,
`crates/gmodule`, `crates/gobject`, `crates/gio`, and `crates/girepository`
(`safe/Cargo.toml:1-11`). Workspace package settings use Rust 2021,
`LGPL-2.1-or-later`, and version `0.1.0` (`safe/Cargo.toml:13-16`).

The GIO crate is `safe-gio` (`safe/crates/gio/Cargo.toml:1-6`). It builds an
`rlib` and `staticlib`, not a direct `cdylib` (`safe/crates/gio/Cargo.toml:8-9`).
`safe/tools/build-abi-shell.py` links that static archive into
`libgio-2.0.so.0.8000.0` with SONAME `libgio-2.0.so.0`, link name
`libgio-2.0.so`, version script `safe/abi/version-scripts/libgio.map`, safe
library dependencies on GLib, GObject, and GModule, and native link libraries
`z`, `mount`, `selinux`, `ffi`, `quadmath`, `dl`, `pthread`, and `m`
(`safe/tools/build-abi-shell.py:71-82`). The build shell verifies that the
linked shared object does not depend on an upstream `libgio-2.0.so.0`, that
the static archive contains `safe_gio-*` Rust members, that fallback archive
fragments such as `libxdgmime` and `libinotify` are absent, and that GIO
sentinel symbols such as `safe_c2rust_g_file_copy`,
`safe_c2rust_g_dbus_connection_signal_subscribe`,
`safe_c2rust_g_proxy_resolver_lookup`,
`safe_c2rust_g_socket_client_connect_to_uri`, and
`safe_c2rust_g_memory_input_stream_new` are present
(`safe/tools/build-abi-shell.py:198-226`).

The public boundary is the GLib-compatible GIO C ABI. `safe/crates/gio/src/lib.rs`
enables `c_variadic` and `extern_types`, carries translated-code lint allowances,
imports shared C primitive aliases from `safe/crates/abi-support/src/ffi.rs`,
and exposes modules for `actions`, `application`, `dbus`, `file`, `proxy`,
`resources`, `settings`, `sockets`, `streams`, `tools`, and `translated`
(`safe/crates/gio/src/lib.rs:1-30`). Its hand-written `abi` module defines the
C-compatible `GActionEntry`, `GDBusInterfaceVTable`, and `GDBusSubtreeVTable`
layouts and the function-pointer aliases used by those tables
(`safe/crates/gio/src/lib.rs:32-71`). The phase marker has not been renamed:
`bootstrap_marker()` still returns `impl-safe-bootstrap`
(`safe/crates/gio/src/lib.rs:76-77`).

The module clusters in `safe/crates/gio/src/actions.rs`,
`application.rs`, `dbus.rs`, `file.rs`, `proxy.rs`, `resources.rs`,
`settings.rs`, `sockets.rs`, and `streams.rs` are cluster markers only:
each currently exports a `CLUSTER` string. The operational implementation still
lives mostly in mechanically translated modules. `safe/crates/gio/src/translated/mod.rs`
loads 233 translated `original/gio/**` modules, including action/application,
file/local-file, stream, socket, proxy, settings, resource, GDBus, inotify,
xdgmime, and GVDB surfaces (`safe/crates/gio/src/translated/mod.rs:1-4`,
`safe/crates/gio/src/translated/mod.rs:220-237`; confirmed with
`rg -n '^#\[path = ' crates/gio/src/translated/mod.rs | wc -l`). Data flow is
therefore ABI-first: a C caller enters an exported `libgio-2.0.so.0` symbol,
which is either a translated `safe_c2rust_*` implementation, a generated
compatibility export, or a generated alias to a translated export. That code
then moves through raw GLib/GObject/GIO pointers, vtables, callbacks, GLib
allocation and container APIs, POSIX file and socket APIs, GDBus state, settings
schema/GVDB data, resources, and package-facing helper tools.

`safe/crates/gio/build.rs` scans the Rust source for `#[unsafe(export_name = ...)]`
exports and translated `#[no_mangle] safe_c2rust_*` exports, then writes alias
assembly to `$OUT_DIR/gio_aliases.rs` for translated symbols that must appear
under the public ABI names (`safe/crates/gio/build.rs:40-49`,
`safe/crates/gio/build.rs:79-124`). `safe/crates/gio/src/exports.rs` includes
that generated alias file (`safe/crates/gio/src/exports.rs:1-4`). Historical
`SAFE_LINK_*` linker hooks are present but `emit_cdylib_arg` is a no-op because
the shared object is produced by `build-abi-shell.py` from the static archive
(`safe/crates/gio/build.rs:6`, `safe/crates/gio/build.rs:141-150`).

GIO helper executables are built from the same crate. The `[[bin]]` entries are
`gapplication`, `gdbus`, `gdbus-codegen`, `gio`, `gio-launch-desktop`,
`gio-querymodules`, `glib-compile-resources`, `glib-compile-schemas`,
`gresource`, and `gsettings` (`safe/crates/gio/Cargo.toml:16-54`). Each binary
delegates to `safe_gio::tools::run_tool("<tool>")`. `run_tool` first handles
`gdbus-codegen` by running the staged Python `codegen_main.py` support modules
when present, delegates to `/usr/bin/<tool>` with `LD_LIBRARY_PATH` removed
when a system tool exists and would not recurse into itself, supports
`--version` and `--help`, and otherwise provides local fallbacks for schema
compilation, resource compilation, module-cache generation, desktop launching,
and minimal `gdbus-codegen` output (`safe/crates/gio/src/tools.rs:17-57`,
`safe/crates/gio/src/tools.rs:75-157`, `safe/crates/gio/src/tools.rs:160-190`).

`safe/tools/build-abi-shell.py` stages rebuilt GIO tools from `cargo build -p
safe-gio --bins`, copies them into the build root, copies the Python
`gdbus-codegen` support modules from the frozen upstream vendor tree, and
checks that staged GIO helpers are not identical to vendored build-check copies
(`safe/tools/build-abi-shell.py:101-115`, `safe/tools/build-abi-shell.py:529-595`).
The build-root `gio-2.0.pc` exposes `schemasdir`, `dtdsdir`, `giomoduledir`,
`gio`, `gio_querymodules`, `glib_compile_schemas`, `glib_compile_resources`,
`gdbus`, `gdbus_codegen`, `gresource`, and `gsettings`
(`safe/tools/build-abi-shell.py:365-379`). The installed package staging writes
equivalent `/usr` pkg-config variables and declares `Requires: glib-2.0,
gobject-2.0` plus `Requires.private: gmodule-no-export-2.0, zlib, mount >=
2.23, libselinux >= 2.2`
(`safe/tools/stage-package-tree.py:180-208`).

Debian packaging drives the ABI shell from `override_dh_auto_build` and stages
the package tree from `override_dh_auto_install`
(`safe/debian/rules:70-78`). The default ABI-shell build profiles append
`nodoc noinsttest nogir noudeb` unless `SAFE_FULL_PACKAGE_BUILD` is set
(`safe/debian/rules:3-6`), and Debian build-time tests are skipped during this
phase (`safe/debian/rules:90-94`). Package-facing helper paths are split across
Debian install lists: `libglib2.0-0t64` installs multiarch
`gio-launch-desktop`, `gio-querymodules`, `glib-compile-schemas`, and
`libgio*.so.*`; `libglib2.0-bin` installs `gapplication`, `gdbus`, `gio`,
`gio-querymodules`, `glib-compile-schemas`, `gresource`, and `gsettings`;
`libglib2.0-dev-bin` installs `gdbus-codegen`, `glib-compile-resources`, and
the Python codegen support modules (`safe/debian/libglib2.0-0t64.install:1-8`,
`safe/debian/libglib2.0-bin.install:1-7`,
`safe/debian/libglib2.0-dev-bin.install:1-10`). `stage-package-tree.py`
copies those helpers to `/usr/bin` and the multiarch GLib helper directory,
and patches installed ELF runpaths for `/usr/bin` and
`/usr/lib/<multiarch>/glib-2.0` helpers (`safe/tools/stage-package-tree.py:11-15`,
`safe/tools/stage-package-tree.py:276-288`,
`safe/tools/stage-package-tree.py:303-324`).

Directory map:

```text
safe/
  Cargo.toml                                  workspace members and shared package settings
  crates/gio/Cargo.toml                       safe-gio library and helper binaries
  crates/gio/build.rs                         export scanner and alias assembly generator
  crates/gio/src/lib.rs                       GIO module map and hand ABI table layouts
  crates/gio/src/generated_compat.rs          hand compatibility exports and stubs
  crates/gio/src/tools.rs                     helper executable dispatcher and fallbacks
  crates/gio/src/translated/compat.rs         translated-code atomic/raw conversion helpers
  crates/gio/src/translated/original/gio/     C2Rust-translated upstream GIO implementation
  crates/gio/src/translated/gvdb.rs           Rust-owned GVDB compatibility implementation
  crates/abi-support/src/ffi.rs               shared GLib-family C primitive aliases
  abi/version-scripts/libgio.map              2107 exported-symbol contract
  abi/link-compat/gio.json                    2138 link-compat entry ids
  abi/installed-files.json                    installed file/source inventory
  debian/                                     Ubuntu package glue and install lists
  tests/manifests/gio.txt                     133 selected upstream GIO test rows
  tests/cve/keyfile-settings-backend.c        GIO keyfile settings CVE regression
  tools/build-abi-shell.py                    staticlib-to-shared-object and helper staging
  tools/stage-package-tree.py                 installed package tree renderer
  tools/run-meson-manifest.py                 upstream manifest runner
  tools/run-cve-regressions.py                CVE regression runner
```

## Where the unsafe Rust lives

The current unsafe inventory command,
`cd safe && rg -n "\bunsafe\b" crates/gio crates/abi-support`, reports 19262
matching source lines across 239 Rust files. A token-level classification over
those same files found:

| Unsafe form | Count | Why it exists |
| --- | ---: | --- |
| `unsafe impl` | 2 | `GEnumValue` and `GFlagsValue` contain raw C string pointers but are immutable leaked type-registration tables shared through statics. |
| `unsafe extern "C" { ... }` blocks | 2 | Hand compatibility code declares libc stdio globals and GLib/GObject registration/allocation symbols it must call. |
| `unsafe extern "C" fn` definitions or function-pointer forms | 18705 | The translated GIO runtime, callback typedefs, vtable slots, exported ABI shims, and generated compatibility functions all use C calling conventions and raw pointers. |
| `unsafe fn` | 23 | Shared translated helpers expose raw pointer casts, atomics, enum/flag registration helpers, and GVDB table helpers. |
| `unsafe { ... }` blocks | 483 | Raw pointer dereferences, static mut access, FFI calls, function-pointer dispatch, and generated table initialization. |
| `#[unsafe(...)]` attributes or unsafe link sections | 45 | Export names, `no_mangle`, and the Linux `.init_array` stdio initializer. |
| Other `unsafe` mentions | 3 | Build-script pattern strings that scan source text for unsafe exports. |

The high-volume unsafe code is intentional but not finished: the crate does not
use `#![forbid(unsafe_code)]`, and most GIO behavior remains in translated
raw-pointer code rather than safe Rust modules.

Grouped inventory and justification:

- Shared callback ABI aliases: `safe/crates/abi-support/src/ffi.rs:34,36`
  define `GenericFn` and `GDestroyNotify` as unsafe C function-pointer aliases.
  They are required because public GLib/GIO APIs store callbacks supplied by C.
- Hand GIO ABI table callbacks: `safe/crates/gio/src/lib.rs:35-36` defines
  `GActionActivateFunc` and `GActionChangeStateFunc` as unsafe extern callback
  types, while `safe/crates/gio/src/lib.rs:37-42` routes GDBus vtable callbacks
  through `GenericFn`. These are required by `GActionEntry`,
  `GDBusInterfaceVTable`, and `GDBusSubtreeVTable`.
- Generated enum/flags compatibility: `safe/crates/gio/src/generated_compat.rs:23-24`
  has the only `unsafe impl` entries, making immutable enum and flags value
  structs usable in statics. `safe/crates/gio/src/generated_compat.rs:32-38`
  declares GLib/GObject functions used for enum/flag registration, allocation,
  object references, and string duplication. `register_enum_type` and
  `register_flags_type` are unsafe at `safe/crates/gio/src/generated_compat.rs:272`
  and `safe/crates/gio/src/generated_compat.rs:276`; their call sites are the
  unsafe blocks at `safe/crates/gio/src/generated_compat.rs:273,277,286,296,308,346`.
- ABI-shell stdio and sanitizer compatibility: when `safe_abi_shell_build` is
  set, `safe/crates/gio/src/generated_compat.rs:27-29` imports libc `stderr`
  and `stdout`, `safe/crates/gio/src/generated_compat.rs:139-157` exports and
  initializes translated stdio globals from `.init_array`, and
  `safe/crates/gio/src/generated_compat.rs:160-165` exports no-op lsan hooks.
  These are build-shell compatibility exports, not the public GIO API itself.
- Rust-owned compatibility exports and placeholder portal/D-Bus helpers:
  `safe/crates/gio/src/generated_compat.rs:282-306` defines export-generating
  macros for many enum and flag `*_get_type` symbols,
  `safe/crates/gio/src/generated_compat.rs:343-344` exports
  `g_credentials_type_get_type`, `safe/crates/gio/src/generated_compat.rs:486-727`
  exports `gxdp_*` portal helper functions, and
  `safe/crates/gio/src/generated_compat.rs:747-822` exports no-op
  `_g_freedesktop_dbus_complete_*` functions through `complete_noop!`. Their
  unsafe exists because the symbols must match C ABI signatures and touch raw
  out-pointers.
- Translated atomic/raw helpers: `safe/crates/gio/src/translated/compat.rs:6,11,16,21`
  are raw value conversion helpers, and
  `safe/crates/gio/src/translated/compat.rs:27,36,45,54,63,72,81,90,99,124,132,140,148,156`
  are public unsafe atomic helpers over caller-provided raw addresses. These
  are not public GIO ABI functions, but they support translated C atomic
  operations.
- GVDB compatibility: `safe/crates/gio/src/translated/gvdb.rs:139,152,188`
  has internal unsafe helpers for GLib error/table handling, and
  `safe/crates/gio/src/translated/gvdb.rs:210-426` exports unsafe C-compatible
  GVDB table/hash functions. GVDB is used by settings schema data; the exports
  remain raw-pointer ABI shims.
- Translated support globals: `safe/crates/gio/src/translated/support.rs:3-4`
  exports a no-op `__lsan_disable`, and
  `safe/crates/gio/src/translated/support.rs:6,13` exports IPv6 address
  statics expected by translated networking code.
- Alias generator patterns: `safe/crates/gio/build.rs:61-62` are string
  patterns used to find unsafe extern functions in translated files. They are
  not runtime unsafe code.
- Mechanically translated upstream GIO modules:
  `safe/crates/gio/src/translated/mod.rs:1-237` pulls in 233 translated
  `original/gio/**` modules. All remaining unsafe matches under
  `safe/crates/gio/src/translated/original/gio/` are translated C ABI shims,
  C callback/vtable entries, or raw pointer implementations of upstream GIO
  behavior. The largest files by current unsafe-token count are:

| File | Unsafe matches | One-sentence justification |
| --- | ---: | --- |
| `crates/gio/src/translated/original/gio/gfile.rs` | 648 | Implements public `GFile` operations, file-copy paths, raw GLib objects, and POSIX file syscalls. |
| `crates/gio/src/translated/original/gio/gdesktopappinfo.rs` | 380 | Implements desktop-app metadata and launch behavior through translated raw structs, strings, and callbacks. |
| `crates/gio/src/translated/original/gio/gdbusconnection.rs` | 358 | Implements GDBus connection state, subscriptions, calls, callbacks, and raw message data. |
| `crates/gio/src/translated/original/gio/glocalfile.rs` | 353 | Implements local-file object behavior over POSIX paths, descriptors, and GLib object pointers. |
| `crates/gio/src/translated/original/gio/gresourcefile.rs` | 339 | Implements resource-backed `GFile` objects and stream vtables. |
| `crates/gio/src/translated/original/gio/gapplication.rs` | 290 | Implements `GApplication` and D-Bus application control callbacks. |
| `crates/gio/src/translated/original/gio/gfileinfo.rs` | 255 | Implements public `GFileInfo` attribute storage and raw value access. |
| `crates/gio/src/translated/original/gio/goutputstream.rs` | 244 | Implements output stream vtables, async callbacks, and raw buffer writes. |
| `crates/gio/src/translated/original/gio/gdbusintrospection.rs` | 229 | Implements D-Bus introspection parsing and raw XML-derived structures. |
| `crates/gio/src/translated/original/gio/gsettings.rs` | 220 | Implements GSettings over schemas, backends, variants, and callbacks. |
| `crates/gio/src/translated/original/gio/gsocket.rs` | 194 | Implements socket objects and their POSIX socket syscalls. |
| `crates/gio/src/translated/original/gio/gthreadedresolver.rs` | 190 | Implements DNS/name resolution over libc resolver APIs and async task callbacks. |
| `crates/gio/src/translated/original/gio/gunixmounts.rs` | 165 | Implements Unix mount APIs over libmount and raw mount table data. |
| `crates/gio/src/translated/original/gio/glocalfileinfo.rs` | 157 | Implements local file metadata over statx, xattrs, SELinux labels, and raw attribute buffers. |
| `crates/gio/src/translated/original/gio/gtask.rs` | 154 | Implements task state, async callbacks, and raw object lifetimes. |
| `crates/gio/src/translated/original/gio/gsocketclient.rs` | 146 | Implements socket client connection and proxy routing with raw async state. |
| `crates/gio/src/translated/original/gio/gbufferedinputstream.rs` | 143 | Implements buffered input stream vtables and raw buffer/skip state. |

Unsafe that is not directly required by the public GIO C ABI boundary is called
out separately: `safe/crates/gio/src/translated/compat.rs` contains translated
atomic and raw conversion helpers, `safe/crates/gio/src/generated_compat.rs`
contains ABI-shell stdio/lsan glue and portal/D-Bus placeholder exports, and
`safe/crates/gio/build.rs` contains source-scanning strings. The remaining
translated unsafe exists because the current port exposes C-compatible GIO
entry points and still implements them through translated raw-pointer code.

## Remaining unsafe FFI beyond the original ABI/API boundary

The intended boundary is the GLib-compatible GIO public C ABI: the 2107
exported symbols in `safe/abi/version-scripts/libgio.map`, the 2138 link
compatibility entries in `safe/abi/link-compat/gio.json`, and the installed
headers and helper paths in `safe/abi/installed-files.json`. That boundary is
expected for a drop-in `libgio-2.0.so.0` replacement and is not counted as
extra FFI.

The implementation also calls or exposes the following non-original-boundary
FFI surfaces:

| Surface | Symbols and evidence | Provider | Why needed now | Plausible safe replacement |
| --- | --- | --- | --- | --- |
| libc stdio globals for ABI-shell translated code | `stderr`, `stdout` at `safe/crates/gio/src/generated_compat.rs:27-29`, copied into exported statics at `safe/crates/gio/src/generated_compat.rs:139-157` | libc | Some translated modules expect C stdio globals under `safe_c2rust_*` names during ABI-shell builds. | Remove once translated modules stop depending on C global stdio shims. |
| GLib/GObject internal calls from hand compat code | `g_enum_register_static`, `g_flags_register_static`, `g_malloc0`, `g_object_ref`, `g_strdup` at `safe/crates/gio/src/generated_compat.rs:32-38` | Safe GLib/GObject libraries linked into this package | Needed to synthesize enum/flag types, portal stub out-parameters, and object references. | Keep as internal GLib-family calls or replace with safe wrappers after GObject/GType APIs are wrapped. |
| POSIX/Linux file and process APIs | Examples include `copy_file_range` in `gfile.rs:27`, `open/openat/close/readlink` in `glocalfile.rs:66-101`, `read/chown/chmod/utimensat/statx/open` in `glocalfileinfo.rs:54-227`, `open/read/write/writev` in `glocalfileoutputstream.rs:19-122`, `fork/pipe/pipe2/kill/fcntl` in `gtestdbus.rs:46-205`, and `kill/read/close/open` in `gsubprocess.rs:37-239`. | libc and Linux syscalls | GIO public behavior includes local file operations, metadata, subprocess/test-D-Bus helpers, and descriptor handling. | Gradually wrap with `std::fs`, `std::os::unix`, `nix`, or small audited safe abstractions, while preserving exact errno and GLib error mapping. |
| POSIX sockets and resolver APIs | `socket`, `bind`, `connect`, `send`, `recv`, `sendmsg`, `sendmmsg`, `recvmsg`, `recvmmsg`, `getsockopt`, `setsockopt`, `listen`, `accept`, `accept4`, and `shutdown` in `gsocket.rs:209-295`; `getaddrinfo`, `freeaddrinfo`, `gai_strerror`, and `getnameinfo` in `gthreadedresolver.rs:170-178`. | libc networking APIs | GSocket, GSocketClient, and resolver compatibility require exact POSIX socket semantics and error behavior. | A safe socket layer could wrap these calls, but public GIO behavior still needs raw fd interop and C sockaddr layouts. |
| Linux inotify | `inotify_init`, `inotify_init1`, `inotify_add_watch`, and `inotify_rm_watch` in `inotify/inotify_kernel.rs:69-76`; supporting `ioctl` and `read` at `inotify/inotify_kernel.rs:8-9`. | Linux inotify syscalls through libc | GIO local file monitors expose inotify-backed behavior. | Replace with an audited Rust inotify wrapper after matching GLib monitor semantics. |
| libmount | `mnt_get_fstab_path`, `mnt_has_regular_mtab`, `mnt_optstr_get_flags`, `mnt_new_iter`, `mnt_free_iter`, `mnt_get_builtin_optmap`, `mnt_fs_get_*`, `mnt_table_parse_*`, `mnt_table_next_fs`, and `mnt_monitor_*` at `gunixmounts.rs:197-248`. | `libmount` | Unix mount enumeration and monitor APIs are part of GIO's Unix surface. | A Rust parser/monitor for `/proc/self/mountinfo`, `/etc/fstab`, and mount notifications could replace this, but would need compatibility validation. |
| libselinux and xattrs | `is_selinux_enabled`, `freecon`, `getfilecon_raw`, `lgetfilecon_raw`, `fgetfilecon_raw`, and `setfilecon_raw` at `glocalfileinfo.rs:246-260`; xattr syscalls at `glocalfileinfo.rs:264-304`. | `libselinux` and libc/Linux xattr APIs | GIO exposes SELinux context and extended attribute metadata through `GFileInfo`. | Safe wrappers are plausible, but the platform ABI is still C and the error mapping must stay exact. |
| zlib | `deflate*` symbols in `gzlibcompressor.rs:99-109` and `inflate*` symbols in `gzlibdecompressor.rs:99-108`. | `zlib` | GZlib compressor/decompressor streams are part of the GIO API. | A pure Rust compression backend could replace this if it preserves zlib/gzip/raw behavior and ABI-facing errors. |
| MIME and resource helpers using libc file/memory APIs | xdgmime modules call `malloc`, `calloc`, `realloc`, `free`, `fopen`, `fread`, `fgets`, `open`, `mmap`, `munmap`, and related APIs, for example `xdgmimecache.rs:5-75` and `xdgmime.rs:11-124`. | libc | GIO content-type detection and MIME cache parsing still come from translated xdgmime code. | A safe Rust MIME cache/parser implementation could replace this subsystem later. |

`safe/tools/build-abi-shell.py:71-82` links `ffi`, `quadmath`, `dl`, `pthread`,
and `m` in addition to GIO's direct native libraries. Those are part of the
combined static GLib/GObject/GModule closure and Debian link contract rather
than direct GIO source calls found in `safe/crates/gio`.

No dynamically loaded third-party Rust plugin API was found in `safe/crates/gio`.
GIO module loading remains translated GIO behavior, and package installation
still writes the conventional `giomoduledir` and `gio-querymodules` paths
(`safe/tools/stage-package-tree.py:187-197`,
`safe/debian/libglib2.0-0t64.install:1-3`).

## Remaining issues

- The port is still mostly translated unsafe Rust. The hand cluster files for
  actions, application, D-Bus, file, proxy, resources, settings, sockets, and
  streams are markers only; `safe/crates/gio/src/translated/mod.rs` still owns
  the actual implementation through 233 translated modules.
- `safe/crates/gio/src/lib.rs:76-77` still reports `impl-safe-bootstrap`
  instead of `impl-gio-rust`, so the source phase marker has drifted.
- Helper tools are compatibility shims. `safe/crates/gio/src/tools.rs:17-34`
  delegates to `/usr/bin/<tool>` with `LD_LIBRARY_PATH` removed when a system
  tool exists; local fallbacks for `glib-compile-schemas`,
  `glib-compile-resources`, `gio-querymodules`, `gio-launch-desktop`, and
  `gdbus-codegen` are minimal (`safe/crates/gio/src/tools.rs:75-157`). This is
  acceptable for ABI-shell packaging tests but is not a full behavioral rewrite
  of those command-line tools.
- `safe/crates/gio/src/generated_compat.rs:486-727` contains portal
  compatibility functions that return nulls, fixed document IDs, or failure
  values for several `gxdp_*` calls. `safe/crates/gio/src/generated_compat.rs:747-822`
  contains no-op `_g_freedesktop_dbus_complete_*` exports. These are exported
  ABI placeholders and should be treated as compatibility debt.
- The translated GDBus daemon still contains an explicit
  `"UpdateActivationEnvironment not implemented"` error string at
  `safe/crates/gio/src/translated/original/gio/gdbusdaemon.rs:2892`, matching
  the current translated behavior.
- Debian packaging has an inherited patch that removes consistently failing
  upstream GIO tests `gdbus-peer` and `gdbus-address-get-session`
  (`safe/debian/patches/disable_failing_gio_tests.patch:1-31`). The phase
  manifest instead contains 133 selected GIO rows and currently includes many
  other GDBus tests such as `gdbus-addresses`, `gdbus-message`,
  `gdbus-subscribe`, `gdbus-threading`, and `gdbus-connection-flush`
  (`safe/tests/manifests/gio.txt:23-24`, `safe/tests/manifests/gio.txt:100-126`).
- The requested verifiers passed locally, but the link-compat output showed
  normal upstream skips and resource-pressure messages from `dbus-daemon`
  such as "Cannot initialize inotify: Too many open files" while later tests
  still reported `ok`. Treat D-Bus-heavy coverage as sensitive to runner file
  descriptor limits.
- `safe/docs/cve-matrix.md` marks GIO-relevant CVEs as implemented:
  CVE-2009-3289, CVE-2019-12450, CVE-2019-13012, CVE-2020-6750,
  CVE-2021-28153, and CVE-2024-34397
  (`safe/docs/cve-matrix.md:5,8-10,13,18`). The current CVE wrapper covers
  GIO file, proxy, socket-client, GDBus, and memory settings rows
  (`safe/tools/run-cve-regressions.py:59-67`) and the keyfile settings C probe
  asserts mode `0700` for the directory and `0600` for the settings file
  (`safe/tests/cve/keyfile-settings-backend.c`). No additional GIO CVE row in
  `relevant_cves.json` was found without corresponding local evidence, but the
  mitigation claim is limited to the included regression matrix.
- `dependents.json` is representative rather than exhaustive. The current list
  has 12 dependents; the GIO or GIO/GDBus consumers in it are
  `network-manager`, `bluez`, `flatpak`, `modemmanager`, `fwupd`,
  `gvfs-daemons`, `udisks2`, and `tracker-miner-fs`. This documentation does
  not claim full application-level validation for all dependent workflows.
- `cargo geiger` was not installed in this workspace (`cargo geiger -V` returned
  `error: no such command: geiger`), so the unsafe dependency assessment below
  uses Cargo metadata/tree output and source grep rather than a Geiger report.

Verification status for this refresh:

- `python3 tools/build-abi-shell.py --build-root build-gio --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gio/.stamp` completed with exit 0.
- `python3 tools/link-compat.py --phase gio --build-root build-gio --compile-original-objects --run` completed with exit 0.
- `python3 tools/run-meson-manifest.py --build-root build-gio --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gio/meson-info/intro-tests.json --manifest tests/manifests/gio.txt --print-errorlogs` completed with exit 0 and no error-log output.
- `python3 tools/run-cve-regressions.py --phase gio --build-root build-gio --rebuild` completed with exit 0.

## Dependencies and other libraries used

Direct Cargo dependencies for `safe-gio` come from
`safe/crates/gio/Cargo.toml:11-14` and were resolved by
`cargo tree -p safe-gio -e normal,build,dev`:

| Dependency | Requested version | Resolved version | Purpose |
| --- | --- | --- | --- |
| `c2rust-asm-casts` | `0.22` | `0.22.1` | Supports C2Rust translated cast/assembly patterns used by the generated runtime. |
| `c2rust-bitfields` | `0.22` | `0.22.1` | Provides translated C bitfield support and derives; pulls in `c2rust-bitfields-derive`, `proc-macro2`, `quote`, `syn`, and `unicode-ident`. |
| `libc` | `0.2` | `0.2.186` | Declares libc, POSIX, Linux, and system C ABI types/functions used by translated GIO code. |

These dependencies are unsafe-adjacent by design. They are acceptable in the
current port because the implementation is still a C ABI replacement with
mechanically translated modules, exact C layouts, raw callback signatures, and
OS-facing behavior. They should be revisited as translated subsystems are
replaced with safe Rust wrappers.

System C libraries linked for GIO are declared in the GIO build-shell config:
`z`, `mount`, `selinux`, `ffi`, `quadmath`, `dl`, `pthread`, and `m`
(`safe/tools/build-abi-shell.py:71-82`). Direct GIO source evidence for
non-GLib libraries is zlib compression/decompression
(`gzlibcompressor.rs:99-109`, `gzlibdecompressor.rs:99-108`), libmount mount
enumeration/monitoring (`gunixmounts.rs:197-248`), libselinux label handling
(`glocalfileinfo.rs:246-260`), and libc/POSIX/Linux calls across file, socket,
resolver, inotify, subprocess, D-Bus test, and xdgmime modules.

Debian build-time tools and libraries are listed in `safe/debian/control:7-32`:
`dbus-daemon`, `cargo`, `debhelper-compat (= 13)`, `dh-sequence-gnome`,
`dh-sequence-python3`, DocBook tools, `dpkg-dev`, `gettext`, `libdbus-1-dev`,
`libelf-dev`, `libffi-dev`, `libmount-dev`, `libpcre2-dev`,
`libselinux1-dev`, `libxml2-utils`, `linux-libc-dev`, `meson`, `patchelf`,
`pkgconf`, Python documentation/packaging tools, `python3`, `rustc`,
`xsltproc`, and `zlib1g-dev`. Architecture build-dependencies include
`desktop-file-utils`, optional GIR tooling, locales, Python D-Bus/GI,
`qemu-user`, `shared-mime-info`, `tzdata`, and `xterm`
(`safe/debian/control:33-42`). Development package dependencies expose
`libffi-dev`, `libmount-dev`, `libpcre2-dev`, `libselinux1-dev`, `pkgconf`,
Python or qemu, and `zlib1g-dev` to consumers
(`safe/debian/control:138-155`).

Build/package helper tools used by this phase include Cargo, rustc, Python 3,
`dpkg-architecture`, `nm`, `readelf`, `ar`, `patchelf`, `pkgconf`, Meson test
metadata consumed by `run-meson-manifest.py`, and the GIO helper binaries built
from `safe-gio`. There is no `bindgen` or `cbindgen` invocation in the GIO crate
build path; public C layouts are hand-written or translated Rust, and the final
shared object is assembled by `safe/tools/build-abi-shell.py`.

## How this document was produced

Commands run from `/home/yans/safelibs/pipeline/ports/port-glib` or its
`safe/` subdirectory:

```bash
sed -n '1,260p' safe/PORT.md
git status --short
git rev-parse --short=12 HEAD
cd safe && cargo metadata --format-version 1 --no-deps
cd safe && cargo tree -p safe-gio -e normal,build,dev
cd safe && cargo geiger -V
cd safe && rg -n "\bunsafe\b" crates/gio crates/abi-support
cd safe && rg -n "unsafe impl|unsafe extern \"C\" \{|unsafe fn|unsafe \{" crates/gio/src/lib.rs crates/gio/src/generated_compat.rs crates/gio/src/translated/compat.rs crates/gio/src/translated/gvdb.rs crates/gio/src/translated/support.rs crates/abi-support/src/ffi.rs crates/gio/build.rs
cd safe && rg -n "extern \"C\"|libc::|socket|DBus|D-Bus|proxy|settings|resource|gio-launch-desktop|glib-compile|gio-querymodules|gdbus|TODO|FIXME" crates/gio debian tests/cve tests/package abi/installed-files.json
cd safe && rg -n '^#\[path = ' crates/gio/src/translated/mod.rs | wc -l
cd safe && wc -l tests/manifests/gio.txt
cd safe && awk '/^[[:space:]]+[A-Za-z_][A-Za-z0-9_]*;$/ {count++} END {print count}' abi/version-scripts/libgio.map
cd safe && jq '.entry_ids | length' abi/link-compat/gio.json
python3 -m json.tool relevant_cves.json >/tmp/port-glib-relevant-cves-gio.json
cd safe && python3 tools/build-abi-shell.py --build-root build-gio --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gio/.stamp
cd safe && python3 tools/link-compat.py --phase gio --build-root build-gio --compile-original-objects --run
cd safe && python3 tools/run-meson-manifest.py --build-root build-gio --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gio/meson-info/intro-tests.json --manifest tests/manifests/gio.txt --print-errorlogs
cd safe && python3 tools/run-cve-regressions.py --phase gio --build-root build-gio --rebuild
```

Files consulted directly include `safe/Cargo.toml`,
`safe/crates/gio/Cargo.toml`, `safe/crates/gio/build.rs`,
`safe/crates/gio/src/lib.rs`, `safe/crates/gio/src/runtime.rs`,
`safe/crates/gio/src/exports.rs`, `safe/crates/gio/src/generated_compat.rs`,
`safe/crates/gio/src/tools.rs`, `safe/crates/gio/src/translated/mod.rs`,
`safe/crates/gio/src/translated/compat.rs`,
`safe/crates/gio/src/translated/gvdb.rs`,
`safe/crates/gio/src/translated/support.rs`,
`safe/crates/gio/src/translated/original/gio/**`,
`safe/crates/abi-support/src/ffi.rs`,
`safe/tools/build-abi-shell.py`, `safe/tools/stage-package-tree.py`,
`safe/tools/run-cve-regressions.py`, `safe/abi/version-scripts/libgio.map`,
`safe/abi/link-compat/gio.json`, `safe/abi/installed-files.json`,
`safe/tests/manifests/gio.txt`, `safe/tests/cve/keyfile-settings-backend.c`,
`safe/docs/cve-matrix.md`, `safe/debian/control`, `safe/debian/rules`,
`safe/debian/libglib2.0-0t64.install`, `safe/debian/libglib2.0-bin.install`,
`safe/debian/libglib2.0-dev-bin.install`,
`safe/debian/patches/disable_failing_gio_tests.patch`,
`dependents.json`, and `relevant_cves.json`.
