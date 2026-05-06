extern "C" {
    pub type _GConverter;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInterface {
    pub g_type: GType,
    pub g_instance_type: GType,
}
pub type GTypeInterface = _GTypeInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
pub type GConverterFlags = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSH: GConverterFlags = 2;
pub const G_CONVERTER_INPUT_AT_END: GConverterFlags = 1;
pub const G_CONVERTER_NO_FLAGS: GConverterFlags = 0;
pub type GConverterResult = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSHED: GConverterResult = 3;
pub const G_CONVERTER_FINISHED: GConverterResult = 2;
pub const G_CONVERTER_CONVERTED: GConverterResult = 1;
pub const G_CONVERTER_ERROR: GConverterResult = 0;
pub type GConverter = _GConverter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GConverterIface {
    pub g_iface: GTypeInterface,
    pub convert: Option<
        unsafe extern "C" fn(
            *mut GConverter,
            *const ::core::ffi::c_void,
            gsize,
            *mut ::core::ffi::c_void,
            gsize,
            GConverterFlags,
            *mut gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GConverterResult,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut GConverter) -> ()>,
}
pub type GConverterIface = _GConverterIface;
pub type GConverterInterface = GConverterIface;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GConverter\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GConverterInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GConverterInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_converter_default_init
                        as unsafe extern "C" fn(*mut GConverterInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_converter_default_init(mut iface: *mut GConverterInterface) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_convert(
    mut converter: *mut GConverter,
    mut inbuf: *const ::core::ffi::c_void,
    mut inbuf_size: gsize,
    mut outbuf: *mut ::core::ffi::c_void,
    mut outbuf_size: gsize,
    mut flags: GConverterFlags,
    mut bytes_read: *mut gsize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> GConverterResult {
    let mut iface: *mut GConverterIface = ::core::ptr::null_mut::<GConverterIface>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = converter as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_converter_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_CONVERTER (converter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_CONVERTER_ERROR;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !inbuf.is_null() || inbuf_size == 0 as gsize {
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
            b"inbuf != NULL || inbuf_size == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_CONVERTER_ERROR;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !outbuf.is_null() {
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
            b"outbuf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_CONVERTER_ERROR;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if outbuf_size > 0 as gsize {
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
            b"outbuf_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_CONVERTER_ERROR;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !bytes_read.is_null() {
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
            b"bytes_read != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_CONVERTER_ERROR;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !bytes_written.is_null() {
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
            b"bytes_written != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_CONVERTER_ERROR;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_CONVERTER_ERROR;
    }
    *bytes_read = 0 as gsize;
    *bytes_written = 0 as gsize;
    iface = g_type_interface_peek(
        (*(converter as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_converter_get_type(),
    ) as *mut GConverterIface;
    return Some((*iface).convert.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        converter,
        inbuf,
        inbuf_size,
        outbuf,
        outbuf_size,
        flags,
        bytes_read,
        bytes_written,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_reset(mut converter: *mut GConverter) {
    let mut iface: *mut GConverterIface = ::core::ptr::null_mut::<GConverterIface>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = converter as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_converter_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_CONVERTER (converter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(converter as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_converter_get_type(),
    ) as *mut GConverterIface;
    Some((*iface).reset.expect("non-null function pointer")).expect("non-null function pointer")(
        converter,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
