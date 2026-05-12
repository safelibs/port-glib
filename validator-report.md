# Validator Report

Phase: `impl-gobject-rust`

## Repository

- Repository commit at build/validator run: `4e2eeb04205c99d9100a902b586069103a66ce23`
- Worktree note: `workflow.yaml` was already modified before this phase work and was left untouched.
- Phase worktree note: `safe/tools/build-abi-shell.py` contained the GObject ownership guard during the build, verifier, and validator runs; it is committed by this phase after this report update.

## Validator Checkout

- Validator directory: `.work/validator`
- Validator checkout/ref tested: `bde8758883d12061dfb2621b6149949909c803f8`
- Managed checkout state: detached `FETCH_HEAD` from default ref `main`
- Default repository: `https://github.com/safelibs/validator`

## Package Artifacts Tested

- `dist/libgirepository-2.0-0_2.80.0-6ubuntu3.8+safelibs1778609672_amd64.deb`
- `dist/libgirepository-2.0-dev_2.80.0-6ubuntu3.8+safelibs1778609672_amd64.deb`
- `dist/libglib2.0-0t64_2.80.0-6ubuntu3.8+safelibs1778609672_amd64.deb`
- `dist/libglib2.0-bin_2.80.0-6ubuntu3.8+safelibs1778609672_amd64.deb`
- `dist/libglib2.0-data_2.80.0-6ubuntu3.8+safelibs1778609672_all.deb`
- `dist/libglib2.0-dev-bin_2.80.0-6ubuntu3.8+safelibs1778609672_amd64.deb`
- `dist/libglib2.0-dev_2.80.0-6ubuntu3.8+safelibs1778609672_amd64.deb`

## Commands

Package build command:

```bash
bash scripts/build-debs.sh
```

Validator command:

```bash
bash scripts/run-validation-tests.sh
```

GObject verifier commands run locally:

```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-gobject --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gobject/.stamp
python3 tools/link-compat.py --phase gobject --build-root build-gobject --compile-original-objects --run
python3 tools/run-meson-manifest.py --build-root build-gobject --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gobject/meson-info/intro-tests.json --manifest tests/manifests/gobject.txt --print-errorlogs
cargo check --workspace
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`
- Artifact note: validator pass produced no failure artifacts under `.work/validation/artifacts`.

## Result

- Build result: PASS
- Validator result: PASS
- GObject link verifier result: PASS
- GObject manifest verifier result: PASS
- Workspace cargo check result: PASS

## Failure Summary

- No failed build, validator, GObject link, GObject manifest, or workspace cargo check run occurred during this phase.

## Fixes Made After Failed Runs

- No post-failure fixes were required in this phase.
- Scoped phase update added a `safe-gobject` ownership guard to `safe/tools/build-abi-shell.py`, rejecting upstream `libgobject-2.0.so.0` linkage, vendored static archive members, or missing Rust-owned GObject sentinel symbols.

## Next Action

Hand off to the fixed verifier phases in order: `check-gobject-link`, then `check-gobject-tests`.
