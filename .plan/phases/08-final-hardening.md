# Final Hardening, Unsafe Reduction, and Full Verification

## Phase Name
Final hardening, unsafe reduction, and full verification

## Implement Phase ID
`impl-final-fixups`

## Source Plan Context
- Overall target is the completed GLib 2.80.0 Rust implementation in `safe/`, source-compatible, link-compatible, runtime-compatible, and package-compatible on Ubuntu 24.04.
- Final export counts remain the authoritative target: `libglib-2.0.so.0` 1872 symbols, `libgthread-2.0.so.0` 2, `libgmodule-2.0.so.0` 10, `libgobject-2.0.so.0` 478, `libgio-2.0.so.0` 2107, and `libgirepository-2.0.so.0` 231.
- Final frozen manifest target: `safe/tests/manifests/full.txt` has 384 rows, and the final CVE run covers all entries represented by `relevant_cves.json` and `safe/tests/cve/*`.
- This phase is the catch-all for residual interoperability, package, and safety breakage only after phases 1-7 have moved shipped runtime, static archives, helper executables, and tools away from `safe/vendor/build-check`.
- The final implementation is incomplete if any shipped library still depends on linked upstream object files, any shipped static archive or helper executable still comes from `safe/vendor/build-check`, any GIRepository placeholder export remains, safe packages fail the package/debian/dependent harness scopes, or avoidable unsafe code remains.

## Preexisting Inputs
- `test-original.sh`
- `original/`
- `safe/debian/`
- `safe/vendor/original/`
- `safe/vendor/build-check/`
- `safe/abi/install-manifests/*.json`
- `safe/abi/installed-files.json`
- `safe/abi/postinst-state/runtime.json`
- `safe/abi/debian-control-preservation.json`
- `safe/abi/tests.json`
- `safe/abi/test-source-path-map.json`
- `safe/abi/link-compat/*.json`
- `safe/abi/layout-manifests/*.json`
- `safe/abi/layouts/*.json`
- `safe/abi/symbols/*.symbols`
- `safe/abi/version-scripts/*.map`
- `safe/abi/debian-patches.json`
- `safe/tests/upstream/`
- `safe/tests/manifests/full.txt`
- `safe/tests/cve/*`
- `safe/tests/package/*`
- `relevant_cves.json`
- `dependents.json`
- `safe/crates/`
- `safe/Cargo.toml`
- `safe/tools/build-abi-shell.py`
- `safe/tools/check-unsafe-audit.py`
- `safe/tools/compare-layouts.py`
- `safe/tools/compare-symbols.py`
- `safe/tools/stage-package-tree.py`
- `safe/tools/compare-debian-control.py`
- `safe/tools/verify-package-baselines.py`
- `safe/tools/extract_abi.py`
- `safe/tools/extract_layouts.py`
- `safe/tools/link-compat.py`
- `safe/tools/run-meson-manifest.py`
- `safe/tools/run-cve-regressions.py`
- `safe/tools/verify-debian-patches.py`
- `safe/docs/cve-matrix.md`
- `safe/docs/debian-patch-provenance.md`

## New Outputs
- Final release-ready safe workspace and package set.
- Final safety audit contract.
- Final docs synchronized with the code and packaging state.

## File Changes
- `safe/tools/check-unsafe-audit.py`
- `safe/docs/cve-matrix.md`
- `safe/docs/debian-patch-provenance.md`
- Targeted residual fixes in the minimum earlier-phase owner file that still violates `check-final-full` or `check-final-safety`; do not reopen earlier phase scope wholesale.

## Implementation Details
- Remove or justify every remaining `unsafe` block. The final audit should allow only required FFI/OS boundary unsafe code and should fail on undocumented or avoidable unsafe.
- Ensure no crate `build.rs` still injects or references vendored payloads from `safe/vendor/build-check` or the GLib backend replay tool in the final cdylibs.
- Ensure no shipped static archive or installed helper executable is still copied from `safe/vendor/build-check`; this explicitly includes `safe/tools/stage-package-tree.py`, whose package payload resolution must come from the current build root or source tree instead. Keeping upstream script-template packaging from `safe/vendor/original/` is acceptable where the shipped interface is already Python or shell.
- Eliminate any remaining placeholder exports, dead bootstrap forwarders, or test-only compatibility shims that would make the installed package depend on the original C implementation.
- Confirm `safe/crates/abi-support/src/ffi.rs` and `safe/crates/abi-support/src/bin/layout-probe.rs` were extended only alongside public ABI types that became Rust-owned, and that final layout verification exercises those shared definitions.
- Refresh the CVE matrix and Debian patch provenance documentation to describe the final Rust-owned behavior, not the bootstrap shell.
- Final verification contract owned by this phase:
  - Metadata and contract verification must run `cargo check --workspace`, ABI extraction verification, layout extraction verification, link-compat manifest verification, and Debian patch manifest verification.
  - Build-root verification must build `build-final`, run the `full` link-compat phase, replay `tests/manifests/full.txt`, run all CVE regressions, and compare all recorded public layouts.
  - Exported-symbol verification must compare every final library in `build-final` against `safe/abi/symbols/*.symbols`.
  - Package verification must build the Debian packages and run the control-preservation and package-baseline checks.
  - Installed-package verification must run `GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=all ./test-original.sh` from the repository root, represented in the checker as `../test-original.sh` after `cd safe`.
  - Final cleanup verification must reject remaining placeholder exports, vendored build-check references in crate build scripts or package staging, GLib backend replay linkage, vendored static-archive fallback keys in `tools/build-abi-shell.py`, and avoidable unsafe code.
