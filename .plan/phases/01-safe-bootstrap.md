# Bootstrap the ABI Shell and Artifact Contract

## Phase Name
Bootstrap the ABI shell and artifact contract

## Implement Phase ID
`impl-safe-bootstrap`

## Source Plan Context
- Overall target: turn upstream GLib 2.80.0 in `original/` into a Rust implementation in `safe/` that is source-compatible, link-compatible, runtime-compatible, and package-compatible on Ubuntu 24.04.
- Current bootstrap state: `safe/` already has a Cargo workspace for `glib`, `gthread`, `gmodule`, `gobject`, `gio`, `girepository`, and `abi-support`; a Meson bootstrap wrapper; a Debian package build path; frozen upstream/build artifacts; ABI, symbol, layout, package, and test manifests; editable upstream test mirrors; CVE probes; and package smoke tests.
- Current Rust ownership is partial. `safe/crates/glib` already owns selected CVE-sensitive exports, `safe/tools/build-glib-backend.py` rewrites 96 frozen GLib compile commands into a replayed backend, `gthread`, `gmodule`, and `gio` still inject upstream objects, and `safe/tools/build-abi-shell.py` still seeds from `safe/vendor/build-check` and fabricates static archives for several libraries.
- GObject is mostly translated Rust but still warning-heavy and not ready for the final unsafe audit. GIRepository currently has 229 placeholder exports plus 2 real exports. `test-original.sh` is still an original-GLib harness rather than a safe-package harness.
- Upstream export counts from `safe/abi/version-scripts/*.map`: `libglib-2.0.so.0` has 1872 symbols, `libgthread-2.0.so.0` has 2, `libgmodule-2.0.so.0` has 10, `libgobject-2.0.so.0` has 478, `libgio-2.0.so.0` has 2107, and `libgirepository-2.0.so.0` has 231.
- Frozen test manifest counts in `safe/tests/manifests/`: `glib-core.txt` has 83 rows, `glib-advanced.txt` has 68, `gobject.txt` has 62, `gio.txt` has 133, `girepository.txt` has 11, `fuzzing.txt` has 22, and `full.txt` has 384.
- The existing hybrid bootstrap is an oracle and compatibility scaffold only. The final implementation must retire direct linkage to upstream object files, vendored static-archive construction, and shipped helper/tool artifacts copied from `safe/vendor/build-check`.

## Preexisting Inputs
- `original/`
- `dependents.json`
- `relevant_cves.json`
- `safe/Cargo.toml`
- `safe/meson.build`
- `safe/debian/rules`
- `safe/debian/`
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
- `safe/docs/cve-matrix.md`
- `safe/docs/debian-patch-provenance.md`
- `test-original.sh`

## New Outputs
- A verified, stable baseline for all later phases.
- An explicit editable-mirror contract for `safe/tests/upstream/*`.
- Refreshed bootstrap manifests only if the contract changes in place.

## File Changes
- `safe/tools/sync-upstream-assets.py`
- `safe/abi/test-source-path-map.json`
- `safe/tests/upstream/glib/meson.build`
- `safe/tests/upstream/gobject/meson.build`
- `safe/tests/upstream/gmodule/meson.build`
- `safe/tests/upstream/gthread/meson.build`
- `safe/tests/upstream/glib/markup-collect.c`
- Any `safe/abi/*` manifest whose current meaning changes because of the editable-mirror contract

## Implementation Details
- Generated workflow contract for the whole phase set:
  - Keep execution linear; do not introduce `parallel_groups`.
  - Emit inline YAML only; do not use top-level `include` or phase-level `prompt_file`, `workflow_file`, `workflow_dir`, `checks`, or any other YAML-source indirection.
  - Model every verifier as an explicit top-level `check` phase with exactly one fixed `bounce_target`.
  - Keep each verifier immediately in the implementation block it verifies, and bounce only to that implement phase.
  - Put every verifier command directly in that checker's instructions; do not model command execution as separate non-agentic phases.
  - Keep each major implementation phase as one implement phase followed only by the listed check phases.
  - Require every implement phase to commit at least once before yielding; checkers must fail unchanged `HEAD` or worktree-only deliverables.
