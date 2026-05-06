extern "C" {
    pub type _GData;
    pub type _GVariant;
    pub type _GIcon;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_list_find_custom(list: *mut GList, data: gconstpointer, func: GCompareFunc) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_type_name(type_0: GType) -> *const gchar;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_strv_get_type() -> GType;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_dup_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
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
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_icon_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
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
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariant = _GVariant;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInterfaceInfo {
    pub interface_init: GInterfaceInitFunc,
    pub interface_finalize: GInterfaceFinalizeFunc,
    pub interface_data: gpointer,
}
pub type GInterfaceFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInfo = _GInterfaceInfo;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_0 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_0 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_0 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_0 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_0 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_0 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_0 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_0 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_0 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_0 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_0 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_0 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_0 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_0 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_0 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_0 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_0 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_0 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_0 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_0 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_0 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_0 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_0 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_0 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_0 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_0 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_0 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_0 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_0 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_0 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_0 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_0 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_0 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_0 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_0 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_0 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_0 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_0 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_0 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_0 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_0 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_0 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_0 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_0 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_0 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_0 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_0 = 0;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThemedIcon {
    pub parent_instance: GObject,
    pub init_names: *mut *mut ::core::ffi::c_char,
    pub names: *mut *mut ::core::ffi::c_char,
    pub use_default_fallbacks: gboolean,
}
pub type GThemedIcon = _GThemedIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThemedIconClass {
    pub parent_class: GObjectClass,
}
pub type GThemedIconClass = _GThemedIconClass;
pub const PROP_USE_DEFAULT_FALLBACKS: C2RustUnnamed_1 = 3;
pub const PROP_NAMES: C2RustUnnamed_1 = 2;
pub const PROP_NAME: C2RustUnnamed_1 = 1;
pub type GIconIface = _GIconIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIconIface {
    pub g_iface: GTypeInterface,
    pub hash: Option<unsafe extern "C" fn(*mut GIcon) -> guint>,
    pub equal: Option<unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean>,
    pub to_tokens: Option<unsafe extern "C" fn(*mut GIcon, *mut GPtrArray, *mut gint) -> gboolean>,
    pub from_tokens:
        Option<unsafe extern "C" fn(*mut *mut gchar, gint, gint, *mut *mut GError) -> *mut GIcon>,
    pub serialize: Option<unsafe extern "C" fn(*mut GIcon) -> *mut GVariant>,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_themed_icon_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GThemedIcon\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GThemedIconClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_themed_icon_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GThemedIcon>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GThemedIcon) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_themed_icon_init as unsafe extern "C" fn(*mut GThemedIcon) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GIconIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_themed_icon_icon_iface_init
                as unsafe extern "C" fn(*mut GIconIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_icon_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_themed_icon_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GThemedIcon_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_themed_icon_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_themed_icon_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_themed_icon_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_themed_icon_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GThemedIcon_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GThemedIcon_private_offset);
    }
    safe_c2rust_g_themed_icon_class_init(klass as *mut GThemedIconClass);
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut icon: *mut GThemedIcon = object as *mut ::core::ffi::c_void as *mut GThemedIcon;
    match prop_id {
        2 => {
            g_value_set_boxed(value, (*icon).init_names as gconstpointer);
        }
        3 => {
            g_value_set_boolean(value, (*icon).use_default_fallbacks);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthemedicon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut icon: *mut GThemedIcon = object as *mut ::core::ffi::c_void as *mut GThemedIcon;
    let mut names: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    match prop_id {
        1 => {
            name = g_value_get_string(value);
            if !name.is_null() {
                if !(*icon).init_names.is_null() {
                    g_strfreev((*icon).init_names as *mut *mut gchar);
                }
                (*icon).init_names = ({
                    let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
                    let mut __s: gsize =
                        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
                    let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    if __s == 1 as gsize {
                        __p = g_malloc(__n);
                    } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                        __p = g_malloc(__n.wrapping_mul(__s));
                    } else {
                        __p = g_malloc_n(__n, __s);
                    }
                    __p
                }) as *mut *mut ::core::ffi::c_char;
                let ref mut fresh0 = *(*icon).init_names.offset(0 as ::core::ffi::c_int as isize);
                *fresh0 = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char);
                let ref mut fresh1 = *(*icon).init_names.offset(1 as ::core::ffi::c_int as isize);
                *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
        2 => {
            names = g_value_dup_boxed(value) as *mut *mut gchar;
            if !names.is_null() {
                if !(*icon).init_names.is_null() {
                    g_strfreev((*icon).init_names as *mut *mut gchar);
                }
                (*icon).init_names = names as *mut *mut ::core::ffi::c_char;
            }
        }
        3 => {
            (*icon).use_default_fallbacks = g_value_get_boolean(value);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthemedicon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_constructed(mut object: *mut GObject) {
    safe_c2rust_g_themed_icon_update_names(object as *mut ::core::ffi::c_void as *mut GThemedIcon);
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_finalize(mut object: *mut GObject) {
    let mut themed: *mut GThemedIcon = ::core::ptr::null_mut::<GThemedIcon>();
    themed = object as *mut ::core::ffi::c_void as *mut GThemedIcon;
    g_strfreev((*themed).init_names as *mut *mut gchar);
    g_strfreev((*themed).names as *mut *mut gchar);
    (*(safe_c2rust_g_themed_icon_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_class_init(mut klass: *mut GThemedIconClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_themed_icon_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).constructed =
        Some(safe_c2rust_g_themed_icon_constructed as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_themed_icon_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_themed_icon_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_NAMES as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"names\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_strv_get_type(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_USE_DEFAULT_FALLBACKS as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"use-default-fallbacks\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_init(mut themed: *mut GThemedIcon) {
    (*themed).init_names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*themed).names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_update_names(mut themed: *mut GThemedIcon) {
    let mut names: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut variants: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut iter: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !(*themed).init_names.is_null()
            && !(*(*themed)
                .init_names
                .offset(0 as ::core::ffi::c_int as isize))
            .is_null()
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
            b"themed->init_names != NULL && themed->init_names[0] != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    i = 0 as guint;
    while !(*(*themed).init_names.offset(i as isize)).is_null() {
        let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut is_symbolic: gboolean = 0;
        is_symbolic = if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = *(*themed).init_names.offset(i as isize);
                let __suffix: *const ::core::ffi::c_char =
                    b"-symbolic\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_11
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __suffix_len: size_t =
                        strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __suffix_len {
                        __result = (memcmp(
                            __str
                                .offset(__str_len as isize)
                                .offset(-(__suffix_len as isize))
                                as *const ::core::ffi::c_void,
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __suffix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_suffix(
                *(*themed).init_names.offset(i as isize),
                b"-symbolic\0" as *const u8 as *const gchar,
            )
        };
        if is_symbolic != 0 {
            name = g_strndup(
                *(*themed).init_names.offset(i as isize),
                (strlen(*(*themed).init_names.offset(i as isize)) as gsize)
                    .wrapping_sub(9 as gsize),
            );
        } else {
            name =
                safe_c2rust_g_strdup_inline(*(*themed).init_names.offset(i as isize)) as *mut gchar;
        }
        if !g_list_find_custom(
            names,
            name as gconstpointer,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                GCompareFunc,
            >(Some(
                g_strcmp0
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            )),
        )
        .is_null()
        {
            g_free(name as gpointer);
        } else {
            if is_symbolic != 0 {
                names = g_list_prepend(
                    names,
                    safe_c2rust_g_strdup_inline(*(*themed).init_names.offset(i as isize))
                        as gpointer,
                );
            } else {
                names = g_list_prepend(names, name as gpointer);
            }
            if (*themed).use_default_fallbacks != 0 {
                let mut dashp: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut last: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                last = name as *mut ::core::ffi::c_char;
                loop {
                    dashp = strrchr(last, '-' as i32);
                    if dashp.is_null() {
                        break;
                    }
                    let mut tmp: *mut gchar = last as *mut gchar;
                    let mut fallback: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    last = g_strndup(
                        last,
                        dashp.offset_from(last) as ::core::ffi::c_long as gsize,
                    ) as *mut ::core::ffi::c_char;
                    if is_symbolic != 0 {
                        g_free(tmp as gpointer);
                        fallback =
                            g_strdup_printf(b"%s-symbolic\0" as *const u8 as *const gchar, last);
                    } else {
                        fallback = last as *mut gchar;
                    }
                    if !g_list_find_custom(
                        names,
                        fallback as gconstpointer,
                        ::core::mem::transmute::<
                            Option<
                                unsafe extern "C" fn(
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                )
                                    -> ::core::ffi::c_int,
                            >,
                            GCompareFunc,
                        >(Some(
                            g_strcmp0
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                )
                                    -> ::core::ffi::c_int,
                        )),
                    )
                    .is_null()
                    {
                        g_free(fallback as gpointer);
                        break;
                    } else {
                        names = g_list_prepend(names, fallback as gpointer);
                    }
                }
                if is_symbolic != 0 {
                    g_free(last as gpointer);
                }
            } else if is_symbolic != 0 {
                g_free(name as gpointer);
            }
        }
        i = i.wrapping_add(1);
    }
    iter = names;
    while !iter.is_null() {
        let mut name_0: *mut gchar = (*iter).data as *mut gchar;
        let mut variant: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut is_symbolic_0: gboolean = 0;
        is_symbolic_0 = if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = name_0;
                let __suffix: *const ::core::ffi::c_char =
                    b"-symbolic\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_12
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __suffix_len: size_t =
                        strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __suffix_len {
                        __result = (memcmp(
                            __str
                                .offset(__str_len as isize)
                                .offset(-(__suffix_len as isize))
                                as *const ::core::ffi::c_void,
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __suffix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_suffix(name_0, b"-symbolic\0" as *const u8 as *const gchar)
        };
        if is_symbolic_0 != 0 {
            variant = g_strndup(name_0, (strlen(name_0) as gsize).wrapping_sub(9 as gsize));
        } else {
            variant = g_strdup_printf(b"%s-symbolic\0" as *const u8 as *const gchar, name_0);
        }
        if !g_list_find_custom(
            names,
            variant as gconstpointer,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                GCompareFunc,
            >(Some(
                g_strcmp0
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            )),
        )
        .is_null()
            || !g_list_find_custom(
                variants,
                variant as gconstpointer,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                    >,
                    GCompareFunc,
                >(Some(
                    g_strcmp0
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                )),
            )
            .is_null()
        {
            g_free(variant as gpointer);
        } else {
            variants = g_list_prepend(variants, variant as gpointer);
        }
        iter = (*iter).next;
    }
    names = g_list_reverse(names);
    g_strfreev((*themed).names as *mut *mut gchar);
    (*themed).names = ({
        let mut __n: gsize = g_list_length(names)
            .wrapping_add(g_list_length(variants))
            .wrapping_add(1 as guint) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut ::core::ffi::c_char;
    iter = names;
    i = 0 as guint;
    while !iter.is_null() {
        let ref mut fresh2 = *(*themed).names.offset(i as isize);
        *fresh2 = (*iter).data as *mut ::core::ffi::c_char;
        iter = (*iter).next;
        i = i.wrapping_add(1);
    }
    iter = variants;
    while !iter.is_null() {
        let ref mut fresh3 = *(*themed).names.offset(i as isize);
        *fresh3 = (*iter).data as *mut ::core::ffi::c_char;
        iter = (*iter).next;
        i = i.wrapping_add(1);
    }
    let ref mut fresh4 = *(*themed).names.offset(i as isize);
    *fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_list_free(names);
    g_list_free(variants);
    g_object_notify(
        themed as *mut ::core::ffi::c_void as *mut GObject,
        b"names\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_themed_icon_new(
    mut iconname: *const ::core::ffi::c_char,
) -> *mut GIcon {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !iconname.is_null() {
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
            b"iconname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    return g_object_new(
        safe_c2rust_g_themed_icon_get_type(),
        b"name\0" as *const u8 as *const gchar,
        iconname,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GIcon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_themed_icon_new_from_names(
    mut iconnames: *mut *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut GIcon {
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !iconnames.is_null() {
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
            b"iconnames != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    if len >= 0 as ::core::ffi::c_int {
        let mut names: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut i: ::core::ffi::c_int = 0;
        names = ({
            let mut __n: gsize = (len + 1 as ::core::ffi::c_int) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < len {
            let ref mut fresh7 = *names.offset(i as isize);
            *fresh7 = *iconnames.offset(i as isize);
            i += 1;
        }
        let ref mut fresh8 = *names.offset(i as isize);
        *fresh8 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        icon = g_object_new(
            safe_c2rust_g_themed_icon_get_type(),
            b"names\0" as *const u8 as *const gchar,
            names,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut GIcon;
        g_free(names as gpointer);
    } else {
        icon = g_object_new(
            safe_c2rust_g_themed_icon_get_type(),
            b"names\0" as *const u8 as *const gchar,
            iconnames,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut GIcon;
    }
    return icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_themed_icon_new_with_default_fallbacks(
    mut iconname: *const ::core::ffi::c_char,
) -> *mut GIcon {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !iconname.is_null() {
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
            b"iconname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    return g_object_new(
        safe_c2rust_g_themed_icon_get_type(),
        b"name\0" as *const u8 as *const gchar,
        iconname,
        b"use-default-fallbacks\0" as *const u8 as *const ::core::ffi::c_char,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GIcon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_themed_icon_get_names(
    mut icon: *mut GThemedIcon,
) -> *const *const gchar {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_themed_icon_get_type();
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
            b"G_IS_THEMED_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<*const gchar>();
    }
    return (*icon).names as *const *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_themed_icon_append_name(
    mut icon: *mut GThemedIcon,
    mut iconname: *const ::core::ffi::c_char,
) {
    let mut num_names: guint = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_themed_icon_get_type();
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
            b"G_IS_THEMED_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !iconname.is_null() {
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
            b"iconname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    num_names = g_strv_length((*icon).init_names as *mut *mut gchar);
    (*icon).init_names = g_realloc(
        (*icon).init_names as gpointer,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize)
            .wrapping_mul(num_names.wrapping_add(2 as guint) as gsize),
    ) as *mut *mut ::core::ffi::c_char;
    let ref mut fresh12 = *(*icon).init_names.offset(num_names as isize);
    *fresh12 = safe_c2rust_g_strdup_inline(iconname);
    let ref mut fresh13 = *(*icon)
        .init_names
        .offset(num_names.wrapping_add(1 as guint) as isize);
    *fresh13 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    safe_c2rust_g_themed_icon_update_names(icon);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_themed_icon_prepend_name(
    mut icon: *mut GThemedIcon,
    mut iconname: *const ::core::ffi::c_char,
) {
    let mut num_names: guint = 0;
    let mut names: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_themed_icon_get_type();
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
            b"G_IS_THEMED_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !iconname.is_null() {
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
            b"iconname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    num_names = g_strv_length((*icon).init_names as *mut *mut gchar);
    names = ({
        let mut __n: gsize = num_names.wrapping_add(2 as guint) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut ::core::ffi::c_char as *mut *mut gchar;
    i = 0 as ::core::ffi::c_int as gint;
    while !(*(*icon).init_names.offset(i as isize)).is_null() {
        let ref mut fresh9 =
            *names.offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        *fresh9 = *(*icon).init_names.offset(i as isize) as *mut gchar;
        i += 1;
    }
    let ref mut fresh10 = *names.offset(0 as ::core::ffi::c_int as isize);
    *fresh10 = safe_c2rust_g_strdup_inline(iconname) as *mut gchar;
    let ref mut fresh11 = *names.offset(num_names.wrapping_add(1 as guint) as isize);
    *fresh11 = ::core::ptr::null_mut::<gchar>();
    g_free((*icon).init_names as gpointer);
    (*icon).init_names = names as *mut *mut ::core::ffi::c_char;
    safe_c2rust_g_themed_icon_update_names(icon);
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_hash(mut icon: *mut GIcon) -> guint {
    let mut themed: *mut GThemedIcon = icon as *mut ::core::ffi::c_void as *mut GThemedIcon;
    let mut hash: guint = 0;
    let mut i: ::core::ffi::c_int = 0;
    hash = 0 as guint;
    i = 0 as ::core::ffi::c_int;
    while !(*(*themed).names.offset(i as isize)).is_null() {
        hash ^= g_str_hash(*(*themed).names.offset(i as isize) as gconstpointer);
        i += 1;
    }
    return hash;
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_equal(
    mut icon1: *mut GIcon,
    mut icon2: *mut GIcon,
) -> gboolean {
    let mut themed1: *mut GThemedIcon = icon1 as *mut ::core::ffi::c_void as *mut GThemedIcon;
    let mut themed2: *mut GThemedIcon = icon2 as *mut ::core::ffi::c_void as *mut GThemedIcon;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*(*themed1).names.offset(i as isize)).is_null()
        && !(*(*themed2).names.offset(i as isize)).is_null()
    {
        if !(strcmp(
            *(*themed1).names.offset(i as isize) as *const ::core::ffi::c_char,
            *(*themed2).names.offset(i as isize) as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            return FALSE;
        }
        i += 1;
    }
    return ((*(*themed1).names.offset(i as isize)).is_null()
        && (*(*themed2).names.offset(i as isize)).is_null()) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_to_tokens(
    mut icon: *mut GIcon,
    mut tokens: *mut GPtrArray,
    mut out_version: *mut gint,
) -> gboolean {
    let mut themed_icon: *mut GThemedIcon = icon as *mut ::core::ffi::c_void as *mut GThemedIcon;
    let mut n: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !out_version.is_null() {
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
            b"out_version != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    *out_version = 0 as ::core::ffi::c_int as gint;
    n = 0 as ::core::ffi::c_int;
    while !(*(*themed_icon).names.offset(n as isize)).is_null() {
        g_ptr_array_add(
            tokens,
            safe_c2rust_g_strdup_inline(*(*themed_icon).names.offset(n as isize)) as gpointer,
        );
        n += 1;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_from_tokens(
    mut tokens: *mut *mut gchar,
    mut num_tokens: gint,
    mut version: gint,
    mut error: *mut *mut GError,
) -> *mut GIcon {
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    let mut names: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n: ::core::ffi::c_int = 0;
    icon = ::core::ptr::null_mut::<GIcon>();
    if version != 0 as ::core::ffi::c_int {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t handle version %d of GThemedIcon encoding\0" as *const u8
                    as *const gchar,
            ),
            version,
        );
    } else {
        names = ({
            let mut __n: gsize =
                (num_tokens as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
        n = 0 as ::core::ffi::c_int;
        while n < num_tokens {
            let ref mut fresh5 = *names.offset(n as isize);
            *fresh5 = *tokens.offset(n as isize);
            n += 1;
        }
        let ref mut fresh6 = *names.offset(n as isize);
        *fresh6 = ::core::ptr::null_mut::<gchar>();
        icon = safe_c2rust_g_themed_icon_new_from_names(
            names as *mut *mut ::core::ffi::c_char,
            num_tokens as ::core::ffi::c_int,
        );
        g_free(names as gpointer);
    }
    return icon;
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_serialize(mut icon: *mut GIcon) -> *mut GVariant {
    let mut themed_icon: *mut GThemedIcon = icon as *mut ::core::ffi::c_void as *mut GThemedIcon;
    return g_variant_new(
        b"(sv)\0" as *const u8 as *const gchar,
        b"themed\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new(b"^as\0" as *const u8 as *const gchar, (*themed_icon).names),
    );
}
unsafe extern "C" fn safe_c2rust_g_themed_icon_icon_iface_init(mut iface: *mut GIconIface) {
    (*iface).hash =
        Some(safe_c2rust_g_themed_icon_hash as unsafe extern "C" fn(*mut GIcon) -> guint)
            as Option<unsafe extern "C" fn(*mut GIcon) -> guint>;
    (*iface).equal = Some(
        safe_c2rust_g_themed_icon_equal as unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean>;
    (*iface).to_tokens = Some(
        safe_c2rust_g_themed_icon_to_tokens
            as unsafe extern "C" fn(*mut GIcon, *mut GPtrArray, *mut gint) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GIcon, *mut GPtrArray, *mut gint) -> gboolean>;
    (*iface).from_tokens = Some(
        safe_c2rust_g_themed_icon_from_tokens
            as unsafe extern "C" fn(*mut *mut gchar, gint, gint, *mut *mut GError) -> *mut GIcon,
    )
        as Option<
            unsafe extern "C" fn(*mut *mut gchar, gint, gint, *mut *mut GError) -> *mut GIcon,
        >;
    (*iface).serialize = Some(
        safe_c2rust_g_themed_icon_serialize as unsafe extern "C" fn(*mut GIcon) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GIcon) -> *mut GVariant>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
