use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _GSourcePrivate;
    pub type _GVariantType;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketPrivate;
    pub type _GSocketControlMessagePrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GTask;
    pub type _GUnixFDListPrivate;
    pub type _GDBusMessage;
    pub type _GUnixFDMessagePrivate;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
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
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_foreach(array: *mut GPtrArray, func: GFunc, user_data: gpointer);
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_error_copy(error: *const GError) -> *mut GError;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_propagate_prefixed_error(
        dest: *mut *mut GError,
        src: *mut GError,
        format: *const gchar,
        ...
    );
    fn g_parse_debug_string(string: *const gchar, keys: *const GDebugKey, nkeys: guint) -> guint;
    fn g_thread_new(name: *const gchar, func: GThreadFunc, data: gpointer) -> *mut GThread;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_signal(cond: *mut GCond);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
    fn g_main_context_new() -> *mut GMainContext;
    fn g_main_context_push_thread_default(context: *mut GMainContext);
    fn g_main_context_pop_thread_default(context: *mut GMainContext);
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    fn g_main_loop_run(loop_0: *mut GMainLoop);
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_variant_type_new_tuple(
        items: *const *const GVariantType,
        length: gint,
    ) -> *mut GVariantType;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_queue_new() -> *mut GQueue;
    fn g_queue_free_full(queue: *mut GQueue, free_func: GDestroyNotify);
    fn g_queue_get_length(queue: *mut GQueue) -> guint;
    fn g_queue_push_tail(queue: *mut GQueue, data: gpointer);
    fn g_queue_pop_head(queue: *mut GQueue) -> gpointer;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_cmpint(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        arg1: guint64,
        cmp: *const ::core::ffi::c_char,
        arg2: guint64,
        numtype: ::core::ffi::c_char,
    );
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_enum_get_value(enum_class: *mut GEnumClass, value: gint) -> *mut GEnumValue;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_dbus_auth_observer_get_type() -> GType;
    fn g_dbus_connection_get_type() -> GType;
    fn g_input_stream_read_async(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_input_stream_read_finish(
        stream: *mut GInputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_has_pending(stream: *mut GInputStream) -> gboolean;
    fn g_output_stream_write_async(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_output_stream_write_finish(
        stream: *mut GOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_output_stream_flush_async(
        stream: *mut GOutputStream,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_output_stream_flush_finish(
        stream: *mut GOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_has_pending(stream: *mut GOutputStream) -> gboolean;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_dbus_error_quark() -> GQuark;
    fn g_dbus_message_get_type() -> GType;
    fn g_dbus_message_print(message: *mut GDBusMessage, indent: guint) -> *mut gchar;
    fn g_dbus_message_get_serial(message: *mut GDBusMessage) -> guint32;
    fn g_dbus_message_get_unix_fd_list(message: *mut GDBusMessage) -> *mut GUnixFDList;
    fn g_dbus_message_set_unix_fd_list(message: *mut GDBusMessage, fd_list: *mut GUnixFDList);
    fn g_dbus_message_new_from_blob(
        blob: *mut guchar,
        blob_len: gsize,
        capabilities: GDBusCapabilityFlags,
        error: *mut *mut GError,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_bytes_needed(
        blob: *mut guchar,
        blob_len: gsize,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_dbus_message_to_blob(
        message: *mut GDBusMessage,
        out_size: *mut gsize,
        capabilities: GDBusCapabilityFlags,
        error: *mut *mut GError,
    ) -> *mut guchar;
    fn g_dbus_proxy_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn g_io_stream_get_type() -> GType;
    fn g_io_stream_get_input_stream(stream: *mut GIOStream) -> *mut GInputStream;
    fn g_io_stream_get_output_stream(stream: *mut GIOStream) -> *mut GOutputStream;
    fn g_io_stream_close_async(
        stream: *mut GIOStream,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_io_stream_close_finish(
        stream: *mut GIOStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_address_get_type() -> GType;
    fn g_socket_family_get_type() -> GType;
    fn g_socket_type_get_type() -> GType;
    fn g_socket_protocol_get_type() -> GType;
    fn g_dbus_connection_flags_get_type() -> GType;
    fn g_dbus_capability_flags_get_type() -> GType;
    fn g_memory_input_stream_get_type() -> GType;
    fn g_socket_get_type() -> GType;
    fn g_socket_condition_check(socket: *mut GSocket, condition: GIOCondition) -> GIOCondition;
    fn g_socket_receive_message(
        socket: *mut GSocket,
        address: *mut *mut GSocketAddress,
        vectors: *mut GInputVector,
        num_vectors: gint,
        messages: *mut *mut *mut GSocketControlMessage,
        num_messages: *mut gint,
        flags: *mut gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_socket_send_message(
        socket: *mut GSocket,
        address: *mut GSocketAddress,
        vectors: *mut GOutputVector,
        num_vectors: gint,
        messages: *mut *mut GSocketControlMessage,
        num_messages: gint,
        flags: gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_socket_create_source(
        socket: *mut GSocket,
        condition: GIOCondition,
        cancellable: *mut GCancellable,
    ) -> *mut GSource;
    fn g_socket_connection_get_type() -> GType;
    fn g_socket_connection_get_socket(connection: *mut GSocketConnection) -> *mut GSocket;
    fn g_task_get_type() -> GType;
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
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_attach_source(task: *mut GTask, source: *mut GSource, callback: GSourceFunc);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
    fn g_unix_credentials_message_get_type() -> GType;
    fn g_unix_fd_list_new_from_array(fds: *const gint, n_fds: gint) -> *mut GUnixFDList;
    fn g_unix_fd_list_append(list: *mut GUnixFDList, fd: gint, error: *mut *mut GError) -> gint;
    fn g_unix_fd_list_get_length(list: *mut GUnixFDList) -> gint;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn _g_socket_output_stream_get_type() -> GType;
    fn g_unix_fd_message_get_type() -> GType;
    fn g_unix_fd_message_new_with_fd_list(fd_list: *mut GUnixFDList) -> *mut GSocketControlMessage;
    fn g_unix_fd_message_steal_fds(message: *mut GUnixFDMessage, length: *mut gint) -> *mut gint;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
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
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDebugKey {
    pub key: *const gchar,
    pub value: guint,
}
pub type GDebugKey = _GDebugKey;
pub type GThreadFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThread {
    pub func: GThreadFunc,
    pub data: gpointer,
    pub joinable: gboolean,
    pub priority: GThreadPriority,
}
pub type GThreadPriority = ::core::ffi::c_uint;
pub const G_THREAD_PRIORITY_URGENT: GThreadPriority = 3;
pub const G_THREAD_PRIORITY_HIGH: GThreadPriority = 2;
pub const G_THREAD_PRIORITY_NORMAL: GThreadPriority = 1;
pub const G_THREAD_PRIORITY_LOW: GThreadPriority = 0;
pub type GThread = _GThread;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCond {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GCond = _GCond;
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
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
pub type GMainContext = _GMainContext;
pub type GMainLoop = _GMainLoop;
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
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type GSignalFlags = ::core::ffi::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_SOCKET_MSG_DONTROUTE: C2RustUnnamed_2 = 4;
pub const G_SOCKET_MSG_PEEK: C2RustUnnamed_2 = 2;
pub const G_SOCKET_MSG_OOB: C2RustUnnamed_2 = 1;
pub const G_SOCKET_MSG_NONE: C2RustUnnamed_2 = 0;
pub type GDBusCapabilityFlags = ::core::ffi::c_uint;
pub const G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING: GDBusCapabilityFlags = 1;
pub const G_DBUS_CAPABILITY_FLAGS_NONE: GDBusCapabilityFlags = 0;
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
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocket {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketPrivate,
}
pub type GSocketPrivate = _GSocketPrivate;
pub type GSocket = _GSocket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketControlMessage {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketControlMessagePrivate,
}
pub type GSocketControlMessagePrivate = _GSocketControlMessagePrivate;
pub type GSocketControlMessage = _GSocketControlMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GSocketConnectionPrivate,
}
pub type GSocketConnectionPrivate = _GSocketConnectionPrivate;
pub type GSocketConnection = _GSocketConnection;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputVector {
    pub buffer: gpointer,
    pub size: gsize,
}
pub type GInputVector = _GInputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputVector {
    pub buffer: gconstpointer,
    pub size: gsize,
}
pub type GOutputVector = _GOutputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusMessage = _GDBusMessage;
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDBusWorker {
    pub ref_count: gint,
    pub shared_thread_data: *mut SharedThreadData,
    pub stopped: gint,
    pub frozen: gboolean,
    pub capabilities: GDBusCapabilityFlags,
    pub received_messages_while_frozen: *mut GQueue,
    pub stream: *mut GIOStream,
    pub cancellable: *mut GCancellable,
    pub message_received_callback: GDBusWorkerMessageReceivedCallback,
    pub message_about_to_be_sent_callback: GDBusWorkerMessageAboutToBeSentCallback,
    pub disconnected_callback: GDBusWorkerDisconnectedCallback,
    pub user_data: gpointer,
    pub socket: *mut GSocket,
    pub read_lock: GMutex,
    pub read_buffer: *mut gchar,
    pub read_buffer_allocated_size: gsize,
    pub read_buffer_cur_size: gsize,
    pub read_buffer_bytes_wanted: gsize,
    pub read_fd_list: *mut GUnixFDList,
    pub read_ancillary_messages: *mut *mut GSocketControlMessage,
    pub read_num_ancillary_messages: gint,
    pub output_pending: OutputPending,
    pub write_lock: GMutex,
    pub write_queue: *mut GQueue,
    pub write_num_messages_written: guint64,
    pub write_num_messages_flushed: guint64,
    pub write_pending_flushes: *mut GList,
    pub pending_close_attempts: *mut GList,
    pub close_expected: gboolean,
}
pub type OutputPending = ::core::ffi::c_uint;
pub const PENDING_CLOSE: OutputPending = 3;
pub const PENDING_FLUSH: OutputPending = 2;
pub const PENDING_WRITE: OutputPending = 1;
pub const PENDING_NONE: OutputPending = 0;
pub type GDBusWorkerDisconnectedCallback =
    Option<unsafe extern "C" fn(*mut GDBusWorker, gboolean, *mut GError, gpointer) -> ()>;
pub type GDBusWorkerMessageAboutToBeSentCallback = Option<
    unsafe extern "C" fn(*mut GDBusWorker, *mut GDBusMessage, gpointer) -> *mut GDBusMessage,
>;
pub type GDBusWorkerMessageReceivedCallback =
    Option<unsafe extern "C" fn(*mut GDBusWorker, *mut GDBusMessage, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharedThreadData {
    pub refcount: gint,
    pub thread: *mut GThread,
    pub context: *mut GMainContext,
    pub loop_0: *mut GMainLoop,
}
pub type MessageToWriteData = _MessageToWriteData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MessageToWriteData {
    pub worker: *mut GDBusWorker,
    pub message: *mut GDBusMessage,
    pub blob: *mut gchar,
    pub blob_size: gsize,
    pub total_written: gsize,
    pub task: *mut GTask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CloseData {
    pub worker: *mut GDBusWorker,
    pub task: *mut GTask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FlushData {
    pub mutex: GMutex,
    pub cond: GCond,
    pub number_to_wait_for: guint64,
    pub finished: gboolean,
    pub error: *mut GError,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FlushAsyncData {
    pub worker: *mut GDBusWorker,
    pub flushers: *mut GList,
}
pub type GUnixFDMessage = _GUnixFDMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDMessage {
    pub parent_instance: GSocketControlMessage,
    pub priv_0: *mut GUnixFDMessagePrivate,
}
pub type GUnixFDMessagePrivate = _GUnixFDMessagePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReadWithControlData {
    pub buffer: *mut ::core::ffi::c_void,
    pub count: gsize,
    pub messages: *mut *mut *mut GSocketControlMessage,
    pub num_messages: *mut gint,
}
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_hexdump(
    mut data: *const gchar,
    mut len: gsize,
    mut indent: guint,
) -> *mut gchar {
    let mut n: guint = 0;
    let mut m: guint = 0;
    let mut ret: *mut GString = ::core::ptr::null_mut::<GString>();
    ret = g_string_new(::core::ptr::null::<gchar>());
    n = 0 as guint;
    while (n as gsize) < len {
        g_string_append_printf(
            ret,
            b"%*s%04x: \0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            n,
        );
        m = n;
        while m < n.wrapping_add(16 as guint) {
            if m > n && m.wrapping_rem(4 as guint) == 0 as guint {
                safe_c2rust_g_string_append_c_inline(ret, ' ' as i32 as gchar);
            }
            if (m as gsize) < len {
                g_string_append_printf(
                    ret,
                    b"%02x \0" as *const u8 as *const gchar,
                    *data.offset(m as isize) as guchar as ::core::ffi::c_int,
                );
            } else {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"   \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            ret,
                            __val,
                            if ({
                                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_10
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        ret,
                        b"   \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            m = m.wrapping_add(1);
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"   \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    ret,
                    __val,
                    if ({
                        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_11
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(
                ret,
                b"   \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        m = n;
        while (m as gsize) < len && m < n.wrapping_add(16 as guint) {
            safe_c2rust_g_string_append_c_inline(
                ret,
                (if *safe_c2rust_g_ascii_table.offset(*data.offset(m as isize) as guchar as isize)
                    as ::core::ffi::c_int
                    & G_ASCII_PRINT as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    *data.offset(m as isize) as ::core::ffi::c_int
                } else {
                    '.' as i32
                }) as gchar,
            );
            m = m.wrapping_add(1);
        }
        safe_c2rust_g_string_append_c_inline(ret, '\n' as i32 as gchar);
        n = n.wrapping_add(16 as guint);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(ret, 0 as gboolean)
        } else {
            g_string_free_and_steal(ret)
        }
    } else {
        g_string_free(ret, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_read_with_control_data_free(mut data: *mut ReadWithControlData) {
    g_slice_free1(
        ::core::mem::size_of::<ReadWithControlData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust__g_socket_read_with_control_messages_ready(
    mut socket: *mut GSocket,
    mut condition: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut ReadWithControlData = g_task_get_task_data(task) as *mut ReadWithControlData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: gssize = 0;
    let mut vector: GInputVector = _GInputVector {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
    };
    error = ::core::ptr::null_mut::<GError>();
    vector.buffer = (*data).buffer as gpointer;
    vector.size = (*data).count;
    result = g_socket_receive_message(
        socket,
        ::core::ptr::null_mut::<*mut GSocketAddress>(),
        &raw mut vector,
        1 as gint,
        (*data).messages,
        (*data).num_messages,
        ::core::ptr::null_mut::<gint>(),
        g_task_get_cancellable(task),
        &raw mut error,
    );
    if g_error_matches(
        error,
        g_io_error_quark(),
        G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int as gint,
    ) != 0
    {
        g_error_free(error);
        return TRUE;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if result >= 0 as gssize || !error.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            157 as ::core::ffi::c_int,
            G_STRFUNC,
            b"result >= 0 || error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if result >= 0 as gssize {
        g_task_return_int(task, result);
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__g_socket_read_with_control_messages(
    mut socket: *mut GSocket,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut messages: *mut *mut *mut GSocketControlMessage,
    mut num_messages: *mut gint,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut ReadWithControlData = ::core::ptr::null_mut::<ReadWithControlData>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<ReadWithControlData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut ReadWithControlData;
    (*data).buffer = buffer;
    (*data).count = count;
    (*data).messages = messages;
    (*data).num_messages = num_messages;
    task = g_task_new(socket as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSocket,
                    *mut ::core::ffi::c_void,
                    gsize,
                    *mut *mut *mut GSocketControlMessage,
                    *mut gint,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust__g_socket_read_with_control_messages
                as unsafe extern "C" fn(
                    *mut GSocket,
                    *mut ::core::ffi::c_void,
                    gsize,
                    *mut *mut *mut GSocketControlMessage,
                    *mut gint,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"_g_socket_read_with_control_messages\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(_task_0, b"[gio] D-Bus read\0" as *const u8 as *const gchar);
    } else {
        g_task_set_name(_task_0, b"[gio] D-Bus read\0" as *const u8 as *const gchar);
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ReadWithControlData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_read_with_control_data_free
                as unsafe extern "C" fn(*mut ReadWithControlData) -> (),
        )),
    );
    if g_socket_condition_check(socket, G_IO_IN) as u64 != 0 {
        if safe_c2rust__g_socket_read_with_control_messages_ready(socket, G_IO_IN, task as gpointer)
            == 0
        {
            return;
        }
    }
    source = g_socket_create_source(
        socket,
        (G_IO_IN as ::core::ffi::c_int
            | G_IO_HUP as ::core::ffi::c_int
            | G_IO_ERR as ::core::ffi::c_int) as GIOCondition,
        cancellable,
    );
    g_task_attach_source(
        task,
        source,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean>,
            GSourceFunc,
        >(Some(
            safe_c2rust__g_socket_read_with_control_messages_ready
                as unsafe extern "C" fn(*mut GSocket, GIOCondition, gpointer) -> gboolean,
        )),
    );
    g_source_unref(source);
}
unsafe extern "C" fn safe_c2rust__g_socket_read_with_control_messages_finish(
    mut socket: *mut GSocket,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = socket as *mut GTypeInstance;
            let mut __t: GType = g_socket_get_type();
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
            b"G_IS_SOCKET (socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, socket as gpointer) != 0 {
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
            b"g_task_is_valid (result, socket)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
static mut safe_c2rust_ensured_classes: *mut GPtrArray =
    ::core::ptr::null::<GPtrArray>() as *mut GPtrArray;
unsafe extern "C" fn safe_c2rust_ensure_type(mut gtype: GType) {
    g_ptr_array_add(safe_c2rust_ensured_classes, g_type_class_ref(gtype));
}
unsafe extern "C" fn safe_c2rust_release_required_types() {
    g_ptr_array_foreach(
        safe_c2rust_ensured_classes,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GFunc>(Some(
            g_type_class_unref as unsafe extern "C" fn(gpointer) -> (),
        )),
        NULL_0,
    );
    g_ptr_array_unref(safe_c2rust_ensured_classes);
    safe_c2rust_ensured_classes = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_ensure_required_types() {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if safe_c2rust_ensured_classes.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            241 as ::core::ffi::c_int,
            G_STRFUNC,
            b"ensured_classes == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_ensured_classes = g_ptr_array_new();
    safe_c2rust_ensure_type(g_task_get_type());
    safe_c2rust_ensure_type(g_memory_input_stream_get_type());
    safe_c2rust_ensure_type(g_dbus_connection_flags_get_type());
    safe_c2rust_ensure_type(g_dbus_capability_flags_get_type());
    safe_c2rust_ensure_type(g_dbus_auth_observer_get_type());
    safe_c2rust_ensure_type(g_dbus_connection_get_type());
    safe_c2rust_ensure_type(g_dbus_proxy_get_type());
    safe_c2rust_ensure_type(g_socket_family_get_type());
    safe_c2rust_ensure_type(g_socket_type_get_type());
    safe_c2rust_ensure_type(g_socket_protocol_get_type());
    safe_c2rust_ensure_type(g_socket_address_get_type());
    safe_c2rust_ensure_type(g_socket_get_type());
}
unsafe extern "C" fn safe_c2rust_gdbus_shared_thread_func(mut user_data: gpointer) -> gpointer {
    let mut data: *mut SharedThreadData = user_data as *mut SharedThreadData;
    g_main_context_push_thread_default((*data).context);
    g_main_loop_run((*data).loop_0);
    g_main_context_pop_thread_default((*data).context);
    safe_c2rust_release_required_types();
    return NULL_0;
}
unsafe extern "C" fn safe_c2rust__g_dbus_shared_thread_ref() -> *mut SharedThreadData {
    static mut safe_c2rust_shared_thread_data: *mut SharedThreadData =
        ::core::ptr::null::<SharedThreadData>() as *mut SharedThreadData;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_shared_thread_data;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut SharedThreadData =
                ::core::ptr::null_mut::<SharedThreadData>();
            let mut gapg_temp_atomic: *mut *mut SharedThreadData =
                &raw mut safe_c2rust_shared_thread_data;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_shared_thread_data as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut data: *mut SharedThreadData = ::core::ptr::null_mut::<SharedThreadData>();
        data = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<SharedThreadData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut SharedThreadData;
        (*data).refcount = 0 as ::core::ffi::c_int as gint;
        (*data).context = g_main_context_new();
        (*data).loop_0 = g_main_loop_new((*data).context, FALSE);
        (*data).thread = g_thread_new(
            b"gdbus\0" as *const u8 as *const gchar,
            Some(
                safe_c2rust_gdbus_shared_thread_func as unsafe extern "C" fn(gpointer) -> gpointer,
            ),
            data as gpointer,
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_shared_thread_data = data;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_shared_thread_data as *mut ::core::ffi::c_void,
            data as guintptr as gpointer,
        );
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*safe_c2rust_shared_thread_data).refcount;
        (*safe_c2rust_shared_thread_data).refcount;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        &raw mut (*safe_c2rust_shared_thread_data).refcount,
        1 as ::core::ffi::c_int,
    );
    return safe_c2rust_shared_thread_data;
}
unsafe extern "C" fn safe_c2rust__g_dbus_shared_thread_unref(mut data: *mut SharedThreadData) {}
unsafe extern "C" fn safe_c2rust_close_data_free(mut close_data: *mut CloseData) {
    let mut _pp: *mut *mut GTask = &raw mut (*close_data).task;
    let mut _ptr: *mut GTask = *_pp;
    *_pp = ::core::ptr::null_mut::<GTask>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    safe_c2rust__g_dbus_worker_unref((*close_data).worker);
    g_slice_free1(
        ::core::mem::size_of::<CloseData>() as gsize,
        close_data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_ref(
    mut worker: *mut GDBusWorker,
) -> *mut GDBusWorker {
    if 0 as ::core::ffi::c_int != 0 {
        (*worker).ref_count;
        (*worker).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*worker).ref_count, 1 as ::core::ffi::c_int);
    return worker;
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_unref(mut worker: *mut GDBusWorker) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*worker).ref_count;
            (*worker).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*worker).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        if ({
            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
            if (*worker).write_pending_flushes.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                456 as ::core::ffi::c_int,
                G_STRFUNC,
                b"worker->write_pending_flushes == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        safe_c2rust__g_dbus_shared_thread_unref((*worker).shared_thread_data);
        g_object_unref((*worker).stream as gpointer);
        g_mutex_clear(&raw mut (*worker).read_lock);
        g_object_unref((*worker).cancellable as gpointer);
        if !(*worker).read_fd_list.is_null() {
            g_object_unref((*worker).read_fd_list as gpointer);
        }
        g_queue_free_full(
            (*worker).received_messages_while_frozen,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
        g_mutex_clear(&raw mut (*worker).write_lock);
        g_queue_free_full(
            (*worker).write_queue,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut MessageToWriteData) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_message_to_write_data_free
                    as unsafe extern "C" fn(*mut MessageToWriteData) -> (),
            )),
        );
        g_free((*worker).read_buffer as gpointer);
        g_free(worker as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_emit_disconnected(
    mut worker: *mut GDBusWorker,
    mut remote_peer_vanished: gboolean,
    mut error: *mut GError,
) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*worker).stopped;
            (*worker).stopped;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*worker).stopped);
        gaig_temp
    }) == 0
    {
        (*worker)
            .disconnected_callback
            .expect("non-null function pointer")(
            worker,
            remote_peer_vanished,
            error,
            (*worker).user_data,
        );
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_emit_message_received(
    mut worker: *mut GDBusWorker,
    mut message: *mut GDBusMessage,
) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*worker).stopped;
            (*worker).stopped;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*worker).stopped);
        gaig_temp
    }) == 0
    {
        (*worker)
            .message_received_callback
            .expect("non-null function pointer")(worker, message, (*worker).user_data);
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_emit_message_about_to_be_sent(
    mut worker: *mut GDBusWorker,
    mut message: *mut GDBusMessage,
) -> *mut GDBusMessage {
    let mut ret: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*worker).stopped;
            (*worker).stopped;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*worker).stopped);
        gaig_temp
    }) == 0
    {
        ret = (*worker)
            .message_about_to_be_sent_callback
            .expect("non-null function pointer")(
            worker,
            safe_c2rust_g_steal_pointer(&raw mut message as gpointer) as *mut GDBusMessage,
            (*worker).user_data,
        );
    } else {
        ret = safe_c2rust_g_steal_pointer(&raw mut message as gpointer) as *mut GDBusMessage
            as *mut GDBusMessage;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_queue_or_deliver_received_message(
    mut worker: *mut GDBusWorker,
    mut message: *mut GDBusMessage,
) {
    if (*worker).frozen != 0
        || g_queue_get_length((*worker).received_messages_while_frozen) > 0 as guint
    {
        g_queue_push_tail(
            (*worker).received_messages_while_frozen,
            safe_c2rust_g_steal_pointer(&raw mut message as gpointer) as *mut GDBusMessage
                as gpointer,
        );
    } else {
        safe_c2rust__g_dbus_worker_emit_message_received(worker, message);
        let mut _pp: *mut *mut GDBusMessage = &raw mut message;
        let mut _ptr: *mut GDBusMessage = *_pp;
        *_pp = ::core::ptr::null_mut::<GDBusMessage>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    };
}
unsafe extern "C" fn safe_c2rust_unfreeze_in_idle_cb(mut user_data: gpointer) -> gboolean {
    let mut worker: *mut GDBusWorker = user_data as *mut GDBusWorker;
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    g_mutex_lock(&raw mut (*worker).read_lock);
    if (*worker).frozen != 0 {
        loop {
            message =
                g_queue_pop_head((*worker).received_messages_while_frozen) as *mut GDBusMessage;
            if message.is_null() {
                break;
            }
            safe_c2rust__g_dbus_worker_emit_message_received(worker, message);
            let mut _pp: *mut *mut GDBusMessage = &raw mut message;
            let mut _ptr: *mut GDBusMessage = *_pp;
            *_pp = ::core::ptr::null_mut::<GDBusMessage>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
        }
        (*worker).frozen = FALSE as gboolean;
    } else if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if g_queue_get_length((*worker).received_messages_while_frozen) == 0 as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            542 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_queue_get_length (worker->received_messages_while_frozen) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    g_mutex_unlock(&raw mut (*worker).read_lock);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_worker_unfreeze(mut worker: *mut GDBusWorker) {
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    idle_source = g_idle_source_new();
    g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        idle_source,
        Some(safe_c2rust_unfreeze_in_idle_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        safe_c2rust__g_dbus_worker_ref(worker) as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusWorker) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__g_dbus_worker_unref as unsafe extern "C" fn(*mut GDBusWorker) -> (),
        )),
    );
    g_source_set_static_name(
        idle_source,
        b"[gio] unfreeze_in_idle_cb\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(idle_source, (*(*worker).shared_thread_data).context);
    g_source_unref(idle_source);
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_do_read_cb(
    mut input_stream: *mut GInputStream,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut current_block: u64;
    let mut worker: *mut GDBusWorker = user_data as *mut GDBusWorker;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut bytes_read: gssize = 0;
    g_mutex_lock(&raw mut (*worker).read_lock);
    if !(({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*worker).stopped;
            (*worker).stopped;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*worker).stopped);
        gaig_temp
    }) != 0)
    {
        error = ::core::ptr::null_mut::<GError>();
        if (*worker).socket.is_null() {
            bytes_read = g_input_stream_read_finish(
                g_io_stream_get_input_stream((*worker).stream),
                res,
                &raw mut error,
            );
        } else {
            bytes_read = safe_c2rust__g_socket_read_with_control_messages_finish(
                (*worker).socket,
                res,
                &raw mut error,
            );
        }
        if (*worker).read_num_ancillary_messages > 0 as ::core::ffi::c_int {
            let mut n: gint = 0;
            n = 0 as ::core::ffi::c_int as gint;
            loop {
                if !(n < (*worker).read_num_ancillary_messages) {
                    current_block = 14434620278749266018;
                    break;
                }
                let mut control_message: *mut GSocketControlMessage =
                    *(*worker).read_ancillary_messages.offset(n as isize)
                        as *mut ::core::ffi::c_void
                        as *mut GSocketControlMessage;
                if ({
                    let mut __inst: *mut GTypeInstance = control_message as *mut GTypeInstance;
                    let mut __t: GType = g_unix_fd_message_get_type();
                    let mut __r: gboolean = 0;
                    if __inst.is_null() {
                        __r = FALSE as gboolean;
                    } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                        __r = TRUE as gboolean;
                    } else {
                        __r = g_type_check_instance_is_a(__inst, __t);
                    }
                    __r
                }) != 0
                {
                    let mut fd_message: *mut GUnixFDMessage =
                        ::core::ptr::null_mut::<GUnixFDMessage>();
                    let mut fds: *mut gint = ::core::ptr::null_mut::<gint>();
                    let mut num_fds: gint = 0;
                    fd_message = control_message as *mut ::core::ffi::c_void as *mut GUnixFDMessage;
                    fds = g_unix_fd_message_steal_fds(fd_message, &raw mut num_fds);
                    if (*worker).read_fd_list.is_null() {
                        (*worker).read_fd_list = g_unix_fd_list_new_from_array(fds, num_fds);
                    } else {
                        let mut n_0: gint = 0;
                        n_0 = 0 as ::core::ffi::c_int as gint;
                        while n_0 < num_fds {
                            g_unix_fd_list_append(
                                (*worker).read_fd_list,
                                *fds.offset(n_0 as isize),
                                ::core::ptr::null_mut::<*mut GError>(),
                            );
                            g_close(
                                *fds.offset(n_0 as isize),
                                ::core::ptr::null_mut::<*mut GError>(),
                            );
                            n_0 += 1;
                        }
                    }
                    g_free(fds as gpointer);
                } else if !(({
                    let mut __inst: *mut GTypeInstance = control_message as *mut GTypeInstance;
                    let mut __t: GType = g_unix_credentials_message_get_type();
                    let mut __r: gboolean = 0;
                    if __inst.is_null() {
                        __r = FALSE as gboolean;
                    } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                        __r = TRUE as gboolean;
                    } else {
                        __r = g_type_check_instance_is_a(__inst, __t);
                    }
                    __r
                }) != 0)
                {
                    if error.is_null() {
                        g_set_error(
                            &raw mut error,
                            g_io_error_quark(),
                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                            b"Unexpected ancillary message of type %s received from peer\0"
                                as *const u8 as *const gchar,
                            g_type_name(
                                (*(*(control_message as *mut GTypeInstance)).g_class).g_type,
                            ),
                        );
                        safe_c2rust__g_dbus_worker_emit_disconnected(worker, TRUE, error);
                        g_error_free(error);
                        g_object_unref(control_message as gpointer);
                        n += 1;
                        while n < (*worker).read_num_ancillary_messages {
                            let fresh1 = n;
                            n = n + 1;
                            g_object_unref(
                                *(*worker).read_ancillary_messages.offset(fresh1 as isize)
                                    as gpointer,
                            );
                        }
                        g_free((*worker).read_ancillary_messages as gpointer);
                        current_block = 9797183305398808869;
                        break;
                    }
                }
                g_object_unref(control_message as gpointer);
                n += 1;
            }
            match current_block {
                9797183305398808869 => {}
                _ => {
                    g_free((*worker).read_ancillary_messages as gpointer);
                    current_block = 7245201122033322888;
                }
            }
        } else {
            current_block = 7245201122033322888;
        }
        match current_block {
            9797183305398808869 => {}
            _ => {
                if bytes_read == -(1 as ::core::ffi::c_int) as gssize {
                    if ({
                        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                        if safe_c2rust__g_dbus_debug_transport() != 0 {
                            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_18
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        safe_c2rust__g_dbus_debug_print_lock();
                        g_print(
                            b"========================================================================\nGDBus-debug:Transport:\n  ---- READ ERROR on stream of type %s:\n  ---- %s %d: %s\n\0"
                                as *const u8 as *const gchar,
                            g_type_name(
                                (*(*(g_io_stream_get_input_stream((*worker).stream)
                                    as *mut GTypeInstance))
                                    .g_class)
                                    .g_type,
                            ),
                            g_quark_to_string((*error).domain),
                            (*error).code,
                            (*error).message,
                        );
                        safe_c2rust__g_dbus_debug_print_unlock();
                    }
                    if (*worker).close_expected != 0
                        || g_error_matches(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
                        ) != 0
                    {
                        safe_c2rust__g_dbus_worker_emit_disconnected(
                            worker,
                            FALSE,
                            ::core::ptr::null_mut::<GError>(),
                        );
                    } else {
                        safe_c2rust__g_dbus_worker_emit_disconnected(worker, TRUE, error);
                    }
                    g_error_free(error);
                } else if bytes_read == 0 as gssize {
                    g_set_error(
                        &raw mut error,
                        g_io_error_quark(),
                        G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                        b"Underlying GIOStream returned 0 bytes on an async read\0" as *const u8
                            as *const gchar,
                    );
                    safe_c2rust__g_dbus_worker_emit_disconnected(worker, TRUE, error);
                    g_error_free(error);
                } else {
                    safe_c2rust_read_message_print_transport_debug(bytes_read, worker);
                    (*worker).read_buffer_cur_size = (*worker)
                        .read_buffer_cur_size
                        .wrapping_add(bytes_read as gsize);
                    if (*worker).read_buffer_bytes_wanted == (*worker).read_buffer_cur_size {
                        if (*worker).read_buffer_bytes_wanted == 16 as gsize {
                            let mut message_len: gssize = 0;
                            error = ::core::ptr::null_mut::<GError>();
                            message_len = g_dbus_message_bytes_needed(
                                (*worker).read_buffer as *mut guchar,
                                16 as gsize,
                                &raw mut error,
                            );
                            if message_len == -(1 as ::core::ffi::c_int) as gssize {
                                g_log(
                                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                                    G_LOG_LEVEL_WARNING,
                                    b"_g_dbus_worker_do_read_cb: error determining bytes needed: %s\0"
                                        as *const u8 as *const gchar,
                                    (*error).message,
                                );
                                safe_c2rust__g_dbus_worker_emit_disconnected(worker, FALSE, error);
                                g_error_free(error);
                            } else {
                                (*worker).read_buffer_bytes_wanted = message_len as gsize;
                                safe_c2rust__g_dbus_worker_do_read_unlocked(worker);
                            }
                        } else {
                            let mut message: *mut GDBusMessage =
                                ::core::ptr::null_mut::<GDBusMessage>();
                            error = ::core::ptr::null_mut::<GError>();
                            message = g_dbus_message_new_from_blob(
                                (*worker).read_buffer as *mut guchar,
                                (*worker).read_buffer_cur_size,
                                (*worker).capabilities,
                                &raw mut error,
                            );
                            if message.is_null() {
                                let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
                                s = safe_c2rust__g_dbus_hexdump(
                                    (*worker).read_buffer,
                                    (*worker).read_buffer_cur_size,
                                    2 as guint,
                                );
                                g_log(
                                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                                    G_LOG_LEVEL_WARNING,
                                    b"Error decoding D-Bus message of %lu bytes\nThe error is: %s\nThe payload is as follows:\n%s\0"
                                        as *const u8 as *const gchar,
                                    (*worker).read_buffer_cur_size,
                                    (*error).message,
                                    s,
                                );
                                g_free(s as gpointer);
                                safe_c2rust__g_dbus_worker_emit_disconnected(worker, FALSE, error);
                                g_error_free(error);
                            } else {
                                if !(*worker).read_fd_list.is_null() {
                                    g_dbus_message_set_unix_fd_list(
                                        message,
                                        (*worker).read_fd_list,
                                    );
                                    g_object_unref((*worker).read_fd_list as gpointer);
                                    (*worker).read_fd_list = ::core::ptr::null_mut::<GUnixFDList>();
                                }
                                if ({
                                    let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                                    if safe_c2rust__g_dbus_debug_message() != 0 {
                                        _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_19
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    let mut s_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
                                    safe_c2rust__g_dbus_debug_print_lock();
                                    g_print(
                                        b"========================================================================\nGDBus-debug:Message:\n  <<<< RECEIVED D-Bus message (%lu bytes)\n\0"
                                            as *const u8 as *const gchar,
                                        (*worker).read_buffer_cur_size,
                                    );
                                    s_0 = g_dbus_message_print(message, 2 as guint);
                                    g_print(b"%s\0" as *const u8 as *const gchar, s_0);
                                    g_free(s_0 as gpointer);
                                    if ({
                                        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                                        if safe_c2rust__g_dbus_debug_payload() != 0 {
                                            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_20
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                        s_0 = safe_c2rust__g_dbus_hexdump(
                                            (*worker).read_buffer,
                                            (*worker).read_buffer_cur_size,
                                            2 as guint,
                                        );
                                        g_print(b"%s\n\0" as *const u8 as *const gchar, s_0);
                                        g_free(s_0 as gpointer);
                                    }
                                    safe_c2rust__g_dbus_debug_print_unlock();
                                }
                                safe_c2rust__g_dbus_worker_queue_or_deliver_received_message(
                                    worker,
                                    safe_c2rust_g_steal_pointer(&raw mut message as gpointer)
                                        as *mut GDBusMessage,
                                );
                                (*worker).read_buffer_bytes_wanted = 0 as gsize;
                                (*worker).read_buffer_cur_size = 0 as gsize;
                                safe_c2rust__g_dbus_worker_do_read_unlocked(worker);
                            }
                        }
                    } else {
                        safe_c2rust__g_dbus_worker_do_read_unlocked(worker);
                    }
                }
            }
        }
    }
    g_mutex_unlock(&raw mut (*worker).read_lock);
    safe_c2rust_schedule_pending_close(worker);
    safe_c2rust__g_dbus_worker_unref(worker);
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_do_read_unlocked(mut worker: *mut GDBusWorker) {
    if (*worker).read_buffer_bytes_wanted == 0 as gsize {
        (*worker).read_buffer_cur_size = 0 as gsize;
        (*worker).read_buffer_bytes_wanted = 16 as gsize;
    }
    if (*worker).read_buffer.is_null()
        || (*worker).read_buffer_bytes_wanted > (*worker).read_buffer_allocated_size
    {
        (*worker).read_buffer_allocated_size = if (*worker).read_buffer_bytes_wanted > 4096 as gsize
        {
            (*worker).read_buffer_bytes_wanted
        } else {
            4096 as gsize
        };
        (*worker).read_buffer = g_realloc(
            (*worker).read_buffer as gpointer,
            (*worker).read_buffer_allocated_size,
        ) as *mut gchar;
    }
    if (*worker).socket.is_null() {
        g_input_stream_read_async(
            g_io_stream_get_input_stream((*worker).stream),
            (*worker)
                .read_buffer
                .offset((*worker).read_buffer_cur_size as isize)
                as *mut ::core::ffi::c_void,
            (*worker)
                .read_buffer_bytes_wanted
                .wrapping_sub((*worker).read_buffer_cur_size),
            G_PRIORITY_DEFAULT,
            (*worker).cancellable,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, gpointer) -> ()>,
                GAsyncReadyCallback,
            >(Some(
                safe_c2rust__g_dbus_worker_do_read_cb
                    as unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, gpointer) -> (),
            )),
            safe_c2rust__g_dbus_worker_ref(worker) as gpointer,
        );
    } else {
        (*worker).read_ancillary_messages = ::core::ptr::null_mut::<*mut GSocketControlMessage>();
        (*worker).read_num_ancillary_messages = 0 as ::core::ffi::c_int as gint;
        safe_c2rust__g_socket_read_with_control_messages(
            (*worker).socket,
            (*worker)
                .read_buffer
                .offset((*worker).read_buffer_cur_size as isize)
                as *mut ::core::ffi::c_void,
            (*worker)
                .read_buffer_bytes_wanted
                .wrapping_sub((*worker).read_buffer_cur_size),
            &raw mut (*worker).read_ancillary_messages,
            &raw mut (*worker).read_num_ancillary_messages,
            G_PRIORITY_DEFAULT,
            (*worker).cancellable,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, gpointer) -> ()>,
                GAsyncReadyCallback,
            >(Some(
                safe_c2rust__g_dbus_worker_do_read_cb
                    as unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, gpointer) -> (),
            )),
            safe_c2rust__g_dbus_worker_ref(worker) as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust__g_dbus_worker_do_initial_read(mut data: gpointer) -> gboolean {
    let mut worker: *mut GDBusWorker = data as *mut GDBusWorker;
    g_mutex_lock(&raw mut (*worker).read_lock);
    safe_c2rust__g_dbus_worker_do_read_unlocked(worker);
    g_mutex_unlock(&raw mut (*worker).read_lock);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_message_to_write_data_free(mut data: *mut MessageToWriteData) {
    safe_c2rust__g_dbus_worker_unref((*data).worker);
    let mut _pp: *mut *mut GDBusMessage = &raw mut (*data).message;
    let mut _ptr: *mut GDBusMessage = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusMessage>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_free((*data).blob as gpointer);
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if (*data).task.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            907 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->task == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_slice_free1(
        ::core::mem::size_of::<MessageToWriteData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_write_message_async_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut MessageToWriteData =
        safe_c2rust_g_steal_pointer(&raw mut user_data as gpointer) as *mut MessageToWriteData;
    let mut bytes_written: gssize = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    error = ::core::ptr::null_mut::<GError>();
    bytes_written = g_output_stream_write_finish(
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream,
        res,
        &raw mut error,
    );
    if bytes_written == -(1 as ::core::ffi::c_int) as gssize {
        let mut task: *mut GTask =
            safe_c2rust_g_steal_pointer(&raw mut (*data).task as gpointer) as *mut GTask;
        g_task_return_error(task, error);
        let mut _pp: *mut *mut GTask = &raw mut task;
        let mut _ptr: *mut GTask = *_pp;
        *_pp = ::core::ptr::null_mut::<GTask>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    } else {
        if ({
            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
            if bytes_written > 0 as gssize {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                948 as ::core::ffi::c_int,
                G_STRFUNC,
                b"bytes_written > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        safe_c2rust_write_message_print_transport_debug(bytes_written, data);
        (*data).total_written = (*data).total_written.wrapping_add(bytes_written as gsize);
        if ({
            let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
            if (*data).total_written <= (*data).blob_size {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                953 as ::core::ffi::c_int,
                G_STRFUNC,
                b"data->total_written <= data->blob_size\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*data).total_written == (*data).blob_size {
            let mut task_0: *mut GTask =
                safe_c2rust_g_steal_pointer(&raw mut (*data).task as gpointer) as *mut GTask;
            g_task_return_boolean(task_0, TRUE);
            let mut _pp_0: *mut *mut GTask = &raw mut task_0;
            let mut _ptr_0: *mut GTask = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<GTask>();
            if !_ptr_0.is_null() {
                g_object_unref(_ptr_0 as gpointer);
            }
        } else {
            safe_c2rust_write_message_continue_writing(safe_c2rust_g_steal_pointer(
                &raw mut data as gpointer,
            ) as *mut MessageToWriteData);
        }
    };
}
unsafe extern "C" fn safe_c2rust_on_socket_ready(
    mut socket: *mut GSocket,
    mut condition: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut MessageToWriteData =
        safe_c2rust_g_steal_pointer(&raw mut user_data as gpointer) as *mut MessageToWriteData;
    safe_c2rust_write_message_continue_writing(safe_c2rust_g_steal_pointer(
        &raw mut data as gpointer,
    ) as *mut MessageToWriteData);
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_write_message_continue_writing(mut data: *mut MessageToWriteData) {
    let mut current_block: u64;
    let mut ostream: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
    ostream = g_io_stream_get_output_stream((*(*data).worker).stream);
    fd_list = g_dbus_message_get_unix_fd_list((*data).message);
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if g_output_stream_has_pending(ostream) == 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1010 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!g_output_stream_has_pending (ostream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut __n1: gint64 = (*data).total_written as gint64;
    let mut __n2: gint64 = (*data).blob_size as gint64;
    if !(__n1 < __n2) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1011 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->total_written < data->blob_size\0" as *const u8 as *const ::core::ffi::c_char,
            __n1 as guint64,
            b"<\0" as *const u8 as *const ::core::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as ::core::ffi::c_char,
        );
    }
    if ({
        let mut __inst: *mut GTypeInstance = ostream as *mut GTypeInstance;
        let mut __t: GType = _g_socket_output_stream_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
        && (*data).total_written == 0 as gsize
    {
        let mut vector: GOutputVector = _GOutputVector {
            buffer: ::core::ptr::null::<::core::ffi::c_void>(),
            size: 0,
        };
        let mut control_message: *mut GSocketControlMessage =
            ::core::ptr::null_mut::<GSocketControlMessage>();
        let mut bytes_written: gssize = 0;
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        vector.buffer = (*data).blob as gconstpointer;
        vector.size = (*data).blob_size;
        control_message = ::core::ptr::null_mut::<GSocketControlMessage>();
        if !fd_list.is_null() && g_unix_fd_list_get_length(fd_list) > 0 as ::core::ffi::c_int {
            if (*(*data).worker).capabilities as ::core::ffi::c_uint
                & G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                == 0
            {
                let mut task: *mut GTask =
                    safe_c2rust_g_steal_pointer(&raw mut (*data).task as gpointer) as *mut GTask;
                g_task_return_new_error_literal(
                    task,
                    g_io_error_quark(),
                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                    b"Tried sending a file descriptor but remote peer does not support this capability\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                let mut _pp: *mut *mut GTask = &raw mut task;
                let mut _ptr: *mut GTask = *_pp;
                *_pp = ::core::ptr::null_mut::<GTask>();
                if !_ptr.is_null() {
                    g_object_unref(_ptr as gpointer);
                }
                current_block = 7178192492338286402;
            } else {
                control_message = g_unix_fd_message_new_with_fd_list(fd_list);
                current_block = 2232869372362427478;
            }
        } else {
            current_block = 2232869372362427478;
        }
        match current_block {
            7178192492338286402 => {}
            _ => {
                error = ::core::ptr::null_mut::<GError>();
                bytes_written = g_socket_send_message(
                    (*(*data).worker).socket,
                    ::core::ptr::null_mut::<GSocketAddress>(),
                    &raw mut vector,
                    1 as gint,
                    if !control_message.is_null() {
                        &raw mut control_message
                    } else {
                        ::core::ptr::null_mut::<*mut GSocketControlMessage>()
                    },
                    if !control_message.is_null() {
                        1 as gint
                    } else {
                        0 as gint
                    },
                    G_SOCKET_MSG_NONE as ::core::ffi::c_int as gint,
                    (*(*data).worker).cancellable,
                    &raw mut error,
                );
                if !control_message.is_null() {
                    g_object_unref(control_message as gpointer);
                }
                if bytes_written == -(1 as ::core::ffi::c_int) as gssize {
                    if g_error_matches(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int as gint,
                    ) != 0
                    {
                        let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
                        source = g_socket_create_source(
                            (*(*data).worker).socket,
                            (G_IO_OUT as ::core::ffi::c_int
                                | G_IO_HUP as ::core::ffi::c_int
                                | G_IO_ERR as ::core::ffi::c_int)
                                as GIOCondition,
                            (*(*data).worker).cancellable,
                        );
                        g_source_set_callback(
                            source,
                            ::core::mem::transmute::<
                                Option<
                                    unsafe extern "C" fn(
                                        *mut GSocket,
                                        GIOCondition,
                                        gpointer,
                                    )
                                        -> gboolean,
                                >,
                                GSourceFunc,
                            >(Some(
                                safe_c2rust_on_socket_ready
                                    as unsafe extern "C" fn(
                                        *mut GSocket,
                                        GIOCondition,
                                        gpointer,
                                    )
                                        -> gboolean,
                            )),
                            safe_c2rust_g_steal_pointer(&raw mut data as gpointer)
                                as *mut MessageToWriteData as gpointer,
                            None,
                        );
                        g_source_attach(source, g_main_context_get_thread_default());
                        g_source_unref(source);
                        g_error_free(error);
                    } else {
                        let mut task_0: *mut GTask =
                            safe_c2rust_g_steal_pointer(&raw mut (*data).task as gpointer)
                                as *mut GTask;
                        g_task_return_error(task_0, error);
                        let mut _pp_0: *mut *mut GTask = &raw mut task_0;
                        let mut _ptr_0: *mut GTask = *_pp_0;
                        *_pp_0 = ::core::ptr::null_mut::<GTask>();
                        if !_ptr_0.is_null() {
                            g_object_unref(_ptr_0 as gpointer);
                        }
                    }
                } else {
                    if ({
                        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                        if bytes_written > 0 as gssize {
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
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1084 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"bytes_written > 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    safe_c2rust_write_message_print_transport_debug(bytes_written, data);
                    (*data).total_written =
                        (*data).total_written.wrapping_add(bytes_written as gsize);
                    if ({
                        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                        if (*data).total_written <= (*data).blob_size {
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
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1089 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"data->total_written <= data->blob_size\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if (*data).total_written == (*data).blob_size {
                        let mut task_1: *mut GTask =
                            safe_c2rust_g_steal_pointer(&raw mut (*data).task as gpointer)
                                as *mut GTask;
                        g_task_return_boolean(task_1, TRUE);
                        let mut _pp_1: *mut *mut GTask = &raw mut task_1;
                        let mut _ptr_1: *mut GTask = *_pp_1;
                        *_pp_1 = ::core::ptr::null_mut::<GTask>();
                        if !_ptr_1.is_null() {
                            g_object_unref(_ptr_1 as gpointer);
                        }
                    } else {
                        safe_c2rust_write_message_continue_writing(safe_c2rust_g_steal_pointer(
                            &raw mut data as gpointer,
                        )
                            as *mut MessageToWriteData);
                    }
                }
            }
        }
    } else if (*data).total_written == 0 as gsize && !fd_list.is_null() {
        let mut task_2: *mut GTask =
            safe_c2rust_g_steal_pointer(&raw mut (*data).task as gpointer) as *mut GTask;
        g_task_return_new_error(
            task_2,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            b"Tried sending a file descriptor on unsupported stream of type %s\0" as *const u8
                as *const ::core::ffi::c_char,
            g_type_name((*(*(ostream as *mut GTypeInstance)).g_class).g_type),
        );
        let mut _pp_2: *mut *mut GTask = &raw mut task_2;
        let mut _ptr_2: *mut GTask = *_pp_2;
        *_pp_2 = ::core::ptr::null_mut::<GTask>();
        if !_ptr_2.is_null() {
            g_object_unref(_ptr_2 as gpointer);
        }
    } else {
        g_output_stream_write_async(
            ostream,
            ((*data).blob as *const gchar).offset((*data).total_written as isize)
                as *const ::core::ffi::c_void,
            (*data).blob_size.wrapping_sub((*data).total_written),
            G_PRIORITY_DEFAULT,
            (*(*data).worker).cancellable,
            Some(
                safe_c2rust_write_message_async_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            data as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_write_message_async(
    mut worker: *mut GDBusWorker,
    mut data: *mut MessageToWriteData,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    (*data).task = g_task_new(
        NULL_0,
        ::core::ptr::null_mut::<GCancellable>(),
        callback,
        user_data,
    );
    let mut _task: *mut GTask = (*data).task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusWorker,
                    *mut MessageToWriteData,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_write_message_async
                as unsafe extern "C" fn(
                    *mut GDBusWorker,
                    *mut MessageToWriteData,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(_task, b"write_message_async\0" as *const u8 as *const gchar);
    }
    let mut _task_0: *mut GTask = (*data).task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] D-Bus write message\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] D-Bus write message\0" as *const u8 as *const gchar,
        );
    }
    (*data).total_written = 0 as gsize;
    safe_c2rust_write_message_continue_writing(safe_c2rust_g_steal_pointer(
        &raw mut data as gpointer,
    ) as *mut MessageToWriteData);
}
unsafe extern "C" fn safe_c2rust_write_message_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            res as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
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
            b"g_task_is_valid (res, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_flush_data_list_complete(
    mut flushers: *const GList,
    mut error: *const GError,
) {
    let mut l: *const GList = ::core::ptr::null::<GList>();
    l = flushers;
    while !l.is_null() {
        let mut f: *mut FlushData = (*l).data as *mut FlushData;
        (*f).error = if !error.is_null() {
            g_error_copy(error)
        } else {
            ::core::ptr::null_mut::<GError>()
        };
        g_mutex_lock(&raw mut (*f).mutex);
        (*f).finished = TRUE as gboolean;
        g_cond_signal(&raw mut (*f).cond);
        g_mutex_unlock(&raw mut (*f).mutex);
        l = (*l).next;
    }
}
unsafe extern "C" fn safe_c2rust_ostream_flush_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut FlushAsyncData = user_data as *mut FlushAsyncData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    error = ::core::ptr::null_mut::<GError>();
    g_output_stream_flush_finish(
        source_object as *mut ::core::ffi::c_void as *mut GOutputStream,
        res,
        &raw mut error,
    );
    if error.is_null() {
        if ({
            let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
            if safe_c2rust__g_dbus_debug_transport() != 0 {
                _g_boolean_var_28 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_28 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_28
        }) as ::core::ffi::c_long
            != 0
        {
            safe_c2rust__g_dbus_debug_print_lock();
            g_print(
                b"========================================================================\nGDBus-debug:Transport:\n  ---- FLUSHED stream of type %s\n\0"
                    as *const u8 as *const gchar,
                g_type_name(
                    (*(*(g_io_stream_get_output_stream((*(*data).worker).stream)
                        as *mut GTypeInstance))
                        .g_class)
                        .g_type,
                ),
            );
            safe_c2rust__g_dbus_debug_print_unlock();
        }
    }
    g_mutex_lock(&raw mut (*(*data).worker).write_lock);
    (*(*data).worker).write_num_messages_flushed = (*(*data).worker).write_num_messages_written;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if (*(*data).worker).output_pending as ::core::ffi::c_uint
            == PENDING_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1225 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->worker->output_pending == PENDING_FLUSH\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*(*data).worker).output_pending = PENDING_NONE;
    g_mutex_unlock(&raw mut (*(*data).worker).write_lock);
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !(*data).flushers.is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1229 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->flushers != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_flush_data_list_complete((*data).flushers, error);
    g_list_free((*data).flushers);
    if !error.is_null() {
        g_error_free(error);
    }
    safe_c2rust_continue_writing((*data).worker);
    safe_c2rust__g_dbus_worker_unref((*data).worker);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_start_flush(mut data: *mut FlushAsyncData) {
    g_output_stream_flush_async(
        g_io_stream_get_output_stream((*(*data).worker).stream),
        G_PRIORITY_DEFAULT,
        (*(*data).worker).cancellable,
        Some(
            safe_c2rust_ostream_flush_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_message_written_unlocked(
    mut worker: *mut GDBusWorker,
    mut message_data: *mut MessageToWriteData,
) {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if safe_c2rust__g_dbus_debug_message() != 0 {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
        let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
        safe_c2rust__g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Message:\n  >>>> SENT D-Bus message (%lu bytes)\n\0"
                as *const u8 as *const gchar,
            (*message_data).blob_size,
        );
        s = g_dbus_message_print((*message_data).message, 2 as guint);
        g_print(b"%s\0" as *const u8 as *const gchar, s);
        g_free(s as gpointer);
        if ({
            let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
            if safe_c2rust__g_dbus_debug_payload() != 0 {
                _g_boolean_var_32 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_32 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_32
        }) as ::core::ffi::c_long
            != 0
        {
            s = safe_c2rust__g_dbus_hexdump(
                (*message_data).blob,
                (*message_data).blob_size,
                2 as guint,
            );
            g_print(b"%s\n\0" as *const u8 as *const gchar, s);
            g_free(s as gpointer);
        }
        safe_c2rust__g_dbus_debug_print_unlock();
    }
    (*worker).write_num_messages_written = (*worker)
        .write_num_messages_written
        .wrapping_add(1 as guint64);
}
unsafe extern "C" fn safe_c2rust_prepare_flush_unlocked(
    mut worker: *mut GDBusWorker,
) -> *mut FlushAsyncData {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ll: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut flushers: *mut GList = ::core::ptr::null_mut::<GList>();
    flushers = ::core::ptr::null_mut::<GList>();
    l = (*worker).write_pending_flushes;
    while !l.is_null() {
        let mut f: *mut FlushData = (*l).data as *mut FlushData;
        ll = (*l).next;
        if (*f).number_to_wait_for == (*worker).write_num_messages_written {
            flushers = g_list_append(flushers, f as gpointer);
            (*worker).write_pending_flushes =
                g_list_delete_link((*worker).write_pending_flushes, l);
        }
        l = ll;
    }
    if !flushers.is_null() {
        if ({
            let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
            if (*worker).output_pending as ::core::ffi::c_uint
                == PENDING_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1317 as ::core::ffi::c_int,
                G_STRFUNC,
                b"worker->output_pending == PENDING_NONE\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*worker).output_pending = PENDING_FLUSH;
    }
    if !flushers.is_null() {
        let mut data: *mut FlushAsyncData = ::core::ptr::null_mut::<FlushAsyncData>();
        data = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<FlushAsyncData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut FlushAsyncData;
        (*data).worker = safe_c2rust__g_dbus_worker_ref(worker);
        (*data).flushers = flushers;
        return data;
    }
    return ::core::ptr::null_mut::<FlushAsyncData>();
}
unsafe extern "C" fn safe_c2rust_write_message_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut MessageToWriteData = user_data as *mut MessageToWriteData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    g_mutex_lock(&raw mut (*(*data).worker).write_lock);
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*(*data).worker).output_pending as ::core::ffi::c_uint
            == PENDING_WRITE as ::core::ffi::c_int as ::core::ffi::c_uint
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1349 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->worker->output_pending == PENDING_WRITE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*(*data).worker).output_pending = PENDING_NONE;
    error = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_write_message_finish(res, &raw mut error) == 0 {
        g_mutex_unlock(&raw mut (*(*data).worker).write_lock);
        safe_c2rust__g_dbus_worker_emit_disconnected((*data).worker, TRUE, error);
        g_error_free(error);
        g_mutex_lock(&raw mut (*(*data).worker).write_lock);
    }
    safe_c2rust_message_written_unlocked((*data).worker, data);
    g_mutex_unlock(&raw mut (*(*data).worker).write_lock);
    safe_c2rust_continue_writing((*data).worker);
    safe_c2rust_message_to_write_data_free(data);
}
unsafe extern "C" fn safe_c2rust_iostream_close_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut worker: *mut GDBusWorker = user_data as *mut GDBusWorker;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut pending_close_attempts: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut pending_flush_attempts: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut send_queue: *mut GQueue = ::core::ptr::null_mut::<GQueue>();
    g_io_stream_close_finish((*worker).stream, res, &raw mut error);
    g_mutex_lock(&raw mut (*worker).write_lock);
    pending_close_attempts = (*worker).pending_close_attempts;
    (*worker).pending_close_attempts = ::core::ptr::null_mut::<GList>();
    pending_flush_attempts = (*worker).write_pending_flushes;
    (*worker).write_pending_flushes = ::core::ptr::null_mut::<GList>();
    send_queue = (*worker).write_queue;
    (*worker).write_queue = g_queue_new();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if (*worker).output_pending as ::core::ffi::c_uint
            == PENDING_CLOSE as ::core::ffi::c_int as ::core::ffi::c_uint
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1401 as ::core::ffi::c_int,
            G_STRFUNC,
            b"worker->output_pending == PENDING_CLOSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*worker).output_pending = PENDING_NONE;
    (*worker).write_num_messages_flushed = (*worker)
        .write_num_messages_written
        .wrapping_add(g_list_length(pending_flush_attempts) as guint64);
    g_mutex_unlock(&raw mut (*worker).write_lock);
    while !pending_close_attempts.is_null() {
        let mut close_data: *mut CloseData = (*pending_close_attempts).data as *mut CloseData;
        pending_close_attempts = g_list_delete_link(pending_close_attempts, pending_close_attempts);
        if !(*close_data).task.is_null() {
            if !error.is_null() {
                g_task_return_error((*close_data).task, g_error_copy(error));
            } else {
                g_task_return_boolean((*close_data).task, TRUE);
            }
        }
        safe_c2rust_close_data_free(close_data);
    }
    g_clear_error(&raw mut error);
    g_queue_free_full(
        send_queue,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut MessageToWriteData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_message_to_write_data_free
                as unsafe extern "C" fn(*mut MessageToWriteData) -> (),
        )),
    );
    error = g_error_new(
        g_io_error_quark(),
        G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
        glib_gettext(b"Operation was cancelled\0" as *const u8 as *const gchar),
    );
    safe_c2rust_flush_data_list_complete(pending_flush_attempts, error);
    g_list_free(pending_flush_attempts);
    g_clear_error(&raw mut error);
    safe_c2rust__g_dbus_worker_unref(worker);
}
unsafe extern "C" fn safe_c2rust_continue_writing(mut worker: *mut GDBusWorker) {
    let mut old_message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut new_blob: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut new_blob_size: gsize = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut current_block: u64;
    let mut data: *mut MessageToWriteData = ::core::ptr::null_mut::<MessageToWriteData>();
    let mut flush_async_data: *mut FlushAsyncData = ::core::ptr::null_mut::<FlushAsyncData>();
    loop {
        if ({
            let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
            if (*worker).output_pending as ::core::ffi::c_uint
                == PENDING_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1455 as ::core::ffi::c_int,
                G_STRFUNC,
                b"worker->output_pending == PENDING_NONE\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_mutex_lock(&raw mut (*worker).write_lock);
        data = ::core::ptr::null_mut::<MessageToWriteData>();
        flush_async_data = ::core::ptr::null_mut::<FlushAsyncData>();
        if !(*worker).pending_close_attempts.is_null() {
            let mut input: *mut GInputStream = g_io_stream_get_input_stream((*worker).stream);
            if g_input_stream_has_pending(input) == 0 {
                (*worker).close_expected = TRUE as gboolean;
                (*worker).output_pending = PENDING_CLOSE;
                g_io_stream_close_async(
                    (*worker).stream,
                    G_PRIORITY_DEFAULT,
                    ::core::ptr::null_mut::<GCancellable>(),
                    Some(
                        safe_c2rust_iostream_close_cb
                            as unsafe extern "C" fn(
                                *mut GObject,
                                *mut GAsyncResult,
                                gpointer,
                            ) -> (),
                    ),
                    safe_c2rust__g_dbus_worker_ref(worker) as gpointer,
                );
            }
        } else {
            flush_async_data = safe_c2rust_prepare_flush_unlocked(worker);
            if flush_async_data.is_null() {
                data = g_queue_pop_head((*worker).write_queue) as *mut MessageToWriteData;
                if !data.is_null() {
                    (*worker).output_pending = PENDING_WRITE;
                }
            }
        }
        g_mutex_unlock(&raw mut (*worker).write_lock);
        if !flush_async_data.is_null() {
            safe_c2rust_start_flush(flush_async_data);
            if ({
                let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
                if data.is_null() {
                    _g_boolean_var_37 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_37 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_37
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1504 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"data == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            current_block = 1847472278776910194;
            break;
        } else {
            if data.is_null() {
                current_block = 1847472278776910194;
                break;
            }
            old_message = ::core::ptr::null_mut::<GDBusMessage>();
            new_blob = ::core::ptr::null_mut::<guchar>();
            new_blob_size = 0;
            error = ::core::ptr::null_mut::<GError>();
            old_message = (*data).message;
            (*data).message =
                safe_c2rust__g_dbus_worker_emit_message_about_to_be_sent(worker, (*data).message);
            if (*data).message == old_message {
                current_block = 5330834795799507926;
                break;
            }
            if (*data).message.is_null() {
                g_mutex_lock(&raw mut (*worker).write_lock);
                (*worker).output_pending = PENDING_NONE;
                g_mutex_unlock(&raw mut (*worker).write_lock);
                safe_c2rust_message_to_write_data_free(data);
            } else {
                error = ::core::ptr::null_mut::<GError>();
                new_blob = g_dbus_message_to_blob(
                    (*data).message,
                    &raw mut new_blob_size,
                    (*worker).capabilities,
                    &raw mut error,
                );
                if new_blob.is_null() {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"Error encoding GDBusMessage with serial %d altered by filter function: %s\0"
                            as *const u8 as *const gchar,
                        g_dbus_message_get_serial((*data).message),
                        (*error).message,
                    );
                    g_error_free(error);
                } else {
                    g_free((*data).blob as gpointer);
                    (*data).blob = new_blob as *mut gchar;
                    (*data).blob_size = new_blob_size;
                }
                current_block = 5330834795799507926;
                break;
            }
        }
    }
    match current_block {
        5330834795799507926 => {
            safe_c2rust_write_message_async(
                worker,
                data,
                Some(
                    safe_c2rust_write_message_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                data as gpointer,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn safe_c2rust_continue_writing_in_idle_cb(mut user_data: gpointer) -> gboolean {
    let mut worker: *mut GDBusWorker = user_data as *mut GDBusWorker;
    if (*worker).output_pending as ::core::ffi::c_uint
        == PENDING_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_continue_writing(worker);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_schedule_writing_unlocked(
    mut worker: *mut GDBusWorker,
    mut write_data: *mut MessageToWriteData,
    mut flush_data: *mut FlushData,
    mut close_data: *mut CloseData,
) {
    if !write_data.is_null() {
        g_queue_push_tail((*worker).write_queue, write_data as gpointer);
    }
    if !flush_data.is_null() {
        (*worker).write_pending_flushes =
            g_list_prepend((*worker).write_pending_flushes, flush_data as gpointer);
    }
    if !close_data.is_null() {
        (*worker).pending_close_attempts =
            g_list_prepend((*worker).pending_close_attempts, close_data as gpointer);
    }
    if (*worker).output_pending as ::core::ffi::c_uint
        == PENDING_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
        idle_source = g_idle_source_new();
        g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
        g_source_set_callback(
            idle_source,
            Some(
                safe_c2rust_continue_writing_in_idle_cb
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            safe_c2rust__g_dbus_worker_ref(worker) as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusWorker) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust__g_dbus_worker_unref as unsafe extern "C" fn(*mut GDBusWorker) -> (),
            )),
        );
        g_source_set_static_name(
            idle_source,
            b"[gio] continue_writing_in_idle_cb\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_source_attach(idle_source, (*(*worker).shared_thread_data).context);
        g_source_unref(idle_source);
    }
}
unsafe extern "C" fn safe_c2rust_schedule_pending_close(mut worker: *mut GDBusWorker) {
    g_mutex_lock(&raw mut (*worker).write_lock);
    if !(*worker).pending_close_attempts.is_null() {
        safe_c2rust_schedule_writing_unlocked(
            worker,
            ::core::ptr::null_mut::<MessageToWriteData>(),
            ::core::ptr::null_mut::<FlushData>(),
            ::core::ptr::null_mut::<CloseData>(),
        );
    }
    g_mutex_unlock(&raw mut (*worker).write_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_worker_send_message(
    mut worker: *mut GDBusWorker,
    mut message: *mut GDBusMessage,
    mut blob: *mut gchar,
    mut blob_len: gsize,
) {
    let mut data: *mut MessageToWriteData = ::core::ptr::null_mut::<MessageToWriteData>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !blob.is_null() {
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
            b"blob != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if blob_len > 16 as gsize {
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
            b"blob_len > 16\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<MessageToWriteData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut MessageToWriteData;
    (*data).worker = safe_c2rust__g_dbus_worker_ref(worker);
    (*data).message = g_object_ref(message as gpointer) as *mut GDBusMessage as *mut GDBusMessage;
    (*data).blob = blob;
    (*data).blob_size = blob_len;
    g_mutex_lock(&raw mut (*worker).write_lock);
    safe_c2rust_schedule_writing_unlocked(
        worker,
        data,
        ::core::ptr::null_mut::<FlushData>(),
        ::core::ptr::null_mut::<CloseData>(),
    );
    g_mutex_unlock(&raw mut (*worker).write_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_worker_new(
    mut stream: *mut GIOStream,
    mut capabilities: GDBusCapabilityFlags,
    mut initially_frozen: gboolean,
    mut message_received_callback: GDBusWorkerMessageReceivedCallback,
    mut message_about_to_be_sent_callback: GDBusWorkerMessageAboutToBeSentCallback,
    mut disconnected_callback: GDBusWorkerDisconnectedCallback,
    mut user_data: gpointer,
) -> *mut GDBusWorker {
    let mut worker: *mut GDBusWorker = ::core::ptr::null_mut::<GDBusWorker>();
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = g_io_stream_get_type();
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
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusWorker>();
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if message_received_callback.is_some() {
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
            b"message_received_callback != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusWorker>();
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if message_about_to_be_sent_callback.is_some() {
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
            b"message_about_to_be_sent_callback != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusWorker>();
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if disconnected_callback.is_some() {
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
            b"disconnected_callback != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusWorker>();
    }
    worker = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GDBusWorker>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GDBusWorker;
    (*worker).ref_count = 1 as ::core::ffi::c_int as gint;
    g_mutex_init(&raw mut (*worker).read_lock);
    (*worker).message_received_callback = message_received_callback;
    (*worker).message_about_to_be_sent_callback = message_about_to_be_sent_callback;
    (*worker).disconnected_callback = disconnected_callback;
    (*worker).user_data = user_data;
    (*worker).stream = g_object_ref(stream as gpointer) as *mut GIOStream as *mut GIOStream;
    (*worker).capabilities = capabilities;
    (*worker).cancellable = g_cancellable_new();
    (*worker).output_pending = PENDING_NONE;
    (*worker).frozen = initially_frozen;
    (*worker).received_messages_while_frozen = g_queue_new();
    g_mutex_init(&raw mut (*worker).write_lock);
    (*worker).write_queue = g_queue_new();
    if ({
        let mut __inst: *mut GTypeInstance = (*worker).stream as *mut GTypeInstance;
        let mut __t: GType = g_socket_connection_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        (*worker).socket = g_socket_connection_get_socket(
            (*worker).stream as *mut ::core::ffi::c_void as *mut GSocketConnection,
        );
    }
    (*worker).shared_thread_data = safe_c2rust__g_dbus_shared_thread_ref();
    idle_source = g_idle_source_new();
    g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        idle_source,
        Some(
            safe_c2rust__g_dbus_worker_do_initial_read
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        safe_c2rust__g_dbus_worker_ref(worker) as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusWorker) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust__g_dbus_worker_unref as unsafe extern "C" fn(*mut GDBusWorker) -> (),
        )),
    );
    g_source_set_static_name(
        idle_source,
        b"[gio] _g_dbus_worker_do_initial_read\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(idle_source, (*(*worker).shared_thread_data).context);
    g_source_unref(idle_source);
    return worker;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_worker_close(
    mut worker: *mut GDBusWorker,
    mut task: *mut GTask,
) {
    let mut close_data: *mut CloseData = ::core::ptr::null_mut::<CloseData>();
    close_data = ({
        let mut __s: gsize = ::core::mem::size_of::<CloseData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut CloseData;
    (*close_data).worker = safe_c2rust__g_dbus_worker_ref(worker);
    (*close_data).task = (if task.is_null() {
        ::core::ptr::null_mut::<GTask>()
    } else {
        g_object_ref(task as gpointer) as *mut GTask
    }) as *mut GTask;
    g_cancellable_cancel((*worker).cancellable);
    g_mutex_lock(&raw mut (*worker).write_lock);
    safe_c2rust_schedule_writing_unlocked(
        worker,
        ::core::ptr::null_mut::<MessageToWriteData>(),
        ::core::ptr::null_mut::<FlushData>(),
        close_data,
    );
    g_mutex_unlock(&raw mut (*worker).write_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_worker_stop(mut worker: *mut GDBusWorker) {
    let mut gais_temp: gint = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*worker).stopped;
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(&raw mut (*worker).stopped, *&raw mut gais_temp);
    safe_c2rust__g_dbus_worker_close(worker, ::core::ptr::null_mut::<GTask>());
    safe_c2rust__g_dbus_worker_unref(worker);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_worker_flush_sync(
    mut worker: *mut GDBusWorker,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = 0;
    let mut data: *mut FlushData = ::core::ptr::null_mut::<FlushData>();
    let mut pending_writes: guint64 = 0;
    data = ::core::ptr::null_mut::<FlushData>();
    ret = TRUE as gboolean;
    g_mutex_lock(&raw mut (*worker).write_lock);
    pending_writes = g_queue_get_length((*worker).write_queue) as guint64;
    if (*worker).output_pending as ::core::ffi::c_uint
        == PENDING_WRITE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pending_writes = pending_writes.wrapping_add(1 as guint64);
    }
    if pending_writes > 0 as guint64
        || (*worker).write_num_messages_written != (*worker).write_num_messages_flushed
    {
        data = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<FlushData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut FlushData;
        g_mutex_init(&raw mut (*data).mutex);
        g_cond_init(&raw mut (*data).cond);
        (*data).number_to_wait_for = (*worker)
            .write_num_messages_written
            .wrapping_add(pending_writes);
        (*data).finished = FALSE as gboolean;
        g_mutex_lock(&raw mut (*data).mutex);
        safe_c2rust_schedule_writing_unlocked(
            worker,
            ::core::ptr::null_mut::<MessageToWriteData>(),
            data,
            ::core::ptr::null_mut::<CloseData>(),
        );
    }
    g_mutex_unlock(&raw mut (*worker).write_lock);
    if !data.is_null() {
        while (*data).finished == 0 {
            g_cond_wait(&raw mut (*data).cond, &raw mut (*data).mutex);
        }
        g_mutex_unlock(&raw mut (*data).mutex);
        g_cond_clear(&raw mut (*data).cond);
        g_mutex_clear(&raw mut (*data).mutex);
        if !(*data).error.is_null() {
            ret = FALSE as gboolean;
            g_propagate_error(error, (*data).error);
        }
        g_free(data as gpointer);
    }
    return ret;
}
pub const G_DBUS_DEBUG_AUTHENTICATION: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_TRANSPORT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_MESSAGE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_PAYLOAD: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_CALL: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_SIGNAL: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_INCOMING: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_RETURN: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_EMISSION: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_ADDRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const G_DBUS_DEBUG_PROXY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
static mut safe_c2rust__gdbus_debug_flags: gint = 0 as gint;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_authentication() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_AUTHENTICATION
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_transport() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_TRANSPORT
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_message() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_MESSAGE
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_payload() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_PAYLOAD
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_call() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_CALL
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_signal() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_SIGNAL
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_incoming() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_INCOMING
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_return() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_RETURN
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_emission() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_EMISSION
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_address() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_ADDRESS
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_proxy() -> gboolean {
    safe_c2rust__g_dbus_initialize();
    return (safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_PROXY
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
static mut safe_c2rust_g__print_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_print_lock() {
    g_mutex_lock(&raw mut safe_c2rust_g__print_lock_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_debug_print_unlock() {
    g_mutex_unlock(&raw mut safe_c2rust_g__print_lock_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_initialize() {
    static mut safe_c2rust_initialized: gsize = 0 as gsize;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialized;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        let mut debug: *const gchar = ::core::ptr::null::<gchar>();
        g_dbus_error_quark();
        debug = g_getenv(b"G_DBUS_DEBUG\0" as *const u8 as *const gchar);
        if !debug.is_null() {
            let keys: [GDebugKey; 11] = [
                _GDebugKey {
                    key: b"authentication\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_AUTHENTICATION as guint,
                },
                _GDebugKey {
                    key: b"transport\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_TRANSPORT as guint,
                },
                _GDebugKey {
                    key: b"message\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_MESSAGE as guint,
                },
                _GDebugKey {
                    key: b"payload\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_PAYLOAD as guint,
                },
                _GDebugKey {
                    key: b"call\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_CALL as guint,
                },
                _GDebugKey {
                    key: b"signal\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_SIGNAL as guint,
                },
                _GDebugKey {
                    key: b"incoming\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_INCOMING as guint,
                },
                _GDebugKey {
                    key: b"return\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_RETURN as guint,
                },
                _GDebugKey {
                    key: b"emission\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_EMISSION as guint,
                },
                _GDebugKey {
                    key: b"address\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_ADDRESS as guint,
                },
                _GDebugKey {
                    key: b"proxy\0" as *const u8 as *const gchar,
                    value: G_DBUS_DEBUG_PROXY as guint,
                },
            ];
            safe_c2rust__gdbus_debug_flags = g_parse_debug_string(
                debug,
                &raw const keys as *const GDebugKey,
                (::core::mem::size_of::<[GDebugKey; 11]>() as usize)
                    .wrapping_div(::core::mem::size_of::<GDebugKey>() as usize)
                    as guint,
            ) as gint;
            if safe_c2rust__gdbus_debug_flags as ::core::ffi::c_int & G_DBUS_DEBUG_PAYLOAD != 0 {
                safe_c2rust__gdbus_debug_flags |= G_DBUS_DEBUG_MESSAGE;
            }
        }
        safe_c2rust_ensure_required_types();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized = 1 as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void,
            1 as ::core::ffi::c_int as gsize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_compute_complete_signature(
    mut args: *mut *mut GDBusArgInfo,
) -> *mut GVariantType {
    let mut arg_types: [*const GVariantType; 256] = [::core::ptr::null::<GVariantType>(); 256];
    let mut n: guint = 0;
    if !args.is_null() {
        n = 0 as guint;
        while !(*args.offset(n as isize)).is_null() {
            if ({
                let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
                if n < 256 as guint {
                    _g_boolean_var_45 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_45 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_45
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusprivate.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2014 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"n < 256\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            arg_types[n as usize] = g_variant_type_checked_((**args.offset(n as isize)).signature);
            if ({
                let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
                if arg_types[n as usize].is_null() {
                    _g_boolean_var_46 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_46 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_46
            }) as ::core::ffi::c_long
                != 0
            {
                return ::core::ptr::null_mut::<GVariantType>();
            }
            n = n.wrapping_add(1);
        }
    } else {
        n = 0 as guint;
    }
    return g_variant_type_new_tuple(&raw mut arg_types as *mut *const GVariantType, n as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_get_machine_id(
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut first_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut i: gsize = 0;
    let mut non_zero: gboolean = FALSE;
    let mut var_lib_path: *const gchar =
        b"/var/local/lib/dbus/machine-id\0" as *const u8 as *const gchar;
    let mut etc_path: *const gchar = b"/etc/machine-id\0" as *const u8 as *const gchar;
    if g_file_get_contents(
        var_lib_path,
        &raw mut ret,
        ::core::ptr::null_mut::<gsize>(),
        &raw mut first_error,
    ) == 0
        && g_file_get_contents(
            etc_path,
            &raw mut ret,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) == 0
    {
        g_propagate_prefixed_error(
            error,
            safe_c2rust_g_steal_pointer(&raw mut first_error as gpointer) as *mut GError,
            glib_gettext(b"Unable to load %s or %s: \0" as *const u8 as *const gchar),
            var_lib_path,
            etc_path,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_clear_error(&raw mut first_error);
    i = 0 as gsize;
    while *ret.offset(i as isize) as ::core::ffi::c_int != '\0' as i32
        && *ret.offset(i as isize) as ::core::ffi::c_int != '\n' as i32
    {
        if !(*safe_c2rust_g_ascii_table.offset(*ret.offset(i as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_XDIGIT as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
            || *safe_c2rust_g_ascii_table.offset(*ret.offset(i as isize) as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_UPPER as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            break;
        }
        if *ret.offset(i as isize) as ::core::ffi::c_int != '0' as i32 {
            non_zero = TRUE as gboolean;
        }
        i = i.wrapping_add(1);
    }
    if i != 32 as gsize
        || *ret.offset(i as isize) as ::core::ffi::c_int != '\n' as i32
        || *ret.offset(i.wrapping_add(1 as gsize) as isize) as ::core::ffi::c_int != '\0' as i32
        || non_zero == 0
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            b"Invalid machine ID in %s or %s\0" as *const u8 as *const gchar,
            var_lib_path,
            etc_path,
        );
        g_free(ret as gpointer);
        return ::core::ptr::null_mut::<gchar>();
    }
    *ret.offset(32 as ::core::ffi::c_int as isize) = '\0' as i32 as gchar;
    return safe_c2rust_g_steal_pointer(&raw mut ret as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_enum_to_string(
    mut enum_type: GType,
    mut value: gint,
) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut klass: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
    let mut enum_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
    klass = g_type_class_ref(enum_type) as *mut GEnumClass;
    enum_value = g_enum_get_value(klass, value);
    if !enum_value.is_null() {
        ret = safe_c2rust_g_strdup_inline((*enum_value).value_nick as *const ::core::ffi::c_char)
            as *mut gchar;
    } else {
        ret = g_strdup_printf(b"unknown (value %d)\0" as *const u8 as *const gchar, value);
    }
    g_type_class_unref(klass as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_write_message_print_transport_debug(
    mut bytes_written: gssize,
    mut data: *mut MessageToWriteData,
) {
    if !(({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if safe_c2rust__g_dbus_debug_transport() == 0 {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0)
    {
        safe_c2rust__g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Transport:\n  >>>> WROTE %li bytes of message with serial %d and\n       size %lu from offset %lu on a %s\n\0"
                as *const u8 as *const gchar,
            bytes_written,
            g_dbus_message_get_serial((*data).message),
            (*data).blob_size,
            (*data).total_written,
            g_type_name(
                (*(*(g_io_stream_get_output_stream((*(*data).worker).stream)
                    as *mut GTypeInstance))
                    .g_class)
                    .g_type,
            ),
        );
        safe_c2rust__g_dbus_debug_print_unlock();
    }
}
unsafe extern "C" fn safe_c2rust_read_message_print_transport_debug(
    mut bytes_read: gssize,
    mut worker: *mut GDBusWorker,
) {
    let mut current_block: u64;
    let mut size: gsize = 0;
    let mut serial: gint32 = 0;
    let mut message_length: gint32 = 0;
    if !(({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if safe_c2rust__g_dbus_debug_transport() == 0 {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0)
    {
        size = (bytes_read as gsize).wrapping_add((*worker).read_buffer_cur_size);
        serial = 0 as ::core::ffi::c_int as gint32;
        message_length = 0 as ::core::ffi::c_int as gint32;
        if size >= 16 as gsize {
            message_length = g_dbus_message_bytes_needed(
                (*worker).read_buffer as *mut guchar,
                size,
                ::core::ptr::null_mut::<*mut GError>(),
            ) as gint32;
        }
        if size >= 1 as gsize {
            match *(*worker)
                .read_buffer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            {
                108 => {
                    current_block = 13482896777352746964;
                    match current_block {
                        4072105254962629116 => {
                            if size >= 12 as gsize {
                                serial = ({
                                    let mut __v: guint32 = 0;
                                    let mut __x: guint32 = *((*worker).read_buffer as *mut guint32)
                                        .offset(2 as ::core::ffi::c_int as isize);
                                    if 0 != 0 {
                                        __v = (__x & 0xff as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int
                                            | (__x & 0xff00 as ::core::ffi::c_uint)
                                                << 8 as ::core::ffi::c_int
                                            | (__x & 0xff0000 as ::core::ffi::c_uint)
                                                >> 8 as ::core::ffi::c_int
                                            | (__x & 0xff000000 as ::core::ffi::c_uint)
                                                >> 24 as ::core::ffi::c_int;
                                    } else {
                                        let fresh2 = &mut __v;
                                        let fresh3;
                                        let fresh4 = __x;
                                        asm!(
                                            "bswapl {0:e}\n", inlateout(reg)
                                            c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh4) =>
                                            fresh3, options(preserves_flags, pure, readonly, att_syntax)
                                        );
                                        c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
                                    }
                                    __v
                                }) as gint32;
                            }
                        }
                        _ => {
                            if size >= 12 as gsize {
                                serial = *((*worker).read_buffer as *mut guint32)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as gint32;
                            }
                        }
                    }
                    current_block = 13586036798005543211;
                }
                66 => {
                    current_block = 4072105254962629116;
                    match current_block {
                        4072105254962629116 => {
                            if size >= 12 as gsize {
                                serial = ({
                                    let mut __v: guint32 = 0;
                                    let mut __x: guint32 = *((*worker).read_buffer as *mut guint32)
                                        .offset(2 as ::core::ffi::c_int as isize);
                                    if 0 != 0 {
                                        __v = (__x & 0xff as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int
                                            | (__x & 0xff00 as ::core::ffi::c_uint)
                                                << 8 as ::core::ffi::c_int
                                            | (__x & 0xff0000 as ::core::ffi::c_uint)
                                                >> 8 as ::core::ffi::c_int
                                            | (__x & 0xff000000 as ::core::ffi::c_uint)
                                                >> 24 as ::core::ffi::c_int;
                                    } else {
                                        let fresh2 = &mut __v;
                                        let fresh3;
                                        let fresh4 = __x;
                                        asm!(
                                            "bswapl {0:e}\n", inlateout(reg)
                                            c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh4) =>
                                            fresh3, options(preserves_flags, pure, readonly, att_syntax)
                                        );
                                        c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
                                    }
                                    __v
                                }) as gint32;
                            }
                        }
                        _ => {
                            if size >= 12 as gsize {
                                serial = *((*worker).read_buffer as *mut guint32)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as gint32;
                            }
                        }
                    }
                    current_block = 13586036798005543211;
                }
                _ => {
                    current_block = 1054647088692577877;
                }
            }
        } else {
            current_block = 13586036798005543211;
        }
        match current_block {
            1054647088692577877 => {}
            _ => {
                safe_c2rust__g_dbus_debug_print_lock();
                g_print(
                    b"========================================================================\nGDBus-debug:Transport:\n  <<<< READ %li bytes of message with serial %d and\n       size %d to offset %lu from a %s\n\0"
                        as *const u8 as *const gchar,
                    bytes_read,
                    serial,
                    message_length,
                    (*worker).read_buffer_cur_size,
                    g_type_name(
                        (*(*(g_io_stream_get_input_stream((*worker).stream)
                            as *mut GTypeInstance))
                            .g_class)
                            .g_type,
                    ),
                );
                safe_c2rust__g_dbus_debug_print_unlock();
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_signal_accumulator_false_handled(
    mut ihint: *mut GSignalInvocationHint,
    mut return_accu: *mut GValue,
    mut handler_return: *const GValue,
    mut dummy: gpointer,
) -> gboolean {
    let mut continue_emission: gboolean = 0;
    let mut signal_return: gboolean = 0;
    signal_return = g_value_get_boolean(handler_return);
    g_value_set_boolean(return_accu, signal_return);
    continue_emission = signal_return;
    return continue_emission;
}
unsafe extern "C" fn safe_c2rust_append_nibble(mut s: *mut GString, mut val: gint) {
    safe_c2rust_g_string_append_c_inline(
        s,
        (if val >= 10 as ::core::ffi::c_int {
            'a' as i32 + val as ::core::ffi::c_int - 10 as ::core::ffi::c_int
        } else {
            '0' as i32 + val as ::core::ffi::c_int
        }) as gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_hexencode(
    mut str: *const gchar,
    mut str_len: gsize,
) -> *mut gchar {
    let mut n: gsize = 0;
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    s = g_string_new(::core::ptr::null::<gchar>());
    n = 0 as gsize;
    while n < str_len {
        let mut val: gint = 0;
        let mut upper_nibble: gint = 0;
        let mut lower_nibble: gint = 0;
        val = *(str as *const guchar).offset(n as isize) as gint;
        upper_nibble = val >> 4 as ::core::ffi::c_int;
        lower_nibble = (val as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as gint;
        safe_c2rust_append_nibble(s, upper_nibble);
        safe_c2rust_append_nibble(s, lower_nibble);
        n = n.wrapping_add(1);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(s, 0 as gboolean)
        } else {
            g_string_free_and_steal(s)
        }
    } else {
        g_string_free(s, 0 as gboolean)
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
