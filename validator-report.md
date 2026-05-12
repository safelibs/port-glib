# Validator Report

Phase: `impl-safe-bootstrap`

## Repository

- Repository commit: `5d83f3d088e6c670e3e0761893c8ed50da5ae7e5`
- Worktree note: `workflow.yaml` was already modified before this phase work and was left untouched.

## Validator Checkout

- Validator directory: `.work/validator`
- Checkout state for this report: not present yet
- Default repository: `https://github.com/safelibs/validator`
- Default ref: `main`
- Checkout/update contract: `scripts/run-validation-tests.sh` clones `.work/validator` when absent, or fetches `SAFELIBS_VALIDATOR_REF` with depth 1 and force-checks out `FETCH_HEAD` when the checkout already exists. `SAFELIBS_VALIDATOR_DIR`, when set, bypasses the managed checkout.

## Package Artifacts Tested

- `dist/*.deb`: none present at report initialization
- Package validation status: deferred until `bash scripts/build-debs.sh` produces package artifacts
- ABI-shell package verifier artifacts produced outside `dist/` by the checker command:
  - `gir1.2-girepository-3.0-dev_2.80.0-6ubuntu3.8_amd64.deb`
  - `gir1.2-girepository-3.0_2.80.0-6ubuntu3.8_amd64.deb`
  - `gir1.2-glib-2.0-dev_2.80.0-6ubuntu3.8_amd64.deb`
  - `gir1.2-glib-2.0_2.80.0-6ubuntu3.8_amd64.deb`
  - `libgirepository-2.0-0_2.80.0-6ubuntu3.8_amd64.deb`
  - `libgirepository-2.0-dev_2.80.0-6ubuntu3.8_amd64.deb`
  - `libglib2.0-0t64_2.80.0-6ubuntu3.8_amd64.deb`
  - `libglib2.0-bin_2.80.0-6ubuntu3.8_amd64.deb`
  - `libglib2.0-data_2.80.0-6ubuntu3.8_all.deb`
  - `libglib2.0-dev-bin_2.80.0-6ubuntu3.8_amd64.deb`
  - `libglib2.0-dev_2.80.0-6ubuntu3.8_amd64.deb`
  - `libglib2.0-tests_2.80.0-6ubuntu3.8_amd64.deb`

## Commands

Bootstrap metadata verification run:

```bash
cd safe
cargo check --workspace
python3 tools/extract_abi.py --verify
python3 tools/extract_layouts.py --verify
python3 tools/link-compat.py --verify-manifests
python3 tools/verify-debian-patches.py --verify-manifest
```

Editable mirror contract verification run:

```bash
cd safe
python3 tools/sync-upstream-assets.py --verify-map abi/test-source-path-map.json
```

Bootstrap ABI-shell verification run:

```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-abi-shell --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-abi-shell/.stamp
python3 tools/link-compat.py --phase abi-shell --build-root build-abi-shell --compile-original-objects --run
dpkg-buildpackage -b -uc -us
python3 tools/verify-package-baselines.py --source . --work-root build-package-baselines --abi-shell-profiles "nodoc noinsttest nogir noudeb" --install-manifests abi/install-manifests --postinst-manifest abi/postinst-state/runtime.json
```

Package build command for validator input:

```bash
bash scripts/build-debs.sh
```

Validator command:

```bash
bash scripts/run-validation-tests.sh
```

## Artifact Root

- Validator artifact root: `.work/validation/artifacts`
- Port deb lock path: `.work/validation/port-deb-lock.json`
- Override deb root: `.work/validation/override-debs`

## Result

- Bootstrap metadata result: PASS
- Bootstrap ABI-shell result: PASS
- Validator result: NOT RUN
- Failure summary: no validator failure has been observed in this phase; package artifacts have not yet been built for validation.
- Fixes made after failed run: none.

## Next Action

Run `bash scripts/build-debs.sh` followed by `bash scripts/run-validation-tests.sh`, then update this report with the validator checkout commit, package artifact list, PASS/FAIL result, and any fixes made after failures.
