#!/usr/bin/env python3
import argparse
import re
import sys
import tempfile
from pathlib import Path

from common import (
    PathNormalizer,
    REPO_ROOT,
    SAFE_ROOT,
    VENDOR_BUILD_CHECK,
    VENDOR_ORIGINAL,
    VENDOR_WORKSPACE,
    clean_subprocess_env,
    compare_jsonish,
    contains_workspace_path,
    ensure_dir,
    normpath,
    path_map_entries,
    placeholder_origin_relpath,
    read_json,
    run,
    series_lines,
    write_json,
    write_text,
    parse_deb822_file,
)


CORE_GLIB_TESTS = [
    "array-test",
    "asyncqueue",
    "atomic",
    "base64",
    "bitlock",
    "cache",
    "checksum",
    "completion",
    "cond",
    "dataset",
    "error",
    "guuid",
    "gwakeup",
    "hmac",
    "hook",
    "list",
    "logging",
    "macros",
    "mainloop",
    "mapping",
    "memchunk",
    "mem-overflow",
    "mutex",
    "node",
    "once",
    "onceinit",
    "overflow",
    "overflow-fallback",
    "queue",
    "rand",
    "rcbox",
    "rec-mutex",
    "refcount",
    "refcount-macro",
    "refstring",
    "relation",
    "rwlock",
    "sequence",
    "slice",
    "slist",
    "sort",
    "strfuncs",
    "string",
    "strvbuilder",
    "testing",
    "test-printf",
    "thread",
    "thread-deprecated",
    "thread-pool",
    "thread-pool-slow",
    "timeout",
    "timer",
    "tree",
    "types",
    "utf8-performance",
    "utf8-pointer",
    "utf8-validate",
    "utf8-misc",
    "utils",
    "utils-isolated",
    "1bit-mutex",
    "1bit-emufutex",
    "642026",
    "642026-ec",
    "autoptr",
    "gwakeup-fallback",
    "constructor",
]

FUZZING_ROWS = [
    "fuzz_bookmark",
    "fuzz_canonicalize_filename",
    "fuzz_date_parse",
    "fuzz_date_time_new_from_iso8601",
    "fuzz_dbus_message",
    "fuzz_inet_address_mask_new_from_string",
    "fuzz_inet_address_new_from_string",
    "fuzz_inet_socket_address_new_from_string",
    "fuzz_key",
    "fuzz_network_address_parse",
    "fuzz_network_address_parse_uri",
    "fuzz_paths",
    "fuzz_resolver",
    "fuzz_uri_escape",
    "fuzz_uri_parse",
    "fuzz_uri_parse_params",
    "fuzz_utf8_normalize",
    "fuzz_utf8_validate",
    "fuzz_uuid_string_is_valid",
    "fuzz_variant_binary",
    "fuzz_variant_binary_byteswap",
    "fuzz_variant_text",
]

ADVANCED_GLIB_PREFIXES = (
    "g_charset_",
    "g_convert",
    "g_hash_",
    "g_markup_",
    "g_regex_",
    "g_shell_",
    "g_spawn_",
    "g_uri_",
    "g_variant_",
)

CONTROL_EXACT_SOURCE_FIELDS = {
    "Source",
    "Section",
    "Priority",
    "Maintainer",
    "XSBC-Original-Maintainer",
    "Uploaders",
    "Rules-Requires-Root",
    "Standards-Version",
    "Homepage",
    "Vcs-Browser",
    "Vcs-Git",
}
CONTROL_REGENERABLE_SOURCE_FIELDS = {
    "Build-Depends",
    "Build-Depends-Arch",
    "Build-Depends-Indep",
}
CONTROL_EXACT_BINARY_FIELDS = {
    "Package",
    "Architecture",
    "Section",
    "Package-Type",
    "Build-Profiles",
    "Multi-Arch",
    "Depends",
    "Recommends",
    "Suggests",
    "Provides",
    "Breaks",
    "Replaces",
}
REQUIRED_BINARY_STANZAS = [
    "libglib2.0-0t64",
    "libglib2.0-tests",
    "libglib2.0-udeb",
    "libglib2.0-bin",
    "libglib2.0-dev",
    "libglib2.0-dev-bin",
    "libglib2.0-data",
    "libglib2.0-doc",
    "libgirepository-2.0-0",
    "libgirepository-2.0-dev",
    "gir1.2-glib-2.0",
    "gir1.2-glib-2.0-dev",
    "gir1.2-girepository-3.0",
    "gir1.2-girepository-3.0-dev",
]
SHARED_LIBRARY_MAP = {
    "libglib-2.0.so.0": "glib",
    "libgthread-2.0.so.0": "gthread",
    "libgmodule-2.0.so.0": "gmodule",
    "libgobject-2.0.so.0": "gobject",
    "libgio-2.0.so.0": "gio",
    "libgirepository-2.0.so.0": "girepository",
}

