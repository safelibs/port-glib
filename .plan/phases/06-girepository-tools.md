# Port GIRepository and the `gi-*` Tools

## Phase Name
Port GIRepository and the `gi-*` tools

## Implement Phase ID
`impl-girepository-rust`

## Source Plan Context
- Overall target remains GLib 2.80.0 compatibility on Ubuntu 24.04, with GIRepository becoming a real Rust library and toolchain after GIO is Rust-owned.
- Relevant export count: `libgirepository-2.0.so.0` has 231 symbols in `safe/abi/version-scripts/libgirepository.map`.
- Relevant frozen manifest: `safe/tests/manifests/girepository.txt` has 11 rows.
- Current bootstrap state for this phase: `safe/crates/girepository/src/exports.rs` exports 229 placeholder symbols plus 2 real exports, and `safe/crates/girepository/src/runtime.rs` contains only minimal real runtime functions.
- `safe/tests/package/girepository-compile-only.sh` and `safe/tests/package/girepository-installed.sh` are existing package-facing acceptance specs. This phase satisfies their interface assumptions, while phase 7 executes them after installing safe packages.

## Preexisting Inputs
- `safe/crates/gio/`
- `safe/tools/build-abi-shell.py`
- `safe/tools/stage-package-tree.py`
- `safe/abi/link-compat/gio.json`
- `safe/abi/installed-files.json`
- `safe/debian/libglib2.0-0t64.install`
- `safe/debian/libglib2.0-bin.install`
- `safe/debian/libglib2.0-dev-bin.install`
- `original/girepository/*.c`, `original/girepository/*.h`, and `original/girepository/tests/*`
- `safe/crates/girepository/src/exports.rs`
- `safe/crates/girepository/src/runtime.rs`
- `safe/crates/girepository/src/repository/mod.rs`
- `safe/crates/girepository/src/parser/mod.rs`
- `safe/crates/girepository/src/invoke/mod.rs`
- `safe/crates/girepository/src/tools/mod.rs`
- `safe/crates/abi-support/src/ffi.rs`
- `safe/crates/abi-support/src/bin/layout-probe.rs`
- `safe/tests/manifests/girepository.txt`
- `safe/tests/package/girepository-compile-only.sh`
- `safe/tests/package/girepository-installed.sh`

## New Outputs
- A real Rust `libgirepository-2.0.so.0` / `libgirepository-2.0.a`.
- Working build-root `gi-compile-repository`, `gi-decompile-typelib`, and `gi-inspect-typelib`.
- GIR and typelib artifacts that remain consistent with the compile-only and installed-package consumers, with installed-package execution of those scripts completed in phase 7.

## File Changes
- `safe/crates/girepository/build.rs`
- `safe/crates/girepository/src/lib.rs`
- `safe/crates/girepository/src/exports.rs`
- `safe/crates/girepository/src/runtime.rs`
- `safe/crates/girepository/src/repository/mod.rs`
- `safe/crates/girepository/src/parser/mod.rs`
- `safe/crates/girepository/src/invoke/mod.rs`
- `safe/crates/girepository/src/tools/mod.rs`
- New Rust submodules under `safe/crates/girepository/src/` for typelib parsing, repository queries, and invoke support
- `safe/crates/abi-support/src/ffi.rs` if shared public ABI primitives must be extended
- `safe/crates/abi-support/src/bin/layout-probe.rs` if layout probing must cover newly Rust-owned public ABI types
- `safe/tools/build-abi-shell.py`
- `safe/tools/stage-package-tree.py`
- `safe/abi/version-scripts/libgirepository.map`
- `safe/abi/link-compat/girepository.json`

