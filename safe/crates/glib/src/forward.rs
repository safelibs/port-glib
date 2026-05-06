use std::ffi::{c_char, c_void};
use std::sync::OnceLock;

const RTLD_LAZY: i32 = 1;
const RTLD_LOCAL: i32 = 0;

unsafe extern "C" {
    fn abort() -> !;
    fn dlopen(filename: *const c_char, flags: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn original_handle() -> *mut c_void {
    static HANDLE: OnceLock<usize> = OnceLock::new();
    (*HANDLE.get_or_init(|| {
        for path in [
            c"/lib/x86_64-linux-gnu/libglib-2.0.so.0",
            c"/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0",
            c"/lib/aarch64-linux-gnu/libglib-2.0.so.0",
            c"/usr/lib/aarch64-linux-gnu/libglib-2.0.so.0",
        ] {
            let handle = unsafe { dlopen(path.as_ptr(), RTLD_LAZY | RTLD_LOCAL) };
            if !handle.is_null() {
                return handle as usize;
            }
        }
        unsafe { abort() }
    })) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_glib_resolve(symbol: *const c_char) -> *mut c_void {
    if symbol.is_null() {
        abort();
    }
    let target = dlsym(original_handle(), symbol);
    if target.is_null() {
        abort();
    }
    target
}

pub(crate) unsafe fn copy_original_symbol(symbol: *const c_char, dest: *mut c_void, size: usize) {
    let source = safe_glib_resolve(symbol);
    core::ptr::copy_nonoverlapping(source.cast::<u8>(), dest.cast::<u8>(), size);
}

#[cfg(target_arch = "x86_64")]
include!(concat!(env!("OUT_DIR"), "/glib_forwarders.rs"));

#[cfg(not(target_arch = "x86_64"))]
compile_error!("safe-glib dynamic forwarders currently require x86_64 SysV ABI");