HEADER_SEARCH_GROUPS = {
    "libglib-2.0.so.0": ["glib", "gthread"],
    "libgthread-2.0.so.0": ["gthread", "glib"],
    "libgmodule-2.0.so.0": ["gmodule", "glib"],
    "libgobject-2.0.so.0": ["gobject", "glib"],
    "libgio-2.0.so.0": ["gio"],
    "libgirepository-2.0.so.0": ["girepository"],
}

CVE_MAP = {
    "CVE-2009-3289": {
        "files": ["original/gio/gfile.c", "original/gio/tests/file.c"],
        "api": "g_file_copy()",
        "planned_regression": "safe/tests/manifests/gio.txt :: file",
    },
    "CVE-2012-0039": {
        "files": ["original/glib/ghash.c"],
        "api": "g_str_hash(), g_hash_table_new()",
        "planned_regression": "safe/tools/run-cve-regressions.py :: hash-collision",
    },
    "CVE-2018-16428": {
        "files": ["original/glib/gmarkup.c"],
        "api": "g_markup_parse_context_end_parse()",
        "planned_regression": "safe/tests/manifests/glib-advanced.txt :: markup*",
    },
    "CVE-2019-12450": {
        "files": ["original/gio/gfile.c", "original/gio/tests/file.c"],
        "api": "g_file_copy() fallback path",
        "planned_regression": "safe/tests/manifests/gio.txt :: file",
    },
    "CVE-2019-13012": {
        "files": ["original/gio/gkeyfilesettingsbackend.c"],
        "api": "GKeyfileSettingsBackend",
        "planned_regression": "safe/tools/run-cve-regressions.py :: keyfile-settings-backend",
    },
    "CVE-2020-6750": {
        "files": ["original/gio/gsocketclient.c"],
        "api": "GSocketClient proxy routing",
        "planned_regression": "safe/tests/manifests/gio.txt :: socket*",
    },
    "CVE-2021-3800": {
        "files": ["original/glib/gcharset.c", "original/glib/gconvert.c"],
        "api": "charset alias and conversion environment handling",
        "planned_regression": "safe/tests/manifests/glib-advanced.txt :: charset, convert",
    },
    "CVE-2021-27218": {
        "files": ["original/glib/garray.c"],
        "api": "g_byte_array_new_take()",
        "planned_regression": "safe/tests/manifests/glib-core.txt :: array-test",
    },
    "CVE-2021-28153": {
        "files": ["original/gio/glocalfileoutputstream.c", "original/gio/tests/file.c"],
        "api": "g_file_replace() with G_FILE_CREATE_REPLACE_DESTINATION",
        "planned_regression": "safe/tests/manifests/gio.txt :: file",
    },
    "CVE-2023-29499": {
        "files": [
            "original/glib/gvariant-core.c",
            "original/glib/gvariant-serialiser.c",
            "original/glib/gvariant-parser.c",
        ],
        "api": "GVariant deserialization",
        "planned_regression": "safe/tests/manifests/glib-advanced.txt :: gvariant + fuzzing manifest",
    },
    "CVE-2023-32611": {
        "files": ["original/glib/gvariant-core.c", "original/fuzzing/fuzz_variant_binary.c"],
        "api": "GVariant complexity limits",
        "planned_regression": "safe/tests/manifests/fuzzing.txt :: fuzz_variant_binary",
    },
    "CVE-2023-32636": {
        "files": ["original/glib/gvariant-core.c", "original/glib/gvariant-serialiser.c"],
        "api": "GVariant offset-table validation",
        "planned_regression": "safe/tests/manifests/fuzzing.txt :: fuzz_variant_binary_byteswap",
    },
    "CVE-2023-32665": {
        "files": ["original/glib/gvariant.c", "original/fuzzing/fuzz_variant_text.c"],
        "api": "GVariant nested text/binary handling",
        "planned_regression": "safe/tests/manifests/fuzzing.txt :: fuzz_variant_text",
    },
    "CVE-2024-34397": {
        "files": ["original/gio/gdbusconnection.c", "original/gio/tests/gdbus-subscribe.c"],
        "api": "GDBus signal subscription sender validation",
        "planned_regression": "safe/tests/manifests/gio.txt :: gdbus-subscribe",
    },
    "CVE-2025-4056": {
        "files": ["original/glib/gspawn.c", "original/glib/tests/spawn-test.c"],
        "api": "Windows long command-line spawning",
        "planned_regression": "safe/tests/manifests/glib-advanced.txt :: spawn-*",
    },
}


