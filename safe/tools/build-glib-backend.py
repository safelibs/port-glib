#!/usr/bin/env python3
import argparse
import json
import shlex
import shutil
import subprocess
import sys
from pathlib import Path


BACKEND_OVERRIDE_SYMBOLS = [
    "g_hash_table_new",
    "g_hash_table_new_full",
    "g_hash_table_new_similar",
    "g_hash_table_destroy",
    "g_hash_table_insert",
    "g_hash_table_replace",
    "g_hash_table_add",
    "g_hash_table_remove",
    "g_hash_table_remove_all",
    "g_hash_table_steal",
    "g_hash_table_steal_extended",
    "g_hash_table_steal_all",
    "g_hash_table_steal_all_keys",
    "g_hash_table_steal_all_values",
    "g_hash_table_lookup",
    "g_hash_table_contains",
    "g_hash_table_lookup_extended",
    "g_hash_table_foreach",
    "g_hash_table_find",
    "g_hash_table_foreach_remove",
    "g_hash_table_foreach_steal",
    "g_hash_table_size",
    "g_hash_table_get_keys",
    "g_hash_table_get_values",
    "g_hash_table_get_keys_as_array",
    "g_hash_table_get_keys_as_ptr_array",
    "g_hash_table_get_values_as_ptr_array",
    "g_hash_table_iter_init",
    "g_hash_table_iter_next",
    "g_hash_table_iter_get_hash_table",
    "g_hash_table_iter_remove",
    "g_hash_table_iter_replace",
    "g_hash_table_iter_steal",
    "g_hash_table_ref",
    "g_hash_table_unref",
    "g_str_equal",
    "g_str_hash",
    "g_canonicalize_filename",
    "g_key_file_load_from_data",
]

LEGACY_SHIM_SYMBOLS = [
    "g_byte_array_new_take",
    "g_get_charset",
    "g_get_filename_charsets",
    "g_locale_to_utf8",
    "g_locale_from_utf8",
    "g_filename_to_utf8",
    "g_filename_from_utf8",
    "g_filename_display_name",
    "g_filename_display_basename",
    "g_spawn_async",
    "g_spawn_async_with_fds",
    "g_spawn_async_with_pipes",
    "g_spawn_async_with_pipes_and_fds",
    "g_spawn_command_line_async",
    "g_spawn_command_line_sync",
    "g_spawn_sync",
    "g_variant_new_from_bytes",
    "g_variant_new_from_data",
    "g_variant_is_normal_form",
    "g_variant_get_normal_form",
    "g_variant_byteswap",
    "g_markup_parse_context_new",
    "g_markup_parse_context_ref",
    "g_markup_parse_context_unref",
    "g_markup_parse_context_free",
    "g_markup_parse_context_parse",
    "g_markup_parse_context_end_parse",
]

INTERNAL_SUPPORT_SYMBOLS = [
    *LEGACY_SHIM_SYMBOLS,
]

LEGACY_PROVIDER_SYMBOLS = [
    "g_byte_array_new_take",
    "g_get_charset",
    "g_get_filename_charsets",
    "g_locale_to_utf8",
    "g_locale_from_utf8",
    "g_filename_to_utf8",
    "g_filename_from_utf8",
    "g_filename_display_name",
    "g_filename_display_basename",
    "g_spawn_async",
    "g_spawn_async_with_fds",
    "g_spawn_async_with_pipes",
    "g_spawn_async_with_pipes_and_fds",
    "g_spawn_command_line_async",
    "g_spawn_command_line_sync",
    "g_spawn_sync",
    "g_variant_new_from_bytes",
    "g_variant_new_from_data",
    "g_variant_is_normal_form",
    "g_variant_get_normal_form",
    "g_variant_byteswap",
    "g_markup_parse_context_new",
    "g_markup_parse_context_ref",
    "g_markup_parse_context_unref",
    "g_markup_parse_context_free",
    "g_markup_parse_context_parse",
    "g_markup_parse_context_end_parse",
]


