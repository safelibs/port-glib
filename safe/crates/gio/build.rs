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

fn asm_symbol_name(symbol: &str) -> String {
    format!(".Lsafe_gio_name_{symbol}")
}

fn asm_slot_name(symbol: &str) -> String {
    format!(".Lsafe_gio_slot_{symbol}")
}

fn asm_resolver_name(symbol: &str) -> String {
    format!(".Lsafe_gio_resolver_{symbol}")
}

fn render_exports(symbols: &[String]) -> String {
    let mut asm = String::new();
    asm.push_str("core::arch::global_asm!(r#\"\n");
    asm.push_str(".hidden safe_gio_resolve_and_store\n");
    asm.push_str(".text\n");
    for symbol in symbols {
        let resolver = asm_resolver_name(symbol);
        let slot = asm_slot_name(symbol);
        let name = asm_symbol_name(symbol);
        asm.push_str(&format!(".globl {symbol}\n"));
        asm.push_str(&format!(".type {symbol}, @function\n"));
        asm.push_str(&format!("{symbol}:\n"));
        asm.push_str(&format!("    jmp *{slot}(%rip)\n"));
        asm.push_str(&format!(".size {symbol}, .-{symbol}\n"));
        asm.push_str(&format!("{resolver}:\n"));
        asm.push_str("    push %rax\n");
        asm.push_str("    push %rdi\n");
        asm.push_str("    push %rsi\n");
        asm.push_str("    push %rdx\n");
        asm.push_str("    push %rcx\n");
        asm.push_str("    push %r8\n");
        asm.push_str("    push %r9\n");
        asm.push_str("    sub $128, %rsp\n");
        asm.push_str("    movaps %xmm0, 0(%rsp)\n");
        asm.push_str("    movaps %xmm1, 16(%rsp)\n");
        asm.push_str("    movaps %xmm2, 32(%rsp)\n");
        asm.push_str("    movaps %xmm3, 48(%rsp)\n");
        asm.push_str("    movaps %xmm4, 64(%rsp)\n");
        asm.push_str("    movaps %xmm5, 80(%rsp)\n");
        asm.push_str("    movaps %xmm6, 96(%rsp)\n");
        asm.push_str("    movaps %xmm7, 112(%rsp)\n");
        asm.push_str(&format!("    lea {name}(%rip), %rdi\n"));
        asm.push_str(&format!("    lea {slot}(%rip), %rsi\n"));
        asm.push_str("    call safe_gio_resolve_and_store\n");
        asm.push_str("    mov %rax, %r11\n");
        asm.push_str("    movaps 0(%rsp), %xmm0\n");
        asm.push_str("    movaps 16(%rsp), %xmm1\n");
        asm.push_str("    movaps 32(%rsp), %xmm2\n");
        asm.push_str("    movaps 48(%rsp), %xmm3\n");
        asm.push_str("    movaps 64(%rsp), %xmm4\n");
        asm.push_str("    movaps 80(%rsp), %xmm5\n");
        asm.push_str("    movaps 96(%rsp), %xmm6\n");
        asm.push_str("    movaps 112(%rsp), %xmm7\n");
        asm.push_str("    add $128, %rsp\n");
        asm.push_str("    pop %r9\n");
        asm.push_str("    pop %r8\n");
        asm.push_str("    pop %rcx\n");
        asm.push_str("    pop %rdx\n");
        asm.push_str("    pop %rsi\n");
        asm.push_str("    pop %rdi\n");
        asm.push_str("    pop %rax\n");
        asm.push_str("    jmp *%r11\n");
    }
    asm.push_str(".section .data,\"aw\",@progbits\n");
    asm.push_str(".p2align 3\n");
    for symbol in symbols {
        let slot = asm_slot_name(symbol);
        let resolver = asm_resolver_name(symbol);
        asm.push_str(&format!("{slot}:\n"));
        asm.push_str(&format!("    .quad {resolver}\n"));
    }
    asm.push_str(".section .rodata\n");
    for symbol in symbols {
        let name = asm_symbol_name(symbol);
        asm.push_str(&format!("{name}:\n"));
        asm.push_str(&format!("    .asciz \"{symbol}\"\n"));
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
    let delegate = safe_root
        .join("vendor/build-check/gio/libgio-2.0.so.0.8000.0")
        .canonicalize()
        .expect("missing frozen GIO runtime delegate");
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
    println!("cargo:rerun-if-changed={}", delegate.display());
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");
    println!(
        "cargo:rustc-env=SAFE_FORWARD_ORIGINAL_LIB={}",
        delegate.display()
    );

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
    println!("cargo:rustc-link-lib=dylib=dl");
}
