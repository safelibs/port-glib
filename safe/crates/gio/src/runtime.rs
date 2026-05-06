use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;
use std::sync::Once;

const RTLD_LOCAL: c_int = 0;
const RTLD_NOW: c_int = 2;
const RTLD_DEEPBIND: c_int = 8;

static INIT: Once = Once::new();
static mut HANDLE: *mut c_void = ptr::null_mut();

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

fn abort_with(message: &str) -> ! {
    eprintln!("{message}");
    std::process::abort();
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

unsafe fn open_system_library() -> *mut c_void {
    let path = concat!(env!("SAFE_SYSTEM_GIO_LIB"), "\0");
    let handle = unsafe { dlopen(path.as_ptr().cast(), RTLD_NOW | RTLD_LOCAL | RTLD_DEEPBIND) };
    if handle.is_null() {
        abort_with(&format!(
            "failed to load system GIO runtime delegate {}: {}",
            env!("SAFE_SYSTEM_GIO_LIB"),
            dlerror_message()
        ));
    }
    handle
}

unsafe fn system_handle() -> *mut c_void {
    INIT.call_once(|| unsafe {
        HANDLE = open_system_library();
    });
    unsafe { HANDLE }
}

unsafe fn require_function(handle: *mut c_void, symbol: &[u8]) -> usize {
    unsafe {
        dlerror();
    }
    let resolved = unsafe { dlsym(handle, symbol.as_ptr().cast()) };
    if resolved.is_null() {
        let name = String::from_utf8_lossy(&symbol[..symbol.len().saturating_sub(1)]);
        abort_with(&format!(
            "failed to resolve system GIO symbol {name}: {}",
            dlerror_message()
        ));
    }
    resolved as usize
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_gio_resolve_and_store(
    symbol: *const c_char,
    slot: *mut usize,
) -> usize {
    if symbol.is_null() || slot.is_null() {
        abort_with("safe GIO resolver received a null symbol or slot");
    }
    let symbol = unsafe { CStr::from_ptr(symbol).to_bytes_with_nul() };
    let resolved = unsafe { require_function(system_handle(), symbol) };
    unsafe {
        ptr::write(slot, resolved);
    }
    resolved
}