- Preserve `safe/vendor/original/` and `safe/vendor/build-check/` as authoritative read-only inputs.
- Preserve the consume-existing-artifacts contract for `original/`, `dependents.json`, `relevant_cves.json`, `safe/vendor/original/`, `safe/vendor/build-check/`, `safe/abi/`, `safe/tests/manifests/`, `safe/tests/upstream/`, `safe/debian/`, `safe/docs/cve-matrix.md`, `safe/docs/debian-patch-provenance.md`, and `test-original.sh`.
- Fix the current mismatch where `python3 tools/sync-upstream-assets.py --verify-map abi/test-source-path-map.json` fails because some editable mirrors intentionally diverge from the vendored originals.
- Change the mirror-verification contract so it verifies the path-map shape remains fixed, the editable trees still correspond to the vendored source roots, and intentional local edits are explicit and preserved in place.
- Keep the current local mirror edits that already encode safe-specific test expectations.
- `safe/tests/upstream/glib/markup-collect.c` uses `"GLib"` as the expected critical-log domain instead of `G_LOG_DOMAIN`.
- `safe/tests/upstream/gobject/meson.build` already raises the `closure-refcount` timeout.
- `safe/tests/upstream/*/meson.build` files already document that these trees are canonical editable mirrors for replay.
- Remove `__pycache__` noise from the editable-mirror contract; these should not control the plan.
- Do not widen or regenerate the upstream artifact set. Later phases must consume the current `safe/abi/*` and `safe/tests/manifests/*` files in place.
- Critical file responsibilities owned by this phase:
  - `safe/Cargo.toml` remains the workspace membership and crate-graph source for all Rust libraries.
  - `safe/meson.build` remains the Meson-side bootstrap entry point for Cargo and ABI-shell builds.
  - `safe/tools/extract_abi.py`, `safe/tools/extract_layouts.py`, `safe/tools/link-compat.py`, and `safe/tools/run-meson-manifest.py` remain the canonical ABI, layout, link/run, and upstream-test verification tools.
  - `safe/tools/sync-upstream-assets.py` becomes the editable-mirror contract verifier instead of a byte-for-byte resync tool.
  - `safe/abi/tests.json`, `safe/abi/test-source-path-map.json`, `safe/abi/link-compat/*.json`, `safe/abi/layout-manifests/*.json`, and `safe/tests/manifests/*.txt` are consumed and updated only in place.
  - `safe/tests/upstream/*` remain canonical editable copies of upstream tests used by manifest-driven replay.

## Verification Phases
### `check-safe-bootstrap-metadata`
- Phase ID: `check-safe-bootstrap-metadata`
- Type: `check`
- Fixed `bounce_target`: `impl-safe-bootstrap`
- Purpose: Verify the frozen metadata, phase manifests, and editable-mirror contract before deeper porting begins.
- Commands:
```bash
cd safe
cargo check --workspace
python3 tools/extract_abi.py --verify
python3 tools/extract_layouts.py --verify
python3 tools/link-compat.py --verify-manifests
python3 tools/verify-debian-patches.py --verify-manifest
```

### `check-safe-bootstrap-abi-shell`
- Phase ID: `check-safe-bootstrap-abi-shell`
- Type: `check`
- Fixed `bounce_target`: `impl-safe-bootstrap`
- Purpose: Verify that the current Rust workspace still builds an ABI shell that links original objects and packages correctly.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-abi-shell --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-abi-shell/.stamp
python3 tools/link-compat.py --phase abi-shell --build-root build-abi-shell --compile-original-objects --run
dpkg-buildpackage -b -uc -us
python3 tools/verify-package-baselines.py --source . --work-root build-package-baselines --abi-shell-profiles "nodoc noinsttest nogir noudeb" --install-manifests abi/install-manifests --postinst-manifest abi/postinst-state/runtime.json
```

## Success Criteria
- `check-safe-bootstrap-metadata` and `check-safe-bootstrap-abi-shell` both pass.
- No later phase depends on a full resync of `safe/tests/upstream/*`.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit containing the scoped implementation before yielding to its verification phases.
- The implementer should report the resulting commit hash to the checker.
- A verifier must treat an unchanged `HEAD`, an empty commit with no file changes, or a worktree-only deliverable as a failure.
