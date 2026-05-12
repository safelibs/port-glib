# Port GIO to Rust

## Phase Name
Port GIO to Rust

## Implement Phase ID
`impl-gio-rust`

## Source Plan Context
- Overall target remains GLib 2.80.0 compatibility on Ubuntu 24.04, now extending the already Rust-owned GLib and GObject layers to the GIO surface.
- Relevant export count: `libgio-2.0.so.0` has 2107 symbols in `safe/abi/version-scripts/libgio.map`.
- Relevant frozen manifest: `safe/tests/manifests/gio.txt` has 133 rows.
- Current bootstrap state for this phase: `safe/crates/gio/build.rs` still injects upstream GIO objects and archives from `safe/vendor/build-check/gio`, and `safe/tools/build-abi-shell.py` still has a vendored static-archive fallback for `libgio-2.0.a`.
- GIO helper executables, libexec helpers, installed file manifests, Debian install lists, and pkg-config variables are existing package artifacts to preserve and replace from `safe/` outputs, not rediscover from upstream.
- GIO security scope is already captured in `relevant_cves.json`, `safe/tests/cve/keyfile-settings-backend.c`, and `safe/tools/run-cve-regressions.py`; consume these in place.

## Preexisting Inputs
- `safe/crates/gobject/`
- `safe/abi/layout-manifests/gobject.json`
- `safe/abi/layouts/gobject.json`
- `safe/abi/version-scripts/libgobject.map`
- `safe/tests/upstream/gobject/meson.build`
- `original/gio/`
- `original/gio/tests/`
- `safe/crates/gio/build.rs`
- `safe/crates/gio/src/lib.rs`
- `safe/crates/gio/src/runtime.rs`
- `safe/crates/abi-support/src/ffi.rs`
- `safe/crates/abi-support/src/bin/layout-probe.rs`
- `safe/abi/link-compat/gio.json`
- `safe/abi/installed-files.json`
- `safe/debian/libglib2.0-0t64.install`
- `safe/debian/libglib2.0-bin.install`
- `safe/debian/libglib2.0-dev-bin.install`
- `safe/tools/stage-package-tree.py`
- `safe/tests/manifests/gio.txt`
- `safe/tests/cve/keyfile-settings-backend.c`
- `relevant_cves.json`

## New Outputs
- A Rust-owned `libgio-2.0.so.0` / `libgio-2.0.a`.
- Rust-owned build-root and package-staging GIO helper tools, libexec helpers, and pkg-config variables consistent with upstream.

## File Changes
- `safe/crates/gio/build.rs`
- `safe/crates/gio/src/lib.rs`
- `safe/crates/gio/src/runtime.rs`
- `safe/crates/gio/src/exports.rs`
- New Rust submodules under `safe/crates/gio/src/` for actions and application, file and local-file, streams and memory streams, sockets and networking, proxy resolution, resources and settings, GDBus core, and tools and command-line helpers
- `safe/crates/abi-support/src/ffi.rs` if shared public ABI primitives must be extended
- `safe/crates/abi-support/src/bin/layout-probe.rs` if layout probing must cover newly Rust-owned public ABI types
- `safe/tools/build-abi-shell.py`
- `safe/abi/version-scripts/libgio.map`
- `safe/abi/link-compat/gio.json`
- `safe/tests/upstream/gio/*` where local mirror adjustments are needed

