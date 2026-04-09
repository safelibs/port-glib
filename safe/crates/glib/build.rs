use std::collections::BTreeMap;
use std::env;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::ptr;
use std::process::Command;

const MODULES: &[&str] = &["collections", "strings", "base", "mainloop", "threading"];
const RTLD_LOCAL: c_int = 0;
const RTLD_NOW: c_int = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExportKind {
    Function,
    Object(usize),
}

#[derive(Clone)]
struct Export {
    name: String,
    ident: String,
    kind: ExportKind,
    module: &'static str,
}

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
    fn dlclose(handle: *mut c_void) -> c_int;
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

fn classify(symbol: &str) -> &'static str {
    for (module, prefixes) in [
        (
            "collections",
            &[
                "g_allocator_",
                "g_array_",
                "g_byte_array_",
                "g_dataset_",
                "g_hash_table_",
                "g_list_",
                "g_node_",
                "g_ptr_array_",
                "g_queue_",
                "g_sequence_",
                "g_slist_",
                "g_tree_",
            ][..],
        ),
        (
            "strings",
            &[
                "g_ascii_",
                "g_convert_",
                "g_locale_",
                "g_markup_",
                "g_pattern_",
                "g_str",
                "g_string_",
                "g_string_chunk_",
                "g_unicode_",
                "g_unichar_",
                "g_uri_",
                "g_utf8_",
                "g_utf_",
            ][..],
        ),
        (
            "mainloop",
            &[
                "g_child_watch_",
                "g_idle_",
                "g_io_",
                "g_main_",
                "g_poll_",
                "g_source_",
                "g_timeout_",
                "g_unix_",
                "g_wakeup_",
            ][..],
        ),
        (
            "threading",
            &[
                "g_async_queue_",
                "g_atomic_",
                "g_bit_lock",
                "g_cond_",
                "g_mutex_",
                "g_once_",
                "g_private_",
                "g_rec_mutex_",
                "g_rw_lock_",
                "g_thread_",
                "g_threads_",
            ][..],
        ),
    ] {
        if prefixes.iter().any(|prefix| symbol.starts_with(prefix)) {
            return module;
        }
    }
    if symbol.contains("thread") {
        return "threading";
    }
    "base"
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
        let Some(kind) = (match parts[2].chars().next().unwrap_or_default() {
            'T' | 'W' | 'i' | 'I' => Some(ExportKind::Function),
            'A' | 'B' | 'C' | 'D' | 'G' | 'R' | 'S' | 'V' => {
                let size = usize::from_str_radix(parts[1], 16).unwrap_or(0);
                Some(ExportKind::Object(size))
            }
            _ => None,
        }) else {
            continue;
        };
        exports.entry(symbol.to_owned()).or_insert_with(|| Export {
            name: symbol.to_owned(),
            ident: sanitize_ident(symbol),
            kind,
            module: classify(symbol),
        });
    }

    exports.into_values().collect()
}

fn write_module(out_dir: &Path, original: &Path, module: &str, exports: &[Export]) {
    let mut body = String::new();

    for export in exports {
        match export.kind {
            ExportKind::Function => {
                let slot = format!("__safe_glib_slot_{}", export.ident);
                let _ = writeln!(
                    body,
                    "#[used]\nstatic mut {slot}: usize = 0;\n"
                );
                let _ = writeln!(
                    body,
                    "core::arch::global_asm!(\".globl {name}\", \".type {name}, @function\", \"{name}:\", \"  mov rax, qword ptr [rip + {{slot}}@GOTPCREL]\", \"  jmp qword ptr [rax]\", \".size {name}, .-{name}\", slot = sym {slot});\n",
                    name = export.name,
                );
            }
            ExportKind::Object(size) => {
                if let Some(definition) = special_object_definition(original, &export.name, size) {
                    body.push_str(&definition);
                    body.push('\n');
                    continue;
                }
                let storage = format!("__safe_glib_object_{}", export.ident);
                let _ = writeln!(
                    body,
                    "#[used]\nstatic mut {storage}: [u8; {size}] = [0; {size}];\n",
                );
                let _ = writeln!(
                    body,
                    "core::arch::global_asm!(\".globl {name}\", \".type {name}, @object\", \".set {name}, {{storage}}\", \".size {name}, {size}\", storage = sym {storage});\n",
                    name = export.name,
                );
            }
        }
    }

    body.push_str("pub(crate) unsafe fn initialize(handle: *mut core::ffi::c_void) {\n");
    for export in exports {
        match export.kind {
            ExportKind::Function => {
                let slot = format!("__safe_glib_slot_{}", export.ident);
                let _ = writeln!(
                    body,
                    "    {slot} = crate::runtime::require_function(handle, b\"{name}\\0\");",
                    name = export.name,
                );
            }
            ExportKind::Object(size) => {
                if special_object_definition(original, &export.name, size).is_some() {
                    continue;
                }
                let storage = format!("__safe_glib_object_{}", export.ident);
                let _ = writeln!(
                    body,
                    "    crate::runtime::copy_object(handle, b\"{name}\\0\", core::ptr::addr_of_mut!({storage}).cast::<u8>(), {size});",
                    name = export.name,
                );
            }
        }
    }
    body.push_str("}\n");

    fs::write(out_dir.join(format!("{module}-forwarders.rs")), body)
        .expect("failed to write generated forwarders");
}

