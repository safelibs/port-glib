use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GIConv;
    pub type _GData;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _GSourcePrivate;
    pub type _GDBusConnection;
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut safe_c2rust_stdout: *mut FILE;
    static mut safe_c2rust_stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_unref(array: *mut GArray);
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_array_remove_index(array: *mut GArray, index_: guint) -> *mut GArray;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe2(__pipedes: *mut ::core::ffi::c_int, __flags: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn fork() -> __pid_t;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_setenv(variable: *const gchar, value: *const gchar, overwrite: gboolean) -> gboolean;
    fn g_unsetenv(variable: *const gchar);
    fn g_file_set_contents_full(
        filename: *const gchar,
        contents: *const gchar,
        length: gssize,
        flags: GFileSetContentsFlags,
        mode: ::core::ffi::c_int,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_open_tmp(
        tmpl: *const gchar,
        name_used: *mut *mut gchar,
        error: *mut *mut GError,
    ) -> gint;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_poll(fds: *mut GPollFD, nfds: guint, timeout: gint) -> gint;
    fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    fn g_main_loop_run(loop_0: *mut GMainLoop);
    fn g_main_loop_quit(loop_0: *mut GMainLoop);
    fn g_main_loop_unref(loop_0: *mut GMainLoop);
    fn g_source_remove(tag: guint) -> gboolean;
    fn g_timeout_add_seconds_once(
        interval: guint,
        function: GSourceOnceFunc,
        data: gpointer,
    ) -> guint;
    fn g_idle_add(function: GSourceFunc, data: gpointer) -> guint;
    fn g_strerror(errnum: gint) -> *const gchar;
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
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_io_channel_unref(channel: *mut GIOChannel);
    fn g_io_channel_shutdown(
        channel: *mut GIOChannel,
        flush: gboolean,
        err: *mut *mut GError,
    ) -> GIOStatus;
    fn g_io_channel_set_close_on_unref(channel: *mut GIOChannel, do_close: gboolean);
    fn g_io_channel_flush(channel: *mut GIOChannel, error: *mut *mut GError) -> GIOStatus;
    fn g_io_channel_read_line(
        channel: *mut GIOChannel,
        str_return: *mut *mut gchar,
        length: *mut gsize,
        terminator_pos: *mut gsize,
        error: *mut *mut GError,
    ) -> GIOStatus;
    fn g_io_channel_write_chars(
        channel: *mut GIOChannel,
        buf: *const gchar,
        count: gssize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> GIOStatus;
    fn g_io_channel_unix_new(fd: ::core::ffi::c_int) -> *mut GIOChannel;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_printerr(format: *const gchar, ...);
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
    fn g_spawn_command_line_async(command_line: *const gchar, error: *mut *mut GError) -> gboolean;
    fn g_spawn_close_pid(pid: GPid);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_error(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        error: *const GError,
        error_domain: GQuark,
        error_code: ::core::ffi::c_int,
    );
    fn g_usleep(microseconds: gulong);
    fn g_unlink(filename: *const gchar) -> ::core::ffi::c_int;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_weak_ref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_dbus_connection_set_exit_on_close(
        connection: *mut GDBusConnection,
        exit_on_close: gboolean,
    );
    fn _g_bus_get_singleton_if_exists(bus_type: GBusType) -> *mut GDBusConnection;
    fn _g_bus_forget_singleton(bus_type: GBusType);
    fn g_test_dbus_flags_get_type() -> GType;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn g_unix_open_pipe(fds: *mut gint, flags: gint, error: *mut *mut GError) -> gboolean;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type GPid = ::core::ffi::c_int;
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
pub type GIConv = *mut _GIConv;
pub type GData = _GData;
pub type GFileSetContentsFlags = ::core::ffi::c_uint;
pub const G_FILE_SET_CONTENTS_ONLY_EXISTING: GFileSetContentsFlags = 4;
pub const G_FILE_SET_CONTENTS_DURABLE: GFileSetContentsFlags = 2;
pub const G_FILE_SET_CONTENTS_CONSISTENT: GFileSetContentsFlags = 1;
pub const G_FILE_SET_CONTENTS_NONE: GFileSetContentsFlags = 0;
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
pub type GSourceOnceFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GIOChannel {
    pub ref_count: gint,
    pub funcs: *mut GIOFuncs,
    pub encoding: *mut gchar,
    pub read_cd: GIConv,
    pub write_cd: GIConv,
    pub line_term: *mut gchar,
    pub line_term_len: guint,
    pub buf_size: gsize,
    pub read_buf: *mut GString,
    pub encoded_read_buf: *mut GString,
    pub write_buf: *mut GString,
    pub partial_write_buf: [gchar; 6],
    #[bitfield(name = "use_buffer", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "do_encode", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "close_on_unref", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "is_readable", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "is_writeable", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "is_seekable", ty = "guint", bits = "5..=5")]
    pub use_buffer_do_encode_close_on_unref_is_readable_is_writeable_is_seekable: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub reserved1: gpointer,
    pub reserved2: gpointer,
}
pub type GIOFuncs = _GIOFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOFuncs {
    pub io_read: Option<
        unsafe extern "C" fn(
            *mut GIOChannel,
            *mut gchar,
            gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GIOStatus,
    >,
    pub io_write: Option<
        unsafe extern "C" fn(
            *mut GIOChannel,
            *const gchar,
            gsize,
            *mut gsize,
            *mut *mut GError,
        ) -> GIOStatus,
    >,
    pub io_seek: Option<
        unsafe extern "C" fn(*mut GIOChannel, gint64, GSeekType, *mut *mut GError) -> GIOStatus,
    >,
    pub io_close: Option<unsafe extern "C" fn(*mut GIOChannel, *mut *mut GError) -> GIOStatus>,
    pub io_create_watch:
        Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition) -> *mut GSource>,
    pub io_free: Option<unsafe extern "C" fn(*mut GIOChannel) -> ()>,
    pub io_set_flags:
        Option<unsafe extern "C" fn(*mut GIOChannel, GIOFlags, *mut *mut GError) -> GIOStatus>,
    pub io_get_flags: Option<unsafe extern "C" fn(*mut GIOChannel) -> GIOFlags>,
}
pub type GIOChannel = _GIOChannel;
pub type GIOFlags = ::core::ffi::c_uint;
pub const G_IO_FLAG_SET_MASK: GIOFlags = 3;
pub const G_IO_FLAG_GET_MASK: GIOFlags = 31;
pub const G_IO_FLAG_MASK: GIOFlags = 31;
pub const G_IO_FLAG_IS_SEEKABLE: GIOFlags = 16;
pub const G_IO_FLAG_IS_WRITEABLE: GIOFlags = 8;
pub const G_IO_FLAG_IS_WRITABLE: GIOFlags = 8;
pub const G_IO_FLAG_IS_READABLE: GIOFlags = 4;
pub const G_IO_FLAG_NONBLOCK: GIOFlags = 2;
pub const G_IO_FLAG_APPEND: GIOFlags = 1;
pub const G_IO_FLAG_NONE: GIOFlags = 0;
pub type GIOStatus = ::core::ffi::c_uint;
pub const G_IO_STATUS_AGAIN: GIOStatus = 3;
pub const G_IO_STATUS_EOF: GIOStatus = 2;
pub const G_IO_STATUS_NORMAL: GIOStatus = 1;
pub const G_IO_STATUS_ERROR: GIOStatus = 0;
pub type GSeekType = ::core::ffi::c_uint;
pub const G_SEEK_END: GSeekType = 2;
pub const G_SEEK_SET: GSeekType = 1;
pub const G_SEEK_CUR: GSeekType = 0;
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
pub type GWeakNotify = Option<unsafe extern "C" fn(gpointer, *mut GObject) -> ()>;
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GTestDBusFlags = ::core::ffi::c_uint;
pub const G_TEST_DBUS_NONE: GTestDBusFlags = 0;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTestDBus {
    pub parent: GObject,
    pub priv_0: *mut GTestDBusPrivate,
}
pub type GTestDBusPrivate = _GTestDBusPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTestDBusPrivate {
    pub flags: GTestDBusFlags,
    pub service_dirs: *mut GPtrArray,
    pub bus_pid: GPid,
    pub bus_address: *mut gchar,
    pub up: gboolean,
}
pub type GTestDBus = _GTestDBus;
pub type GTestDBusClass = _GTestDBusClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTestDBusClass {
    pub parent_class: GObjectClass,
}
pub const PROP_FLAGS: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WeakNotifyData {
    pub loop_0: *mut GMainLoop,
    pub timed_out: gboolean,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
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
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_open_pipe_internal(
    mut fds: *mut ::core::ffi::c_int,
    mut close_on_exec: gboolean,
    mut nonblock: gboolean,
) -> gboolean {
    let mut ecode: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if close_on_exec != 0 {
        flags |= O_CLOEXEC;
    }
    if nonblock != 0 {
        flags |= O_NONBLOCK;
    }
    ecode = pipe2(fds as *mut ::core::ffi::c_int, flags);
    if ecode == -(1 as ::core::ffi::c_int) && *__errno_location() != ENOSYS {
        return FALSE;
    } else if ecode == 0 as ::core::ffi::c_int {
        return TRUE;
    }
    if pipe(fds as *mut ::core::ffi::c_int) == -(1 as ::core::ffi::c_int) {
        return FALSE;
    }
    if close_on_exec != 0 {
        if fcntl(
            *fds.offset(0 as ::core::ffi::c_int as isize),
            F_SETFD,
            FD_CLOEXEC,
        ) == -(1 as ::core::ffi::c_int)
            || fcntl(
                *fds.offset(1 as ::core::ffi::c_int as isize),
                F_SETFD,
                FD_CLOEXEC,
            ) == -(1 as ::core::ffi::c_int)
        {
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            close(*fds.offset(0 as ::core::ffi::c_int as isize));
            close(*fds.offset(1 as ::core::ffi::c_int as isize));
            *fds.offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *fds.offset(1 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *__errno_location() = saved_errno;
            return FALSE;
        }
    }
    if nonblock != 0 {
        let mut flags_0: ::core::ffi::c_int = O_NONBLOCK;
        if fcntl(
            *fds.offset(0 as ::core::ffi::c_int as isize),
            F_SETFL,
            flags_0,
        ) == -(1 as ::core::ffi::c_int)
            || fcntl(
                *fds.offset(1 as ::core::ffi::c_int as isize),
                F_SETFL,
                flags_0,
            ) == -(1 as ::core::ffi::c_int)
        {
            let mut saved_errno_0: ::core::ffi::c_int = *__errno_location();
            close(*fds.offset(0 as ::core::ffi::c_int as isize));
            close(*fds.offset(1 as ::core::ffi::c_int as isize));
            *fds.offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *fds.offset(1 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            *__errno_location() = saved_errno_0;
            return FALSE;
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_on_weak_notify_timeout(mut user_data: gpointer) {
    let mut data: *mut WeakNotifyData = user_data as *mut WeakNotifyData;
    (*data).timed_out = TRUE as gboolean;
    g_main_loop_quit((*data).loop_0);
}
unsafe extern "C" fn safe_c2rust_unref_on_idle(mut object: gpointer) -> gboolean {
    g_object_unref(object);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__g_object_unref_and_wait_weak_notify(
    mut object: gpointer,
) -> gboolean {
    let mut data: WeakNotifyData = WeakNotifyData {
        loop_0: ::core::ptr::null_mut::<GMainLoop>(),
        timed_out: 0,
    };
    let mut timeout_id: guint = 0;
    data.loop_0 = g_main_loop_new(::core::ptr::null_mut::<GMainContext>(), FALSE);
    data.timed_out = FALSE as gboolean;
    g_object_weak_ref(
        object as *mut GObject,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GMainLoop) -> ()>, GWeakNotify>(
            Some(g_main_loop_quit as unsafe extern "C" fn(*mut GMainLoop) -> ()),
        ),
        data.loop_0 as gpointer,
    );
    g_idle_add(
        Some(safe_c2rust_unref_on_idle as unsafe extern "C" fn(gpointer) -> gboolean),
        object,
    );
    timeout_id = g_timeout_add_seconds_once(
        30 as guint,
        Some(safe_c2rust_on_weak_notify_timeout as unsafe extern "C" fn(gpointer) -> ()),
        &raw mut data as gpointer,
    );
    g_main_loop_run(data.loop_0);
    if data.timed_out != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Weak notify timeout, object ref_count=%d\0" as *const u8 as *const gchar,
            (*(object as *mut GObject)).ref_count,
        );
    } else {
        g_source_remove(timeout_id);
    }
    g_main_loop_unref(data.loop_0);
    return data.timed_out;
}
pub const ADD_PID_FORMAT: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"add pid %d\n\0") };
pub const REMOVE_PID_FORMAT: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"remove pid %d\n\0") };
unsafe extern "C" fn safe_c2rust_watch_parent(mut fd: gint) {
    let mut channel: *mut GIOChannel = ::core::ptr::null_mut::<GIOChannel>();
    let mut fds: [GPollFD; 1] = [_GPollFD {
        fd: 0,
        events: 0,
        revents: 0,
    }; 1];
    let mut pids_to_kill: *mut GArray = ::core::ptr::null_mut::<GArray>();
    channel = g_io_channel_unix_new(fd as ::core::ffi::c_int);
    fds[0 as ::core::ffi::c_int as usize].fd = fd;
    fds[0 as ::core::ffi::c_int as usize].events =
        (G_IO_HUP as ::core::ffi::c_int | G_IO_IN as ::core::ffi::c_int) as gushort;
    fds[0 as ::core::ffi::c_int as usize].revents = 0 as gushort;
    pids_to_kill = g_array_new(FALSE, FALSE, ::core::mem::size_of::<guint>() as guint);
    loop {
        let mut num_events: gint = 0;
        let mut command: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut pid: guint = 0;
        let mut n: guint = 0;
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        num_events = g_poll(&raw mut fds as *mut GPollFD, 1 as guint, -(1 as gint));
        if !(num_events == 0 as ::core::ffi::c_int) {
            if fds[0 as ::core::ffi::c_int as usize].revents as ::core::ffi::c_int
                & G_IO_HUP as ::core::ffi::c_int
                != 0
            {
                n = 0 as guint;
                while n < (*pids_to_kill).len {
                    pid = *((*pids_to_kill).data as *mut ::core::ffi::c_void as *mut guint)
                        .offset(n as isize);
                    g_printerr(b"cleaning up pid %d\n\0" as *const u8 as *const gchar, pid);
                    kill(pid as __pid_t, SIGTERM);
                    n = n.wrapping_add(1);
                }
                g_array_unref(pids_to_kill);
                g_io_channel_shutdown(channel, FALSE, &raw mut error);
                if !error.is_null() {
                    g_assertion_message_error(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        195 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char,
                        error,
                        0 as GQuark,
                        0 as ::core::ffi::c_int,
                    );
                }
                g_io_channel_unref(channel);
                exit(0 as ::core::ffi::c_int);
            }
            g_io_channel_read_line(
                channel,
                &raw mut command,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<gsize>(),
                &raw mut error,
            );
            if !error.is_null() {
                g_assertion_message_error(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char,
                    error,
                    0 as GQuark,
                    0 as ::core::ffi::c_int,
                );
            }
            if sscanf(command, ADD_PID_FORMAT.as_ptr(), &raw mut pid) == 1 as ::core::ffi::c_int {
                g_array_append_vals(pids_to_kill, &raw mut pid as gconstpointer, 1 as guint);
            } else if sscanf(command, REMOVE_PID_FORMAT.as_ptr(), &raw mut pid)
                == 1 as ::core::ffi::c_int
            {
                n = 0 as guint;
                while n < (*pids_to_kill).len {
                    if *((*pids_to_kill).data as *mut ::core::ffi::c_void as *mut guint)
                        .offset(n as isize)
                        == pid
                    {
                        g_array_remove_index(pids_to_kill, n);
                        pid = 0 as guint;
                        break;
                    } else {
                        n = n.wrapping_add(1);
                    }
                }
                if pid != 0 as guint {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"unknown pid %d to remove\0" as *const u8 as *const gchar,
                        pid,
                    );
                }
            } else {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"unknown command from parent '%s'\0" as *const u8 as *const gchar,
                    command,
                );
            }
            g_free(command as gpointer);
        }
        if !(FALSE == 0) {
            break;
        }
    }
}
unsafe extern "C" fn safe_c2rust_watcher_init() -> *mut GIOChannel {
    static mut safe_c2rust_started: gsize = 0 as gsize;
    static mut safe_c2rust_channel: *mut GIOChannel =
        ::core::ptr::null::<GIOChannel>() as *mut GIOChannel;
    let mut errsv: ::core::ffi::c_int = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_started;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_started;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_started as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        let mut pipe_fds: [gint; 2] = [0; 2];
        if safe_c2rust_g_unix_open_pipe_internal(
            &raw mut pipe_fds as *mut ::core::ffi::c_int,
            TRUE,
            FALSE,
        ) == 0
        {
            errsv = *__errno_location();
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"pipe() failed: %s\0" as *const u8 as *const gchar,
                g_strerror(errsv as gint),
            );
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                252 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        fflush(safe_c2rust_stdout);
        fflush(safe_c2rust_stderr);
        match fork() {
            -1 => {
                errsv = *__errno_location();
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"fork() failed: %s\0" as *const u8 as *const gchar,
                    g_strerror(errsv as gint),
                );
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    270 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            0 => {
                close(pipe_fds[1 as ::core::ffi::c_int as usize]);
                safe_c2rust_watch_parent(pipe_fds[0 as ::core::ffi::c_int as usize]);
            }
            _ => {
                close(pipe_fds[0 as ::core::ffi::c_int as usize]);
                safe_c2rust_channel =
                    g_io_channel_unix_new(pipe_fds[1 as ::core::ffi::c_int as usize]);
            }
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_started = 1 as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_started as *mut ::core::ffi::c_void,
            1 as ::core::ffi::c_int as gsize,
        );
    }
    return safe_c2rust_channel;
}
unsafe extern "C" fn safe_c2rust_watcher_send_command(mut command: *const gchar) {
    let mut channel: *mut GIOChannel = ::core::ptr::null_mut::<GIOChannel>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    channel = safe_c2rust_watcher_init();
    loop {
        status = g_io_channel_write_chars(
            channel,
            command,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            &raw mut error,
        );
        if !(status as ::core::ffi::c_uint
            == G_IO_STATUS_AGAIN as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
    }
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            303 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    g_io_channel_flush(channel, &raw mut error);
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            306 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn safe_c2rust__g_test_watcher_add_pid(mut pid: GPid) {
    let mut command: *mut gchar = ::core::ptr::null_mut::<gchar>();
    command = g_strdup_printf(ADD_PID_FORMAT.as_ptr() as *const gchar, pid as guint);
    safe_c2rust_watcher_send_command(command);
    g_free(command as gpointer);
}
unsafe extern "C" fn safe_c2rust__g_test_watcher_remove_pid(mut pid: GPid) {
    let mut command: *mut gchar = ::core::ptr::null_mut::<gchar>();
    command = g_strdup_printf(REMOVE_PID_FORMAT.as_ptr() as *const gchar, pid as guint);
    safe_c2rust_watcher_send_command(command);
    g_free(command as gpointer);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_test_dbus_get_instance_private(
    mut self_0: *mut GTestDBus,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GTestDBus_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_test_dbus_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_test_dbus_get_type_once();
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_test_dbus_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GTestDBus\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GTestDBusClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_test_dbus_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GTestDBus>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTestDBus) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_test_dbus_init as unsafe extern "C" fn(*mut GTestDBus) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GTestDBus_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GTestDBusPrivate>() as gsize,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_test_dbus_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_test_dbus_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GTestDBus_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GTestDBus_private_offset);
    }
    safe_c2rust_g_test_dbus_class_init(klass as *mut GTestDBusClass);
}
static mut safe_c2rust_GTestDBus_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_test_dbus_init(mut self_0: *mut GTestDBus) {
    (*self_0).priv_0 =
        safe_c2rust_g_test_dbus_get_instance_private(self_0) as *mut GTestDBusPrivate;
    (*(*self_0).priv_0).service_dirs =
        g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
}
unsafe extern "C" fn safe_c2rust_g_test_dbus_dispose(mut object: *mut GObject) {
    let mut self_0: *mut GTestDBus = object as *mut GTestDBus;
    if (*(*self_0).priv_0).up != 0 {
        safe_c2rust_g_test_dbus_down(self_0);
    }
    (*(safe_c2rust_g_test_dbus_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_test_dbus_finalize(mut object: *mut GObject) {
    let mut self_0: *mut GTestDBus = object as *mut GTestDBus;
    g_ptr_array_unref((*(*self_0).priv_0).service_dirs);
    g_free((*(*self_0).priv_0).bus_address as gpointer);
    (*(safe_c2rust_g_test_dbus_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_test_dbus_get_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GTestDBus = object as *mut GTestDBus;
    match property_id {
        1 => {
            g_value_set_flags(value, safe_c2rust_g_test_dbus_get_flags(self_0) as guint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                494 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_test_dbus_set_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GTestDBus = object as *mut GTestDBus;
    match property_id {
        1 => {
            (*(*self_0).priv_0).flags = g_value_get_flags(value) as GTestDBusFlags;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                513 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_test_dbus_class_init(mut klass: *mut GTestDBusClass) {
    let mut object_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).dispose =
        Some(safe_c2rust_g_test_dbus_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_test_dbus_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_test_dbus_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_test_dbus_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        object_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_test_dbus_flags_get_type(),
            G_TEST_DBUS_NONE as ::core::ffi::c_int as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_write_config_file(mut self_0: *mut GTestDBus) -> *mut gchar {
    let mut contents: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut fd: gint = 0;
    let mut i: guint = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    fd = g_file_open_tmp(
        b"g-test-dbus-XXXXXX\0" as *const u8 as *const gchar,
        &raw mut path,
        &raw mut error,
    );
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            553 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    contents = g_string_new(::core::ptr::null::<gchar>());
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"<busconfig>\n  <type>session</type>\n  <listen>unix:tmpdir=/tmp</listen>\n\0"
                    as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                contents,
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
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            contents,
            b"<busconfig>\n  <type>session</type>\n  <listen>unix:tmpdir=/tmp</listen>\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    i = 0 as guint;
    while i < (*(*(*self_0).priv_0).service_dirs).len {
        let mut dir_path: *const gchar =
            *(*(*(*self_0).priv_0).service_dirs).pdata.offset(i as isize) as *const gchar;
        g_string_append_printf(
            contents,
            b"  <servicedir>%s</servicedir>\n\0" as *const u8 as *const gchar,
            dir_path,
        );
        i = i.wrapping_add(1);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = b"  <policy context=\"default\">\n    <!-- Allow everything to be sent -->\n    <allow send_destination=\"*\" eavesdrop=\"true\"/>\n    <!-- Allow everything to be received -->\n    <allow eavesdrop=\"true\"/>\n    <!-- Allow anyone to own anything -->\n    <allow own=\"*\"/>\n  </policy>\n</busconfig>\n\0"
                as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                contents,
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
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            contents,
            b"  <policy context=\"default\">\n    <!-- Allow everything to be sent -->\n    <allow send_destination=\"*\" eavesdrop=\"true\"/>\n    <!-- Allow everything to be received -->\n    <allow eavesdrop=\"true\"/>\n    <!-- Allow anyone to own anything -->\n    <allow own=\"*\"/>\n  </policy>\n</busconfig>\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    close(fd as ::core::ffi::c_int);
    g_file_set_contents_full(
        path,
        (*contents).str_0,
        (*contents).len as gssize,
        G_FILE_SET_CONTENTS_NONE,
        0o600 as ::core::ffi::c_int,
        &raw mut error,
    );
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            589 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                contents,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(contents);
        };
    } else {
        g_string_free(
            contents,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
    return path;
}
unsafe extern "C" fn safe_c2rust_make_pipe(
    mut pipe_fds: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_unix_open_pipe(pipe_fds as *mut gint, O_CLOEXEC, error);
}
unsafe extern "C" fn safe_c2rust_start_daemon(mut self_0: *mut GTestDBus) {
    let mut argv: [*const gchar; 4] = [
        b"dbus-daemon\0" as *const u8 as *const ::core::ffi::c_char,
        b"--print-address\0" as *const u8 as *const ::core::ffi::c_char,
        b"--config-file=foo\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<gchar>(),
    ];
    let mut pipe_fds: [gint; 2] = [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)];
    let mut config_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut config_arg: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut print_address: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut channel: *mut GIOChannel = ::core::ptr::null_mut::<GIOChannel>();
    let mut termpos: gsize = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if !g_getenv(b"G_TEST_DBUS_DAEMON\0" as *const u8 as *const gchar).is_null() {
        argv[0 as ::core::ffi::c_int as usize] =
            g_getenv(b"G_TEST_DBUS_DAEMON\0" as *const u8 as *const gchar) as *mut gchar;
    }
    safe_c2rust_make_pipe(&raw mut pipe_fds as *mut gint, &raw mut error);
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            636 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    print_address = g_strdup_printf(
        b"--print-address=%d\0" as *const u8 as *const gchar,
        pipe_fds[1 as ::core::ffi::c_int as usize],
    );
    argv[1 as ::core::ffi::c_int as usize] = print_address;
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            640 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    config_path = safe_c2rust_write_config_file(self_0);
    config_arg = g_strdup_printf(
        b"--config-file=%s\0" as *const u8 as *const gchar,
        config_path,
    );
    argv[2 as ::core::ffi::c_int as usize] = config_arg;
    g_spawn_async_with_pipes_and_fds(
        ::core::ptr::null::<gchar>(),
        &raw mut argv as *mut *const gchar,
        ::core::ptr::null::<*const gchar>(),
        (G_SPAWN_DO_NOT_REAP_CHILD as ::core::ffi::c_int
            | G_SPAWN_SEARCH_PATH as ::core::ffi::c_int
            | G_SPAWN_LEAVE_DESCRIPTORS_OPEN as ::core::ffi::c_int) as GSpawnFlags,
        None,
        NULL_0,
        -(1 as gint),
        -(1 as gint),
        -(1 as gint),
        (&raw mut pipe_fds as *mut gint).offset(1 as ::core::ffi::c_int as isize) as *mut gint,
        (&raw mut pipe_fds as *mut gint).offset(1 as ::core::ffi::c_int as isize) as *mut gint,
        1 as gsize,
        &raw mut (*(*self_0).priv_0).bus_pid,
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        &raw mut error,
    );
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            663 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    safe_c2rust__g_test_watcher_add_pid((*(*self_0).priv_0).bus_pid);
    channel = g_io_channel_unix_new(pipe_fds[0 as ::core::ffi::c_int as usize]);
    pipe_fds[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int) as gint;
    g_io_channel_set_close_on_unref(channel, TRUE);
    g_io_channel_read_line(
        channel,
        &raw mut (*(*self_0).priv_0).bus_address,
        ::core::ptr::null_mut::<gsize>(),
        &raw mut termpos,
        &raw mut error,
    );
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            673 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    *(*(*self_0).priv_0).bus_address.offset(termpos as isize) = '\0' as i32 as gchar;
    close(pipe_fds[1 as ::core::ffi::c_int as usize]);
    pipe_fds[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int) as gint;
    if !g_getenv(b"G_DBUS_MONITOR\0" as *const u8 as *const gchar).is_null() {
        let mut command: *mut gchar = ::core::ptr::null_mut::<gchar>();
        command = g_strdup_printf(
            b"dbus-monitor --address %s\0" as *const u8 as *const gchar,
            (*(*self_0).priv_0).bus_address,
        );
        g_spawn_command_line_async(command, ::core::ptr::null_mut::<*mut GError>());
        g_free(command as gpointer);
        g_usleep((500 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int) as gulong);
    }
    g_io_channel_shutdown(channel, FALSE, &raw mut error);
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            693 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    g_io_channel_unref(channel);
    if g_unlink(config_path) != 0 as ::core::ffi::c_int {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtestdbus.c\0" as *const u8
                as *const ::core::ffi::c_char,
            698 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    g_free(print_address as gpointer);
    g_free(config_path as gpointer);
    g_free(config_arg as gpointer);
}
unsafe extern "C" fn safe_c2rust_stop_daemon(mut self_0: *mut GTestDBus) {
    kill((*(*self_0).priv_0).bus_pid as __pid_t, SIGTERM);
    safe_c2rust__g_test_watcher_remove_pid((*(*self_0).priv_0).bus_pid);
    g_spawn_close_pid((*(*self_0).priv_0).bus_pid);
    (*(*self_0).priv_0).bus_pid = 0 as ::core::ffi::c_int as GPid;
    g_free((*(*self_0).priv_0).bus_address as gpointer);
    (*(*self_0).priv_0).bus_address = ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_new(mut flags: GTestDBusFlags) -> *mut GTestDBus {
    return g_object_new(
        safe_c2rust_g_test_dbus_get_type(),
        b"flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        NULL_0,
    ) as *mut GTestDBus;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_get_flags(
    mut self_0: *mut GTestDBus,
) -> GTestDBusFlags {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_test_dbus_get_type();
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
            b"G_IS_TEST_DBUS (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TEST_DBUS_NONE;
    }
    return (*(*self_0).priv_0).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_get_bus_address(
    mut self_0: *mut GTestDBus,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_test_dbus_get_type();
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
            b"G_IS_TEST_DBUS (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*self_0).priv_0).bus_address;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_add_service_dir(
    mut self_0: *mut GTestDBus,
    mut path: *const gchar,
) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_test_dbus_get_type();
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
            b"G_IS_TEST_DBUS (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*(*self_0).priv_0).bus_address.is_null() {
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
            b"self->priv->bus_address == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_ptr_array_add(
        (*(*self_0).priv_0).service_dirs,
        safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_up(mut self_0: *mut GTestDBus) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_test_dbus_get_type();
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
            b"G_IS_TEST_DBUS (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if (*(*self_0).priv_0).bus_address.is_null() {
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
            b"self->priv->bus_address == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*(*self_0).priv_0).up == 0 {
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
            b"!self->priv->up\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_start_daemon(self_0);
    safe_c2rust_g_test_dbus_unset();
    g_setenv(
        b"DBUS_SESSION_BUS_ADDRESS\0" as *const u8 as *const gchar,
        (*(*self_0).priv_0).bus_address,
        TRUE,
    );
    (*(*self_0).priv_0).up = TRUE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_stop(mut self_0: *mut GTestDBus) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_test_dbus_get_type();
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
            b"G_IS_TEST_DBUS (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !(*(*self_0).priv_0).bus_address.is_null() {
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
            b"self->priv->bus_address != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_stop_daemon(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_down(mut self_0: *mut GTestDBus) {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_test_dbus_get_type();
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
            b"G_IS_TEST_DBUS (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if (*(*self_0).priv_0).up != 0 {
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
            b"self->priv->up\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    connection = _g_bus_get_singleton_if_exists(G_BUS_TYPE_SESSION);
    if !connection.is_null() {
        g_dbus_connection_set_exit_on_close(connection, FALSE);
    }
    if !(*(*self_0).priv_0).bus_address.is_null() {
        safe_c2rust_stop_daemon(self_0);
    }
    if !connection.is_null() {
        safe_c2rust__g_object_unref_and_wait_weak_notify(connection as gpointer);
    }
    safe_c2rust_g_test_dbus_unset();
    _g_bus_forget_singleton(G_BUS_TYPE_SESSION);
    (*(*self_0).priv_0).up = FALSE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_unset() {
    g_unsetenv(b"DISPLAY\0" as *const u8 as *const gchar);
    g_unsetenv(b"DBUS_SESSION_BUS_ADDRESS\0" as *const u8 as *const gchar);
    g_unsetenv(b"DBUS_STARTER_ADDRESS\0" as *const u8 as *const gchar);
    g_unsetenv(b"DBUS_STARTER_BUS_TYPE\0" as *const u8 as *const gchar);
    g_unsetenv(b"XDG_RUNTIME_DIR\0" as *const u8 as *const gchar);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
