# Integrate Debian Packaging and Convert the Dependent Harness

## Phase Name
Integrate Debian packaging and convert the dependent harness

## Implement Phase ID
`impl-package-integration`

## Source Plan Context
- Overall target remains GLib 2.80.0 package compatibility on Ubuntu 24.04, after the library crates have stopped relying on shipped upstream object payloads.
- `safe/debian/tests/control` already declares the Debian package test surface: `build`, `build-static`, `installed-tests`, `closure-refcount`, `debugcontroller`, `gdbus-server-auth`, `gdbus-threading`, `gmenumodel`, `mainloop`, `memory-monitor-dbus`, `socket`, `testfilemonitor`, `thread-pool-slow`, `threadtests`, `timeout`, `timer`, and `1065022-futureproofing`.
- Current harness state: `test-original.sh` builds and installs upstream GLib into `/opt/glib-original`, asserts binaries load the original library, installs dependents, and runs package-specific runtime probes. It does not yet build, install, or exercise the safe Debian packages.
- The package-content contracts already exist in `safe/abi/install-manifests/*.json`, `safe/abi/installed-files.json`, `safe/abi/postinst-state/runtime.json`, and `safe/abi/debian-control-preservation.json`; consume and update them in place.
- `dependents.json` is the canonical dependent inventory and must remain the source of truth for dependent coverage, including the compile-time-only `budgie-artwork` / `pocillo-icon-theme` path.

## Preexisting Inputs
- `safe/crates/girepository/`
- `safe/tools/build-abi-shell.py`
- `safe/tools/stage-package-tree.py`
- `safe/abi/link-compat/girepository.json`
- `safe/debian/`
- `safe/debian/control`
- `safe/debian/rules`
- `safe/debian/tests/1065022-futureproofing`
- `safe/debian/tests/build`
- `safe/debian/tests/build-static`
- `safe/debian/tests/closure-refcount`
- `safe/debian/tests/control`
- `safe/debian/tests/debugcontroller`
- `safe/debian/tests/gdbus-server-auth`
- `safe/debian/tests/gdbus-threading`
- `safe/debian/tests/gmenumodel`
- `safe/debian/tests/installed-tests`
- `safe/debian/tests/mainloop`
- `safe/debian/tests/memory-monitor-dbus`
- `safe/debian/tests/run-with-locales`
- `safe/debian/tests/socket`
- `safe/debian/tests/testfilemonitor`
- `safe/debian/tests/thread-pool-slow`
- `safe/debian/tests/threadtests`
- `safe/debian/tests/timeout`
- `safe/debian/tests/timer`
- `safe/abi/install-manifests/abi-shell.json`
- `safe/abi/install-manifests/dev-package.json`
- `safe/abi/install-manifests/doc-package.json`
- `safe/abi/install-manifests/full.json`
- `safe/abi/install-manifests/runtime.json`
- `safe/abi/install-manifests/udeb.json`
- `safe/abi/installed-files.json`
- `safe/abi/postinst-state/runtime.json`
- `safe/abi/debian-control-preservation.json`
- `safe/tests/package/girepository-compile-only.sh`
- `safe/tests/package/girepository-consumer.c`
- `safe/tests/package/girepository-installed.sh`
- `dependents.json`
- `test-original.sh`

## New Outputs
- Safe Debian packages that match the recorded install manifests and trigger behavior.
- A converted dependent harness that installs and exercises the safe packages rather than the original prefix build, including explicit `package-smoke`, `debian-tests`, `dependents`, and `all` scopes.

## File Changes
- `test-original.sh`
- `safe/debian/control`
- `safe/debian/rules`
- `safe/debian/tests/control`
- `safe/debian/tests/{build,build-static,installed-tests,closure-refcount,debugcontroller,gdbus-server-auth,gdbus-threading,gmenumodel,mainloop,memory-monitor-dbus,run-with-locales,socket,testfilemonitor,thread-pool-slow,threadtests,timeout,timer,1065022-futureproofing}`
- `safe/tests/package/girepository-compile-only.sh`
- `safe/tests/package/girepository-installed.sh`
- `safe/tools/stage-package-tree.py`
- `safe/tools/compare-debian-control.py`
- `safe/tools/verify-package-baselines.py`
- `safe/tools/compare-installed-files.py`
- `safe/tools/check-postinst-state.py`
- `safe/abi/install-manifests/*.json`
- `safe/abi/installed-files.json`
- `safe/abi/postinst-state/runtime.json`
- `safe/abi/debian-control-preservation.json`

