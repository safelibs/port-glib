#![allow(non_camel_case_types)]

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::thread::{Builder, JoinHandle};

use crate::abi::GError;
use crate::ffi::{gchar, gint, glongdouble, guint64, gpointer, GQuark};

fn lossless_or_lossy(ptr: *const gchar) -> Cow<'static, str> {
    if ptr.is_null() {
        return Cow::Borrowed("(null)");
    }
    let value = unsafe { CStr::from_ptr(ptr) };
    Cow::Owned(value.to_string_lossy().into_owned())
}

fn abort_with(message: &str) -> ! {
    eprintln!("{message}");
    std::process::abort()
}

fn strcmp_bytes(left: &[u8], right: &[u8]) -> gint {
    for (lhs, rhs) in left.iter().zip(right.iter()) {
        if lhs != rhs {
            return (*lhs as gint) - (*rhs as gint);
        }
    }
    (left.len() as gint) - (right.len() as gint)
}

pub type GThreadFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;

pub struct GThreadHandle {
    pub handle: Option<JoinHandle<usize>>,
}

pub unsafe extern "C" fn g_strcmp0_impl(str1: *const gchar, str2: *const gchar) -> gint {
    match (str1.is_null(), str2.is_null()) {
        (true, true) => 0,
        (true, false) => -1,
        (false, true) => 1,
        (false, false) => {
            let left = CStr::from_ptr(str1).to_bytes();
            let right = CStr::from_ptr(str2).to_bytes();
            strcmp_bytes(left, right)
        }
    }
}

pub unsafe extern "C" fn g_getenv_impl(variable: *const gchar) -> *const gchar {
    if variable.is_null() {
        return std::ptr::null();
    }
    let name = CStr::from_ptr(variable).to_string_lossy().into_owned();
    match std::env::var_os(name) {
        Some(value) => CString::new(value.to_string_lossy().as_bytes())
            .map(CString::into_raw)
            .map(|ptr| ptr as *const gchar)
            .unwrap_or(std::ptr::null()),
        None => std::ptr::null(),
    }
}

pub unsafe extern "C" fn g_thread_new_impl(
    name: *const gchar,
    func: GThreadFunc,
    data: gpointer,
) -> *mut GThreadHandle {
    let thread_func = match func {
        Some(func) => func,
        None => return std::ptr::null_mut(),
    };
    let mut builder = Builder::new();
    if !name.is_null() {
        builder = builder.name(CStr::from_ptr(name).to_string_lossy().into_owned());
    }
    let payload = data as usize;
    let handle = match builder.spawn(move || unsafe { thread_func(payload as gpointer) as usize }) {
        Ok(handle) => handle,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(GThreadHandle {
        handle: Some(handle),
    }))
}

pub unsafe extern "C" fn g_thread_join_impl(thread: *mut GThreadHandle) -> gpointer {
    if thread.is_null() {
        return std::ptr::null_mut();
    }
    let mut thread = Box::from_raw(thread);
    match thread.handle.take().unwrap().join() {
        Ok(result) => result as gpointer,
        Err(_) => std::ptr::null_mut(),
    }
}

pub unsafe extern "C" fn g_assertion_message_impl(
    domain: *const gchar,
    file: *const gchar,
    line: gint,
    func: *const gchar,
    message: *const gchar,
) -> ! {
    abort_with(&format!(
        "GLib assertion failed domain={} file={} line={} func={} message={}",
        lossless_or_lossy(domain),
        lossless_or_lossy(file),
        line,
        lossless_or_lossy(func),
        lossless_or_lossy(message),
    ))
}

pub unsafe extern "C" fn g_assertion_message_expr_impl(
    domain: *const gchar,
    file: *const gchar,
    line: gint,
    func: *const gchar,
    expr: *const gchar,
) -> ! {
    abort_with(&format!(
        "GLib assertion expression failed domain={} file={} line={} func={} expr={}",
        lossless_or_lossy(domain),
        lossless_or_lossy(file),
        line,
        lossless_or_lossy(func),
        lossless_or_lossy(expr),
    ))
}

pub unsafe extern "C" fn g_assertion_message_cmpstr_impl(
    domain: *const gchar,
    file: *const gchar,
    line: gint,
    func: *const gchar,
    expr: *const gchar,
    arg1: *const gchar,
    cmp: *const gchar,
    arg2: *const gchar,
) -> ! {
    abort_with(&format!(
        "GLib cmpstr assertion failed domain={} file={} line={} func={} expr={} left={} cmp={} right={}",
        lossless_or_lossy(domain),
        lossless_or_lossy(file),
        line,
        lossless_or_lossy(func),
        lossless_or_lossy(expr),
        lossless_or_lossy(arg1),
        lossless_or_lossy(cmp),
        lossless_or_lossy(arg2),
    ))
}

pub unsafe extern "C" fn g_assertion_message_cmpint_impl(
    domain: *const gchar,
    file: *const gchar,
    line: gint,
    func: *const gchar,
    expr: *const gchar,
    arg1: guint64,
    cmp: *const gchar,
    arg2: guint64,
    numtype: gchar,
) -> ! {
    abort_with(&format!(
        "GLib cmpint assertion failed domain={} file={} line={} func={} expr={} left={} cmp={} right={} type={}",
        lossless_or_lossy(domain),
        lossless_or_lossy(file),
        line,
        lossless_or_lossy(func),
        lossless_or_lossy(expr),
        arg1,
        lossless_or_lossy(cmp),
        arg2,
        numtype as u8 as char,
    ))
}

pub unsafe extern "C" fn g_assertion_message_cmpnum_impl(
    domain: *const gchar,
    file: *const gchar,
    line: gint,
    func: *const gchar,
    expr: *const gchar,
    _arg1: glongdouble,
    cmp: *const gchar,
    _arg2: glongdouble,
    numtype: gchar,
) -> ! {
    abort_with(&format!(
        "GLib cmpnum assertion failed domain={} file={} line={} func={} expr={} cmp={} type={}",
        lossless_or_lossy(domain),
        lossless_or_lossy(file),
        line,
        lossless_or_lossy(func),
        lossless_or_lossy(expr),
        lossless_or_lossy(cmp),
        numtype as u8 as char,
    ))
}

pub unsafe extern "C" fn g_assertion_message_error_impl(
    domain: *const gchar,
    file: *const gchar,
    line: gint,
    func: *const gchar,
    expr: *const gchar,
    error: *const GError,
    error_domain: GQuark,
    error_code: gint,
) -> ! {
    let error_message = if error.is_null() {
        Cow::Borrowed("(null)")
    } else {
        lossless_or_lossy((*error).message)
    };
    abort_with(&format!(
        "GLib error assertion failed domain={} file={} line={} func={} expr={} error_domain={} error_code={} message={}",
        lossless_or_lossy(domain),
        lossless_or_lossy(file),
        line,
        lossless_or_lossy(func),
        lossless_or_lossy(expr),
        error_domain,
        error_code,
        error_message,
    ))
}