def run(cmd: list[str], cwd: Path, failure: str) -> None:
    result = subprocess.run(cmd, cwd=cwd, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stderr.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise SystemExit(f"{failure}: {' '.join(cmd)} exited with {result.returncode}")


def nm_defined_symbols(path: Path) -> list[str]:
    result = subprocess.run(
        ["nm", "-g", "--defined-only", str(path)],
        text=True,
        capture_output=True,
        check=True,
    )
    symbols = []
    for line in result.stdout.splitlines():
        parts = line.split()
        if parts:
            symbols.append(parts[-1])
    return symbols


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--build-check-dir", required=True)
    parser.add_argument("--out-dir", required=True)
    return parser.parse_args()


def support_symbol(symbol: str) -> str:
    return f"safe_glib_backend_{symbol}"


def legacy_symbol(symbol: str) -> str:
    return f"safe_glib_legacy_{symbol}"


def sanitize_arguments(args: list[str], object_out: Path) -> list[str]:
    sanitized: list[str] = []
    it = iter(range(len(args)))
    replaced_output = False
    index = 0
    while index < len(args):
        arg = args[index]
        if arg == "-o":
            index += 1
            if index >= len(args):
                raise SystemExit("compile command missing -o value")
            sanitized.extend(["-o", str(object_out)])
            replaced_output = True
        elif arg in {"-MF", "-MQ", "-MT"}:
            index += 1
            if index >= len(args):
                raise SystemExit("compile command missing depfile value")
        elif arg in {"-MD", "-MMD"}:
            pass
        elif arg.startswith("-MF") or arg.startswith("-MQ") or arg.startswith("-MT") or arg == "-Winvalid-pch":
            pass
        else:
            sanitized.append(arg)
        index += 1

    if not replaced_output:
        raise SystemExit("compile command did not specify an output path")
    return sanitized


def rename_symbols(path: Path, build_check_dir: Path) -> None:
    backend_overrides = {symbol: support_symbol(symbol) for symbol in BACKEND_OVERRIDE_SYMBOLS}
    legacy_shims = {symbol: legacy_symbol(symbol) for symbol in LEGACY_SHIM_SYMBOLS}
    internal = {symbol: support_symbol(symbol) for symbol in INTERNAL_SUPPORT_SYMBOLS}
    redefine_args: list[str] = []

    for symbol, target in internal.items():
        if symbol in legacy_shims:
            target = legacy_shims[symbol]
        redefine_args.extend(["--redefine-sym", f"{symbol}={target}"])

    for symbol in nm_defined_symbols(path):
        target = backend_overrides.get(symbol)
        if target is None or symbol in internal:
            continue
        redefine_args.extend(["--redefine-sym", f"{symbol}={target}"])

    if redefine_args:
        run(["objcopy", *redefine_args, str(path)], build_check_dir, "failed to rename backend symbols")


def collect_commands(build_check_dir: Path) -> list[dict]:
    compile_commands = build_check_dir / "compile_commands.json"
    entries = json.loads(compile_commands.read_text())
    selected = [
        entry
        for entry in entries
        if entry.get("output", "").startswith("glib/libglib-2.0.so.0.8000.0.p/")
        or entry.get("output", "").startswith("glib/libcharset/libcharset.a.p/")
    ]
    selected.sort(key=lambda entry: entry.get("output", ""))
    if len(selected) != 96:
        raise SystemExit(f"expected 96 frozen GLib compile commands, found {len(selected)}")
    return selected


def parse_command(entry: dict) -> tuple[str, list[str]]:
    if entry.get("arguments"):
        arguments = entry["arguments"]
        return arguments[0], arguments[1:]
    command = entry.get("command")
    if not command:
        raise SystemExit("compile command missing command text")
    parts = shlex.split(command)
    if not parts:
        raise SystemExit("compile command missing program")
    return parts[0], parts[1:]


def assert_override_coverage(objects: list[Path]) -> None:
    expected = {support_symbol(symbol) for symbol in BACKEND_OVERRIDE_SYMBOLS}
    expected.update(legacy_symbol(symbol) for symbol in LEGACY_PROVIDER_SYMBOLS)
    seen: set[str] = set()
    for obj in objects:
        seen.update(nm_defined_symbols(obj))
    missing = sorted(expected - seen)
    if missing:
        raise SystemExit(f"failed to rebuild override providers for symbols: {', '.join(missing)}")


def main() -> None:
    args = parse_args()
    build_check_dir = Path(args.build_check_dir).resolve()
    out_dir = Path(args.out_dir).resolve()
    replay_root = out_dir / "backend-objects"
    backend_object = out_dir / "safe_glib_backend.o"

    if replay_root.exists():
        shutil.rmtree(replay_root)
    replay_root.mkdir(parents=True, exist_ok=True)
    if backend_object.exists():
        backend_object.unlink()

    objects: list[Path] = []
    for entry in collect_commands(build_check_dir):
        output = entry["output"]
        object_out = replay_root / output
        object_out.parent.mkdir(parents=True, exist_ok=True)
        program, command_args = parse_command(entry)
        sanitized = sanitize_arguments(command_args, object_out)
        run([program, *sanitized], build_check_dir, f"failed to replay {output}")
        rename_symbols(object_out, build_check_dir)
        objects.append(object_out)

    assert_override_coverage(objects)
    run(
        ["ld", "-r", "-o", str(backend_object), *[str(obj) for obj in objects]],
        build_check_dir,
        "failed to link backend object",
    )
    print(backend_object)


if __name__ == "__main__":
    main()
