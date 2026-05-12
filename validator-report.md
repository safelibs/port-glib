# Validator Report

Phase: `impl-glib-core`

## Repository

- Repository commit at build/validator run: `49140169937a364903591c85bf57358b073663e6`
- Worktree note: `workflow.yaml` was already modified before this phase work and was left untouched.

## Validator Checkout

- Validator directory: `.work/validator`
- Validator checkout/ref tested: `bde8758883d12061dfb2621b6149949909c803f8`
- Managed checkout state: detached `FETCH_HEAD` from default ref `main`
- Default repository: `https://github.com/safelibs/validator`

## Package Artifacts Tested

- `dist/libgirepository-2.0-0_2.80.0-6ubuntu3.8+safelibs1778598804_amd64.deb`
- `dist/libgirepository-2.0-dev_2.80.0-6ubuntu3.8+safelibs1778598804_amd64.deb`
- `dist/libglib2.0-0t64_2.80.0-6ubuntu3.8+safelibs1778598804_amd64.deb`
- `dist/libglib2.0-bin_2.80.0-6ubuntu3.8+safelibs1778598804_amd64.deb`
- `dist/libglib2.0-data_2.80.0-6ubuntu3.8+safelibs1778598804_all.deb`
- `dist/libglib2.0-dev-bin_2.80.0-6ubuntu3.8+safelibs1778598804_amd64.deb`
- `dist/libglib2.0-dev_2.80.0-6ubuntu3.8+safelibs1778598804_amd64.deb`

## Commands

Package build command:

```bash
bash scripts/build-debs.sh
```

Validator command:

```bash
bash scripts/run-validation-tests.sh
```

Core verifier commands run locally:

```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-glib-core --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-glib-core/.stamp
python3 tools/link-compat.py --phase glib-core --build-root build-glib-core --compile-original-objects --run
python3 tools/run-meson-manifest.py --build-root build-glib-core --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-glib-core/meson-info/intro-tests.json --manifest tests/manifests/glib-core.txt --print-errorlogs
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`
- Artifact note: final validator pass produced no failure artifacts under `.work/validation/artifacts`.

## Result

- Build result: PASS
- Validator result: PASS
- Core link verifier result: PASS
- Core manifest verifier result: PASS

## Failure Summary

- First package build failed because `safe/vendor/build-check` contained a stale Meson build root pointing at a different checkout.
- A later package build failed in `dh_makeshlibs` because private GObject symbols present in the Rust static archive were not exported by the shared library version script.
- The first validator run failed before the matrix because `dist/` contained duplicate package names from stale unstamped parent artifacts copied alongside the stamped packages.
- A core manifest rerun failed because regenerated Meson test inventory normalized to the same `LD_LIBRARY_PATH` components in a different order than the frozen baseline.

## Fixes Made After Failed Runs

- Added stale `vendor/build-check` root detection and removal to `scripts/build-debs.sh`.
- Made Debian changelog stamping idempotent in `scripts/lib/build-deb-common.sh`.
- Preserved and restored `safe/debian/patches` around the binary package build instead of deleting it permanently.
- Filtered copied `dist/` artifacts to the current stamped Debian version.
- Exported the missing private GObject symbols in `safe/abi/version-scripts/libgobject.map`.
- Rewrote staged ABI-shell `intro-tests.json` paths away from in-repository `safe/vendor/build-check` locations.
- Made the manifest drift comparison order-insensitive for build-root-only `LD_LIBRARY_PATH` values while keeping runtime execution unchanged.

## Next Action

Hand off to the fixed verifier phases in order: `check-glib-core-link`, then `check-glib-core-tests`.
