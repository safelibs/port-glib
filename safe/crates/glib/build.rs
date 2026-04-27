use std::collections::HashSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SymbolKind {
    Function,
    Data { size: usize },
}

#[derive(Clone)]
struct Symbol {
    name: String,
    kind: SymbolKind,
}

const RUST_OWNED_SYMBOLS: &[&str] = &[
    "g_byte_array_new_take",
    "g_canonicalize_filename",
    "g_filename_display_basename",
    "g_filename_display_name",
    "g_filename_from_utf8",
    "g_filename_to_utf8",
    "g_get_charset",
    "g_get_filename_charsets",
    "g_hash_table_add",
    "g_hash_table_contains",
    "g_hash_table_destroy",
    "g_hash_table_find",
    "g_hash_table_foreach",
    "g_hash_table_foreach_remove",
    "g_hash_table_foreach_steal",
    "g_hash_table_get_keys",
    "g_hash_table_get_keys_as_array",
    "g_hash_table_get_keys_as_ptr_array",
    "g_hash_table_get_values",
    "g_hash_table_get_values_as_ptr_array",
    "g_hash_table_insert",
    "g_hash_table_iter_get_hash_table",
    "g_hash_table_iter_init",
    "g_hash_table_iter_next",
    "g_hash_table_iter_remove",
    "g_hash_table_iter_replace",
    "g_hash_table_iter_steal",
    "g_hash_table_lookup",
    "g_hash_table_lookup_extended",
    "g_hash_table_new",
    "g_hash_table_new_full",
    "g_hash_table_new_similar",
    "g_hash_table_ref",
    "g_hash_table_remove",
    "g_hash_table_remove_all",
    "g_hash_table_replace",
    "g_hash_table_size",
    "g_hash_table_steal",
    "g_hash_table_steal_all",
    "g_hash_table_steal_all_keys",
    "g_hash_table_steal_all_values",
    "g_hash_table_steal_extended",
    "g_hash_table_unref",
    "g_key_file_load_from_data",
    "g_locale_from_utf8",
    "g_locale_to_utf8",
    "g_markup_parse_context_end_parse",
    "g_markup_parse_context_free",
    "g_markup_parse_context_new",
    "g_markup_parse_context_parse",
    "g_markup_parse_context_ref",
    "g_markup_parse_context_unref",
    "g_spawn_async",
    "g_spawn_async_with_fds",
    "g_spawn_async_with_pipes",
    "g_spawn_async_with_pipes_and_fds",
    "g_spawn_command_line_async",
    "g_spawn_command_line_sync",
    "g_spawn_sync",
    "g_ascii_table",
    "g_str_equal",
    "g_str_hash",
    "g_test_config_vars",
    "g_test_run",
    "g_test_run_suite",
    "g_thread_use_default_impl",
    "g_threads_got_initialized",
    "g_utf8_skip",
    "glib_binary_age",
    "glib_interface_age",
    "glib_major_version",
    "glib_micro_version",
    "glib_minor_version",
    "g_variant_byteswap",
    "g_variant_get_normal_form",
    "g_variant_is_normal_form",
    "g_variant_new_from_bytes",
    "g_variant_new_from_data",
];