## Implementation Details
- Consume `original/girepository/*`, `safe/tests/manifests/girepository.txt`, and the package-facing GIRepository test scripts in place.
- Replace the placeholder export wall in `safe/crates/girepository/src/exports.rs` with real Rust functions. The current state is symbol-complete but runtime-incomplete; the final phase output must remove `PLACEHOLDER_*` entirely.
- Replace the vendored static-archive fallback for `libgirepository-2.0.a` in `safe/tools/build-abi-shell.py`; the static archive shipped to developers must exercise the safe implementation.
- Implement the repository loader, typelib parser/validator, callable info queries, type-info helpers, and invoke routines used by `tests/manifests/girepository.txt`, the package consumer in `safe/tests/package/girepository-consumer.c`, and the installed tool smoke tests that phase 7 runs through `safe/tests/package/girepository-installed.sh`.
- Extend `safe/crates/abi-support/src/ffi.rs` and `safe/crates/abi-support/src/bin/layout-probe.rs` only as GIRepository public ABI types become Rust-owned, and keep those definitions synchronized with the GIRepository layout and symbol contracts.
- Preserve the installed artifact layout already described in `safe/crates/girepository/src/repository/mod.rs`, `safe/crates/girepository/src/parser/mod.rs`, and `safe/crates/girepository/src/tools/mod.rs`.
- Keep the Debian cross-wrapper contract for `gi-*` tools intact, because `safe/debian/rules` already synthesizes cross-prefixed wrappers for them.
- Do not leave the build-root `gi-compile-repository`, `gi-decompile-typelib`, or `gi-inspect-typelib` paths copied from `safe/vendor/build-check`; phase completion requires those shipped tool paths to come from `safe/` outputs.
- Treat `safe/tests/package/girepository-compile-only.sh` and `safe/tests/package/girepository-installed.sh` as the package-facing acceptance specs. Phase 6 should satisfy their interface assumptions, but phase 7 is where those exact scripts are executed after package installation.
- Critical file responsibilities owned by this phase:
  - `safe/crates/girepository/src/exports.rs` is the current placeholder export wall; replace every `PLACEHOLDER_` export with real implementation.
  - `safe/crates/girepository/src/{runtime.rs,repository/mod.rs,parser/mod.rs,invoke/mod.rs,tools/mod.rs}` is the current minimal metadata/runtime surface that must become a real library and toolchain.
  - `safe/abi/version-scripts/libgirepository.map` and `safe/abi/link-compat/girepository.json` are the symbol and link/run contracts for `libgirepository-2.0`.
  - `safe/tests/package/girepository-compile-only.sh` and `safe/tests/package/girepository-installed.sh` are package-level acceptance specs that must remain compatible for phase 7 installed-package execution.

## Verification Phases
### `check-girepository-link`
- Phase ID: `check-girepository-link`
- Type: `check`
- Fixed `bounce_target`: `impl-girepository-rust`
- Purpose: Prove that original object files compiled against the installed GIRepository headers still link and run.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-girepository --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-girepository/.stamp
python3 tools/link-compat.py --phase girepository --build-root build-girepository --compile-original-objects --run
```

### `check-girepository-tests`
- Phase ID: `check-girepository-tests`
- Type: `check`
- Fixed `bounce_target`: `impl-girepository-rust`
- Purpose: Replay the frozen GIRepository manifest and verify the build-root library surface; installed-package tool execution is deferred to phase 7.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-girepository --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-girepository/.stamp
python3 tools/run-meson-manifest.py --build-root build-girepository --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-girepository/meson-info/intro-tests.json --manifest tests/manifests/girepository.txt --print-errorlogs
cargo check --workspace
! rg -n "PLACEHOLDER_" crates/girepository/src/exports.rs
```

## Success Criteria
- `check-girepository-link` and `check-girepository-tests` both pass.
- There are no `PLACEHOLDER_` exports left in `safe/crates/girepository/src/exports.rs`.
- `safe/tools/build-abi-shell.py` no longer sources `libgirepository-2.0.a` or the shipped `gi-*` tool paths from `safe/vendor/build-check`.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit before yielding to its verification phases.
- A verifier must treat an unchanged `HEAD` or a worktree-only deliverable as a failure.
