# Validator Report

Phase: `impl-glib-advanced`

## Repository

- Repository commit at build/validator run: `40b74ae848ed01732eebdf883b61aa4c2a0eeb18`
- Worktree note: `workflow.yaml` was already modified before this phase work and was left untouched.

## Validator Checkout

- Validator directory: `.work/validator`
- Validator checkout/ref tested: `bde8758883d12061dfb2621b6149949909c803f8`
- Managed checkout state: detached `FETCH_HEAD` from default ref `main`
- Default repository: `https://github.com/safelibs/validator`

## Package Artifacts Tested

- `dist/libgirepository-2.0-0_2.80.0-6ubuntu3.8+safelibs1778605455_amd64.deb`
- `dist/libgirepository-2.0-dev_2.80.0-6ubuntu3.8+safelibs1778605455_amd64.deb`
- `dist/libglib2.0-0t64_2.80.0-6ubuntu3.8+safelibs1778605455_amd64.deb`
- `dist/libglib2.0-bin_2.80.0-6ubuntu3.8+safelibs1778605455_amd64.deb`
- `dist/libglib2.0-data_2.80.0-6ubuntu3.8+safelibs1778605455_all.deb`
- `dist/libglib2.0-dev-bin_2.80.0-6ubuntu3.8+safelibs1778605455_amd64.deb`
- `dist/libglib2.0-dev_2.80.0-6ubuntu3.8+safelibs1778605455_amd64.deb`

## Commands

Package build command:

```bash
bash scripts/build-debs.sh
```

Validator command:

```bash
bash scripts/run-validation-tests.sh
```

Advanced verifier commands run locally:

```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-glib-advanced --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-glib-advanced/.stamp
python3 tools/link-compat.py --phase glib-advanced --build-root build-glib-advanced --compile-original-objects --run
python3 tools/run-meson-manifest.py --build-root build-glib-advanced --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-glib-advanced/meson-info/intro-tests.json --manifest tests/manifests/glib-advanced.txt --print-errorlogs
python3 tools/run-cve-regressions.py --phase glib --build-root build-glib-advanced --rebuild
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`
- Artifact note: validator pass produced no failure artifacts under `.work/validation/artifacts`.

## Result

- Build result: PASS
- Validator result: PASS
- Advanced link verifier result: PASS
- Advanced manifest verifier result: PASS
- GLib CVE regression result: PASS

## Failure Summary

- No failed build, validator, advanced link, advanced manifest, or GLib CVE run occurred during this phase.

## Fixes Made After Failed Runs

- No post-failure fixes were required in this phase.
- Scoped phase updates recorded GLib advanced ownership in the crate phase marker, kept `build-glib-backend.py` as a retired replay-helper tombstone, and updated `safe/docs/cve-matrix.md` with the exact GLib advanced CVE command.

## Next Action

Hand off to the fixed verifier phases in order: `check-glib-advanced-link`, `check-glib-advanced-tests`, then `check-glib-cves`.
