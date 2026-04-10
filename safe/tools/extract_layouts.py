#!/usr/bin/env python3
import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path

from common import (
    REPO_ROOT,
    SAFE_ROOT,
    VENDOR_BUILD_CHECK,
    VENDOR_ORIGINAL,
    clean_subprocess_env,
    compare_jsonish,
    ensure_dir,
    read_json,
    run,
    write_json,
)


LAYOUT_SEEDS = {
    "glib": {
        "glib/glist.h": ["GList"],
        "glib/gslist.h": ["GSList"],
        "glib/gqueue.h": ["GQueue"],
        "glib/garray.h": ["GArray", "GPtrArray", "GByteArray"],
        "glib/gstring.h": ["GString"],
        "glib/gerror.h": ["GError"],
        "glib/goption.h": ["GOptionEntry"],
    },
    "gthread": {
        "glib/gthread.h": ["GMutex", "GRecMutex", "GRWLock", "GCond", "GOnce"],
    },
    "gmodule": {
        "gmodule/gmodule.h": ["GModuleFlags"],
    },
    "gobject": {
        "gobject/gtype.h": [
            "GTypeInfo",
            "GTypeQuery",
            "GTypeValueTable",
            "GInterfaceInfo",
            "GTypeInstance",
            "GTypeClass",
            "GTypeInterface",
        ],
        "gobject/gvalue.h": ["GValue"],
        "gobject/gclosure.h": ["GClosure", "GCClosure"],
        "gobject/gobject.h": ["GObjectConstructParam"],
    },
    "gio": {
        "gio/gactionmap.h": ["GActionEntry"],
        "gio/gdbusconnection.h": ["GDBusInterfaceVTable", "GDBusSubtreeVTable"],
    },
    "girepository": {
        "girepository/gitypes.h": [
            "GIBaseInfoStack",
            "GIArgInfo",
            "GITypeInfo",
            "GIArgument",
            "GITypeTag",
            "GIArrayType",
        ],
        "girepository/gibaseinfo.h": ["GIAttributeIter"],
    },
}

PROBE_INCLUDES = {
    "glib": "glib.h",
    "gthread": "glib.h",
    "gmodule": "gmodule.h",
    "gobject": "glib-object.h",
    "gio": "gio/gio.h",
    "girepository": "girepository/girepository.h",
}

FIELD_OVERRIDES = {
    "GValue": ["g_type", "data"],
    "GClosure": ["marshal", "data", "notifiers"],
    "GCClosure": ["closure", "callback"],
    "GDBusInterfaceVTable": ["method_call", "get_property", "set_property", "padding"],
    "GDBusSubtreeVTable": ["enumerate", "introspect", "dispatch", "padding"],
    "GIBaseInfoStack": ["parent_instance", "dummy0", "dummy1", "dummy2", "dummy3"],
    "GIArgInfo": ["parent", "padding"],
    "GITypeInfo": ["parent", "padding"],
}

TYPE_KIND_OVERRIDES = {
    "GModuleFlags": "enum",
    "GITypeTag": "enum",
    "GIArrayType": "enum",
}


def extract_type_definition(header_text: str, type_name: str) -> tuple[str, list[str]]:
    if type_name in TYPE_KIND_OVERRIDES:
        if re.search(rf"\b{re.escape(type_name)}\b", header_text):
            return (TYPE_KIND_OVERRIDES[type_name], [])
        raise ValueError(f"Could not find a public definition for {type_name}")
    typedef_pattern = re.compile(
        rf"typedef\s+(struct|union|enum)\s+_?{re.escape(type_name)}?\s*\{{(?P<body>.*?)\}}\s*{re.escape(type_name)}\s*;",
        re.S,
    )
    anonymous_typedef_pattern = re.compile(
        rf"typedef\s+(struct|union|enum)\s*\{{(?P<body>.*?)\}}\s*{re.escape(type_name)}\s*;",
        re.S,
    )
    direct_pattern = re.compile(
        rf"(struct|union)\s+_{re.escape(type_name)}\s*\{{(?P<body>.*?)\}};",
        re.S,
    )
    for pattern in [typedef_pattern, anonymous_typedef_pattern, direct_pattern]:
        match = pattern.search(header_text)
        if match:
            body = match.group("body")
            kind = "enum" if "enum" in match.group(0).split("{", 1)[0] else ("union" if "union" in match.group(0).split("{", 1)[0] else "struct")
            return kind, [] if kind == "enum" else parse_fields(body)
    enum_pattern = re.compile(
        rf"typedef\s+enum\s*\{{(?P<body>.*?)\}}\s*{re.escape(type_name)}\s*;",
        re.S,
    )
    match = enum_pattern.search(header_text)
    if match:
        return ("enum", [])
    raise ValueError(f"Could not find a public definition for {type_name}")


def parse_fields(body: str) -> list[str]:
    fields = []
    for statement in body.split(";"):
        line = statement.strip()
        if not line or line.startswith("#"):
            continue
        line = re.sub(r"/\*.*?\*/", "", line)
        function_pointer = re.search(r"\(\s*\*\s*([A-Za-z_]\w*)\s*\)", line)
        if function_pointer:
            fields.append(function_pointer.group(1))
            continue
        candidate = line.split(":")[0]
        match = re.search(r"([A-Za-z_]\w*)\s*(?:\[[^\]]+\])?$", candidate)
        if match:
            fields.append(match.group(1))
    return fields


