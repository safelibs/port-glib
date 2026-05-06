extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_steal(array: *mut GPtrArray, len: *mut gsize) -> *mut gpointer;
    fn g_ptr_array_ref(array: *mut GPtrArray) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GStrv = *mut *mut gchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GStrvBuilder {
    pub array: GPtrArray,
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GStrvBuilder = _GStrvBuilder;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_strv_builder_addv\0" as *const u8 as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_new() -> *mut GStrvBuilder {
    return g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()))
        as *mut GStrvBuilder;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_unref(mut builder: *mut GStrvBuilder) {
    g_ptr_array_unref(&raw mut (*builder).array);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_ref(
    mut builder: *mut GStrvBuilder,
) -> *mut GStrvBuilder {
    return g_ptr_array_ref(&raw mut (*builder).array) as *mut GStrvBuilder;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_add(
    mut builder: *mut GStrvBuilder,
    mut value: *const ::core::ffi::c_char,
) {
    g_ptr_array_add(
        &raw mut (*builder).array,
        safe_c2rust_g_strdup_inline(value) as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_addv(
    mut builder: *mut GStrvBuilder,
    mut value: *mut *const ::core::ffi::c_char,
) {
    let mut i: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !builder.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"builder != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    i = 0 as gsize;
    while !(*value.offset(i as isize)).is_null() {
        safe_c2rust_g_strv_builder_add(builder, *value.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_add_many(
    mut builder: *mut GStrvBuilder,
    mut args: ...
) {
    let mut var_args: ::core::ffi::VaList;
    let mut str: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !builder.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"builder != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    var_args = args.clone();
    loop {
        str = var_args.arg::<*mut gchar>();
        if str.is_null() {
            break;
        }
        safe_c2rust_g_strv_builder_add(builder, str as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_take(
    mut builder: *mut GStrvBuilder,
    mut value: *mut ::core::ffi::c_char,
) {
    g_ptr_array_add(&raw mut (*builder).array, value as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_builder_end(mut builder: *mut GStrvBuilder) -> GStrv {
    g_ptr_array_add(&raw mut (*builder).array, NULL);
    return g_ptr_array_steal(&raw mut (*builder).array, ::core::ptr::null_mut::<gsize>()) as GStrv;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
