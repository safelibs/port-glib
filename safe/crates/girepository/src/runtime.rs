#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub struct GIRepositoryHandle {
    _opaque: usize,
}

static mut GIREPOSITORY_DEFAULT: GIRepositoryHandle = GIRepositoryHandle { _opaque: 1 };

pub unsafe extern "C" fn gi_repository_new_impl() -> *mut GIRepositoryHandle {
    Box::into_raw(Box::new(GIRepositoryHandle { _opaque: 1 }))
}

pub unsafe extern "C" fn gi_repository_get_type_impl() -> crate::ffi::GType {
    0x3001
}

pub unsafe extern "C" fn gi_repository_default_impl() -> *mut GIRepositoryHandle {
    std::ptr::addr_of_mut!(GIREPOSITORY_DEFAULT)
}
