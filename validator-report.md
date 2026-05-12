# Validator Report

Phase: `impl-gio-rust`

## Repository

- Repository commit at build/validator run: `c1ae1a95e32a268f8b48fb26e3c8b55ff67793bd`
- Worktree note: `workflow.yaml` was already modified before this phase work and was left untouched.
- Phase worktree note: `bash scripts/build-debs.sh` stamped `safe/debian/changelog` and rebuilt `safe/debian/cross-tools/deb-can-run` as generated package-build outputs; they are left unstaged and outside the scoped phase commit.

## Validator Checkout

- Validator directory: `.work/validator`
- Validator checkout/ref tested: `9ae971508c9381f32a531078037851d960cab61f`
- Managed checkout state: detached `FETCH_HEAD` from default ref `main`
- Default repository: `https://github.com/safelibs/validator`

## Package Artifacts Tested

- `dist/libgirepository-2.0-0_2.80.0-6ubuntu3.8+safelibs1778614527_amd64.deb`
- `dist/libgirepository-2.0-dev_2.80.0-6ubuntu3.8+safelibs1778614527_amd64.deb`
- `dist/libglib2.0-0t64_2.80.0-6ubuntu3.8+safelibs1778614527_amd64.deb`
- `dist/libglib2.0-bin_2.80.0-6ubuntu3.8+safelibs1778614527_amd64.deb`
- `dist/libglib2.0-data_2.80.0-6ubuntu3.8+safelibs1778614527_all.deb`
- `dist/libglib2.0-dev-bin_2.80.0-6ubuntu3.8+safelibs1778614527_amd64.deb`
- `dist/libglib2.0-dev_2.80.0-6ubuntu3.8+safelibs1778614527_amd64.deb`

## Commands

Package build command:

```bash
bash scripts/build-debs.sh
```

Validator command:

```bash
bash scripts/run-validation-tests.sh
```

GIO verifier commands run locally:

```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-gio --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-gio/.stamp
python3 tools/link-compat.py --phase gio --build-root build-gio --compile-original-objects --run
python3 tools/run-meson-manifest.py --build-root build-gio --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-gio/meson-info/intro-tests.json --manifest tests/manifests/gio.txt --print-errorlogs
python3 tools/run-cve-regressions.py --phase gio --build-root build-gio --rebuild
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`
- Artifact note: validator pass produced no failure artifacts under `.work/validation/artifacts`.

## Result

- Build result: PASS
- Validator result: PASS
- GIO link verifier result: PASS
- GIO manifest verifier result: PASS
- GIO CVE verifier result: PASS

## Failure Summary

- No failed build, validator, GIO link, GIO manifest, or GIO CVE run occurred during this phase.

## Fixes Made After Failed Runs

- No post-failure fixes were required in this phase.
- Scoped phase update changed the GIO bootstrap marker in `safe/crates/gio/src/lib.rs` from `impl-safe-bootstrap` to `impl-gio-rust`.

## Next Action

Hand off to the fixed verifier phases in order: `check-gio-link`, then `check-gio-tests`, then `check-gio-cves`.
