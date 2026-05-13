# Validator Report

Phase: `impl-package-integration`

## Repository

- Repository commit used for the validator lock: `4f83a0d4b70309c07a21ab72b21b7a7e2b753674`
- Release tag synthesized by the validator lock: `build-4f83a0d4b703`
- Worktree note: `workflow.yaml` was already modified outside this phase and was left untouched.
- Generated-output note: `bash scripts/build-debs.sh` updated `safe/debian/changelog` and `safe/debian/cross-tools/deb-can-run`; both remain unstaged generated build outputs.

## Validator Checkout

- Validator directory: `.work/validator`
- Validator checkout/ref tested: `9ae971508c9381f32a531078037851d960cab61f`
- Validator checkout summary: `9ae9715 ci(pages): fall back to GITHUB_TOKEN for port deb/source fetches`
- Default repository/ref: `https://github.com/safelibs/validator`, `main`

## Package Artifacts Tested

- `dist/libgirepository-2.0-0_2.80.0-6ubuntu3.8+safelibs1778655048_amd64.deb`
- `dist/libgirepository-2.0-dev_2.80.0-6ubuntu3.8+safelibs1778655048_amd64.deb`
- `dist/libglib2.0-0t64_2.80.0-6ubuntu3.8+safelibs1778655048_amd64.deb`
- `dist/libglib2.0-bin_2.80.0-6ubuntu3.8+safelibs1778655048_amd64.deb`
- `dist/libglib2.0-data_2.80.0-6ubuntu3.8+safelibs1778655048_all.deb`
- `dist/libglib2.0-dev-bin_2.80.0-6ubuntu3.8+safelibs1778655048_amd64.deb`
- `dist/libglib2.0-dev_2.80.0-6ubuntu3.8+safelibs1778655048_amd64.deb`

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

Phase package checks run locally:

```bash
cd safe
dpkg-buildpackage -b -uc -us
python3 tools/compare-debian-control.py --baseline abi/debian-control-preservation.json --control debian/control
python3 tools/verify-package-baselines.py --source . --work-root build-package-baselines --abi-shell-profiles "nodoc noinsttest nogir noudeb" --install-manifests abi/install-manifests --postinst-manifest abi/postinst-state/runtime.json
```

Installed-package harness checks run locally:

```bash
GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=package-smoke GLIB_PACKAGE_BUILD_JOBS=2 ./test-original.sh
GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=debian-tests GLIB_PACKAGE_BUILD_JOBS=2 ./test-original.sh
GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=dependents GLIB_PACKAGE_BUILD_JOBS=2 ./test-original.sh
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`
- Result directory: `.work/validation/artifacts/port/results/glib`
- Summary file: `.work/validation/artifacts/port/results/glib/summary.json`

## Result

- Package baseline result: PASS.
- Package smoke harness result: PASS.
- Debian autopkgtest harness result: PASS.
- Dependent harness result: PASS.
- Build result: PASS; `scripts/build-debs.sh` refreshed `dist/*.deb` with the `safelibs1778655048` package set.
- Validator result: PASS; `252` cases passed, `0` failed.
- Validator failure scan: PASS; no failed/error statuses and no `override_debs_installed: false`.

## Failure Summary

- No package-integration verifier command failed in this run.
- No validator failures occurred against the refreshed `safelibs1778655048` package set.

## Fixes Made After Failed Runs

- No post-failure code fix was needed during this phase run.
- The report was updated to replace stale GIRepository-phase evidence with current package-integration build, harness, and validator evidence.

## Next Action

Hand off to the fixed verifier phases in order: `check-package-baselines`, `check-package-smoke`, `check-package-autopkgtests`, then `check-dependent-harness`.
