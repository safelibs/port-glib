import json
import os
import posixpath
import re
import shutil
import subprocess
import tempfile
from functools import lru_cache
from pathlib import Path
from typing import Any


SAFE_ROOT = Path(os.environ.get("SAFE_ROOT_OVERRIDE", Path(__file__).resolve().parents[1])).resolve()
REPO_ROOT = SAFE_ROOT.parent
VENDOR_ORIGINAL = SAFE_ROOT / "vendor" / "original"
VENDOR_BUILD_CHECK = SAFE_ROOT / "vendor" / "build-check"
VENDOR_WORKSPACE = SAFE_ROOT / "vendor" / "workspace"

PATH_LIST_ENV_VARS = {
    "GI_TYPELIB_PATH",
    "LD_LIBRARY_PATH",
    "PATH",
    "PKG_CONFIG_PATH",
    "XDG_DATA_DIRS",
}
SCRUBBED_ENV_VARS = {
    "LD_PRELOAD",
}
SCRIPT_SUFFIXES = (".py", ".pl", ".sh")
BUILD_RELATIVE_ROOTS = {
    "fuzzing",
    "gio",
    "girepository",
    "glib",
    "gmodule",
    "gobject",
    "gthread",
    "meson-info",
    "meson-private",
    "po",
    "tests",
    "tools",
}
ORIGINAL_RELATIVE_ROOTS = {
    "debian",
    "fuzzing",
    "gio",
    "girepository",
    "glib",
    "gmodule",
    "gobject",
    "gthread",
    "m4macros",
    "po",
    "tests",
    "unicode-data",
}


def ensure_dir(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)


def read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def write_json(path: Path, data: Any) -> None:
    ensure_dir(path.parent)
    path.write_text(json.dumps(data, indent=2, sort_keys=False) + "\n")


def write_text(path: Path, content: str) -> None:
    ensure_dir(path.parent)
    path.write_text(content)


@lru_cache(maxsize=1)
def default_pkg_config_libdir() -> str:
    probe_env = dict(os.environ)
    probe_env.pop("LD_PRELOAD", None)
    probe_env.pop("PKG_CONFIG_LIBDIR", None)
    probe_env.pop("PKG_CONFIG_SYSROOT_DIR", None)
    try:
        result = subprocess.run(
            ["pkg-config", "--variable", "pc_path", "pkg-config"],
            env=probe_env,
            check=True,
            text=True,
            capture_output=True,
        )
    except (FileNotFoundError, subprocess.CalledProcessError):
        fallback_dirs = [
            "/usr/local/lib/x86_64-linux-gnu/pkgconfig",
            "/usr/local/lib/pkgconfig",
            "/usr/local/share/pkgconfig",
            "/usr/lib/x86_64-linux-gnu/pkgconfig",
            "/usr/lib/pkgconfig",
            "/usr/share/pkgconfig",
        ]
        return ":".join(path for path in fallback_dirs if Path(path).is_dir())
    return result.stdout.strip()


def clean_subprocess_env(
    *,
    base: dict[str, str] | None = None,
    updates: dict[str, str] | None = None,
) -> dict[str, str]:
    env = dict(base if base is not None else os.environ)
    for key in SCRUBBED_ENV_VARS:
        env.pop(key, None)
    env["PKG_CONFIG_LIBDIR"] = default_pkg_config_libdir()
    env["PKG_CONFIG_SYSROOT_DIR"] = ""
    if updates:
        env.update(updates)
    return env


def run(
    argv: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    capture: bool = False,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        argv,
        cwd=str(cwd) if cwd else None,
        env=clean_subprocess_env() if env is None else env,
        check=True,
        text=True,
        capture_output=capture,
    )


def normpath(value: str) -> str:
    return posixpath.normpath(value.replace("\\", "/"))


def path_map_entries() -> list[dict[str, str]]:
    return [
        {
            "editable_prefix": "safe/tests/upstream/glib",
            "canonical_prefix": "safe/vendor/original/glib/tests",
        },
        {
            "editable_prefix": "safe/tests/upstream/gthread",
            "canonical_prefix": "safe/vendor/original/gthread/tests",
        },
        {
            "editable_prefix": "safe/tests/upstream/gmodule",
            "canonical_prefix": "safe/vendor/original/gmodule/tests",
        },
        {
            "editable_prefix": "safe/tests/upstream/gobject",
            "canonical_prefix": "safe/vendor/original/gobject/tests",
        },
        {
            "editable_prefix": "safe/tests/upstream/gio",
            "canonical_prefix": "safe/vendor/original/gio/tests",
        },
        {
            "editable_prefix": "safe/tests/upstream/girepository",
            "canonical_prefix": "safe/vendor/original/girepository/tests",
        },
        {
            "editable_prefix": "safe/tests/upstream/fuzzing",
            "canonical_prefix": "safe/vendor/original/fuzzing",
        },
    ]


