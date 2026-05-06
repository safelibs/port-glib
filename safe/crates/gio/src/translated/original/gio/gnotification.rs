extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GIcon;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new_full(
        reserved_size: guint,
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new_va(
        format_string: *const gchar,
        endptr: *mut *const gchar,
        app: *mut ::core::ffi::VaList,
    ) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_enum_get_value(enum_class: *mut GEnumClass, value: gint) -> *mut GEnumValue;
    fn g_icon_serialize(icon: *mut GIcon) -> *mut GVariant;
    fn g_action_name_is_valid(action_name: *const gchar) -> gboolean;
    fn g_action_parse_detailed_name(
        detailed_name: *const gchar,
        action_name: *mut *mut gchar,
        target_value: *mut *mut GVariant,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_notification_priority_get_type() -> GType;
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
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
pub type GLogLevelFlags = ::core::ffi::c_int;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor:
        Option<unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject>,
    pub set_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>,
    pub get_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub finalize: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub dispatch_properties_changed:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> ()>,
    pub notify: Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>,
    pub constructed: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub flags: gsize,
    pub n_construct_properties: gsize,
    pub pspecs: gpointer,
    pub n_pspecs: gsize,
    pub pdummy: [gpointer; 3],
}
pub type GObjectConstructParam = _GObjectConstructParam;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}
pub type GObjectClass = _GObjectClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumClass {
    pub g_type_class: GTypeClass,
    pub minimum: gint,
    pub maximum: gint,
    pub n_values: guint,
    pub values: *mut GEnumValue,
}
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GEnumClass = _GEnumClass;
pub type GNotificationPriority = ::core::ffi::c_uint;
pub const G_NOTIFICATION_PRIORITY_URGENT: GNotificationPriority = 3;
pub const G_NOTIFICATION_PRIORITY_HIGH: GNotificationPriority = 2;
pub const G_NOTIFICATION_PRIORITY_LOW: GNotificationPriority = 1;
pub const G_NOTIFICATION_PRIORITY_NORMAL: GNotificationPriority = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNotification {
    pub parent: GObject,
    pub title: *mut gchar,
    pub body: *mut gchar,
    pub icon: *mut GIcon,
    pub priority: GNotificationPriority,
    pub category: *mut gchar,
    pub buttons: *mut GPtrArray,
    pub default_action: *mut gchar,
    pub default_action_target: *mut GVariant,
}
pub type GIcon = _GIcon;
pub type GNotification = _GNotification;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Button {
    pub label: *mut gchar,
    pub action_name: *mut gchar,
    pub target: *mut GVariant,
}
pub type GNotificationClass = GObjectClass;
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
unsafe extern "C" fn safe_c2rust_g_notification_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_notification_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNotification_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNotification_private_offset,
        );
    }
    safe_c2rust_g_notification_class_init(klass as *mut GNotificationClass);
}
static mut safe_c2rust_GNotification_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_notification_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GNotification\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNotificationClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_notification_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNotification>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNotification) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_notification_init as unsafe extern "C" fn(*mut GNotification) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_notification_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_notification_get_type_once();
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
unsafe extern "C" fn safe_c2rust_button_free(mut data: gpointer) {
    let mut button: *mut Button = data as *mut Button;
    g_free((*button).label as gpointer);
    g_free((*button).action_name as gpointer);
    if !(*button).target.is_null() {
        g_variant_unref((*button).target);
    }
    g_slice_free1(
        ::core::mem::size_of::<Button>() as gsize,
        button as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_notification_dispose(mut object: *mut GObject) {
    let mut notification: *mut GNotification =
        object as *mut ::core::ffi::c_void as *mut GNotification;
    let mut _pp: *mut *mut GIcon = &raw mut (*notification).icon;
    let mut _ptr: *mut GIcon = *_pp;
    *_pp = ::core::ptr::null_mut::<GIcon>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    (*(safe_c2rust_g_notification_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_notification_finalize(mut object: *mut GObject) {
    let mut notification: *mut GNotification =
        object as *mut ::core::ffi::c_void as *mut GNotification;
    g_free((*notification).title as gpointer);
    g_free((*notification).body as gpointer);
    g_free((*notification).category as gpointer);
    g_free((*notification).default_action as gpointer);
    if !(*notification).default_action_target.is_null() {
        g_variant_unref((*notification).default_action_target);
    }
    g_ptr_array_free((*notification).buttons, TRUE);
    (*(safe_c2rust_g_notification_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_notification_class_init(mut klass: *mut GNotificationClass) {
    let mut object_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).dispose =
        Some(safe_c2rust_g_notification_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_notification_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_notification_init(mut notification: *mut GNotification) {
    (*notification).buttons = g_ptr_array_new_full(
        2 as guint,
        Some(safe_c2rust_button_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_new(
    mut title: *const gchar,
) -> *mut GNotification {
    let mut notification: *mut GNotification = ::core::ptr::null_mut::<GNotification>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !title.is_null() {
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
            b"title != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNotification>();
    }
    notification = g_object_new(
        safe_c2rust_g_notification_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GNotification;
    (*notification).title =
        safe_c2rust_g_strdup_inline(title as *const ::core::ffi::c_char) as *mut gchar;
    return notification;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_title(
    mut notification: *mut GNotification,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*notification).title;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_title(
    mut notification: *mut GNotification,
    mut title: *const gchar,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !title.is_null() {
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
            b"title != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*notification).title as gpointer);
    (*notification).title =
        safe_c2rust_g_strdup_inline(title as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_body(
    mut notification: *mut GNotification,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*notification).body;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_body(
    mut notification: *mut GNotification,
    mut body: *const gchar,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !body.is_null() {
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
            b"body != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*notification).body as gpointer);
    (*notification).body =
        safe_c2rust_g_strdup_inline(body as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_icon(
    mut notification: *mut GNotification,
) -> *mut GIcon {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    return (*notification).icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_icon(
    mut notification: *mut GNotification,
    mut icon: *mut GIcon,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*notification).icon.is_null() {
        g_object_unref((*notification).icon as gpointer);
    }
    (*notification).icon = g_object_ref(icon as gpointer) as *mut GIcon as *mut GIcon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_priority(
    mut notification: *mut GNotification,
) -> GNotificationPriority {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_NOTIFICATION_PRIORITY_NORMAL;
    }
    return (*notification).priority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_urgent(
    mut notification: *mut GNotification,
    mut urgent: gboolean,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*notification).priority = (if urgent != 0 {
        G_NOTIFICATION_PRIORITY_URGENT as ::core::ffi::c_int
    } else {
        G_NOTIFICATION_PRIORITY_NORMAL as ::core::ffi::c_int
    }) as GNotificationPriority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_category(
    mut notification: *mut GNotification,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*notification).category;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_category(
    mut notification: *mut GNotification,
    mut category: *const gchar,
) {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if category.is_null() || *category as ::core::ffi::c_int != '\0' as i32 {
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
            b"category == NULL || *category != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*notification).category as gpointer);
    (*notification).category =
        safe_c2rust_g_strdup_inline(category as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_priority(
    mut notification: *mut GNotification,
    mut priority: GNotificationPriority,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*notification).priority = priority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_add_button(
    mut notification: *mut GNotification,
    mut label: *const gchar,
    mut detailed_action: *const gchar,
) {
    let mut action: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut target: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !detailed_action.is_null() {
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
            b"detailed_action != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_action_parse_detailed_name(
        detailed_action,
        &raw mut action,
        &raw mut target,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: %s\0" as *const u8 as *const gchar,
            b"g_notification_add_button\0" as *const u8 as *const ::core::ffi::c_char,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    safe_c2rust_g_notification_add_button_with_target_value(notification, label, action, target);
    g_free(action as gpointer);
    if !target.is_null() {
        g_variant_unref(target);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_add_button_with_target(
    mut notification: *mut GNotification,
    mut label: *const gchar,
    mut action: *const gchar,
    mut target_format: *const gchar,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    let mut target: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !target_format.is_null() {
        args_0 = args.clone();
        target = g_variant_new_va(
            target_format,
            ::core::ptr::null_mut::<*const gchar>(),
            &raw mut args_0,
        );
    }
    safe_c2rust_g_notification_add_button_with_target_value(notification, label, action, target);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_add_button_with_target_value(
    mut notification: *mut GNotification,
    mut label: *const gchar,
    mut action: *const gchar,
    mut target: *mut GVariant,
) {
    let mut button: *mut Button = ::core::ptr::null_mut::<Button>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !label.is_null() {
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
            b"label != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !action.is_null() && g_action_name_is_valid(action) != 0 {
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
            b"action != NULL && g_action_name_is_valid (action)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = action as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char =
                b"app.\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_29 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_29 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_29
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(action, b"app.\0" as *const u8 as *const gchar)
    } == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: action '%s' does not start with 'app.'.This is unlikely to work properly.\0"
                as *const u8 as *const gchar,
            b"g_notification_add_button_with_target_value\0" as *const u8
                as *const ::core::ffi::c_char,
            action,
        );
    }
    button = ({
        let mut __s: gsize = ::core::mem::size_of::<Button>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut Button;
    (*button).label =
        safe_c2rust_g_strdup_inline(label as *const ::core::ffi::c_char) as *mut gchar;
    (*button).action_name =
        safe_c2rust_g_strdup_inline(action as *const ::core::ffi::c_char) as *mut gchar;
    if !target.is_null() {
        (*button).target = g_variant_ref_sink(target);
    }
    g_ptr_array_add((*notification).buttons, button as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_n_buttons(
    mut notification: *mut GNotification,
) -> guint {
    return (*(*notification).buttons).len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_button(
    mut notification: *mut GNotification,
    mut index: gint,
    mut label: *mut *mut gchar,
    mut action: *mut *mut gchar,
    mut target: *mut *mut GVariant,
) {
    let mut button: *mut Button = ::core::ptr::null_mut::<Button>();
    button = *(*(*notification).buttons).pdata.offset(index as isize) as *mut Button;
    if !label.is_null() {
        *label = safe_c2rust_g_strdup_inline((*button).label) as *mut gchar;
    }
    if !action.is_null() {
        *action = safe_c2rust_g_strdup_inline((*button).action_name) as *mut gchar;
    }
    if !target.is_null() {
        *target = if !(*button).target.is_null() {
            g_variant_ref((*button).target)
        } else {
            ::core::ptr::null_mut::<GVariant>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_button_with_action(
    mut notification: *mut GNotification,
    mut action: *const gchar,
) -> gint {
    let mut i: guint = 0;
    i = 0 as guint;
    while i < (*(*notification).buttons).len {
        let mut button: *mut Button = ::core::ptr::null_mut::<Button>();
        button = *(*(*notification).buttons).pdata.offset(i as isize) as *mut Button;
        if strcmp(
            action as *const ::core::ffi::c_char,
            (*button).action_name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return i as gint;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_get_default_action(
    mut notification: *mut GNotification,
    mut action: *mut *mut gchar,
    mut target: *mut *mut GVariant,
) -> gboolean {
    if (*notification).default_action.is_null() {
        return FALSE;
    }
    if !action.is_null() {
        *action = safe_c2rust_g_strdup_inline((*notification).default_action) as *mut gchar;
    }
    if !target.is_null() {
        if !(*notification).default_action_target.is_null() {
            *target = g_variant_ref((*notification).default_action_target);
        } else {
            *target = ::core::ptr::null_mut::<GVariant>();
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_default_action(
    mut notification: *mut GNotification,
    mut detailed_action: *const gchar,
) {
    let mut action: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut target: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if g_action_parse_detailed_name(
        detailed_action,
        &raw mut action,
        &raw mut target,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: %s\0" as *const u8 as *const gchar,
            b"g_notification_set_default_action\0" as *const u8 as *const ::core::ffi::c_char,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    safe_c2rust_g_notification_set_default_action_and_target_value(notification, action, target);
    g_free(action as gpointer);
    if !target.is_null() {
        g_variant_unref(target);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_default_action_and_target(
    mut notification: *mut GNotification,
    mut action: *const gchar,
    mut target_format: *const gchar,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    let mut target: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !target_format.is_null() {
        args_0 = args.clone();
        target = g_variant_new_va(
            target_format,
            ::core::ptr::null_mut::<*const gchar>(),
            &raw mut args_0,
        );
    }
    safe_c2rust_g_notification_set_default_action_and_target_value(notification, action, target);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_set_default_action_and_target_value(
    mut notification: *mut GNotification,
    mut action: *const gchar,
    mut target: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_notification_get_type();
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
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !action.is_null() && g_action_name_is_valid(action) != 0 {
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
            b"action != NULL && g_action_name_is_valid (action)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = action as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char =
                b"app.\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_32 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_32 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_32
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(action, b"app.\0" as *const u8 as *const gchar)
    } == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: action '%s' does not start with 'app.'.This is unlikely to work properly.\0"
                as *const u8 as *const gchar,
            b"g_notification_set_default_action_and_target_value\0" as *const u8
                as *const ::core::ffi::c_char,
            action,
        );
    }
    g_free((*notification).default_action as gpointer);
    let mut _pp: *mut *mut GVariant = &raw mut (*notification).default_action_target;
    let mut _ptr: *mut GVariant = *_pp;
    *_pp = ::core::ptr::null_mut::<GVariant>();
    if !_ptr.is_null() {
        g_variant_unref(_ptr as *mut GVariant);
    }
    (*notification).default_action =
        safe_c2rust_g_strdup_inline(action as *const ::core::ffi::c_char) as *mut gchar;
    if !target.is_null() {
        (*notification).default_action_target = g_variant_ref_sink(target);
    }
}
unsafe extern "C" fn safe_c2rust_g_notification_serialize_button(
    mut button: *mut Button,
) -> *mut GVariant {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_add(
        &raw mut builder,
        b"{sv}\0" as *const u8 as *const gchar,
        b"label\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new_string((*button).label),
    );
    g_variant_builder_add(
        &raw mut builder,
        b"{sv}\0" as *const u8 as *const gchar,
        b"action\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new_string((*button).action_name),
    );
    if !(*button).target.is_null() {
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"target\0" as *const u8 as *const ::core::ffi::c_char,
            (*button).target,
        );
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_g_notification_get_priority_nick(
    mut notification: *mut GNotification,
) -> *mut GVariant {
    let mut enum_class: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
    let mut value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
    let mut nick: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    enum_class = g_type_class_ref(g_notification_priority_get_type()) as *mut GEnumClass;
    value = g_enum_get_value(
        enum_class,
        safe_c2rust_g_notification_get_priority(notification) as gint,
    );
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gnotification.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            790 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    nick = g_variant_new_string((*value).value_nick);
    g_type_class_unref(enum_class as gpointer);
    return nick;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_serialize(
    mut notification: *mut GNotification,
) -> *mut GVariant {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    if !(*notification).title.is_null() {
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"title\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_string((*notification).title),
        );
    }
    if !(*notification).body.is_null() {
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"body\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_string((*notification).body),
        );
    }
    if !(*notification).icon.is_null() {
        let mut serialized_icon: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        serialized_icon = g_icon_serialize((*notification).icon);
        if !serialized_icon.is_null() {
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                b"icon\0" as *const u8 as *const ::core::ffi::c_char,
                serialized_icon,
            );
            g_variant_unref(serialized_icon);
        }
    }
    g_variant_builder_add(
        &raw mut builder,
        b"{sv}\0" as *const u8 as *const gchar,
        b"priority\0" as *const u8 as *const ::core::ffi::c_char,
        safe_c2rust_g_notification_get_priority_nick(notification),
    );
    if !(*notification).default_action.is_null() {
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"default-action\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_string((*notification).default_action),
        );
        if !(*notification).default_action_target.is_null() {
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                b"default-action-target\0" as *const u8 as *const ::core::ffi::c_char,
                (*notification).default_action_target,
            );
        }
    }
    if (*(*notification).buttons).len > 0 as guint {
        let mut actions_builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed {
                s: C2RustUnnamed_0 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        let mut i: guint = 0;
        g_variant_builder_init(
            &raw mut actions_builder,
            g_variant_type_checked_(b"aa{sv}\0" as *const u8 as *const gchar),
        );
        i = 0 as guint;
        while i < (*(*notification).buttons).len {
            let mut button: *mut Button =
                *(*(*notification).buttons).pdata.offset(i as isize) as *mut Button;
            g_variant_builder_add(
                &raw mut actions_builder,
                b"@a{sv}\0" as *const u8 as *const gchar,
                safe_c2rust_g_notification_serialize_button(button),
            );
            i = i.wrapping_add(1);
        }
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"buttons\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_builder_end(&raw mut actions_builder),
        );
    }
    return g_variant_builder_end(&raw mut builder);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
