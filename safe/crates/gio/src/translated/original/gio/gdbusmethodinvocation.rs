extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GUnixFDListPrivate;
    pub type _GDBusMessage;
    pub type _GDBusConnection;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
    fn g_variant_new_tuple(children: *const *mut GVariant, n_children: gsize) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_member_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_interface_name(string: *const gchar) -> gboolean;
    fn g_dbus_connection_get_type() -> GType;
    fn g_dbus_connection_send_message(
        connection: *mut GDBusConnection,
        message: *mut GDBusMessage,
        flags: GDBusSendMessageFlags,
        out_serial: *mut guint32,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_message_get_type() -> GType;
    fn g_dbus_message_new_method_reply(method_call_message: *mut GDBusMessage)
        -> *mut GDBusMessage;
    fn g_dbus_message_new_method_error_literal(
        method_call_message: *mut GDBusMessage,
        error_name: *const gchar,
        error_message: *const gchar,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_get_flags(message: *mut GDBusMessage) -> GDBusMessageFlags;
    fn g_dbus_message_get_serial(message: *mut GDBusMessage) -> guint32;
    fn g_dbus_message_set_body(message: *mut GDBusMessage, body: *mut GVariant);
    fn g_dbus_message_set_unix_fd_list(message: *mut GDBusMessage, fd_list: *mut GUnixFDList);
    fn g_dbus_method_info_ref(info: *mut GDBusMethodInfo) -> *mut GDBusMethodInfo;
    fn g_dbus_property_info_ref(info: *mut GDBusPropertyInfo) -> *mut GDBusPropertyInfo;
    fn g_dbus_method_info_unref(info: *mut GDBusMethodInfo);
    fn g_dbus_property_info_unref(info: *mut GDBusPropertyInfo);
    fn g_dbus_error_encode_gerror(error: *const GError) -> *mut gchar;
    fn _g_dbus_debug_return() -> gboolean;
    fn _g_dbus_debug_print_lock();
    fn _g_dbus_debug_print_unlock();
    fn _g_dbus_compute_complete_signature(args: *mut *mut GDBusArgInfo) -> *mut GVariantType;
    fn g_io_error_quark() -> GQuark;
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
pub type GDBusMessageFlags = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusMessageFlags = 4;
pub const G_DBUS_MESSAGE_FLAGS_NO_AUTO_START: GDBusMessageFlags = 2;
pub const G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED: GDBusMessageFlags = 1;
pub const G_DBUS_MESSAGE_FLAGS_NONE: GDBusMessageFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusSendMessageFlags = ::core::ffi::c_uint;
pub const G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL: GDBusSendMessageFlags = 1;
pub const G_DBUS_SEND_MESSAGE_FLAGS_NONE: GDBusSendMessageFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusMessage = _GDBusMessage;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInvocation {
    pub parent_instance: GObject,
    pub sender: *mut gchar,
    pub object_path: *mut gchar,
    pub interface_name: *mut gchar,
    pub method_name: *mut gchar,
    pub method_info: *mut GDBusMethodInfo,
    pub property_info: *mut GDBusPropertyInfo,
    pub connection: *mut GDBusConnection,
    pub message: *mut GDBusMessage,
    pub parameters: *mut GVariant,
    pub user_data: gpointer,
}
pub type GDBusPropertyInfo = _GDBusPropertyInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusPropertyInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub flags: GDBusPropertyInfoFlags,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInfo = _GDBusMethodInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusArgInfo = _GDBusArgInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusArgInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInvocation = _GDBusMethodInvocation;
pub type GDBusMethodInvocationClass = _GDBusMethodInvocationClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInvocationClass {
    pub parent_class: GObjectClass,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
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
pub const G_VARIANT_TYPE_UNIT: *const GVariantType =
    b"()\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
static mut safe_c2rust_g_dbus_method_invocation_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GDBusMethodInvocation_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_method_invocation_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_method_invocation_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusMethodInvocation_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusMethodInvocation_private_offset,
        );
    }
    safe_c2rust_g_dbus_method_invocation_class_init(klass as *mut GDBusMethodInvocationClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusMethodInvocation\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusMethodInvocationClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_method_invocation_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusMethodInvocation>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusMethodInvocation) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_method_invocation_init
                    as unsafe extern "C" fn(*mut GDBusMethodInvocation) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_finalize(mut object: *mut GObject) {
    let mut invocation: *mut GDBusMethodInvocation =
        object as *mut ::core::ffi::c_void as *mut GDBusMethodInvocation;
    g_free((*invocation).sender as gpointer);
    g_free((*invocation).object_path as gpointer);
    g_free((*invocation).interface_name as gpointer);
    g_free((*invocation).method_name as gpointer);
    if !(*invocation).method_info.is_null() {
        g_dbus_method_info_unref((*invocation).method_info);
    }
    if !(*invocation).property_info.is_null() {
        g_dbus_property_info_unref((*invocation).property_info);
    }
    g_object_unref((*invocation).connection as gpointer);
    g_object_unref((*invocation).message as gpointer);
    g_variant_unref((*invocation).parameters);
    (*(safe_c2rust_g_dbus_method_invocation_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_class_init(
    mut klass: *mut GDBusMethodInvocationClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_dbus_method_invocation_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_init(
    mut invocation: *mut GDBusMethodInvocation,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_sender(
    mut invocation: *mut GDBusMethodInvocation,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*invocation).sender;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_object_path(
    mut invocation: *mut GDBusMethodInvocation,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*invocation).object_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_interface_name(
    mut invocation: *mut GDBusMethodInvocation,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*invocation).interface_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_method_info(
    mut invocation: *mut GDBusMethodInvocation,
) -> *const GDBusMethodInfo {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GDBusMethodInfo>();
    }
    return (*invocation).method_info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_property_info(
    mut invocation: *mut GDBusMethodInvocation,
) -> *const GDBusPropertyInfo {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GDBusPropertyInfo>();
    }
    return (*invocation).property_info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_method_name(
    mut invocation: *mut GDBusMethodInvocation,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*invocation).method_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_connection(
    mut invocation: *mut GDBusMethodInvocation,
) -> *mut GDBusConnection {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    return (*invocation).connection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_message(
    mut invocation: *mut GDBusMethodInvocation,
) -> *mut GDBusMessage {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    return (*invocation).message;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_parameters(
    mut invocation: *mut GDBusMethodInvocation,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return (*invocation).parameters;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_get_user_data(
    mut invocation: *mut GDBusMethodInvocation,
) -> gpointer {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*invocation).user_data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_method_invocation_new(
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut method_info: *const GDBusMethodInfo,
    mut property_info: *const GDBusPropertyInfo,
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) -> *mut GDBusMethodInvocation {
    let mut invocation: *mut GDBusMethodInvocation =
        ::core::ptr::null_mut::<GDBusMethodInvocation>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if sender.is_null() || g_dbus_is_name(sender) != 0 {
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
            b"sender == NULL || g_dbus_is_name (sender)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMethodInvocation>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMethodInvocation>();
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if interface_name.is_null() || g_dbus_is_interface_name(interface_name) != 0 {
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
            b"interface_name == NULL || g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMethodInvocation>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if g_dbus_is_member_name(method_name) != 0 {
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
            b"g_dbus_is_member_name (method_name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMethodInvocation>();
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMethodInvocation>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMethodInvocation>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if g_variant_is_of_type(
            parameters,
            b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) != 0
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
            b"g_variant_is_of_type (parameters, G_VARIANT_TYPE_TUPLE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMethodInvocation>();
    }
    invocation = g_object_new(
        safe_c2rust_g_dbus_method_invocation_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDBusMethodInvocation;
    (*invocation).sender =
        safe_c2rust_g_strdup_inline(sender as *const ::core::ffi::c_char) as *mut gchar;
    (*invocation).object_path =
        safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
    (*invocation).interface_name =
        safe_c2rust_g_strdup_inline(interface_name as *const ::core::ffi::c_char) as *mut gchar;
    (*invocation).method_name =
        safe_c2rust_g_strdup_inline(method_name as *const ::core::ffi::c_char) as *mut gchar;
    if !method_info.is_null() {
        (*invocation).method_info = g_dbus_method_info_ref(method_info as *mut GDBusMethodInfo);
    }
    if !property_info.is_null() {
        (*invocation).property_info =
            g_dbus_property_info_ref(property_info as *mut GDBusPropertyInfo);
    }
    (*invocation).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    (*invocation).message =
        g_object_ref(message as gpointer) as *mut GDBusMessage as *mut GDBusMessage;
    (*invocation).parameters = g_variant_ref(parameters);
    (*invocation).user_data = user_data;
    return invocation;
}
unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_value_internal(
    mut invocation: *mut GDBusMethodInvocation,
    mut parameters: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
) {
    let mut current_block: u64;
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if parameters.is_null()
            || g_variant_is_of_type(
                parameters,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
        {
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
            b"(parameters == NULL) || g_variant_is_of_type (parameters, G_VARIANT_TYPE_TUPLE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(g_dbus_message_get_flags((*invocation).message) as ::core::ffi::c_uint
        & G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0)
    {
        if parameters.is_null() {
            parameters = g_variant_new_tuple(::core::ptr::null::<*mut GVariant>(), 0 as gsize);
        }
        if !(*invocation).method_info.is_null() {
            let mut type_0: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
            type_0 = _g_dbus_compute_complete_signature((*(*invocation).method_info).out_args);
            if g_variant_is_of_type(parameters, type_0) == 0 {
                let mut type_string: *mut gchar = g_variant_type_dup_string(type_0);
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Type of return value is incorrect: expected '%s', got '%s'\0" as *const u8
                        as *const gchar,
                    type_string,
                    g_variant_get_type_string(parameters),
                );
                g_variant_type_free(type_0);
                g_free(type_string as gpointer);
                current_block = 743651752669054841;
            } else {
                g_variant_type_free(type_0);
                current_block = 224731115979188411;
            }
        } else {
            current_block = 224731115979188411;
        }
        match current_block {
            743651752669054841 => {}
            _ => {
                if !(*invocation).property_info.is_null() {
                    if strcmp(
                        (*invocation).method_name as *const ::core::ffi::c_char,
                        b"Get\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut nested: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                        if g_variant_is_of_type(
                            parameters,
                            g_variant_type_checked_(b"(v)\0" as *const u8 as *const gchar),
                        ) == 0
                        {
                            g_log(
                                G_LOG_DOMAIN.as_ptr() as *const gchar,
                                G_LOG_LEVEL_WARNING,
                                b"Type of return value for property 'Get' call should be '(v)' but got '%s'\0"
                                    as *const u8 as *const gchar,
                                g_variant_get_type_string(parameters),
                            );
                            current_block = 743651752669054841;
                        } else {
                            g_variant_get(
                                parameters,
                                b"(v)\0" as *const u8 as *const gchar,
                                &raw mut nested,
                            );
                            if !(strcmp(
                                g_variant_get_type_string(nested) as *const ::core::ffi::c_char,
                                (*(*invocation).property_info).signature
                                    as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int)
                            {
                                g_log(
                                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                                    G_LOG_LEVEL_WARNING,
                                    b"Value returned from property 'Get' call for '%s' should be '%s' but is '%s'\0"
                                        as *const u8 as *const gchar,
                                    (*(*invocation).property_info).name,
                                    (*(*invocation).property_info).signature,
                                    g_variant_get_type_string(nested),
                                );
                                g_variant_unref(nested);
                                current_block = 743651752669054841;
                            } else {
                                g_variant_unref(nested);
                                current_block = 980989089337379490;
                            }
                        }
                    } else if strcmp(
                        (*invocation).method_name as *const ::core::ffi::c_char,
                        b"Set\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if g_variant_is_of_type(parameters, G_VARIANT_TYPE_UNIT) == 0 {
                            g_log(
                                G_LOG_DOMAIN.as_ptr() as *const gchar,
                                G_LOG_LEVEL_WARNING,
                                b"Type of return value for property 'Set' call should be '()' but got '%s'\0"
                                    as *const u8 as *const gchar,
                                g_variant_get_type_string(parameters),
                            );
                            current_block = 743651752669054841;
                        } else {
                            current_block = 980989089337379490;
                        }
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmethodinvocation.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            465 as ::core::ffi::c_int,
                            G_STRFUNC,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                } else if strcmp(
                    (*invocation).interface_name as *const ::core::ffi::c_char,
                    b"org.freedesktop.DBus.Properties\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    && strcmp(
                        (*invocation).method_name as *const ::core::ffi::c_char,
                        b"GetAll\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    if g_variant_is_of_type(
                        parameters,
                        g_variant_type_checked_(b"(a{sv})\0" as *const u8 as *const gchar),
                    ) == 0
                    {
                        g_log(
                            G_LOG_DOMAIN.as_ptr() as *const gchar,
                            G_LOG_LEVEL_WARNING,
                            b"Type of return value for property 'GetAll' call should be '(a{sv})' but got '%s'\0"
                                as *const u8 as *const gchar,
                            g_variant_get_type_string(parameters),
                        );
                        current_block = 743651752669054841;
                    } else {
                        current_block = 980989089337379490;
                    }
                } else {
                    current_block = 980989089337379490;
                }
                match current_block {
                    743651752669054841 => {}
                    _ => {
                        if ({
                            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
                            if _g_dbus_debug_return() != 0 {
                                _g_boolean_var_29 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_29 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_29
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            _g_dbus_debug_print_lock();
                            g_print(
                                b"========================================================================\nGDBus-debug:Return:\n >>>> METHOD RETURN\n      in response to %s.%s()\n      on object %s\n      to name %s\n      reply-serial %d\n\0"
                                    as *const u8 as *const gchar,
                                (*invocation).interface_name,
                                (*invocation).method_name,
                                (*invocation).object_path,
                                (*invocation).sender,
                                g_dbus_message_get_serial((*invocation).message),
                            );
                            _g_dbus_debug_print_unlock();
                        }
                        reply = g_dbus_message_new_method_reply((*invocation).message);
                        g_dbus_message_set_body(
                            reply,
                            safe_c2rust_g_steal_pointer(&raw mut parameters as gpointer)
                                as *mut GVariant,
                        );
                        if !fd_list.is_null() {
                            g_dbus_message_set_unix_fd_list(reply, fd_list);
                        }
                        error = ::core::ptr::null_mut::<GError>();
                        if g_dbus_connection_send_message(
                            safe_c2rust_g_dbus_method_invocation_get_connection(invocation),
                            reply,
                            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                            ::core::ptr::null_mut::<guint32>(),
                            &raw mut error,
                        ) == 0
                        {
                            if g_error_matches(
                                error,
                                g_io_error_quark(),
                                G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
                            ) == 0
                            {
                                g_log(
                                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                                    G_LOG_LEVEL_WARNING,
                                    b"Error sending message: %s\0" as *const u8 as *const gchar,
                                    (*error).message,
                                );
                            }
                            g_error_free(error);
                        }
                        g_object_unref(reply as gpointer);
                    }
                }
            }
        }
    }
    if !parameters.is_null() {
        g_variant_ref_sink(parameters);
        g_variant_unref(parameters);
    }
    g_object_unref(invocation as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_value(
    mut invocation: *mut GDBusMethodInvocation,
    mut parameters: *mut GVariant,
) {
    safe_c2rust_g_dbus_method_invocation_return_value_internal(
        invocation,
        parameters,
        ::core::ptr::null_mut::<GUnixFDList>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_value_with_unix_fd_list(
    mut invocation: *mut GDBusMethodInvocation,
    mut parameters: *mut GVariant,
    mut fd_list: *mut GUnixFDList,
) {
    safe_c2rust_g_dbus_method_invocation_return_value_internal(invocation, parameters, fd_list);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_error(
    mut invocation: *mut GDBusMethodInvocation,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const gchar,
    mut args: ...
) {
    let mut var_args: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    var_args = args.clone();
    safe_c2rust_g_dbus_method_invocation_return_error_valist(
        invocation,
        domain,
        code,
        format,
        var_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_error_valist(
    mut invocation: *mut GDBusMethodInvocation,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const gchar,
    mut var_args: ::core::ffi::VaList,
) {
    let mut literal_message: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    literal_message = g_strdup_vprintf(format, var_args);
    safe_c2rust_g_dbus_method_invocation_return_error_literal(
        invocation,
        domain,
        code,
        literal_message,
    );
    g_free(literal_message as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_error_literal(
    mut invocation: *mut GDBusMethodInvocation,
    mut domain: GQuark,
    mut code: gint,
    mut message: *const gchar,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !message.is_null() {
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
            b"message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    error = g_error_new_literal(domain, code, message);
    safe_c2rust_g_dbus_method_invocation_return_gerror(invocation, error);
    g_error_free(error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_gerror(
    mut invocation: *mut GDBusMethodInvocation,
    mut error: *const GError,
) {
    let mut dbus_error_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    dbus_error_name = g_dbus_error_encode_gerror(error);
    safe_c2rust_g_dbus_method_invocation_return_dbus_error(
        invocation,
        dbus_error_name,
        (*error).message,
    );
    g_free(dbus_error_name as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_take_error(
    mut invocation: *mut GDBusMethodInvocation,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_dbus_method_invocation_return_gerror(invocation, error);
    g_error_free(error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_invocation_return_dbus_error(
    mut invocation: *mut GDBusMethodInvocation,
    mut error_name: *const gchar,
    mut error_message: *const gchar,
) {
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !error_name.is_null() && g_dbus_is_name(error_name) != 0 {
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
            b"error_name != NULL && g_dbus_is_name (error_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !error_message.is_null() {
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
            b"error_message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(g_dbus_message_get_flags((*invocation).message) as ::core::ffi::c_uint
        & G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0)
    {
        if ({
            let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
            if _g_dbus_debug_return() != 0 {
                _g_boolean_var_43 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_43 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_43
        }) as ::core::ffi::c_long
            != 0
        {
            _g_dbus_debug_print_lock();
            g_print(
                b"========================================================================\nGDBus-debug:Return:\n >>>> METHOD ERROR %s\n      message '%s'\n      in response to %s.%s()\n      on object %s\n      to name %s\n      reply-serial %d\n\0"
                    as *const u8 as *const gchar,
                error_name,
                error_message,
                (*invocation).interface_name,
                (*invocation).method_name,
                (*invocation).object_path,
                (*invocation).sender,
                g_dbus_message_get_serial((*invocation).message),
            );
            _g_dbus_debug_print_unlock();
        }
        reply = g_dbus_message_new_method_error_literal(
            (*invocation).message,
            error_name,
            error_message,
        );
        g_dbus_connection_send_message(
            safe_c2rust_g_dbus_method_invocation_get_connection(invocation),
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
    }
    g_object_unref(invocation as gpointer);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
