# GLib Advanced

## Phase Name
Port advanced GLib and retire the GLib backend replay

## Implement Phase ID
`impl-glib-advanced`

## Preexisting Inputs
- Pure-Rust `libgthread-2.0.so.0` / `libgthread-2.0.a` from `impl-glib-core`
- Pure-Rust `libgmodule-2.0.so.0` / `libgmodule-2.0.a` from `impl-glib-core`
- A greatly expanded Rust-owned `libglib-2.0.so.0` / `libglib-2.0.a` core surface from `impl-glib-core`
- Core ABI/layout parity verified from the safe build root by `impl-glib-core`
- `safe/crates/glib/src/hash/api.rs:396-970`
- `safe/crates/glib/src/gvariant/api.rs:229-280`
- `safe/crates/glib/src/markup/api.rs:57-184`
- `safe/crates/glib/src/charset/api.rs:82-144`
- `safe/crates/glib/src/spawn/api.rs:140-316`
- `safe/crates/glib/src/bytes/api.rs:11-20`
- `safe/crates/glib/src/fileutils/api.rs:65-71`
- `safe/crates/glib/src/keyfile/api.rs:18-36`
- `original/glib/gbookmarkfile.c`
- `original/glib/gcharset.c`
- `original/glib/gconvert.c`
- `original/glib/gfileutils.c`
- `original/glib/ghash.c`
- `original/glib/gkeyfile.c`
- `original/glib/gmarkup.c`
- `original/glib/gregex.c`
- `original/glib/gscanner.c`
- `original/glib/gspawn.c`
- `original/glib/gunicode*.c`
- `original/glib/guri.c`
- `original/glib/gvariant*.c`
- `relevant_cves.json`
- `safe/tests/cve/*.c`
- `safe/tests/manifests/glib-advanced.txt`
- `safe/tests/manifests/fuzzing.txt`

## New Outputs
- A pure-Rust `libglib-2.0.so.0` / `libglib-2.0.a` with no remaining dependency on replayed upstream GLib objects.
- Updated CVE documentation and regression coverage.

## File Changes
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
- `safe/crates/glib/src/backend.rs`
- `safe/crates/glib/src/bridge.rs`
- `safe/crates/glib/build.rs`
- `safe/tools/build-glib-backend.py`
- `safe/abi/version-scripts/libglib.map`
- `safe/abi/link-compat/glib-advanced.json`
- `safe/tests/upstream/glib/markup-collect.c`
- `safe/docs/cve-matrix.md`

## Implementation Details
- Expand the existing Rust interposition pattern until it owns the entire advanced GLib surface covered by `tests/manifests/glib-advanced.txt`.
- Preserve the already-started security semantics:
  - `g_str_hash()` remains ABI-compatible, while Rust-owned hash tables stay collision-resilient.
  - `g_markup_parse_context_end_parse()` remains total over failed states and preserves the critical-message expectation already patched into `safe/tests/upstream/glib/markup-collect.c`.
  - privileged charset handling clears `CHARSET`, `G_FILENAME_ENCODING`, and `G_BROKEN_FILENAMES`.
  - `g_byte_array_new_take()` rejects lengths above `G_MAXUINT`.
  - `g_variant_*` entry points validate layout and bound complexity before deep traversal.
  - Windows spawn wrappers reject overlong command lines.
- Retire the replayed backend produced by `safe/tools/build-glib-backend.py`. That tool can remain only as a differential oracle if still useful for tests, but `safe/crates/glib/build.rs` must stop linking replayed upstream GLib objects into the final library.
- Ensure the same retirement applies to the Cargo-produced static archive: `libglib-2.0.a` must not retain the replayed backend through `safe/crates/glib/build.rs`.
- Update `safe/docs/cve-matrix.md` to match the final Rust-owned implementations and exact regression commands.

## Verification Phases

### `check-glib-advanced-link`
- Phase ID: `check-glib-advanced-link`
- Type: `check`
- Fixed `bounce_target`: `impl-glib-advanced`
- Purpose: Prove source- and link-compatibility for the non-core GLib exports.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-glib-advanced --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-glib-advanced/.stamp
python3 tools/link-compat.py --phase glib-advanced --build-root build-glib-advanced --compile-original-objects --run
```

### `check-glib-advanced-tests`
- Phase ID: `check-glib-advanced-tests`
- Type: `check`
- Fixed `bounce_target`: `impl-glib-advanced`
- Purpose: Replay the frozen advanced GLib manifest against the safe build.
- Commands:
```bash
cd safe
python3 tools/build-abi-shell.py --build-root build-glib-advanced --multiarch "$(dpkg-architecture -qDEB_HOST_MULTIARCH)" --stamp build-glib-advanced/.stamp
python3 tools/run-meson-manifest.py --build-root build-glib-advanced --baseline abi/tests.json --path-map abi/test-source-path-map.json --intro-tests build-glib-advanced/meson-info/intro-tests.json --manifest tests/manifests/glib-advanced.txt --print-errorlogs
```

### `check-glib-cves`
- Phase ID: `check-glib-cves`
- Type: `check`
- Fixed `bounce_target`: `impl-glib-advanced`
- Purpose: Re-run all GLib-specific CVE probes and fuzz-manifest rows against the safe build.
- Commands:
```bash
cd safe
python3 tools/run-cve-regressions.py --phase glib --build-root build-glib-advanced --rebuild
```

## Success Criteria
- All three check phases pass.
- The committed `glib` shared and static libraries no longer depend on the replayed backend from `safe/tools/build-glib-backend.py`.
- CVE documentation and regression coverage are updated to match the Rust-owned behavior.
- `safe/crates/glib/src/bridge.rs` is gone or test-only, `safe/crates/glib/src/backend.rs` is not required to satisfy final exported symbols, and `safe/crates/glib/build.rs` no longer archives or links the backend generated by `safe/tools/build-glib-backend.py`.

## Git Commit Requirement
- The implementer must commit work to git before yielding.
- This phase must produce at least one new git commit before yielding to its verifiers.
- A verifier must treat an unchanged `HEAD` or a worktree-only deliverable as a failure.