def contains_workspace_path(value: Any) -> bool:
    if isinstance(value, str):
        return str(REPO_ROOT) in value
    if isinstance(value, list):
        return any(contains_workspace_path(item) for item in value)
    if isinstance(value, dict):
        return any(contains_workspace_path(item) for item in value.values())
    return False


class PathNormalizer:
    def __init__(
        self,
        *,
        build_root: Path,
        original_root: Path,
        build_root_placeholder: str = "$BUILD_ROOT",
        original_placeholder: str = "$SAFE_VENDOR_ORIGINAL",
        vendor_build_check_placeholder: str | None = None,
        path_map: list[dict[str, str]] | None = None,
    ) -> None:
        self.build_root = build_root if build_root.is_absolute() else REPO_ROOT / build_root
        self.original_root = original_root if original_root.is_absolute() else REPO_ROOT / original_root
        self.build_root_placeholder = build_root_placeholder
        self.original_placeholder = original_placeholder
        self.vendor_build_check_placeholder = vendor_build_check_placeholder
        self.path_map = path_map or []
        self.build_root_rel = self._repo_rel(self.build_root)
        self.original_root_rel = self._repo_rel(self.original_root)

    @staticmethod
    def _repo_rel(path: Path) -> str | None:
        try:
            return path.relative_to(REPO_ROOT).as_posix()
        except ValueError:
            return None

    @staticmethod
    def _rewrite_prefixed(relpath: str, root_rel: str | None, placeholder: str) -> str | None:
        if root_rel is None:
            return None
        if relpath == root_rel:
            return placeholder
        if relpath.startswith(root_rel + "/"):
            return f"{placeholder}/{relpath[len(root_rel) + 1:]}"
        return None

    def _apply_path_map(self, text: str) -> str:
        normalized = normpath(text)
        for entry in self.path_map:
            src = entry["editable_prefix"]
            dst = entry["canonical_prefix"]
            if normalized == src:
                return dst
            if normalized.startswith(src + "/"):
                return dst + normalized[len(src) :]
        return normalized

    def _normalize_repo_relative(self, relpath: str) -> str | None:
        relpath = self._apply_path_map(normpath(relpath))
        rewritten = self._rewrite_prefixed(relpath, self.original_root_rel, self.original_placeholder)
        if rewritten is not None:
            return rewritten
        rewritten = self._rewrite_prefixed(relpath, self.build_root_rel, self.build_root_placeholder)
        if rewritten is not None:
            return rewritten
        if self.vendor_build_check_placeholder:
            rewritten = self._rewrite_prefixed(
                relpath,
                self._repo_rel(SAFE_ROOT / "vendor" / "build-check"),
                self.vendor_build_check_placeholder,
            )
            if rewritten is not None:
                return rewritten
        if relpath.startswith("safe/tests/upstream/"):
            mapped = self._apply_path_map(relpath)
            if mapped != relpath:
                return self._normalize_repo_relative(mapped)
        return None

    def normalize_token(self, token: str, *, command_position: bool = False) -> str:
        if not token or token.startswith("$"):
            return token
        token = token.replace("\\", "/")
        absolute = token.startswith("/")
        if absolute:
            token = normpath(token)
            if token == str(REPO_ROOT):
                return token
            try:
                relpath = Path(token).relative_to(REPO_ROOT).as_posix()
            except ValueError:
                return f"$SYSTEM_TOOL/{Path(token).name}" if command_position else token
            rewritten = self._normalize_repo_relative(relpath)
            return rewritten if rewritten is not None else token

        token = self._apply_path_map(token)
        if token in {".", ".."} or token.startswith("../"):
            collapsed = normpath(f"{self.build_root}/{token}")
            try:
                relpath = Path(collapsed).relative_to(REPO_ROOT).as_posix()
            except ValueError:
                return token
            rewritten = self._normalize_repo_relative(relpath)
            return rewritten if rewritten is not None else token

        rewritten = self._normalize_repo_relative(token)
        if rewritten is not None:
            return rewritten

        first = token.split("/", 1)[0]
        if first in BUILD_RELATIVE_ROOTS:
            candidate = self.build_root / token
            if candidate.exists():
                return f"{self.build_root_placeholder}/{normpath(token)}"
        if first in ORIGINAL_RELATIVE_ROOTS:
            candidate = REPO_ROOT / token
            if candidate.exists():
                return f"{self.original_placeholder}/{normpath(token[len('original/'):] if token.startswith('original/') else token)}"
        return token

    def normalize_env_value(self, name: str, value: str) -> str:
        if name in PATH_LIST_ENV_VARS and ":" in value:
            components = [self.normalize_token(component) for component in value.split(":")]
            return ":".join(components)
        return self.normalize_token(value)

    def normalize_workdir(self, workdir: str | None) -> tuple[str, str | None, str]:
        if not workdir:
            return ("none", None, "none")
        normalized = self.normalize_token(workdir)
        origin, relpath = placeholder_origin_relpath(normalized)
        if origin is None:
            return ("literal", normalized, normalized)
        return (origin, relpath, f"{origin}:{relpath}")

    def command_descriptor(
        self,
        cmd: list[str],
        env: dict[str, str],
        workdir: str | None,
    ) -> dict[str, Any]:
        normalized_cmd = [
            self.normalize_token(item, command_position=(index == 0))
            for index, item in enumerate(cmd)
        ]
        normalized_env = {
            key: self.normalize_env_value(key, value)
            for key, value in sorted(env.items())
        }
        workdir_kind, workdir_relpath, workdir_key = self.normalize_workdir(workdir)

        command_origin = "system_tool"
        command_relpath = None
        command_basename = None
        command_key = "system_tool:unknown"

        script_operand = next(
            (
                item
                for item in normalized_cmd[1:]
                if placeholder_origin_relpath(item)[0] in {"build_root", "vendor_original"}
                and item.endswith(SCRIPT_SUFFIXES)
            ),
            None,
        )
        if script_operand is not None:
            command_origin, command_relpath = placeholder_origin_relpath(script_operand)
            command_key = f"{command_origin}:{command_relpath}"
        elif normalized_cmd:
            origin, relpath = placeholder_origin_relpath(normalized_cmd[0])
            if origin is not None:
                command_origin = origin
                command_relpath = relpath
                command_key = f"{origin}:{relpath}"
            elif normalized_cmd[0].startswith("$SYSTEM_TOOL/"):
                command_basename = normalized_cmd[0].split("/", 1)[1]
                command_key = f"system_tool:{command_basename}"
            else:
                command_basename = normalized_cmd[0]
                command_key = f"system_tool:{command_basename}"

        return {
            "command_origin": command_origin,
            "command_relpath": command_relpath,
            "command_basename": command_basename,
            "command_key": command_key,
            "cmd_normalized_argv": normalized_cmd,
            "env_normalized": normalized_env,
            "workdir_kind": workdir_kind,
            "workdir_relpath": workdir_relpath,
            "workdir_key": workdir_key,
        }


