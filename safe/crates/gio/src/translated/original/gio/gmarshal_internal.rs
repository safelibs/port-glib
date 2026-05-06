use ::c2rust_bitfields;
extern "C" {
    pub type _GVariant;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_value_peek_pointer(value: *const GValue) -> gpointer;
    fn g_boxed_copy(boxed_type: GType, src_boxed: gconstpointer) -> gpointer;
    fn g_boxed_free(boxed_type: GType, boxed: gpointer);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_set_int(value: *mut GValue, v_int: gint);
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
pub type size_t = usize;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type va_list = __builtin_va_list;
pub type GVariant = _GVariant;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCClosure {
    pub closure: GClosure,
    pub callback: gpointer,
}
pub type GCClosure = _GCClosure;
pub type GMarshalFunc_BOOLEAN__OBJECT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__OBJECT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__OBJECT_FLAGS =
    Option<unsafe extern "C" fn(gpointer, gpointer, guint, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__OBJECT_FLAGS_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, guint, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__OBJECT_OBJECT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__OBJECT_OBJECT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__POINTER_INT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gint, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__POINTER_INT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gint, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__STRING =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__STRING_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__UINT =
    Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__UINT_0 =
    Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__VOID = Option<unsafe extern "C" fn(gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__VOID_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_INT__BOXED =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gint>;
pub type GMarshalFunc_INT__BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gint>;
pub type GMarshalFunc_INT__OBJECT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gint>;
pub type GMarshalFunc_INT__OBJECT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gint>;
pub type GMarshalFunc_VOID__BOOLEAN_BOXED =
    Option<unsafe extern "C" fn(gpointer, gboolean, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__BOOLEAN_BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gboolean, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ENUM_OBJECT =
    Option<unsafe extern "C" fn(gpointer, gint, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ENUM_OBJECT_0 =
    Option<unsafe extern "C" fn(gpointer, gint, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ENUM_OBJECT_OBJECT =
    Option<unsafe extern "C" fn(gpointer, gint, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ENUM_OBJECT_OBJECT_0 =
    Option<unsafe extern "C" fn(gpointer, gint, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__INT_INT_INT =
    Option<unsafe extern "C" fn(gpointer, gint, gint, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__INT_INT_INT_0 =
    Option<unsafe extern "C" fn(gpointer, gint, gint, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT_ENUM =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT_ENUM_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT = Option<
    unsafe extern "C" fn(
        gpointer,
        gpointer,
        gpointer,
        gpointer,
        gpointer,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT_0 = Option<
    unsafe extern "C" fn(
        gpointer,
        gpointer,
        gpointer,
        gpointer,
        gpointer,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT_VARIANT_BOXED =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_OBJECT_VARIANT_BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_VARIANT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_VARIANT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__POINTER_INT_STRING =
    Option<unsafe extern "C" fn(gpointer, gpointer, gint, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__POINTER_INT_STRING_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gint, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_BOOLEAN =
    Option<unsafe extern "C" fn(gpointer, gpointer, gboolean, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_BOOLEAN_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gboolean, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_BOXED =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_BOXED_BOXED =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_BOXED_BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_INT64_INT64 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gint64, gint64, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_INT64_INT64_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gint64, gint64, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_STRING_STRING_FLAGS =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_STRING_STRING_FLAGS_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_STRING_VARIANT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_STRING_VARIANT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_VARIANT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_VARIANT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UINT_UINT_UINT =
    Option<unsafe extern "C" fn(gpointer, guint, guint, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UINT_UINT_UINT_0 =
    Option<unsafe extern "C" fn(gpointer, guint, guint, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__VARIANT_BOXED =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__VARIANT_BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const G_TYPE_FLAG_RESERVED_ID_BIT: GType =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType;
pub const G_SIGNAL_TYPE_STATIC_SCOPE: GType =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__OBJECT = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
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
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if n_param_values == 2 as guint {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__OBJECT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__OBJECTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__OBJECT_0 = None;
    let mut v_return: gboolean = 0;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__OBJECT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(data1, arg0, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__OBJECT_FLAGS(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__OBJECT_FLAGS = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__OBJECT_FLAGS>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_ulong as guint,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__OBJECT_FLAGSv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__OBJECT_FLAGS_0 = None;
    let mut v_return: gboolean = 0;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: guint = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<guint>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_BOOLEAN__OBJECT_FLAGS_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__OBJECT_OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__OBJECT_OBJECT = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_BOOLEAN__OBJECT_OBJECT,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__OBJECT_OBJECTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__OBJECT_OBJECT_0 = None;
    let mut v_return: gboolean = 0;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if !arg1.is_null() {
        arg1 = g_object_ref(arg1) as gpointer;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_BOOLEAN__OBJECT_OBJECT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    if !arg1.is_null() {
        g_object_unref(arg1);
    }
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__POINTER_INT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__POINTER_INT = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__POINTER_INT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__POINTER_INTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__POINTER_INT_0 = None;
    let mut v_return: gboolean = 0;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gint = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    arg1 = args_copy.arg::<gint>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_BOOLEAN__POINTER_INT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__STRING(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__STRING = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if n_param_values == 2 as guint {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__STRING>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__STRINGv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__STRING_0 = None;
    let mut v_return: gboolean = 0;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__STRING_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(data1, arg0, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__UINT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__UINT = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if n_param_values == 2 as guint {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__UINT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_uint,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__UINTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__UINT_0 = None;
    let mut v_return: gboolean = 0;
    let mut arg0: guint = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<guint>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__UINT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(data1, arg0, data2);
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__VOID(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__VOID = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if n_param_values == 1 as guint {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__VOID>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(data1, data2);
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_BOOLEAN__VOIDv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_BOOLEAN__VOID_0 = None;
    let mut v_return: gboolean = 0;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__VOID_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(data1, data2);
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_INT__BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_INT__BOXED = None;
    let mut v_return: gint = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if n_param_values == 2 as guint {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_INT__BOXED>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
    g_value_set_int(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_INT__BOXEDv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_INT__BOXED_0 = None;
    let mut v_return: gint = 0;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = g_boxed_copy(
            *param_types.offset(0 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg0 as gconstpointer,
        );
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_INT__BOXED_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(data1, arg0, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_boxed_free(
            *param_types.offset(0 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg0,
        );
    }
    g_value_set_int(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_INT__OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_INT__OBJECT = None;
    let mut v_return: gint = 0;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if n_param_values == 2 as guint {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_INT__OBJECT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
    g_value_set_int(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_INT__OBJECTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_INT__OBJECT_0 = None;
    let mut v_return: gint = 0;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !return_value.is_null() {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_INT__OBJECT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")(data1, arg0, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    g_value_set_int(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__BOOLEAN_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__BOOLEAN_BOXED = None;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__BOOLEAN_BOXED>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int as gboolean,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__BOOLEAN_BOXEDv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__BOOLEAN_BOXED_0 = None;
    let mut arg0: gboolean = 0;
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gboolean>();
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = g_boxed_copy(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1 as gconstpointer,
        );
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__BOOLEAN_BOXED_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_boxed_free(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__ENUM_OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__ENUM_OBJECT = None;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__ENUM_OBJECT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_long as gint,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__ENUM_OBJECTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__ENUM_OBJECT_0 = None;
    let mut arg0: gint = 0;
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gint>();
    arg1 = args_copy.arg::<gpointer>();
    if !arg1.is_null() {
        arg1 = g_object_ref(arg1) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__ENUM_OBJECT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if !arg1.is_null() {
        g_object_unref(arg1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__ENUM_OBJECT_OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__ENUM_OBJECT_OBJECT = None;
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__ENUM_OBJECT_OBJECT,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_long as gint,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__ENUM_OBJECT_OBJECTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__ENUM_OBJECT_OBJECT_0 = None;
    let mut arg0: gint = 0;
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gint>();
    arg1 = args_copy.arg::<gpointer>();
    if !arg1.is_null() {
        arg1 = g_object_ref(arg1) as gpointer;
    }
    arg2 = args_copy.arg::<gpointer>();
    if !arg2.is_null() {
        arg2 = g_object_ref(arg2) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__ENUM_OBJECT_OBJECT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
    if !arg1.is_null() {
        g_object_unref(arg1);
    }
    if !arg2.is_null() {
        g_object_unref(arg2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__INT_INT_INT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__INT_INT_INT = None;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__INT_INT_INT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__INT_INT_INTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__INT_INT_INT_0 = None;
    let mut arg0: gint = 0;
    let mut arg1: gint = 0;
    let mut arg2: gint = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gint>();
    arg1 = args_copy.arg::<gint>();
    arg2 = args_copy.arg::<gint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__INT_INT_INT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT = None;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__OBJECT_OBJECT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if !arg1.is_null() {
        arg1 = g_object_ref(arg1) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__OBJECT_OBJECT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    if !arg1.is_null() {
        g_object_unref(arg1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECT_ENUM(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT_ENUM = None;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__OBJECT_OBJECT_ENUM,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_long as gint,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECT_ENUMv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT_ENUM_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg2: gint = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if !arg1.is_null() {
        arg1 = g_object_ref(arg1) as gpointer;
    }
    arg2 = args_copy.arg::<gint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__OBJECT_OBJECT_ENUM_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    if !arg1.is_null() {
        g_object_unref(arg1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT = None;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if n_param_values == 6 as guint {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 6\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(5 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECT_STRING_STRING_VARIANTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg3: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg4: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if !arg1.is_null() {
        arg1 = g_object_ref(arg1) as gpointer;
    }
    arg2 = args_copy.arg::<gpointer>();
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        arg2 = safe_c2rust_g_strdup_inline(arg2 as *const ::core::ffi::c_char) as gpointer;
    }
    arg3 = args_copy.arg::<gpointer>();
    if *param_types.offset(3 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg3.is_null()
    {
        arg3 = safe_c2rust_g_strdup_inline(arg3 as *const ::core::ffi::c_char) as gpointer;
    }
    arg4 = args_copy.arg::<gpointer>();
    if *param_types.offset(4 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg4.is_null()
    {
        arg4 = g_variant_ref_sink(arg4 as *mut GVariant) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__OBJECT_OBJECT_STRING_STRING_VARIANT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, arg3, arg4, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    if !arg1.is_null() {
        g_object_unref(arg1);
    }
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        g_free(arg2);
    }
    if *param_types.offset(3 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg3.is_null()
    {
        g_free(arg3);
    }
    if *param_types.offset(4 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg4.is_null()
    {
        g_variant_unref(arg4 as *mut GVariant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECT_VARIANT_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT_VARIANT_BOXED = None;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if n_param_values == 5 as guint {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 5\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__OBJECT_OBJECT_VARIANT_BOXED,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_OBJECT_VARIANT_BOXEDv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_OBJECT_VARIANT_BOXED_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg3: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if !arg1.is_null() {
        arg1 = g_object_ref(arg1) as gpointer;
    }
    arg2 = args_copy.arg::<gpointer>();
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        arg2 = g_variant_ref_sink(arg2 as *mut GVariant) as gpointer;
    }
    arg3 = args_copy.arg::<gpointer>();
    if *param_types.offset(3 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg3.is_null()
    {
        arg3 = g_boxed_copy(
            *param_types.offset(3 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg3 as gconstpointer,
        );
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__OBJECT_OBJECT_VARIANT_BOXED_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, arg3, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    if !arg1.is_null() {
        g_object_unref(arg1);
    }
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        g_variant_unref(arg2 as *mut GVariant);
    }
    if *param_types.offset(3 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg3.is_null()
    {
        g_boxed_free(
            *param_types.offset(3 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg3,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_VARIANT = None;
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__OBJECT_VARIANT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__OBJECT_VARIANTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__OBJECT_VARIANT_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = g_variant_ref_sink(arg1 as *mut GVariant) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__OBJECT_VARIANT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_variant_unref(arg1 as *mut GVariant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__POINTER_INT_STRING(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__POINTER_INT_STRING = None;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__POINTER_INT_STRING,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__POINTER_INT_STRINGv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__POINTER_INT_STRING_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gint = 0;
    let mut arg2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    arg1 = args_copy.arg::<gint>();
    arg2 = args_copy.arg::<gpointer>();
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        arg2 = safe_c2rust_g_strdup_inline(arg2 as *const ::core::ffi::c_char) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__POINTER_INT_STRING_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        g_free(arg2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_BOOLEAN(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_BOOLEAN = None;
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__STRING_BOOLEAN>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int as gboolean,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_BOOLEANv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_BOOLEAN_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gboolean = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    arg1 = args_copy.arg::<gboolean>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_BOOLEAN_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_BOXED = None;
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__STRING_BOXED>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_BOXEDv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_BOXED_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = g_boxed_copy(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1 as gconstpointer,
        );
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__STRING_BOXED_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_boxed_free(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_BOXED_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_BOXED_BOXED = None;
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_BOXED_BOXED,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_BOXED_BOXEDv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_BOXED_BOXED_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = g_boxed_copy(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1 as gconstpointer,
        );
    }
    arg2 = args_copy.arg::<gpointer>();
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        arg2 = g_boxed_copy(
            *param_types.offset(2 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg2 as gconstpointer,
        );
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_BOXED_BOXED_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_boxed_free(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1,
        );
    }
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        g_boxed_free(
            *param_types.offset(2 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg2,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_INT64_INT64(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_INT64_INT64 = None;
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_INT64_INT64,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int64,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_int64,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_INT64_INT64v(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_INT64_INT64_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gint64 = 0;
    let mut arg2: gint64 = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    arg1 = args_copy.arg::<gint64>();
    arg2 = args_copy.arg::<gint64>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_INT64_INT64_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_STRING_STRING_FLAGS(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_STRING_STRING_FLAGS = None;
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if n_param_values == 5 as guint {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 5\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_STRING_STRING_FLAGS,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(4 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_ulong as guint,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_STRING_STRING_FLAGSv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_STRING_STRING_FLAGS_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg3: guint = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = safe_c2rust_g_strdup_inline(arg1 as *const ::core::ffi::c_char) as gpointer;
    }
    arg2 = args_copy.arg::<gpointer>();
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        arg2 = safe_c2rust_g_strdup_inline(arg2 as *const ::core::ffi::c_char) as gpointer;
    }
    arg3 = args_copy.arg::<guint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_STRING_STRING_FLAGS_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, arg3, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_free(arg1);
    }
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        g_free(arg2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_STRING_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_STRING_VARIANT = None;
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_STRING_VARIANT,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_STRING_VARIANTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_STRING_VARIANT_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = safe_c2rust_g_strdup_inline(arg1 as *const ::core::ffi::c_char) as gpointer;
    }
    arg2 = args_copy.arg::<gpointer>();
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        arg2 = g_variant_ref_sink(arg2 as *mut GVariant) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_STRING_VARIANT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_free(arg1);
    }
    if *param_types.offset(2 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg2.is_null()
    {
        g_variant_unref(arg2 as *mut GVariant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_VARIANT = None;
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__STRING_VARIANT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__STRING_VARIANTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__STRING_VARIANT_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = g_variant_ref_sink(arg1 as *mut GVariant) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__STRING_VARIANT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_variant_unref(arg1 as *mut GVariant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__UINT_UINT_UINT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__UINT_UINT_UINT = None;
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if n_param_values == 4 as guint {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 4\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__UINT_UINT_UINT>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_uint,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_uint,
        (*param_values.offset(3 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_uint,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__UINT_UINT_UINTv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__UINT_UINT_UINT_0 = None;
    let mut arg0: guint = 0;
    let mut arg1: guint = 0;
    let mut arg2: guint = 0;
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<guint>();
    arg1 = args_copy.arg::<guint>();
    arg2 = args_copy.arg::<guint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_VOID__UINT_UINT_UINT_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    callback.expect("non-null function pointer")(data1, arg0, arg1, arg2, data2);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__VARIANT_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__VARIANT_BOXED = None;
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if n_param_values == 3 as guint {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_param_values == 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
    } else {
        data1 = g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize));
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__VARIANT_BOXED>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(
        data1,
        (*param_values.offset(1 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        (*param_values.offset(2 as ::core::ffi::c_int as isize)).data
            [0 as ::core::ffi::c_int as usize]
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_cclosure_marshal_VOID__VARIANT_BOXEDv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut callback: GMarshalFunc_VOID__VARIANT_BOXED_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = g_variant_ref_sink(arg0 as *mut GVariant) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = g_boxed_copy(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1 as gconstpointer,
        );
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__VARIANT_BOXED_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg0.is_null()
    {
        g_variant_unref(arg0 as *mut GVariant);
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize) & G_SIGNAL_TYPE_STATIC_SCOPE
        == 0 as GType
        && !arg1.is_null()
    {
        g_boxed_free(
            *param_types.offset(1 as ::core::ffi::c_int as isize) & !G_SIGNAL_TYPE_STATIC_SCOPE,
            arg1,
        );
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
