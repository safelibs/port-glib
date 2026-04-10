use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;
use std::sync::Once;

const RTLD_LOCAL: c_int = 0;
const RTLD_NOW: c_int = 2;

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

unsafe fn open_original_library() -> *mut c_void {
    let path = concat!(env!("SAFE_FORWARD_ORIGINAL_LIB"), "\0");
    let handle = dlopen(path.as_ptr().cast(), RTLD_NOW | RTLD_LOCAL);
    if handle.is_null() {
        abort_with(&format!(
            "failed to load frozen GObject runtime delegate {}: {}",
            env!("SAFE_FORWARD_ORIGINAL_LIB"),
            dlerror_message()
        ));
    }
    handle
}

pub(crate) unsafe fn initialize(init: unsafe fn(*mut c_void)) {
    INIT.call_once(|| unsafe {
        let handle = open_original_library();
        HANDLE = handle;
        init(handle);
    });
}

pub(crate) unsafe fn require_function(handle: *mut c_void, symbol: &[u8]) -> usize {
    dlerror();
    let resolved = dlsym(handle, symbol.as_ptr().cast());
    if resolved.is_null() {
        let name = String::from_utf8_lossy(&symbol[..symbol.len().saturating_sub(1)]);
        abort_with(&format!(
            "failed to resolve frozen GObject symbol {name}: {}",
            dlerror_message()
        ));
    }
    resolved as usize
}

pub(crate) unsafe fn initialize_live_exports(handle: *mut c_void) {
    let g_type_from_name: unsafe extern "C" fn(*const c_char) -> crate::ffi::GType =
        core::mem::transmute(require_function(handle, b"g_type_from_name\0"));
    for (index, name) in [
        b"GParamChar\0".as_slice(),
        b"GParamUChar\0".as_slice(),
        b"GParamBoolean\0".as_slice(),
        b"GParamInt\0".as_slice(),
        b"GParamUInt\0".as_slice(),
        b"GParamLong\0".as_slice(),
        b"GParamULong\0".as_slice(),
        b"GParamInt64\0".as_slice(),
        b"GParamUInt64\0".as_slice(),
        b"GParamUnichar\0".as_slice(),
        b"GParamEnum\0".as_slice(),
        b"GParamFlags\0".as_slice(),
        b"GParamFloat\0".as_slice(),
        b"GParamDouble\0".as_slice(),
        b"GParamString\0".as_slice(),
        b"GParamParam\0".as_slice(),
        b"GParamBoxed\0".as_slice(),
        b"GParamPointer\0".as_slice(),
        b"GParamValueArray\0".as_slice(),
        b"GParamObject\0".as_slice(),
        b"GParamOverride\0".as_slice(),
        b"GParamGType\0".as_slice(),
        b"GParamVariant\0".as_slice(),
    ]
    .into_iter()
    .enumerate()
    {
        let type_ = g_type_from_name(name.as_ptr().cast());
        if type_ == 0 {
            let rendered = String::from_utf8_lossy(&name[..name.len() - 1]);
            abort_with(&format!(
                "failed to resolve frozen GObject param spec type {rendered}"
            ));
        }
        crate::exports::__safe_gobject_param_spec_types_storage[index] = type_;
    }
    crate::exports::g_param_spec_types =
        core::ptr::addr_of_mut!(crate::exports::__safe_gobject_param_spec_types_storage)
            .cast::<crate::ffi::GType>();
}

pub(crate) unsafe fn copy_object(
    handle: *mut c_void,
    symbol: &[u8],
    destination: *mut u8,
    size: usize,
) {
    if size == 0 {
        return;
    }
    dlerror();
    let resolved = dlsym(handle, symbol.as_ptr().cast());
    if resolved.is_null() {
        let name = String::from_utf8_lossy(&symbol[..symbol.len().saturating_sub(1)]);
        abort_with(&format!(
            "failed to resolve frozen GObject object {name}: {}",
            dlerror_message()
        ));
    }
    ptr::copy_nonoverlapping(resolved.cast::<u8>(), destination, size);
}
