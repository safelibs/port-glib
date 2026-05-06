use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

fn exported_symbols(version_script: &Path) -> Vec<String> {
    let text = fs::read_to_string(version_script)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", version_script.display()));
    let mut in_global = false;
    let mut symbols = Vec::new();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line == "global:" {
            in_global = true;
            continue;
        }
        if line == "local:" {
            in_global = false;
            continue;
        }
        if !in_global || !line.ends_with(';') {
            continue;
        }
        let symbol = line.trim_end_matches(';').trim();
        if symbol.is_empty() || symbol.contains('*') {
            continue;
        }
        symbols.push(symbol.to_owned());
    }
    if symbols.is_empty() {
        panic!("no GIO exports found in {}", version_script.display());
    }
    symbols.sort();
    symbols.dedup();
    symbols
}

fn rust_implemented_symbol(symbol: &str) -> bool {
    matches!(
        symbol,
        "g_application_new" | "g_file_new_for_path" | "g_settings_new" | "g_socket_new"
    )
}

fn render_exports(symbols: &[String]) -> String {
    let mut asm = String::new();
    asm.push_str("core::arch::global_asm!(r#\"\n");
    asm.push_str(".hidden safe_gio_stub_entry\n");
    asm.push_str(".text\n");
    for symbol in symbols {
        if rust_implemented_symbol(symbol) {
            continue;
        }
        asm.push_str(&format!(".globl {symbol}\n"));
        asm.push_str(&format!(".type {symbol}, @function\n"));
        asm.push_str(&format!("{symbol}:\n"));
        asm.push_str("    sub $8, %rsp\n");
        asm.push_str("    call safe_gio_stub_entry\n");
        asm.push_str("    add $8, %rsp\n");
        asm.push_str("    ret\n");
        asm.push_str(&format!(".size {symbol}, .-{symbol}\n"));
    }
    asm.push_str("\"#, options(att_syntax));\n");
    asm
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let safe_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under safe/crates")
        .to_path_buf();
    let version_script = safe_root.join("abi/version-scripts/libgio.map");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    let symbols = exported_symbols(&version_script);
    fs::write(
        out_dir.join("generated-exports.rs"),
        render_exports(&symbols),
    )
    .expect("failed to write generated GIO exports");

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("build.rs").display()
    );
    println!("cargo:rerun-if-changed={}", version_script.display());
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");

    if let Ok(soname) = env::var("SAFE_LINK_SONAME") {
        emit_cdylib_arg(format!("-Wl,-soname,{soname}"));
    }
    if let Ok(version_script) = env::var("SAFE_LINK_VERSION_SCRIPT") {
        emit_cdylib_arg(format!("-Wl,--version-script={version_script}"));
    }
    emit_cdylib_arg("-Wl,--no-undefined");
    emit_cdylib_arg("-Wl,-z,nodelete");
    emit_cdylib_arg("-Wl,-Bsymbolic-functions");
    emit_cdylib_arg("-Wl,--push-state,--no-as-needed");
    emit_cdylib_arg("-lglib-2.0");
    emit_cdylib_arg("-lgobject-2.0");
    emit_cdylib_arg("-lgmodule-2.0");
    emit_cdylib_arg("-lz");
    emit_cdylib_arg("-lmount");
    emit_cdylib_arg("-lselinux");
    emit_cdylib_arg("-Wl,--pop-state");
}
