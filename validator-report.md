# Validator Report

Phase: `impl-final-fixups`

## Repository

- Repository commit used for the validator lock: `7425e4cdcd3494276027872f8e502467ff74d0ae`
- Release tag synthesized by the validator lock: `build-7425e4cdcd34`
- Worktree note: `workflow.yaml` was already modified outside this phase and was left untouched.
- Generated-output note: package builds updated `safe/debian/changelog` and `safe/debian/cross-tools/deb-can-run`; both remain unstaged generated build outputs.

## Validator Checkout

- Validator directory: `.work/validator`
- Validator checkout/ref tested: `9ae971508c9381f32a531078037851d960cab61f`
- Validator checkout summary: `9ae9715 ci(pages): fall back to GITHUB_TOKEN for port deb/source fetches`
- Default repository/ref: `https://github.com/safelibs/validator`, `main`

## Package Artifacts Tested

- `dist/libgirepository-2.0-0_2.80.0-6ubuntu3.8+safelibs1778664198_amd64.deb`
  - sha256: `d502433ff692070b77eae210b4762af7119bcf9acdd9bc2b1c06d4cae2f94d37`
- `dist/libgirepository-2.0-dev_2.80.0-6ubuntu3.8+safelibs1778664198_amd64.deb`
  - sha256: `2463636a11f0bd926f0a9c735b2880aeec7243cab1bd7e6a71a123721692ac57`
- `dist/libglib2.0-0t64_2.80.0-6ubuntu3.8+safelibs1778664198_amd64.deb`
  - sha256: `6f0fe8982bc9ad674796fb84e5672315d43d8ec0e8f41b9b1e33234dfdd673aa`
- `dist/libglib2.0-bin_2.80.0-6ubuntu3.8+safelibs1778664198_amd64.deb`
  - sha256: `6b663a76c66c7ff266fe35df9a92dda7e79f37d1b17944e42591bf917c5cd7c2`
- `dist/libglib2.0-data_2.80.0-6ubuntu3.8+safelibs1778664198_all.deb`
  - sha256: `7b16428cc66192184b481839b5d2108e8f2c80b3024ec97ce2494743ec87830f`
- `dist/libglib2.0-dev-bin_2.80.0-6ubuntu3.8+safelibs1778664198_amd64.deb`
  - sha256: `b76c3c83768fad75a9fbdbf9cf2f5f8911c31f4d5bcec215d2d60e774e30caeb`
- `dist/libglib2.0-dev_2.80.0-6ubuntu3.8+safelibs1778664198_amd64.deb`
  - sha256: `3e1b744a2d338f611260d42374348dfb5252817c5dcf1efb3a46faf82ec3d9b9`

The synthesized port lock matched 7 canonical validator packages and recorded these unported original packages: `gir1.2-glib-2.0`, `gir1.2-glib-2.0-dev`, `gir1.2-girepository-3.0`, and `gir1.2-girepository-3.0-dev`.

## Commands

Package build command:

```bash
bash scripts/build-debs.sh
```

Validator command:

```bash
bash scripts/run-validation-tests.sh
```

Final full verifier commands run locally:

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

Final safety verifier commands run locally after the export-surface fix:

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
python3 tools/compare-symbols.py --expected abi/symbols/libglib-2.0.so.0.symbols --library build-final/glib/libglib-2.0.so.0.8000.0
python3 tools/compare-symbols.py --expected abi/symbols/libgthread-2.0.so.0.symbols --library build-final/gthread/libgthread-2.0.so.0.8000.0
python3 tools/compare-symbols.py --expected abi/symbols/libgmodule-2.0.so.0.symbols --library build-final/gmodule/libgmodule-2.0.so.0.8000.0
python3 tools/compare-symbols.py --expected abi/symbols/libgobject-2.0.so.0.symbols --library build-final/gobject/libgobject-2.0.so.0.8000.0
python3 tools/compare-symbols.py --expected abi/symbols/libgio-2.0.so.0.symbols --library build-final/gio/libgio-2.0.so.0.8000.0
python3 tools/compare-symbols.py --expected abi/symbols/libgirepository-2.0.so.0.symbols --library build-final/girepository/libgirepository-2.0.so.0.8000.0
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`
- Result directory: `.work/validation/artifacts/port/results/glib`
- Summary file: `.work/validation/artifacts/port/results/glib/summary.json`

## Result

Result: PASS

- Metadata and ABI/layout manifest verification: PASS.
- Full link-compat replay: PASS.
- Meson manifest replay: PASS.
- CVE regression replay: PASS.
- Debian package build and package baseline verification: PASS.
- Installed-package harness with `GLIB_TEST_SCOPE=all`: PASS.
- Final safety, layout, and symbol verification: PASS.
- Build result: PASS; `scripts/build-debs.sh` refreshed `dist/*.deb` with the `safelibs1778664198` package set.
- Validator result: PASS; `252` cases passed, `0` failed.
- Validator failure scan: PASS; no failed/error statuses and no `override_debs_installed: false`.

## Failure Summary

- A local final-safety run initially failed because `libgobject-2.0.so.0` exported 23 private `_g_*` symbols beyond `safe/abi/symbols/libgobject-2.0.so.0.symbols`.
- No validator failures occurred against the final `safelibs1778664198` package set.

## Fixes Made After Failed Runs

- Removed the private `_g_*` entries from `safe/abi/version-scripts/libgobject.map`, making them local-only in the final shared object.
- Removed the same private `_g_*` entries from `safe/debian/libglib2.0-0t64.symbols` so package symbol metadata matches the final public surface.
- Reran final safety verification; all final library symbol counts matched the authoritative baselines: GLib 1872, GThread 2, GModule 10, GObject 478, GIO 2107, GIRepository 231.
- No post-validator code fix was needed.

## Next Action

Hand off to the fixed verifier phases in order: `check-final-full`, then `check-final-safety`.
