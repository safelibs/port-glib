use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

fn read_dir_recursive(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).unwrap_or_else(|error| {
        panic!("failed to read {}: {error}", root.display());
    }) {
        let entry = entry.expect("failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            read_dir_recursive(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

fn collect_quoted_values(text: &str, marker: &str) -> BTreeSet<String> {
    let mut values = BTreeSet::new();
    let mut rest = text;
    while let Some(marker_index) = rest.find(marker) {
        rest = &rest[marker_index + marker.len()..];
        let Some(start) = rest.find('"') else {
            break;
        };
        let after_start = &rest[start + 1..];
        let Some(end) = after_start.find('"') else {
            break;
        };
        values.insert(after_start[..end].to_owned());
        rest = &after_start[end + 1..];
    }
    values
}

fn rust_owned_exports(src_dir: &Path) -> BTreeSet<String> {
    let mut files = Vec::new();
    read_dir_recursive(src_dir, &mut files);
    let mut exports = BTreeSet::new();
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
        let text = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", file.display()));
        exports.extend(collect_quoted_values(&text, "export_name = "));
    }
    exports
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum AliasKind {
    Function,
    Object,
}

fn next_item_name(line: &str) -> Option<(AliasKind, &str)> {
    let patterns = [
        ("pub unsafe extern \"C\" fn ", AliasKind::Function),
        ("unsafe extern \"C\" fn ", AliasKind::Function),
        ("pub static mut ", AliasKind::Object),
        ("pub static ", AliasKind::Object),
    ];
    for (pattern, kind) in patterns {
        if let Some(rest) = line.trim_start().strip_prefix(pattern) {
            return rest
                .split(|ch: char| !(ch == '_' || ch.is_ascii_alphanumeric()))
                .next()
                .map(|name| (kind, name));
        }
    }
    None
}

fn translated_exports(translated_dir: &Path) -> BTreeSet<(String, AliasKind)> {
    let mut files = Vec::new();
    read_dir_recursive(translated_dir, &mut files);
    let mut exports = BTreeSet::new();
    for file in files {
        let text = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", file.display()));
        let lines = text.lines().collect::<Vec<_>>();
        for index in 0..lines.len() {
            if lines[index].trim() != "#[no_mangle]" {
                continue;
            }
            for candidate in lines.iter().skip(index + 1).take(8) {
                if let Some((kind, name)) = next_item_name(candidate) {
                    if name.starts_with("safe_c2rust_") {
                        exports.insert((name.to_owned(), kind));
                    }
                    break;
                }
            }
        }
    }
    exports
}

fn write_aliases(out_dir: &Path, aliases: &[(String, String, AliasKind)]) {
    let mut asm = String::new();
    asm.push_str("core::arch::global_asm!(r#\"\n");
    asm.push_str(".text\n");
    for (export, target, kind) in aliases {
        asm.push_str(&format!(".globl {export}\n"));
        match kind {
            AliasKind::Function => {
                asm.push_str(&format!(".type {export}, @function\n"));
                asm.push_str(&format!("{export}:\n"));
                asm.push_str(&format!("    jmp {target}\n"));
                asm.push_str(&format!(".size {export}, .-{export}\n"));
            }
            AliasKind::Object => {
                asm.push_str(&format!(".type {export}, @object\n"));
                asm.push_str(&format!(".set {export}, {target}\n"));
            }
        }
    }
    asm.push_str("\"#, options(att_syntax));\n");
    fs::write(out_dir.join("glib_aliases.rs"), asm).expect("failed to write translated aliases");
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let src_dir = manifest_dir.join("src");
    let translated_dir = src_dir.join("translated");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing out dir"));

    println!("cargo:rerun-if-changed={}", manifest_dir.join("build.rs").display());
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");

    if let Ok(soname) = env::var("SAFE_LINK_SONAME") {
        emit_cdylib_arg(format!("-Wl,-soname,{soname}"));
    }
    if let Ok(version_script) = env::var("SAFE_LINK_VERSION_SCRIPT") {
        emit_cdylib_arg(format!("-Wl,--version-script={version_script}"));
    }

    emit_cdylib_arg("-Wl,--as-needed");
    emit_cdylib_arg("-Wl,--no-undefined");
    emit_cdylib_arg("-Wl,-z,nodelete");
    emit_cdylib_arg("-Wl,-Bsymbolic-functions");
    println!("cargo:rustc-link-lib=dylib=pcre2-8");

    let rust_exports = rust_owned_exports(&src_dir);
    let mut aliases = Vec::new();
    for (target, kind) in translated_exports(&translated_dir) {
        let Some(export) = target.strip_prefix("safe_c2rust_") else {
            continue;
        };
        if rust_exports.contains(export) {
            continue;
        }
        aliases.push((export.to_owned(), target, kind));
    }
    write_aliases(&out_dir, &aliases);
}
