use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone)]
struct Export {
    name: String,
    ident: String,
}

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

fn sanitize_ident(symbol: &str) -> String {
    symbol
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

fn collect_exports(library: &Path) -> Vec<Export> {
    let output = Command::new("nm")
        .args(["-D", "-S", "--defined-only", &library.display().to_string()])
        .output()
        .expect("failed to run nm");
    if !output.status.success() {
        panic!("nm failed for {}", library.display());
    }

    let mut exports = BTreeMap::<String, Export>::new();
    for line in String::from_utf8(output.stdout)
        .expect("nm output must be utf-8")
        .lines()
    {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let symbol = parts[3];
        if !matches!(parts[2].chars().next().unwrap_or_default(), 'T' | 'W' | 'i' | 'I') {
            continue;
        }
        exports.entry(symbol.to_owned()).or_insert_with(|| Export {
            name: symbol.to_owned(),
            ident: sanitize_ident(symbol),
        });
    }

    exports.into_values().collect()
}

fn write_exports(out_dir: &Path, exports: &[Export]) {
    let mut body = String::new();

    for export in exports {
        let slot = format!("__safe_gobject_slot_{}", export.ident);
        let _ = writeln!(body, "#[used]\nstatic mut {slot}: usize = 0;\n");
        let _ = writeln!(
            body,
            "core::arch::global_asm!(\".globl {name}\", \".type {name}, @function\", \"{name}:\", \"  mov rax, qword ptr [rip + {{slot}}@GOTPCREL]\", \"  jmp qword ptr [rax]\", \".size {name}, .-{name}\", slot = sym {slot});\n",
            name = export.name,
        );
    }

    body.push_str("pub(crate) unsafe fn initialize(handle: *mut core::ffi::c_void) {\n");
    for export in exports {
        let slot = format!("__safe_gobject_slot_{}", export.ident);
        let _ = writeln!(
            body,
            "    {slot} = crate::runtime::require_function(handle, b\"{name}\\0\");",
            name = export.name,
        );
    }
    body.push_str("}\n");

    fs::write(out_dir.join("generated-exports.rs"), body)
        .expect("failed to write generated forwarders");
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let original = manifest_dir
        .join("../../vendor/build-check/gobject/libgobject-2.0.so.0.8000.0")
        .canonicalize()
        .expect("missing frozen original gobject");
    println!("cargo:rerun-if-changed={}", original.display());
    println!("cargo:rustc-env=SAFE_FORWARD_ORIGINAL_LIB={}", original.display());

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

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    write_exports(&out_dir, &collect_exports(&original));
}
