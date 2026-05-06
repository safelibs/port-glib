extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GMemoryOutputStreamPrivate;
    pub type _GTask;
    pub type _GWakeup;
    pub type _GUnixOutputStreamPrivate;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_snprintf(string: *mut gchar, n: gulong, format: *const gchar, ...) -> gint;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_bytes_new(data: gconstpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_filename_display_name(filename: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_slist_free_1(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_main_context_new() -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_iteration(context: *mut GMainContext, may_block: gboolean) -> gboolean;
    fn g_main_context_push_thread_default(context: *mut GMainContext);
    fn g_main_context_pop_thread_default(context: *mut GMainContext);
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_child_watch_source_new(pid: GPid) -> *mut GSource;
    fn g_main_context_invoke_full(
        context: *mut GMainContext,
        priority: gint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_copy(block_size: gsize, mem_block: gconstpointer) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_spawn_async_with_pipes_and_fds(
        working_directory: *const gchar,
        argv: *const *const gchar,
        envp: *const *const gchar,
        flags: GSpawnFlags,
        child_setup: GSpawnChildSetupFunc,
        user_data: gpointer,
        stdin_fd: gint,
        stdout_fd: gint,
        stderr_fd: gint,
        source_fds: *const gint,
        target_fds: *const gint,
        n_fds: gsize,
        child_pid_out: *mut GPid,
        stdin_pipe_out: *mut gint,
        stdout_pipe_out: *mut gint,
        stderr_pipe_out: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_spawn_check_wait_status(wait_status: gint, error: *mut *mut GError) -> gboolean;
    fn g_spawn_close_pid(pid: GPid);
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_strv_get_type() -> GType;
    fn g_value_dup_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_signal_connect_object(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        gobject: gpointer,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_output_stream_write_all(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        bytes_written: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_close(
        stream: *mut GOutputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_splice_async(
        stream: *mut GOutputStream,
        source: *mut GInputStream,
        flags: GOutputStreamSpliceFlags,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_output_stream_splice_finish(
        stream: *mut GOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_cancellable_get_type() -> GType;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_source_new(cancellable: *mut GCancellable) -> *mut GSource;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_memory_input_stream_new_from_bytes(bytes: *mut GBytes) -> *mut GInputStream;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_subprocess_flags_get_type() -> GType;
    fn g_memory_output_stream_new_resizable() -> *mut GOutputStream;
    fn g_memory_output_stream_steal_data(ostream: *mut GMemoryOutputStream) -> gpointer;
    fn g_memory_output_stream_steal_as_bytes(ostream: *mut GMemoryOutputStream) -> *mut GBytes;
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
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_unix_output_stream_new(fd: gint, close_fd: gboolean) -> *mut GOutputStream;
    fn g_unix_output_stream_get_fd(stream: *mut GUnixOutputStream) -> gint;
    fn g_unix_input_stream_new(fd: gint, close_fd: gboolean) -> *mut GInputStream;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn g_unix_set_fd_nonblocking(fd: gint, nonblock: gboolean, error: *mut *mut GError)
        -> gboolean;
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
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type GPid = ::core::ffi::c_int;
pub type __pid_t = ::core::ffi::c_int;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBytes = _GBytes;
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
pub type GSpawnChildSetupFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GSpawnFlags = ::core::ffi::c_uint;
pub const G_SPAWN_STDIN_FROM_DEV_NULL: GSpawnFlags = 2048;
pub const G_SPAWN_CHILD_INHERITS_STDERR: GSpawnFlags = 1024;
pub const G_SPAWN_CHILD_INHERITS_STDOUT: GSpawnFlags = 512;
pub const G_SPAWN_CLOEXEC_PIPES: GSpawnFlags = 256;
pub const G_SPAWN_SEARCH_PATH_FROM_ENVP: GSpawnFlags = 128;
pub const G_SPAWN_FILE_AND_ARGV_ZERO: GSpawnFlags = 64;
pub const G_SPAWN_CHILD_INHERITS_STDIN: GSpawnFlags = 32;
pub const G_SPAWN_STDERR_TO_DEV_NULL: GSpawnFlags = 16;
pub const G_SPAWN_STDOUT_TO_DEV_NULL: GSpawnFlags = 8;
pub const G_SPAWN_SEARCH_PATH: GSpawnFlags = 4;
pub const G_SPAWN_DO_NOT_REAP_CHILD: GSpawnFlags = 2;
pub const G_SPAWN_LEAVE_DESCRIPTORS_OPEN: GSpawnFlags = 1;
pub const G_SPAWN_DEFAULT: GSpawnFlags = 0;
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
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GOutputStreamSpliceFlags = ::core::ffi::c_uint;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET: GOutputStreamSpliceFlags = 2;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE: GOutputStreamSpliceFlags = 1;
pub const G_OUTPUT_STREAM_SPLICE_NONE: GOutputStreamSpliceFlags = 0;
pub type GSubprocessFlags = ::core::ffi::c_uint;
pub const G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP: GSubprocessFlags = 256;
pub const G_SUBPROCESS_FLAGS_INHERIT_FDS: GSubprocessFlags = 128;
pub const G_SUBPROCESS_FLAGS_STDERR_MERGE: GSubprocessFlags = 64;
pub const G_SUBPROCESS_FLAGS_STDERR_SILENCE: GSubprocessFlags = 32;
pub const G_SUBPROCESS_FLAGS_STDERR_PIPE: GSubprocessFlags = 16;
pub const G_SUBPROCESS_FLAGS_STDOUT_SILENCE: GSubprocessFlags = 8;
pub const G_SUBPROCESS_FLAGS_STDOUT_PIPE: GSubprocessFlags = 4;
pub const G_SUBPROCESS_FLAGS_STDIN_INHERIT: GSubprocessFlags = 2;
pub const G_SUBPROCESS_FLAGS_STDIN_PIPE: GSubprocessFlags = 1;
pub const G_SUBPROCESS_FLAGS_NONE: GSubprocessFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GMemoryOutputStreamPrivate,
}
pub type GMemoryOutputStreamPrivate = _GMemoryOutputStreamPrivate;
pub type GMemoryOutputStream = _GMemoryOutputStream;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSubprocess {
    pub parent: GObject,
    pub launcher: *mut GSubprocessLauncher,
    pub flags: GSubprocessFlags,
    pub argv: *mut *mut gchar,
    pub identifier: [gchar; 24],
    pub status: ::core::ffi::c_int,
    pub pid: GPid,
    pub pending_waits_lock: GMutex,
    pub pending_waits: *mut GSList,
    pub stdin_pipe: *mut GOutputStream,
    pub stdout_pipe: *mut GInputStream,
    pub stderr_pipe: *mut GInputStream,
}
pub type GSubprocessLauncher = _GSubprocessLauncher;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSubprocessLauncher {
    pub parent: GObject,
    pub flags: GSubprocessFlags,
    pub envp: *mut *mut ::core::ffi::c_char,
    pub cwd: *mut ::core::ffi::c_char,
    pub stdin_fd: gint,
    pub stdin_path: *mut gchar,
    pub stdout_fd: gint,
    pub stdout_path: *mut gchar,
    pub stderr_fd: gint,
    pub stderr_path: *mut gchar,
    pub source_fds: *mut GArray,
    pub target_fds: *mut GArray,
    pub closed_fd: gboolean,
    pub child_setup_func: GSpawnChildSetupFunc,
    pub child_setup_user_data: gpointer,
    pub child_setup_destroy_notify: GDestroyNotify,
}
pub type GSubprocess = _GSubprocess;
pub type GSubprocessClass = GObjectClass;
pub const PROP_ARGV: C2RustUnnamed_0 = 2;
pub const PROP_FLAGS: C2RustUnnamed_0 = 1;
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalRecord {
    pub subprocess: *mut GSubprocess,
    pub signalnum: gint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommunicateState {
    pub stdin_data: *const gchar,
    pub stdin_length: gsize,
    pub stdin_offset: gsize,
    pub add_nul: gboolean,
    pub stdin_buf: *mut GInputStream,
    pub stdout_buf: *mut GMemoryOutputStream,
    pub stderr_buf: *mut GMemoryOutputStream,
    pub cancellable: *mut GCancellable,
    pub cancellable_source: *mut GSource,
    pub outstanding_ops: guint,
    pub reported_error: gboolean,
}
pub type GUnixOutputStream = _GUnixOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GUnixOutputStreamPrivate,
}
pub type GUnixOutputStreamPrivate = _GUnixOutputStreamPrivate;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const N_PROPS: C2RustUnnamed_0 = 3;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_PRIORITY_HIGH_IDLE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_subprocess_get_type_once();
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
static mut safe_c2rust_g_subprocess_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GSubprocess_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_subprocess_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_subprocess_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSubprocess_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GSubprocess_private_offset);
    }
    safe_c2rust_g_subprocess_class_init(klass as *mut GSubprocessClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_subprocess_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSubprocess\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSubprocessClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_subprocess_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSubprocess>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSubprocess) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_subprocess_init as unsafe extern "C" fn(*mut GSubprocess) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_initable_iface_init as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_platform_input_stream_from_spawn_fd(
    mut fd: gint,
) -> *mut GInputStream {
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GInputStream>();
    }
    return g_unix_input_stream_new(fd, TRUE);
}
unsafe extern "C" fn safe_c2rust_platform_output_stream_from_spawn_fd(
    mut fd: gint,
) -> *mut GOutputStream {
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GOutputStream>();
    }
    return g_unix_output_stream_new(fd, TRUE);
}
unsafe extern "C" fn safe_c2rust_unix_open_file(
    mut filename: *const ::core::ffi::c_char,
    mut mode: gint,
    mut error: *mut *mut GError,
) -> gint {
    let mut my_fd: gint = 0;
    my_fd = open(
        filename,
        mode as ::core::ffi::c_int | O_BINARY | O_CLOEXEC,
        0o666 as ::core::ffi::c_int,
    ) as gint;
    if my_fd < 0 as ::core::ffi::c_int {
        let mut saved_errno: gint = *__errno_location();
        let mut display_name: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        display_name =
            g_filename_display_name(filename as *const gchar) as *mut ::core::ffi::c_char;
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(saved_errno) as gint,
            glib_gettext(
                b"Error opening file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8 as *const gchar,
            ),
            display_name,
            g_strerror(saved_errno),
        );
        g_free(display_name as gpointer);
    }
    return my_fd;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GSubprocess = object as *mut ::core::ffi::c_void as *mut GSubprocess;
    match prop_id {
        1 => {
            (*self_0).flags = g_value_get_flags(value) as GSubprocessFlags;
        }
        2 => {
            (*self_0).argv = g_value_dup_boxed(value) as *mut *mut gchar;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_subprocess_exited(
    mut pid: GPid,
    mut status: gint,
    mut user_data: gpointer,
) -> gboolean {
    let mut self_0: *mut GSubprocess = user_data as *mut GSubprocess;
    let mut tasks: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*self_0).pid == pid {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            272 as ::core::ffi::c_int,
            G_STRFUNC,
            b"self->pid == pid\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*self_0).pending_waits_lock);
    (*self_0).status = status as ::core::ffi::c_int;
    tasks = (*self_0).pending_waits;
    (*self_0).pending_waits = ::core::ptr::null_mut::<GSList>();
    (*self_0).pid = 0 as ::core::ffi::c_int as GPid;
    g_mutex_unlock(&raw mut (*self_0).pending_waits_lock);
    while !tasks.is_null() {
        g_task_return_boolean((*tasks).data as *mut GTask, TRUE);
        g_object_unref((*tasks).data);
        tasks = g_slist_delete_link(tasks, tasks);
    }
    g_spawn_close_pid(pid);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut self_0: *mut GSubprocess = initable as *mut ::core::ffi::c_void as *mut GSubprocess;
    let mut pipe_ptrs: [*mut gint; 3] = [
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
    ];
    let mut pipe_fds: [gint; 3] = [
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
    ];
    let mut close_fds: [gint; 3] = [
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
    ];
    let mut stdin_fd: gint = -(1 as gint);
    let mut stdout_fd: gint = -(1 as gint);
    let mut stderr_fd: gint = -(1 as gint);
    let mut spawn_flags: GSpawnFlags = G_SPAWN_DEFAULT;
    let mut success: gboolean = FALSE;
    let mut i: gint = 0;
    if (*self_0).argv.is_null()
        || (*(*self_0).argv.offset(0 as ::core::ffi::c_int as isize)).is_null()
        || *(*(*self_0).argv.offset(0 as ::core::ffi::c_int as isize))
            .offset(0 as ::core::ffi::c_int as isize)
            == 0
    {
        return FALSE;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    if (*self_0).flags as ::core::ffi::c_uint
        & G_SUBPROCESS_FLAGS_STDIN_INHERIT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        spawn_flags = ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
            spawn_flags as ::core::ffi::c_uint
                | G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        current_block = 17860125682698302841;
    } else if (*self_0).flags as ::core::ffi::c_uint
        & G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pipe_ptrs[0 as ::core::ffi::c_int as usize] =
            (&raw mut pipe_fds as *mut gint).offset(0 as ::core::ffi::c_int as isize) as *mut gint;
        current_block = 17860125682698302841;
    } else if !(*self_0).launcher.is_null() {
        if (*(*self_0).launcher).stdin_fd != -(1 as ::core::ffi::c_int) {
            stdin_fd = (*(*self_0).launcher).stdin_fd;
            current_block = 17860125682698302841;
        } else if !(*(*self_0).launcher).stdin_path.is_null() {
            close_fds[0 as ::core::ffi::c_int as usize] =
                safe_c2rust_unix_open_file((*(*self_0).launcher).stdin_path, O_RDONLY, error);
            stdin_fd = close_fds[0 as ::core::ffi::c_int as usize];
            if stdin_fd == -(1 as ::core::ffi::c_int) {
                current_block = 15070319019838248776;
            } else {
                current_block = 17860125682698302841;
            }
        } else {
            current_block = 17860125682698302841;
        }
    } else {
        current_block = 17860125682698302841;
    }
    match current_block {
        17860125682698302841 => {
            if (*self_0).flags as ::core::ffi::c_uint
                & G_SUBPROCESS_FLAGS_STDOUT_SILENCE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                spawn_flags = ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
                    spawn_flags as ::core::ffi::c_uint
                        | G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
                current_block = 10043043949733653460;
            } else if (*self_0).flags as ::core::ffi::c_uint
                & G_SUBPROCESS_FLAGS_STDOUT_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                pipe_ptrs[1 as ::core::ffi::c_int as usize] = (&raw mut pipe_fds as *mut gint)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut gint;
                current_block = 10043043949733653460;
            } else if !(*self_0).launcher.is_null() {
                if (*(*self_0).launcher).stdout_fd != -(1 as ::core::ffi::c_int) {
                    stdout_fd = (*(*self_0).launcher).stdout_fd;
                    current_block = 10043043949733653460;
                } else if !(*(*self_0).launcher).stdout_path.is_null() {
                    close_fds[1 as ::core::ffi::c_int as usize] = safe_c2rust_unix_open_file(
                        (*(*self_0).launcher).stdout_path,
                        O_CREAT | O_WRONLY,
                        error,
                    );
                    stdout_fd = close_fds[1 as ::core::ffi::c_int as usize];
                    if stdout_fd == -(1 as ::core::ffi::c_int) {
                        current_block = 15070319019838248776;
                    } else {
                        current_block = 10043043949733653460;
                    }
                } else {
                    current_block = 10043043949733653460;
                }
            } else {
                current_block = 10043043949733653460;
            }
            match current_block {
                15070319019838248776 => {}
                _ => {
                    if (*self_0).flags as ::core::ffi::c_uint
                        & G_SUBPROCESS_FLAGS_STDERR_SILENCE as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                    {
                        spawn_flags = ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
                            spawn_flags as ::core::ffi::c_uint
                                | G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int
                                    as ::core::ffi::c_uint,
                        );
                        current_block = 7333393191927787629;
                    } else if (*self_0).flags as ::core::ffi::c_uint
                        & G_SUBPROCESS_FLAGS_STDERR_PIPE as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                    {
                        pipe_ptrs[2 as ::core::ffi::c_int as usize] = (&raw mut pipe_fds
                            as *mut gint)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut gint;
                        current_block = 7333393191927787629;
                    } else if (*self_0).flags as ::core::ffi::c_uint
                        & G_SUBPROCESS_FLAGS_STDERR_MERGE as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                    {
                        stderr_fd = 1 as ::core::ffi::c_int as gint;
                        current_block = 7333393191927787629;
                    } else if !(*self_0).launcher.is_null() {
                        if (*(*self_0).launcher).stderr_fd != -(1 as ::core::ffi::c_int) {
                            stderr_fd = (*(*self_0).launcher).stderr_fd;
                            current_block = 7333393191927787629;
                        } else if !(*(*self_0).launcher).stderr_path.is_null() {
                            close_fds[2 as ::core::ffi::c_int as usize] =
                                safe_c2rust_unix_open_file(
                                    (*(*self_0).launcher).stderr_path,
                                    O_CREAT | O_WRONLY,
                                    error,
                                );
                            stderr_fd = close_fds[2 as ::core::ffi::c_int as usize];
                            if stderr_fd == -(1 as ::core::ffi::c_int) {
                                current_block = 15070319019838248776;
                            } else {
                                current_block = 7333393191927787629;
                            }
                        } else {
                            current_block = 7333393191927787629;
                        }
                    } else {
                        current_block = 7333393191927787629;
                    }
                    match current_block {
                        15070319019838248776 => {}
                        _ => {
                            if strchr(
                                *(*self_0).argv.offset(0 as ::core::ffi::c_int as isize),
                                G_DIR_SEPARATOR,
                            )
                            .is_null()
                            {
                                if !(*self_0).launcher.is_null()
                                    && (*(*self_0).launcher).flags as ::core::ffi::c_uint
                                        & G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                        != 0
                                {
                                    spawn_flags =
                                        ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
                                            spawn_flags as ::core::ffi::c_uint
                                                | G_SPAWN_SEARCH_PATH_FROM_ENVP
                                                    as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint,
                                        );
                                } else {
                                    spawn_flags =
                                        ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
                                            spawn_flags as ::core::ffi::c_uint
                                                | G_SPAWN_SEARCH_PATH as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint,
                                        );
                                }
                            }
                            if (*self_0).flags as ::core::ffi::c_uint
                                & G_SUBPROCESS_FLAGS_INHERIT_FDS as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                != 0
                            {
                                spawn_flags =
                                    ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
                                        spawn_flags as ::core::ffi::c_uint
                                            | G_SPAWN_LEAVE_DESCRIPTORS_OPEN as ::core::ffi::c_int
                                                as ::core::ffi::c_uint,
                                    );
                            }
                            spawn_flags = ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
                                spawn_flags as ::core::ffi::c_uint
                                    | G_SPAWN_DO_NOT_REAP_CHILD as ::core::ffi::c_int
                                        as ::core::ffi::c_uint,
                            );
                            spawn_flags = ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
                                spawn_flags as ::core::ffi::c_uint
                                    | G_SPAWN_CLOEXEC_PIPES as ::core::ffi::c_int
                                        as ::core::ffi::c_uint,
                            );
                            success = g_spawn_async_with_pipes_and_fds(
                                if !(*self_0).launcher.is_null() {
                                    (*(*self_0).launcher).cwd
                                } else {
                                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                                },
                                (*self_0).argv as *const *const gchar,
                                (if !(*self_0).launcher.is_null() {
                                    (*(*self_0).launcher).envp
                                } else {
                                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>()
                                }) as *const *const gchar,
                                spawn_flags,
                                if !(*self_0).launcher.is_null() {
                                    (*(*self_0).launcher).child_setup_func
                                } else {
                                    None
                                },
                                if !(*self_0).launcher.is_null() {
                                    (*(*self_0).launcher).child_setup_user_data
                                } else {
                                    NULL_0
                                },
                                stdin_fd,
                                stdout_fd,
                                stderr_fd,
                                if !(*self_0).launcher.is_null() {
                                    (*(*(*self_0).launcher).source_fds).data as *const gint
                                } else {
                                    ::core::ptr::null::<gint>()
                                },
                                if !(*self_0).launcher.is_null() {
                                    (*(*(*self_0).launcher).target_fds).data as *const gint
                                } else {
                                    ::core::ptr::null::<gint>()
                                },
                                (if !(*self_0).launcher.is_null() {
                                    (*(*(*self_0).launcher).source_fds).len
                                } else {
                                    0 as guint
                                }) as gsize,
                                &raw mut (*self_0).pid,
                                pipe_ptrs[0 as ::core::ffi::c_int as usize],
                                pipe_ptrs[1 as ::core::ffi::c_int as usize],
                                pipe_ptrs[2 as ::core::ffi::c_int as usize],
                                error,
                            );
                            if ({
                                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                                if success
                                    == ((*self_0).pid != 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
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
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    415 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"success == (self->pid != 0)\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            let mut identifier: guint64 = 0;
                            let mut s: gint = 0;
                            identifier = (*self_0).pid as guint64;
                            s = g_snprintf(
                                &raw mut (*self_0).identifier as *mut gchar,
                                ::core::mem::size_of::<[gchar; 24]>() as gulong,
                                b"%lu\0" as *const u8 as *const gchar,
                                identifier,
                            );
                            if ({
                                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                                if (0 as ::core::ffi::c_int) < s
                                    && (s as usize) < ::core::mem::size_of::<[gchar; 24]>() as usize
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
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    428 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"0 < s && (gsize) s < sizeof self->identifier\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                            if success != 0 {
                                let mut worker_context: *mut GMainContext =
                                    ::core::ptr::null_mut::<GMainContext>();
                                let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
                                worker_context = (*glib__private__())
                                    .g_get_worker_context
                                    .expect("non-null function pointer")(
                                );
                                source = g_child_watch_source_new((*self_0).pid);
                                g_source_set_callback(
                                    source,
                                    ::core::mem::transmute::<
                                        Option<
                                            unsafe extern "C" fn(GPid, gint, gpointer) -> gboolean,
                                        >,
                                        GSourceFunc,
                                    >(Some(
                                        safe_c2rust_g_subprocess_exited
                                            as unsafe extern "C" fn(
                                                GPid,
                                                gint,
                                                gpointer,
                                            )
                                                -> gboolean,
                                    )),
                                    g_object_ref(self_0 as gpointer) as *mut GSubprocess
                                        as gpointer,
                                    Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
                                );
                                g_source_attach(source, worker_context);
                                g_source_unref(source);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    (*self_0).launcher = ::core::ptr::null_mut::<GSubprocessLauncher>();
    i = 0 as ::core::ffi::c_int as gint;
    while i < 3 as ::core::ffi::c_int {
        if close_fds[i as usize] != -(1 as ::core::ffi::c_int) {
            close(close_fds[i as usize]);
        }
        i += 1;
    }
    (*self_0).stdin_pipe = safe_c2rust_platform_output_stream_from_spawn_fd(
        pipe_fds[0 as ::core::ffi::c_int as usize],
    );
    (*self_0).stdout_pipe =
        safe_c2rust_platform_input_stream_from_spawn_fd(pipe_fds[1 as ::core::ffi::c_int as usize]);
    (*self_0).stderr_pipe =
        safe_c2rust_platform_input_stream_from_spawn_fd(pipe_fds[2 as ::core::ffi::c_int as usize]);
    return success;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_finalize(mut object: *mut GObject) {
    let mut self_0: *mut GSubprocess = object as *mut ::core::ffi::c_void as *mut GSubprocess;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*self_0).pending_waits.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            466 as ::core::ffi::c_int,
            G_STRFUNC,
            b"self->pending_waits == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*self_0).pid == 0 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            467 as ::core::ffi::c_int,
            G_STRFUNC,
            b"self->pid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _pp: *mut *mut GOutputStream = &raw mut (*self_0).stdin_pipe;
    let mut _ptr: *mut GOutputStream = *_pp;
    *_pp = ::core::ptr::null_mut::<GOutputStream>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GInputStream = &raw mut (*self_0).stdout_pipe;
    let mut _ptr_0: *mut GInputStream = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GInputStream>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut GInputStream = &raw mut (*self_0).stderr_pipe;
    let mut _ptr_1: *mut GInputStream = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GInputStream>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    g_strfreev((*self_0).argv);
    g_mutex_clear(&raw mut (*self_0).pending_waits_lock);
    (*(safe_c2rust_g_subprocess_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_subprocess_init(mut self_0: *mut GSubprocess) {
    g_mutex_init(&raw mut (*self_0).pending_waits_lock);
}
unsafe extern "C" fn safe_c2rust_initable_iface_init(mut initable_iface: *mut GInitableIface) {
    (*initable_iface).init = Some(
        safe_c2rust_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_class_init(mut class: *mut GSubprocessClass) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_subprocess_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_subprocess_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_subprocess_flags_get_type(),
            0 as guint,
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_ARGV as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"argv\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_strv_get_type(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_new(
    mut flags: GSubprocessFlags,
    mut error: *mut *mut GError,
    mut argv0: *const gchar,
    mut args: ...
) -> *mut GSubprocess {
    let mut result: *mut GSubprocess = ::core::ptr::null_mut::<GSubprocess>();
    let mut args_0: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut arg: *const gchar = ::core::ptr::null::<gchar>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !argv0.is_null()
            && *argv0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
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
            b"argv0 != NULL && argv0[0] != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSubprocess>();
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
        return ::core::ptr::null_mut::<GSubprocess>();
    }
    args_0 = g_ptr_array_new();
    ap = args.clone();
    g_ptr_array_add(args_0, argv0 as *mut gchar as gpointer);
    loop {
        arg = ap.arg::<*const gchar>();
        if arg.is_null() {
            break;
        }
        g_ptr_array_add(args_0, arg as *mut gchar as gpointer);
    }
    g_ptr_array_add(args_0, NULL_0);
    result = safe_c2rust_g_subprocess_newv((*args_0).pdata as *const *const gchar, flags, error);
    g_ptr_array_free(args_0, TRUE);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_newv(
    mut argv: *const *const gchar,
    mut flags: GSubprocessFlags,
    mut error: *mut *mut GError,
) -> *mut GSubprocess {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !argv.is_null()
            && !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null()
            && *(*argv.offset(0 as ::core::ffi::c_int as isize))
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32
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
            b"argv != NULL && argv[0] != NULL && argv[0][0] != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSubprocess>();
    }
    return g_initable_new(
        safe_c2rust_g_subprocess_get_type(),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
        b"argv\0" as *const u8 as *const gchar,
        argv,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        NULL_0,
    ) as *mut GSubprocess;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_identifier(
    mut subprocess: *mut GSubprocess,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if (*subprocess).pid != 0 {
        return &raw mut (*subprocess).identifier as *mut gchar;
    } else {
        return ::core::ptr::null::<gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_stdin_pipe(
    mut subprocess: *mut GSubprocess,
) -> *mut GOutputStream {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GOutputStream>();
    }
    return (*subprocess).stdin_pipe;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_stdout_pipe(
    mut subprocess: *mut GSubprocess,
) -> *mut GInputStream {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    return (*subprocess).stdout_pipe;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_stderr_pipe(
    mut subprocess: *mut GSubprocess,
) -> *mut GInputStream {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    return (*subprocess).stderr_pipe;
}
unsafe extern "C" fn safe_c2rust_slist_remove_if_present(
    mut list: *mut *mut GSList,
    mut data: gconstpointer,
) -> gboolean {
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut prev: *mut GSList = ::core::ptr::null_mut::<GSList>();
    l = *list;
    prev = ::core::ptr::null_mut::<GSList>();
    while !l.is_null() {
        if (*l).data == data as gpointer {
            if !prev.is_null() {
                (*prev).next = (*l).next;
            } else {
                *list = (*l).next;
            }
            g_slist_free_1(l);
            return TRUE;
        }
        prev = l;
        l = (*prev).next;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_wait_cancelled(
    mut cancellable: *mut GCancellable,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut self_0: *mut GSubprocess = ::core::ptr::null_mut::<GSubprocess>();
    let mut task_was_pending: gboolean = 0;
    self_0 = g_task_get_source_object(task) as *mut GSubprocess;
    g_mutex_lock(&raw mut (*self_0).pending_waits_lock);
    task_was_pending = safe_c2rust_slist_remove_if_present(
        &raw mut (*self_0).pending_waits,
        task as gconstpointer,
    );
    g_mutex_unlock(&raw mut (*self_0).pending_waits_lock);
    if task_was_pending != 0 {
        g_task_return_boolean(task, FALSE);
        g_object_unref(task as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_wait_async(
    mut subprocess: *mut GSubprocess,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(subprocess as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSubprocess,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_subprocess_wait_async
                as unsafe extern "C" fn(
                    *mut GSubprocess,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_subprocess_wait_async\0" as *const u8 as *const gchar,
        );
    }
    g_mutex_lock(&raw mut (*subprocess).pending_waits_lock);
    if (*subprocess).pid != 0 {
        if !cancellable.is_null() {
            g_signal_connect_object(
                cancellable as gpointer,
                b"cancelled\0" as *const u8 as *const gchar,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
                    GCallback,
                >(Some(
                    safe_c2rust_g_subprocess_wait_cancelled
                        as unsafe extern "C" fn(*mut GCancellable, gpointer) -> (),
                )),
                task as gpointer,
                G_CONNECT_DEFAULT,
            );
        }
        (*subprocess).pending_waits =
            g_slist_prepend((*subprocess).pending_waits, task as gpointer);
        task = ::core::ptr::null_mut::<GTask>();
    }
    g_mutex_unlock(&raw mut (*subprocess).pending_waits_lock);
    if !task.is_null() {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_wait_finish(
    mut subprocess: *mut GSubprocess,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_subprocess_sync_setup() {
    g_main_context_push_thread_default(g_main_context_new());
}
unsafe extern "C" fn safe_c2rust_g_subprocess_sync_done(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut result_ptr: *mut *mut GAsyncResult = user_data as *mut *mut GAsyncResult;
    *result_ptr = g_object_ref(result as gpointer) as *mut GAsyncResult as *mut GAsyncResult;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_sync_complete(mut result: *mut *mut GAsyncResult) {
    let mut context: *mut GMainContext = g_main_context_get_thread_default();
    while (*result).is_null() {
        g_main_context_iteration(context, TRUE);
    }
    g_main_context_pop_thread_default(context);
    g_main_context_unref(context);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_wait(
    mut subprocess: *mut GSubprocess,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut result: *mut GAsyncResult = ::core::ptr::null_mut::<GAsyncResult>();
    let mut success: gboolean = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    if (*subprocess).pid == 0 as ::core::ffi::c_int {
        return TRUE;
    }
    safe_c2rust_g_subprocess_sync_setup();
    safe_c2rust_g_subprocess_wait_async(
        subprocess,
        cancellable,
        Some(
            safe_c2rust_g_subprocess_sync_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &raw mut result as gpointer,
    );
    safe_c2rust_g_subprocess_sync_complete(&raw mut result);
    success = safe_c2rust_g_subprocess_wait_finish(subprocess, result, error);
    g_object_unref(result as gpointer);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_wait_check(
    mut subprocess: *mut GSubprocess,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return (safe_c2rust_g_subprocess_wait(subprocess, cancellable, error) != 0
        && g_spawn_check_wait_status((*subprocess).status as gint, error) != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_wait_check_async(
    mut subprocess: *mut GSubprocess,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_g_subprocess_wait_async(subprocess, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_wait_check_finish(
    mut subprocess: *mut GSubprocess,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return (safe_c2rust_g_subprocess_wait_finish(subprocess, result, error) != 0
        && g_spawn_check_wait_status((*subprocess).status as gint, error) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_actually_send_signal(
    mut user_data: gpointer,
) -> gboolean {
    let mut signal_record: *mut SignalRecord = user_data as *mut SignalRecord;
    if (*(*signal_record).subprocess).pid != 0 {
        kill(
            (*(*signal_record).subprocess).pid as __pid_t,
            (*signal_record).signalnum as ::core::ffi::c_int,
        );
    }
    g_object_unref((*signal_record).subprocess as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<SignalRecord>() as gsize,
        signal_record as gpointer,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_dispatch_signal(
    mut subprocess: *mut GSubprocess,
    mut signalnum: gint,
) {
    let mut signal_record: SignalRecord = SignalRecord {
        subprocess: g_object_ref(subprocess as gpointer) as *mut GSubprocess,
        signalnum: signalnum,
    };
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_main_context_invoke_full(
        (*glib__private__())
            .g_get_worker_context
            .expect("non-null function pointer")(),
        G_PRIORITY_HIGH_IDLE,
        Some(
            safe_c2rust_g_subprocess_actually_send_signal
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        (if 1 as ::core::ffi::c_int != 0 {
            g_slice_copy(
                ::core::mem::size_of::<SignalRecord>() as gsize,
                &raw mut signal_record as gconstpointer,
            ) as *mut SignalRecord
        } else {
            &raw mut signal_record;
            ::core::ptr::null_mut::<SignalRecord>()
        }) as gpointer,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_send_signal(
    mut subprocess: *mut GSubprocess,
    mut signal_num: gint,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_subprocess_dispatch_signal(subprocess, signal_num);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_force_exit(mut subprocess: *mut GSubprocess) {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_subprocess_dispatch_signal(subprocess, SIGKILL);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_status(
    mut subprocess: *mut GSubprocess,
) -> gint {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if (*subprocess).pid == 0 as ::core::ffi::c_int {
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
            b"subprocess->pid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*subprocess).status as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_successful(
    mut subprocess: *mut GSubprocess,
) -> gboolean {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if (*subprocess).pid == 0 as ::core::ffi::c_int {
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
            b"subprocess->pid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*subprocess).status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && ((*subprocess).status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_if_exited(
    mut subprocess: *mut GSubprocess,
) -> gboolean {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*subprocess).pid == 0 as ::core::ffi::c_int {
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
            b"subprocess->pid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*subprocess).status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_exit_status(
    mut subprocess: *mut GSubprocess,
) -> gint {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 1 as gint;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if (*subprocess).pid == 0 as ::core::ffi::c_int {
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
            b"subprocess->pid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 1 as gint;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*subprocess).status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
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
            b"WIFEXITED (subprocess->status)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 1 as gint;
    }
    return ((*subprocess).status as gint & 0xff00 as gint) >> 8 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_if_signaled(
    mut subprocess: *mut GSubprocess,
) -> gboolean {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if (*subprocess).pid == 0 as ::core::ffi::c_int {
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
            b"subprocess->pid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((((*subprocess).status & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
        as ::core::ffi::c_schar as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_get_term_sig(
    mut subprocess: *mut GSubprocess,
) -> gint {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if (*subprocess).pid == 0 as ::core::ffi::c_int {
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
            b"subprocess->pid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if (((*subprocess).status & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_schar as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
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
            b"WIFSIGNALED (subprocess->status)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*subprocess).status as gint & 0x7f as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_set_launcher(
    mut subprocess: *mut GSubprocess,
    mut launcher: *mut GSubprocessLauncher,
) {
    (*subprocess).launcher = launcher;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_made_progress(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut current_block: u64;
    let mut state: *mut CommunicateState = ::core::ptr::null_mut::<CommunicateState>();
    let mut subprocess: *mut GSubprocess = ::core::ptr::null_mut::<GSubprocess>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut source: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !source_object.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1324 as ::core::ffi::c_int,
            G_STRFUNC,
            b"source_object != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    task = user_data as *mut GTask;
    subprocess = g_task_get_source_object(task) as *mut GSubprocess;
    state = g_task_get_task_data(task) as *mut CommunicateState;
    source = source_object as gpointer;
    (*state).outstanding_ops = (*state).outstanding_ops.wrapping_sub(1);
    if source == (*subprocess).stdin_pipe as gpointer
        || source == (*state).stdout_buf as gpointer
        || source == (*state).stderr_buf as gpointer
    {
        if !(g_output_stream_splice_finish(source as *mut GOutputStream, result, &raw mut error)
            == -(1 as ::core::ffi::c_int) as gssize)
        {
            if source == (*state).stdout_buf as gpointer
                || source == (*state).stderr_buf as gpointer
            {
                if (*state).add_nul != 0 {
                    let mut bytes_written: gsize = 0;
                    if g_output_stream_write_all(
                        source as *mut GOutputStream,
                        b"\0\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        1 as gsize,
                        &raw mut bytes_written,
                        ::core::ptr::null_mut::<GCancellable>(),
                        &raw mut error,
                    ) == 0
                    {
                        current_block = 9412839717817013801;
                    } else {
                        current_block = 12599329904712511516;
                    }
                } else {
                    current_block = 12599329904712511516;
                }
                match current_block {
                    9412839717817013801 => {}
                    _ => {
                        g_output_stream_close(
                            source as *mut GOutputStream,
                            ::core::ptr::null_mut::<GCancellable>(),
                            &raw mut error,
                        ) == 0;
                    }
                }
            }
        }
    } else if source == subprocess as gpointer {
        safe_c2rust_g_subprocess_wait_finish(subprocess, result, &raw mut error);
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1362 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if !error.is_null() {
        if (*state).reported_error == 0 {
            (*state).reported_error = TRUE as gboolean;
            g_cancellable_cancel((*state).cancellable);
            g_task_return_error(task, error);
        } else {
            g_error_free(error);
        }
    } else if (*state).outstanding_ops == 0 as guint {
        g_task_return_boolean(task, TRUE);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_cancelled(
    mut cancellable: *mut GCancellable,
    mut user_data: gpointer,
) -> gboolean {
    let mut state: *mut CommunicateState = user_data as *mut CommunicateState;
    g_cancellable_cancel((*state).cancellable);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_state_free(mut data: gpointer) {
    let mut state: *mut CommunicateState = data as *mut CommunicateState;
    let mut _pp: *mut *mut GCancellable = &raw mut (*state).cancellable;
    let mut _ptr: *mut GCancellable = *_pp;
    *_pp = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GInputStream = &raw mut (*state).stdin_buf;
    let mut _ptr_0: *mut GInputStream = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GInputStream>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut GMemoryOutputStream = &raw mut (*state).stdout_buf;
    let mut _ptr_1: *mut GMemoryOutputStream = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GMemoryOutputStream>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    let mut _pp_2: *mut *mut GMemoryOutputStream = &raw mut (*state).stderr_buf;
    let mut _ptr_2: *mut GMemoryOutputStream = *_pp_2;
    *_pp_2 = ::core::ptr::null_mut::<GMemoryOutputStream>();
    if !_ptr_2.is_null() {
        g_object_unref(_ptr_2 as gpointer);
    }
    if !(*state).cancellable_source.is_null() {
        g_source_destroy((*state).cancellable_source);
        g_source_unref((*state).cancellable_source);
    }
    g_slice_free1(
        ::core::mem::size_of::<CommunicateState>() as gsize,
        state as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_internal(
    mut subprocess: *mut GSubprocess,
    mut add_nul: gboolean,
    mut stdin_buf: *mut GBytes,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) -> *mut CommunicateState {
    let mut state: *mut CommunicateState = ::core::ptr::null_mut::<CommunicateState>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(subprocess as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSubprocess,
                    gboolean,
                    *mut GBytes,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> *mut CommunicateState,
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_subprocess_communicate_internal
                as unsafe extern "C" fn(
                    *mut GSubprocess,
                    gboolean,
                    *mut GBytes,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> *mut CommunicateState,
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_subprocess_communicate_internal\0" as *const u8 as *const gchar,
        );
    }
    state = ({
        let mut __s: gsize = ::core::mem::size_of::<CommunicateState>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut CommunicateState;
    g_task_set_task_data(
        task,
        state as gpointer,
        Some(
            safe_c2rust_g_subprocess_communicate_state_free as unsafe extern "C" fn(gpointer) -> (),
        ),
    );
    (*state).cancellable = g_cancellable_new();
    (*state).add_nul = add_nul;
    if !cancellable.is_null() {
        (*state).cancellable_source = g_cancellable_source_new(cancellable);
        g_source_set_callback(
            (*state).cancellable_source,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GSourceFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> gboolean>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_subprocess_communicate_cancelled
                        as unsafe extern "C" fn(*mut GCancellable, gpointer) -> gboolean,
                )),
            ),
            state as gpointer,
            None,
        );
        g_source_attach(
            (*state).cancellable_source,
            g_main_context_get_thread_default(),
        );
    }
    if !(*subprocess).stdin_pipe.is_null() {
        if ({
            let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
            if !stdin_buf.is_null() {
                _g_boolean_var_41 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_41 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_41
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocess.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1452 as ::core::ffi::c_int,
                G_STRFUNC,
                b"stdin_buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_unix_set_fd_nonblocking(
            g_unix_output_stream_get_fd(
                (*subprocess).stdin_pipe as *mut ::core::ffi::c_void as *mut GUnixOutputStream,
            ),
            TRUE,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        (*state).stdin_buf = g_memory_input_stream_new_from_bytes(stdin_buf);
        g_output_stream_splice_async(
            (*subprocess).stdin_pipe,
            (*state).stdin_buf,
            (G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE as ::core::ffi::c_int
                | G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET as ::core::ffi::c_int)
                as GOutputStreamSpliceFlags,
            G_PRIORITY_DEFAULT,
            (*state).cancellable,
            Some(
                safe_c2rust_g_subprocess_communicate_made_progress
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            g_object_ref(task as gpointer) as *mut GTask as gpointer,
        );
        (*state).outstanding_ops = (*state).outstanding_ops.wrapping_add(1);
    }
    if !(*subprocess).stdout_pipe.is_null() {
        (*state).stdout_buf = g_memory_output_stream_new_resizable() as *mut GMemoryOutputStream;
        g_output_stream_splice_async(
            (*state).stdout_buf as *mut GOutputStream,
            (*subprocess).stdout_pipe,
            G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE,
            G_PRIORITY_DEFAULT,
            (*state).cancellable,
            Some(
                safe_c2rust_g_subprocess_communicate_made_progress
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            g_object_ref(task as gpointer) as *mut GTask as gpointer,
        );
        (*state).outstanding_ops = (*state).outstanding_ops.wrapping_add(1);
    }
    if !(*subprocess).stderr_pipe.is_null() {
        (*state).stderr_buf = g_memory_output_stream_new_resizable() as *mut GMemoryOutputStream;
        g_output_stream_splice_async(
            (*state).stderr_buf as *mut GOutputStream,
            (*subprocess).stderr_pipe,
            G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE,
            G_PRIORITY_DEFAULT,
            (*state).cancellable,
            Some(
                safe_c2rust_g_subprocess_communicate_made_progress
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            g_object_ref(task as gpointer) as *mut GTask as gpointer,
        );
        (*state).outstanding_ops = (*state).outstanding_ops.wrapping_add(1);
    }
    safe_c2rust_g_subprocess_wait_async(
        subprocess,
        (*state).cancellable,
        Some(
            safe_c2rust_g_subprocess_communicate_made_progress
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        g_object_ref(task as gpointer) as *mut GTask as gpointer,
    );
    (*state).outstanding_ops = (*state).outstanding_ops.wrapping_add(1);
    g_object_unref(task as gpointer);
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_communicate(
    mut subprocess: *mut GSubprocess,
    mut stdin_buf: *mut GBytes,
    mut cancellable: *mut GCancellable,
    mut stdout_buf: *mut *mut GBytes,
    mut stderr_buf: *mut *mut GBytes,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut result: *mut GAsyncResult = ::core::ptr::null_mut::<GAsyncResult>();
    let mut success: gboolean = 0;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if stdin_buf.is_null()
            || (*subprocess).flags as ::core::ffi::c_uint
                & G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
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
            b"stdin_buf == NULL || (subprocess->flags & G_SUBPROCESS_FLAGS_STDIN_PIPE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    safe_c2rust_g_subprocess_sync_setup();
    safe_c2rust_g_subprocess_communicate_internal(
        subprocess,
        FALSE,
        stdin_buf,
        cancellable,
        Some(
            safe_c2rust_g_subprocess_sync_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &raw mut result as gpointer,
    );
    safe_c2rust_g_subprocess_sync_complete(&raw mut result);
    success = safe_c2rust_g_subprocess_communicate_finish(
        subprocess, result, stdout_buf, stderr_buf, error,
    );
    g_object_unref(result as gpointer);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_async(
    mut subprocess: *mut GSubprocess,
    mut stdin_buf: *mut GBytes,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if stdin_buf.is_null()
            || (*subprocess).flags as ::core::ffi::c_uint
                & G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
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
            b"stdin_buf == NULL || (subprocess->flags & G_SUBPROCESS_FLAGS_STDIN_PIPE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_subprocess_communicate_internal(
        subprocess,
        FALSE,
        stdin_buf,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_finish(
    mut subprocess: *mut GSubprocess,
    mut result: *mut GAsyncResult,
    mut stdout_buf: *mut *mut GBytes,
    mut stderr_buf: *mut *mut GBytes,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut success: gboolean = 0;
    let mut state: *mut CommunicateState = ::core::ptr::null_mut::<CommunicateState>();
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, subprocess as gpointer) != 0 {
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
            b"g_task_is_valid (result, subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_object_ref(result as gpointer);
    state = g_task_get_task_data(result as *mut GTask) as *mut CommunicateState;
    success = g_task_propagate_boolean(result as *mut GTask, error);
    if success != 0 {
        if !stdout_buf.is_null() {
            *stdout_buf = if !(*state).stdout_buf.is_null() {
                g_memory_output_stream_steal_as_bytes((*state).stdout_buf)
            } else {
                ::core::ptr::null_mut::<GBytes>()
            };
        }
        if !stderr_buf.is_null() {
            *stderr_buf = if !(*state).stderr_buf.is_null() {
                g_memory_output_stream_steal_as_bytes((*state).stderr_buf)
            } else {
                ::core::ptr::null_mut::<GBytes>()
            };
        }
    }
    g_object_unref(result as gpointer);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_utf8(
    mut subprocess: *mut GSubprocess,
    mut stdin_buf: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut stdout_buf: *mut *mut ::core::ffi::c_char,
    mut stderr_buf: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut result: *mut GAsyncResult = ::core::ptr::null_mut::<GAsyncResult>();
    let mut success: gboolean = 0;
    let mut stdin_bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut stdin_buf_len: size_t = 0 as size_t;
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if stdin_buf.is_null()
            || (*subprocess).flags as ::core::ffi::c_uint
                & G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
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
            b"stdin_buf == NULL || (subprocess->flags & G_SUBPROCESS_FLAGS_STDIN_PIPE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
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
        return 0 as gboolean;
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
        return 0 as gboolean;
    }
    if !stdin_buf.is_null() {
        stdin_buf_len = strlen(stdin_buf);
    }
    stdin_bytes = g_bytes_new(stdin_buf as gconstpointer, stdin_buf_len as gsize);
    safe_c2rust_g_subprocess_sync_setup();
    safe_c2rust_g_subprocess_communicate_internal(
        subprocess,
        TRUE,
        stdin_bytes,
        cancellable,
        Some(
            safe_c2rust_g_subprocess_sync_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &raw mut result as gpointer,
    );
    safe_c2rust_g_subprocess_sync_complete(&raw mut result);
    success = safe_c2rust_g_subprocess_communicate_utf8_finish(
        subprocess, result, stdout_buf, stderr_buf, error,
    );
    g_object_unref(result as gpointer);
    g_bytes_unref(stdin_bytes);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_utf8_async(
    mut subprocess: *mut GSubprocess,
    mut stdin_buf: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut stdin_bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut stdin_buf_len: size_t = 0 as size_t;
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if stdin_buf.is_null()
            || (*subprocess).flags as ::core::ffi::c_uint
                & G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
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
            b"stdin_buf == NULL || (subprocess->flags & G_SUBPROCESS_FLAGS_STDIN_PIPE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !stdin_buf.is_null() {
        stdin_buf_len = strlen(stdin_buf);
    }
    stdin_bytes = g_bytes_new(stdin_buf as gconstpointer, stdin_buf_len as gsize);
    safe_c2rust_g_subprocess_communicate_internal(
        subprocess,
        TRUE,
        stdin_bytes,
        cancellable,
        callback,
        user_data,
    );
    g_bytes_unref(stdin_bytes);
}
unsafe extern "C" fn safe_c2rust_communicate_result_validate_utf8(
    mut stream_name: *const ::core::ffi::c_char,
    mut return_location: *mut *mut ::core::ffi::c_char,
    mut buffer: *mut GMemoryOutputStream,
    mut error: *mut *mut GError,
) -> gboolean {
    if return_location.is_null() {
        return TRUE;
    }
    if !buffer.is_null() {
        let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        *return_location = g_memory_output_stream_steal_data(buffer) as *mut ::core::ffi::c_char;
        if g_utf8_validate(
            *return_location,
            -(1 as ::core::ffi::c_int) as gssize,
            &raw mut end,
        ) == 0
        {
            g_free(*return_location as gpointer);
            *return_location = ::core::ptr::null_mut::<::core::ffi::c_char>();
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                b"Invalid UTF-8 in child %s at offset %lu\0" as *const u8 as *const gchar,
                stream_name,
                end.offset_from(*return_location) as ::core::ffi::c_long as ::core::ffi::c_ulong,
            );
            return FALSE;
        }
    } else {
        *return_location = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_communicate_utf8_finish(
    mut subprocess: *mut GSubprocess,
    mut result: *mut GAsyncResult,
    mut stdout_buf: *mut *mut ::core::ffi::c_char,
    mut stderr_buf: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    let mut state: *mut CommunicateState = ::core::ptr::null_mut::<CommunicateState>();
    let mut local_stdout_buf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut local_stderr_buf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = subprocess as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_get_type();
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
            b"G_IS_SUBPROCESS (subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, subprocess as gpointer) != 0 {
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
            b"g_task_is_valid (result, subprocess)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_object_ref(result as gpointer);
    state = g_task_get_task_data(result as *mut GTask) as *mut CommunicateState;
    if !(g_task_propagate_boolean(result as *mut GTask, error) == 0) {
        if !(safe_c2rust_communicate_result_validate_utf8(
            b"stdout\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut local_stdout_buf,
            (*state).stdout_buf,
            error,
        ) == 0)
        {
            if !(safe_c2rust_communicate_result_validate_utf8(
                b"stderr\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut local_stderr_buf,
                (*state).stderr_buf,
                error,
            ) == 0)
            {
                ret = TRUE as gboolean;
            }
        }
    }
    g_object_unref(result as gpointer);
    if ret != 0 && !stdout_buf.is_null() {
        *stdout_buf = safe_c2rust_g_steal_pointer(&raw mut local_stdout_buf as gpointer)
            as *mut gchar as *mut ::core::ffi::c_char;
    }
    if ret != 0 && !stderr_buf.is_null() {
        *stderr_buf = safe_c2rust_g_steal_pointer(&raw mut local_stderr_buf as gpointer)
            as *mut gchar as *mut ::core::ffi::c_char;
    }
    g_free(local_stderr_buf as gpointer);
    g_free(local_stdout_buf as gpointer);
    return ret;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
