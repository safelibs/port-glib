use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn internal_forwarders(src_dir: &Path) -> BTreeMap<String, String> {
    let mut files = Vec::new();
    read_dir_recursive(src_dir, &mut files);
    let mut forwarders = BTreeMap::new();
    for file in files {
        let text = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", file.display()));
        for link_name in collect_quoted_values(&text, "link_name = ") {
            if let Some(symbol) = link_name.strip_prefix("safe_glib_forward_") {
                forwarders.insert(link_name.clone(), symbol.to_owned());
            } else if let Some(symbol) = link_name.strip_prefix("safe_glib_legacy_") {
                forwarders.insert(link_name.clone(), symbol.to_owned());
            }
        }
    }
    forwarders
}

fn pkg_config_libdir() -> Option<PathBuf> {
    let output = Command::new("pkg-config")
        .args(["--variable=libdir", "glib-2.0"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let path = PathBuf::from(text.trim());
    (!path.as_os_str().is_empty()).then_some(path)
}

fn original_glib_path() -> PathBuf {
    if let Ok(path) = env::var("SAFE_GLIB_ORIGINAL_LIB") {
        return PathBuf::from(path);
    }

    let mut candidates = Vec::new();
    if let Some(libdir) = pkg_config_libdir() {
        candidates.push(libdir.join("libglib-2.0.so.0"));
    }
    candidates.extend([
        PathBuf::from("/lib/x86_64-linux-gnu/libglib-2.0.so.0"),
        PathBuf::from("/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0"),
        PathBuf::from("/lib/aarch64-linux-gnu/libglib-2.0.so.0"),
        PathBuf::from("/usr/lib/aarch64-linux-gnu/libglib-2.0.so.0"),
    ]);

    candidates
        .into_iter()
        .find(|path| path.exists())
        .expect("failed to find the host libglib-2.0.so.0 for dynamic ABI forwarding")
}

fn system_function_symbols(original_glib: &Path) -> BTreeSet<String> {
    let output = Command::new("nm")
        .args(["-D", "--defined-only"])
        .arg(original_glib)
        .output()
        .unwrap_or_else(|error| panic!("failed to run nm on {}: {error}", original_glib.display()));
    if !output.status.success() {
        panic!(
            "nm failed for {}:\n{}",
            original_glib.display(),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    let mut symbols = BTreeSet::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 3 {
            continue;
        }
        let kind = parts[parts.len() - 2];
        if !matches!(kind, "T" | "W" | "t") {
            continue;
        }
        let symbol = parts[parts.len() - 1].split('@').next().unwrap_or_default();
        if symbol.starts_with("g_") || symbol.starts_with("glib_") || symbol == "__glib_assert_msg"
        {
            symbols.insert(symbol.to_owned());
        }
    }
    symbols
}

fn write_forwarders(out_dir: &Path, forwarders: &BTreeMap<String, String>) {
    let mut asm = String::new();
    asm.push_str("core::arch::global_asm!(r#\"\n");
    asm.push_str(".text\n");

    for (index, (export, target)) in forwarders.iter().enumerate() {
        let name_label = format!(".Lsafe_glib_forward_name_{index}");
        asm.push_str(".p2align 4\n");
        asm.push_str(&format!(".globl {export}\n"));
        asm.push_str(&format!(".type {export}, @function\n"));
        asm.push_str(&format!("{export}:\n"));
        asm.push_str("    pushq %rax\n");
        asm.push_str("    pushq %rdi\n");
        asm.push_str("    pushq %rsi\n");
        asm.push_str("    pushq %rdx\n");
        asm.push_str("    pushq %rcx\n");
        asm.push_str("    pushq %r8\n");
        asm.push_str("    pushq %r9\n");
        asm.push_str("    subq $128, %rsp\n");
        asm.push_str("    movdqu %xmm0, 0(%rsp)\n");
        asm.push_str("    movdqu %xmm1, 16(%rsp)\n");
        asm.push_str("    movdqu %xmm2, 32(%rsp)\n");
        asm.push_str("    movdqu %xmm3, 48(%rsp)\n");
        asm.push_str("    movdqu %xmm4, 64(%rsp)\n");
        asm.push_str("    movdqu %xmm5, 80(%rsp)\n");
        asm.push_str("    movdqu %xmm6, 96(%rsp)\n");
        asm.push_str("    movdqu %xmm7, 112(%rsp)\n");
        asm.push_str(&format!("    leaq {name_label}(%rip), %rdi\n"));
        asm.push_str("    call {resolver}\n");
        asm.push_str("    movq %rax, %r10\n");
        asm.push_str("    movdqu 0(%rsp), %xmm0\n");
        asm.push_str("    movdqu 16(%rsp), %xmm1\n");
        asm.push_str("    movdqu 32(%rsp), %xmm2\n");
        asm.push_str("    movdqu 48(%rsp), %xmm3\n");
        asm.push_str("    movdqu 64(%rsp), %xmm4\n");
        asm.push_str("    movdqu 80(%rsp), %xmm5\n");
        asm.push_str("    movdqu 96(%rsp), %xmm6\n");
        asm.push_str("    movdqu 112(%rsp), %xmm7\n");
        asm.push_str("    addq $128, %rsp\n");
        asm.push_str("    popq %r9\n");
        asm.push_str("    popq %r8\n");
        asm.push_str("    popq %rcx\n");
        asm.push_str("    popq %rdx\n");
        asm.push_str("    popq %rsi\n");
        asm.push_str("    popq %rdi\n");
        asm.push_str("    popq %rax\n");
        asm.push_str("    jmp *%r10\n");
        asm.push_str(&format!(".size {export}, .-{export}\n"));
        asm.push_str(".section .rodata.safe_glib_forwarders,\"a\",@progbits\n");
        asm.push_str(&format!("{name_label}:\n"));
        asm.push_str(&format!("    .asciz \"{target}\"\n"));
        asm.push_str(".text\n");
    }

    asm.push_str("\"#, resolver = sym safe_glib_resolve_impl, options(att_syntax));\n");
    fs::write(out_dir.join("glib_forwarders.rs"), asm).expect("failed to write forwarders");
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let src_dir = manifest_dir.join("src");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing out dir"));

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("build.rs").display()
    );
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");
    println!("cargo:rerun-if-env-changed=SAFE_GLIB_ORIGINAL_LIB");

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
    println!("cargo:rustc-link-lib=dylib=pcre2-8");

    let original_glib = original_glib_path();
    let rust_exports = rust_owned_exports(&src_dir);
    let mut forwarders = BTreeMap::new();
    for symbol in system_function_symbols(&original_glib) {
        if !rust_exports.contains(&symbol) {
            forwarders.insert(symbol.clone(), symbol);
        }
    }
    forwarders.extend(internal_forwarders(&src_dir));
    write_forwarders(&out_dir, &forwarders);
}
