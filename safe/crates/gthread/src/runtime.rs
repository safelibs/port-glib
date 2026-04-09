#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::CStr;
use std::thread::{Builder, JoinHandle};

use crate::ffi::{gchar, gpointer};

pub type GThreadFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;

pub struct GThreadHandle {
    pub handle: Option<JoinHandle<usize>>,
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
        let name = CStr::from_ptr(name).to_string_lossy().into_owned();
        builder = builder.name(name);
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
