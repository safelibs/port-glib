#!/usr/bin/env python3
import argparse
import json
from pathlib import Path

from common import SAFE_ROOT, clean_subprocess_env, read_json, run, write_json


def probe_measurements() -> dict[str, dict[str, object]]:
    result = run(
        ["cargo", "run", "--quiet", "-p", "abi-support", "--bin", "layout-probe"],
        cwd=SAFE_ROOT,
        env=clean_subprocess_env(),
        capture=True,
    )
    payload = json.loads(result.stdout)
    measurements = {}
    for library in payload["libraries"]:
        measurements[library["library"]] = {
            entry["type_name"]: entry
            for entry in library["entries"]
        }
    return measurements


def verify_contract() -> None:
    run(
        ["cargo", "run", "--quiet", "-p", "abi-support", "--bin", "layout-probe", "--", "--verify-contract"],
        cwd=SAFE_ROOT,
        env=clean_subprocess_env(),
    )
    measurements = probe_measurements()
    for manifest_path in sorted((SAFE_ROOT / "abi" / "layout-manifests").glob("*.json")):
        manifest = read_json(manifest_path)
        library_measurements = measurements.get(manifest["library"])
        if library_measurements is None:
            raise ValueError(f"No Rust measurements were emitted for {manifest['library']}")
        for entry in manifest["entries"]:
            if not (entry.get("rust_probe_symbol") or entry.get("rust_type_path")):
                raise ValueError(f"{manifest_path.name} entry lacks a Rust-side probe contract: {entry['type_name']}")
            measured = library_measurements.get(entry["type_name"])
            if measured is None:
                raise ValueError(
                    f"{manifest_path.name} entry is missing from the Rust layout probe output: {entry['type_name']}"
                )
            if measured["kind"] != entry["kind"]:
                raise ValueError(
                    f"Rust layout probe kind mismatch for {manifest_path.name}:{entry['type_name']}"
                )


def export_layouts(output_dir: Path) -> None:
    measurements = probe_measurements()
    output_dir.mkdir(parents=True, exist_ok=True)
    for manifest_path in sorted((SAFE_ROOT / "abi" / "layout-manifests").glob("*.json")):
        manifest = read_json(manifest_path)
        library_measurements = measurements[manifest["library"]]
        write_json(
            output_dir / manifest_path.name,
            {
                "library": manifest["library"],
                "entries": [
                    library_measurements[entry["type_name"]]
                    for entry in manifest["entries"]
                ],
            },
        )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-contract", action="store_true")
    parser.add_argument("--output-dir", type=Path)
    args = parser.parse_args()
    if args.verify_contract:
        verify_contract()
        return
    if args.output_dir is None:
        raise SystemExit("--output-dir is required when not using --verify-contract")
    export_layouts(args.output_dir)


if __name__ == "__main__":
    main()
