extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GSocketConnectable;
    pub type _GTask;
    pub type _GTlsCertificatePrivate;
    pub type _GTlsDatabasePrivate;
    pub type _GTlsInteractionPrivate;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_byte_array_ref(array: *mut GByteArray) -> *mut GByteArray;
    fn g_byte_array_unref(array: *mut GByteArray);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_async_result_get_type() -> GType;
    fn g_cancellable_get_type() -> GType;
    fn g_socket_connectable_get_type() -> GType;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_name(task: *mut GTask, name: *const gchar);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
    fn g_tls_certificate_get_type() -> GType;
    fn g_tls_interaction_get_type() -> GType;
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
pub type GTlsDatabaseVerifyFlags = ::core::ffi::c_uint;
pub const G_TLS_DATABASE_VERIFY_NONE: GTlsDatabaseVerifyFlags = 0;
pub type GTlsDatabaseLookupFlags = ::core::ffi::c_uint;
pub const G_TLS_DATABASE_LOOKUP_KEYPAIR: GTlsDatabaseLookupFlags = 1;
pub const G_TLS_DATABASE_LOOKUP_NONE: GTlsDatabaseLookupFlags = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GSocketConnectable = _GSocketConnectable;
pub type GTask = _GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsCertificate {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsCertificatePrivate,
}
pub type GTlsCertificatePrivate = _GTlsCertificatePrivate;
pub type GTlsCertificate = _GTlsCertificate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsDatabase {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsDatabasePrivate,
}
pub type GTlsDatabasePrivate = _GTlsDatabasePrivate;
pub type GTlsDatabase = _GTlsDatabase;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsInteraction {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsInteractionPrivate,
}
pub type GTlsInteractionPrivate = _GTlsInteractionPrivate;
pub type GTlsInteraction = _GTlsInteraction;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsDatabaseClass {
    pub parent_class: GObjectClass,
    pub verify_chain: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *const gchar,
            *mut GSocketConnectable,
            *mut GTlsInteraction,
            GTlsDatabaseVerifyFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> GTlsCertificateFlags,
    >,
    pub verify_chain_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *const gchar,
            *mut GSocketConnectable,
            *mut GTlsInteraction,
            GTlsDatabaseVerifyFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub verify_chain_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> GTlsCertificateFlags,
    >,
    pub create_certificate_handle:
        Option<unsafe extern "C" fn(*mut GTlsDatabase, *mut GTlsCertificate) -> *mut gchar>,
    pub lookup_certificate_for_handle: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *const gchar,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificate_for_handle_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *const gchar,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_certificate_for_handle_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificate_issuer: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificate_issuer_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GTlsCertificate,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_certificate_issuer_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GTlsCertificate,
    >,
    pub lookup_certificates_issued_by: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GByteArray,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub lookup_certificates_issued_by_async: Option<
        unsafe extern "C" fn(
            *mut GTlsDatabase,
            *mut GByteArray,
            *mut GTlsInteraction,
            GTlsDatabaseLookupFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_certificates_issued_by_finish: Option<
        unsafe extern "C" fn(*mut GTlsDatabase, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
    >,
    pub padding: [gpointer; 16],
}
pub type GTlsDatabaseClass = _GTlsDatabaseClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AsyncLookupCertificatesIssuedBy {
    pub issuer: *mut GByteArray,
    pub interaction: *mut GTlsInteraction,
    pub flags: GTlsDatabaseLookupFlags,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AsyncLookupCertificateIssuer {
    pub certificate: *mut GTlsCertificate,
    pub interaction: *mut GTlsInteraction,
    pub flags: GTlsDatabaseLookupFlags,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AsyncLookupCertificateForHandle {
    pub handle: *mut gchar,
    pub interaction: *mut GTlsInteraction,
    pub flags: GTlsDatabaseLookupFlags,
}
pub type AsyncVerifyChain = _AsyncVerifyChain;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AsyncVerifyChain {
    pub chain: *mut GTlsCertificate,
    pub purpose: *mut gchar,
    pub identity: *mut GSocketConnectable,
    pub interaction: *mut GTlsInteraction,
    pub flags: GTlsDatabaseVerifyFlags,
}
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
static mut safe_c2rust_GTlsDatabase_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_tls_database_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GTlsDatabase\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GTlsDatabaseClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_database_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GTlsDatabase>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTlsDatabase) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_database_init as unsafe extern "C" fn(*mut GTlsDatabase) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_tls_database_get_type_once();
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
static mut safe_c2rust_g_tls_database_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_tls_database_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_tls_database_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GTlsDatabase_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GTlsDatabase_private_offset);
    }
    safe_c2rust_g_tls_database_class_init(klass as *mut GTlsDatabaseClass);
}
unsafe extern "C" fn safe_c2rust_g_tls_database_init(mut cert: *mut GTlsDatabase) {}
unsafe extern "C" fn safe_c2rust_async_verify_chain_free(mut data: gpointer) {
    let mut args: *mut AsyncVerifyChain = data as *mut AsyncVerifyChain;
    let mut _pp: *mut *mut GTlsCertificate = &raw mut (*args).chain;
    let mut _ptr: *mut GTlsCertificate = *_pp;
    *_pp = ::core::ptr::null_mut::<GTlsCertificate>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_free((*args).purpose as gpointer);
    let mut _pp_0: *mut *mut GSocketConnectable = &raw mut (*args).identity;
    let mut _ptr_0: *mut GSocketConnectable = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GSocketConnectable>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut GTlsInteraction = &raw mut (*args).interaction;
    let mut _ptr_1: *mut GTlsInteraction = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GTlsInteraction>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<AsyncVerifyChain>() as gsize,
        args as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_async_verify_chain_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut args: *mut AsyncVerifyChain = task_data as *mut AsyncVerifyChain;
    let mut verify_result: GTlsCertificateFlags = G_TLS_CERTIFICATE_NO_FLAGS;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    verify_result = safe_c2rust_g_tls_database_verify_chain(
        object as *mut GTlsDatabase,
        (*args).chain,
        (*args).purpose,
        (*args).identity,
        (*args).interaction,
        (*args).flags,
        cancellable,
        &raw mut error,
    );
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_int(task, verify_result as gssize);
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_verify_chain_async(
    mut self_0: *mut GTlsDatabase,
    mut chain: *mut GTlsCertificate,
    mut purpose: *const gchar,
    mut identity: *mut GSocketConnectable,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseVerifyFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut args: *mut AsyncVerifyChain = ::core::ptr::null_mut::<AsyncVerifyChain>();
    args = ({
        let mut __s: gsize = ::core::mem::size_of::<AsyncVerifyChain>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut AsyncVerifyChain;
    (*args).chain = g_object_ref(chain as gpointer) as *mut GTlsCertificate as *mut GTlsCertificate;
    (*args).purpose =
        safe_c2rust_g_strdup_inline(purpose as *const ::core::ffi::c_char) as *mut gchar;
    (*args).identity = (if !identity.is_null() {
        g_object_ref(identity as gpointer) as *mut GSocketConnectable
    } else {
        ::core::ptr::null_mut::<GSocketConnectable>()
    }) as *mut GSocketConnectable;
    (*args).interaction = (if !interaction.is_null() {
        g_object_ref(interaction as gpointer) as *mut GTlsInteraction
    } else {
        ::core::ptr::null_mut::<GTlsInteraction>()
    }) as *mut GTlsInteraction;
    (*args).flags = flags;
    task = g_task_new(self_0 as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *mut GTlsCertificate,
                    *const gchar,
                    *mut GSocketConnectable,
                    *mut GTlsInteraction,
                    GTlsDatabaseVerifyFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_tls_database_real_verify_chain_async
                as unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *mut GTlsCertificate,
                    *const gchar,
                    *mut GSocketConnectable,
                    *mut GTlsInteraction,
                    GTlsDatabaseVerifyFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_tls_database_real_verify_chain_async\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] verify TLS chain\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] verify TLS chain\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        args as gpointer,
        Some(safe_c2rust_async_verify_chain_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_async_verify_chain_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_verify_chain_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> GTlsCertificateFlags {
    let mut ret: GTlsCertificateFlags = G_TLS_CERTIFICATE_NO_FLAGS;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, self_0 as gpointer) != 0 {
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
            b"g_task_is_valid (result, self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    ret = g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as GTlsCertificateFlags;
    if ret as ::core::ffi::c_uint == 4294967295 as GTlsCertificateFlags as ::core::ffi::c_uint {
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    } else {
        return ret;
    };
}
unsafe extern "C" fn safe_c2rust_async_lookup_certificate_for_handle_free(mut data: gpointer) {
    let mut args: *mut AsyncLookupCertificateForHandle =
        data as *mut AsyncLookupCertificateForHandle;
    g_free((*args).handle as gpointer);
    let mut _pp: *mut *mut GTlsInteraction = &raw mut (*args).interaction;
    let mut _ptr: *mut GTlsInteraction = *_pp;
    *_pp = ::core::ptr::null_mut::<GTlsInteraction>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<AsyncLookupCertificateForHandle>() as gsize,
        args as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_async_lookup_certificate_for_handle_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut args: *mut AsyncLookupCertificateForHandle =
        task_data as *mut AsyncLookupCertificateForHandle;
    let mut result: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    result = safe_c2rust_g_tls_database_lookup_certificate_for_handle(
        object as *mut GTlsDatabase,
        (*args).handle,
        (*args).interaction,
        (*args).flags,
        cancellable,
        &raw mut error,
    );
    if !result.is_null() {
        g_task_return_pointer(
            task,
            result as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_lookup_certificate_for_handle_async(
    mut self_0: *mut GTlsDatabase,
    mut handle: *const gchar,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut args: *mut AsyncLookupCertificateForHandle =
        ::core::ptr::null_mut::<AsyncLookupCertificateForHandle>();
    args = ({
        let mut __s: gsize = ::core::mem::size_of::<AsyncLookupCertificateForHandle>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut AsyncLookupCertificateForHandle;
    (*args).handle =
        safe_c2rust_g_strdup_inline(handle as *const ::core::ffi::c_char) as *mut gchar;
    (*args).interaction = (if !interaction.is_null() {
        g_object_ref(interaction as gpointer) as *mut GTlsInteraction
    } else {
        ::core::ptr::null_mut::<GTlsInteraction>()
    }) as *mut GTlsInteraction;
    task = g_task_new(self_0 as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *const gchar,
                    *mut GTlsInteraction,
                    GTlsDatabaseLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_tls_database_real_lookup_certificate_for_handle_async
                as unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *const gchar,
                    *mut GTlsInteraction,
                    GTlsDatabaseLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_tls_database_real_lookup_certificate_for_handle_async\0" as *const u8
                as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] lookup TLS certificate\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] lookup TLS certificate\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        args as gpointer,
        Some(
            safe_c2rust_async_lookup_certificate_for_handle_free
                as unsafe extern "C" fn(gpointer) -> (),
        ),
    );
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_async_lookup_certificate_for_handle_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_lookup_certificate_for_handle_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, self_0 as gpointer) != 0 {
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
            b"g_task_is_valid (result, self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GTlsCertificate;
}
unsafe extern "C" fn safe_c2rust_async_lookup_certificate_issuer_free(mut data: gpointer) {
    let mut args: *mut AsyncLookupCertificateIssuer = data as *mut AsyncLookupCertificateIssuer;
    let mut _pp: *mut *mut GTlsCertificate = &raw mut (*args).certificate;
    let mut _ptr: *mut GTlsCertificate = *_pp;
    *_pp = ::core::ptr::null_mut::<GTlsCertificate>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GTlsInteraction = &raw mut (*args).interaction;
    let mut _ptr_0: *mut GTlsInteraction = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GTlsInteraction>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<AsyncLookupCertificateIssuer>() as gsize,
        args as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_async_lookup_certificate_issuer_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut args: *mut AsyncLookupCertificateIssuer =
        task_data as *mut AsyncLookupCertificateIssuer;
    let mut issuer: *mut GTlsCertificate = ::core::ptr::null_mut::<GTlsCertificate>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    issuer = safe_c2rust_g_tls_database_lookup_certificate_issuer(
        object as *mut GTlsDatabase,
        (*args).certificate,
        (*args).interaction,
        (*args).flags,
        cancellable,
        &raw mut error,
    );
    if !issuer.is_null() {
        g_task_return_pointer(
            task,
            issuer as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_lookup_certificate_issuer_async(
    mut self_0: *mut GTlsDatabase,
    mut certificate: *mut GTlsCertificate,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut args: *mut AsyncLookupCertificateIssuer =
        ::core::ptr::null_mut::<AsyncLookupCertificateIssuer>();
    args = ({
        let mut __s: gsize = ::core::mem::size_of::<AsyncLookupCertificateIssuer>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut AsyncLookupCertificateIssuer;
    (*args).certificate =
        g_object_ref(certificate as gpointer) as *mut GTlsCertificate as *mut GTlsCertificate;
    (*args).flags = flags;
    (*args).interaction = (if !interaction.is_null() {
        g_object_ref(interaction as gpointer) as *mut GTlsInteraction
    } else {
        ::core::ptr::null_mut::<GTlsInteraction>()
    }) as *mut GTlsInteraction;
    task = g_task_new(self_0 as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *mut GTlsCertificate,
                    *mut GTlsInteraction,
                    GTlsDatabaseLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_tls_database_real_lookup_certificate_issuer_async
                as unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *mut GTlsCertificate,
                    *mut GTlsInteraction,
                    GTlsDatabaseLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_tls_database_real_lookup_certificate_issuer_async\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] lookup certificate issuer\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] lookup certificate issuer\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        args as gpointer,
        Some(
            safe_c2rust_async_lookup_certificate_issuer_free
                as unsafe extern "C" fn(gpointer) -> (),
        ),
    );
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_async_lookup_certificate_issuer_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_lookup_certificate_issuer_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, self_0 as gpointer) != 0 {
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
            b"g_task_is_valid (result, self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GTlsCertificate;
}
unsafe extern "C" fn safe_c2rust_async_lookup_certificates_issued_by_free(mut data: gpointer) {
    let mut args: *mut AsyncLookupCertificatesIssuedBy =
        data as *mut AsyncLookupCertificatesIssuedBy;
    g_byte_array_unref((*args).issuer);
    let mut _pp: *mut *mut GTlsInteraction = &raw mut (*args).interaction;
    let mut _ptr: *mut GTlsInteraction = *_pp;
    *_pp = ::core::ptr::null_mut::<GTlsInteraction>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<AsyncLookupCertificatesIssuedBy>() as gsize,
        args as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_async_lookup_certificates_free_certificates(mut data: gpointer) {
    let mut list: *mut GList = data as *mut GList;
    g_list_free_full(
        list,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_async_lookup_certificates_issued_by_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut args: *mut AsyncLookupCertificatesIssuedBy =
        task_data as *mut AsyncLookupCertificatesIssuedBy;
    let mut results: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    results = safe_c2rust_g_tls_database_lookup_certificates_issued_by(
        object as *mut GTlsDatabase,
        (*args).issuer,
        (*args).interaction,
        (*args).flags,
        cancellable,
        &raw mut error,
    );
    if !results.is_null() {
        g_task_return_pointer(
            task,
            results as gpointer,
            Some(
                safe_c2rust_async_lookup_certificates_free_certificates
                    as unsafe extern "C" fn(gpointer) -> (),
            ),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_lookup_certificates_issued_by_async(
    mut self_0: *mut GTlsDatabase,
    mut issuer: *mut GByteArray,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut args: *mut AsyncLookupCertificatesIssuedBy =
        ::core::ptr::null_mut::<AsyncLookupCertificatesIssuedBy>();
    args = ({
        let mut __s: gsize = ::core::mem::size_of::<AsyncLookupCertificatesIssuedBy>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut AsyncLookupCertificatesIssuedBy;
    (*args).issuer = g_byte_array_ref(issuer);
    (*args).flags = flags;
    (*args).interaction = (if !interaction.is_null() {
        g_object_ref(interaction as gpointer) as *mut GTlsInteraction
    } else {
        ::core::ptr::null_mut::<GTlsInteraction>()
    }) as *mut GTlsInteraction;
    task = g_task_new(self_0 as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *mut GByteArray,
                    *mut GTlsInteraction,
                    GTlsDatabaseLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_tls_database_real_lookup_certificates_issued_by_async
                as unsafe extern "C" fn(
                    *mut GTlsDatabase,
                    *mut GByteArray,
                    *mut GTlsInteraction,
                    GTlsDatabaseLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_tls_database_real_lookup_certificates_issued_by_async\0" as *const u8
                as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] lookup certificates issued by\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] lookup certificates issued by\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        args as gpointer,
        Some(
            safe_c2rust_async_lookup_certificates_issued_by_free
                as unsafe extern "C" fn(gpointer) -> (),
        ),
    );
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_async_lookup_certificates_issued_by_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_tls_database_real_lookup_certificates_issued_by_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, self_0 as gpointer) != 0 {
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
            b"g_task_is_valid (result, self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GList;
}
unsafe extern "C" fn safe_c2rust_g_tls_database_class_init(mut klass: *mut GTlsDatabaseClass) {
    (*klass).verify_chain_async = Some(
        safe_c2rust_g_tls_database_real_verify_chain_async
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GTlsCertificate,
                *const gchar,
                *mut GSocketConnectable,
                *mut GTlsInteraction,
                GTlsDatabaseVerifyFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GTlsCertificate,
                *const gchar,
                *mut GSocketConnectable,
                *mut GTlsInteraction,
                GTlsDatabaseVerifyFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).verify_chain_finish = Some(
        safe_c2rust_g_tls_database_real_verify_chain_finish
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> GTlsCertificateFlags,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> GTlsCertificateFlags,
        >;
    (*klass).lookup_certificate_for_handle_async = Some(
        safe_c2rust_g_tls_database_real_lookup_certificate_for_handle_async
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *const gchar,
                *mut GTlsInteraction,
                GTlsDatabaseLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *const gchar,
                *mut GTlsInteraction,
                GTlsDatabaseLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).lookup_certificate_for_handle_finish = Some(
        safe_c2rust_g_tls_database_real_lookup_certificate_for_handle_finish
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GTlsCertificate,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GTlsCertificate,
        >;
    (*klass).lookup_certificate_issuer_async = Some(
        safe_c2rust_g_tls_database_real_lookup_certificate_issuer_async
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GTlsCertificate,
                *mut GTlsInteraction,
                GTlsDatabaseLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GTlsCertificate,
                *mut GTlsInteraction,
                GTlsDatabaseLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).lookup_certificate_issuer_finish = Some(
        safe_c2rust_g_tls_database_real_lookup_certificate_issuer_finish
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GTlsCertificate,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GTlsCertificate,
        >;
    (*klass).lookup_certificates_issued_by_async = Some(
        safe_c2rust_g_tls_database_real_lookup_certificates_issued_by_async
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GByteArray,
                *mut GTlsInteraction,
                GTlsDatabaseLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GByteArray,
                *mut GTlsInteraction,
                GTlsDatabaseLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).lookup_certificates_issued_by_finish = Some(
        safe_c2rust_g_tls_database_real_lookup_certificates_issued_by_finish
            as unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GTlsDatabase,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_verify_chain(
    mut self_0: *mut GTlsDatabase,
    mut chain: *mut GTlsCertificate,
    mut purpose: *const gchar,
    mut identity: *mut GSocketConnectable,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseVerifyFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> GTlsCertificateFlags {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = chain as *mut GTypeInstance;
            let mut __t: GType = g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (chain)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !purpose.is_null() {
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
            b"purpose\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if identity.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = identity as *mut GTypeInstance;
                let mut __t: GType = g_socket_connectable_get_type();
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
            b"identity == NULL || G_IS_SOCKET_CONNECTABLE (identity)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .verify_chain
            .is_some()
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
            b"G_TLS_DATABASE_GET_CLASS (self)->verify_chain\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .verify_chain
        .expect("non-null function pointer")(
        self_0,
        chain,
        purpose,
        identity,
        interaction,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_verify_chain_async(
    mut self_0: *mut GTlsDatabase,
    mut chain: *mut GTlsCertificate,
    mut purpose: *const gchar,
    mut identity: *mut GSocketConnectable,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseVerifyFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = chain as *mut GTypeInstance;
            let mut __t: GType = g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (chain)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !purpose.is_null() {
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
            b"purpose != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if identity.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = identity as *mut GTypeInstance;
                let mut __t: GType = g_socket_connectable_get_type();
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
            b"identity == NULL || G_IS_SOCKET_CONNECTABLE (identity)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if callback.is_some() {
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
            b"callback != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .verify_chain_async
            .is_some()
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
            b"G_TLS_DATABASE_GET_CLASS (self)->verify_chain_async\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .verify_chain_async
        .expect("non-null function pointer")(
        self_0,
        chain,
        purpose,
        identity,
        interaction,
        flags,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_verify_chain_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> GTlsCertificateFlags {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .verify_chain_finish
            .is_some()
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
            b"G_TLS_DATABASE_GET_CLASS (self)->verify_chain_finish\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_CERTIFICATE_GENERIC_ERROR;
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .verify_chain_finish
        .expect("non-null function pointer")(self_0, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_create_certificate_handle(
    mut self_0: *mut GTlsDatabase,
    mut certificate: *mut GTlsCertificate,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = certificate as *mut GTypeInstance;
            let mut __t: GType = g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (certificate)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .create_certificate_handle
            .is_some()
        {
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
            b"G_TLS_DATABASE_GET_CLASS (self)->create_certificate_handle\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .create_certificate_handle
        .expect("non-null function pointer")(self_0, certificate);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificate_for_handle(
    mut self_0: *mut GTlsDatabase,
    mut handle: *const gchar,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !handle.is_null() {
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
            b"handle != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificate_for_handle
            .is_some()
        {
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
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificate_for_handle\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificate_for_handle
        .expect("non-null function pointer")(
        self_0,
        handle,
        interaction,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificate_for_handle_async(
    mut self_0: *mut GTlsDatabase,
    mut handle: *const gchar,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !handle.is_null() {
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
            b"handle != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificate_for_handle_async
            .is_some()
        {
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
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificate_for_handle_async\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificate_for_handle_async
        .expect("non-null function pointer")(
        self_0,
        handle,
        interaction,
        flags,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificate_for_handle_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificate_for_handle_finish
            .is_some()
        {
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
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificate_for_handle_finish\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificate_for_handle_finish
        .expect("non-null function pointer")(self_0, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificate_issuer(
    mut self_0: *mut GTlsDatabase,
    mut certificate: *mut GTlsCertificate,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = certificate as *mut GTypeInstance;
            let mut __t: GType = g_tls_certificate_get_type();
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
            b"G_IS_TLS_CERTIFICATE (certificate)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificate_issuer
            .is_some()
        {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificate_issuer\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificate_issuer
        .expect("non-null function pointer")(
        self_0,
        certificate,
        interaction,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificate_issuer_async(
    mut self_0: *mut GTlsDatabase,
    mut certificate: *mut GTlsCertificate,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = certificate as *mut GTypeInstance;
            let mut __t: GType = g_tls_certificate_get_type();
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
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_TLS_CERTIFICATE (certificate)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if callback.is_some() {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"callback != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificate_issuer_async
            .is_some()
        {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificate_issuer_async\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificate_issuer_async
        .expect("non-null function pointer")(
        self_0,
        certificate,
        interaction,
        flags,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificate_issuer_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GTlsCertificate {
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
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
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
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
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificate_issuer_finish
            .is_some()
        {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificate_issuer_finish\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTlsCertificate>();
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificate_issuer_finish
        .expect("non-null function pointer")(self_0, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificates_issued_by(
    mut self_0: *mut GTlsDatabase,
    mut issuer_raw_dn: *mut GByteArray,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if !issuer_raw_dn.is_null() {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"issuer_raw_dn\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificates_issued_by
            .is_some()
        {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificates_issued_by\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificates_issued_by
        .expect("non-null function pointer")(
        self_0,
        issuer_raw_dn,
        interaction,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificates_issued_by_async(
    mut self_0: *mut GTlsDatabase,
    mut issuer_raw_dn: *mut GByteArray,
    mut interaction: *mut GTlsInteraction,
    mut flags: GTlsDatabaseLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if !issuer_raw_dn.is_null() {
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"issuer_raw_dn != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if interaction.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
                let mut __t: GType = g_tls_interaction_get_type();
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
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interaction == NULL || G_IS_TLS_INTERACTION (interaction)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if callback.is_some() {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"callback != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificates_issued_by_async
            .is_some()
        {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificates_issued_by_async\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificates_issued_by_async
        .expect("non-null function pointer")(
        self_0,
        issuer_raw_dn,
        interaction,
        flags,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_certificates_issued_by_finish(
    mut self_0: *mut GTlsDatabase,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_database_get_type();
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
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_TLS_DATABASE (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
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
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
            .lookup_certificates_issued_by_finish
            .is_some()
        {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_TLS_DATABASE_GET_CLASS (self)->lookup_certificates_issued_by_finish\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return (*((*(self_0 as *mut GTypeInstance)).g_class as *mut GTlsDatabaseClass))
        .lookup_certificates_issued_by_finish
        .expect("non-null function pointer")(self_0, result, error);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
