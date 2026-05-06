extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GInetAddressPrivate;
    pub type _GInetAddressMaskPrivate;
    pub type _GTask;
    pub type _GProxyResolver;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_hostname_is_non_ascii(hostname: *const gchar) -> gboolean;
    fn g_hostname_to_ascii(hostname: *const gchar) -> *mut gchar;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_ascii_strcasecmp(s1: *const gchar, s2: *const gchar) -> gint;
    fn g_ascii_strdown(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_uri_split_network(
        uri_string: *const gchar,
        flags: GUriFlags,
        scheme: *mut *mut gchar,
        host: *mut *mut gchar,
        port: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_uri_is_valid(
        uri_string: *const gchar,
        flags: GUriFlags,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_strv_get_type() -> GType;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
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
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_proxy_resolver_get_type() -> GType;
    fn g_inet_address_new_from_string(string: *const gchar) -> *mut GInetAddress;
    fn g_inet_address_mask_new_from_string(
        mask_string: *const gchar,
        error: *mut *mut GError,
    ) -> *mut GInetAddressMask;
    fn g_inet_address_mask_matches(
        mask: *mut GInetAddressMask,
        address: *mut GInetAddress,
    ) -> gboolean;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
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
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
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
pub type GHashTable = _GHashTable;
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
pub type GUriFlags = ::core::ffi::c_uint;
pub const G_URI_FLAGS_SCHEME_NORMALIZE: GUriFlags = 256;
pub const G_URI_FLAGS_ENCODED_FRAGMENT: GUriFlags = 128;
pub const G_URI_FLAGS_ENCODED_PATH: GUriFlags = 64;
pub const G_URI_FLAGS_ENCODED_QUERY: GUriFlags = 32;
pub const G_URI_FLAGS_NON_DNS: GUriFlags = 16;
pub const G_URI_FLAGS_ENCODED: GUriFlags = 8;
pub const G_URI_FLAGS_HAS_AUTH_PARAMS: GUriFlags = 4;
pub const G_URI_FLAGS_HAS_PASSWORD: GUriFlags = 2;
pub const G_URI_FLAGS_PARSE_RELAXED: GUriFlags = 1;
pub const G_URI_FLAGS_NONE: GUriFlags = 0;
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
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetAddress {
    pub parent_instance: GObject,
    pub priv_0: *mut GInetAddressPrivate,
}
pub type GInetAddressPrivate = _GInetAddressPrivate;
pub type GInetAddress = _GInetAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetAddressMask {
    pub parent_instance: GObject,
    pub priv_0: *mut GInetAddressMaskPrivate,
}
pub type GInetAddressMaskPrivate = _GInetAddressMaskPrivate;
pub type GInetAddressMask = _GInetAddressMask;
pub type GTask = _GTask;
pub type GProxyResolver = _GProxyResolver;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GProxyResolverInterface {
    pub g_iface: GTypeInterface,
    pub is_supported: Option<unsafe extern "C" fn(*mut GProxyResolver) -> gboolean>,
    pub lookup: Option<
        unsafe extern "C" fn(
            *mut GProxyResolver,
            *const gchar,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut *mut gchar,
    >,
    pub lookup_async: Option<
        unsafe extern "C" fn(
            *mut GProxyResolver,
            *const gchar,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_finish: Option<
        unsafe extern "C" fn(
            *mut GProxyResolver,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut *mut gchar,
    >,
}
pub type GProxyResolverInterface = _GProxyResolverInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleProxyResolver {
    pub parent_instance: GObject,
    pub priv_0: *mut GSimpleProxyResolverPrivate,
}
pub type GSimpleProxyResolverPrivate = _GSimpleProxyResolverPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleProxyResolverPrivate {
    pub default_proxy: *mut gchar,
    pub ignore_hosts: *mut *mut gchar,
    pub uri_proxies: *mut GHashTable,
    pub ignore_ips: *mut GPtrArray,
    pub ignore_domains: *mut GSimpleProxyResolverDomain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSimpleProxyResolverDomain {
    pub name: *mut gchar,
    pub length: gsize,
    pub port: gushort,
}
pub type GSimpleProxyResolver = _GSimpleProxyResolver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleProxyResolverClass {
    pub parent_class: GObjectClass,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSimpleProxyResolverClass = _GSimpleProxyResolverClass;
pub const PROP_IGNORE_HOSTS: C2RustUnnamed_0 = 2;
pub const PROP_DEFAULT_PROXY: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_simple_proxy_resolver_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSimpleProxyResolver_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSimpleProxyResolver_private_offset,
        );
    }
    safe_c2rust_g_simple_proxy_resolver_class_init(klass as *mut GSimpleProxyResolverClass);
}
static mut safe_c2rust_g_simple_proxy_resolver_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_simple_proxy_resolver_get_type_once();
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
static mut safe_c2rust_GSimpleProxyResolver_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSimpleProxyResolver\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSimpleProxyResolverClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_proxy_resolver_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSimpleProxyResolver>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSimpleProxyResolver) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_proxy_resolver_init
                    as unsafe extern "C" fn(*mut GSimpleProxyResolver) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GSimpleProxyResolver_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSimpleProxyResolverPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GProxyResolverInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_simple_proxy_resolver_iface_init
                as unsafe extern "C" fn(*mut GProxyResolverInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_proxy_resolver_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_get_instance_private(
    mut self_0: *mut GSimpleProxyResolver,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GSimpleProxyResolver_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_finalize(mut object: *mut GObject) {
    let mut resolver: *mut GSimpleProxyResolver =
        object as *mut ::core::ffi::c_void as *mut GSimpleProxyResolver;
    let mut priv_0: *mut GSimpleProxyResolverPrivate = (*resolver).priv_0;
    g_free((*priv_0).default_proxy as gpointer);
    g_hash_table_destroy((*priv_0).uri_proxies);
    let mut _pp: *mut *mut *mut gchar = &raw mut (*priv_0).ignore_hosts;
    let mut _ptr: *mut *mut gchar = *_pp;
    *_pp = ::core::ptr::null_mut::<*mut gchar>();
    if !_ptr.is_null() {
        g_strfreev(_ptr as *mut *mut gchar);
    }
    safe_c2rust_reparse_ignore_hosts(resolver);
    (*(safe_c2rust_g_simple_proxy_resolver_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_init(
    mut resolver: *mut GSimpleProxyResolver,
) {
    (*resolver).priv_0 = safe_c2rust_g_simple_proxy_resolver_get_instance_private(resolver)
        as *mut GSimpleProxyResolverPrivate;
    (*(*resolver).priv_0).uri_proxies = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut resolver: *mut GSimpleProxyResolver =
        object as *mut ::core::ffi::c_void as *mut GSimpleProxyResolver;
    match prop_id {
        1 => {
            safe_c2rust_g_simple_proxy_resolver_set_default_proxy(
                resolver,
                g_value_get_string(value),
            );
        }
        2 => {
            safe_c2rust_g_simple_proxy_resolver_set_ignore_hosts(
                resolver,
                g_value_get_boxed(value) as *mut *mut gchar,
            );
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsimpleproxyresolver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut resolver: *mut GSimpleProxyResolver =
        object as *mut ::core::ffi::c_void as *mut GSimpleProxyResolver;
    match prop_id {
        1 => {
            g_value_set_string(value, (*(*resolver).priv_0).default_proxy);
        }
        2 => {
            g_value_set_boxed(value, (*(*resolver).priv_0).ignore_hosts as gconstpointer);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsimpleproxyresolver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                146 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_reparse_ignore_hosts(mut resolver: *mut GSimpleProxyResolver) {
    let mut current_block: u64;
    let mut priv_0: *mut GSimpleProxyResolverPrivate = (*resolver).priv_0;
    let mut ignore_ips: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut ignore_domains: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut host: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut colon: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut bracket: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut iaddr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
    let mut mask: *mut GInetAddressMask = ::core::ptr::null_mut::<GInetAddressMask>();
    let mut domain: GSimpleProxyResolverDomain = GSimpleProxyResolverDomain {
        name: ::core::ptr::null_mut::<gchar>(),
        length: 0,
        port: 0,
    };
    let mut port: gushort = 0;
    let mut i: ::core::ffi::c_int = 0;
    if !(*priv_0).ignore_ips.is_null() {
        g_ptr_array_free((*priv_0).ignore_ips, TRUE);
    }
    if !(*priv_0).ignore_domains.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*(*priv_0).ignore_domains.offset(i as isize))
            .name
            .is_null()
        {
            g_free((*(*priv_0).ignore_domains.offset(i as isize)).name as gpointer);
            i += 1;
        }
        g_free((*priv_0).ignore_domains as gpointer);
    }
    (*priv_0).ignore_ips = ::core::ptr::null_mut::<GPtrArray>();
    (*priv_0).ignore_domains = ::core::ptr::null_mut::<GSimpleProxyResolverDomain>();
    if (*priv_0).ignore_hosts.is_null()
        || (*(*priv_0)
            .ignore_hosts
            .offset(0 as ::core::ffi::c_int as isize))
        .is_null()
    {
        return;
    }
    ignore_ips = g_ptr_array_new_with_free_func(Some(
        g_object_unref as unsafe extern "C" fn(gpointer) -> (),
    ));
    ignore_domains = g_array_new(
        TRUE,
        FALSE,
        ::core::mem::size_of::<GSimpleProxyResolverDomain>() as guint,
    );
    i = 0 as ::core::ffi::c_int;
    while !(*(*priv_0).ignore_hosts.offset(i as isize)).is_null() {
        host = g_strchomp(*(*priv_0).ignore_hosts.offset(i as isize));
        mask = g_inet_address_mask_new_from_string(host, ::core::ptr::null_mut::<*mut GError>());
        if !mask.is_null() {
            g_ptr_array_add(ignore_ips, mask as gpointer);
        } else {
            port = 0 as gushort;
            if *host as ::core::ffi::c_int == '[' as i32 {
                host = host.offset(1);
                bracket = strchr(host, ']' as i32) as *mut gchar;
                if bracket.is_null()
                    || *bracket.offset(1 as ::core::ffi::c_int as isize) == 0
                    || *bracket.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != ':' as i32
                {
                    current_block = 3992665981560110839;
                } else {
                    port = strtoul(
                        bracket.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut tmp,
                        10 as ::core::ffi::c_int,
                    ) as gushort;
                    if *tmp != 0 {
                        current_block = 3992665981560110839;
                    } else {
                        *bracket = '\0' as i32 as gchar;
                        current_block = 17788412896529399552;
                    }
                }
            } else {
                colon = strchr(host, ':' as i32) as *mut gchar;
                if !colon.is_null()
                    && strchr(colon.offset(1 as ::core::ffi::c_int as isize), ':' as i32).is_null()
                {
                    port = strtoul(
                        colon.offset(1 as ::core::ffi::c_int as isize),
                        &raw mut tmp,
                        10 as ::core::ffi::c_int,
                    ) as gushort;
                    if *tmp != 0 {
                        current_block = 3992665981560110839;
                    } else {
                        *colon = '\0' as i32 as gchar;
                        current_block = 17788412896529399552;
                    }
                } else {
                    current_block = 17788412896529399552;
                }
            }
            match current_block {
                3992665981560110839 => {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"Ignoring invalid ignore_hosts value '%s'\0" as *const u8 as *const gchar,
                        host,
                    );
                }
                _ => {
                    iaddr = g_inet_address_new_from_string(host);
                    if !iaddr.is_null() {
                        g_object_unref(iaddr as gpointer);
                    } else if if 0 != 0 {
                        ({
                            let __str: *const ::core::ffi::c_char = host;
                            let __prefix: *const ::core::ffi::c_char =
                                b"*.\0" as *const u8 as *const ::core::ffi::c_char;
                            let mut __result: gboolean = FALSE;
                            if ({
                                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                                if __str.is_null() || __prefix.is_null() {
                                    _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_10
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                __result = g_str_has_prefix(
                                    __str as *const gchar,
                                    __prefix as *const gchar,
                                );
                            } else {
                                let __str_len: size_t = strlen(
                                    __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                                ) as size_t;
                                let __prefix_len: size_t = strlen(
                                    __prefix
                                        .offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                                )
                                    as size_t;
                                if __str_len >= __prefix_len {
                                    __result = (memcmp(
                                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        __prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize
                                        )
                                            as *const ::core::ffi::c_void,
                                        __prefix_len,
                                    ) == 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as gboolean;
                                }
                            }
                            __result
                        })
                    } else {
                        g_str_has_prefix(host, b"*.\0" as *const u8 as *const gchar)
                    } != 0
                    {
                        host = host.offset(2 as ::core::ffi::c_int as isize);
                    } else if *host as ::core::ffi::c_int == '.' as i32 {
                        host = host.offset(1);
                    }
                    memset(
                        &raw mut domain as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<GSimpleProxyResolverDomain>() as size_t,
                    );
                    domain.name = safe_c2rust_g_strdup_inline(host) as *mut gchar;
                    domain.length = strlen(domain.name) as gsize;
                    domain.port = port;
                    g_array_append_vals(
                        ignore_domains,
                        &raw mut domain as gconstpointer,
                        1 as guint,
                    );
                }
            }
        }
        i += 1;
    }
    if (*ignore_ips).len != 0 {
        (*priv_0).ignore_ips = ignore_ips;
    } else {
        g_ptr_array_free(ignore_ips, TRUE);
    }
    if (*ignore_domains).len != 0 {
        (*priv_0).ignore_domains = (*ignore_domains).data as *mut GSimpleProxyResolverDomain;
    }
    g_array_free(
        ignore_domains,
        ((*ignore_domains).len == 0 as guint) as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn safe_c2rust_ignore_host(
    mut resolver: *mut GSimpleProxyResolver,
    mut host: *const gchar,
    mut port: gushort,
) -> gboolean {
    let mut priv_0: *mut GSimpleProxyResolverPrivate = (*resolver).priv_0;
    let mut ascii_host: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ignore: gboolean = FALSE;
    let mut offset: gsize = 0;
    let mut length: gsize = 0;
    let mut i: guint = 0;
    if !(*priv_0).ignore_ips.is_null() {
        let mut iaddr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        iaddr = g_inet_address_new_from_string(host);
        if !iaddr.is_null() {
            i = 0 as guint;
            while i < (*(*priv_0).ignore_ips).len {
                let mut mask: *mut GInetAddressMask =
                    *(*(*priv_0).ignore_ips).pdata.offset(i as isize) as *mut GInetAddressMask;
                if g_inet_address_mask_matches(mask, iaddr) != 0 {
                    ignore = TRUE as gboolean;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            g_object_unref(iaddr as gpointer);
            if ignore != 0 {
                return TRUE;
            }
        }
    }
    if !(*priv_0).ignore_domains.is_null() {
        length = 0 as gsize;
        if g_hostname_is_non_ascii(host) != 0 {
            ascii_host = g_hostname_to_ascii(host);
            host = ascii_host;
        }
        if !host.is_null() {
            length = strlen(host as *const ::core::ffi::c_char) as gsize;
        }
        i = 0 as guint;
        while length > 0 as gsize && (*(*priv_0).ignore_domains.offset(i as isize)).length != 0 {
            let mut domain: *mut GSimpleProxyResolverDomain =
                (*priv_0).ignore_domains.offset(i as isize) as *mut GSimpleProxyResolverDomain;
            if !((*domain).length > length) {
                offset = length.wrapping_sub((*domain).length);
                if ((*domain).port as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || (*domain).port as ::core::ffi::c_int == port as ::core::ffi::c_int)
                    && (offset == 0 as gsize
                        || offset > 0 as gsize
                            && *host.offset(offset.wrapping_sub(1 as gsize) as isize)
                                as ::core::ffi::c_int
                                == '.' as i32)
                    && g_ascii_strcasecmp((*domain).name, host.offset(offset as isize))
                        == 0 as ::core::ffi::c_int
                {
                    ignore = TRUE as gboolean;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        g_free(ascii_host as gpointer);
    }
    return ignore;
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_lookup(
    mut proxy_resolver: *mut GProxyResolver,
    mut uri: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut resolver: *mut GSimpleProxyResolver =
        proxy_resolver as *mut ::core::ffi::c_void as *mut GSimpleProxyResolver;
    let mut priv_0: *mut GSimpleProxyResolverPrivate = (*resolver).priv_0;
    let mut proxy: *const gchar = ::core::ptr::null::<gchar>();
    let mut proxies: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if !(*priv_0).ignore_ips.is_null() || !(*priv_0).ignore_domains.is_null() {
        let mut host: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut port: gint = 0;
        if g_uri_split_network(
            uri,
            G_URI_FLAGS_NONE,
            ::core::ptr::null_mut::<*mut gchar>(),
            &raw mut host,
            &raw mut port,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != 0
            && safe_c2rust_ignore_host(
                resolver,
                host,
                (if port > 0 as ::core::ffi::c_int {
                    port as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as gushort,
            ) != 0
        {
            proxy = b"direct://\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        }
        g_free(host as gpointer);
    }
    if proxy.is_null() && g_hash_table_size((*priv_0).uri_proxies) != 0 {
        let mut scheme: *mut gchar = g_ascii_strdown(
            uri,
            strcspn(
                uri as *const ::core::ffi::c_char,
                b":\0" as *const u8 as *const ::core::ffi::c_char,
            ) as gssize,
        );
        proxy = g_hash_table_lookup((*priv_0).uri_proxies, scheme as gconstpointer) as *const gchar;
        g_free(scheme as gpointer);
    }
    if proxy.is_null() {
        proxy = (*priv_0).default_proxy;
    }
    if proxy.is_null() {
        proxy = b"direct://\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if strncmp(
        proxy as *const ::core::ffi::c_char,
        b"socks://\0" as *const u8 as *const ::core::ffi::c_char,
        8 as size_t,
    ) == 0
    {
        proxies = ({
            let mut __n: gsize = 4 as ::core::ffi::c_int as gsize;
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
        let ref mut fresh0 = *proxies.offset(0 as ::core::ffi::c_int as isize);
        *fresh0 = g_strdup_printf(
            b"socks5://%s\0" as *const u8 as *const gchar,
            proxy.offset(8 as ::core::ffi::c_int as isize),
        );
        let ref mut fresh1 = *proxies.offset(1 as ::core::ffi::c_int as isize);
        *fresh1 = g_strdup_printf(
            b"socks4a://%s\0" as *const u8 as *const gchar,
            proxy.offset(8 as ::core::ffi::c_int as isize),
        );
        let ref mut fresh2 = *proxies.offset(2 as ::core::ffi::c_int as isize);
        *fresh2 = g_strdup_printf(
            b"socks4://%s\0" as *const u8 as *const gchar,
            proxy.offset(8 as ::core::ffi::c_int as isize),
        );
        let ref mut fresh3 = *proxies.offset(3 as ::core::ffi::c_int as isize);
        *fresh3 = ::core::ptr::null_mut::<gchar>();
    } else {
        proxies = ({
            let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
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
        let ref mut fresh4 = *proxies.offset(0 as ::core::ffi::c_int as isize);
        *fresh4 = safe_c2rust_g_strdup_inline(proxy as *const ::core::ffi::c_char) as *mut gchar;
    }
    return proxies;
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_lookup_async(
    mut proxy_resolver: *mut GProxyResolver,
    mut uri: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut resolver: *mut GSimpleProxyResolver =
        proxy_resolver as *mut ::core::ffi::c_void as *mut GSimpleProxyResolver;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut proxies: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    task = g_task_new(resolver as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GProxyResolver,
                    *const gchar,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_simple_proxy_resolver_lookup_async
                as unsafe extern "C" fn(
                    *mut GProxyResolver,
                    *const gchar,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_simple_proxy_resolver_lookup_async\0" as *const u8 as *const gchar,
        );
    }
    proxies = safe_c2rust_g_simple_proxy_resolver_lookup(
        proxy_resolver,
        uri,
        cancellable,
        &raw mut error,
    ) as *mut *mut ::core::ffi::c_char;
    if !proxies.is_null() {
        g_task_return_pointer(
            task,
            proxies as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut *mut gchar) -> ()>,
                GDestroyNotify,
            >(Some(
                g_strfreev as unsafe extern "C" fn(*mut *mut gchar) -> (),
            )),
        );
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_lookup_finish(
    mut resolver: *mut GProxyResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, resolver as gpointer) != 0 {
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
            b"g_task_is_valid (result, resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut *mut gchar;
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_class_init(
    mut resolver_class: *mut GSimpleProxyResolverClass,
) {
    let mut object_class: *mut GObjectClass =
        resolver_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_simple_proxy_resolver_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_simple_proxy_resolver_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize = Some(
        safe_c2rust_g_simple_proxy_resolver_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        object_class,
        PROP_DEFAULT_PROXY as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"default-proxy\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_IGNORE_HOSTS as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"ignore-hosts\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_strv_get_type(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_iface_init(
    mut iface: *mut GProxyResolverInterface,
) {
    (*iface).lookup = Some(
        safe_c2rust_g_simple_proxy_resolver_lookup
            as unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut *mut gchar,
        >;
    (*iface).lookup_async = Some(
        safe_c2rust_g_simple_proxy_resolver_lookup_async
            as unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxyResolver,
                *const gchar,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).lookup_finish = Some(
        safe_c2rust_g_simple_proxy_resolver_lookup_finish
            as unsafe extern "C" fn(
                *mut GProxyResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GProxyResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut *mut gchar,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_new(
    mut default_proxy: *const gchar,
    mut ignore_hosts: *mut *mut gchar,
) -> *mut GProxyResolver {
    return g_object_new(
        safe_c2rust_g_simple_proxy_resolver_get_type(),
        b"default-proxy\0" as *const u8 as *const gchar,
        default_proxy,
        b"ignore-hosts\0" as *const u8 as *const ::core::ffi::c_char,
        ignore_hosts,
        NULL_0,
    ) as *mut GProxyResolver;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_set_default_proxy(
    mut resolver: *mut GSimpleProxyResolver,
    mut default_proxy: *const gchar,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_proxy_resolver_get_type();
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
            b"G_IS_SIMPLE_PROXY_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if default_proxy.is_null()
            || g_uri_is_valid(
                default_proxy,
                G_URI_FLAGS_NONE,
                ::core::ptr::null_mut::<*mut GError>(),
            ) != 0
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
            b"default_proxy == NULL || g_uri_is_valid (default_proxy, G_URI_FLAGS_NONE, NULL)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*resolver).priv_0).default_proxy as gpointer);
    (*(*resolver).priv_0).default_proxy =
        safe_c2rust_g_strdup_inline(default_proxy as *const ::core::ffi::c_char) as *mut gchar;
    g_object_notify(
        resolver as *mut ::core::ffi::c_void as *mut GObject,
        b"default-proxy\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_set_ignore_hosts(
    mut resolver: *mut GSimpleProxyResolver,
    mut ignore_hosts: *mut *mut gchar,
) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_proxy_resolver_get_type();
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
            b"G_IS_SIMPLE_PROXY_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_strfreev((*(*resolver).priv_0).ignore_hosts);
    (*(*resolver).priv_0).ignore_hosts = g_strdupv(ignore_hosts);
    safe_c2rust_reparse_ignore_hosts(resolver);
    g_object_notify(
        resolver as *mut ::core::ffi::c_void as *mut GObject,
        b"ignore-hosts\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_proxy_resolver_set_uri_proxy(
    mut resolver: *mut GSimpleProxyResolver,
    mut uri_scheme: *const gchar,
    mut proxy: *const gchar,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_proxy_resolver_get_type();
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
            b"G_IS_SIMPLE_PROXY_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_hash_table_replace(
        (*(*resolver).priv_0).uri_proxies,
        g_ascii_strdown(uri_scheme, -(1 as ::core::ffi::c_int) as gssize) as gpointer,
        safe_c2rust_g_strdup_inline(proxy as *const ::core::ffi::c_char) as gpointer,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
