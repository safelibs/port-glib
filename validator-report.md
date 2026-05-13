# Validator Report

Phase: `impl-girepository-rust`

## Repository

- Repository commit used for the validator lock: `d127376484789f6dd1ae4042b48d365033b4e92a`
- Repository commit containing the `check-girepository-tests` bounce fix: `4c453ba4b83141d5e06f1c91ad57b4435be7796d`
- Worktree note: the validator/build pass included the scoped GIRepository/GIO implementation changes before the final phase commit was created.
- Unrelated dirty-file note: `workflow.yaml` was already modified outside this phase and was left untouched.
- Generated-output note: `bash scripts/build-debs.sh` updated `safe/debian/changelog` and `safe/debian/cross-tools/deb-can-run`; both remain unstaged generated build outputs.

## Validator Checkout

- Validator directory: `.work/validator`
- Validator checkout/ref tested: `9ae971508c9381f32a531078037851d960cab61f`
- Validator checkout summary: `9ae9715 ci(pages): fall back to GITHUB_TOKEN for port deb/source fetches`
- Default repository/ref: `https://github.com/safelibs/validator`, `main`

## Package Artifacts Tested

- `dist/libgirepository-2.0-0_2.80.0-6ubuntu3.8+safelibs1778623699_amd64.deb`
- `dist/libgirepository-2.0-dev_2.80.0-6ubuntu3.8+safelibs1778623699_amd64.deb`
- `dist/libglib2.0-0t64_2.80.0-6ubuntu3.8+safelibs1778623699_amd64.deb`
- `dist/libglib2.0-bin_2.80.0-6ubuntu3.8+safelibs1778623699_amd64.deb`
- `dist/libglib2.0-data_2.80.0-6ubuntu3.8+safelibs1778623699_all.deb`
- `dist/libglib2.0-dev-bin_2.80.0-6ubuntu3.8+safelibs1778623699_amd64.deb`
- `dist/libglib2.0-dev_2.80.0-6ubuntu3.8+safelibs1778623699_amd64.deb`

## Commands

Package build command:

```bash
bash scripts/build-debs.sh
```

Validator command:

```bash
bash scripts/run-validation-tests.sh
```

GIRepository verifier commands run locally:

```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-girepository --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-girepository/.stamp
python3 tools/link-compat.py --phase girepository --build-root build-girepository --compile-original-objects --run
python3 tools/run-meson-manifest.py --build-root build-girepository --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-girepository/meson-info/intro-tests.json --manifest tests/manifests/girepository.txt --print-errorlogs
cargo check --workspace
! rg -n "PLACEHOLDER_" crates/girepository/src/exports.rs
! rg -n "PLACEHOLDER_|abi_zero_arg_symbols" crates/girepository/src/exports.rs
! rg -n "pub unsafe extern \"C\" fn [A-Za-z0-9_]+\\(\\) -> usize" crates/girepository/src/exports.rs
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`
- Result directory: `.work/validation/artifacts/port/results/glib`

## Result

- Build result: PASS
- Validator result: PASS (`252` test result JSON files passed; `summary.json` has no case status)
- Validator failure scan: PASS, no failed/error statuses and no `override_debs_installed: false`
- `cve-2019-13012`: PASS, exit code 0 after the keyfile settings backend fix
- GIRepository link verifier result: PASS
- GIRepository manifest verifier result: PASS
- Workspace cargo check result: PASS
- Placeholder/export-stub scan result: PASS
- Bounce fix result: PASS, the `abi_zero_arg_symbols` export wall has been removed and the affected public exports now have explicit ABI signatures.

## Failure Summary

- An early validator attempt was stopped after it was confirmed to be using obsolete package artifacts with empty installed `gio` behavior.
- Follow-up validator runs exposed installed `gio` CLI gaps, schema/GSettings behavior gaps, Python GI usage gaps, and a `cve-2019-13012` regression failure where `g_settings_set_string()` returned false.
- The last failing full validator run had only `cve-2019-13012` failing with exit code 18.
- A later `check-girepository-tests` verifier review failed because `safe/crates/girepository/src/exports.rs` still retained `abi_zero_arg_symbols` stubs returning `0` for dozens of public GIRepository exports.

## Fixes Made After Failed Runs

- Replaced GIRepository placeholder runtime exports with real repository and typelib loading/query entry points.
- Made the build-root GIRepository static archive and `gi-*` tools come from safe outputs, with `build-abi-shell.py` rejecting vendored `safe/vendor/build-check` fallbacks.
- Added a Rust-owned installed `gio` CLI surface for the validator-covered commands and help paths.
- Added safe schema compilation and GSettings lookup support, including a custom compiled-schema fallback in GVDB table reads.
- Fixed the keyfile settings backend writable/path conversion path so relative root keys map to their configured root group and emit valid change notifications.
- Rebuilt packages and reran the full validator matrix to a clean pass.
- Removed the `abi_zero_arg_symbols` macro entirely, replaced every affected symbol with an explicit typed export, and backed those exports with Rust runtime functions for repository/base-info attributes, constants, enum values, field access, interface/object/property/struct queries, type-tag helpers, repository dump, and vfunc/callable invoke shims.
- Extended the GIR parser model to retain constant type/value data and enum member numeric values for the new GIRepository value/constant query exports.
- Reran `build-abi-shell.py`, `link-compat.py`, `run-meson-manifest.py`, `cargo check --workspace`, and the export-wall scans after the bounce fix; all passed.

## Next Action

Hand off to the fixed verifier phases in order: `check-girepository-link`, then `check-girepository-tests`.