def placeholder_origin_relpath(value: str | None) -> tuple[str | None, str | None]:
    if not value:
        return (None, None)
    if value.startswith("$BUILD_ROOT/"):
        return ("build_root", value[len("$BUILD_ROOT/") :])
    if value == "$BUILD_ROOT":
        return ("build_root", "")
    if value.startswith("$SAFE_VENDOR_ORIGINAL/"):
        return ("vendor_original", value[len("$SAFE_VENDOR_ORIGINAL/") :])
    if value == "$SAFE_VENDOR_ORIGINAL":
        return ("vendor_original", "")
    if value.startswith("$SAFE_VENDOR_BUILD_CHECK/"):
        return ("vendor_build_check", value[len("$SAFE_VENDOR_BUILD_CHECK/") :])
    if value == "$SAFE_VENDOR_BUILD_CHECK":
        return ("vendor_build_check", "")
    return (None, None)


def parse_deb822_file(path: Path) -> list[dict[str, str]]:
    stanzas: list[dict[str, list[str]]] = []
    current: dict[str, list[str]] = {}
    current_field: str | None = None
    for raw_line in path.read_text().splitlines():
        line = raw_line.rstrip()
        if not line:
            if current:
                stanzas.append(current)
                current = {}
                current_field = None
            continue
        if line.startswith("#"):
            continue
        if line[0].isspace():
            if current_field is None:
                raise ValueError(f"Continuation line without field in {path}: {line!r}")
            current[current_field].append(line.strip())
            continue
        field, value = line.split(":", 1)
        current_field = field
        current.setdefault(field, []).append(value.strip())
    if current:
        stanzas.append(current)
    normalized: list[dict[str, str]] = []
    for stanza in stanzas:
        normalized.append(
            {
                field: normalize_deb822_value(" ".join(parts))
                for field, parts in stanza.items()
            }
        )
    return normalized


def normalize_deb822_value(value: str) -> str:
    return re.sub(r"\s+", " ", value).strip()


def series_lines(series_path: Path) -> list[str]:
    lines: list[str] = []
    for raw_line in series_path.read_text().splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        lines.append(line)
    return lines


def compare_jsonish(expected: Any, actual: Any) -> None:
    if expected != actual:
        raise AssertionError(
            "JSON mismatch\nEXPECTED:\n"
            + json.dumps(expected, indent=2, sort_keys=True)
            + "\nACTUAL:\n"
            + json.dumps(actual, indent=2, sort_keys=True)
        )


def temp_workdir(prefix: str) -> tempfile.TemporaryDirectory[str]:
    return tempfile.TemporaryDirectory(prefix=prefix, dir=str(SAFE_ROOT))


def copy_tree(src: Path, dst: Path) -> None:
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src, dst, symlinks=True)