fn parse_nm_symbols(nm: &str, oracle: &Path) -> Vec<Symbol> {
    let output = Command::new(nm)
        .args(["-D", "-S", "--defined-only"])
        .arg(oracle)
        .output()
        .unwrap_or_else(|error| panic!("failed to spawn {nm}: {error}"));
    if !output.status.success() {
        panic!(
            "{nm} failed with status {}:\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    let overrides: HashSet<&str> = RUST_OWNED_SYMBOLS.iter().copied().collect();
    let mut symbols = Vec::new();

    for line in String::from_utf8(output.stdout)
        .expect("nm output must be valid utf-8")
        .lines()
    {
        let mut parts = line.split_whitespace();
        let _address = parts.next();
        let size = parts
            .next()
            .and_then(|value| usize::from_str_radix(value, 16).ok())
            .unwrap_or(0);
        let kind = parts.next().and_then(|value| value.chars().next());
        let name = parts.next();
        let (Some(kind), Some(name)) = (kind, name) else {
            continue;
        };
        if overrides.contains(name) {
            continue;
        }

        let kind = match kind {
            'T' | 'W' | 'i' | 'I' => SymbolKind::Function,
            'B' | 'D' | 'G' | 'R' | 'S' | 'V' => SymbolKind::Data {
                size: size.max(1),
            },
            _ => continue,
        };
        symbols.push(Symbol {
            name: name.to_owned(),
            kind,
        });
    }

    symbols.sort_by(|left, right| left.name.cmp(&right.name));
    symbols
}

fn rust_byte_string(value: &str) -> String {
    let mut literal = String::from("b\"");
    for byte in value.bytes() {
        match byte {
            b'\\' => literal.push_str("\\\\"),
            b'"' => literal.push_str("\\\""),
            0x20..=0x7e => literal.push(char::from(byte)),
            _ => {
                let _ = write!(literal, "\\x{byte:02x}");
            }
        }
    }
    literal.push_str("\\0\"");
    literal
}

fn generate_forwarders(path: &Path, oracle: &Path, symbols: &[Symbol]) {
    let mut source = String::new();
    source.push_str("use core::arch::global_asm;\n");
    source.push_str("use core::ffi::{c_char, c_int, c_void};\n");
    source.push_str("use std::sync::OnceLock;\n\n");
    source.push_str("unsafe extern \"C\" {\n");
    source.push_str("    fn abort() -> !;\n");
    source.push_str("    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;\n");
    source.push_str("    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;\n");
    source.push_str("    fn getenv(name: *const c_char) -> *const c_char;\n");
    source.push_str("}\n\n");
    source.push_str("const RTLD_NOW: c_int = 2;\n");
    source.push_str("const RTLD_LOCAL: c_int = 0;\n");
    source.push_str("const ORACLE_ENV: &[u8] = b\"SAFE_GLIB_ORACLE_PATH\\0\";\n");
    let embedded = rust_byte_string(&oracle.display().to_string());
    source.push_str(&format!(
        "const EMBEDDED_ORACLE_PATH: &[u8] = {embedded};\n"
    ));
    source.push_str("const FALLBACK_ORACLE_PATHS: &[&[u8]] = &[\n");
    source.push_str("    EMBEDDED_ORACLE_PATH,\n");
    source.push_str("    b\"/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0\\0\",\n");
    source.push_str("    b\"/lib/x86_64-linux-gnu/libglib-2.0.so.0\\0\",\n");
    source.push_str("];\n\n");
    source.push_str("static ORACLE_HANDLE: OnceLock<usize> = OnceLock::new();\n\n");
    source.push_str("unsafe fn try_dlopen(path: *const c_char) -> *mut c_void {\n");
    source.push_str("    if path.is_null() {\n");
    source.push_str("        return core::ptr::null_mut();\n");
    source.push_str("    }\n");
    source.push_str("    dlopen(path, RTLD_NOW | RTLD_LOCAL)\n");
    source.push_str("}\n\n");
    source.push_str("unsafe fn oracle_handle() -> *mut c_void {\n");
    source.push_str("    let raw = *ORACLE_HANDLE.get_or_init(|| {\n");
    source.push_str("        let override_path = getenv(ORACLE_ENV.as_ptr().cast());\n");
    source.push_str("        if !override_path.is_null() {\n");
    source.push_str("            let handle = try_dlopen(override_path);\n");
    source.push_str("            if !handle.is_null() {\n");
    source.push_str("                return handle as usize;\n");
    source.push_str("            }\n");
    source.push_str("        }\n");
    source.push_str("        for candidate in FALLBACK_ORACLE_PATHS {\n");
    source.push_str("            let handle = try_dlopen(candidate.as_ptr().cast());\n");
    source.push_str("            if !handle.is_null() {\n");
    source.push_str("                return handle as usize;\n");
    source.push_str("            }\n");
    source.push_str("        }\n");
    source.push_str("        unsafe { abort() }\n");
    source.push_str("    });\n");
    source.push_str("    raw as *mut c_void\n");
    source.push_str("}\n\n");
    source.push_str("pub(crate) unsafe fn resolve_symbol(name: &'static [u8]) -> *mut c_void {\n");
    source.push_str("    let handle = oracle_handle();\n");
    source.push_str("    let resolved = dlsym(handle, name.as_ptr().cast());\n");
    source.push_str("    if resolved.is_null() {\n");
    source.push_str("        abort();\n");
    source.push_str("    }\n");
    source.push_str("    resolved\n");
    source.push_str("}\n\n");

    let mut assembly = String::from(".text\n");
    let mut init_body = String::new();

    for (index, symbol) in symbols.iter().enumerate() {
        let symbol_literal = rust_byte_string(&symbol.name);
        match symbol.kind {
            SymbolKind::Function => {
                let slot = format!("SAFE_GLIB_ORACLE_SLOT_{index}");
                let asm_slot = format!("safe_glib_oracle_slot_{index}");
                source.push_str("#[used]\n");
                source.push_str(&format!(
                    "#[unsafe(export_name = \"{asm_slot}\")]\nstatic mut {slot}: usize = 0;\n\n"
                ));
                let _ = writeln!(
                    assembly,
                    ".globl {name}\n.type {name}, @function\n{name}:\n  mov rax, qword ptr [rip + {slot}@GOTPCREL]\n  mov rax, qword ptr [rax]\n  jmp rax\n",
                    name = symbol.name,
                    slot = asm_slot,
                );
                let _ = writeln!(
                    init_body,
                    "    {slot} = resolve_symbol({symbol_literal}) as usize;",
                    slot = slot,
                );
            }
            SymbolKind::Data { size } => {
                let data = format!("SAFE_GLIB_ORACLE_DATA_{index}");
                source.push_str("#[used]\n");
                source.push_str(&format!(
                    "#[unsafe(export_name = \"{name}\")]\nstatic mut {data}: [u8; {size}] = [0; {size}];\n\n",
                    name = symbol.name,
                ));
                let _ = writeln!(
                    init_body,
                    "    core::ptr::copy_nonoverlapping(resolve_symbol({symbol_literal}).cast::<u8>(), core::ptr::addr_of_mut!({data}).cast::<u8>(), {size});",
                    data = data,
                );
            }
        }
    }

    source.push_str("unsafe extern \"C\" fn init_oracle_bindings() {\n");
    source.push_str("    let _ = oracle_handle();\n");
    source.push_str(&init_body);
    source.push_str("}\n\n");
    source.push_str("#[used]\n");
    source.push_str("#[cfg_attr(target_os = \"linux\", unsafe(link_section = \".init_array\"))]\n");
    source.push_str("static SAFE_GLIB_INIT_ORACLE_BINDINGS: unsafe extern \"C\" fn() = init_oracle_bindings;\n\n");
    source.push_str("global_asm!(r#\"");
    source.push_str(&assembly);
    source.push_str("\"#);\n");

    fs::write(path, source).unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let oracle = manifest_dir
        .join("../../vendor/build-check/glib/libglib-2.0.so.0.8000.0")
        .canonicalize()
        .expect("missing vendored libglib oracle");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing out dir"));
    let generated = out_dir.join("glib_forwarders.rs");
    let nm = env::var("NM").unwrap_or_else(|_| "nm".to_owned());

    println!("cargo:rerun-if-changed={}", oracle.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("../../abi/version-scripts/libglib.map").display()
    );
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");
    println!("cargo:rerun-if-env-changed=NM");

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

    let symbols = parse_nm_symbols(&nm, &oracle);
    generate_forwarders(&generated, &oracle, &symbols);
}