def replace_for_replay(value: str) -> str:
    replacements = {
        str(REPO_ROOT / "build-check"): "$SAFE_VENDOR_BUILD_CHECK",
        str(REPO_ROOT / "original"): "$SAFE_VENDOR_ORIGINAL",
        str(SAFE_ROOT / "vendor" / "build-check"): "$SAFE_VENDOR_BUILD_CHECK",
        str(SAFE_ROOT / "vendor" / "original"): "$SAFE_VENDOR_ORIGINAL",
    }
    if value.startswith("-I") and len(value) > 2:
        return "-I" + replace_for_replay(value[2:])
    if value.startswith("-L") and len(value) > 2:
        return "-L" + replace_for_replay(value[2:])
    if any(src in value for src in replacements):
        for src, dst in replacements.items():
            value = value.replace(src, dst)
        return value
    if value.startswith("-"):
        return value
    value = normpath(value)
    for src, dst in replacements.items():
        if value == src:
            return dst
        if value.startswith(src + "/"):
            return dst + value[len(src) :]
    relative_lib_rewrites = {
        "glib/libglib-2.0.so.0.8000.0": "$BUILD_ROOT/glib/libglib-2.0.so.0",
        "gthread/libgthread-2.0.so.0.8000.0": "$BUILD_ROOT/gthread/libgthread-2.0.so.0",
        "gmodule/libgmodule-2.0.so.0.8000.0": "$BUILD_ROOT/gmodule/libgmodule-2.0.so.0",
        "gobject/libgobject-2.0.so.0.8000.0": "$BUILD_ROOT/gobject/libgobject-2.0.so.0",
        "gio/libgio-2.0.so.0.8000.0": "$BUILD_ROOT/gio/libgio-2.0.so.0",
        "girepository/libgirepository-2.0.so.0.8000.0": "$BUILD_ROOT/girepository/libgirepository-2.0.so.0",
    }
    if value in relative_lib_rewrites:
        return relative_lib_rewrites[value]
    return value


def build_ninja_link_objects() -> dict[str, list[str]]:
    build_root = REPO_ROOT / "build-check"
    logical_lines: list[str] = []
    current = ""
    for raw_line in (VENDOR_BUILD_CHECK / "build.ninja").read_text().splitlines():
        if current:
            current += raw_line
        else:
            current = raw_line
        if current.endswith("$"):
            current = current[:-1]
            continue
        logical_lines.append(current)
        current = ""

    mapping: dict[str, list[str]] = {}
    for line in logical_lines:
        if not line.startswith("build ") or ": c_LINKER " not in line:
            continue
        output, explicit = line[len("build ") :].split(": c_LINKER ", 1)
        tokens = explicit.split(" | ", 1)[0].split()
        objects = [
            replace_for_replay(str(build_root / token))
            for token in tokens
            if token.endswith(".o") and ".p/" not in token
        ]
        if objects:
            mapping[str(build_root / output)] = objects
    return mapping


def public_header_library(source_path: str, install_path: str) -> str:
    install_rel = install_path.replace("/usr/local/include/glib-2.0/", "")
    if install_rel.startswith("gobject/"):
        return "gobject"
    if install_rel.startswith("gio/"):
        return "gio"
    if install_rel.startswith("gmodule/"):
        return "gmodule"
    if install_rel.startswith("girepository/"):
        return "girepository"
    if install_rel in {
        "glib/gthread.h",
        "glib/deprecated/gthread.h",
    }:
        return "gthread"
    if source_path.endswith("/gthread.h"):
        return "gthread"
    return "glib"


def parse_symbol_sections(path: Path) -> dict[str, list[str]]:
    sections: dict[str, list[str]] = {}
    current: str | None = None
    for raw_line in path.read_text().splitlines():
        if raw_line.startswith("lib") and "#MINVER#" in raw_line:
            current = raw_line.split()[0]
            sections[current] = [raw_line]
            continue
        if current is None:
            continue
        sections[current].append(raw_line)
    return sections


def build_symbols() -> None:
    abi_dir = SAFE_ROOT / "abi" / "symbols"
    ensure_dir(abi_dir)
    package_symbol_files = [
        "libglib2.0-0t64.symbols",
        "libgirepository-2.0-0.symbols",
    ]
    for name in package_symbol_files:
        source = VENDOR_ORIGINAL / "debian" / name
        destination = abi_dir / name
        destination.write_text(source.read_text())

    sections = parse_symbol_sections(VENDOR_ORIGINAL / "debian" / "libglib2.0-0t64.symbols")
    for library, lines in sections.items():
        write_text(abi_dir / f"{library}.symbols", "\n".join(lines).rstrip() + "\n")
    gir_sections = parse_symbol_sections(VENDOR_ORIGINAL / "debian" / "libgirepository-2.0-0.symbols")
    for library, lines in gir_sections.items():
        write_text(abi_dir / f"{library}.symbols", "\n".join(lines).rstrip() + "\n")


