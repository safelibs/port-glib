use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

fn needs_safety_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains("unsafe {")
        || trimmed.contains(" unsafe fn ")
        || trimmed.starts_with("unsafe fn ")
        || trimmed.contains("unsafe(link_section")
        || trimmed.starts_with("#[unsafe(")
        || (trimmed.contains("unsafe extern") && trimmed.ends_with('{'))
}

fn has_safety_comment(lines: &[&str], index: usize) -> bool {
    let start = index.saturating_sub(2);
    lines[start..=index]
        .iter()
        .any(|line| line.contains("SAFETY:"))
}

fn file_is_translated_audit(lines: &[&str]) -> bool {
    lines.iter().take(3).any(|line| line.contains("SAFETY:"))
}

fn audit_unsafe_usage(root: &Path) {
    fn visit(dir: &Path, missing: &mut Vec<String>) {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", dir.display()))
            .collect();
        entries.sort_by_key(|entry| entry.as_ref().unwrap().path());
        for entry in entries {
            let entry = entry.unwrap_or_else(|err| panic!("failed to read directory entry: {err}"));
            let path = entry.path();
            if path.is_dir() {
                visit(&path, missing);
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let text = fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
            let lines: Vec<_> = text.lines().collect();
            if file_is_translated_audit(&lines) {
                continue;
            }
            for (index, line) in lines.iter().enumerate() {
                if needs_safety_comment(line) && !has_safety_comment(&lines, index) {
                    missing.push(format!("{}:{}", path.display(), index + 1));
                }
            }
        }
    }

    let mut missing = Vec::new();
    visit(root, &mut missing);
    if !missing.is_empty() {
        panic!(
            "safe-gobject requires a SAFETY audit comment for every unsafe boundary:\n{}",
            missing.join("\n")
        );
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    audit_unsafe_usage(&manifest_dir.join("src"));
    println!("cargo:rerun-if-changed={}", manifest_dir.join("src").display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir
            .join("../../abi/version-scripts/libgobject.map")
            .display()
    );
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
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=ffi");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=glib-2.0");
    println!("cargo:rustc-link-lib=dylib=gthread-2.0");
    println!("cargo:rustc-link-lib=dylib=gmodule-2.0");
}