- Critical file responsibilities owned by this phase:
  - `safe/tools/check-unsafe-audit.py` is the final safety gate and must allow only justified FFI/OS-boundary unsafe code.
  - `safe/abi/symbols/*.symbols` are the Debian symbol files and final symbol-surface verification baselines.
  - `safe/docs/debian-patch-provenance.md` records how Debian/Ubuntu patch behavior is preserved or absorbed into the safe source.

## Verification Phases
### `check-final-full`
- Phase ID: `check-final-full`
- Type: `check`
- Fixed `bounce_target`: `impl-final-fixups`
- Purpose: Run the full metadata, build-root interoperability, CVE, package, and installed-package harness checks against one final build root.
- Commands:
```bash
cd safe
cargo check --workspace
python3 tools/extract_abi.py --verify
python3 tools/extract_layouts.py --verify
python3 tools/link-compat.py --verify-manifests
python3 tools/verify-debian-patches.py --verify-manifest
python3 tools/build-abi-shell.py --build-root build-final --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-final/.stamp
python3 tools/link-compat.py --phase full --build-root build-final --compile-original-objects --run
python3 tools/run-meson-manifest.py --build-root build-final --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-final/meson-info/intro-tests.json --manifest tests/manifests/full.txt --print-errorlogs
python3 tools/run-cve-regressions.py --all --build-root build-final --rebuild
dpkg-buildpackage -b -uc -us
python3 tools/compare-debian-control.py --baseline abi/debian-control-preservation.json --control debian/control
python3 tools/verify-package-baselines.py --source . --work-root build-package-baselines-final --abi-shell-profiles "nodoc noinsttest nogir noudeb" --install-manifests abi/install-manifests --postinst-manifest abi/postinst-state/runtime.json
GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=all ../test-original.sh
```

### `check-final-safety`
- Phase ID: `check-final-safety`
- Type: `check`
- Fixed `bounce_target`: `impl-final-fixups`
- Purpose: Enforce the final safety, symbol/layout parity, and “no bootstrap leftovers” contract.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-final --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-final/.stamp
python3 tools/check-unsafe-audit.py
! rg -n "PLACEHOLDER_" crates/girepository/src/exports.rs
! rg -n "vendor/build-check" crates/*/build.rs
! rg -n "build-glib-backend.py" crates/glib/build.rs
! rg -n "SAFE_VENDOR_BUILD_CHECK|vendor/build-check" tools/stage-package-tree.py
! rg -n "static_objects|static_archives|build_vendored_static_archive" tools/build-abi-shell.py
python3 tools/compare-layouts.py --build-root build-final --baseline abi/layouts/glib.json --baseline abi/layouts/gthread.json --baseline abi/layouts/gmodule.json --baseline abi/layouts/gobject.json --baseline abi/layouts/gio.json --baseline abi/layouts/girepository.json
for pair in \
  "abi/symbols/libglib-2.0.so.0.symbols build-final/glib/libglib-2.0.so.0.8000.0" \
  "abi/symbols/libgthread-2.0.so.0.symbols build-final/gthread/libgthread-2.0.so.0.8000.0" \
  "abi/symbols/libgmodule-2.0.so.0.symbols build-final/gmodule/libgmodule-2.0.so.0.8000.0" \
  "abi/symbols/libgobject-2.0.so.0.symbols build-final/gobject/libgobject-2.0.so.0.8000.0" \
  "abi/symbols/libgio-2.0.so.0.symbols build-final/gio/libgio-2.0.so.0.8000.0" \
  "abi/symbols/libgirepository-2.0.so.0.symbols build-final/girepository/libgirepository-2.0.so.0.8000.0"; do \
    set -- $pair; \
    python3 tools/compare-symbols.py --expected "$1" --library "$2"; \
  done
```

## Success Criteria
- `check-final-full` and `check-final-safety` both pass.
- The workspace has no remaining interoperability, package, or safety breakage that requires another phase-specific bounce target.
- The implementation is not complete if any library still depends on linked upstream object files for its shipped runtime, any shipped static archive or helper executable still comes from `safe/vendor/build-check`, any placeholder export remains, the safe packages fail `package-smoke`, `debian-tests`, or `dependents`, or the final unsafe audit flags avoidable unsafe code.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit before yielding to its verification phases.
- A verifier must treat an unchanged `HEAD` or a worktree-only deliverable as a failure.
