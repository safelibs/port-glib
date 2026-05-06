extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAction;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_print_string(
        value: *mut GVariant,
        string: *mut GString,
        type_annotate: gboolean,
    ) -> *mut GString;
    fn g_variant_parse_error_quark() -> GQuark;
    fn g_variant_parse(
        type_0: *const GVariantType,
        text: *const gchar,
        limit: *const gchar,
        endptr: *mut *const gchar,
        error: *mut *mut GError,
    ) -> *mut GVariant;
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
    fn g_variant_type_get_gtype() -> GType;
    fn g_object_interface_install_property(g_iface: gpointer, pspec: *mut GParamSpec);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_variant(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        type_0: *const GVariantType,
        default_value: *mut GVariant,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
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
pub type GData = _GData;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_VARIANT_PARSE_ERROR_RECURSION: C2RustUnnamed_0 = 18;
pub const G_VARIANT_PARSE_ERROR_VALUE_EXPECTED: C2RustUnnamed_0 = 17;
pub const G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT: C2RustUnnamed_0 = 16;
pub const G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD: C2RustUnnamed_0 = 15;
pub const G_VARIANT_PARSE_ERROR_UNEXPECTED_TOKEN: C2RustUnnamed_0 = 14;
pub const G_VARIANT_PARSE_ERROR_TYPE_ERROR: C2RustUnnamed_0 = 13;
pub const G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG: C2RustUnnamed_0 = 12;
pub const G_VARIANT_PARSE_ERROR_NUMBER_OUT_OF_RANGE: C2RustUnnamed_0 = 11;
pub const G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE: C2RustUnnamed_0 = 10;
pub const G_VARIANT_PARSE_ERROR_INVALID_TYPE_STRING: C2RustUnnamed_0 = 9;
pub const G_VARIANT_PARSE_ERROR_INVALID_SIGNATURE: C2RustUnnamed_0 = 8;
pub const G_VARIANT_PARSE_ERROR_INVALID_OBJECT_PATH: C2RustUnnamed_0 = 7;
pub const G_VARIANT_PARSE_ERROR_INVALID_FORMAT_STRING: C2RustUnnamed_0 = 6;
pub const G_VARIANT_PARSE_ERROR_INVALID_CHARACTER: C2RustUnnamed_0 = 5;
pub const G_VARIANT_PARSE_ERROR_INPUT_NOT_AT_END: C2RustUnnamed_0 = 4;
pub const G_VARIANT_PARSE_ERROR_DEFINITE_TYPE_EXPECTED: C2RustUnnamed_0 = 3;
pub const G_VARIANT_PARSE_ERROR_CANNOT_INFER_TYPE: C2RustUnnamed_0 = 2;
pub const G_VARIANT_PARSE_ERROR_BASIC_TYPE_EXPECTED: C2RustUnnamed_0 = 1;
pub const G_VARIANT_PARSE_ERROR_FAILED: C2RustUnnamed_0 = 0;
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
pub type GAction = _GAction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionInterface {
    pub g_iface: GTypeInterface,
    pub get_name: Option<unsafe extern "C" fn(*mut GAction) -> *const gchar>,
    pub get_parameter_type: Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>,
    pub get_state_type: Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>,
    pub get_state_hint: Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>,
    pub get_enabled: Option<unsafe extern "C" fn(*mut GAction) -> gboolean>,
    pub get_state: Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>,
    pub change_state: Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>,
    pub activate: Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>,
}
pub type GActionInterface = _GActionInterface;
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
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_c_inline(
    mut gstring: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if !gstring.is_null() && (*gstring).len.wrapping_add(1 as gsize) < (*gstring).allocated_len
        {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        let fresh0 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh0 as isize) = c;
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
    } else {
        g_string_insert_c(gstring, -(1 as ::core::ffi::c_int) as gssize, c);
    }
    return gstring;
}
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_ANY: *const GVariantType =
    b"*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_get_type() -> GType {
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
            g_intern_static_string(b"GAction\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GActionInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GActionInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_action_default_init
                        as unsafe extern "C" fn(*mut GActionInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL_0),
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
unsafe extern "C" fn safe_c2rust_g_action_default_init(mut iface: *mut GActionInterface) {
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_string(
            b"name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_boxed(
            b"parameter-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_variant_type_get_gtype(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_boolean(
            b"enabled\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_boxed(
            b"state-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_variant_type_get_gtype(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_variant(
            b"state\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_VARIANT_TYPE_ANY,
            ::core::ptr::null_mut::<GVariant>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_change_state(
    mut action: *mut GAction,
    mut value: *mut GVariant,
) {
    let mut state_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    state_type = safe_c2rust_g_action_get_state_type(action);
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !state_type.is_null() {
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
            b"state_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if g_variant_is_of_type(value, state_type) != 0 {
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
            b"g_variant_is_of_type (value, state_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_variant_ref_sink(value);
    (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .change_state
        .expect("non-null function pointer")(action, value);
    g_variant_unref(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_get_state(mut action: *mut GAction) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .get_state
        .expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_get_name(mut action: *mut GAction) -> *const gchar {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .get_name
        .expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_get_parameter_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    return (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .get_parameter_type
        .expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_get_state_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    return (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .get_state_type
        .expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_get_state_hint(
    mut action: *mut GAction,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .get_state_hint
        .expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_get_enabled(mut action: *mut GAction) -> gboolean {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .get_enabled
        .expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_activate(
    mut action: *mut GAction,
    mut parameter: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_get_type();
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
            b"G_IS_ACTION (action)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !parameter.is_null() {
        g_variant_ref_sink(parameter);
    }
    (*(g_type_interface_peek(
        (*(action as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_get_type(),
    ) as *mut GActionInterface))
        .activate
        .expect("non-null function pointer")(action, parameter);
    if !parameter.is_null() {
        g_variant_unref(parameter);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_name_is_valid(
    mut action_name: *const gchar,
) -> gboolean {
    let mut c: gchar = 0;
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    i = 0 as ::core::ffi::c_int as gint;
    loop {
        c = *action_name.offset(i as isize);
        if !(c != 0) {
            break;
        }
        if !(*safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALNUM as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
            && c as ::core::ffi::c_int != '.' as i32
            && c as ::core::ffi::c_int != '-' as i32
        {
            return FALSE;
        }
        i += 1;
    }
    return (i > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_parse_detailed_name(
    mut detailed_name: *const gchar,
    mut action_name: *mut *mut gchar,
    mut target_value: *mut *mut GVariant,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut target: *const gchar = ::core::ptr::null::<gchar>();
    let mut target_len: gsize = 0;
    let mut base_len: gsize = 0;
    if !(*detailed_name as ::core::ffi::c_int == '\0' as i32
        || *detailed_name as ::core::ffi::c_int == ' ' as i32)
    {
        base_len = strcspn(
            detailed_name as *const ::core::ffi::c_char,
            b": ()\0" as *const u8 as *const ::core::ffi::c_char,
        ) as gsize;
        target = detailed_name.offset(base_len as isize);
        target_len = strlen(target as *const ::core::ffi::c_char) as gsize;
        match *target.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
            32 | 41 => {}
            58 => {
                current_block = 10708110048831862781;
                match current_block {
                    10708110048831862781 => {
                        if *target.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            != ':' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_ref_sink(g_variant_new_string(
                                target.offset(2 as ::core::ffi::c_int as isize),
                            ));
                            current_block = 12800627514080957624;
                        }
                    }
                    6937071982253665452 => {
                        if *target.offset(target_len.wrapping_sub(1 as gsize) as isize)
                            as ::core::ffi::c_int
                            != ')' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_parse(
                                ::core::ptr::null::<GVariantType>(),
                                target.offset(1 as ::core::ffi::c_int as isize),
                                target
                                    .offset(target_len as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                ::core::ptr::null_mut::<*const gchar>(),
                                error,
                            );
                            if (*target_value).is_null() {
                                current_block = 9439570661566304930;
                            } else {
                                current_block = 12800627514080957624;
                            }
                        }
                    }
                    11286526357252652378 => {
                        *target_value = ::core::ptr::null_mut::<GVariant>();
                        current_block = 12800627514080957624;
                    }
                    _ => {}
                }
                match current_block {
                    9439570661566304930 => {}
                    _ => {
                        *action_name = g_strndup(detailed_name, base_len);
                        return TRUE;
                    }
                }
            }
            40 => {
                current_block = 6937071982253665452;
                match current_block {
                    10708110048831862781 => {
                        if *target.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            != ':' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_ref_sink(g_variant_new_string(
                                target.offset(2 as ::core::ffi::c_int as isize),
                            ));
                            current_block = 12800627514080957624;
                        }
                    }
                    6937071982253665452 => {
                        if *target.offset(target_len.wrapping_sub(1 as gsize) as isize)
                            as ::core::ffi::c_int
                            != ')' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_parse(
                                ::core::ptr::null::<GVariantType>(),
                                target.offset(1 as ::core::ffi::c_int as isize),
                                target
                                    .offset(target_len as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                ::core::ptr::null_mut::<*const gchar>(),
                                error,
                            );
                            if (*target_value).is_null() {
                                current_block = 9439570661566304930;
                            } else {
                                current_block = 12800627514080957624;
                            }
                        }
                    }
                    11286526357252652378 => {
                        *target_value = ::core::ptr::null_mut::<GVariant>();
                        current_block = 12800627514080957624;
                    }
                    _ => {}
                }
                match current_block {
                    9439570661566304930 => {}
                    _ => {
                        *action_name = g_strndup(detailed_name, base_len);
                        return TRUE;
                    }
                }
            }
            0 => {
                current_block = 11286526357252652378;
                match current_block {
                    10708110048831862781 => {
                        if *target.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            != ':' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_ref_sink(g_variant_new_string(
                                target.offset(2 as ::core::ffi::c_int as isize),
                            ));
                            current_block = 12800627514080957624;
                        }
                    }
                    6937071982253665452 => {
                        if *target.offset(target_len.wrapping_sub(1 as gsize) as isize)
                            as ::core::ffi::c_int
                            != ')' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_parse(
                                ::core::ptr::null::<GVariantType>(),
                                target.offset(1 as ::core::ffi::c_int as isize),
                                target
                                    .offset(target_len as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                ::core::ptr::null_mut::<*const gchar>(),
                                error,
                            );
                            if (*target_value).is_null() {
                                current_block = 9439570661566304930;
                            } else {
                                current_block = 12800627514080957624;
                            }
                        }
                    }
                    11286526357252652378 => {
                        *target_value = ::core::ptr::null_mut::<GVariant>();
                        current_block = 12800627514080957624;
                    }
                    _ => {}
                }
                match current_block {
                    9439570661566304930 => {}
                    _ => {
                        *action_name = g_strndup(detailed_name, base_len);
                        return TRUE;
                    }
                }
            }
            _ => {
                current_block = 12800627514080957624;
                match current_block {
                    10708110048831862781 => {
                        if *target.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            != ':' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_ref_sink(g_variant_new_string(
                                target.offset(2 as ::core::ffi::c_int as isize),
                            ));
                            current_block = 12800627514080957624;
                        }
                    }
                    6937071982253665452 => {
                        if *target.offset(target_len.wrapping_sub(1 as gsize) as isize)
                            as ::core::ffi::c_int
                            != ')' as i32
                        {
                            current_block = 9439570661566304930;
                        } else {
                            *target_value = g_variant_parse(
                                ::core::ptr::null::<GVariantType>(),
                                target.offset(1 as ::core::ffi::c_int as isize),
                                target
                                    .offset(target_len as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                ::core::ptr::null_mut::<*const gchar>(),
                                error,
                            );
                            if (*target_value).is_null() {
                                current_block = 9439570661566304930;
                            } else {
                                current_block = 12800627514080957624;
                            }
                        }
                    }
                    11286526357252652378 => {
                        *target_value = ::core::ptr::null_mut::<GVariant>();
                        current_block = 12800627514080957624;
                    }
                    _ => {}
                }
                match current_block {
                    9439570661566304930 => {}
                    _ => {
                        *action_name = g_strndup(detailed_name, base_len);
                        return TRUE;
                    }
                }
            }
        }
    }
    if !error.is_null() {
        if (*error).is_null() {
            g_set_error(
                error,
                g_variant_parse_error_quark(),
                G_VARIANT_PARSE_ERROR_FAILED as ::core::ffi::c_int as gint,
                b"Detailed action name '%s' has invalid format\0" as *const u8 as *const gchar,
                detailed_name,
            );
        } else {
            g_prefix_error(
                error,
                b"Detailed action name '%s' has invalid format: \0" as *const u8 as *const gchar,
                detailed_name,
            );
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_print_detailed_name(
    mut action_name: *const gchar,
    mut target_value: *mut GVariant,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if safe_c2rust_g_action_name_is_valid(action_name) != 0 {
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
            b"g_action_name_is_valid (action_name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if target_value.is_null() {
        return safe_c2rust_g_strdup_inline(action_name as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    if g_variant_is_of_type(target_value, G_VARIANT_TYPE_STRING) != 0 {
        let mut str: *const gchar =
            g_variant_get_string(target_value, ::core::ptr::null_mut::<gsize>());
        if safe_c2rust_g_action_name_is_valid(str) != 0 {
            return g_strconcat(
                action_name,
                b"::\0" as *const u8 as *const ::core::ffi::c_char,
                str,
                NULL_0,
            );
        }
    }
    let mut result: *mut GString = g_string_new(action_name);
    safe_c2rust_g_string_append_c_inline(result, '(' as i32 as gchar);
    g_variant_print_string(target_value, result, TRUE);
    safe_c2rust_g_string_append_c_inline(result, ')' as i32 as gchar);
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(result, 0 as gboolean)
        } else {
            g_string_free_and_steal(result)
        }
    } else {
        g_string_free(result, 0 as gboolean)
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
