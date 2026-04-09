#!/usr/bin/env python3
import argparse
import os
import subprocess
import sys
from pathlib import Path

from common import (
    SAFE_ROOT,
    PathNormalizer,
    contains_workspace_path,
    path_map_entries,
    read_json,
)


def verify_contract(baseline_path: Path, path_map_path: Path) -> None:
    baseline = read_json(baseline_path)
    path_map = read_json(path_map_path)
    if path_map != path_map_entries():
        raise ValueError("Path map does not match the frozen seven-root rewrite contract")

    by_row = {}
    by_triple = {}
    for row in baseline:
        if not row.get("suite"):
            raise ValueError(f"Test {row.get('name')!r} lacks a suite")
        row_key = (row["primary_suite"], row["name"])
        triple = (row["primary_suite"], row["name"], row["command_key"])
        if row_key in by_row:
            raise ValueError(f"Duplicate manifest row key: {row_key}")
        if triple in by_triple:
            raise ValueError(f"Duplicate manifest row/command key triple: {triple}")
        by_row[row_key] = row
        by_triple[triple] = row
        if contains_workspace_path(
            {
                "cmd_normalized_argv": row["cmd_normalized_argv"],
                "env_normalized": row["env_normalized"],
                "workdir_key": row["workdir_key"],
            }
        ):
            raise ValueError(f"Normalized row still contains repo-root workspace paths: {row_key}")
        if not row.get("command_key"):
            raise ValueError(f"Row lacks stable command_key: {row_key}")


def load_manifest_rows(path: Path) -> list[tuple[str, str]]:
    rows = []
    for line_number, raw_line in enumerate(path.read_text().splitlines(), start=1):
        if not raw_line:
            continue
        parts = raw_line.split("\t", 1)
        if len(parts) != 2:
            raise ValueError(f"{path}:{line_number} is not a primary_suite<TAB>test_name row")
        rows.append((parts[0], parts[1]))
    return rows


def normalize_current_tests(intro_tests_path: Path, build_root: Path, path_map: list[dict[str, str]]) -> list[dict[str, object]]:
    intro_tests = read_json(intro_tests_path)
    normalizer = PathNormalizer(
        build_root=build_root,
        original_root=SAFE_ROOT / "vendor" / "original",
        path_map=path_map,
    )
    records = []
    seen_triples = set()
    for index, row in enumerate(intro_tests):
        suite = row.get("suite") or []
        if not suite:
            raise ValueError(f"Current-build test {row.get('name')!r} lacks a suite")
        primary_suite = suite[0]
        descriptor = normalizer.command_descriptor(
            row.get("cmd", []),
            row.get("env", {}),
            row.get("workdir"),
        )
        triple = (primary_suite, row["name"], descriptor["command_key"])
        if triple in seen_triples:
            raise ValueError(f"Current-build test inventory is ambiguous for {triple}")
        seen_triples.add(triple)
        record = {
            "index": index,
            "name": row["name"],
            "suite": suite,
            "primary_suite": primary_suite,
            "timeout": row.get("timeout"),
            "protocol": row.get("protocol"),
            "raw_cmd": row.get("cmd", []),
            "raw_env": row.get("env", {}),
            "raw_workdir": row.get("workdir"),
            **descriptor,
        }
        if contains_workspace_path(
            {
                "cmd_normalized_argv": record["cmd_normalized_argv"],
                "env_normalized": record["env_normalized"],
                "workdir_key": record["workdir_key"],
            }
        ):
            raise ValueError(
                "Current-build normalized test row still embeds repo-root workspace paths: "
                f"{(primary_suite, row['name'])}"
            )
        records.append(record)
    return records


def execute_row(row: dict[str, object], build_root: Path, *, print_errorlogs: bool) -> None:
    cmd = [str(item) for item in row["raw_cmd"]]
    env = os.environ.copy()
    env.update({str(key): str(value) for key, value in row["raw_env"].items()})
    cwd = Path(str(row["raw_workdir"])) if row["raw_workdir"] else build_root
    timeout = row.get("timeout")
    completed = subprocess.run(
        cmd,
        cwd=str(cwd),
        env=env,
        text=True,
        capture_output=True,
        timeout=int(timeout) if timeout else None,
        check=False,
    )
    if completed.returncode == 0:
        return

    if print_errorlogs:
        if completed.stdout:
            sys.stderr.write(completed.stdout)
        if completed.stderr:
            sys.stderr.write(completed.stderr)
    raise SystemExit(
        f"Test failed: {row['primary_suite']}\t{row['name']} "
        f"(exit {completed.returncode})"
    )


def run_manifest(
    *,
    baseline_path: Path,
    path_map_path: Path,
    build_root: Path,
    intro_tests_path: Path,
    manifest_path: Path,
    print_errorlogs: bool,
) -> None:
    baseline = read_json(baseline_path)
    path_map = read_json(path_map_path)
    if path_map != path_map_entries():
        raise ValueError("Path map does not match the frozen seven-root rewrite contract")
    current = normalize_current_tests(intro_tests_path, build_root, path_map)

    baseline_by_row = {
        (row["primary_suite"], row["name"]): row
        for row in baseline
    }
    current_by_triple = {
        (row["primary_suite"], row["name"], row["command_key"]): row
        for row in current
    }

    for primary_suite, test_name in load_manifest_rows(manifest_path):
        baseline_row = baseline_by_row.get((primary_suite, test_name))
        if baseline_row is None:
            raise ValueError(
                f"Manifest row {primary_suite!r}\\t{test_name!r} is not present in abi/tests.json"
            )

        current_row = current_by_triple.get(
            (primary_suite, test_name, baseline_row["command_key"])
        )
        if current_row is None:
            raise ValueError(
                "No current-build test matches the frozen manifest row "
                f"{primary_suite!r}\\t{test_name!r} with command key {baseline_row['command_key']!r}"
            )

        if current_row["cmd_normalized_argv"] != baseline_row["cmd_normalized_argv"]:
            raise ValueError(
                f"Current-build argv drifted for {primary_suite!r}\\t{test_name!r}"
            )
        if current_row["env_normalized"] != baseline_row["env_normalized"]:
            raise ValueError(
                f"Current-build environment drifted for {primary_suite!r}\\t{test_name!r}"
            )
        if current_row["workdir_key"] != baseline_row["workdir_key"]:
            raise ValueError(
                f"Current-build workdir drifted for {primary_suite!r}\\t{test_name!r}"
            )

        execute_row(current_row, build_root, print_errorlogs=print_errorlogs)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-contract", action="store_true")
    parser.add_argument("--baseline", type=Path, default=SAFE_ROOT / "abi" / "tests.json")
    parser.add_argument("--path-map", type=Path, default=SAFE_ROOT / "abi" / "test-source-path-map.json")
    parser.add_argument("--build-root", type=Path)
    parser.add_argument("--intro-tests", type=Path)
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--print-errorlogs", action="store_true")
    args = parser.parse_args()
    if args.verify_contract:
        verify_contract(args.baseline, args.path_map)
        return
    if args.build_root is None or args.manifest is None:
        raise SystemExit("--build-root and --manifest are required when not using --verify-contract")
    intro_tests = args.intro_tests or args.build_root / "meson-info" / "intro-tests.json"
    run_manifest(
        baseline_path=args.baseline,
        path_map_path=args.path_map,
        build_root=args.build_root.resolve(),
        intro_tests_path=intro_tests,
        manifest_path=args.manifest,
        print_errorlogs=args.print_errorlogs,
    )


if __name__ == "__main__":
    main()