def build_test_source_path_map() -> list[dict[str, str]]:
    entries = path_map_entries()
    write_json(SAFE_ROOT / "abi" / "test-source-path-map.json", entries)
    return entries


def normalize_tests(path_map: list[dict[str, str]]) -> list[dict[str, object]]:
    intro_tests = read_json(VENDOR_BUILD_CHECK / "meson-info" / "intro-tests.json")
    normalizer = PathNormalizer(
        build_root=REPO_ROOT / "build-check",
        original_root=REPO_ROOT / "original",
        path_map=path_map,
    )
    records = []
    seen_pairs = set()
    seen_triples = set()
    for index, row in enumerate(intro_tests):
        suite = row.get("suite") or []
        if not suite:
            raise ValueError(f"Test {row.get('name')!r} lacks a suite")
        primary_suite = suite[0]
        descriptor = normalizer.command_descriptor(
            row.get("cmd", []),
            row.get("env", {}),
            row.get("workdir"),
        )
        pair = (primary_suite, row["name"])
        triple = (primary_suite, row["name"], descriptor["command_key"])
        if pair in seen_pairs:
            raise ValueError(f"Duplicate primary_suite/name pair: {pair}")
        if triple in seen_triples:
            raise ValueError(f"Duplicate primary_suite/name/command_key triple: {triple}")
        seen_pairs.add(pair)
        seen_triples.add(triple)

        record = {
            "index": index,
            "name": row["name"],
            "suite": suite,
            "primary_suite": primary_suite,
            "protocol": row.get("protocol"),
            "timeout": row.get("timeout"),
            "depends": row.get("depends", []),
            **descriptor,
            "provenance": {
                "cmd": row.get("cmd", []),
                "env": row.get("env", {}),
                "workdir": row.get("workdir"),
            },
        }
        if contains_workspace_path(
            {
                "cmd_normalized_argv": record["cmd_normalized_argv"],
                "env_normalized": record["env_normalized"],
                "workdir_key": record["workdir_key"],
            }
        ):
            raise ValueError(f"Normalized test row still embeds workspace paths: {row['name']}")
        records.append(record)

    write_json(SAFE_ROOT / "abi" / "tests.json", records)
    return records


def build_test_manifests(records: list[dict[str, object]]) -> None:
    manifest_dir = SAFE_ROOT / "tests" / "manifests"
    ensure_dir(manifest_dir)

    full_rows = [f"{row['primary_suite']}\t{row['name']}" for row in records]
    write_text(manifest_dir / "full.txt", "\n".join(full_rows) + "\n")

    fuzz_rows = [
        f"{row['primary_suite']}\t{row['name']}"
        for row in records
        if row["primary_suite"] == "glib:fuzzing" and row["name"] in FUZZING_ROWS
    ]
    expected_fuzz_rows = [f"glib:fuzzing\t{name}" for name in FUZZING_ROWS]
    if fuzz_rows != expected_fuzz_rows:
        raise ValueError("The frozen fuzzing manifest does not match the required 22-row inventory")
    write_text(manifest_dir / "fuzzing.txt", "\n".join(fuzz_rows) + "\n")

    core_rows = []
    advanced_rows = []
    gobject_rows = []
    gio_rows = []
    girepository_rows = []

    for row in records:
        manifest_row = f"{row['primary_suite']}\t{row['name']}"
        if row["primary_suite"] == "glib:glib":
            if row["name"] in CORE_GLIB_TESTS:
                core_rows.append(manifest_row)
            else:
                advanced_rows.append(manifest_row)
        if row["primary_suite"] == "glib:gthread":
            core_rows.append(manifest_row)
        if row["primary_suite"] == "glib:gmodule":
            core_rows.append(manifest_row)
        if "glib:gobject" in row["suite"]:
            gobject_rows.append(manifest_row)
        if "glib:gio" in row["suite"]:
            gio_rows.append(manifest_row)
        if "glib:girepository" in row["suite"]:
            girepository_rows.append(manifest_row)

    write_text(manifest_dir / "glib-core.txt", "\n".join(core_rows) + "\n")
    write_text(manifest_dir / "glib-advanced.txt", "\n".join(advanced_rows) + "\n")
    write_text(manifest_dir / "gobject.txt", "\n".join(gobject_rows) + "\n")
    write_text(manifest_dir / "gio.txt", "\n".join(gio_rows) + "\n")
    write_text(manifest_dir / "girepository.txt", "\n".join(girepository_rows) + "\n")