## Implementation Details
- Consume `safe/debian/*`, `safe/tests/package/*`, `safe/debian/tests/*`, `dependents.json`, and `test-original.sh` in place; do not replace them with a separate generated harness.
- Replace the original-only harness logic in `test-original.sh`: `assert_binary_uses_original_glib()` at line 60, `build_original_glib()` at line 119, `install_runtime_packages()` at line 152, `run_manifest_entry()` at line 481, and `main()` at line 539. Replace it with a safe-package path that builds the Debian packages from `safe/`, installs them in the container, asserts binaries resolve GLib from the installed safe package path, and reuses the existing dependent runtime probes.
- In safe mode, copy `safe/` into a writable container path before building, because the repository is mounted read-only in the container. Install the build dependencies declared by `safe/debian/control` before invoking `dpkg-buildpackage`; do not reuse the original-tree Meson install as a proxy for the safe package build.
- Keep an original-mode path only if it is useful as an oracle, but safe mode must be the default from this phase onward.
- Add a harness mode variable such as `GLIB_UNDER_TEST=original|safe` and a scope variable such as `GLIB_TEST_SCOPE=package-smoke|debian-tests|dependents|all` so later checks can select the right package/runtime coverage without maintaining multiple scripts.
- Define `package-smoke` so it builds and installs the safe packages, then runs `safe/debian/tests/build`, `safe/debian/tests/build-static`, `safe/tests/package/girepository-compile-only.sh`, and `safe/tests/package/girepository-installed.sh`.
- Define `debian-tests` so it performs the same safe-package build/install setup needed for the current harness run, then executes every Debian autopkgtest entry point declared in `safe/debian/tests/control`: `installed-tests`, `closure-refcount`, `debugcontroller`, `gdbus-server-auth`, `gdbus-threading`, `gmenumodel`, `mainloop`, `memory-monitor-dbus`, `socket`, `testfilemonitor`, `thread-pool-slow`, `threadtests`, `timeout`, `timer`, and `1065022-futureproofing`, invoking `safe/debian/tests/run-with-locales` wherever the test scripts already depend on it.
- Define `dependents` so it installs the safe runtime packages, then runs the existing runtime probes for the `compile_time_and_runtime` dependents and the `budgie-artwork` source build that covers `pocillo-icon-theme`.
- Define `all` so it runs `package-smoke`, `debian-tests`, and `dependents` in sequence.
- In safe mode, remove the original prefix-style `set_glib_env()` override model. The checks must prove that the installed package files on the container filesystem are what the binaries and build tools actually resolve.
- Make the package test scripts repository-relative and container-safe instead of assuming a fixed `/src/...` path.
- Preserve the `verify_manifest()` exact dependent inventory check in `test-original.sh`; only update that assertion if `dependents.json` itself changes.
- Keep the package content, triggers, and `debian/control` structure aligned with `safe/abi/install-manifests/*.json`, `safe/abi/postinst-state/runtime.json`, and `safe/abi/debian-control-preservation.json`.
- Critical file responsibilities owned by this phase:
  - `safe/debian/*` is the Debian source package, install lists, autopkgtests, triggers, and helper-script surface for Ubuntu drop-in replacement.
  - `safe/tools/stage-package-tree.py` maps the build root into the Debian package filesystem layout and must stop resolving package payloads through `safe/vendor/build-check`.
  - `safe/tools/verify-package-baselines.py`, `safe/tools/compare-debian-control.py`, `safe/tools/compare-installed-files.py`, and `safe/tools/check-postinst-state.py` are the package, installed-file, control-preservation, and trigger-state verification tools.
  - `safe/abi/install-manifests/*.json`, `safe/abi/installed-files.json`, `safe/abi/postinst-state/runtime.json`, and `safe/abi/debian-control-preservation.json` are package-content and trigger-state contracts consumed and updated only in place.
  - `safe/tests/package/*` are package-level compile/install smoke tests that must run against installed safe packages, not host distro GLib.
  - `test-original.sh` becomes the safe-package harness while preserving dependent inventory coverage and explicit `package-smoke`, `debian-tests`, `dependents`, and `all` scopes.
  - `dependents.json` remains the canonical dependent inventory.

## Verification Phases
### `check-package-baselines`
- Phase ID: `check-package-baselines`
- Type: `check`
- Fixed `bounce_target`: `impl-package-integration`
- Purpose: Verify the built packages, `debian/control` preservation contract, installed-file manifests, and postinst/postrm trigger behavior.
- Commands:
```bash
cd safe
dpkg-buildpackage -b -uc -us
python3 tools/compare-debian-control.py --baseline abi/debian-control-preservation.json --control debian/control
python3 tools/verify-package-baselines.py --source . --work-root build-package-baselines --abi-shell-profiles "nodoc noinsttest nogir noudeb" --install-manifests abi/install-manifests --postinst-manifest abi/postinst-state/runtime.json
```

### `check-package-smoke`
- Phase ID: `check-package-smoke`
- Type: `check`
- Fixed `bounce_target`: `impl-package-integration`
- Purpose: Verify compile-only and installed-package developer workflows against packages actually built and installed from `safe/`.
- Commands:
```bash
GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=package-smoke ./test-original.sh
```

### `check-package-autopkgtests`
- Phase ID: `check-package-autopkgtests`
- Type: `check`
- Fixed `bounce_target`: `impl-package-integration`
- Purpose: Verify that the installed safe packages satisfy the Debian autopkgtest surface declared in `safe/debian/tests/control`, not just the compile/install smoke subset.
- Commands:
```bash
GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=debian-tests ./test-original.sh
```

### `check-dependent-harness`
- Phase ID: `check-dependent-harness`
- Type: `check`
- Fixed `bounce_target`: `impl-package-integration`
- Purpose: Prove that the safe packages are drop-in replacements for the dependent inventory in `dependents.json`, including the compile-time-only `budgie-artwork` / `pocillo-icon-theme` coverage path.
- Commands:
```bash
GLIB_UNDER_TEST=safe GLIB_TEST_SCOPE=dependents ./test-original.sh
```

## Success Criteria
- `check-package-baselines`, `check-package-smoke`, `check-package-autopkgtests`, and `check-dependent-harness` all pass.
- `test-original.sh` installs and exercises the safe packages in its default mode.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit containing the scoped implementation before yielding to its verification phases.
- The implementer should report the resulting commit hash to the checker.
- A verifier must treat an unchanged `HEAD`, an empty commit with no file changes, or a worktree-only deliverable as a failure.
