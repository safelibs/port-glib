extern "C" {
    pub type _GDateTime;
    pub type _GData;
    pub type _GCancellablePrivate;
    pub type _GSocketConnectable;
    pub type _GTlsBackend;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_byte_array_new() -> *mut GByteArray;
    fn g_byte_array_unref(array: *mut GByteArray);
    fn g_byte_array_append(
        array: *mut GByteArray,
        data: *const guint8,
        len: guint,
    ) -> *mut GByteArray;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_concat(list1: *mut GSList, list2: *mut GSList) -> *mut GSList;
    fn g_slist_last(list: *mut GSList) -> *mut GSList;
    fn g_strstr_len(
        haystack: *const gchar,
        haystack_len: gssize,
        needle: *const gchar,
    ) -> *mut gchar;
    fn g_strrstr_len(
        haystack: *const gchar,
        haystack_len: gssize,
        needle: *const gchar,
    ) -> *mut gchar;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_queue_push_tail(queue: *mut GQueue, data: gpointer);
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_byte_array_get_type() -> GType;
    fn g_ptr_array_get_type() -> GType;
    fn g_date_time_get_type() -> GType;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
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
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_static_string(value: *mut GValue, v_string: *const gchar);
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_tls_backend_get_default() -> *mut GTlsBackend;
    fn g_tls_backend_get_certificate_type(backend: *mut GTlsBackend) -> GType;
    fn g_io_error_quark() -> GQuark;
    fn g_tls_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
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
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
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
pub type GDateTime = _GDateTime;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GQueue {
    pub head: *mut GList,
    pub tail: *mut GList,
    pub length: guint,
}
pub type GQueue = _GQueue;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_TLS_ERROR_BAD_CERTIFICATE_PASSWORD: C2RustUnnamed_1 = 8;
pub const G_TLS_ERROR_INAPPROPRIATE_FALLBACK: C2RustUnnamed_1 = 7;
pub const G_TLS_ERROR_EOF: C2RustUnnamed_1 = 6;
pub const G_TLS_ERROR_CERTIFICATE_REQUIRED: C2RustUnnamed_1 = 5;
pub const G_TLS_ERROR_HANDSHAKE: C2RustUnnamed_1 = 4;
pub const G_TLS_ERROR_NOT_TLS: C2RustUnnamed_1 = 3;
pub const G_TLS_ERROR_BAD_CERTIFICATE: C2RustUnnamed_1 = 2;
pub const G_TLS_ERROR_MISC: C2RustUnnamed_1 = 1;
pub const G_TLS_ERROR_UNAVAILABLE: C2RustUnnamed_1 = 0;
pub type GTlsCertificateFlags = ::core::ffi::c_uint;
pub const G_TLS_CERTIFICATE_VALIDATE_ALL: GTlsCertificateFlags = 127;
pub const G_TLS_CERTIFICATE_GENERIC_ERROR: GTlsCertificateFlags = 64;
pub const G_TLS_CERTIFICATE_INSECURE: GTlsCertificateFlags = 32;
pub const G_TLS_CERTIFICATE_REVOKED: GTlsCertificateFlags = 16;
pub const G_TLS_CERTIFICATE_EXPIRED: GTlsCertificateFlags = 8;
pub const G_TLS_CERTIFICATE_NOT_ACTIVATED: GTlsCertificateFlags = 4;
pub const G_TLS_CERTIFICATE_BAD_IDENTITY: GTlsCertificateFlags = 2;
pub const G_TLS_CERTIFICATE_UNKNOWN_CA: GTlsCertificateFlags = 1;
pub const G_TLS_CERTIFICATE_NO_FLAGS: GTlsCertificateFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GSocketConnectable = _GSocketConnectable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsCertificate {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsCertificatePrivate,
}
pub type GTlsCertificatePrivate = _GTlsCertificatePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsCertificatePrivate {
    pub pkcs12_properties_not_overridden: gboolean,
}
pub type GTlsCertificate = _GTlsCertificate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsCertificateClass {
    pub parent_class: GObjectClass,
    pub verify: Option<
        unsafe extern "C" fn(
            *mut GTlsCertificate,
            *mut GSocketConnectable,
            *mut GTlsCertificate,
        ) -> GTlsCertificateFlags,
    >,
    pub padding: [gpointer; 8],
}
pub type GTlsCertificateClass = _GTlsCertificateClass;
pub const PROP_IP_ADDRESSES: C2RustUnnamed_2 = 13;
pub const PROP_DNS_NAMES: C2RustUnnamed_2 = 12;
pub const PROP_ISSUER_NAME: C2RustUnnamed_2 = 11;
pub const PROP_SUBJECT_NAME: C2RustUnnamed_2 = 10;
pub const PROP_NOT_VALID_AFTER: C2RustUnnamed_2 = 9;
pub const PROP_NOT_VALID_BEFORE: C2RustUnnamed_2 = 8;
pub const PROP_PRIVATE_KEY_PKCS11_URI: C2RustUnnamed_2 = 7;
pub const PROP_PKCS11_URI: C2RustUnnamed_2 = 6;
pub const PROP_ISSUER: C2RustUnnamed_2 = 5;
pub const PROP_PRIVATE_KEY_PEM: C2RustUnnamed_2 = 4;
pub const PROP_PRIVATE_KEY: C2RustUnnamed_2 = 3;
pub const PROP_CERTIFICATE_PEM: C2RustUnnamed_2 = 2;
pub const PROP_CERTIFICATE: C2RustUnnamed_2 = 1;
pub const PROP_PASSWORD: C2RustUnnamed_2 = 15;
pub const PROP_PKCS12_DATA: C2RustUnnamed_2 = 14;
pub type GTlsBackend = _GTlsBackend;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_instance_private(
    mut self_0: *mut GTlsCertificate,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GTlsCertificate_private_offset as glong as isize)
        as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_tls_certificate_get_type_once();
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
static mut safe_c2rust_g_tls_certificate_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_tls_certificate_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_tls_certificate_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GTlsCertificate_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GTlsCertificate_private_offset,
        );
    }
    safe_c2rust_g_tls_certificate_class_init(klass as *mut GTlsCertificateClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GTlsCertificate\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GTlsCertificateClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_certificate_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GTlsCertificate>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTlsCertificate) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_certificate_init
                    as unsafe extern "C" fn(*mut GTlsCertificate) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GTlsCertificate_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GTlsCertificatePrivate>() as gsize,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GTlsCertificate_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_tls_certificate_init(mut cert: *mut GTlsCertificate) {}
unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    match prop_id {
        3 | 4 | 6 | 7 => {
            g_value_set_static_string(value, ::core::ptr::null::<gchar>());
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlscertificate.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                91 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_certificate_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut cert: *mut GTlsCertificate = object as *mut GTlsCertificate;
    let mut priv_0: *mut GTlsCertificatePrivate =
        safe_c2rust_g_tls_certificate_get_instance_private(cert) as *mut GTlsCertificatePrivate;
    match prop_id {
        6 | 7 => {}
        14 | 15 => {
            (*priv_0).pkcs12_properties_not_overridden = TRUE as gboolean;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlscertificate.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_certificate_class_init(
    mut class: *mut GTlsCertificateClass,
) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_tls_certificate_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_tls_certificate_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_PKCS12_DATA as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"pkcs12-data\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_byte_array_get_type(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PASSWORD as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"password\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_CERTIFICATE as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"certificate\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_byte_array_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_CERTIFICATE_PEM as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"certificate-pem\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PRIVATE_KEY as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"private-key\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_byte_array_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PRIVATE_KEY_PEM as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"private-key-pem\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ISSUER as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"issuer\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            safe_c2rust_g_tls_certificate_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PKCS11_URI as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"pkcs11-uri\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_PRIVATE_KEY_PKCS11_URI as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"private-key-pkcs11-uri\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_NOT_VALID_BEFORE as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"not-valid-before\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_date_time_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_NOT_VALID_AFTER as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"not-valid-after\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_date_time_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_SUBJECT_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"subject-name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ISSUER_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"issuer-name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_DNS_NAMES as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"dns-names\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_ptr_array_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_IP_ADDRESSES as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"ip-addresses\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_ptr_array_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_tls_certificate_new_internal(
    mut certificate_pem: *const gchar,
    mut private_key_pem: *const gchar,
    mut issuer: *mut GTlsCertificate,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut cert: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut backend: *mut GTlsBackend = ::core::ptr::null_mut::<GTlsBackend>();
    backend = g_tls_backend_get_default();
    cert = g_initable_new(
        g_tls_backend_get_certificate_type(backend),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"certificate-pem\0" as *const u8 as *const gchar,
        certificate_pem,
        b"private-key-pem\0" as *const u8 as *const ::core::ffi::c_char,
        private_key_pem,
        b"issuer\0" as *const u8 as *const ::core::ffi::c_char,
        issuer,
        NULL_0,
    ) as *mut GObject;
    return cert as *mut ::core::ffi::c_void as *mut GTlsCertificate;
}
pub const PEM_CERTIFICATE_HEADER: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"-----BEGIN CERTIFICATE-----\0")
};
pub const PEM_CERTIFICATE_FOOTER: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"-----END CERTIFICATE-----\0")
};
pub const PEM_PRIVKEY_HEADER_BEGIN: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"-----BEGIN \0") };
pub const PEM_PRIVKEY_HEADER_END: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"PRIVATE KEY-----\0")
};
pub const PEM_PRIVKEY_FOOTER_BEGIN: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"-----END \0") };
pub const PEM_PRIVKEY_FOOTER_END: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"PRIVATE KEY-----\0")
};
pub const PEM_PKCS8_ENCRYPTED_HEADER: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<[u8; 38], [::core::ffi::c_char; 38]>(
        *b"-----BEGIN ENCRYPTED PRIVATE KEY-----\0",
    )
};
unsafe extern "C" fn safe_c2rust_parse_private_key(
    mut data: *const gchar,
    mut data_len: gsize,
    mut required: gboolean,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut header_start: *const gchar = ::core::ptr::null::<gchar>();
    let mut header_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut footer_start: *const gchar = ::core::ptr::null::<gchar>();
    let mut footer_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut data_end: *const gchar = data.offset(data_len as isize);
    header_end = g_strstr_len(
        data,
        data_len as gssize,
        PEM_PRIVKEY_HEADER_END.as_ptr() as *const gchar,
    );
    if !header_end.is_null() {
        header_start = g_strrstr_len(
            data,
            header_end.offset_from(data) as gssize,
            PEM_PRIVKEY_HEADER_BEGIN.as_ptr() as *const gchar,
        );
    }
    if header_start.is_null() {
        if required != 0 {
            g_set_error_literal(
                error,
                g_tls_error_quark(),
                G_TLS_ERROR_BAD_CERTIFICATE as ::core::ffi::c_int as gint,
                glib_gettext(b"No PEM-encoded private key found\0" as *const u8 as *const gchar),
            );
        }
        return ::core::ptr::null_mut::<gchar>();
    }
    header_end = header_end.offset(strlen(PEM_PRIVKEY_HEADER_END.as_ptr()) as isize);
    if strncmp(
        header_start as *const ::core::ffi::c_char,
        PEM_PKCS8_ENCRYPTED_HEADER.as_ptr(),
        header_end.offset_from(header_start) as ::core::ffi::c_long as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        g_set_error_literal(
            error,
            g_tls_error_quark(),
            G_TLS_ERROR_BAD_CERTIFICATE as ::core::ffi::c_int as gint,
            glib_gettext(b"Cannot decrypt PEM-encoded private key\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    footer_end = g_strstr_len(
        header_end,
        data_len.wrapping_sub(header_end.offset_from(data) as ::core::ffi::c_long as gsize)
            as gssize,
        PEM_PRIVKEY_FOOTER_END.as_ptr() as *const gchar,
    );
    if !footer_end.is_null() {
        footer_start = g_strrstr_len(
            header_end,
            footer_end.offset_from(header_end) as gssize,
            PEM_PRIVKEY_FOOTER_BEGIN.as_ptr() as *const gchar,
        );
    }
    if footer_start.is_null() {
        g_set_error_literal(
            error,
            g_tls_error_quark(),
            G_TLS_ERROR_BAD_CERTIFICATE as ::core::ffi::c_int as gint,
            glib_gettext(b"Could not parse PEM-encoded private key\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    footer_end = footer_end.offset(strlen(PEM_PRIVKEY_FOOTER_END.as_ptr()) as isize);
    while footer_end < data_end
        && (*footer_end as ::core::ffi::c_int == '\r' as i32
            || *footer_end as ::core::ffi::c_int == '\n' as i32)
    {
        footer_end = footer_end.offset(1);
    }
    return g_strndup(
        header_start,
        footer_end.offset_from(header_start) as ::core::ffi::c_long as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_parse_next_pem_certificate(
    mut data: *mut *const gchar,
    mut data_end: *const gchar,
    mut required: gboolean,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut start: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    start = g_strstr_len(
        *data,
        data_end.offset_from(*data) as gssize,
        PEM_CERTIFICATE_HEADER.as_ptr() as *const gchar,
    );
    if start.is_null() {
        if required != 0 {
            g_set_error_literal(
                error,
                g_tls_error_quark(),
                G_TLS_ERROR_BAD_CERTIFICATE as ::core::ffi::c_int as gint,
                glib_gettext(b"No PEM-encoded certificate found\0" as *const u8 as *const gchar),
            );
        }
        return ::core::ptr::null_mut::<gchar>();
    }
    end = g_strstr_len(
        start,
        data_end.offset_from(start) as gssize,
        PEM_CERTIFICATE_FOOTER.as_ptr() as *const gchar,
    );
    if end.is_null() {
        g_set_error_literal(
            error,
            g_tls_error_quark(),
            G_TLS_ERROR_BAD_CERTIFICATE as ::core::ffi::c_int as gint,
            glib_gettext(b"Could not parse PEM-encoded certificate\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    end = end.offset(strlen(PEM_CERTIFICATE_FOOTER.as_ptr()) as isize);
    while end < data_end
        && (*end as ::core::ffi::c_int == '\r' as i32 || *end as ::core::ffi::c_int == '\n' as i32)
    {
        end = end.offset(1);
    }
    *data = end;
    return g_strndup(
        start,
        end.offset_from(start) as ::core::ffi::c_long as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_parse_and_create_certificate_list(
    mut data: *const gchar,
    mut data_len: gsize,
    mut error: *mut *mut GError,
) -> *mut GSList {
    let mut first_pem_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut pem_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut first_pem: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    p = data;
    end = p.offset(data_len as isize);
    first_pem = safe_c2rust_parse_next_pem_certificate(&raw mut p, end, TRUE, error);
    if first_pem.is_null() {
        return ::core::ptr::null_mut::<GSList>();
    }
    first_pem_list = g_slist_prepend(first_pem_list, first_pem as gpointer);
    while p < end && !p.is_null() && *p as ::core::ffi::c_int != 0 {
        let mut cert_pem: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
        cert_pem =
            safe_c2rust_parse_next_pem_certificate(&raw mut p, end, FALSE, &raw mut my_error);
        if !my_error.is_null() {
            g_slist_free_full(
                pem_list,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
            g_error_free(my_error);
            return first_pem_list;
        } else {
            if cert_pem.is_null() {
                break;
            }
            pem_list = g_slist_prepend(pem_list, cert_pem as gpointer);
        }
    }
    pem_list = g_slist_concat(pem_list, first_pem_list);
    return pem_list;
}
unsafe extern "C" fn safe_c2rust_create_certificate_chain_from_list(
    mut pem_list: *mut GSList,
    mut key_pem: *const gchar,
) -> *mut GTlsCertificate {
    let mut cert: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut issuer: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut root: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut flags: GTlsCertificateFlags = G_TLS_CERTIFICATE_NO_FLAGS;
    let mut pem: *mut GSList = ::core::ptr::null_mut::<GSList>();
    pem = pem_list;
    while !pem.is_null() {
        let mut key: *const gchar = ::core::ptr::null::<gchar>();
        if (*pem).next.is_null() {
            key = key_pem;
        }
        issuer = cert;
        cert = safe_c2rust_g_tls_certificate_new_internal(
            (*pem).data as *const gchar,
            key,
            issuer,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !issuer.is_null() {
            g_object_unref(issuer as gpointer);
        }
        if cert.is_null() {
            return ::core::ptr::null_mut::<GTlsCertificate>();
        }
        if root.is_null() {
            root = g_object_ref(cert as gpointer) as *mut GTlsCertificate as *mut GTlsCertificate;
        }
        pem = if !pem.is_null() {
            (*pem).next
        } else {
            ::core::ptr::null_mut::<GSList>()
        };
    }
    flags = safe_c2rust_g_tls_certificate_verify(
        cert,
        ::core::ptr::null_mut::<GSocketConnectable>(),
        root,
    );
    if flags as ::core::ffi::c_uint
        & G_TLS_CERTIFICATE_UNKNOWN_CA as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut _pp: *mut *mut GTlsCertificate = &raw mut cert;
        let mut _ptr: *mut GTlsCertificate = *_pp;
        *_pp = ::core::ptr::null_mut::<GTlsCertificate>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    }
    let mut _pp_0: *mut *mut GTlsCertificate = &raw mut root;
    let mut _ptr_0: *mut GTlsCertificate = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GTlsCertificate>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    return cert;
}
unsafe extern "C" fn safe_c2rust_parse_and_create_certificate(
    mut data: *const gchar,
    mut data_len: gsize,
    mut key_pem: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut pem_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut cert: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    pem_list = safe_c2rust_parse_and_create_certificate_list(data, data_len, error);
    if pem_list.is_null() {
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    cert = safe_c2rust_create_certificate_chain_from_list(pem_list, key_pem);
    if cert.is_null() {
        let mut last: *mut GSList = ::core::ptr::null_mut::<GSList>();
        last = g_slist_last(pem_list);
        cert = safe_c2rust_g_tls_certificate_new_internal(
            (*last).data as *const gchar,
            key_pem,
            ::core::ptr::null_mut::<GTlsCertificate>(),
            error,
        );
    }
    g_slist_free_full(
        pem_list,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    return cert;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_new_from_pem(
    mut data: *const gchar,
    mut length: gssize,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut child_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut key_pem: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut cert: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !data.is_null() {
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
            b"data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if length == -(1 as ::core::ffi::c_int) as gssize {
        length = strlen(data as *const ::core::ffi::c_char) as gssize;
    }
    key_pem = safe_c2rust_parse_private_key(data, length as gsize, FALSE, &raw mut child_error);
    if !child_error.is_null() {
        g_propagate_error(error, child_error);
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    cert = safe_c2rust_parse_and_create_certificate(data, length as gsize, key_pem, error);
    g_free(key_pem as gpointer);
    return cert;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_new_from_pkcs12(
    mut data: *const guint8,
    mut length: gsize,
    mut password: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut cert: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut backend: *mut GTlsBackend = ::core::ptr::null_mut::<GTlsBackend>();
    let mut bytes: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !data.is_null() || length == 0 as gsize {
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
            b"data != NULL || length == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    backend = g_tls_backend_get_default();
    bytes = g_byte_array_new();
    g_byte_array_append(bytes, data, length as guint);
    cert = g_initable_new(
        g_tls_backend_get_certificate_type(backend),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"pkcs12-data\0" as *const u8 as *const gchar,
        bytes,
        b"password\0" as *const u8 as *const ::core::ffi::c_char,
        password,
        NULL_0,
    ) as *mut GObject;
    g_byte_array_unref(bytes);
    if !cert.is_null() {
        let mut priv_0: *mut GTlsCertificatePrivate =
            safe_c2rust_g_tls_certificate_get_instance_private(
                cert as *mut ::core::ffi::c_void as *mut GTlsCertificate,
            ) as *mut GTlsCertificatePrivate;
        if (*priv_0).pkcs12_properties_not_overridden != 0 {
            let mut _pp: *mut *mut GObject = &raw mut cert;
            let mut _ptr: *mut GObject = *_pp;
            *_pp = ::core::ptr::null_mut::<GObject>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"The current TLS backend does not support PKCS #12\0" as *const u8
                        as *const gchar,
                ),
            );
            return ::core::ptr::null_mut::<GTlsCertificate>();
        }
    }
    return cert as *mut ::core::ffi::c_void as *mut GTlsCertificate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_new_from_file_with_password(
    mut file: *const gchar,
    mut password: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut cert: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !file.is_null() {
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
            b"file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !password.is_null() {
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
            b"password != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
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
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = file as *const ::core::ffi::c_char;
            let __suffix: *const ::core::ffi::c_char =
                b".p12\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                if __str.is_null() || __suffix.is_null() {
                    _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_17
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
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
        g_str_has_suffix(file, b".p12\0" as *const u8 as *const gchar)
    }) == 0
        && (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = file as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b".pfx\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_18
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
            g_str_has_suffix(file, b".pfx\0" as *const u8 as *const gchar)
        }) == 0
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            b"The file type of \"%s\" is unknown. Only .p12 and .pfx files are supported currently.\0"
                as *const u8 as *const gchar,
            file,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if g_file_get_contents(file, &raw mut contents, &raw mut length, error) == 0 {
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    cert = safe_c2rust_g_tls_certificate_new_from_pkcs12(
        contents as *mut guint8,
        length,
        password,
        error,
    );
    g_free(contents as gpointer);
    return cert;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_new_from_file(
    mut file: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut cert: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !file.is_null() {
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
            b"file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if g_file_get_contents(file, &raw mut contents, &raw mut length, error) == 0 {
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = file as *const ::core::ffi::c_char;
            let __suffix: *const ::core::ffi::c_char =
                b".p12\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                if __str.is_null() || __suffix.is_null() {
                    _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_21
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
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
        g_str_has_suffix(file, b".p12\0" as *const u8 as *const gchar)
    }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = file as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b".pfx\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_22
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
            g_str_has_suffix(file, b".pfx\0" as *const u8 as *const gchar)
        }) != 0
    {
        cert = safe_c2rust_g_tls_certificate_new_from_pkcs12(
            contents as *mut guint8,
            length,
            ::core::ptr::null::<gchar>(),
            error,
        );
    } else {
        cert = safe_c2rust_g_tls_certificate_new_from_pem(contents, length as gssize, error);
    }
    g_free(contents as gpointer);
    return cert;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_new_from_files(
    mut cert_file: *const gchar,
    mut key_file: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut cert: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut cert_data: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut key_data: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut cert_len: gsize = 0;
    let mut key_len: gsize = 0;
    let mut key_pem: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if g_file_get_contents(key_file, &raw mut key_data, &raw mut key_len, error) == 0 {
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    key_pem = safe_c2rust_parse_private_key(key_data, key_len, TRUE, error);
    g_free(key_data as gpointer);
    if key_pem.is_null() {
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if g_file_get_contents(cert_file, &raw mut cert_data, &raw mut cert_len, error) == 0 {
        g_free(key_pem as gpointer);
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    cert = safe_c2rust_parse_and_create_certificate(cert_data, cert_len, key_pem, error);
    g_free(cert_data as gpointer);
    g_free(key_pem as gpointer);
    return cert;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_new_from_pkcs11_uris(
    mut pkcs11_uri: *const gchar,
    mut private_key_pkcs11_uri: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    let mut cert: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut backend: *mut GTlsBackend = ::core::ptr::null_mut::<GTlsBackend>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !pkcs11_uri.is_null() {
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
            b"pkcs11_uri\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    backend = g_tls_backend_get_default();
    cert = g_initable_new(
        g_tls_backend_get_certificate_type(backend),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"pkcs11-uri\0" as *const u8 as *const gchar,
        pkcs11_uri,
        b"private-key-pkcs11-uri\0" as *const u8 as *const ::core::ffi::c_char,
        private_key_pkcs11_uri,
        NULL_0,
    ) as *mut GObject;
    if !cert.is_null() {
        let mut objects_uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
        g_object_get(
            cert as gpointer,
            b"pkcs11-uri\0" as *const u8 as *const gchar,
            &raw mut objects_uri,
            NULL_0,
        );
        if objects_uri.is_null() {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"This GTlsBackend does not support creating PKCS #11 certificates\0"
                        as *const u8 as *const gchar,
                ),
            );
            g_object_unref(cert as gpointer);
            return ::core::ptr::null_mut::<GTlsCertificate>();
        }
        g_free(objects_uri as gpointer);
    }
    return cert as *mut ::core::ffi::c_void as *mut GTlsCertificate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_list_new_from_file(
    mut file: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut queue: GQueue = _GQueue {
        head: ::core::ptr::null_mut::<GList>(),
        tail: ::core::ptr::null_mut::<GList>(),
        length: 0 as guint,
    };
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut length: gsize = 0;
    if g_file_get_contents(file, &raw mut contents, &raw mut length, error) == 0 {
        return ::core::ptr::null_mut::<GList>();
    }
    end = contents.offset(length as isize);
    p = contents;
    while !p.is_null() && *p as ::core::ffi::c_int != 0 {
        let mut cert_pem: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut cert: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
        let mut parse_error: *mut GError = ::core::ptr::null_mut::<GError>();
        cert_pem =
            safe_c2rust_parse_next_pem_certificate(&raw mut p, end, FALSE, &raw mut parse_error);
        if !cert_pem.is_null() {
            cert = safe_c2rust_g_tls_certificate_new_internal(
                cert_pem,
                ::core::ptr::null::<gchar>(),
                ::core::ptr::null_mut::<GTlsCertificate>(),
                &raw mut parse_error,
            );
            g_free(cert_pem as gpointer);
        }
        if cert.is_null() {
            if !parse_error.is_null() {
                g_propagate_error(error, parse_error);
                g_list_free_full(
                    queue.head,
                    Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
                );
                queue.head = ::core::ptr::null_mut::<GList>();
            }
            break;
        } else {
            g_queue_push_tail(&raw mut queue, cert as gpointer);
        }
    }
    g_free(contents as gpointer);
    return queue.head;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_issuer(
    mut cert: *mut GTlsCertificate,
) -> *mut GTlsCertificate {
    let mut issuer: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    g_object_get(
        cert as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"issuer\0" as *const u8 as *const gchar,
        &raw mut issuer,
        NULL_0,
    );
    if !issuer.is_null() {
        g_object_unref(issuer as gpointer);
    }
    return issuer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_verify(
    mut cert: *mut GTlsCertificate,
    mut identity: *mut GSocketConnectable,
    mut trusted_ca: *mut GTlsCertificate,
) -> GTlsCertificateFlags {
    return (*((*(cert as *mut GTypeInstance)).g_class as *mut GTlsCertificateClass))
        .verify
        .expect("non-null function pointer")(cert, identity, trusted_ca);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_is_same(
    mut cert_one: *mut GTlsCertificate,
    mut cert_two: *mut GTlsCertificate,
) -> gboolean {
    let mut b1: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    let mut b2: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    let mut equal: gboolean = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert_one as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert_one)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert_two as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert_two)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_object_get(
        cert_one as gpointer,
        b"certificate\0" as *const u8 as *const gchar,
        &raw mut b1,
        NULL_0,
    );
    g_object_get(
        cert_two as gpointer,
        b"certificate\0" as *const u8 as *const gchar,
        &raw mut b2,
        NULL_0,
    );
    equal = ((*b1).len == (*b2).len
        && memcmp(
            (*b1).data as *const ::core::ffi::c_void,
            (*b2).data as *const ::core::ffi::c_void,
            (*b1).len as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    g_byte_array_unref(b1);
    g_byte_array_unref(b2);
    return equal;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_not_valid_before(
    mut cert: *mut GTlsCertificate,
) -> *mut GDateTime {
    let mut not_valid_before: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    g_object_get(
        cert as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"not-valid-before\0" as *const u8 as *const gchar,
        &raw mut not_valid_before,
        NULL_0,
    );
    return safe_c2rust_g_steal_pointer(&raw mut not_valid_before as gpointer) as *mut GDateTime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_not_valid_after(
    mut cert: *mut GTlsCertificate,
) -> *mut GDateTime {
    let mut not_valid_after: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    g_object_get(
        cert as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"not-valid-after\0" as *const u8 as *const gchar,
        &raw mut not_valid_after,
        NULL_0,
    );
    return safe_c2rust_g_steal_pointer(&raw mut not_valid_after as gpointer) as *mut GDateTime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_subject_name(
    mut cert: *mut GTlsCertificate,
) -> *mut gchar {
    let mut subject_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_object_get(
        cert as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"subject-name\0" as *const u8 as *const gchar,
        &raw mut subject_name,
        NULL_0,
    );
    return safe_c2rust_g_steal_pointer(&raw mut subject_name as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_issuer_name(
    mut cert: *mut GTlsCertificate,
) -> *mut gchar {
    let mut issuer_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_object_get(
        cert as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"issuer-name\0" as *const u8 as *const gchar,
        &raw mut issuer_name,
        NULL_0,
    );
    return safe_c2rust_g_steal_pointer(&raw mut issuer_name as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_dns_names(
    mut cert: *mut GTlsCertificate,
) -> *mut GPtrArray {
    let mut dns_names: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    g_object_get(
        cert as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"dns-names\0" as *const u8 as *const gchar,
        &raw mut dns_names,
        NULL_0,
    );
    return safe_c2rust_g_steal_pointer(&raw mut dns_names as gpointer) as *mut GPtrArray;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_get_ip_addresses(
    mut cert: *mut GTlsCertificate,
) -> *mut GPtrArray {
    let mut ip_addresses: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = cert as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (cert)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPtrArray>();
    }
    g_object_get(
        cert as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"ip-addresses\0" as *const u8 as *const gchar,
        &raw mut ip_addresses,
        NULL_0,
    );
    return safe_c2rust_g_steal_pointer(&raw mut ip_addresses as gpointer) as *mut GPtrArray;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