fn write_modules(out_dir: &Path, original: &Path, exports: &[Export]) {
    for module in MODULES {
        let grouped: Vec<_> = exports
            .iter()
            .filter(|export| export.module == *module)
            .cloned()
            .collect();
        write_module(out_dir, original, module, &grouped);
    }
}

fn dlerror_message() -> String {
    unsafe {
        let message = dlerror();
        if message.is_null() {
            "unknown dynamic loader error".to_owned()
        } else {
            CStr::from_ptr(message).to_string_lossy().into_owned()
        }
    }
}

fn with_original_library<T>(path: &Path, f: impl FnOnce(*mut c_void) -> T) -> T {
    let path = CString::new(path.display().to_string()).unwrap();
    unsafe {
        let handle = dlopen(path.as_ptr(), RTLD_NOW | RTLD_LOCAL);
        if handle.is_null() {
            panic!("failed to load frozen original glib: {}", dlerror_message());
        }
        let result = f(handle);
        dlclose(handle);
        result
    }
}

fn require_symbol(handle: *mut c_void, symbol: &str) -> *mut c_void {
    let symbol = CString::new(symbol).unwrap();
    unsafe {
        dlerror();
        let resolved = dlsym(handle, symbol.as_ptr());
        if resolved.is_null() {
            panic!("failed to resolve {symbol:?}: {}", dlerror_message());
        }
        resolved
    }
}

fn read_scalar_bytes(path: &Path, symbol: &str, size: usize) -> Vec<u8> {
    with_original_library(path, |handle| unsafe {
        let resolved = require_symbol(handle, symbol).cast::<u8>();
        std::slice::from_raw_parts(resolved, size).to_vec()
    })
}

fn read_pointer_target(path: &Path, symbol: &str, size: usize) -> Vec<u8> {
    with_original_library(path, |handle| unsafe {
        let resolved = require_symbol(handle, symbol).cast::<*const u8>();
        let pointee = ptr::read(resolved);
        std::slice::from_raw_parts(pointee, size).to_vec()
    })
}

fn render_u8_array(values: &[u8]) -> String {
    let mut rendered = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            rendered.push_str(", ");
        }
        let _ = write!(rendered, "{value}");
    }
    rendered.push(']');
    rendered
}

fn render_u16_array(bytes: &[u8]) -> String {
    let mut rendered = String::from("[");
    for (index, chunk) in bytes.chunks_exact(2).enumerate() {
        if index > 0 {
            rendered.push_str(", ");
        }
        let value = u16::from_ne_bytes([chunk[0], chunk[1]]);
        let _ = write!(rendered, "{value}");
    }
    rendered.push(']');
    rendered
}

fn render_scalar_u32(path: &Path, symbol: &str) -> String {
    let bytes = read_scalar_bytes(path, symbol, 4);
    u32::from_ne_bytes(bytes.try_into().unwrap()).to_string()
}

fn render_scalar_i32(path: &Path, symbol: &str) -> String {
    let bytes = read_scalar_bytes(path, symbol, 4);
    i32::from_ne_bytes(bytes.try_into().unwrap()).to_string()
}

fn special_object_definition(path: &Path, symbol: &str, _size: usize) -> Option<String> {
    match symbol {
        "g_test_config_vars" => Some(String::from(
            "#[used]\nstatic __safe_glib_test_config: [i32; 6] = [0, 1, 0, 0, 0, 1];\n#[used]\n#[unsafe(export_name = \"g_test_config_vars\")]\npub static g_test_config_vars: crate::runtime::SyncPtr<[i32; 6]> = crate::runtime::SyncPtr(core::ptr::addr_of!(__safe_glib_test_config));\n",
        )),
        "g_ascii_table" => Some(format!(
            "#[used]\nstatic __safe_glib_ascii_table_data: [u16; 256] = {};\n#[used]\n#[unsafe(export_name = \"g_ascii_table\")]\npub static g_ascii_table: crate::runtime::SyncPtr<u16> = crate::runtime::SyncPtr(core::ptr::addr_of!(__safe_glib_ascii_table_data).cast::<u16>());\n",
            render_u16_array(&read_pointer_target(path, "g_ascii_table", 512))
        )),
        "g_utf8_skip" => Some(format!(
            "#[used]\nstatic __safe_glib_utf8_skip_data: [u8; {}] = {};\n#[used]\n#[unsafe(export_name = \"g_utf8_skip\")]\npub static g_utf8_skip: crate::runtime::SyncPtr<i8> = crate::runtime::SyncPtr(core::ptr::addr_of!(__safe_glib_utf8_skip_data).cast::<i8>());\n",
            256,
            render_u8_array(&read_pointer_target(path, "g_utf8_skip", 256))
        )),
        "glib_major_version" | "glib_minor_version" | "glib_micro_version" | "glib_interface_age"
        | "glib_binary_age" => Some(format!(
            "#[used]\n#[unsafe(export_name = \"{symbol}\")]\npub static {symbol}: u32 = {};\n",
            render_scalar_u32(path, symbol)
        )),
        "g_mem_gc_friendly" | "g_threads_got_initialized" | "g_thread_use_default_impl" => {
            Some(format!(
                "#[used]\n#[unsafe(export_name = \"{symbol}\")]\npub static mut {symbol}: i32 = {};\n",
                render_scalar_i32(path, symbol)
            ))
        }
        _ => None,
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let original = manifest_dir
        .join("../../vendor/build-check/glib/libglib-2.0.so.0.8000.0")
        .canonicalize()
        .expect("missing frozen original glib");
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
    write_modules(&out_dir, &original, &collect_exports(&original));
}