def normalize_installed_files() -> list[dict[str, object]]:
    intro_installed = read_json(VENDOR_BUILD_CHECK / "meson-info" / "intro-installed.json")
    normalizer = PathNormalizer(
        build_root=REPO_ROOT / "build-check",
        original_root=REPO_ROOT / "original",
    )
    entries = []
    for source, install_value in intro_installed.items():
        install_paths = install_value if isinstance(install_value, list) else [install_value]
        for install_path in install_paths:
            entries.append(
                {
                    "source": normalizer.normalize_token(str(source)),
                    "install_path": install_path,
                }
            )
    entries.sort(key=lambda item: (item["install_path"], item["source"]))
    write_json(SAFE_ROOT / "abi" / "installed-files.json", {"entries": entries})
    return entries


def build_control_preservation() -> None:
    stanzas = parse_deb822_file(VENDOR_ORIGINAL / "debian" / "control")
    if not stanzas or stanzas[0].get("Source") != "glib2.0":
        raise ValueError("Unexpected source stanza in debian/control")
    binary_names = [stanza.get("Package") for stanza in stanzas[1:]]
    for package in REQUIRED_BINARY_STANZAS:
        if package not in binary_names:
            raise ValueError(f"Required binary stanza missing from debian/control: {package}")

    manifest_stanzas = []
    for index, stanza in enumerate(stanzas):
        kind = "source" if index == 0 else "binary"
        name = stanza.get("Source") if kind == "source" else stanza.get("Package")
        fields = {}
        for field, value in stanza.items():
            if kind == "source":
                if field in CONTROL_EXACT_SOURCE_FIELDS:
                    mode = "exact-after-normalization"
                elif field in CONTROL_REGENERABLE_SOURCE_FIELDS:
                    mode = "regenerable-with-rationale"
                else:
                    mode = "preserve-for-provenance"
            else:
                if field in CONTROL_EXACT_BINARY_FIELDS:
                    mode = "exact-after-normalization"
                else:
                    mode = "preserve-for-provenance"
            fields[field] = {
                "mode": mode,
                "value": value,
            }
        manifest_stanzas.append(
            {
                "kind": kind,
                "name": name,
                "fields": fields,
            }
        )
    write_json(
        SAFE_ROOT / "abi" / "debian-control-preservation.json",
        {
            "source_path": "safe/vendor/original/debian/control",
            "stanzas": manifest_stanzas,
        },
    )


def build_patch_manifest() -> None:
    entries = []
    for patch in series_lines(VENDOR_ORIGINAL / "debian" / "patches" / "series"):
        if "CVE-" in patch:
            category = "CVE-driven"
        elif patch.startswith("debian/"):
            category = "packaging"
        elif "workaround" in patch or "disable_" in patch or "Disable-" in patch or "Skip-" in patch:
            category = "workaround"
        else:
            category = "upstream-source"
        entries.append(
            {
                "patch": patch,
                "category": category,
                "expected_disposition": None,
                "active_queue": None,
                "regressions": [],
                "provenance": [],
            }
        )
    write_json(SAFE_ROOT / "abi" / "debian-patches.json", {"entries": entries})


def load_public_headers() -> list[dict[str, str]]:
    intro_installed = read_json(VENDOR_BUILD_CHECK / "meson-info" / "intro-installed.json")
    headers = []
    for source, install_value in intro_installed.items():
        install_paths = install_value if isinstance(install_value, list) else [install_value]
        for install_path in install_paths:
            if not install_path.startswith("/usr/local/include/") or not install_path.endswith(".h"):
                continue
            source_path = replace_for_replay(str(source))
            source_real = Path(str(source).replace(str(REPO_ROOT / "build-check"), str(VENDOR_BUILD_CHECK)).replace(str(REPO_ROOT / "original"), str(VENDOR_ORIGINAL)))
            if not source_real.exists():
                continue
            headers.append(
                {
                    "source_path": source_path,
                    "install_path": install_path,
                    "include": install_path.replace("/usr/local/include/glib-2.0/", ""),
                    "library": public_header_library(source_path, install_path),
                    "text": source_real.read_text(errors="ignore"),
                }
            )
    return headers


