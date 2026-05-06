pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTrashStack {
    pub next: *mut GTrashStack,
}
pub type GTrashStack = _GTrashStack;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trash_stack_push(
    mut stack_p: *mut *mut GTrashStack,
    mut data_p: gpointer,
) {
    let mut data: *mut GTrashStack = data_p as *mut GTrashStack;
    (*data).next = *stack_p;
    *stack_p = data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trash_stack_pop(
    mut stack_p: *mut *mut GTrashStack,
) -> gpointer {
    let mut data: *mut GTrashStack = ::core::ptr::null_mut::<GTrashStack>();
    data = *stack_p;
    if !data.is_null() {
        *stack_p = (*data).next;
        (*data).next = ::core::ptr::null_mut::<GTrashStack>();
    }
    return data as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trash_stack_peek(
    mut stack_p: *mut *mut GTrashStack,
) -> gpointer {
    let mut data: *mut GTrashStack = ::core::ptr::null_mut::<GTrashStack>();
    data = *stack_p;
    return data as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trash_stack_height(
    mut stack_p: *mut *mut GTrashStack,
) -> guint {
    let mut data: *mut GTrashStack = ::core::ptr::null_mut::<GTrashStack>();
    let mut i: guint = 0 as guint;
    data = *stack_p;
    while !data.is_null() {
        i = i.wrapping_add(1);
        data = (*data).next;
    }
    return i;
}
