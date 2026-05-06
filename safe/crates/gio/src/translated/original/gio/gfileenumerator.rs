use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GTask;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_set_qdata_full(
        object: *mut GObject,
        quark: GQuark,
        data: gpointer,
        destroy: GDestroyNotify,
    );
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_file_get_type() -> GType;
    fn g_file_get_child(file: *mut GFile, name: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_async_result_get_type() -> GType;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_async_result_is_tagged(res: *mut GAsyncResult, source_tag: gpointer) -> gboolean;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_push_current(cancellable: *mut GCancellable);
    fn g_cancellable_pop_current(cancellable: *mut GCancellable);
    fn g_file_info_get_type() -> GType;
    fn g_file_info_get_name(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn g_io_error_quark() -> GQuark;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_report_new_error(
        source_object: gpointer,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
        source_tag: gpointer,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_priority(task: *mut GTask, priority: gint);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileEnumerator {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileEnumeratorPrivate,
}
pub type GFileEnumeratorPrivate = _GFileEnumeratorPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GFileEnumeratorPrivate {
    pub container: *mut GFile,
    #[bitfield(name = "closed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "pending", ty = "guint", bits = "1..=1")]
    pub closed_pending: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub outstanding_callback: GAsyncReadyCallback,
    pub outstanding_error: *mut GError,
}
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GFile = _GFile;
pub type GFileEnumerator = _GFileEnumerator;
pub type GFileInfo = _GFileInfo;
pub type GTask = _GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileEnumeratorClass {
    pub parent_class: GObjectClass,
    pub next_file: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GFileEnumerator, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub next_files_async: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub next_files_finish: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GFileEnumerator, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileEnumeratorClass = _GFileEnumeratorClass;
pub const PROP_CONTAINER: C2RustUnnamed_1 = 1;
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_file_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GFileEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GFileEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_file_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GFileEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GFileEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_file_enumerator_init
                    as unsafe extern "C" fn(*mut GFileEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GFileEnumerator_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GFileEnumeratorPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_file_enumerator_get_instance_private(
    mut self_0: *mut GFileEnumerator,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GFileEnumerator_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_file_enumerator_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_file_enumerator_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_file_enumerator_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GFileEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GFileEnumerator_private_offset,
        );
    }
    safe_c2rust_g_file_enumerator_class_init(klass as *mut GFileEnumeratorClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_file_enumerator_get_type_once();
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
static mut safe_c2rust_GFileEnumerator_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_file_enumerator_set_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut enumerator: *mut GFileEnumerator = ::core::ptr::null_mut::<GFileEnumerator>();
    enumerator = object as *mut ::core::ffi::c_void as *mut GFileEnumerator;
    match property_id {
        1 => {
            (*(*enumerator).priv_0).container = g_value_dup_object(value) as *mut GFile;
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = property_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileenumerator.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                113 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_dispose(mut object: *mut GObject) {
    let mut enumerator: *mut GFileEnumerator = ::core::ptr::null_mut::<GFileEnumerator>();
    enumerator = object as *mut ::core::ffi::c_void as *mut GFileEnumerator;
    if !(*(*enumerator).priv_0).container.is_null() {
        g_object_unref((*(*enumerator).priv_0).container as gpointer);
        (*(*enumerator).priv_0).container = ::core::ptr::null_mut::<GFile>();
    }
    (*(safe_c2rust_g_file_enumerator_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_finalize(mut object: *mut GObject) {
    let mut enumerator: *mut GFileEnumerator = ::core::ptr::null_mut::<GFileEnumerator>();
    enumerator = object as *mut ::core::ffi::c_void as *mut GFileEnumerator;
    if (*(*enumerator).priv_0).closed() == 0 {
        safe_c2rust_g_file_enumerator_close(
            enumerator,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    (*(safe_c2rust_g_file_enumerator_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_class_init(
    mut klass: *mut GFileEnumeratorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_file_enumerator_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_file_enumerator_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_file_enumerator_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*klass).next_files_async = Some(
        safe_c2rust_g_file_enumerator_real_next_files_async
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).next_files_finish = Some(
        safe_c2rust_g_file_enumerator_real_next_files_finish
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
        >;
    (*klass).close_async = Some(
        safe_c2rust_g_file_enumerator_real_close_async
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*klass).close_finish = Some(
        safe_c2rust_g_file_enumerator_real_close_finish
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
        >;
    g_object_class_install_property(
        gobject_class,
        PROP_CONTAINER as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"container\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_file_get_type(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_init(mut enumerator: *mut GFileEnumerator) {
    (*enumerator).priv_0 = safe_c2rust_g_file_enumerator_get_instance_private(enumerator)
        as *mut GFileEnumeratorPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_next_file(
    mut enumerator: *mut GFileEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !enumerator.is_null() {
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
            b"enumerator != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if (*(*enumerator).priv_0).closed() != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"Enumerator is closed\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if (*(*enumerator).priv_0).pending() != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PENDING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"File enumerator has outstanding operation\0" as *const u8 as *const gchar,
            ),
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if !(*(*enumerator).priv_0).outstanding_error.is_null() {
        g_propagate_error(error, (*(*enumerator).priv_0).outstanding_error);
        (*(*enumerator).priv_0).outstanding_error = ::core::ptr::null_mut::<GError>();
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    (*(*enumerator).priv_0).set_pending(TRUE as guint as guint);
    info = Some((*class).next_file.expect("non-null function pointer"))
        .expect("non-null function pointer")(enumerator, cancellable, error);
    (*(*enumerator).priv_0).set_pending(FALSE as guint as guint);
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_close(
    mut enumerator: *mut GFileEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !enumerator.is_null() {
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
            b"enumerator != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    if (*(*enumerator).priv_0).closed() != 0 {
        return TRUE;
    }
    if (*(*enumerator).priv_0).pending() != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PENDING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"File enumerator has outstanding operation\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    if !cancellable.is_null() {
        g_cancellable_push_current(cancellable);
    }
    (*(*enumerator).priv_0).set_pending(TRUE as guint as guint);
    Some((*class).close_fn.expect("non-null function pointer")).expect("non-null function pointer")(
        enumerator,
        cancellable,
        error,
    );
    (*(*enumerator).priv_0).set_pending(FALSE as guint as guint);
    (*(*enumerator).priv_0).set_closed(TRUE as guint as guint);
    if !cancellable.is_null() {
        g_cancellable_pop_current(cancellable);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_next_async_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut enumerator: *mut GFileEnumerator =
        source_object as *mut ::core::ffi::c_void as *mut GFileEnumerator;
    (*(*enumerator).priv_0).set_pending(FALSE as guint as guint);
    if (*(*enumerator).priv_0).outstanding_callback.is_some() {
        Some(
            (*(*enumerator).priv_0)
                .outstanding_callback
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(source_object, res, user_data);
    }
    g_object_unref(enumerator as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_next_files_async(
    mut enumerator: *mut GFileEnumerator,
    mut num_files: ::core::ffi::c_int,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !enumerator.is_null() {
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
            b"enumerator != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if num_files >= 0 as ::core::ffi::c_int {
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
            b"num_files >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if num_files == 0 as ::core::ffi::c_int {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        task = g_task_new(enumerator as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_enumerator_next_files_async
                    as unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        );
        if g_task_get_name(_task).is_null() {
            g_task_set_static_name(
                _task,
                b"g_file_enumerator_next_files_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_pointer(task, NULL, None);
        g_object_unref(task as gpointer);
        return;
    }
    if (*(*enumerator).priv_0).closed() != 0 {
        g_task_report_new_error(
            enumerator as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_enumerator_next_files_async
                    as unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"File enumerator is already closed\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*enumerator).priv_0).pending() != 0 {
        g_task_report_new_error(
            enumerator as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_enumerator_next_files_async
                    as unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_PENDING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"File enumerator has outstanding operation\0" as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char,
        );
        return;
    }
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    (*(*enumerator).priv_0).set_pending(TRUE as guint as guint);
    (*(*enumerator).priv_0).outstanding_callback = callback;
    g_object_ref(enumerator as gpointer);
    Some(
        (*class)
            .next_files_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        enumerator,
        num_files,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_next_async_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_next_files_finish(
    mut enumerator: *mut GFileEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return ::core::ptr::null_mut::<GList>();
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_enumerator_next_files_async
                as unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
            as *mut GList;
    }
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    return (*class)
        .next_files_finish
        .expect("non-null function pointer")(enumerator, result, error);
}
unsafe extern "C" fn safe_c2rust_close_async_callback_wrapper(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut enumerator: *mut GFileEnumerator =
        source_object as *mut ::core::ffi::c_void as *mut GFileEnumerator;
    (*(*enumerator).priv_0).set_pending(FALSE as guint as guint);
    (*(*enumerator).priv_0).set_closed(TRUE as guint as guint);
    if (*(*enumerator).priv_0).outstanding_callback.is_some() {
        Some(
            (*(*enumerator).priv_0)
                .outstanding_callback
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(source_object, res, user_data);
    }
    g_object_unref(enumerator as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_close_async(
    mut enumerator: *mut GFileEnumerator,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*enumerator).priv_0).closed() != 0 {
        g_task_report_new_error(
            enumerator as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_enumerator_close_async
                    as unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"File enumerator is already closed\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*enumerator).priv_0).pending() != 0 {
        g_task_report_new_error(
            enumerator as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_enumerator_close_async
                    as unsafe extern "C" fn(
                        *mut GFileEnumerator,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_PENDING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"File enumerator has outstanding operation\0" as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char,
        );
        return;
    }
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    (*(*enumerator).priv_0).set_pending(TRUE as guint as guint);
    (*(*enumerator).priv_0).outstanding_callback = callback;
    g_object_ref(enumerator as gpointer);
    Some((*class).close_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        enumerator,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_close_async_callback_wrapper
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_close_finish(
    mut enumerator: *mut GFileEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return FALSE;
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_enumerator_close_async
                as unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    return (*class).close_finish.expect("non-null function pointer")(enumerator, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_is_closed(
    mut enumerator: *mut GFileEnumerator,
) -> gboolean {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return (*(*enumerator).priv_0).closed() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_has_pending(
    mut enumerator: *mut GFileEnumerator,
) -> gboolean {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return (*(*enumerator).priv_0).pending() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_set_pending(
    mut enumerator: *mut GFileEnumerator,
    mut pending: gboolean,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*enumerator).priv_0).set_pending(pending as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_iterate(
    mut direnum: *mut GFileEnumerator,
    mut out_info: *mut *mut GFileInfo,
    mut out_child: *mut *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    let mut temp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret_info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    static mut safe_c2rust_cached_info_quark: GQuark = 0;
    static mut safe_c2rust_cached_child_quark: GQuark = 0;
    static mut safe_c2rust_quarks_initialized: gsize = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !direnum.is_null() {
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
            b"direnum != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !out_info.is_null() || !out_child.is_null() {
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
            b"out_info != NULL || out_child != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_quarks_initialized;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_quarks_initialized;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(
                &raw mut safe_c2rust_quarks_initialized as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_cached_info_quark =
            g_quark_from_static_string(b"g-cached-info\0" as *const u8 as *const gchar);
        safe_c2rust_cached_child_quark =
            g_quark_from_static_string(b"g-cached-child\0" as *const u8 as *const gchar);
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_quarks_initialized = 1 as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_quarks_initialized as *mut ::core::ffi::c_void,
            1 as ::core::ffi::c_int as gsize,
        );
    }
    ret_info = safe_c2rust_g_file_enumerator_next_file(direnum, cancellable, &raw mut temp_error);
    if !temp_error.is_null() {
        g_propagate_error(error, temp_error);
    } else {
        if !ret_info.is_null() {
            if !out_child.is_null() {
                let mut name: *const ::core::ffi::c_char = g_file_info_get_name(ret_info);
                if ({
                    let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                    if name.is_null() {
                        _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_27
                }) as ::core::ffi::c_long
                    != 0
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"g_file_enumerator_iterate() created without standard::name\0" as *const u8
                            as *const gchar,
                    );
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"file %s: line %d (%s): should not be reached\0" as *const u8
                            as *const gchar,
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileenumerator.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        718 as ::core::ffi::c_int,
                        G_STRFUNC,
                    );
                    return 0 as gboolean;
                } else {
                    *out_child = g_file_get_child(
                        safe_c2rust_g_file_enumerator_get_container(direnum),
                        name,
                    );
                    g_object_set_qdata_full(
                        direnum as *mut GObject,
                        safe_c2rust_cached_child_quark,
                        *out_child as gpointer,
                        ::core::mem::transmute::<
                            Option<unsafe extern "C" fn(gpointer) -> ()>,
                            GDestroyNotify,
                        >(Some(
                            g_object_unref as unsafe extern "C" fn(gpointer) -> (),
                        )),
                    );
                }
            }
            if !out_info.is_null() {
                g_object_set_qdata_full(
                    direnum as *mut GObject,
                    safe_c2rust_cached_info_quark,
                    ret_info as gpointer,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(gpointer) -> ()>,
                        GDestroyNotify,
                    >(Some(
                        g_object_unref as unsafe extern "C" fn(gpointer) -> (),
                    )),
                );
                *out_info = ret_info;
            } else {
                g_object_unref(ret_info as gpointer);
            }
        } else {
            if !out_info.is_null() {
                *out_info = ::core::ptr::null_mut::<GFileInfo>();
            }
            if !out_child.is_null() {
                *out_child = ::core::ptr::null_mut::<GFile>();
            }
        }
        ret = TRUE as gboolean;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_get_container(
    mut enumerator: *mut GFileEnumerator,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return (*(*enumerator).priv_0).container;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerator_get_child(
    mut enumerator: *mut GFileEnumerator,
    mut info: *mut GFileInfo,
) -> *mut GFile {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_enumerator_get_type();
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
            b"G_IS_FILE_ENUMERATOR (enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = g_file_info_get_type();
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    name = g_file_info_get_name(info) as *const gchar;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if name.is_null() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileEnumerator created without standard::name\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileenumerator.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            803 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return g_file_get_child(
        (*(*enumerator).priv_0).container,
        name as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn safe_c2rust_next_async_op_free(mut files: *mut GList) {
    g_list_free_full(
        files,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_next_files_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut enumerator: *mut GFileEnumerator = source_object as *mut GFileEnumerator;
    let mut num_files: ::core::ffi::c_int = task_data as glong as ::core::ffi::c_int;
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    let mut files: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut i: ::core::ffi::c_int = 0;
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    i = 0 as ::core::ffi::c_int;
    while i < num_files {
        if g_cancellable_set_error_if_cancelled(cancellable, &raw mut error) != 0 {
            info = ::core::ptr::null_mut::<GFileInfo>();
        } else {
            info = (*class).next_file.expect("non-null function pointer")(
                enumerator,
                cancellable,
                &raw mut error,
            );
        }
        if info.is_null() {
            if !error.is_null() && i > 0 as ::core::ffi::c_int {
                if g_error_matches(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
                ) != 0
                {
                    g_error_free(error);
                } else {
                    (*(*enumerator).priv_0).outstanding_error = error;
                }
                error = ::core::ptr::null_mut::<GError>();
            }
            break;
        } else {
            files = g_list_prepend(files, info as gpointer);
            i += 1;
        }
    }
    if !error.is_null() {
        g_list_free_full(
            files,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            files as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GList) -> ()>, GDestroyNotify>(
                Some(safe_c2rust_next_async_op_free as unsafe extern "C" fn(*mut GList) -> ()),
            ),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_real_next_files_async(
    mut enumerator: *mut GFileEnumerator,
    mut num_files: ::core::ffi::c_int,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(enumerator as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_enumerator_real_next_files_async
                as unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_enumerator_real_next_files_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(task, num_files as glong as gpointer, None);
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_next_files_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_real_next_files_finish(
    mut enumerator: *mut GFileEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, enumerator as gpointer) != 0 {
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
            b"g_task_is_valid (result, enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GList;
}
unsafe extern "C" fn safe_c2rust_close_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut enumerator: *mut GFileEnumerator = source_object as *mut GFileEnumerator;
    let mut class: *mut GFileEnumeratorClass = ::core::ptr::null_mut::<GFileEnumeratorClass>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: gboolean = 0;
    class = (*(enumerator as *mut GTypeInstance)).g_class as *mut GFileEnumeratorClass;
    result = (*class).close_fn.expect("non-null function pointer")(
        enumerator,
        cancellable,
        &raw mut error,
    );
    if result != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_real_close_async(
    mut enumerator: *mut GFileEnumerator,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(enumerator as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_enumerator_real_close_async
                as unsafe extern "C" fn(
                    *mut GFileEnumerator,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_enumerator_real_close_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_close_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_enumerator_real_close_finish(
    mut enumerator: *mut GFileEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, enumerator as gpointer) != 0 {
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
            b"g_task_is_valid (result, enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