## Implementation Details
- Consume `original/gio/*`, `safe/tests/manifests/gio.txt`, `safe/tests/upstream/gio/*`, `relevant_cves.json`, and the recorded package-install artifacts in place.
- Replace the current direct object/archive injection in `safe/crates/gio/build.rs` with Rust modules. If an intermediate compatibility backend is necessary, add it explicitly and then remove it before the phase ends; do not leave direct upstream object linkage in the final library.
- Retire the vendored static-archive fallback for `libgio-2.0.a` in `safe/tools/build-abi-shell.py`; static-link consumers must exercise the Rust implementation too.
- Port the file, stream, socket, application, settings, proxy, and D-Bus stacks in clusters that match the upstream tests and dependent runtime surface.
- Extend `safe/crates/abi-support/src/ffi.rs` and `safe/crates/abi-support/src/bin/layout-probe.rs` only as GIO public ABI types become Rust-owned, and keep any additions tied to updated GIO layout contracts.
- Treat `safe/abi/installed-files.json`, `safe/debian/libglib2.0-0t64.install`, `safe/debian/libglib2.0-bin.install`, and `safe/debian/libglib2.0-dev-bin.install` as the authoritative shipped GIO helper surface for later packaging. Phase completion requires the matching build-root artifacts to exist at the same logical paths without fallback to `safe/vendor/build-check`.
- Preserve the GIO-specific security behaviors already captured in `relevant_cves.json` and `safe/tools/run-cve-regressions.py`.
- Secure `g_file_copy()` and replace-destination behavior.
- Private-permission `GKeyfileSettingsBackend`.
- No direct-connect fallback once proxy routing is selected.
- Trusted-sender validation for `g_dbus_connection_signal_subscribe()`.
- Keep the shipped helper executables, libexec helpers, and pkg-config variables working: `gapplication`, `gio`, `gdbus`, `gsettings`, `gresource`, `gio-launch-desktop`, `glib-compile-schemas`, `glib-compile-resources`, `gio-querymodules`, and `gdbus-codegen` plus its installed `gdbus-2.0/codegen/*.py` support modules.
- Do not leave the shipped helper executable or libexec-helper paths copied verbatim from `safe/vendor/build-check`; by the end of the phase the build-root paths later consumed by packaging must be rebuilt or replaced from `safe/` outputs.
- Critical file responsibilities owned by this phase:
  - `safe/crates/gio/build.rs`, `safe/crates/gio/src/lib.rs`, and `safe/crates/gio/src/runtime.rs` are the current GIO bootstrap surface that still depends on vendored original objects.
  - New modules under `safe/crates/gio/src/` must turn GIO into a real Rust crate while preserving the 2107-symbol public surface recorded in `safe/abi/version-scripts/libgio.map`.
  - `safe/abi/link-compat/gio.json` and `safe/tests/manifests/gio.txt` are updated only in place as GIO coverage becomes Rust-owned.
  - `safe/abi/installed-files.json` and the `safe/debian/libglib2.0-*.install` files are authoritative for the shipped GIO helper and libexec surface consumed by packaging.

## Verification Phases
### `check-gio-link`
- Phase ID: `check-gio-link`
- Type: `check`
- Fixed `bounce_target`: `impl-gio-rust`
- Purpose: Prove that the full `libgio-2.0.so.0` public surface still links and runs from original objects.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-gio --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gio/.stamp
python3 tools/link-compat.py --phase gio --build-root build-gio --compile-original-objects --run
```

### `check-gio-tests`
- Phase ID: `check-gio-tests`
- Type: `check`
- Fixed `bounce_target`: `impl-gio-rust`
- Purpose: Replay the frozen GIO manifest against the safe build.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-gio --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gio/.stamp
python3 tools/run-meson-manifest.py --build-root build-gio --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gio/meson-info/intro-tests.json --manifest tests/manifests/gio.txt --print-errorlogs
```

### `check-gio-cves`
- Phase ID: `check-gio-cves`
- Type: `check`
- Fixed `bounce_target`: `impl-gio-rust`
- Purpose: Re-run the GIO CVE probes and targeted manifest rows for file copy, settings backend, proxy routing, and sender validation.
- Commands:
```bash
cd safe
python3 tools/run-cve-regressions.py --phase gio --build-root build-gio --rebuild
```

## Success Criteria
- `check-gio-link`, `check-gio-tests`, and `check-gio-cves` all pass.
- `safe/crates/gio/build.rs` no longer injects `safe/vendor/build-check/gio/*.o` or the vendored `xdgmime` / `inotify` archives into the final cdylib.
- `safe/tools/build-abi-shell.py` no longer assembles `libgio-2.0.a` from vendored objects or leaves vendored GIO helper executables/libexec helpers at the shipped `gapplication`, `gio`, `gdbus`, `gsettings`, `gresource`, `gio-launch-desktop`, `glib-compile-schemas`, `glib-compile-resources`, `gio-querymodules`, or `gdbus-codegen` paths.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit containing the scoped implementation before yielding to its verification phases.
- The implementer should report the resulting commit hash to the checker.
- A verifier must treat an unchanged `HEAD`, an empty commit with no file changes, or a worktree-only deliverable as a failure.
