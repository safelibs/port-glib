use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GSequence;
    pub type _GSequenceNode;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GUnixMountEntry;
    pub type _GUnixMountMonitor;
    pub type _GWakeup;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_iter_remove(iter: *mut GHashTableIter);
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_is_destroyed(source: *mut GSource) -> gboolean;
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
    fn g_source_get_time(source: *mut GSource) -> gint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_queue_push_tail(queue: *mut GQueue, data: gpointer);
    fn g_queue_pop_head(queue: *mut GQueue) -> gpointer;
    fn g_sequence_new(data_destroy: GDestroyNotify) -> *mut GSequence;
    fn g_sequence_free(seq: *mut GSequence);
    fn g_sequence_sort(seq: *mut GSequence, cmp_func: GCompareDataFunc, cmp_data: gpointer);
    fn g_sequence_is_empty(seq: *mut GSequence) -> gboolean;
    fn g_sequence_get_begin_iter(seq: *mut GSequence) -> *mut GSequenceIter;
    fn g_sequence_insert_sorted(
        seq: *mut GSequence,
        data: gpointer,
        cmp_func: GCompareDataFunc,
        cmp_data: gpointer,
    ) -> *mut GSequenceIter;
    fn g_sequence_sort_changed(
        iter: *mut GSequenceIter,
        cmp_func: GCompareDataFunc,
        cmp_data: gpointer,
    );
    fn g_sequence_remove(iter: *mut GSequenceIter);
    fn g_sequence_get(iter: *mut GSequenceIter) -> gpointer;
    fn g_sequence_iter_is_end(iter: *mut GSequenceIter) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_signal_connect_object(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        gobject: gpointer,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_weak_ref_set(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_file_monitor_get_type() -> GType;
    fn g_file_monitor_emit_event(
        monitor: *mut GFileMonitor,
        child: *mut GFile,
        other_file: *mut GFile,
        event_type: GFileMonitorEvent,
    );
    fn g_file_new_for_path(path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_io_error_quark() -> GQuark;
    fn g_unix_mount_free(mount_entry: *mut GUnixMountEntry);
    fn g_unix_mount_at(
        mount_path: *const ::core::ffi::c_char,
        time_read: *mut guint64,
    ) -> *mut GUnixMountEntry;
    fn g_unix_mount_monitor_get() -> *mut GUnixMountMonitor;
    fn _g_io_module_get_default_type(
        extension_point: *const gchar,
        envvar: *const gchar,
        is_supported_offset: guint,
    ) -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_local_file_new(filename: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_local_file_is_nfs_home(filename: *const gchar) -> gboolean;
    fn g_local_file_new_from_dirname_and_basename(
        dirname: *const ::core::ffi::c_char,
        basename: *const ::core::ffi::c_char,
    ) -> *mut GFile;
    fn glib__private__() -> *const GLibPrivateVTable;
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
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
pub type GDir = _GDir;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GQueue {
    pub head: *mut GList,
    pub tail: *mut GList,
    pub length: guint,
}
pub type GQueue = _GQueue;
pub type GSequence = _GSequence;
pub type GSequenceIter = _GSequenceNode;
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
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
pub type GConnectFlags = ::core::ffi::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
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
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p: gpointer,
}
pub type GFileMonitorFlags = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_WATCH_MOVES: GFileMonitorFlags = 8;
pub const G_FILE_MONITOR_WATCH_HARD_LINKS: GFileMonitorFlags = 4;
pub const G_FILE_MONITOR_SEND_MOVED: GFileMonitorFlags = 2;
pub const G_FILE_MONITOR_WATCH_MOUNTS: GFileMonitorFlags = 1;
pub const G_FILE_MONITOR_NONE: GFileMonitorFlags = 0;
pub type GFileMonitorEvent = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_EVENT_MOVED_OUT: GFileMonitorEvent = 10;
pub const G_FILE_MONITOR_EVENT_MOVED_IN: GFileMonitorEvent = 9;
pub const G_FILE_MONITOR_EVENT_RENAMED: GFileMonitorEvent = 8;
pub const G_FILE_MONITOR_EVENT_MOVED: GFileMonitorEvent = 7;
pub const G_FILE_MONITOR_EVENT_UNMOUNTED: GFileMonitorEvent = 6;
pub const G_FILE_MONITOR_EVENT_PRE_UNMOUNT: GFileMonitorEvent = 5;
pub const G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED: GFileMonitorEvent = 4;
pub const G_FILE_MONITOR_EVENT_CREATED: GFileMonitorEvent = 3;
pub const G_FILE_MONITOR_EVENT_DELETED: GFileMonitorEvent = 2;
pub const G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT: GFileMonitorEvent = 1;
pub const G_FILE_MONITOR_EVENT_CHANGED: GFileMonitorEvent = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_1 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_1 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_1 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_1 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_1 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_1 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_1 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_1 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_1 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_1 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_1 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_1 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_1 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_1 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_1 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_1 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_1 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_1 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_1 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_1 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_1 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_1 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_1 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_1 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_1 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_1 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_1 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_1 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_1 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_1 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_1 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_1 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_1 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_1 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_1 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_1 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_1 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_1 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_1 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_1 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_1 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_1 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_1 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_1 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
pub type GFile = _GFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitorClass {
    pub parent_class: GObjectClass,
    pub changed: Option<
        unsafe extern "C" fn(*mut GFileMonitor, *mut GFile, *mut GFile, GFileMonitorEvent) -> (),
    >,
    pub cancel: Option<unsafe extern "C" fn(*mut GFileMonitor) -> gboolean>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileMonitorClass = _GFileMonitorClass;
pub type GUnixMountEntry = _GUnixMountEntry;
pub type GUnixMountMonitor = _GUnixMountMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileMonitor {
    pub parent_instance: GFileMonitor,
    pub source: *mut GFileMonitorSource,
    pub mount_monitor: *mut GUnixMountMonitor,
    pub was_mounted: gboolean,
}
pub type GFileMonitorSource = _GFileMonitorSource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitorSource {
    pub source: GSource,
    pub lock: GMutex,
    pub instance_ref: GWeakRef,
    pub flags: GFileMonitorFlags,
    pub dirname: *mut gchar,
    pub basename: *mut gchar,
    pub filename: *mut gchar,
    pub pending_changes: *mut GSequence,
    pub pending_changes_table: *mut GHashTable,
    pub event_queue: GQueue,
    pub rate_limit: gint64,
}
pub type GLocalFileMonitor = _GLocalFileMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileMonitorClass {
    pub parent_class: GFileMonitorClass,
    pub is_supported: Option<unsafe extern "C" fn() -> gboolean>,
    pub start: Option<
        unsafe extern "C" fn(
            *mut GLocalFileMonitor,
            *const gchar,
            *const gchar,
            *const gchar,
            *mut GFileMonitorSource,
        ) -> (),
    >,
    pub mount_notify: gboolean,
}
pub type GLocalFileMonitorClass = _GLocalFileMonitorClass;
pub const PROP_RATE_LIMIT: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct PendingChange {
    pub child: *mut gchar,
    #[bitfield(name = "last_emission", ty = "guint64", bits = "0..=62")]
    #[bitfield(name = "dirty", ty = "guint64", bits = "63..=63")]
    pub last_emission_dirty: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueuedEvent {
    pub event_type: GFileMonitorEvent,
    pub child: *mut GFile,
    pub other: *mut GFile,
}
pub type GFileMonitorCallback = Option<
    unsafe extern "C" fn(
        *mut GFileMonitor,
        *mut GFile,
        *mut GFile,
        GFileMonitorEvent,
        gpointer,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLibPrivateVTable {
    pub g_wakeup_new: Option<unsafe extern "C" fn() -> *mut GWakeup>,
    pub g_wakeup_free: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_get_pollfd: Option<unsafe extern "C" fn(*mut GWakeup, *mut GPollFD) -> ()>,
    pub g_wakeup_signal: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_acknowledge: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_get_worker_context: Option<unsafe extern "C" fn() -> *mut GMainContext>,
    pub g_check_setuid: Option<unsafe extern "C" fn() -> gboolean>,
    pub g_main_context_new_with_next_id: Option<unsafe extern "C" fn(guint) -> *mut GMainContext>,
    pub g_dir_open_with_errno: Option<unsafe extern "C" fn(*const gchar, guint) -> *mut GDir>,
    pub g_dir_new_from_dirp: Option<unsafe extern "C" fn(gpointer) -> *mut GDir>,
    pub glib_init: Option<unsafe extern "C" fn() -> ()>,
    pub g_win32_push_empty_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_win32_pop_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_find_program_for_path: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub g_uri_get_default_scheme_port:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub g_set_prgname_once: Option<unsafe extern "C" fn(*const gchar) -> gboolean>,
    pub g_datalist_id_update_atomic: Option<
        unsafe extern "C" fn(
            *mut *mut GData,
            GQuark,
            GDataListUpdateAtomicFunc,
            gpointer,
        ) -> gpointer,
    >,
}
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWakeup = _GWakeup;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TIME_SPAN_SECOND: ::core::ffi::c_long = 1000000 as ::core::ffi::c_long;
pub const G_TIME_SPAN_MILLISECOND: ::core::ffi::c_long = 1000 as ::core::ffi::c_long;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_LOCAL_FILE_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"gio-local-file-monitor\0")
};
pub const G_NFS_FILE_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"gio-nfs-file-monitor\0")
};
unsafe extern "C" fn safe_c2rust_pending_change_get_ready_time(
    mut change: *const PendingChange,
    mut fms: *mut GFileMonitorSource,
) -> gint64 {
    if (*change).dirty() != 0 {
        return (*change)
            .last_emission()
            .wrapping_add((*fms).rate_limit as guint64) as gint64;
    } else {
        return (*change)
            .last_emission()
            .wrapping_add((2 as ::core::ffi::c_long * G_TIME_SPAN_SECOND) as guint64)
            as gint64;
    };
}
unsafe extern "C" fn safe_c2rust_pending_change_compare_ready_time(
    mut a_p: gconstpointer,
    mut b_p: gconstpointer,
    mut user_data: gpointer,
) -> ::core::ffi::c_int {
    let mut fms: *mut GFileMonitorSource = user_data as *mut GFileMonitorSource;
    let mut a: *const PendingChange = a_p as *const PendingChange;
    let mut b: *const PendingChange = b_p as *const PendingChange;
    let mut ready_time_a: gint64 = 0;
    let mut ready_time_b: gint64 = 0;
    ready_time_a = safe_c2rust_pending_change_get_ready_time(a, fms);
    ready_time_b = safe_c2rust_pending_change_get_ready_time(b, fms);
    if ready_time_a < ready_time_b {
        return -(1 as ::core::ffi::c_int);
    } else {
        return (ready_time_a > ready_time_b) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn safe_c2rust_pending_change_free(mut data: gpointer) {
    let mut change: *mut PendingChange = data as *mut PendingChange;
    g_free((*change).child as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<PendingChange>() as gsize,
        change as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_queued_event_free(mut event: *mut QueuedEvent) {
    g_object_unref((*event).child as gpointer);
    if !(*event).other.is_null() {
        g_object_unref((*event).other as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<QueuedEvent>() as gsize,
        event as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_get_ready_time(
    mut fms: *mut GFileMonitorSource,
) -> gint64 {
    let mut iter: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if (*fms).event_queue.length != 0 {
        return 0 as gint64;
    }
    iter = g_sequence_get_begin_iter((*fms).pending_changes);
    if g_sequence_iter_is_end(iter) != 0 {
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    return safe_c2rust_pending_change_get_ready_time(
        g_sequence_get(iter) as *const PendingChange,
        fms,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_update_ready_time(
    mut fms: *mut GFileMonitorSource,
) {
    g_source_set_ready_time(
        fms as *mut GSource,
        safe_c2rust_g_file_monitor_source_get_ready_time(fms),
    );
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_find_pending_change(
    mut fms: *mut GFileMonitorSource,
    mut child: *const gchar,
) -> *mut GSequenceIter {
    return g_hash_table_lookup((*fms).pending_changes_table, child as gconstpointer)
        as *mut GSequenceIter;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_add_pending_change(
    mut fms: *mut GFileMonitorSource,
    mut child: *const gchar,
    mut now: gint64,
) {
    let mut change: *mut PendingChange = ::core::ptr::null_mut::<PendingChange>();
    let mut iter: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    change = g_slice_alloc(::core::mem::size_of::<PendingChange>() as gsize) as *mut PendingChange;
    (*change).child =
        safe_c2rust_g_strdup_inline(child as *const ::core::ffi::c_char) as *mut gchar;
    (*change).set_last_emission(now as guint64 as guint64);
    (*change).set_dirty(FALSE as guint64 as guint64);
    iter = g_sequence_insert_sorted(
        (*fms).pending_changes,
        change as gpointer,
        Some(
            safe_c2rust_pending_change_compare_ready_time
                as unsafe extern "C" fn(
                    gconstpointer,
                    gconstpointer,
                    gpointer,
                ) -> ::core::ffi::c_int,
        ),
        fms as gpointer,
    );
    g_hash_table_insert(
        (*fms).pending_changes_table,
        (*change).child as gpointer,
        iter as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_set_pending_change_dirty(
    mut fms: *mut GFileMonitorSource,
    mut iter: *mut GSequenceIter,
) -> gboolean {
    let mut change: *mut PendingChange = ::core::ptr::null_mut::<PendingChange>();
    change = g_sequence_get(iter) as *mut PendingChange;
    if (*change).dirty() != 0 {
        return FALSE;
    }
    (*change).set_dirty(TRUE as guint64 as guint64);
    g_sequence_sort_changed(
        iter,
        Some(
            safe_c2rust_pending_change_compare_ready_time
                as unsafe extern "C" fn(
                    gconstpointer,
                    gconstpointer,
                    gpointer,
                ) -> ::core::ffi::c_int,
        ),
        fms as gpointer,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_get_pending_change_dirty(
    mut fms: *mut GFileMonitorSource,
    mut iter: *mut GSequenceIter,
) -> gboolean {
    let mut change: *mut PendingChange = ::core::ptr::null_mut::<PendingChange>();
    change = g_sequence_get(iter) as *mut PendingChange;
    return (*change).dirty() as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_remove_pending_change(
    mut fms: *mut GFileMonitorSource,
    mut iter: *mut GSequenceIter,
    mut child: *const gchar,
) {
    g_hash_table_remove((*fms).pending_changes_table, child as gconstpointer);
    g_sequence_remove(iter);
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_queue_event(
    mut fms: *mut GFileMonitorSource,
    mut event_type: GFileMonitorEvent,
    mut child: *const gchar,
    mut other: *mut GFile,
) {
    let mut event: *mut QueuedEvent = ::core::ptr::null_mut::<QueuedEvent>();
    event = g_slice_alloc(::core::mem::size_of::<QueuedEvent>() as gsize) as *mut QueuedEvent;
    (*event).event_type = event_type;
    if !child.is_null() && !(*fms).dirname.is_null() {
        (*event).child = g_local_file_new_from_dirname_and_basename(
            (*fms).dirname,
            child as *const ::core::ffi::c_char,
        );
    } else if !child.is_null() {
        let mut dirname: *mut gchar = g_path_get_dirname((*fms).filename);
        (*event).child = g_local_file_new_from_dirname_and_basename(
            dirname,
            child as *const ::core::ffi::c_char,
        );
        g_free(dirname as gpointer);
    } else if !(*fms).dirname.is_null() {
        (*event).child = _g_local_file_new((*fms).dirname);
    } else if !(*fms).filename.is_null() {
        (*event).child = _g_local_file_new((*fms).filename);
    }
    (*event).other = other;
    if !other.is_null() {
        g_object_ref(other as gpointer);
    }
    g_queue_push_tail(&raw mut (*fms).event_queue, event as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_file_changed(
    mut fms: *mut GFileMonitorSource,
    mut child: *const gchar,
    mut now: gint64,
) -> gboolean {
    let mut pending: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut interesting: gboolean = 0;
    pending = safe_c2rust_g_file_monitor_source_find_pending_change(fms, child);
    if pending.is_null() {
        safe_c2rust_g_file_monitor_source_queue_event(
            fms,
            G_FILE_MONITOR_EVENT_CHANGED,
            child,
            ::core::ptr::null_mut::<GFile>(),
        );
        safe_c2rust_g_file_monitor_source_add_pending_change(fms, child, now);
        interesting = TRUE as gboolean;
    } else {
        interesting = safe_c2rust_g_file_monitor_source_set_pending_change_dirty(fms, pending);
    }
    safe_c2rust_g_file_monitor_source_update_ready_time(fms);
    return interesting;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_file_changes_done(
    mut fms: *mut GFileMonitorSource,
    mut child: *const gchar,
) {
    let mut pending: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    pending = safe_c2rust_g_file_monitor_source_find_pending_change(fms, child);
    if !pending.is_null() {
        if safe_c2rust_g_file_monitor_source_get_pending_change_dirty(fms, pending) != 0 {
            safe_c2rust_g_file_monitor_source_queue_event(
                fms,
                G_FILE_MONITOR_EVENT_CHANGED,
                child,
                ::core::ptr::null_mut::<GFile>(),
            );
        }
        safe_c2rust_g_file_monitor_source_queue_event(
            fms,
            G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT,
            child,
            ::core::ptr::null_mut::<GFile>(),
        );
        safe_c2rust_g_file_monitor_source_remove_pending_change(fms, pending, child);
    }
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_file_created(
    mut fms: *mut GFileMonitorSource,
    mut child: *const gchar,
    mut event_time: gint64,
) {
    safe_c2rust_g_file_monitor_source_file_changes_done(fms, child);
    safe_c2rust_g_file_monitor_source_queue_event(
        fms,
        G_FILE_MONITOR_EVENT_CREATED,
        child,
        ::core::ptr::null_mut::<GFile>(),
    );
    safe_c2rust_g_file_monitor_source_add_pending_change(fms, child, event_time);
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_send_event(
    mut fms: *mut GFileMonitorSource,
    mut event_type: GFileMonitorEvent,
    mut child: *const gchar,
    mut other: *mut GFile,
) {
    safe_c2rust_g_file_monitor_source_file_changes_done(fms, child);
    safe_c2rust_g_file_monitor_source_queue_event(fms, event_type, child, other);
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_send_synthetic_created(
    mut fms: *mut GFileMonitorSource,
    mut child: *const gchar,
) {
    safe_c2rust_g_file_monitor_source_file_changes_done(fms, child);
    safe_c2rust_g_file_monitor_source_queue_event(
        fms,
        G_FILE_MONITOR_EVENT_CREATED,
        child,
        ::core::ptr::null_mut::<GFile>(),
    );
    safe_c2rust_g_file_monitor_source_queue_event(
        fms,
        G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT,
        child,
        ::core::ptr::null_mut::<GFile>(),
    );
}
unsafe extern "C" fn safe_c2rust_is_basename(mut name: *const gchar) -> gboolean {
    if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
        && (*name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
            && *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
            || *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32)
    {
        return FALSE;
    }
    return strchr(name as *const ::core::ffi::c_char, '/' as i32).is_null() as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_monitor_source_handle_event(
    mut fms: *mut GFileMonitorSource,
    mut event_type: GFileMonitorEvent,
    mut child: *const gchar,
    mut rename_to: *const gchar,
    mut other: *mut GFile,
    mut event_time: gint64,
) -> gboolean {
    let mut interesting: gboolean = TRUE;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if child.is_null() || safe_c2rust_is_basename(child) != 0 {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            354 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!child || is_basename (child)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if rename_to.is_null() || safe_c2rust_is_basename(rename_to) != 0 {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            355 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!rename_to || is_basename (rename_to)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*fms).basename.is_null()
        && (child.is_null()
            || !(strcmp(
                child as *const ::core::ffi::c_char,
                (*fms).basename as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int))
        && (rename_to.is_null()
            || !(strcmp(
                rename_to as *const ::core::ffi::c_char,
                (*fms).basename as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int))
    {
        return TRUE;
    }
    g_mutex_lock(&raw mut (*fms).lock);
    if g_source_is_destroyed(fms as *mut GSource) != 0 {
        g_mutex_unlock(&raw mut (*fms).lock);
        return TRUE;
    }
    match event_type as ::core::ffi::c_uint {
        3 => {
            if ({
                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                if other.is_null() && rename_to.is_null() {
                    _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_12
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    389 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!other && !rename_to\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_g_file_monitor_source_file_created(fms, child, event_time);
        }
        0 => {
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if other.is_null() && rename_to.is_null() {
                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_13
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    394 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!other && !rename_to\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            interesting = safe_c2rust_g_file_monitor_source_file_changed(fms, child, event_time);
        }
        1 => {
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if other.is_null() && rename_to.is_null() {
                    _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_14
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    399 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!other && !rename_to\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_g_file_monitor_source_file_changes_done(fms, child);
        }
        9 => {
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if rename_to.is_null() {
                    _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_15
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!rename_to\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*fms).flags as ::core::ffi::c_uint
                & G_FILE_MONITOR_WATCH_MOVES as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                safe_c2rust_g_file_monitor_source_send_event(
                    fms,
                    G_FILE_MONITOR_EVENT_MOVED_IN,
                    child,
                    other,
                );
            } else {
                safe_c2rust_g_file_monitor_source_send_synthetic_created(fms, child);
            }
        }
        10 => {
            if ({
                let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                if rename_to.is_null() {
                    _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_16
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    412 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!rename_to\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*fms).flags as ::core::ffi::c_uint
                & G_FILE_MONITOR_WATCH_MOVES as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                safe_c2rust_g_file_monitor_source_send_event(
                    fms,
                    G_FILE_MONITOR_EVENT_MOVED_OUT,
                    child,
                    other,
                );
            } else if !other.is_null()
                && (*fms).flags as ::core::ffi::c_uint
                    & G_FILE_MONITOR_SEND_MOVED as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
            {
                safe_c2rust_g_file_monitor_source_send_event(
                    fms,
                    G_FILE_MONITOR_EVENT_MOVED,
                    child,
                    other,
                );
            } else {
                safe_c2rust_g_file_monitor_source_send_event(
                    fms,
                    G_FILE_MONITOR_EVENT_DELETED,
                    child,
                    ::core::ptr::null_mut::<GFile>(),
                );
            }
        }
        8 => {
            if ({
                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                if other.is_null() && !rename_to.is_null() {
                    _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_17
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    422 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!other && rename_to\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*fms).flags as ::core::ffi::c_uint
                & (G_FILE_MONITOR_WATCH_MOVES as ::core::ffi::c_int
                    | G_FILE_MONITOR_SEND_MOVED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                != 0
            {
                let mut other_file: *mut GFile = ::core::ptr::null_mut::<GFile>();
                let mut dirname: *const gchar = ::core::ptr::null::<gchar>();
                let mut allocated_dirname: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut event: GFileMonitorEvent = G_FILE_MONITOR_EVENT_CHANGED;
                event = (if (*fms).flags as ::core::ffi::c_uint
                    & G_FILE_MONITOR_WATCH_MOVES as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    G_FILE_MONITOR_EVENT_RENAMED as ::core::ffi::c_int
                } else {
                    G_FILE_MONITOR_EVENT_MOVED as ::core::ffi::c_int
                }) as GFileMonitorEvent;
                if !(*fms).dirname.is_null() {
                    dirname = (*fms).dirname;
                } else {
                    allocated_dirname = g_path_get_dirname((*fms).filename);
                    dirname = allocated_dirname;
                }
                other_file = g_local_file_new_from_dirname_and_basename(
                    dirname as *const ::core::ffi::c_char,
                    rename_to as *const ::core::ffi::c_char,
                );
                safe_c2rust_g_file_monitor_source_file_changes_done(fms, rename_to);
                safe_c2rust_g_file_monitor_source_send_event(fms, event, child, other_file);
                g_object_unref(other_file as gpointer);
                g_free(allocated_dirname as gpointer);
            } else {
                safe_c2rust_g_file_monitor_source_send_event(
                    fms,
                    G_FILE_MONITOR_EVENT_DELETED,
                    child,
                    ::core::ptr::null_mut::<GFile>(),
                );
                safe_c2rust_g_file_monitor_source_send_synthetic_created(fms, rename_to);
            }
        }
        2 | 4 | 5 | 6 => {
            if ({
                let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                if other.is_null() && rename_to.is_null() {
                    _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_18
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    458 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!other && !rename_to\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_g_file_monitor_source_send_event(
                fms,
                event_type,
                child,
                ::core::ptr::null_mut::<GFile>(),
            );
        }
        7 | _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                465 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    safe_c2rust_g_file_monitor_source_update_ready_time(fms);
    g_mutex_unlock(&raw mut (*fms).lock);
    return interesting;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_get_rate_limit(
    mut fms: *mut GFileMonitorSource,
) -> gint64 {
    let mut rate_limit: gint64 = 0;
    g_mutex_lock(&raw mut (*fms).lock);
    rate_limit = (*fms).rate_limit;
    g_mutex_unlock(&raw mut (*fms).lock);
    return rate_limit;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_set_rate_limit(
    mut fms: *mut GFileMonitorSource,
    mut rate_limit: gint64,
) -> gboolean {
    let mut changed: gboolean = 0;
    g_mutex_lock(&raw mut (*fms).lock);
    if rate_limit != (*fms).rate_limit {
        (*fms).rate_limit = rate_limit;
        g_sequence_sort(
            (*fms).pending_changes,
            Some(
                safe_c2rust_pending_change_compare_ready_time
                    as unsafe extern "C" fn(
                        gconstpointer,
                        gconstpointer,
                        gpointer,
                    ) -> ::core::ffi::c_int,
            ),
            fms as gpointer,
        );
        safe_c2rust_g_file_monitor_source_update_ready_time(fms);
        changed = TRUE as gboolean;
    } else {
        changed = FALSE as gboolean;
    }
    g_mutex_unlock(&raw mut (*fms).lock);
    return changed;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut fms: *mut GFileMonitorSource = source as *mut GFileMonitorSource;
    let mut event: *mut QueuedEvent = ::core::ptr::null_mut::<QueuedEvent>();
    let mut event_queue: GQueue = _GQueue {
        head: ::core::ptr::null_mut::<GList>(),
        tail: ::core::ptr::null_mut::<GList>(),
        length: 0,
    };
    let mut now: gint64 = 0;
    let mut instance: *mut GFileMonitor = ::core::ptr::null_mut::<GFileMonitor>();
    instance = g_weak_ref_get(&raw mut (*fms).instance_ref) as *mut GFileMonitor;
    if instance.is_null() {
        return FALSE;
    }
    now = g_source_get_time(source);
    g_mutex_lock(&raw mut (*fms).lock);
    while g_sequence_is_empty((*fms).pending_changes) == 0 {
        let mut iter: *mut GSequenceIter = g_sequence_get_begin_iter((*fms).pending_changes);
        let mut pending: *mut PendingChange = g_sequence_get(iter) as *mut PendingChange;
        if safe_c2rust_pending_change_get_ready_time(pending, fms) > now {
            break;
        }
        if (*pending).dirty() != 0 {
            safe_c2rust_g_file_monitor_source_queue_event(
                fms,
                G_FILE_MONITOR_EVENT_CHANGED,
                (*pending).child,
                ::core::ptr::null_mut::<GFile>(),
            );
            (*pending).set_last_emission(now as guint64 as guint64);
            (*pending).set_dirty(FALSE as guint64 as guint64);
            g_sequence_sort_changed(
                iter,
                Some(
                    safe_c2rust_pending_change_compare_ready_time
                        as unsafe extern "C" fn(
                            gconstpointer,
                            gconstpointer,
                            gpointer,
                        ) -> ::core::ffi::c_int,
                ),
                fms as gpointer,
            );
        } else {
            safe_c2rust_g_file_monitor_source_queue_event(
                fms,
                G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT,
                (*pending).child,
                ::core::ptr::null_mut::<GFile>(),
            );
            safe_c2rust_g_file_monitor_source_remove_pending_change(fms, iter, (*pending).child);
        }
    }
    memcpy(
        &raw mut event_queue as *mut ::core::ffi::c_void,
        &raw mut (*fms).event_queue as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GQueue>() as size_t,
    );
    memset(
        &raw mut (*fms).event_queue as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GQueue>() as size_t,
    );
    safe_c2rust_g_file_monitor_source_update_ready_time(fms);
    g_mutex_unlock(&raw mut (*fms).lock);
    let mut _pp: *mut *mut GFileMonitor = &raw mut instance;
    let mut _ptr: *mut GFileMonitor = *_pp;
    *_pp = ::core::ptr::null_mut::<GFileMonitor>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    loop {
        event = g_queue_pop_head(&raw mut event_queue) as *mut QueuedEvent;
        if event.is_null() {
            break;
        }
        instance = g_weak_ref_get(&raw mut (*fms).instance_ref) as *mut GFileMonitor;
        if !instance.is_null() {
            g_file_monitor_emit_event(
                instance,
                (*event).child,
                (*event).other,
                (*event).event_type,
            );
        }
        let mut _pp_0: *mut *mut GFileMonitor = &raw mut instance;
        let mut _ptr_0: *mut GFileMonitor = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<GFileMonitor>();
        if !_ptr_0.is_null() {
            g_object_unref(_ptr_0 as gpointer);
        }
        safe_c2rust_queued_event_free(event);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_dispose(mut fms: *mut GFileMonitorSource) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut seqiter: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut event: *mut QueuedEvent = ::core::ptr::null_mut::<QueuedEvent>();
    g_mutex_lock(&raw mut (*fms).lock);
    g_hash_table_iter_init(&raw mut iter, (*fms).pending_changes_table);
    while g_hash_table_iter_next(
        &raw mut iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut seqiter,
    ) != 0
    {
        g_hash_table_iter_remove(&raw mut iter);
        g_sequence_remove(seqiter as *mut GSequenceIter);
    }
    loop {
        event = g_queue_pop_head(&raw mut (*fms).event_queue) as *mut QueuedEvent;
        if event.is_null() {
            break;
        }
        safe_c2rust_queued_event_free(event);
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if g_sequence_is_empty((*fms).pending_changes) != 0 {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            610 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_sequence_is_empty (fms->pending_changes)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_hash_table_size((*fms).pending_changes_table) == 0 as guint {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            611 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_size (fms->pending_changes_table) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if (*fms).event_queue.length == 0 as guint {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            612 as ::core::ffi::c_int,
            G_STRFUNC,
            b"fms->event_queue.length == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_weak_ref_set(&raw mut (*fms).instance_ref, NULL_0);
    safe_c2rust_g_file_monitor_source_update_ready_time(fms);
    g_source_destroy(fms as *mut GSource);
    g_mutex_unlock(&raw mut (*fms).lock);
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_finalize(mut source: *mut GSource) {
    let mut fms: *mut GFileMonitorSource = source as *mut GFileMonitorSource;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if g_weak_ref_get(&raw mut (*fms).instance_ref).is_null() {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            628 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_weak_ref_get (&fms->instance_ref) == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    g_weak_ref_clear(&raw mut (*fms).instance_ref);
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if g_sequence_is_empty((*fms).pending_changes) != 0 {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            631 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_sequence_is_empty (fms->pending_changes)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if g_hash_table_size((*fms).pending_changes_table) == 0 as guint {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            632 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_size (fms->pending_changes_table) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if (*fms).event_queue.length == 0 as guint {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            633 as ::core::ffi::c_int,
            G_STRFUNC,
            b"fms->event_queue.length == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_unref((*fms).pending_changes_table);
    g_sequence_free((*fms).pending_changes);
    g_free((*fms).dirname as gpointer);
    g_free((*fms).basename as gpointer);
    g_free((*fms).filename as gpointer);
    g_mutex_clear(&raw mut (*fms).lock);
}
unsafe extern "C" fn safe_c2rust_str_hash0(mut str: gconstpointer) -> guint {
    return if !str.is_null() {
        g_str_hash(str)
    } else {
        0 as guint
    };
}
unsafe extern "C" fn safe_c2rust_str_equal0(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gboolean {
    return (g_strcmp0(
        a as *const ::core::ffi::c_char,
        b as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_file_monitor_source_new(
    mut instance: gpointer,
    mut filename: *const gchar,
    mut is_directory: gboolean,
    mut flags: GFileMonitorFlags,
) -> *mut GFileMonitorSource {
    static mut safe_c2rust_source_funcs: GSourceFuncs = unsafe {
        _GSourceFuncs {
            prepare: None,
            check: None,
            dispatch: Some(
                safe_c2rust_g_file_monitor_source_dispatch
                    as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
            ),
            finalize: Some(
                safe_c2rust_g_file_monitor_source_finalize
                    as unsafe extern "C" fn(*mut GSource) -> (),
            ),
            closure_callback: None,
            closure_marshal: None,
        }
    };
    let mut fms: *mut GFileMonitorSource = ::core::ptr::null_mut::<GFileMonitorSource>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    source = g_source_new(
        &raw mut safe_c2rust_source_funcs,
        ::core::mem::size_of::<GFileMonitorSource>() as guint,
    );
    fms = source as *mut GFileMonitorSource;
    g_source_set_static_name(
        source,
        b"GFileMonitorSource\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_mutex_init(&raw mut (*fms).lock);
    g_weak_ref_init(&raw mut (*fms).instance_ref, instance);
    (*fms).pending_changes = g_sequence_new(Some(
        safe_c2rust_pending_change_free as unsafe extern "C" fn(gpointer) -> (),
    ));
    (*fms).pending_changes_table = g_hash_table_new(
        Some(safe_c2rust_str_hash0 as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            safe_c2rust_str_equal0
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
    );
    (*fms).rate_limit = (800 as ::core::ffi::c_long * G_TIME_SPAN_MILLISECOND) as gint64;
    (*fms).flags = flags;
    if is_directory != 0 {
        (*fms).dirname =
            safe_c2rust_g_strdup_inline(filename as *const ::core::ffi::c_char) as *mut gchar;
        (*fms).basename = ::core::ptr::null_mut::<gchar>();
        (*fms).filename = ::core::ptr::null_mut::<gchar>();
    } else if flags as ::core::ffi::c_uint
        & G_FILE_MONITOR_WATCH_HARD_LINKS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        (*fms).dirname = ::core::ptr::null_mut::<gchar>();
        (*fms).basename = ::core::ptr::null_mut::<gchar>();
        (*fms).filename =
            safe_c2rust_g_strdup_inline(filename as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        (*fms).dirname = g_path_get_dirname(filename);
        (*fms).basename = g_path_get_basename(filename);
        (*fms).filename = ::core::ptr::null_mut::<gchar>();
    }
    return fms;
}
static mut safe_c2rust_g_local_file_monitor_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_monitor_get_type(),
        g_intern_static_string(b"GLocalFileMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GLocalFileMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GLocalFileMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GLocalFileMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_monitor_init
                    as unsafe extern "C" fn(*mut GLocalFileMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_local_file_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GLocalFileMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GLocalFileMonitor_private_offset,
        );
    }
    safe_c2rust_g_local_file_monitor_class_init(klass as *mut GLocalFileMonitorClass);
}
static mut safe_c2rust_GLocalFileMonitor_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_local_file_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_local_file_monitor_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut monitor: *mut GLocalFileMonitor =
        object as *mut ::core::ffi::c_void as *mut GLocalFileMonitor;
    let mut rate_limit: gint64 = 0;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if prop_id == PROP_RATE_LIMIT as ::core::ffi::c_int as guint {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            721 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id == PROP_RATE_LIMIT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    rate_limit = safe_c2rust_g_file_monitor_source_get_rate_limit((*monitor).source);
    rate_limit /= G_TIME_SPAN_MILLISECOND;
    g_value_set_int(value, rate_limit as gint);
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut monitor: *mut GLocalFileMonitor =
        object as *mut ::core::ffi::c_void as *mut GLocalFileMonitor;
    let mut rate_limit: gint64 = 0;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if prop_id == PROP_RATE_LIMIT as ::core::ffi::c_int as guint {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            736 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id == PROP_RATE_LIMIT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    rate_limit = g_value_get_int(value) as gint64;
    rate_limit *= G_TIME_SPAN_MILLISECOND;
    if safe_c2rust_g_file_monitor_source_set_rate_limit((*monitor).source, rate_limit) != 0 {
        g_object_notify(object, b"rate-limit\0" as *const u8 as *const gchar);
    }
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_mounts_changed(
    mut mount_monitor: *mut GUnixMountMonitor,
    mut user_data: gpointer,
) {
    let mut local_monitor: *mut GLocalFileMonitor = user_data as *mut GLocalFileMonitor;
    let mut mount: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    let mut is_mounted: gboolean = 0;
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    mount = g_unix_mount_at(
        (*(*local_monitor).source).dirname,
        ::core::ptr::null_mut::<guint64>(),
    );
    is_mounted = (mount != NULL_0 as *mut GUnixMountEntry) as ::core::ffi::c_int as gboolean;
    if !mount.is_null() {
        g_unix_mount_free(mount);
    }
    if (*local_monitor).was_mounted != is_mounted {
        if (*local_monitor).was_mounted != 0 && is_mounted == 0 {
            file = g_file_new_for_path((*(*local_monitor).source).dirname);
            g_file_monitor_emit_event(
                local_monitor as *mut ::core::ffi::c_void as *mut GFileMonitor,
                file,
                ::core::ptr::null_mut::<GFile>(),
                G_FILE_MONITOR_EVENT_UNMOUNTED,
            );
            g_object_unref(file as gpointer);
        }
        (*local_monitor).was_mounted = is_mounted;
    }
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_start(
    mut local_monitor: *mut GLocalFileMonitor,
    mut filename: *const gchar,
    mut is_directory: gboolean,
    mut flags: GFileMonitorFlags,
    mut context: *mut GMainContext,
) {
    let mut class: *mut GLocalFileMonitorClass =
        (*(local_monitor as *mut GTypeInstance)).g_class as *mut GLocalFileMonitorClass;
    let mut source: *mut GFileMonitorSource = ::core::ptr::null_mut::<GFileMonitorSource>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = local_monitor as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_local_file_monitor_get_type();
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
            b"G_IS_LOCAL_FILE_MONITOR (local_monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if (*local_monitor).source.is_null() {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfilemonitor.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            788 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!local_monitor->source\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    source = safe_c2rust_g_file_monitor_source_new(
        local_monitor as gpointer,
        filename,
        is_directory,
        flags,
    );
    (*local_monitor).source = source;
    if is_directory != 0
        && (*class).mount_notify == 0
        && flags as ::core::ffi::c_uint
            & G_FILE_MONITOR_WATCH_MOUNTS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        let mut mount: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
        mount = g_unix_mount_at(
            (*(*local_monitor).source).dirname,
            ::core::ptr::null_mut::<guint64>(),
        );
        (*local_monitor).was_mounted =
            (mount != NULL_0 as *mut GUnixMountEntry) as ::core::ffi::c_int as gboolean;
        if !mount.is_null() {
            g_unix_mount_free(mount);
        }
        (*local_monitor).mount_monitor = g_unix_mount_monitor_get();
        g_signal_connect_object(
            (*local_monitor).mount_monitor as gpointer,
            b"mounts-changed\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_g_local_file_monitor_mounts_changed
                    as unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> (),
            )),
            local_monitor as gpointer,
            G_CONNECT_DEFAULT,
        );
    }
    g_source_attach(source as *mut GSource, context);
    (*((*(local_monitor as *mut GTypeInstance)).g_class as *mut GLocalFileMonitorClass))
        .start
        .expect("non-null function pointer")(
        local_monitor,
        (*source).dirname,
        (*source).basename,
        (*source).filename,
        source,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_dispose(mut object: *mut GObject) {
    let mut local_monitor: *mut GLocalFileMonitor =
        object as *mut ::core::ffi::c_void as *mut GLocalFileMonitor;
    safe_c2rust_g_file_monitor_source_dispose((*local_monitor).source);
    (*(safe_c2rust_g_local_file_monitor_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_finalize(mut object: *mut GObject) {
    let mut local_monitor: *mut GLocalFileMonitor =
        object as *mut ::core::ffi::c_void as *mut GLocalFileMonitor;
    g_source_unref((*local_monitor).source as *mut GSource);
    (*(safe_c2rust_g_local_file_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_init(
    mut local_monitor: *mut GLocalFileMonitor,
) {
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_class_init(
    mut class: *mut GLocalFileMonitorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_local_file_monitor_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_local_file_monitor_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_local_file_monitor_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_local_file_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_override_property(
        gobject_class,
        PROP_RATE_LIMIT as ::core::ffi::c_int as guint,
        b"rate-limit\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_new(
    mut is_remote_fs: gboolean,
    mut is_directory: gboolean,
    mut error: *mut *mut GError,
) -> *mut GLocalFileMonitor {
    let mut type_0: GType = G_TYPE_INVALID;
    if is_remote_fs != 0 {
        type_0 = _g_io_module_get_default_type(
            G_NFS_FILE_MONITOR_EXTENSION_POINT_NAME.as_ptr() as *const gchar,
            b"GIO_USE_FILE_MONITOR\0" as *const u8 as *const gchar,
            192 as ::core::ffi::c_ulong as glong as guint,
        );
    }
    if type_0 == G_TYPE_INVALID && (is_remote_fs == 0 || is_directory != 0) {
        type_0 = _g_io_module_get_default_type(
            G_LOCAL_FILE_MONITOR_EXTENSION_POINT_NAME.as_ptr() as *const gchar,
            b"GIO_USE_FILE_MONITOR\0" as *const u8 as *const gchar,
            192 as ::core::ffi::c_ulong as glong as guint,
        );
    }
    if type_0 == G_TYPE_INVALID {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Unable to find default local file monitor type\0" as *const u8 as *const gchar,
            ),
        );
        return ::core::ptr::null_mut::<GLocalFileMonitor>();
    }
    return g_object_new(type_0, ::core::ptr::null::<gchar>()) as *mut GLocalFileMonitor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_local_file_monitor_new_for_path(
    mut pathname: *const gchar,
    mut is_directory: gboolean,
    mut flags: GFileMonitorFlags,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    let mut monitor: *mut GLocalFileMonitor = ::core::ptr::null_mut::<GLocalFileMonitor>();
    let mut is_remote_fs: gboolean = 0;
    is_remote_fs = g_local_file_is_nfs_home(pathname);
    monitor = safe_c2rust_g_local_file_monitor_new(is_remote_fs, is_directory, error);
    if !monitor.is_null() {
        safe_c2rust_g_local_file_monitor_start(
            monitor,
            pathname,
            is_directory,
            flags,
            g_main_context_get_thread_default(),
        );
    }
    return monitor as *mut ::core::ffi::c_void as *mut GFileMonitor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_local_file_monitor_new_in_worker(
    mut pathname: *const gchar,
    mut is_directory: gboolean,
    mut flags: GFileMonitorFlags,
    mut callback: GFileMonitorCallback,
    mut user_data: gpointer,
    mut destroy_user_data: GClosureNotify,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    let mut monitor: *mut GLocalFileMonitor = ::core::ptr::null_mut::<GLocalFileMonitor>();
    let mut is_remote_fs: gboolean = 0;
    is_remote_fs = g_local_file_is_nfs_home(pathname);
    monitor = safe_c2rust_g_local_file_monitor_new(is_remote_fs, is_directory, error);
    if !monitor.is_null() {
        if callback.is_some() {
            g_signal_connect_data(
                monitor as gpointer,
                b"changed\0" as *const u8 as *const gchar,
                ::core::mem::transmute::<GFileMonitorCallback, GCallback>(callback),
                user_data,
                destroy_user_data,
                G_CONNECT_DEFAULT,
            );
        }
        safe_c2rust_g_local_file_monitor_start(
            monitor,
            pathname,
            is_directory,
            flags,
            (*glib__private__())
                .g_get_worker_context
                .expect("non-null function pointer")(),
        );
    }
    return monitor as *mut ::core::ffi::c_void as *mut GFileMonitor;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
