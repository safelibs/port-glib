// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
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
    fn g_param_spec_ref(pspec: *mut GParamSpec) -> *mut GParamSpec;
    fn g_param_spec_unref(pspec: *mut GParamSpec);
    fn g_boxed_copy(boxed_type: GType, src_boxed: gconstpointer) -> gpointer;
    fn g_boxed_free(boxed_type: GType, boxed: gpointer);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_take_string(value: *mut GValue, v_string: *mut gchar);
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
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type va_list = __builtin_va_list;
pub type GData = _GData;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GParamFlags = ::core::ffi::c_int;
pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const G_PARAM_PRIVATE: GParamFlags = 32;
pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const G_PARAM_READWRITE: GParamFlags = 3;
pub const G_PARAM_WRITABLE: GParamFlags = 2;
pub const G_PARAM_READABLE: GParamFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
pub type GParamSpec = _GParamSpec;
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
pub type GMarshalFunc_VOID__VOID = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__VOID_0 = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__BOOLEAN =
    Option<unsafe extern "C" fn(gpointer, gboolean, gpointer) -> ()>;
pub type GMarshalFunc_VOID__BOOLEAN_0 =
    Option<unsafe extern "C" fn(gpointer, gboolean, gpointer) -> ()>;
pub type GMarshalFunc_VOID__CHAR = Option<unsafe extern "C" fn(gpointer, gchar, gpointer) -> ()>;
pub type GMarshalFunc_VOID__CHAR_0 = Option<unsafe extern "C" fn(gpointer, gchar, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UCHAR = Option<unsafe extern "C" fn(gpointer, guchar, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UCHAR_0 =
    Option<unsafe extern "C" fn(gpointer, guchar, gpointer) -> ()>;
pub type GMarshalFunc_VOID__INT = Option<unsafe extern "C" fn(gpointer, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__INT_0 = Option<unsafe extern "C" fn(gpointer, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UINT = Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UINT_0 = Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__LONG = Option<unsafe extern "C" fn(gpointer, glong, gpointer) -> ()>;
pub type GMarshalFunc_VOID__LONG_0 = Option<unsafe extern "C" fn(gpointer, glong, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ULONG = Option<unsafe extern "C" fn(gpointer, gulong, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ULONG_0 =
    Option<unsafe extern "C" fn(gpointer, gulong, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ENUM = Option<unsafe extern "C" fn(gpointer, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__ENUM_0 = Option<unsafe extern "C" fn(gpointer, gint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__FLAGS = Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__FLAGS_0 = Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> ()>;
pub type GMarshalFunc_VOID__FLOAT = Option<unsafe extern "C" fn(gpointer, gfloat, gpointer) -> ()>;
pub type GMarshalFunc_VOID__FLOAT_0 =
    Option<unsafe extern "C" fn(gpointer, gfloat, gpointer) -> ()>;
pub type GMarshalFunc_VOID__DOUBLE =
    Option<unsafe extern "C" fn(gpointer, gdouble, gpointer) -> ()>;
pub type GMarshalFunc_VOID__DOUBLE_0 =
    Option<unsafe extern "C" fn(gpointer, gdouble, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__STRING_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__PARAM =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__PARAM_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__BOXED =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__POINTER =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__POINTER_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__OBJECT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__VARIANT =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__VARIANT_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UINT_POINTER =
    Option<unsafe extern "C" fn(gpointer, guint, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_VOID__UINT_POINTER_0 =
    Option<unsafe extern "C" fn(gpointer, guint, gpointer, gpointer) -> ()>;
pub type GMarshalFunc_BOOLEAN__FLAGS =
    Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__FLAGS_0 =
    Option<unsafe extern "C" fn(gpointer, guint, gpointer) -> gboolean>;
pub type GMarshalFunc_STRING__OBJECT_POINTER =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> *mut gchar>;
pub type GMarshalFunc_STRING__OBJECT_POINTER_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> *mut gchar>;
pub type GMarshalFunc_BOOLEAN__BOXED_BOXED =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> gboolean>;
pub type GMarshalFunc_BOOLEAN__BOXED_BOXED_0 =
    Option<unsafe extern "C" fn(gpointer, gpointer, gpointer, gpointer) -> gboolean>;
#[inline(always)]
unsafe extern "C" fn g_strdup_inline(
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
pub unsafe extern "C" fn g_cclosure_marshal_VOID__VOID(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__VOID = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 1 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__VOID\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__VOID>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__VOIDv(
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
    let mut callback: GMarshalFunc_VOID__VOID_0 = None;
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__VOID_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__BOOLEAN(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__BOOLEAN = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__BOOLEAN\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__BOOLEAN>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__BOOLEANv(
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
    let mut callback: GMarshalFunc_VOID__BOOLEAN_0 = None;
    let mut arg0: gboolean = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gboolean>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__BOOLEAN_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__CHAR(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__CHAR = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__CHAR\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__CHAR>(
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
            .v_int as gchar,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__CHARv(
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
    let mut callback: GMarshalFunc_VOID__CHAR_0 = None;
    let mut arg0: gchar = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gint>() as gchar;
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__CHAR_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__UCHAR(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__UCHAR = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__UCHAR\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__UCHAR>(
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
            .v_uint as guchar,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__UCHARv(
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
    let mut callback: GMarshalFunc_VOID__UCHAR_0 = None;
    let mut arg0: guchar = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<guint>() as guchar;
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__UCHAR_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__INT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__INT = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__INT\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__INT>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__INTv(
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
    let mut callback: GMarshalFunc_VOID__INT_0 = None;
    let mut arg0: gint = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__INT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__UINT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__UINT = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__UINT\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__UINT>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__UINTv(
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
    let mut callback: GMarshalFunc_VOID__UINT_0 = None;
    let mut arg0: guint = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<guint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__UINT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__LONG(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__LONG = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__LONG\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__LONG>(
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
            .v_long,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__LONGv(
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
    let mut callback: GMarshalFunc_VOID__LONG_0 = None;
    let mut arg0: glong = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<glong>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__LONG_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__ULONG(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__ULONG = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__ULONG\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__ULONG>(
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
            .v_ulong,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__ULONGv(
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
    let mut callback: GMarshalFunc_VOID__ULONG_0 = None;
    let mut arg0: gulong = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gulong>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__ULONG_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__ENUM(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__ENUM = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__ENUM\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__ENUM>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__ENUMv(
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
    let mut callback: GMarshalFunc_VOID__ENUM_0 = None;
    let mut arg0: gint = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__ENUM_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__FLAGS(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__FLAGS = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__FLAGS\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__FLAGS>(
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
            .v_ulong as guint,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__FLAGSv(
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
    let mut callback: GMarshalFunc_VOID__FLAGS_0 = None;
    let mut arg0: guint = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<guint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__FLAGS_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__FLOAT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__FLOAT = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__FLOAT\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__FLOAT>(
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
            .v_float,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__FLOATv(
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
    let mut callback: GMarshalFunc_VOID__FLOAT_0 = None;
    let mut arg0: gfloat = 0.;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gdouble>() as gfloat;
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__FLOAT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__DOUBLE(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__DOUBLE = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__DOUBLE\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__DOUBLE>(
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
            .v_double,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__DOUBLEv(
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
    let mut callback: GMarshalFunc_VOID__DOUBLE_0 = None;
    let mut arg0: gdouble = 0.;
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gdouble>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__DOUBLE_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__STRING(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__STRING = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__STRING\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__STRING>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__STRINGv(
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
    let mut callback: GMarshalFunc_VOID__STRING_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = g_strdup_inline(arg0 as *const ::core::ffi::c_char) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__STRING_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        g_free(arg0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__PARAM(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__PARAM = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__PARAM\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__PARAM>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__PARAMv(
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
    let mut callback: GMarshalFunc_VOID__PARAM_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = g_param_spec_ref(arg0 as *mut GParamSpec) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__PARAM_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        g_param_spec_unref(arg0 as *mut GParamSpec);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__BOXED = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__BOXED\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__BOXED>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__BOXEDv(
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
    let mut callback: GMarshalFunc_VOID__BOXED_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = g_boxed_copy(
            *param_types.offset(0 as ::core::ffi::c_int as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
            arg0 as gconstpointer,
        );
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__BOXED_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        g_boxed_free(
            *param_types.offset(0 as ::core::ffi::c_int as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
            arg0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__POINTER(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__POINTER = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__POINTER\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__POINTER>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__POINTERv(
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
    let mut callback: GMarshalFunc_VOID__POINTER_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__POINTER_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__OBJECT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__OBJECT = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__OBJECT\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__OBJECT>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__OBJECTv(
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
    let mut callback: GMarshalFunc_VOID__OBJECT_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__OBJECT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__VARIANT(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__VARIANT = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__VARIANT\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__VARIANT>(
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
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__VARIANTv(
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
    let mut callback: GMarshalFunc_VOID__VARIANT_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = g_variant_ref_sink(arg0 as *mut GVariant) as gpointer;
    }
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__VARIANT_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        g_variant_unref(arg0 as *mut GVariant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__UINT_POINTER(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_VOID__UINT_POINTER = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if n_param_values == 3 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_VOID__UINT_POINTER\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__UINT_POINTER>(
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
            .v_pointer,
        data2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_VOID__UINT_POINTERv(
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
    let mut callback: GMarshalFunc_VOID__UINT_POINTER_0 = None;
    let mut arg0: guint = 0;
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    arg0 = args_copy.arg::<guint>();
    arg1 = args_copy.arg::<gpointer>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_VOID__UINT_POINTER_0>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_BOOLEAN__FLAGS(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_BOOLEAN__FLAGS = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if !return_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_BOOLEAN__FLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_param_values == 2 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_BOOLEAN__FLAGS\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__FLAGS>(
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
            .v_ulong as guint,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_BOOLEAN__FLAGSv(
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
    let mut callback: GMarshalFunc_BOOLEAN__FLAGS_0 = None;
    let mut arg0: guint = 0;
    let mut args_copy: ::core::ffi::VaList<'_>;
    let mut v_return: gboolean = 0;
    if !return_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_BOOLEAN__FLAGSv\0" as *const u8 as *const ::core::ffi::c_char,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    args_copy = args.clone();
    arg0 = args_copy.arg::<guint>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__FLAGS_0>(
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
pub unsafe extern "C" fn g_cclosure_marshal_STRING__OBJECT_POINTER(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_STRING__OBJECT_POINTER = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !return_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_STRING__OBJECT_POINTER\0" as *const u8
                as *const ::core::ffi::c_char,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_param_values == 3 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_STRING__OBJECT_POINTER\0" as *const u8
                as *const ::core::ffi::c_char,
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
        GMarshalFunc_STRING__OBJECT_POINTER,
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
    g_value_take_string(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_STRING__OBJECT_POINTERv(
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
    let mut callback: GMarshalFunc_STRING__OBJECT_POINTER_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    let mut v_return: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !return_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_STRING__OBJECT_POINTERv\0" as *const u8
                as *const ::core::ffi::c_char,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if !arg0.is_null() {
        arg0 = g_object_ref(arg0) as gpointer;
    }
    arg1 = args_copy.arg::<gpointer>();
    if (*closure).derivative_flag() != 0 {
        data1 = (*closure).data;
        data2 = instance;
    } else {
        data1 = instance;
        data2 = (*closure).data;
    }
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_STRING__OBJECT_POINTER_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if !arg0.is_null() {
        g_object_unref(arg0);
    }
    g_value_take_string(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_BOOLEAN__BOXED_BOXED(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GMarshalFunc_BOOLEAN__BOXED_BOXED = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut data1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut data2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut v_return: gboolean = 0;
    if !return_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_BOOLEAN__BOXED_BOXED\0" as *const u8 as *const ::core::ffi::c_char,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_param_values == 3 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_BOOLEAN__BOXED_BOXED\0" as *const u8 as *const ::core::ffi::c_char,
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
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GMarshalFunc_BOOLEAN__BOXED_BOXED>(
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
            .v_pointer,
        data2,
    );
    g_value_set_boolean(return_value, v_return);
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_BOOLEAN__BOXED_BOXEDv(
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
    let mut callback: GMarshalFunc_BOOLEAN__BOXED_BOXED_0 = None;
    let mut arg0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arg1: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut args_copy: ::core::ffi::VaList<'_>;
    let mut v_return: gboolean = 0;
    if !return_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_marshal_BOOLEAN__BOXED_BOXEDv\0" as *const u8
                as *const ::core::ffi::c_char,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    args_copy = args.clone();
    arg0 = args_copy.arg::<gpointer>();
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        arg0 = g_boxed_copy(
            *param_types.offset(0 as ::core::ffi::c_int as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
            arg0 as gconstpointer,
        );
    }
    arg1 = args_copy.arg::<gpointer>();
    if *param_types.offset(1 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg1.is_null()
    {
        arg1 = g_boxed_copy(
            *param_types.offset(1 as ::core::ffi::c_int as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
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
    callback = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        GMarshalFunc_BOOLEAN__BOXED_BOXED_0,
    >(if !marshal_data.is_null() {
        marshal_data as *mut ::core::ffi::c_void
    } else {
        (*cc).callback as *mut ::core::ffi::c_void
    });
    v_return = callback.expect("non-null function pointer")(data1, arg0, arg1, data2);
    if *param_types.offset(0 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg0.is_null()
    {
        g_boxed_free(
            *param_types.offset(0 as ::core::ffi::c_int as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
            arg0,
        );
    }
    if *param_types.offset(1 as ::core::ffi::c_int as isize)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
        == 0 as GType
        && !arg1.is_null()
    {
        g_boxed_free(
            *param_types.offset(1 as ::core::ffi::c_int as isize)
                & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType),
            arg1,
        );
    }
    g_value_set_boolean(return_value, v_return);
}
