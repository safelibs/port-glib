# Final Hardening, Unsafe Reduction, and Full Verification

## Phase Name
Final hardening, unsafe reduction, and full verification

## Implement Phase ID
`impl-final-fixups`

## Preexisting Inputs
- `test-original.sh`
- `safe/debian/`
- `safe/abi/install-manifests/`
- `safe/abi/installed-files.json`
- `safe/abi/postinst-state/runtime.json`
- `safe/abi/debian-control-preservation.json`
- `safe/tests/package/`
- `safe/debian/tests/`
- `safe/tools/sync-upstream-assets.py`
- `safe/vendor/original/`
- `safe/vendor/build-check/`
- `safe/abi/tests.json`
- `safe/abi/test-source-path-map.json`
- `safe/abi/link-compat/*.json`
- `safe/abi/layout-manifests/*.json`
- `safe/abi/layouts/*.json`
- `safe/abi/version-scripts/*.map`
- `safe/abi/debian-patches.json`
- `safe/tests/upstream/glib/meson.build`
- `safe/tests/upstream/gobject/meson.build`
- `safe/tests/upstream/gmodule/meson.build`
- `safe/tests/upstream/gthread/meson.build`
- `safe/tests/upstream/glib/markup-collect.c`
- `safe/tests/upstream/*`
- `safe/tests/manifests/*`
- `safe/tests/manifests/full.txt`
- `safe/tests/cve/*`
- `relevant_cves.json`
- `dependents.json`
- `original/glib/*.c`, `original/glib/*.h`, and `original/glib/tests/*`
- `original/gthread/gthread-impl.c`
- `original/gmodule/gmodule.c`
- `original/gmodule/gmodule-deprecated.c`
- `original/gmodule/gmodule.h`
- `original/gobject/*.c`, `original/gobject/*.h`, and `original/gobject/tests/*`
- `original/gio/*.c`, `original/gio/*.h`, and `original/gio/tests/*`
- `original/girepository/*.c`, `original/girepository/*.h`, and `original/girepository/tests/*`
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
- `safe/abi/layouts/{glib,gthread,gmodule}.json`
- `safe/crates/glib/src/bookmark/api.rs`
- `safe/crates/glib/src/bytes/api.rs`
- `safe/crates/glib/src/charset/api.rs`
- `safe/crates/glib/src/fileutils/api.rs`
- `safe/crates/glib/src/gvariant/api.rs`
- `safe/crates/glib/src/hash/api.rs`
- `safe/crates/glib/src/keyfile/api.rs`
- `safe/crates/glib/src/markup/api.rs`
- `safe/crates/glib/src/options/api.rs`
- `safe/crates/glib/src/regex/api.rs`
- `safe/crates/glib/src/scanner/api.rs`
- `safe/crates/glib/src/spawn/api.rs`
- `safe/crates/glib/src/unicode/api.rs`
- `safe/crates/glib/src/uri/api.rs`
- `safe/crates/glib/src/bridge.rs`
- `safe/abi/link-compat/glib-advanced.json`
- `safe/docs/cve-matrix.md`
- `safe/crates/gobject/src/lib.rs`
- `safe/crates/gobject/src/object/mod.rs`
- `safe/crates/gobject/src/signal/mod.rs`
- `safe/crates/gobject/src/tools/mod.rs`
- `safe/crates/gobject/src/type_system/mod.rs`
- `safe/crates/gobject/src/value/mod.rs`
- `safe/crates/gobject/src/translated/compat.rs`
- `safe/crates/gobject/src/translated/original/gobject/*.rs`
- `safe/abi/version-scripts/libgobject.map`
- `safe/abi/layout-manifests/gobject.json`
- `safe/abi/layouts/gobject.json`
- `safe/crates/gio/build.rs`
- `safe/crates/gio/src/lib.rs`
- `safe/crates/gio/src/runtime.rs`
- `safe/crates/gio/src/exports.rs`
- `safe/crates/gio/src/*`
- `safe/abi/version-scripts/libgio.map`
- `safe/abi/link-compat/gio.json`
- `safe/tests/upstream/gio/*`
- `safe/crates/girepository/build.rs`
- `safe/crates/girepository/src/lib.rs`
- `safe/crates/girepository/src/exports.rs`
- `safe/crates/girepository/src/runtime.rs`
- `safe/crates/girepository/src/repository/mod.rs`
- `safe/crates/girepository/src/parser/mod.rs`
- `safe/crates/girepository/src/invoke/mod.rs`
- `safe/crates/girepository/src/tools/mod.rs`
- `safe/crates/girepository/src/*`
- `safe/tools/stage-package-tree.py`
- `safe/abi/version-scripts/libgirepository.map`
- `safe/abi/link-compat/girepository.json`
- `test-original.sh`
- `safe/debian/control`
- `safe/debian/rules`
- `safe/debian/tests/control`
- `safe/debian/tests/*`
- `safe/tests/package/girepository-compile-only.sh`
- `safe/tests/package/girepository-installed.sh`
- `safe/tools/compare-debian-control.py`
- `safe/tools/verify-package-baselines.py`
- `safe/tools/compare-installed-files.py`
- `safe/tools/check-postinst-state.py`
- `safe/abi/install-manifests/*.json`
- `safe/abi/installed-files.json`
- `safe/abi/postinst-state/runtime.json`
- `safe/abi/debian-control-preservation.json`

## New Outputs
- Final release-ready safe workspace and package set.
- Final safety audit contract.
- Final docs synchronized with the code and packaging state.

## File Changes
- Any remaining files from phases 1-7 that still violate the final contract
- `safe/tools/check-unsafe-audit.py`
- `safe/docs/cve-matrix.md`
- `safe/docs/debian-patch-provenance.md`

## Implementation Details
- Remove or justify every remaining `unsafe` block. The final audit should allow only required FFI/OS boundary unsafe code and should fail on undocumented or avoidable unsafe.
- Ensure no crate `build.rs` still injects or references vendored payloads from `safe/vendor/build-check` or the GLib backend replay tool in the final cdylibs.
- Ensure no shipped static archive or installed helper executable is still copied from `safe/vendor/build-check`; this explicitly includes `safe/tools/stage-package-tree.py`, whose package payload resolution must come from the current build root or source tree instead. Keeping upstream script-template packaging from `safe/vendor/original/` is acceptable where the shipped interface is already Python or shell.
- Eliminate any remaining placeholder exports, dead bootstrap forwarders, or test-only compatibility shims that would make the installed package depend on the original C implementation.
- Refresh the CVE matrix and Debian patch provenance documentation to describe the final Rust-owned behavior, not the bootstrap shell.

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

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit before yielding to its verification phases.
- A verifier must treat an unchanged `HEAD` or a worktree-only deliverable as a failure.
