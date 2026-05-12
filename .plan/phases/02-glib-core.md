# Port Core GLib, GThread, and GModule

## Phase Name
Port core GLib, GThread, and GModule

## Implement Phase ID
`impl-glib-core`

## Source Plan Context
- Overall target remains GLib 2.80.0 compatibility on Ubuntu 24.04 using the existing `original/`, `safe/vendor/original/`, `safe/vendor/build-check/`, `safe/abi/`, `safe/tests/manifests/`, and editable `safe/tests/upstream/` artifacts in place.
- Relevant export counts: `libglib-2.0.so.0` has 1872 symbols, `libgthread-2.0.so.0` has 2, and `libgmodule-2.0.so.0` has 10.
- Relevant frozen manifest: `safe/tests/manifests/glib-core.txt` has 83 rows.
- Current bootstrap state for this phase: `safe/tools/build-glib-backend.py` still rewrites 96 frozen GLib compile commands, and `safe/crates/gthread/build.rs` plus `safe/crates/gmodule/build.rs` still inject upstream object files from `safe/vendor/build-check`.
- This phase consumes the phase 1 artifact contract as an existing baseline and moves the core surface toward Rust ownership without rediscovering or regenerating upstream snapshots or frozen metadata.

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/meson.build`
- `safe/debian/rules`
- `safe/vendor/original/`
- `safe/vendor/build-check/`
- `safe/abi/tests.json`
- `safe/abi/test-source-path-map.json`
- `safe/abi/link-compat/*.json`
- `safe/abi/layout-manifests/*.json`
- `safe/abi/install-manifests/*.json`
- `safe/abi/debian-patches.json`
- `safe/tests/upstream/*`
- `safe/tests/manifests/*`
- `original/glib/*.c`, `original/glib/*.h`, and `original/glib/tests/*`
- `original/gthread/gthread-impl.c`
- `original/gmodule/gmodule.c`, `original/gmodule/gmodule-deprecated.c`, `original/gmodule/gmodule.h`
- `safe/abi/link-compat/glib-core.json`
- `safe/tests/manifests/glib-core.txt`
- `safe/abi/layout-manifests/glib.json`
- `safe/abi/layout-manifests/gthread.json`
- `safe/abi/layout-manifests/gmodule.json`
- `safe/crates/abi-support/src/ffi.rs`
- `safe/crates/abi-support/src/bin/layout-probe.rs`

## New Outputs
- Pure-Rust `libgthread-2.0.so.0` / `libgthread-2.0.a` and `libgmodule-2.0.so.0` / `libgmodule-2.0.a`.
- A greatly expanded Rust-owned `libglib-2.0.so.0` / `libglib-2.0.a` core surface.
- Core ABI/layout parity verified from the safe build root.

## File Changes
- `safe/crates/glib/src/lib.rs`
- `safe/crates/glib/src/base/mod.rs`
- `safe/crates/glib/src/collections/mod.rs`
- `safe/crates/glib/src/mainloop/mod.rs`
- `safe/crates/glib/src/strings/mod.rs`
- `safe/crates/glib/src/threading/mod.rs`
- `safe/crates/glib/build.rs`
- `safe/crates/glib/src/backend.rs`
- `safe/crates/gthread/build.rs`
- `safe/crates/gthread/src/lib.rs`
- `safe/crates/gthread/src/runtime.rs`
- `safe/crates/gthread/src/compat.rs`
- `safe/crates/gmodule/build.rs`
- `safe/crates/gmodule/src/lib.rs`
- `safe/crates/gmodule/src/module_api.rs`
- `safe/crates/gmodule/src/runtime.rs`
- `safe/tools/build-abi-shell.py`
- `safe/tools/build-glib-backend.py`
- `safe/abi/version-scripts/libglib.map`
- `safe/abi/version-scripts/libgthread.map`
- `safe/abi/version-scripts/libgmodule.map`
- `safe/abi/link-compat/glib-core.json`
- `safe/abi/layouts/{glib,gthread,gmodule}.json` if layout baselines must expand
- `safe/crates/abi-support/src/ffi.rs` if shared public ABI primitives must be extended
- `safe/crates/abi-support/src/bin/layout-probe.rs` if layout probing must cover newly Rust-owned public ABI types

## Implementation Details
- Consume the existing `original/`, `safe/vendor/original/`, `safe/vendor/build-check/`, `safe/abi/`, `safe/tests/manifests/`, and `safe/tests/upstream/` artifacts in place; do not refetch or regenerate them.
- Replace the direct vendored-object strategy in `safe/crates/gthread/build.rs` and `safe/crates/gmodule/build.rs` with normal Rust-owned exports that preserve the upstream ABI and public symbol list.
- Replace the vendored static-archive fallback in `safe/tools/build-abi-shell.py` for `gthread` and `gmodule`; phase completion requires `build-glib-core/gthread/libgthread-2.0.a` and `build-glib-core/gmodule/libgmodule-2.0.a` to come from Cargo/Rust outputs, not `safe/vendor/build-check`.
- Port the data-structure and runtime surfaces exercised by `tests/manifests/glib-core.txt`: arrays, bytes, lists, queues, strings, async queues, atomics, rcbox/refcount, threading helpers, main-loop primitives, timers, and the core utility surface.
- Keep the public layouts in `safe/abi/layout-manifests/{glib,gthread,gmodule}.json` exact; expand the manifests if additional public types become Rust-owned.
- Extend `safe/crates/abi-support/src/ffi.rs` and `safe/crates/abi-support/src/bin/layout-probe.rs` only when core GLib, GThread, or GModule public ABI types become Rust-owned, and keep those definitions synchronized with the layout manifests.
- Use the current `safe-glib` backend-replay machinery only as an incremental scaffold. The phase should move the core surface out of replayed C objects and into Rust code.
- Keep the generated-export and version-script contracts stable so that link-compat rows in `safe/abi/link-compat/glib-core.json` continue to point at the same symbol names.
- Preserve the upstream-installed helper binaries and pkg-config behavior needed by later phases.
- Critical file responsibilities owned by this phase:
  - `safe/tools/build-abi-shell.py` remains the central build-root orchestrator for staged libraries, pkg-config files, helper binaries, intro-test inventory, and layout exports; this phase retires its `gthread` and `gmodule` vendored static-archive fallbacks.
  - `safe/abi/version-scripts/libglib.map`, `safe/abi/version-scripts/libgthread.map`, and `safe/abi/version-scripts/libgmodule.map` are the authoritative exported-symbol lists for this core surface.
  - `safe/crates/abi-support/src/ffi.rs` and `safe/crates/abi-support/src/bin/layout-probe.rs` hold shared ABI primitives and layout probing; extend them only with public ABI types that become Rust-owned.
  - `safe/crates/glib/src/lib.rs` remains the top-level GLib module map for core and later advanced GLib modules.
  - `safe/crates/gthread/*` and `safe/crates/gmodule/*` are small but critical libraries whose current build paths must stop injecting upstream objects.

## Verification Phases
### `check-glib-core-link`
- Phase ID: `check-glib-core-link`
- Type: `check`
- Fixed `bounce_target`: `impl-glib-core`
- Purpose: Prove that original objects compiled against the upstream public headers still link and run against the safe core libraries.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-glib-core --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-glib-core/.stamp
python3 tools/link-compat.py --phase glib-core --build-root build-glib-core --compile-original-objects --run
```

### `check-glib-core-tests`
- Phase ID: `check-glib-core-tests`
- Type: `check`
- Fixed `bounce_target`: `impl-glib-core`
- Purpose: Replay the frozen core upstream tests against the safe build.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-glib-core --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-glib-core/.stamp
python3 tools/run-meson-manifest.py --build-root build-glib-core --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-glib-core/meson-info/intro-tests.json --manifest tests/manifests/glib-core.txt --print-errorlogs
```

## Success Criteria
- `check-glib-core-link` and `check-glib-core-tests` both pass.
- `safe/crates/gthread/build.rs` and `safe/crates/gmodule/build.rs` no longer add upstream `.o` files from `safe/vendor/build-check`.
- `safe/tools/build-abi-shell.py` no longer builds `libgthread-2.0.a` or `libgmodule-2.0.a` from vendored object directories.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit containing the scoped implementation before yielding to its verification phases.
- The implementer should report the resulting commit hash to the checker.
- A verifier must treat an unchanged `HEAD`, an empty commit with no file changes, or a worktree-only deliverable as a failure.