def build_manifests() -> dict[str, dict[str, object]]:
    manifests = {}
    for library, headers in LAYOUT_SEEDS.items():
        entries = []
        for header_relpath, type_names in headers.items():
            header_path = VENDOR_ORIGINAL / header_relpath
            header_text = header_path.read_text(errors="ignore")
            for type_name in type_names:
                kind, fields = extract_type_definition(header_text, type_name)
                entries.append(
                    {
                        "type_name": type_name,
                        "kind": kind,
                        "header": header_relpath,
                        "probe_include": PROBE_INCLUDES[library],
                        "fields": FIELD_OVERRIDES.get(type_name, fields),
                        "rust_probe_symbol": f"pending::{library}::{type_name}",
                    }
                )
        manifest = {"library": library, "entries": entries}
        manifests[library] = manifest
        write_json(SAFE_ROOT / "abi" / "layout-manifests" / f"{library}.json", manifest)
    return manifests


def include_args() -> list[str]:
    return [
        f"-I{VENDOR_BUILD_CHECK}",
        f"-I{VENDOR_ORIGINAL}",
        f"-I{VENDOR_BUILD_CHECK / 'glib'}",
        f"-I{VENDOR_BUILD_CHECK / 'gobject'}",
        f"-I{VENDOR_BUILD_CHECK / 'gmodule'}",
        f"-I{VENDOR_BUILD_CHECK / 'gio'}",
        f"-I{VENDOR_BUILD_CHECK / 'girepository'}",
        f"-I{VENDOR_ORIGINAL / 'glib'}",
        f"-I{VENDOR_ORIGINAL / 'gobject'}",
        f"-I{VENDOR_ORIGINAL / 'gio'}",
        f"-I{VENDOR_ORIGINAL / 'gmodule'}",
        f"-I{VENDOR_ORIGINAL / 'girepository'}",
    ]


def measure_entry(entry: dict[str, object]) -> dict[str, object]:
    fields = entry["fields"]
    source = [
        "#include <stddef.h>",
        "#include <stdio.h>",
        f"#include <{entry['probe_include']}>",
        "int main(void) {",
        f"    printf(\"%zu\\n\", sizeof({entry['type_name']}));",
        f"    printf(\"%zu\\n\", _Alignof({entry['type_name']}));",
    ]
    for field in fields:
        source.append(f"    printf(\"%zu\\n\", offsetof({entry['type_name']}, {field}));")
    source.append("    return 0;")
    source.append("}")
    with tempfile.TemporaryDirectory(prefix="safe-layout-") as tmpdir:
        tmpdir_path = Path(tmpdir)
        c_path = tmpdir_path / "probe.c"
        exe_path = tmpdir_path / "probe"
        c_path.write_text("\n".join(source) + "\n")
        compile_proc = subprocess.run(
            ["gcc", "-std=gnu11", *include_args(), str(c_path), "-o", str(exe_path)],
            text=True,
            capture_output=True,
        )
        if compile_proc.returncode != 0:
            raise RuntimeError(
                f"Failed to compile layout probe for {entry['type_name']} from {entry['header']}:\n"
                + compile_proc.stderr
            )
        output = subprocess.run([str(exe_path)], check=True, text=True, capture_output=True).stdout.splitlines()
    values = [int(item) for item in output]
    result = {
        "type_name": entry["type_name"],
        "kind": entry["kind"],
        "size": values[0],
        "align": values[1],
        "fields": {field: values[index + 2] for index, field in enumerate(fields)},
    }
    return result


def generate() -> None:
    manifests = build_manifests()
    ensure_dir(SAFE_ROOT / "abi" / "layouts")
    for library, manifest in manifests.items():
        results = {"library": library, "entries": [measure_entry(entry) for entry in manifest["entries"]]}
        write_json(SAFE_ROOT / "abi" / "layouts" / f"{library}.json", results)


def verify() -> None:
    with tempfile.TemporaryDirectory(prefix=".extract-layouts-verify-", dir=REPO_ROOT) as tmpdir:
        tmp_safe = Path(tmpdir)
        (tmp_safe / "vendor").symlink_to(SAFE_ROOT / "vendor", target_is_directory=True)
        run(
            [sys.executable, str(Path(__file__).resolve())],
            env=clean_subprocess_env(updates={"SAFE_ROOT_OVERRIDE": str(tmp_safe)}),
        )

        for library in LAYOUT_SEEDS:
            compare_jsonish(
                read_json(SAFE_ROOT / "abi" / "layout-manifests" / f"{library}.json"),
                read_json(tmp_safe / "abi" / "layout-manifests" / f"{library}.json"),
            )
            compare_jsonish(
                read_json(SAFE_ROOT / "abi" / "layouts" / f"{library}.json"),
                read_json(tmp_safe / "abi" / "layouts" / f"{library}.json"),
            )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    if args.verify:
        verify()
    else:
        generate()


if __name__ == "__main__":
    main()
