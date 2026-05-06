#![allow(non_camel_case_types)]

pub struct GIRepositoryHandle {
    pub(crate) _opaque: usize,
}

pub fn new_repository_handle() -> *mut GIRepositoryHandle {
    Box::into_raw(Box::new(GIRepositoryHandle { _opaque: 1 }))
}