def build_link_compat(records: list[dict[str, object]]) -> None:
    abi_dir = SAFE_ROOT / "abi" / "link-compat"
    ensure_dir(abi_dir)
    headers = load_public_headers()
    header_index = {}
    for header in headers:
        header_index.setdefault(header["library"], []).append(header)

    unmatched = []
    generated_entries = []
    smoke_entries = []

    for soname, library_id in SHARED_LIBRARY_MAP.items():
        symbol_path = SAFE_ROOT / "abi" / "symbols" / f"{soname}.symbols"
        symbol_lines = symbol_path.read_text().splitlines()
        matched_entries = []
        search_headers = []
        for group in HEADER_SEARCH_GROUPS.get(soname, [library_id]):
            search_headers.extend(header_index.get(group, []))
        for line in symbol_lines:
            stripped = line.strip()
            if not stripped or stripped.startswith("*") or stripped.startswith("lib"):
                continue
            symbol = stripped.split("@", 1)[0]
            header_match = None
            for header in search_headers:
                if re.search(rf"\b{re.escape(symbol)}\b", header["text"]):
                    header_match = header
                    break
            if header_match is None:
                unmatched.append(
                    {
                        "library": soname,
                        "symbol": symbol,
                        "reason": "not declared in installed public headers",
                    }
                )
                continue
            entry_id = f"generated:{soname}:{symbol}"
            entry = {
                "id": entry_id,
                "kind": "generated_abi_consumer",
                "library": soname,
                "symbol": symbol,
                "header": {
                    "include": header_match["include"],
                    "install_path": header_match["install_path"],
                    "source_path": header_match["source_path"],
                },
                "translation_unit": {
                    "path": f"generated/{soname}/{symbol}.c",
                    "source": (
                        "#include <stdint.h>\n"
                        f"#include <{header_match['include']}>\n"
                        f"uintptr_t safe_consume_{re.sub(r'[^A-Za-z0-9_]', '_', symbol)}(void) {{\n"
                        f"    return (uintptr_t) &{symbol};\n"
                        "}\n"
                    ),
                },
                "compile": {
                    "include_dirs": [
                        "$SAFE_VENDOR_ORIGINAL",
                        "$SAFE_VENDOR_BUILD_CHECK",
                    ],
                    "cflags": ["-std=gnu99", "-D_GNU_SOURCE"],
                },
                "link": {
                    "expected": [f"$BUILD_ROOT/{library_id}/{soname}"],
                },
                "runnable": False,
            }
            generated_entries.append(entry)
            matched_entries.append(entry)

        if matched_entries:
            representative = matched_entries[0]
            smoke_entries.append(
                {
                    **representative,
                    "id": f"generated-smoke:{soname}",
                    "smoke": True,
                }
            )

    smoke_entries.append(
        {
            "id": "generated-smoke:libgirepository-2.0.so.0:package",
            "kind": "generated_abi_consumer",
            "library": "libgirepository-2.0.so.0",
            "symbol": "g_irepository_get_default",
            "header": {
                "include": "girepository/girepository.h",
                "install_path": "/usr/include/girepository/girepository.h",
                "source_path": "$SAFE_VENDOR_ORIGINAL/girepository/girepository.h",
            },
            "translation_unit": {
                "path": "generated/libgirepository-2.0.so.0/package-smoke.c",
                "source": (
                    "#include <girepository/girepository.h>\n"
                    "int main(void) {\n"
                    "    return g_irepository_get_default() == 0;\n"
                    "}\n"
                ),
            },
            "compile": {
                "pkg_config": ["girepository-2.0"],
                "cflags": ["-std=gnu99"],
            },
            "link": {
                "expected": ["$BUILD_ROOT/girepository/libgirepository-2.0.so.0"],
            },
            "runnable": True,
        }
    )

    debian_smokes = []
    for script in ["build", "build-static"]:
        debian_smokes.append(
            {
                "id": f"debian-build-smoke:{script}",
                "kind": "debian_build_smoke",
                "script": f"$SAFE_VENDOR_ORIGINAL/debian/tests/{script}",
                "runnable": True,
            }
        )

    intro_targets = read_json(VENDOR_BUILD_CHECK / "meson-info" / "intro-targets.json")
    target_index = {target["id"]: target for target in intro_targets}
    executable_targets = [target for target in intro_targets if target.get("type") == "executable"]
    executable_by_filename = {
        filename: target
        for target in executable_targets
        for filename in target.get("filename", [])
    }
    linker_objects = build_ninja_link_objects()
    upstream_entries = []
    for row in records:
        runtime_cmd = row.get("provenance", {}).get("cmd", [])
        target = None
        if runtime_cmd:
            target = executable_by_filename.get(runtime_cmd[0])

        candidates = [
            target_index[target_id]
            for target_id in row["depends"]
            if target_id in target_index and target_index[target_id].get("type") == "executable"
        ]
        if target is None:
            target = next(
                (
                    candidate
                    for candidate in candidates
                    if (
                        "/tests/" in replace_for_replay(candidate.get("defined_in", ""))
                        or replace_for_replay(candidate.get("defined_in", "")).startswith("$SAFE_VENDOR_ORIGINAL/fuzzing/")
                    )
                ),
                None,
            )
        if target is None:
            continue
        defined_in = replace_for_replay(target.get("defined_in", ""))
        if "/tests/" not in defined_in and not defined_in.startswith("$SAFE_VENDOR_ORIGINAL/fuzzing/"):
            continue
        compile_recipes = []
        link_recipe = None
        for source_info in target.get("target_sources", []):
            if "compiler" in source_info:
                compile_recipes.append(
                    {
                        "language": source_info.get("language"),
                        "compiler": [replace_for_replay(item) for item in source_info.get("compiler", [])],
                        "parameters": [replace_for_replay(item) for item in source_info.get("parameters", [])],
                        "sources": [replace_for_replay(item) for item in source_info.get("sources", [])],
                        "generated_sources": [
                            replace_for_replay(item)
                            for item in source_info.get("generated_sources", [])
                        ],
                    }
                )
            if "linker" in source_info:
                link_recipe = {
                    "linker": [replace_for_replay(item) for item in source_info.get("linker", [])],
                    "parameters": [replace_for_replay(item) for item in source_info.get("parameters", [])],
                }
                if target.get("filename"):
                    objects = linker_objects.get(target["filename"][0], [])
                    if objects:
                        link_recipe["objects"] = objects
        upstream_entries.append(
            {
                "id": f"upstream:{row['primary_suite']}:{row['name']}",
                "kind": "upstream_test_target",
                "primary_suite": row["primary_suite"],
                "name": row["name"],
                "target_id": target["id"],
                "defined_in": defined_in,
                "filename": [replace_for_replay(item) for item in target.get("filename", [])],
                "compile": compile_recipes,
                "link": link_recipe,
                "runtime": {
                    "command_key": row["command_key"],
                    "cmd_normalized_argv": row["cmd_normalized_argv"],
                    "env_normalized": row["env_normalized"],
                    "workdir_key": row["workdir_key"],
                },
                "runnable": True,
            }
        )

    entries = generated_entries + smoke_entries + debian_smokes + upstream_entries
    write_json(
        abi_dir / "catalog.json",
        {
            "entries": entries,
        },
    )
    write_json(abi_dir / "unmatched-symbols.json", {"entries": unmatched})

    glib_core_generated = []
    glib_advanced_generated = []
    gobject_generated = []
    gio_generated = []
    girepository_generated = []
    glib_allowlist = []

    for entry in generated_entries + smoke_entries:
        library = entry["library"]
        if library == "libglib-2.0.so.0":
            is_advanced_glib = entry["id"].startswith("generated:") and any(
                entry["symbol"].startswith(prefix) for prefix in ADVANCED_GLIB_PREFIXES
            )
            if is_advanced_glib:
                glib_advanced_generated.append(entry["id"])
            else:
                glib_core_generated.append(entry["id"])
                glib_allowlist.append(entry["id"])
        elif library in {"libgthread-2.0.so.0", "libgmodule-2.0.so.0"}:
            glib_core_generated.append(entry["id"])
        elif library == "libgobject-2.0.so.0":
            gobject_generated.append(entry["id"])
        elif library == "libgio-2.0.so.0":
            gio_generated.append(entry["id"])
        elif library == "libgirepository-2.0.so.0":
            girepository_generated.append(entry["id"])

    write_json(abi_dir / "libglib-core-allowlist.json", {"entry_ids": glib_allowlist})

    manifest_files = {
        "glib-core": SAFE_ROOT / "tests" / "manifests" / "glib-core.txt",
        "glib-advanced": SAFE_ROOT / "tests" / "manifests" / "glib-advanced.txt",
        "gobject": SAFE_ROOT / "tests" / "manifests" / "gobject.txt",
        "gio": SAFE_ROOT / "tests" / "manifests" / "gio.txt",
        "girepository": SAFE_ROOT / "tests" / "manifests" / "girepository.txt",
    }
    upstream_by_row = {
        f"{entry['primary_suite']}\t{entry['name']}": entry["id"]
        for entry in upstream_entries
    }

    selections = {
        "abi-shell": [entry["id"] for entry in smoke_entries if entry["id"].startswith("generated-smoke:lib")] + [entry["id"] for entry in debian_smokes],
        "glib-core": list(glib_core_generated),
        "glib-advanced": list(glib_advanced_generated),
        "gobject": list(gobject_generated),
        "gio": list(gio_generated),
        "girepository": list(girepository_generated),
    }

    for phase, manifest_path in manifest_files.items():
        for line in manifest_path.read_text().splitlines():
            if not line:
                continue
            entry_id = upstream_by_row.get(line)
            if entry_id is not None:
                selections[phase].append(entry_id)

    full_entries = (
        [entry["id"] for entry in generated_entries]
        + [entry["id"] for entry in smoke_entries]
        + [entry["id"] for entry in debian_smokes]
        + [entry["id"] for entry in upstream_entries]
    )
    selections["full"] = full_entries

    for phase, entry_ids in selections.items():
        write_json(abi_dir / f"{phase}.json", {"phase": phase, "entry_ids": entry_ids})


