# Port GObject to a Stable Rust Runtime

## Phase Name
Port GObject to a stable Rust runtime

## Implement Phase ID
`impl-gobject-rust`

## Source Plan Context
- Overall target remains GLib 2.80.0 compatibility on Ubuntu 24.04, building on the Rust-owned GLib core and advanced surfaces from earlier phases.
- Relevant export count: `libgobject-2.0.so.0` has 478 symbols in `safe/abi/version-scripts/libgobject.map`.
- Relevant frozen manifest: `safe/tests/manifests/gobject.txt` has 62 rows.
- Current bootstrap state for this phase: `safe/crates/gobject` is mostly translated Rust under `safe/crates/gobject/src/translated/original/gobject/*.rs`; it compiles, but `cargo check --workspace` emits many warnings and the repository-wide unsafe audit still fails.
- The editable upstream mirror already contains a safe-specific `closure-refcount` timeout adjustment in `safe/tests/upstream/gobject/meson.build`; preserve it in place.

## Preexisting Inputs
- `safe/crates/glib/`
- `safe/tools/build-abi-shell.py`
- `safe/tools/run-cve-regressions.py`
- `safe/docs/cve-matrix.md`
- `safe/abi/link-compat/glib-advanced.json`
- `original/gobject/*.c`, `original/gobject/*.h`, and `original/gobject/tests/*`
- `safe/crates/gobject/src/translated/original/gobject/*.rs`
- `safe/crates/gobject/src/object/mod.rs`
- `safe/crates/gobject/src/signal/mod.rs`
- `safe/crates/gobject/src/type_system/mod.rs`
- `safe/crates/gobject/src/value/mod.rs`
- `safe/crates/gobject/build.rs`
- `safe/crates/abi-support/src/ffi.rs`
- `safe/crates/abi-support/src/bin/layout-probe.rs`
- `safe/tests/manifests/gobject.txt`

## New Outputs
- A stable Rust-owned `libgobject-2.0.so.0` / `libgobject-2.0.a`.
- Reduced warning surface and unified ABI types across translated modules.

## File Changes
- `safe/crates/gobject/src/lib.rs`
- `safe/crates/gobject/src/object/mod.rs`
- `safe/crates/gobject/src/signal/mod.rs`
- `safe/crates/gobject/src/tools/mod.rs`
- `safe/crates/gobject/src/type_system/mod.rs`
- `safe/crates/gobject/src/value/mod.rs`
- `safe/crates/gobject/src/translated/compat.rs`
- `safe/crates/gobject/src/translated/original/gobject/*.rs`
- `safe/crates/gobject/build.rs`
- `safe/abi/version-scripts/libgobject.map`
- `safe/abi/layout-manifests/gobject.json`
- `safe/abi/layouts/gobject.json`
- `safe/tests/upstream/gobject/meson.build`
- `safe/crates/abi-support/src/ffi.rs` if shared public ABI primitives must be extended
- `safe/crates/abi-support/src/bin/layout-probe.rs` if layout probing must cover newly Rust-owned public ABI types

## Implementation Details
- Consume `original/gobject/*`, `safe/tests/manifests/gobject.txt`, and the editable `safe/tests/upstream/gobject/` mirror in place.
- Keep the translated-Rust strategy as the starting point, but turn it into a coherent runtime instead of a direct file-for-file mechanical port.
- Replace duplicated opaque typedef islands and incompatible forward declarations across translated files with shared ABI structs from `safe/crates/gobject/src/object/mod.rs`, `safe/crates/gobject/src/signal/mod.rs`, `safe/crates/gobject/src/type_system/mod.rs`, and `safe/crates/gobject/src/value/mod.rs`.
- Extend `safe/crates/abi-support/src/ffi.rs` and `safe/crates/abi-support/src/bin/layout-probe.rs` only as GObject public ABI types become Rust-owned, and keep the shared definitions synchronized with `safe/abi/layout-manifests/gobject.json` and `safe/abi/layouts/gobject.json`.
- Preserve the behaviors that `safe/tools/run-meson-manifest.py` explicitly treats as required coverage for `gobject.txt`: closure ownership and refcount, signal callback ordering, and threaded and performance test stability.
- Keep `glib-genmarshal`, `glib-mkenums`, and `gobject-query` build products working because package and build tests consume them.
- Move unsafe code toward explicit ABI/FFI boundaries only; document each remaining boundary in source comments so the final unsafe audit can become strict without breaking required FFI.
- Critical file responsibilities owned by this phase:
  - `safe/crates/gobject/src/translated/original/gobject/*.rs` is the current translated GObject runtime; clean or replace this code in place.
  - `safe/crates/gobject/src/{object,signal,type_system,value}/mod.rs` define the shared runtime and ABI structures that translated files must converge on.
  - `safe/abi/version-scripts/libgobject.map`, `safe/abi/layout-manifests/gobject.json`, and `safe/abi/layouts/gobject.json` are the symbol and public-layout contracts for `libgobject-2.0`.
  - `safe/tests/upstream/gobject/meson.build` remains an editable mirror file and keeps the safe-specific `closure-refcount` timeout adjustment.

## Verification Phases
### `check-gobject-link`
- Phase ID: `check-gobject-link`
- Type: `check`
- Fixed `bounce_target`: `impl-gobject-rust`
- Purpose: Prove link-compatibility for `libgobject-2.0.so.0`.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-gobject --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gobject/.stamp
python3 tools/link-compat.py --phase gobject --build-root build-gobject --compile-original-objects --run
```

### `check-gobject-tests`
- Phase ID: `check-gobject-tests`
- Type: `check`
- Fixed `bounce_target`: `impl-gobject-rust`
- Purpose: Replay the frozen GObject manifest, including the required closure and signal regressions enforced by `run-meson-manifest.py`.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-gobject --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gobject/.stamp
python3 tools/run-meson-manifest.py --build-root build-gobject --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gobject/meson-info/intro-tests.json --manifest tests/manifests/gobject.txt --print-errorlogs
cargo check --workspace
```

## Success Criteria
- `check-gobject-link` and `check-gobject-tests` both pass.
- The large type-redeclaration warning class from the translated modules is substantially reduced or eliminated.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit containing the scoped implementation before yielding to its verification phases.
- The implementer should report the resulting commit hash to the checker.
- A verifier must treat an unchanged `HEAD`, an empty commit with no file changes, or a worktree-only deliverable as a failure.