def build_cve_matrix() -> None:
    relevant = read_json(VENDOR_WORKSPACE / "relevant_cves.json")
    lines = [
        "# CVE Matrix",
        "",
        "| CVE | Category | Upstream files | Affected API | Planned regression | Status |",
        "| --- | --- | --- | --- | --- | --- |",
    ]
    for entry in relevant["relevant_cves"]:
        mapping = CVE_MAP.get(
            entry["id"],
            {
                "files": ["original/UNKNOWN"],
                "api": "TBD",
                "planned_regression": "TBD",
            },
        )
        lines.append(
            "| {id} | {category} | {files} | {api} | {regression} | pending |".format(
                id=entry["id"],
                category=entry["category"],
                files="<br>".join(mapping["files"]),
                api=mapping["api"],
                regression=mapping["planned_regression"],
            )
        )
    write_text(SAFE_ROOT / "docs" / "cve-matrix.md", "\n".join(lines) + "\n")


def generate_all() -> None:
    build_symbols()
    path_map = build_test_source_path_map()
    records = normalize_tests(path_map)
    build_test_manifests(records)
    normalize_installed_files()
    build_control_preservation()
    build_patch_manifest()
    build_link_compat(records)
    build_cve_matrix()


def verify() -> None:
    with tempfile.TemporaryDirectory(prefix=".extract-abi-verify-", dir=REPO_ROOT) as tmpdir:
        tmp_safe = Path(tmpdir)
        (tmp_safe / "vendor").symlink_to(SAFE_ROOT / "vendor", target_is_directory=True)
        run(
            [sys.executable, str(Path(__file__).resolve())],
            env=clean_subprocess_env(updates={"SAFE_ROOT_OVERRIDE": str(tmp_safe)}),
        )

        for name in [
            "libglib2.0-0t64.symbols",
            "libgirepository-2.0-0.symbols",
            *[f"{soname}.symbols" for soname in SHARED_LIBRARY_MAP],
        ]:
            baseline = (SAFE_ROOT / "abi" / "symbols" / name).read_text()
            generated = (tmp_safe / "abi" / "symbols" / name).read_text()
            if baseline != generated:
                raise ValueError(f"Frozen ABI symbol baseline drifted: {name}")

        for relpath in [
            "abi/test-source-path-map.json",
            "abi/tests.json",
            "abi/installed-files.json",
            "abi/debian-control-preservation.json",
            "abi/debian-patches.json",
            "abi/link-compat/catalog.json",
            "abi/link-compat/unmatched-symbols.json",
            "abi/link-compat/abi-shell.json",
            "abi/link-compat/glib-core.json",
            "abi/link-compat/glib-advanced.json",
            "abi/link-compat/gobject.json",
            "abi/link-compat/gio.json",
            "abi/link-compat/girepository.json",
            "abi/link-compat/full.json",
            "abi/link-compat/libglib-core-allowlist.json",
        ]:
            compare_jsonish(read_json(SAFE_ROOT / relpath), read_json(tmp_safe / relpath))

        for relpath in [
            "tests/manifests/full.txt",
            "tests/manifests/fuzzing.txt",
            "tests/manifests/glib-core.txt",
            "tests/manifests/glib-advanced.txt",
            "tests/manifests/gobject.txt",
            "tests/manifests/gio.txt",
            "tests/manifests/girepository.txt",
            "docs/cve-matrix.md",
        ]:
            baseline = (SAFE_ROOT / relpath).read_text()
            generated = (tmp_safe / relpath).read_text()
            if baseline != generated:
                raise ValueError(f"Frozen text baseline drifted: {relpath}")

    tests = read_json(SAFE_ROOT / "abi" / "tests.json")
    if any(not row.get("primary_suite") for row in tests):
        raise ValueError("A frozen test row is missing primary_suite")
    if any(
        contains_workspace_path(
            {
                "cmd_normalized_argv": row["cmd_normalized_argv"],
                "env_normalized": row["env_normalized"],
                "workdir_key": row["workdir_key"],
            }
        )
        for row in tests
    ):
        raise ValueError("Frozen abi/tests.json still embeds workspace paths")
    unmatched = read_json(SAFE_ROOT / "abi" / "link-compat" / "unmatched-symbols.json")
    allowed = {
        "not declared in installed public headers",
        "tooling parser limitation with tracked blocker",
    }
    if any(entry["reason"] not in allowed for entry in unmatched["entries"]):
        raise ValueError("Found unexpected unmatched-symbol reason")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    if args.verify:
        verify()
    else:
        generate_all()


if __name__ == "__main__":
    main()
