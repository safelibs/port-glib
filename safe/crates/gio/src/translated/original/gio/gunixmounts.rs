use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GIConv;
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GCancellablePrivate;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GIcon;
    pub type libmnt_table;
    pub type libmnt_iter;
    pub type libmnt_fs;
    pub type libmnt_monitor;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn getuid() -> __uid_t;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_get_user_name() -> *const gchar;
    fn g_get_home_dir() -> *const gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_filename_display_basename(filename: *const gchar) -> *mut gchar;
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_file_read_link(filename: *const gchar, error: *mut *mut GError) -> *mut gchar;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_list_copy_deep(list: *mut GList, func: GCopyFunc, user_data: gpointer) -> *mut GList;
    fn g_main_context_default() -> *mut GMainContext;
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_is_destroyed(source: *mut GSource) -> gboolean;
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_timeout_source_new_seconds(interval: guint) -> *mut GSource;
    fn g_get_monotonic_time() -> gint64;
    fn g_source_remove(tag: guint) -> gboolean;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strstr_len(
        haystack: *const gchar,
        haystack_len: gssize,
        needle: *const gchar,
    ) -> *mut gchar;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_io_channel_unref(channel: *mut GIOChannel);
    fn g_io_create_watch(channel: *mut GIOChannel, condition: GIOCondition) -> *mut GSource;
    fn g_io_channel_unix_new(fd: ::core::ffi::c_int) -> *mut GIOChannel;
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
    fn g_access(filename: *const gchar, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        ...
    ) -> guint;
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn g_object_unref(object: gpointer);
    fn g_file_new_for_path(path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_monitor_file(
        file: *mut GFile,
        flags: GFileMonitorFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileMonitor;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_file_monitor_cancel(monitor: *mut GFileMonitor) -> gboolean;
    fn g_themed_icon_new_with_default_fallbacks(iconname: *const ::core::ffi::c_char)
        -> *mut GIcon;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_local_file_find_topdir_for(file_path: *const ::core::ffi::c_char) -> *mut gchar;
    fn g_context_specific_group_get(
        group: *mut GContextSpecificGroup,
        type_0: GType,
        context_offset: goffset,
        start_func: GCallback,
    ) -> gpointer;
    fn g_context_specific_group_remove(
        group: *mut GContextSpecificGroup,
        context: *mut GMainContext,
        instance: gpointer,
        stop_func: GCallback,
    );
    fn g_context_specific_group_emit(group: *mut GContextSpecificGroup, signal_id: guint);
    fn getmntent_r(
        __stream: *mut FILE,
        __result: *mut mntent,
        __buffer: *mut ::core::ffi::c_char,
        __bufsize: ::core::ffi::c_int,
    ) -> *mut mntent;
    fn endmntent(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn mnt_get_fstab_path() -> *const ::core::ffi::c_char;
    fn mnt_has_regular_mtab(
        mtab: *mut *const ::core::ffi::c_char,
        writable: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn mnt_optstr_get_flags(
        optstr: *const ::core::ffi::c_char,
        flags: *mut ::core::ffi::c_ulong,
        map: *const libmnt_optmap,
    ) -> ::core::ffi::c_int;
    fn mnt_new_iter(direction: ::core::ffi::c_int) -> *mut libmnt_iter;
    fn mnt_free_iter(itr: *mut libmnt_iter);
    fn mnt_get_builtin_optmap(id: ::core::ffi::c_int) -> *const libmnt_optmap;
    fn mnt_fs_get_source(fs: *mut libmnt_fs) -> *const ::core::ffi::c_char;
    fn mnt_fs_get_target(fs: *mut libmnt_fs) -> *const ::core::ffi::c_char;
    fn mnt_fs_get_fstype(fs: *mut libmnt_fs) -> *const ::core::ffi::c_char;
    fn mnt_fs_strdup_options(fs: *mut libmnt_fs) -> *mut ::core::ffi::c_char;
    fn mnt_fs_get_options(fs: *mut libmnt_fs) -> *const ::core::ffi::c_char;
    fn mnt_fs_get_root(fs: *mut libmnt_fs) -> *const ::core::ffi::c_char;
    fn mnt_table_parse_fstab(
        tb: *mut libmnt_table,
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn mnt_table_parse_mtab(
        tb: *mut libmnt_table,
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn mnt_new_table() -> *mut libmnt_table;
    fn mnt_free_table(tb: *mut libmnt_table);
    fn mnt_table_next_fs(
        tb: *mut libmnt_table,
        itr: *mut libmnt_iter,
        fs: *mut *mut libmnt_fs,
    ) -> ::core::ffi::c_int;
    fn mnt_new_monitor() -> *mut libmnt_monitor;
    fn mnt_unref_monitor(mn: *mut libmnt_monitor);
    fn mnt_monitor_enable_kernel(
        mn: *mut libmnt_monitor,
        enable: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn mnt_monitor_enable_userspace(
        mn: *mut libmnt_monitor,
        enable: ::core::ffi::c_int,
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn mnt_monitor_get_fd(mn: *mut libmnt_monitor) -> ::core::ffi::c_int;
    fn mnt_monitor_next_change(
        mn: *mut libmnt_monitor,
        filename: *mut *const ::core::ffi::c_char,
        type_0: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn mnt_monitor_event_cleanup(mn: *mut libmnt_monitor) -> ::core::ffi::c_int;
}
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type dev_t = __dev_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCond {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GCond = _GCond;
pub type GIConv = *mut _GIConv;
pub type GData = _GData;
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
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
pub type GClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
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
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalAccumulator = Option<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
pub type GConnectFlags = ::core::ffi::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
pub type GFile = _GFile;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixMountEntry {
    pub mount_path: *mut ::core::ffi::c_char,
    pub device_path: *mut ::core::ffi::c_char,
    pub root_path: *mut ::core::ffi::c_char,
    pub filesystem_type: *mut ::core::ffi::c_char,
    pub options: *mut ::core::ffi::c_char,
    pub is_read_only: gboolean,
    pub is_system_internal: gboolean,
}
pub type GUnixMountEntry = _GUnixMountEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GUnixMountEntry) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GUnixMountEntry) -> *mut GUnixMountEntry>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GUnixMountEntry) -> *mut GUnixMountEntry>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixMountPoint {
    pub mount_path: *mut ::core::ffi::c_char,
    pub device_path: *mut ::core::ffi::c_char,
    pub filesystem_type: *mut ::core::ffi::c_char,
    pub options: *mut ::core::ffi::c_char,
    pub is_read_only: gboolean,
    pub is_user_mountable: gboolean,
    pub is_loopback: gboolean,
}
pub type GUnixMountPoint = _GUnixMountPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GUnixMountPoint) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GUnixMountPoint) -> *mut GUnixMountPoint>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GUnixMountPoint) -> *mut GUnixMountPoint>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixMountMonitor {
    pub parent: GObject,
    pub context: *mut GMainContext,
}
pub type GUnixMountMonitor = _GUnixMountMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixMountMonitorClass {
    pub parent_class: GObjectClass,
}
pub type GUnixMountMonitorClass = _GUnixMountMonitorClass;
pub const G_UNIX_MOUNT_TYPE_CDROM: GUnixMountType = 2;
pub type GUnixMountType = ::core::ffi::c_uint;
pub const G_UNIX_MOUNT_TYPE_HD: GUnixMountType = 12;
pub const G_UNIX_MOUNT_TYPE_CAMERA: GUnixMountType = 11;
pub const G_UNIX_MOUNT_TYPE_IPOD: GUnixMountType = 10;
pub const G_UNIX_MOUNT_TYPE_SDMMC: GUnixMountType = 9;
pub const G_UNIX_MOUNT_TYPE_SM: GUnixMountType = 8;
pub const G_UNIX_MOUNT_TYPE_CF: GUnixMountType = 7;
pub const G_UNIX_MOUNT_TYPE_MEMSTICK: GUnixMountType = 6;
pub const G_UNIX_MOUNT_TYPE_JAZ: GUnixMountType = 5;
pub const G_UNIX_MOUNT_TYPE_ZIP: GUnixMountType = 4;
pub const G_UNIX_MOUNT_TYPE_NFS: GUnixMountType = 3;
pub const G_UNIX_MOUNT_TYPE_FLOPPY: GUnixMountType = 1;
pub const G_UNIX_MOUNT_TYPE_UNKNOWN: GUnixMountType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut ::core::ffi::c_char,
    pub mnt_dir: *mut ::core::ffi::c_char,
    pub mnt_type: *mut ::core::ffi::c_char,
    pub mnt_opts: *mut ::core::ffi::c_char,
    pub mnt_freq: ::core::ffi::c_int,
    pub mnt_passno: ::core::ffi::c_int,
}
pub const MS_RDONLY: C2RustUnnamed_4 = 1;
pub const MS_BIND: C2RustUnnamed_4 = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libmnt_optmap {
    pub name: *const ::core::ffi::c_char,
    pub id: ::core::ffi::c_int,
    pub mask: ::core::ffi::c_int,
}
pub const MNT_USERSPACE_MAP: C2RustUnnamed_6 = 2;
pub const MNT_LINUX_MAP: C2RustUnnamed_6 = 1;
pub const MNT_ITER_FORWARD: C2RustUnnamed_5 = 0;
pub const MOUNTPOINTS_CHANGED: C2RustUnnamed_7 = 1;
pub const MOUNTS_CHANGED: C2RustUnnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GContextSpecificGroup {
    pub table: *mut GHashTable,
    pub lock: GMutex,
    pub cond: GCond,
    pub requested_state: gboolean,
    pub requested_func: GCallback,
    pub effective_state: gboolean,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_int;
pub const MS_NOUSER: C2RustUnnamed_4 = -2147483648;
pub const MS_ACTIVE: C2RustUnnamed_4 = 1073741824;
pub const MS_LAZYTIME: C2RustUnnamed_4 = 33554432;
pub const MS_STRICTATIME: C2RustUnnamed_4 = 16777216;
pub const MS_I_VERSION: C2RustUnnamed_4 = 8388608;
pub const MS_KERNMOUNT: C2RustUnnamed_4 = 4194304;
pub const MS_RELATIME: C2RustUnnamed_4 = 2097152;
pub const MS_SHARED: C2RustUnnamed_4 = 1048576;
pub const MS_SLAVE: C2RustUnnamed_4 = 524288;
pub const MS_PRIVATE: C2RustUnnamed_4 = 262144;
pub const MS_UNBINDABLE: C2RustUnnamed_4 = 131072;
pub const MS_POSIXACL: C2RustUnnamed_4 = 65536;
pub const MS_SILENT: C2RustUnnamed_4 = 32768;
pub const MS_REC: C2RustUnnamed_4 = 16384;
pub const MS_MOVE: C2RustUnnamed_4 = 8192;
pub const MS_NODIRATIME: C2RustUnnamed_4 = 2048;
pub const MS_NOATIME: C2RustUnnamed_4 = 1024;
pub const MS_NOSYMFOLLOW: C2RustUnnamed_4 = 256;
pub const MS_DIRSYNC: C2RustUnnamed_4 = 128;
pub const MS_MANDLOCK: C2RustUnnamed_4 = 64;
pub const MS_REMOUNT: C2RustUnnamed_4 = 32;
pub const MS_SYNCHRONOUS: C2RustUnnamed_4 = 16;
pub const MS_NOEXEC: C2RustUnnamed_4 = 8;
pub const MS_NODEV: C2RustUnnamed_4 = 4;
pub const MS_NOSUID: C2RustUnnamed_4 = 2;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const MNT_ITER_BACKWARD: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_7 = 2;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_mount_entry_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_1, C2RustUnnamed_0) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_1, C2RustUnnamed_0) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GUnixMountEntry\0" as *const u8 as *const gchar),
        C2RustUnnamed_1 {
            do_copy_type: Some(
                safe_c2rust_g_unix_mount_copy
                    as unsafe extern "C" fn(*mut GUnixMountEntry) -> *mut GUnixMountEntry,
            ),
        },
        C2RustUnnamed_0 {
            do_free_type: Some(
                safe_c2rust_g_unix_mount_free as unsafe extern "C" fn(*mut GUnixMountEntry) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_entry_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_mount_entry_get_type_once();
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_mount_point_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_unix_mount_point_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_3, C2RustUnnamed_2) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_3, C2RustUnnamed_2) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GUnixMountPoint\0" as *const u8 as *const gchar),
        C2RustUnnamed_3 {
            do_copy_type: Some(
                safe_c2rust_g_unix_mount_point_copy
                    as unsafe extern "C" fn(*mut GUnixMountPoint) -> *mut GUnixMountPoint,
            ),
        },
        C2RustUnnamed_2 {
            do_free_type: Some(
                safe_c2rust_g_unix_mount_point_free
                    as unsafe extern "C" fn(*mut GUnixMountPoint) -> (),
            ),
        },
    );
    return g_define_type_id;
}
static mut safe_c2rust_g__proc_mounts_source_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_mount_poller_time: guint64 = 0 as guint64;
static mut safe_c2rust_proc_mounts_watch_source: *mut GSource =
    ::core::ptr::null::<GSource>() as *mut GSource;
pub const MNT_MS_USER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const MNT_MS_USERS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const MNT_MS_OWNER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const MNT_MS_LOOP: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
static mut safe_c2rust_proc_mounts_monitor: *mut libmnt_monitor =
    ::core::ptr::null::<libmnt_monitor>() as *mut libmnt_monitor;
unsafe extern "C" fn safe_c2rust_is_in(
    mut value: *const ::core::ffi::c_char,
    mut set: *mut *const ::core::ffi::c_char,
) -> gboolean {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*set.offset(i as isize)).is_null() {
        if strcmp(*set.offset(i as isize), value) == 0 as ::core::ffi::c_int {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_is_mount_path_system_internal(
    mut mount_path: *const ::core::ffi::c_char,
) -> gboolean {
    let mut ignore_mountpoints: [*const ::core::ffi::c_char; 43] = [
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
        b"/bin\0" as *const u8 as *const ::core::ffi::c_char,
        b"/boot\0" as *const u8 as *const ::core::ffi::c_char,
        b"/compat/linux/proc\0" as *const u8 as *const ::core::ffi::c_char,
        b"/compat/linux/sys\0" as *const u8 as *const ::core::ffi::c_char,
        b"/dev\0" as *const u8 as *const ::core::ffi::c_char,
        b"/etc\0" as *const u8 as *const ::core::ffi::c_char,
        b"/home\0" as *const u8 as *const ::core::ffi::c_char,
        b"/lib\0" as *const u8 as *const ::core::ffi::c_char,
        b"/lib64\0" as *const u8 as *const ::core::ffi::c_char,
        b"/libexec\0" as *const u8 as *const ::core::ffi::c_char,
        b"/live/cow\0" as *const u8 as *const ::core::ffi::c_char,
        b"/live/image\0" as *const u8 as *const ::core::ffi::c_char,
        b"/media\0" as *const u8 as *const ::core::ffi::c_char,
        b"/mnt\0" as *const u8 as *const ::core::ffi::c_char,
        b"/opt\0" as *const u8 as *const ::core::ffi::c_char,
        b"/rescue\0" as *const u8 as *const ::core::ffi::c_char,
        b"/root\0" as *const u8 as *const ::core::ffi::c_char,
        b"/sbin\0" as *const u8 as *const ::core::ffi::c_char,
        b"/srv\0" as *const u8 as *const ::core::ffi::c_char,
        b"/tmp\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/X11R6\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/local\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/obj\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/ports\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/src\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/xobj\0" as *const u8 as *const ::core::ffi::c_char,
        b"/var\0" as *const u8 as *const ::core::ffi::c_char,
        b"/var/crash\0" as *const u8 as *const ::core::ffi::c_char,
        b"/var/local\0" as *const u8 as *const ::core::ffi::c_char,
        GLIB_LOCALSTATEDIR.as_ptr(),
        b"/var/log\0" as *const u8 as *const ::core::ffi::c_char,
        b"/var/log/audit\0" as *const u8 as *const ::core::ffi::c_char,
        b"/var/mail\0" as *const u8 as *const ::core::ffi::c_char,
        b"/var/run\0" as *const u8 as *const ::core::ffi::c_char,
        GLIB_RUNSTATEDIR.as_ptr(),
        b"/var/tmp\0" as *const u8 as *const ::core::ffi::c_char,
        b"/proc\0" as *const u8 as *const ::core::ffi::c_char,
        b"/sbin\0" as *const u8 as *const ::core::ffi::c_char,
        b"/net\0" as *const u8 as *const ::core::ffi::c_char,
        b"/sys\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    if safe_c2rust_is_in(
        mount_path,
        &raw mut ignore_mountpoints as *mut *const ::core::ffi::c_char,
    ) != 0
    {
        return TRUE;
    }
    if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = mount_path;
            let __prefix: *const ::core::ffi::c_char =
                b"/dev/\0" as *const u8 as *const ::core::ffi::c_char;
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
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(
            mount_path as *const gchar,
            b"/dev/\0" as *const u8 as *const gchar,
        )
    }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = mount_path;
                let __prefix: *const ::core::ffi::c_char =
                    b"/proc/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_11
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                mount_path as *const gchar,
                b"/proc/\0" as *const u8 as *const gchar,
            )
        }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = mount_path;
                let __prefix: *const ::core::ffi::c_char =
                    b"/sys/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_12
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                mount_path as *const gchar,
                b"/sys/\0" as *const u8 as *const gchar,
            )
        }) != 0
    {
        return TRUE;
    }
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = mount_path;
            let __suffix: *const ::core::ffi::c_char =
                b"/.gvfs\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if __str.is_null() || __suffix.is_null() {
                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_13
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
        g_str_has_suffix(
            mount_path as *const gchar,
            b"/.gvfs\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_is_system_fs_type(
    mut fs_type: *const ::core::ffi::c_char,
) -> gboolean {
    let mut ignore_fs: [*const ::core::ffi::c_char; 43] = [
        b"adfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"afs\0" as *const u8 as *const ::core::ffi::c_char,
        b"auto\0" as *const u8 as *const ::core::ffi::c_char,
        b"autofs\0" as *const u8 as *const ::core::ffi::c_char,
        b"autofs4\0" as *const u8 as *const ::core::ffi::c_char,
        b"cgroup\0" as *const u8 as *const ::core::ffi::c_char,
        b"configfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"cxfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"debugfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"devfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"devpts\0" as *const u8 as *const ::core::ffi::c_char,
        b"devtmpfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"ecryptfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"fdescfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"fusectl\0" as *const u8 as *const ::core::ffi::c_char,
        b"gfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"gfs2\0" as *const u8 as *const ::core::ffi::c_char,
        b"gpfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"hugetlbfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"kernfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"linprocfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"linsysfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"lustre\0" as *const u8 as *const ::core::ffi::c_char,
        b"lustre_lite\0" as *const u8 as *const ::core::ffi::c_char,
        b"mfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"mqueue\0" as *const u8 as *const ::core::ffi::c_char,
        b"ncpfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"nfsd\0" as *const u8 as *const ::core::ffi::c_char,
        b"nullfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"ocfs2\0" as *const u8 as *const ::core::ffi::c_char,
        b"overlay\0" as *const u8 as *const ::core::ffi::c_char,
        b"proc\0" as *const u8 as *const ::core::ffi::c_char,
        b"procfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"pstore\0" as *const u8 as *const ::core::ffi::c_char,
        b"ptyfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"rootfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"rpc_pipefs\0" as *const u8 as *const ::core::ffi::c_char,
        b"securityfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"selinuxfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"sysfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"tmpfs\0" as *const u8 as *const ::core::ffi::c_char,
        b"usbfs\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !fs_type.is_null() && *fs_type as ::core::ffi::c_int != '\0' as i32 {
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
            b"fs_type != NULL && *fs_type != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_is_in(
        fs_type,
        &raw mut ignore_fs as *mut *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_is_system_device_path(
    mut device_path: *const ::core::ffi::c_char,
) -> gboolean {
    let mut ignore_devices: [*const ::core::ffi::c_char; 7] = [
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        b"sunrpc\0" as *const u8 as *const ::core::ffi::c_char,
        b"devpts\0" as *const u8 as *const ::core::ffi::c_char,
        b"nfsd\0" as *const u8 as *const ::core::ffi::c_char,
        b"/dev/loop\0" as *const u8 as *const ::core::ffi::c_char,
        b"/dev/vn\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !device_path.is_null() && *device_path as ::core::ffi::c_int != '\0' as i32 {
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
            b"device_path != NULL && *device_path != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_is_in(
        device_path,
        &raw mut ignore_devices as *mut *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn safe_c2rust_guess_system_internal(
    mut mountpoint: *const ::core::ffi::c_char,
    mut fs: *const ::core::ffi::c_char,
    mut device: *const ::core::ffi::c_char,
    mut root: *const ::core::ffi::c_char,
) -> gboolean {
    if safe_c2rust_g_unix_is_system_fs_type(fs) != 0 {
        return TRUE;
    }
    if safe_c2rust_g_unix_is_system_device_path(device) != 0 {
        return TRUE;
    }
    if safe_c2rust_g_unix_is_mount_path_system_internal(mountpoint) != 0 {
        return TRUE;
    }
    if !root.is_null()
        && g_strcmp0(root, b"/\0" as *const u8 as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_create_unix_mount_entry(
    mut device_path: *const ::core::ffi::c_char,
    mut mount_path: *const ::core::ffi::c_char,
    mut root_path: *const ::core::ffi::c_char,
    mut filesystem_type: *const ::core::ffi::c_char,
    mut options: *const ::core::ffi::c_char,
    mut is_read_only: gboolean,
) -> *mut GUnixMountEntry {
    let mut mount_entry: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    mount_entry = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GUnixMountEntry>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GUnixMountEntry;
    (*mount_entry).device_path = safe_c2rust_g_strdup_inline(device_path);
    (*mount_entry).mount_path = safe_c2rust_g_strdup_inline(mount_path);
    (*mount_entry).root_path = safe_c2rust_g_strdup_inline(root_path);
    (*mount_entry).filesystem_type = safe_c2rust_g_strdup_inline(filesystem_type);
    (*mount_entry).options = safe_c2rust_g_strdup_inline(options);
    (*mount_entry).is_read_only = is_read_only;
    (*mount_entry).is_system_internal = safe_c2rust_guess_system_internal(
        (*mount_entry).mount_path,
        (*mount_entry).filesystem_type,
        (*mount_entry).device_path,
        (*mount_entry).root_path,
    );
    return mount_entry;
}
unsafe extern "C" fn safe_c2rust_create_unix_mount_point(
    mut device_path: *const ::core::ffi::c_char,
    mut mount_path: *const ::core::ffi::c_char,
    mut filesystem_type: *const ::core::ffi::c_char,
    mut options: *const ::core::ffi::c_char,
    mut is_read_only: gboolean,
    mut is_user_mountable: gboolean,
    mut is_loopback: gboolean,
) -> *mut GUnixMountPoint {
    let mut mount_point: *mut GUnixMountPoint = ::core::ptr::null_mut::<GUnixMountPoint>();
    mount_point = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GUnixMountPoint>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GUnixMountPoint;
    (*mount_point).device_path = safe_c2rust_g_strdup_inline(device_path);
    (*mount_point).mount_path = safe_c2rust_g_strdup_inline(mount_path);
    (*mount_point).filesystem_type = safe_c2rust_g_strdup_inline(filesystem_type);
    (*mount_point).options = safe_c2rust_g_strdup_inline(options);
    (*mount_point).is_read_only = is_read_only;
    (*mount_point).is_user_mountable = is_user_mountable;
    (*mount_point).is_loopback = is_loopback;
    return mount_point;
}
pub const PROC_MOUNTINFO_PATH: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"/proc/self/mountinfo\0")
};
unsafe extern "C" fn safe_c2rust__g_get_unix_mounts() -> *mut GList {
    let mut table: *mut libmnt_table = ::core::ptr::null_mut::<libmnt_table>();
    let mut iter: *mut libmnt_iter = ::core::ptr::null_mut::<libmnt_iter>();
    let mut fs: *mut libmnt_fs = ::core::ptr::null_mut::<libmnt_fs>();
    let mut mount_entry: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    let mut return_list: *mut GList = ::core::ptr::null_mut::<GList>();
    table = mnt_new_table();
    if !(mnt_table_parse_mtab(table, ::core::ptr::null::<::core::ffi::c_char>())
        < 0 as ::core::ffi::c_int)
    {
        iter = mnt_new_iter(MNT_ITER_FORWARD as ::core::ffi::c_int);
        while mnt_table_next_fs(table, iter, &raw mut fs) == 0 as ::core::ffi::c_int {
            let mut device_path: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut mount_options: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut mount_flags: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
            let mut is_read_only: gboolean = FALSE;
            device_path = mnt_fs_get_source(fs);
            if g_strcmp0(
                device_path,
                b"/dev/root\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                device_path = safe_c2rust__resolve_dev_root();
            }
            mount_options = mnt_fs_strdup_options(fs);
            if !mount_options.is_null() {
                mnt_optstr_get_flags(
                    mount_options,
                    &raw mut mount_flags,
                    mnt_get_builtin_optmap(MNT_LINUX_MAP as ::core::ffi::c_int),
                );
                g_free(mount_options as gpointer);
            }
            is_read_only =
                (if mount_flags & MS_RDONLY as ::core::ffi::c_int as ::core::ffi::c_ulong != 0 {
                    TRUE
                } else {
                    FALSE
                }) as gboolean;
            mount_entry = safe_c2rust_create_unix_mount_entry(
                device_path,
                mnt_fs_get_target(fs),
                mnt_fs_get_root(fs),
                mnt_fs_get_fstype(fs),
                mnt_fs_get_options(fs),
                is_read_only,
            );
            return_list = g_list_prepend(return_list, mount_entry as gpointer);
        }
        mnt_free_iter(iter);
    }
    mnt_free_table(table);
    return g_list_reverse(return_list);
}
unsafe extern "C" fn safe_c2rust_get_mtab_monitor_file() -> *const ::core::ffi::c_char {
    static mut safe_c2rust_mountinfo_path: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if !safe_c2rust_mountinfo_path.is_null() {
        return safe_c2rust_mountinfo_path;
    }
    if mnt_has_regular_mtab(
        &raw mut safe_c2rust_mountinfo_path,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) != 0
    {
        return safe_c2rust_mountinfo_path;
    }
    if stat(PROC_MOUNTINFO_PATH.as_ptr(), &raw mut buf) == 0 as ::core::ffi::c_int {
        safe_c2rust_mountinfo_path = PROC_MOUNTINFO_PATH.as_ptr();
        return safe_c2rust_mountinfo_path;
    }
    safe_c2rust_mountinfo_path = b"/proc/mounts\0" as *const u8 as *const ::core::ffi::c_char;
    return safe_c2rust_mountinfo_path;
}
unsafe extern "C" fn safe_c2rust_get_fstab_file() -> *mut ::core::ffi::c_char {
    return mnt_get_fstab_path() as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust__g_get_unix_mount_points() -> *mut GList {
    let mut table: *mut libmnt_table = ::core::ptr::null_mut::<libmnt_table>();
    let mut iter: *mut libmnt_iter = ::core::ptr::null_mut::<libmnt_iter>();
    let mut fs: *mut libmnt_fs = ::core::ptr::null_mut::<libmnt_fs>();
    let mut mount_point: *mut GUnixMountPoint = ::core::ptr::null_mut::<GUnixMountPoint>();
    let mut return_list: *mut GList = ::core::ptr::null_mut::<GList>();
    table = mnt_new_table();
    if !(mnt_table_parse_fstab(table, ::core::ptr::null::<::core::ffi::c_char>())
        < 0 as ::core::ffi::c_int)
    {
        iter = mnt_new_iter(MNT_ITER_FORWARD as ::core::ffi::c_int);
        while mnt_table_next_fs(table, iter, &raw mut fs) == 0 as ::core::ffi::c_int {
            let mut device_path: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut mount_path: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut mount_fstype: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut mount_options: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut is_read_only: gboolean = FALSE;
            let mut is_user_mountable: gboolean = FALSE;
            let mut is_loopback: gboolean = FALSE;
            mount_path = mnt_fs_get_target(fs);
            if strcmp(
                mount_path,
                b"ignore\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    mount_path,
                    b"swap\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    mount_path,
                    b"none\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                continue;
            }
            mount_fstype = mnt_fs_get_fstype(fs);
            mount_options = mnt_fs_strdup_options(fs);
            if !mount_options.is_null() {
                let mut mount_flags: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
                let mut userspace_flags: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
                mnt_optstr_get_flags(
                    mount_options,
                    &raw mut mount_flags,
                    mnt_get_builtin_optmap(MNT_LINUX_MAP as ::core::ffi::c_int),
                );
                mnt_optstr_get_flags(
                    mount_options,
                    &raw mut userspace_flags,
                    mnt_get_builtin_optmap(MNT_USERSPACE_MAP as ::core::ffi::c_int),
                );
                if mount_flags & MS_BIND as ::core::ffi::c_int as ::core::ffi::c_ulong != 0 {
                    g_free(mount_options as gpointer);
                    continue;
                } else {
                    is_read_only = (mount_flags
                        & MS_RDONLY as ::core::ffi::c_int as ::core::ffi::c_ulong
                        != 0 as ::core::ffi::c_ulong)
                        as ::core::ffi::c_int as gboolean;
                    is_loopback = (userspace_flags & MNT_MS_LOOP as ::core::ffi::c_ulong
                        != 0 as ::core::ffi::c_ulong)
                        as ::core::ffi::c_int as gboolean;
                    if !mount_fstype.is_null()
                        && g_strcmp0(
                            b"supermount\0" as *const u8 as *const ::core::ffi::c_char,
                            mount_fstype,
                        ) == 0 as ::core::ffi::c_int
                        || userspace_flags & MNT_MS_USER as ::core::ffi::c_ulong != 0
                            && g_strstr_len(
                                mount_options,
                                -(1 as ::core::ffi::c_int) as gssize,
                                b"user_xattr\0" as *const u8 as *const gchar,
                            )
                            .is_null()
                        || userspace_flags & MNT_MS_USERS as ::core::ffi::c_ulong != 0
                        || userspace_flags & MNT_MS_OWNER as ::core::ffi::c_ulong != 0
                    {
                        is_user_mountable = TRUE as gboolean;
                    }
                }
            }
            device_path = mnt_fs_get_source(fs);
            if g_strcmp0(
                device_path,
                b"/dev/root\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                device_path = safe_c2rust__resolve_dev_root();
            }
            mount_point = safe_c2rust_create_unix_mount_point(
                device_path,
                mount_path,
                mount_fstype,
                mount_options,
                is_read_only,
                is_user_mountable,
                is_loopback,
            );
            if !mount_options.is_null() {
                g_free(mount_options as gpointer);
            }
            return_list = g_list_prepend(return_list, mount_point as gpointer);
        }
        mnt_free_iter(iter);
    }
    mnt_free_table(table);
    return g_list_reverse(return_list);
}
unsafe extern "C" fn safe_c2rust_get_mounts_timestamp() -> guint64 {
    let mut monitor_file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut timestamp: guint64 = 0 as guint64;
    g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
    monitor_file = safe_c2rust_get_mtab_monitor_file();
    if !monitor_file.is_null()
        && (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = monitor_file;
                let __prefix: *const ::core::ffi::c_char =
                    b"/proc/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_16
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                monitor_file as *const gchar,
                b"/proc/\0" as *const u8 as *const gchar,
            )
        }) == 0
    {
        if stat(monitor_file, &raw mut buf) == 0 as ::core::ffi::c_int {
            timestamp = buf.st_mtim.tv_sec as guint64;
        }
    } else if safe_c2rust_proc_mounts_watch_is_running() != 0 {
        timestamp = safe_c2rust_mount_poller_time;
    } else {
        timestamp = g_get_monotonic_time() as guint64;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
    return timestamp;
}
unsafe extern "C" fn safe_c2rust_get_mount_points_timestamp() -> guint64 {
    let mut monitor_file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    monitor_file = safe_c2rust_get_fstab_file();
    if !monitor_file.is_null() {
        if stat(monitor_file, &raw mut buf) == 0 as ::core::ffi::c_int {
            return buf.st_mtim.tv_sec as guint64;
        }
    }
    return 0 as guint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mounts_get(mut time_read: *mut guint64) -> *mut GList {
    if !time_read.is_null() {
        *time_read = safe_c2rust_get_mounts_timestamp();
    }
    return safe_c2rust__g_get_unix_mounts();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_at(
    mut mount_path: *const ::core::ffi::c_char,
    mut time_read: *mut guint64,
) -> *mut GUnixMountEntry {
    let mut mounts: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut mount_entry: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    let mut found: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    mounts = safe_c2rust_g_unix_mounts_get(time_read);
    found = ::core::ptr::null_mut::<GUnixMountEntry>();
    l = mounts;
    while !l.is_null() {
        mount_entry = (*l).data as *mut GUnixMountEntry;
        if strcmp(mount_path, (*mount_entry).mount_path) == 0 as ::core::ffi::c_int {
            if !found.is_null() {
                safe_c2rust_g_unix_mount_free(found);
            }
            found = mount_entry;
        } else {
            safe_c2rust_g_unix_mount_free(mount_entry);
        }
        l = (*l).next;
    }
    g_list_free(mounts);
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_for(
    mut file_path: *const ::core::ffi::c_char,
    mut time_read: *mut guint64,
) -> *mut GUnixMountEntry {
    let mut entry: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !file_path.is_null() {
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
            b"file_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUnixMountEntry>();
    }
    entry = safe_c2rust_g_unix_mount_at(file_path, time_read);
    if entry.is_null() {
        let mut topdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        topdir = _g_local_file_find_topdir_for(file_path) as *mut ::core::ffi::c_char;
        if !topdir.is_null() {
            entry = safe_c2rust_g_unix_mount_at(topdir, time_read);
            g_free(topdir as gpointer);
        }
    }
    return entry;
}
unsafe extern "C" fn safe_c2rust_copy_mount_point_cb(
    mut src: gconstpointer,
    mut data: gpointer,
) -> gpointer {
    let mut src_mount_point: *mut GUnixMountPoint = src as *mut GUnixMountPoint;
    return safe_c2rust_g_unix_mount_point_copy(src_mount_point) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_points_get(
    mut time_read: *mut guint64,
) -> *mut GList {
    static mut safe_c2rust_mnt_pts_last: *mut GList = ::core::ptr::null::<GList>() as *mut GList;
    static mut safe_c2rust_time_read_last: guint64 = 0 as guint64;
    let mut mnt_pts: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut time_read_now: guint64 = 0;
    static mut safe_c2rust_g__unix_mount_points_lock: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    g_mutex_lock(&raw mut safe_c2rust_g__unix_mount_points_lock);
    time_read_now = safe_c2rust_get_mount_points_timestamp();
    if time_read_now != safe_c2rust_time_read_last || safe_c2rust_mnt_pts_last.is_null() {
        safe_c2rust_time_read_last = time_read_now;
        g_list_free_full(
            safe_c2rust_mnt_pts_last,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixMountPoint) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_unix_mount_point_free
                    as unsafe extern "C" fn(*mut GUnixMountPoint) -> (),
            )),
        );
        safe_c2rust_mnt_pts_last = safe_c2rust__g_get_unix_mount_points();
    }
    mnt_pts = g_list_copy_deep(
        safe_c2rust_mnt_pts_last,
        Some(
            safe_c2rust_copy_mount_point_cb
                as unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer,
        ),
        NULL_0,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__unix_mount_points_lock);
    if !time_read.is_null() {
        *time_read = time_read_now;
    }
    return mnt_pts;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_at(
    mut mount_path: *const ::core::ffi::c_char,
    mut time_read: *mut guint64,
) -> *mut GUnixMountPoint {
    let mut mount_points: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut mount_point: *mut GUnixMountPoint = ::core::ptr::null_mut::<GUnixMountPoint>();
    let mut found: *mut GUnixMountPoint = ::core::ptr::null_mut::<GUnixMountPoint>();
    mount_points = safe_c2rust_g_unix_mount_points_get(time_read);
    found = ::core::ptr::null_mut::<GUnixMountPoint>();
    l = mount_points;
    while !l.is_null() {
        mount_point = (*l).data as *mut GUnixMountPoint;
        if strcmp(mount_path, (*mount_point).mount_path) == 0 as ::core::ffi::c_int {
            if !found.is_null() {
                safe_c2rust_g_unix_mount_point_free(found);
            }
            found = mount_point;
        } else {
            safe_c2rust_g_unix_mount_point_free(mount_point);
        }
        l = (*l).next;
    }
    g_list_free(mount_points);
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mounts_changed_since(mut time: guint64) -> gboolean {
    return (safe_c2rust_get_mounts_timestamp() != time) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_points_changed_since(
    mut time: guint64,
) -> gboolean {
    return (safe_c2rust_get_mount_points_timestamp() != time) as ::core::ffi::c_int;
}
static mut safe_c2rust_signals: [guint; 2] = [0; 2];
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_mount_monitor_get_type_once();
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
static mut safe_c2rust_GUnixMountMonitor_private_offset: gint = 0;
static mut safe_c2rust_g_unix_mount_monitor_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GUnixMountMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixMountMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_mount_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixMountMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixMountMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_mount_monitor_init
                    as unsafe extern "C" fn(*mut GUnixMountMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_mount_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixMountMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnixMountMonitor_private_offset,
        );
    }
    safe_c2rust_g_unix_mount_monitor_class_init(klass as *mut GUnixMountMonitorClass);
}
static mut safe_c2rust_mount_monitor_group: GContextSpecificGroup = GContextSpecificGroup {
    table: ::core::ptr::null::<GHashTable>() as *mut GHashTable,
    lock: _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    cond: _GCond {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        i: [0; 2],
    },
    requested_state: 0,
    requested_func: None,
    effective_state: 0,
};
static mut safe_c2rust_fstab_monitor: *mut GFileMonitor =
    ::core::ptr::null::<GFileMonitor>() as *mut GFileMonitor;
static mut safe_c2rust_mtab_monitor: *mut GFileMonitor =
    ::core::ptr::null::<GFileMonitor>() as *mut GFileMonitor;
static mut safe_c2rust_mount_poller_mounts: *mut GList = ::core::ptr::null::<GList>() as *mut GList;
static mut safe_c2rust_mtab_file_changed_id: guint = 0;
unsafe extern "C" fn safe_c2rust_proc_mounts_watch_is_running() -> gboolean {
    return (!safe_c2rust_proc_mounts_watch_source.is_null()
        && g_source_is_destroyed(safe_c2rust_proc_mounts_watch_source) == 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_fstab_file_changed(
    mut monitor: *mut GFileMonitor,
    mut file: *mut GFile,
    mut other_file: *mut GFile,
    mut event_type: GFileMonitorEvent,
    mut user_data: gpointer,
) {
    if event_type as ::core::ffi::c_uint
        != G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int as ::core::ffi::c_uint
        && event_type as ::core::ffi::c_uint
            != G_FILE_MONITOR_EVENT_CREATED as ::core::ffi::c_int as ::core::ffi::c_uint
        && event_type as ::core::ffi::c_uint
            != G_FILE_MONITOR_EVENT_DELETED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    g_context_specific_group_emit(
        &raw mut safe_c2rust_mount_monitor_group,
        safe_c2rust_signals[MOUNTPOINTS_CHANGED as ::core::ffi::c_int as usize],
    );
}
unsafe extern "C" fn safe_c2rust_mtab_file_changed_cb(mut user_data: gpointer) -> gboolean {
    safe_c2rust_mtab_file_changed_id = 0 as guint;
    g_context_specific_group_emit(
        &raw mut safe_c2rust_mount_monitor_group,
        safe_c2rust_signals[MOUNTS_CHANGED as ::core::ffi::c_int as usize],
    );
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_mtab_file_changed(
    mut monitor: *mut GFileMonitor,
    mut file: *mut GFile,
    mut other_file: *mut GFile,
    mut event_type: GFileMonitorEvent,
    mut user_data: gpointer,
) {
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if event_type as ::core::ffi::c_uint
        != G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int as ::core::ffi::c_uint
        && event_type as ::core::ffi::c_uint
            != G_FILE_MONITOR_EVENT_CREATED as ::core::ffi::c_int as ::core::ffi::c_uint
        && event_type as ::core::ffi::c_uint
            != G_FILE_MONITOR_EVENT_DELETED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if safe_c2rust_mtab_file_changed_id > 0 as guint {
        return;
    }
    context = g_main_context_get_thread_default();
    if context.is_null() {
        context = g_main_context_default();
    }
    source = g_idle_source_new();
    g_source_set_priority(source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        source,
        Some(safe_c2rust_mtab_file_changed_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        NULL_0,
        None,
    );
    g_source_set_static_name(
        source,
        b"[gio] mtab_file_changed_cb\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(source, context);
    g_source_unref(source);
}
unsafe extern "C" fn safe_c2rust_proc_mounts_changed(
    mut channel: *mut GIOChannel,
    mut cond: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut has_changed: gboolean = FALSE;
    if cond as ::core::ffi::c_uint & G_IO_IN as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
        if !safe_c2rust_proc_mounts_monitor.is_null() {
            let mut ret: ::core::ffi::c_int = 0;
            ret = mnt_monitor_next_change(
                safe_c2rust_proc_mounts_monitor,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if ret == 0 as ::core::ffi::c_int {
                has_changed = TRUE as gboolean;
                ret = mnt_monitor_event_cleanup(safe_c2rust_proc_mounts_monitor);
            }
            if ret < 0 as ::core::ffi::c_int {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"mnt_monitor_next_change failed: %s\0" as *const u8 as *const gchar,
                    g_strerror(-(ret as gint)),
                );
            }
        }
        g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
    }
    if has_changed != 0 {
        g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
        safe_c2rust_mount_poller_time = g_get_monotonic_time() as guint64;
        g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
        g_context_specific_group_emit(
            &raw mut safe_c2rust_mount_monitor_group,
            safe_c2rust_signals[MOUNTS_CHANGED as ::core::ffi::c_int as usize],
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_mount_change_poller(mut user_data: gpointer) -> gboolean {
    let mut current_mounts: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut new_it: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut old_it: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut has_changed: gboolean = FALSE;
    current_mounts = safe_c2rust__g_get_unix_mounts();
    new_it = current_mounts;
    old_it = safe_c2rust_mount_poller_mounts;
    while !new_it.is_null() && !old_it.is_null() {
        if safe_c2rust_g_unix_mount_compare(
            (*new_it).data as *mut GUnixMountEntry,
            (*old_it).data as *mut GUnixMountEntry,
        ) != 0 as ::core::ffi::c_int
        {
            has_changed = TRUE as gboolean;
            break;
        } else {
            new_it = (if !new_it.is_null() {
                (*new_it).next
            } else {
                ::core::ptr::null_mut::<GList>()
            });
            old_it = (if !old_it.is_null() {
                (*old_it).next
            } else {
                ::core::ptr::null_mut::<GList>()
            });
        }
    }
    if !(new_it.is_null() && old_it.is_null()) {
        has_changed = TRUE as gboolean;
    }
    g_list_free_full(
        safe_c2rust_mount_poller_mounts,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountEntry) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_g_unix_mount_free as unsafe extern "C" fn(*mut GUnixMountEntry) -> (),
        )),
    );
    safe_c2rust_mount_poller_mounts = current_mounts;
    if has_changed != 0 {
        g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
        safe_c2rust_mount_poller_time = g_get_monotonic_time() as guint64;
        g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
        g_context_specific_group_emit(
            &raw mut safe_c2rust_mount_monitor_group,
            safe_c2rust_signals[MOUNTPOINTS_CHANGED as ::core::ffi::c_int as usize],
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_start_mount_poller() {
    safe_c2rust_proc_mounts_watch_source = g_timeout_source_new_seconds(3 as guint);
    safe_c2rust_mount_poller_mounts = safe_c2rust__g_get_unix_mounts();
    safe_c2rust_mount_poller_time = g_get_monotonic_time() as guint64;
    g_source_set_callback(
        safe_c2rust_proc_mounts_watch_source,
        Some(safe_c2rust_mount_change_poller as unsafe extern "C" fn(gpointer) -> gboolean),
        NULL_0,
        None,
    );
    g_source_attach(
        safe_c2rust_proc_mounts_watch_source,
        g_main_context_get_thread_default(),
    );
    g_source_unref(safe_c2rust_proc_mounts_watch_source);
}
unsafe extern "C" fn safe_c2rust_mount_monitor_stop() {
    if !safe_c2rust_fstab_monitor.is_null() {
        g_file_monitor_cancel(safe_c2rust_fstab_monitor);
        g_object_unref(safe_c2rust_fstab_monitor as gpointer);
    }
    g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
    if !safe_c2rust_proc_mounts_watch_source.is_null() {
        g_source_destroy(safe_c2rust_proc_mounts_watch_source);
        safe_c2rust_proc_mounts_watch_source = ::core::ptr::null_mut::<GSource>();
    }
    let mut _pp: *mut *mut libmnt_monitor = &raw mut safe_c2rust_proc_mounts_monitor;
    let mut _ptr: *mut libmnt_monitor = *_pp;
    *_pp = ::core::ptr::null_mut::<libmnt_monitor>();
    if !_ptr.is_null() {
        mnt_unref_monitor(_ptr as *mut libmnt_monitor);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
    if !safe_c2rust_mtab_monitor.is_null() {
        g_file_monitor_cancel(safe_c2rust_mtab_monitor);
        g_object_unref(safe_c2rust_mtab_monitor as gpointer);
    }
    if safe_c2rust_mtab_file_changed_id != 0 {
        g_source_remove(safe_c2rust_mtab_file_changed_id);
        safe_c2rust_mtab_file_changed_id = 0 as guint;
    }
    g_list_free_full(
        safe_c2rust_mount_poller_mounts,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountEntry) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_g_unix_mount_free as unsafe extern "C" fn(*mut GUnixMountEntry) -> (),
        )),
    );
}
unsafe extern "C" fn safe_c2rust_mount_monitor_start() {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    if !safe_c2rust_get_fstab_file().is_null() {
        file = g_file_new_for_path(safe_c2rust_get_fstab_file());
        safe_c2rust_fstab_monitor = g_file_monitor_file(
            file,
            G_FILE_MONITOR_NONE,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(file as gpointer);
        g_signal_connect_data(
            safe_c2rust_fstab_monitor as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
                >,
                GCallback,
            >(Some(
                safe_c2rust_fstab_file_changed
                    as unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
            )),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            G_CONNECT_DEFAULT,
        );
    }
    if !safe_c2rust_get_mtab_monitor_file().is_null() {
        let mut mtab_path: *const gchar = ::core::ptr::null::<gchar>();
        mtab_path = safe_c2rust_get_mtab_monitor_file() as *const gchar;
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = mtab_path as *const ::core::ffi::c_char;
                let __prefix: *const ::core::ffi::c_char =
                    b"/proc/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_18
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(mtab_path, b"/proc/\0" as *const u8 as *const gchar)
        } != 0
        {
            let mut proc_mounts_channel: *mut GIOChannel = ::core::ptr::null_mut::<GIOChannel>();
            let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
            let mut ret: ::core::ffi::c_int = 0;
            g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
            safe_c2rust_proc_mounts_monitor = mnt_new_monitor();
            ret = mnt_monitor_enable_kernel(safe_c2rust_proc_mounts_monitor, TRUE);
            if ret < 0 as ::core::ffi::c_int {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"mnt_monitor_enable_kernel failed: %s\0" as *const u8 as *const gchar,
                    g_strerror(-(ret as gint)),
                );
            }
            ret = mnt_monitor_enable_userspace(
                safe_c2rust_proc_mounts_monitor,
                TRUE,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if ret < 0 as ::core::ffi::c_int {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"mnt_monitor_enable_userspace failed: %s\0" as *const u8 as *const gchar,
                    g_strerror(-(ret as gint)),
                );
            }
            ret = mnt_monitor_get_fd(safe_c2rust_proc_mounts_monitor);
            if ret >= 0 as ::core::ffi::c_int {
                proc_mounts_channel = g_io_channel_unix_new(ret);
            } else {
                g_set_error_literal(
                    &raw mut error,
                    g_io_error_quark(),
                    g_io_error_from_errno(-(ret as gint)) as gint,
                    g_strerror(-(ret as gint)),
                );
            }
            g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
            if proc_mounts_channel.is_null() {
                if !error.is_null() {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_DEBUG,
                        b"Error creating IO channel for %s: %s (%s, %d); falling back to polling\0"
                            as *const u8 as *const gchar,
                        mtab_path,
                        (*error).message,
                        g_quark_to_string((*error).domain),
                        (*error).code,
                    );
                    g_error_free(error);
                }
                g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
                let mut _pp: *mut *mut libmnt_monitor = &raw mut safe_c2rust_proc_mounts_monitor;
                let mut _ptr: *mut libmnt_monitor = *_pp;
                *_pp = ::core::ptr::null_mut::<libmnt_monitor>();
                if !_ptr.is_null() {
                    mnt_unref_monitor(_ptr as *mut libmnt_monitor);
                }
                safe_c2rust_start_mount_poller();
                g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
            } else {
                g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
                safe_c2rust_proc_mounts_watch_source =
                    g_io_create_watch(proc_mounts_channel, G_IO_IN);
                safe_c2rust_mount_poller_time = g_get_monotonic_time() as guint64;
                g_source_set_callback(
                    safe_c2rust_proc_mounts_watch_source,
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut GIOChannel,
                                GIOCondition,
                                gpointer,
                            ) -> gboolean,
                        >,
                        GSourceFunc,
                    >(Some(
                        safe_c2rust_proc_mounts_changed
                            as unsafe extern "C" fn(
                                *mut GIOChannel,
                                GIOCondition,
                                gpointer,
                            ) -> gboolean,
                    )),
                    NULL_0,
                    None,
                );
                g_source_attach(
                    safe_c2rust_proc_mounts_watch_source,
                    g_main_context_get_thread_default(),
                );
                g_source_unref(safe_c2rust_proc_mounts_watch_source);
                g_io_channel_unref(proc_mounts_channel);
                g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
            }
        } else {
            file = g_file_new_for_path(mtab_path as *const ::core::ffi::c_char);
            safe_c2rust_mtab_monitor = g_file_monitor_file(
                file,
                G_FILE_MONITOR_NONE,
                ::core::ptr::null_mut::<GCancellable>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_object_unref(file as gpointer);
            g_signal_connect_data(
                safe_c2rust_mtab_monitor as gpointer,
                b"changed\0" as *const u8 as *const gchar,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut GFileMonitor,
                            *mut GFile,
                            *mut GFile,
                            GFileMonitorEvent,
                            gpointer,
                        ) -> (),
                    >,
                    GCallback,
                >(Some(
                    safe_c2rust_mtab_file_changed
                        as unsafe extern "C" fn(
                            *mut GFileMonitor,
                            *mut GFile,
                            *mut GFile,
                            GFileMonitorEvent,
                            gpointer,
                        ) -> (),
                )),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
                G_CONNECT_DEFAULT,
            );
        }
    } else {
        g_mutex_lock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
        safe_c2rust_start_mount_poller();
        g_mutex_unlock(&raw mut safe_c2rust_g__proc_mounts_source_lock);
    };
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_finalize(mut object: *mut GObject) {
    let mut monitor: *mut GUnixMountMonitor = ::core::ptr::null_mut::<GUnixMountMonitor>();
    monitor = object as *mut ::core::ffi::c_void as *mut GUnixMountMonitor;
    g_context_specific_group_remove(
        &raw mut safe_c2rust_mount_monitor_group,
        (*monitor).context,
        monitor as gpointer,
        Some(safe_c2rust_mount_monitor_stop as unsafe extern "C" fn() -> ()),
    );
    (*(safe_c2rust_g_unix_mount_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_class_init(
    mut klass: *mut GUnixMountMonitorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_unix_mount_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_signals[MOUNTS_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"mounts-changed\0" as *const u8 as *const gchar),
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        0 as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
    safe_c2rust_signals[MOUNTPOINTS_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"mountpoints-changed\0" as *const u8 as *const gchar),
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        0 as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_init(mut monitor: *mut GUnixMountMonitor) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_set_rate_limit(
    mut mount_monitor: *mut GUnixMountMonitor,
    mut limit_msec: gint,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_get() -> *mut GUnixMountMonitor {
    return g_context_specific_group_get(
        &raw mut safe_c2rust_mount_monitor_group,
        safe_c2rust_g_unix_mount_monitor_get_type(),
        24 as ::core::ffi::c_ulong as goffset,
        Some(safe_c2rust_mount_monitor_start as unsafe extern "C" fn() -> ()),
    ) as *mut GUnixMountMonitor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_monitor_new() -> *mut GUnixMountMonitor {
    return safe_c2rust_g_unix_mount_monitor_get();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_free(mut mount_entry: *mut GUnixMountEntry) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*mount_entry).mount_path as gpointer);
    g_free((*mount_entry).device_path as gpointer);
    g_free((*mount_entry).root_path as gpointer);
    g_free((*mount_entry).filesystem_type as gpointer);
    g_free((*mount_entry).options as gpointer);
    g_free(mount_entry as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_copy(
    mut mount_entry: *mut GUnixMountEntry,
) -> *mut GUnixMountEntry {
    let mut copy: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUnixMountEntry>();
    }
    copy = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GUnixMountEntry>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GUnixMountEntry;
    (*copy).mount_path = safe_c2rust_g_strdup_inline((*mount_entry).mount_path);
    (*copy).device_path = safe_c2rust_g_strdup_inline((*mount_entry).device_path);
    (*copy).root_path = safe_c2rust_g_strdup_inline((*mount_entry).root_path);
    (*copy).filesystem_type = safe_c2rust_g_strdup_inline((*mount_entry).filesystem_type);
    (*copy).options = safe_c2rust_g_strdup_inline((*mount_entry).options);
    (*copy).is_read_only = (*mount_entry).is_read_only;
    (*copy).is_system_internal = (*mount_entry).is_system_internal;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_free(
    mut mount_point: *mut GUnixMountPoint,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*mount_point).mount_path as gpointer);
    g_free((*mount_point).device_path as gpointer);
    g_free((*mount_point).filesystem_type as gpointer);
    g_free((*mount_point).options as gpointer);
    g_free(mount_point as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_copy(
    mut mount_point: *mut GUnixMountPoint,
) -> *mut GUnixMountPoint {
    let mut copy: *mut GUnixMountPoint = ::core::ptr::null_mut::<GUnixMountPoint>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUnixMountPoint>();
    }
    copy = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GUnixMountPoint>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GUnixMountPoint;
    (*copy).mount_path = safe_c2rust_g_strdup_inline((*mount_point).mount_path);
    (*copy).device_path = safe_c2rust_g_strdup_inline((*mount_point).device_path);
    (*copy).filesystem_type = safe_c2rust_g_strdup_inline((*mount_point).filesystem_type);
    (*copy).options = safe_c2rust_g_strdup_inline((*mount_point).options);
    (*copy).is_read_only = (*mount_point).is_read_only;
    (*copy).is_user_mountable = (*mount_point).is_user_mountable;
    (*copy).is_loopback = (*mount_point).is_loopback;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_compare(
    mut mount1: *mut GUnixMountEntry,
    mut mount2: *mut GUnixMountEntry,
) -> gint {
    let mut res: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !mount1.is_null() && !mount2.is_null() {
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
            b"mount1 != NULL && mount2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    res = g_strcmp0((*mount1).mount_path, (*mount2).mount_path);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = g_strcmp0((*mount1).device_path, (*mount2).device_path);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = g_strcmp0((*mount1).root_path, (*mount2).root_path);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = g_strcmp0((*mount1).filesystem_type, (*mount2).filesystem_type);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = g_strcmp0((*mount1).options, (*mount2).options);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = ((*mount1).is_read_only - (*mount2).is_read_only) as ::core::ffi::c_int;
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    return 0 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_get_mount_path(
    mut mount_entry: *mut GUnixMountEntry,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_entry).mount_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_get_device_path(
    mut mount_entry: *mut GUnixMountEntry,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_entry).device_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_get_root_path(
    mut mount_entry: *mut GUnixMountEntry,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_entry).root_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_get_fs_type(
    mut mount_entry: *mut GUnixMountEntry,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_entry).filesystem_type;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_get_options(
    mut mount_entry: *mut GUnixMountEntry,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_entry).options;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_is_readonly(
    mut mount_entry: *mut GUnixMountEntry,
) -> gboolean {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*mount_entry).is_read_only;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_is_system_internal(
    mut mount_entry: *mut GUnixMountEntry,
) -> gboolean {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*mount_entry).is_system_internal;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_compare(
    mut mount1: *mut GUnixMountPoint,
    mut mount2: *mut GUnixMountPoint,
) -> gint {
    let mut res: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !mount1.is_null() && !mount2.is_null() {
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
            b"mount1 != NULL && mount2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    res = g_strcmp0((*mount1).mount_path, (*mount2).mount_path);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = g_strcmp0((*mount1).device_path, (*mount2).device_path);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = g_strcmp0((*mount1).filesystem_type, (*mount2).filesystem_type);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = g_strcmp0((*mount1).options, (*mount2).options);
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = ((*mount1).is_read_only - (*mount2).is_read_only) as ::core::ffi::c_int;
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = ((*mount1).is_user_mountable - (*mount2).is_user_mountable) as ::core::ffi::c_int;
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    res = ((*mount1).is_loopback - (*mount2).is_loopback) as ::core::ffi::c_int;
    if res != 0 as ::core::ffi::c_int {
        return res as gint;
    }
    return 0 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_get_mount_path(
    mut mount_point: *mut GUnixMountPoint,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_point).mount_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_get_device_path(
    mut mount_point: *mut GUnixMountPoint,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_point).device_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_get_fs_type(
    mut mount_point: *mut GUnixMountPoint,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_point).filesystem_type;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_get_options(
    mut mount_point: *mut GUnixMountPoint,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*mount_point).options;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_is_readonly(
    mut mount_point: *mut GUnixMountPoint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*mount_point).is_read_only;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_is_user_mountable(
    mut mount_point: *mut GUnixMountPoint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*mount_point).is_user_mountable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_is_loopback(
    mut mount_point: *mut GUnixMountPoint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*mount_point).is_loopback;
}
unsafe extern "C" fn safe_c2rust_guess_mount_type(
    mut mount_path: *const ::core::ffi::c_char,
    mut device_path: *const ::core::ffi::c_char,
    mut filesystem_type: *const ::core::ffi::c_char,
) -> GUnixMountType {
    let mut type_0: GUnixMountType = G_UNIX_MOUNT_TYPE_UNKNOWN;
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    type_0 = G_UNIX_MOUNT_TYPE_UNKNOWN;
    if strcmp(
        filesystem_type,
        b"udf\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            filesystem_type,
            b"iso9660\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            filesystem_type,
            b"cd9660\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        type_0 = G_UNIX_MOUNT_TYPE_CDROM;
    } else if strcmp(
        filesystem_type,
        b"nfs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            filesystem_type,
            b"nfs4\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        type_0 = G_UNIX_MOUNT_TYPE_NFS;
    } else if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = device_path;
            let __prefix: *const ::core::ffi::c_char =
                b"/vol/dev/diskette/\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_39 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_39 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_39
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(
            device_path as *const gchar,
            b"/vol/dev/diskette/\0" as *const u8 as *const gchar,
        )
    }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = device_path;
                let __prefix: *const ::core::ffi::c_char =
                    b"/dev/fd\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_40 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_40 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_40
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                device_path as *const gchar,
                b"/dev/fd\0" as *const u8 as *const gchar,
            )
        }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = device_path;
                let __prefix: *const ::core::ffi::c_char =
                    b"/dev/floppy\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_41 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_41 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_41
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                device_path as *const gchar,
                b"/dev/floppy\0" as *const u8 as *const gchar,
            )
        }) != 0
    {
        type_0 = G_UNIX_MOUNT_TYPE_FLOPPY;
    } else if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = device_path;
            let __prefix: *const ::core::ffi::c_char =
                b"/dev/cdrom\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_42 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_42 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_42
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(
            device_path as *const gchar,
            b"/dev/cdrom\0" as *const u8 as *const gchar,
        )
    }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = device_path;
                let __prefix: *const ::core::ffi::c_char =
                    b"/dev/acd\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_43 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_43 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_43
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                device_path as *const gchar,
                b"/dev/acd\0" as *const u8 as *const gchar,
            )
        }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = device_path;
                let __prefix: *const ::core::ffi::c_char =
                    b"/dev/cd\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_44 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_44 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_44
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                device_path as *const gchar,
                b"/dev/cd\0" as *const u8 as *const gchar,
            )
        }) != 0
    {
        type_0 = G_UNIX_MOUNT_TYPE_CDROM;
    } else if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = device_path;
            let __prefix: *const ::core::ffi::c_char =
                b"/vol/\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_45 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_45 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_45
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(
            device_path as *const gchar,
            b"/vol/\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        let mut name: *const ::core::ffi::c_char =
            mount_path.offset(strlen(b"/\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = name;
                let __prefix: *const ::core::ffi::c_char =
                    b"cdrom\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_46 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_46 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_46
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                name as *const gchar,
                b"cdrom\0" as *const u8 as *const gchar,
            )
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_CDROM;
        } else if (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = name;
                let __prefix: *const ::core::ffi::c_char =
                    b"floppy\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_47 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_47 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_47
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                name as *const gchar,
                b"floppy\0" as *const u8 as *const gchar,
            )
        }) != 0
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = device_path;
                    let __prefix: *const ::core::ffi::c_char =
                        b"/vol/dev/diskette/\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_48
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(
                    device_path as *const gchar,
                    b"/vol/dev/diskette/\0" as *const u8 as *const gchar,
                )
            }) != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_FLOPPY;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = name;
                let __prefix: *const ::core::ffi::c_char =
                    b"rmdisk\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_49 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_49 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_49
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                name as *const gchar,
                b"rmdisk\0" as *const u8 as *const gchar,
            )
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_ZIP;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = name;
                let __prefix: *const ::core::ffi::c_char =
                    b"jaz\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_50 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_50 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_50
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(name as *const gchar, b"jaz\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_JAZ;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = name;
                let __prefix: *const ::core::ffi::c_char =
                    b"memstick\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_51 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_51 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_51
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(
                name as *const gchar,
                b"memstick\0" as *const u8 as *const gchar,
            )
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_MEMSTICK;
        }
    } else {
        basename = g_path_get_basename(mount_path as *const gchar) as *mut ::core::ffi::c_char;
        if (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"cdr\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_52 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_52 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_52
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"cdr\0" as *const u8 as *const gchar)
        }) != 0
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = basename;
                    let __prefix: *const ::core::ffi::c_char =
                        b"cdwriter\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_53
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(basename, b"cdwriter\0" as *const u8 as *const gchar)
            }) != 0
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = basename;
                    let __prefix: *const ::core::ffi::c_char =
                        b"burn\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_54
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(basename, b"burn\0" as *const u8 as *const gchar)
            }) != 0
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = basename;
                    let __prefix: *const ::core::ffi::c_char =
                        b"dvdr\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_55
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(basename, b"dvdr\0" as *const u8 as *const gchar)
            }) != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_CDROM;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"floppy\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_56 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_56 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_56
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"floppy\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_FLOPPY;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"zip\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_57 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_57 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_57
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"zip\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_ZIP;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"jaz\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_58 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_58 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_58
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"jaz\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_JAZ;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"camera\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_59 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_59 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_59
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"camera\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_CAMERA;
        } else if (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"memstick\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_60 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_60 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_60
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"memstick\0" as *const u8 as *const gchar)
        }) != 0
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = basename;
                    let __prefix: *const ::core::ffi::c_char =
                        b"memory_stick\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_61
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(basename, b"memory_stick\0" as *const u8 as *const gchar)
            }) != 0
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = basename;
                    let __prefix: *const ::core::ffi::c_char =
                        b"ram\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_62
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(basename, b"ram\0" as *const u8 as *const gchar)
            }) != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_MEMSTICK;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"compact_flash\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_63 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_63 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_63
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"compact_flash\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_CF;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"smart_media\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_64 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_64 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_64
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"smart_media\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_SM;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"sd_mmc\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_65 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_65 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_65
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"sd_mmc\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_SDMMC;
        } else if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __prefix: *const ::core::ffi::c_char =
                    b"ipod\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_66 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_66 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_66
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(basename, b"ipod\0" as *const u8 as *const gchar)
        } != 0
        {
            type_0 = G_UNIX_MOUNT_TYPE_IPOD;
        }
        g_free(basename as gpointer);
    }
    if type_0 as ::core::ffi::c_uint
        == G_UNIX_MOUNT_TYPE_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        type_0 = G_UNIX_MOUNT_TYPE_HD;
    }
    return type_0;
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_guess_type(
    mut mount_entry: *mut GUnixMountEntry,
) -> GUnixMountType {
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !mount_entry.is_null() {
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
            b"mount_entry != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if !(*mount_entry).mount_path.is_null() {
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
            b"mount_entry->mount_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if !(*mount_entry).device_path.is_null() {
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
            b"mount_entry->device_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if !(*mount_entry).filesystem_type.is_null() {
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
            b"mount_entry->filesystem_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    return safe_c2rust_guess_mount_type(
        (*mount_entry).mount_path,
        (*mount_entry).device_path,
        (*mount_entry).filesystem_type,
    );
}
unsafe extern "C" fn safe_c2rust_g_unix_mount_point_guess_type(
    mut mount_point: *mut GUnixMountPoint,
) -> GUnixMountType {
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !mount_point.is_null() {
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
            b"mount_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if !(*mount_point).mount_path.is_null() {
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
            b"mount_point->mount_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if !(*mount_point).device_path.is_null() {
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
            b"mount_point->device_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if !(*mount_point).filesystem_type.is_null() {
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
            b"mount_point->filesystem_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_UNIX_MOUNT_TYPE_UNKNOWN;
    }
    return safe_c2rust_guess_mount_type(
        (*mount_point).mount_path,
        (*mount_point).device_path,
        (*mount_point).filesystem_type,
    );
}
unsafe extern "C" fn safe_c2rust_type_to_icon(
    mut type_0: GUnixMountType,
    mut is_mount_point: gboolean,
    mut use_symbolic: gboolean,
) -> *const ::core::ffi::c_char {
    let mut icon_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match type_0 as ::core::ffi::c_uint {
        12 => {
            if is_mount_point != 0 {
                icon_name = if use_symbolic != 0 {
                    b"drive-removable-media-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-removable-media\0" as *const u8 as *const ::core::ffi::c_char
                };
            } else {
                icon_name = if use_symbolic != 0 {
                    b"drive-harddisk-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-harddisk\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
        }
        1 | 4 | 5 => {
            if is_mount_point != 0 {
                icon_name = if use_symbolic != 0 {
                    b"drive-removable-media-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-removable-media\0" as *const u8 as *const ::core::ffi::c_char
                };
            } else {
                icon_name = if use_symbolic != 0 {
                    b"media-removable-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"media-floppy\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
        }
        2 => {
            if is_mount_point != 0 {
                icon_name = if use_symbolic != 0 {
                    b"drive-optical-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-optical\0" as *const u8 as *const ::core::ffi::c_char
                };
            } else {
                icon_name = if use_symbolic != 0 {
                    b"media-optical-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"media-optical\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
        }
        3 => {
            icon_name = if use_symbolic != 0 {
                b"folder-remote-symbolic\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"folder-remote\0" as *const u8 as *const ::core::ffi::c_char
            };
        }
        6 => {
            if is_mount_point != 0 {
                icon_name = if use_symbolic != 0 {
                    b"drive-removable-media-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-removable-media\0" as *const u8 as *const ::core::ffi::c_char
                };
            } else {
                icon_name = if use_symbolic != 0 {
                    b"media-removable-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"media-flash\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
        }
        11 => {
            if is_mount_point != 0 {
                icon_name = if use_symbolic != 0 {
                    b"drive-removable-media-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-removable-media\0" as *const u8 as *const ::core::ffi::c_char
                };
            } else {
                icon_name = if use_symbolic != 0 {
                    b"camera-photo-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"camera-photo\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
        }
        10 => {
            if is_mount_point != 0 {
                icon_name = if use_symbolic != 0 {
                    b"drive-removable-media-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-removable-media\0" as *const u8 as *const ::core::ffi::c_char
                };
            } else {
                icon_name = if use_symbolic != 0 {
                    b"multimedia-player-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"multimedia-player\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
        }
        0 | _ => {
            if is_mount_point != 0 {
                icon_name = if use_symbolic != 0 {
                    b"drive-removable-media-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-removable-media\0" as *const u8 as *const ::core::ffi::c_char
                };
            } else {
                icon_name = if use_symbolic != 0 {
                    b"drive-harddisk-symbolic\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"drive-harddisk\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
        }
    }
    return icon_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_guess_name(
    mut mount_entry: *mut GUnixMountEntry,
) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strcmp(
        (*mount_entry).mount_path,
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = safe_c2rust_g_strdup_inline(glib_gettext(
            b"Filesystem root\0" as *const u8 as *const gchar,
        ) as *const ::core::ffi::c_char);
    } else {
        name = g_filename_display_basename((*mount_entry).mount_path) as *mut ::core::ffi::c_char;
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_guess_icon(
    mut mount_entry: *mut GUnixMountEntry,
) -> *mut GIcon {
    return g_themed_icon_new_with_default_fallbacks(safe_c2rust_type_to_icon(
        safe_c2rust_g_unix_mount_guess_type(mount_entry),
        FALSE,
        FALSE,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_guess_symbolic_icon(
    mut mount_entry: *mut GUnixMountEntry,
) -> *mut GIcon {
    return g_themed_icon_new_with_default_fallbacks(safe_c2rust_type_to_icon(
        safe_c2rust_g_unix_mount_guess_type(mount_entry),
        FALSE,
        TRUE,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_guess_name(
    mut mount_point: *mut GUnixMountPoint,
) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strcmp(
        (*mount_point).mount_path,
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = safe_c2rust_g_strdup_inline(glib_gettext(
            b"Filesystem root\0" as *const u8 as *const gchar,
        ) as *const ::core::ffi::c_char);
    } else {
        name = g_filename_display_basename((*mount_point).mount_path) as *mut ::core::ffi::c_char;
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_guess_icon(
    mut mount_point: *mut GUnixMountPoint,
) -> *mut GIcon {
    return g_themed_icon_new_with_default_fallbacks(safe_c2rust_type_to_icon(
        safe_c2rust_g_unix_mount_point_guess_type(mount_point),
        TRUE,
        FALSE,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_guess_symbolic_icon(
    mut mount_point: *mut GUnixMountPoint,
) -> *mut GIcon {
    return g_themed_icon_new_with_default_fallbacks(safe_c2rust_type_to_icon(
        safe_c2rust_g_unix_mount_point_guess_type(mount_point),
        TRUE,
        TRUE,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_guess_can_eject(
    mut mount_entry: *mut GUnixMountEntry,
) -> gboolean {
    let mut guessed_type: GUnixMountType = G_UNIX_MOUNT_TYPE_UNKNOWN;
    guessed_type = safe_c2rust_g_unix_mount_guess_type(mount_entry);
    if guessed_type as ::core::ffi::c_uint
        == G_UNIX_MOUNT_TYPE_IPOD as ::core::ffi::c_int as ::core::ffi::c_uint
        || guessed_type as ::core::ffi::c_uint
            == G_UNIX_MOUNT_TYPE_CDROM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_guess_should_display(
    mut mount_entry: *mut GUnixMountEntry,
) -> gboolean {
    let mut mount_path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut user_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut user_name_len: gsize = 0;
    if safe_c2rust_g_unix_mount_is_system_internal(mount_entry) != 0 {
        return FALSE;
    }
    mount_path = (*mount_entry).mount_path;
    if !mount_path.is_null() {
        let running_as_root: gboolean = (getuid() == 0 as __uid_t) as ::core::ffi::c_int;
        let mut is_in_runtime_dir: gboolean = FALSE;
        if !g_strstr_len(
            mount_path as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            b"/.\0" as *const u8 as *const gchar,
        )
        .is_null()
        {
            return FALSE;
        }
        if running_as_root != 0 {
            if strncmp(
                mount_path,
                b"/run/media/\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"/run/media/\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int
            {
                is_in_runtime_dir = TRUE as gboolean;
            }
        } else {
            user_name = g_get_user_name();
            user_name_len = strlen(user_name as *const ::core::ffi::c_char) as gsize;
            if strncmp(
                mount_path,
                b"/run/media/\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"/run/media/\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int
                && strncmp(
                    mount_path.offset(strlen(
                        b"/run/media/\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as isize),
                    user_name as *const ::core::ffi::c_char,
                    user_name_len as size_t,
                ) == 0 as ::core::ffi::c_int
                && *mount_path.offset(
                    strlen(b"/run/media/\0" as *const u8 as *const ::core::ffi::c_char)
                        .wrapping_add(user_name_len as size_t) as isize,
                ) as ::core::ffi::c_int
                    == '/' as i32
            {
                is_in_runtime_dir = TRUE as gboolean;
            }
        }
        if is_in_runtime_dir != 0
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = mount_path;
                    let __prefix: *const ::core::ffi::c_char =
                        b"/media/\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_75
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(
                    mount_path as *const gchar,
                    b"/media/\0" as *const u8 as *const gchar,
                )
            }) != 0
        {
            let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            path = g_path_get_dirname(mount_path as *const gchar) as *mut ::core::ffi::c_char;
            if if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = path;
                    let __prefix: *const ::core::ffi::c_char =
                        b"/media/\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_76
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                g_str_has_prefix(path, b"/media/\0" as *const u8 as *const gchar)
            } != 0
            {
                if g_access(path, R_OK | X_OK) != 0 as ::core::ffi::c_int {
                    g_free(path as gpointer);
                    return FALSE;
                }
            }
            g_free(path as gpointer);
            if !(*mount_entry).device_path.is_null()
                && *(*mount_entry)
                    .device_path
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '/' as i32
            {
                let mut st: stat = stat {
                    st_dev: 0,
                    st_ino: 0,
                    st_nlink: 0,
                    st_mode: 0,
                    st_uid: 0,
                    st_gid: 0,
                    __pad0: 0,
                    st_rdev: 0,
                    st_size: 0,
                    st_blksize: 0,
                    st_blocks: 0,
                    st_atim: timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    st_mtim: timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    st_ctim: timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    __glibc_reserved: [0; 3],
                };
                if stat((*mount_entry).device_path, &raw mut st) == 0 as ::core::ffi::c_int
                    && st.st_mode & __S_IFMT as __mode_t == 0o60000 as __mode_t
                    && g_access(mount_path as *const gchar, R_OK | X_OK) != 0 as ::core::ffi::c_int
                {
                    return FALSE;
                }
            }
            return TRUE;
        }
        if (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = mount_path;
                let __prefix: *const ::core::ffi::c_char =
                    g_get_home_dir() as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_77 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_77 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_77
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(mount_path as *const gchar, g_get_home_dir())
        }) != 0
            && *mount_path.offset(strlen(g_get_home_dir() as *const ::core::ffi::c_char) as isize)
                as ::core::ffi::c_int
                == G_DIR_SEPARATOR
        {
            return TRUE;
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_mount_point_guess_can_eject(
    mut mount_point: *mut GUnixMountPoint,
) -> gboolean {
    let mut guessed_type: GUnixMountType = G_UNIX_MOUNT_TYPE_UNKNOWN;
    guessed_type = safe_c2rust_g_unix_mount_point_guess_type(mount_point);
    if guessed_type as ::core::ffi::c_uint
        == G_UNIX_MOUNT_TYPE_IPOD as ::core::ffi::c_int as ::core::ffi::c_uint
        || guessed_type as ::core::ffi::c_uint
            == G_UNIX_MOUNT_TYPE_CDROM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__canonicalize_filename(mut filename: *mut gchar) {
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut last_was_slash: gboolean = FALSE;
    p = filename;
    q = filename;
    while *p != 0 {
        if *p as ::core::ffi::c_int == G_DIR_SEPARATOR {
            if last_was_slash == 0 {
                let fresh0 = q;
                q = q.offset(1);
                *fresh0 = G_DIR_SEPARATOR as gchar;
            }
            last_was_slash = TRUE as gboolean;
        } else if last_was_slash != 0 && *p as ::core::ffi::c_int == '.' as i32 {
            if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == G_DIR_SEPARATOR
                || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
            {
                if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
                {
                    break;
                }
                p = p.offset(1 as ::core::ffi::c_int as isize);
            } else if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32
                && (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == G_DIR_SEPARATOR
                    || *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '\0' as i32)
            {
                if q > filename.offset(1 as ::core::ffi::c_int as isize) {
                    q = q.offset(-1);
                    while q > filename.offset(1 as ::core::ffi::c_int as isize)
                        && *q.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                            != G_DIR_SEPARATOR
                    {
                        q = q.offset(-1);
                    }
                }
                if *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
                {
                    break;
                }
                p = p.offset(2 as ::core::ffi::c_int as isize);
            } else {
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = *p;
                last_was_slash = FALSE as gboolean;
            }
        } else {
            let fresh2 = q;
            q = q.offset(1);
            *fresh2 = *p;
            last_was_slash = FALSE as gboolean;
        }
        p = p.offset(1);
    }
    if q > filename.offset(1 as ::core::ffi::c_int as isize)
        && *q.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int == G_DIR_SEPARATOR
    {
        q = q.offset(-1);
    }
    *q = '\0' as i32 as gchar;
}
unsafe extern "C" fn safe_c2rust__resolve_symlink(
    mut file: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut link: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut f: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut f1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    f = safe_c2rust_g_strdup_inline(file);
    while g_file_test(f, G_FILE_TEST_IS_SYMLINK) != 0 {
        link = g_file_read_link(f, &raw mut error) as *mut ::core::ffi::c_char;
        if link.is_null() {
            g_error_free(error);
            g_free(f as gpointer);
            f = ::core::ptr::null_mut::<::core::ffi::c_char>();
            break;
        } else {
            dir = g_path_get_dirname(f) as *mut ::core::ffi::c_char;
            f1 = g_strdup_printf(b"%s/%s\0" as *const u8 as *const gchar, dir, link)
                as *mut ::core::ffi::c_char;
            g_free(dir as gpointer);
            g_free(link as gpointer);
            g_free(f as gpointer);
            f = f1;
        }
    }
    if !f.is_null() {
        safe_c2rust__canonicalize_filename(f as *mut gchar);
    }
    return f;
}
unsafe extern "C" fn safe_c2rust__resolve_dev_root() -> *const ::core::ffi::c_char {
    let mut current_block: u64;
    static mut safe_c2rust_have_real_dev_root: gboolean = FALSE;
    static mut safe_c2rust_real_dev_root: [::core::ffi::c_char; 256] = [0; 256];
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if !(safe_c2rust_have_real_dev_root != 0) {
        safe_c2rust_have_real_dev_root = TRUE as gboolean;
        if stat(
            b"/dev/root\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut statbuf,
        ) == 0 as ::core::ffi::c_int
        {
            if !(statbuf.st_mode & __S_IFMT as __mode_t == 0o120000 as __mode_t) {
                let mut root_dev: dev_t = statbuf.st_dev as dev_t;
                let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
                f = fopen(
                    b"/etc/mtab\0" as *const u8 as *const ::core::ffi::c_char,
                    b"re\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if !f.is_null() {
                    let mut entp: *mut mntent = ::core::ptr::null_mut::<mntent>();
                    let mut ent: mntent = mntent {
                        mnt_fsname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        mnt_dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        mnt_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        mnt_opts: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        mnt_freq: 0,
                        mnt_passno: 0,
                    };
                    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
                    loop {
                        entp = getmntent_r(
                            f,
                            &raw mut ent,
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                                as ::core::ffi::c_int,
                        );
                        if entp.is_null() {
                            current_block = 7651349459974463963;
                            break;
                        }
                        if !(stat((*entp).mnt_fsname, &raw mut statbuf) == 0 as ::core::ffi::c_int
                            && statbuf.st_dev == root_dev)
                        {
                            continue;
                        }
                        strncpy(
                            &raw mut safe_c2rust_real_dev_root as *mut ::core::ffi::c_char,
                            (*entp).mnt_fsname,
                            (::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t)
                                .wrapping_sub(1 as size_t),
                        );
                        safe_c2rust_real_dev_root
                            [(::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
                                .wrapping_sub(1 as usize) as usize] =
                            '\0' as i32 as ::core::ffi::c_char;
                        fclose(f);
                        current_block = 10160457471091490206;
                        break;
                    }
                    match current_block {
                        10160457471091490206 => {}
                        _ => {
                            endmntent(f);
                            current_block = 2838571290723028321;
                        }
                    }
                } else {
                    current_block = 2838571290723028321;
                }
            } else {
                let mut resolved: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                resolved = safe_c2rust__resolve_symlink(
                    b"/dev/root\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if !resolved.is_null() {
                    strncpy(
                        &raw mut safe_c2rust_real_dev_root as *mut ::core::ffi::c_char,
                        resolved,
                        (::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t)
                            .wrapping_sub(1 as size_t),
                    );
                    safe_c2rust_real_dev_root[(::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                        as usize)
                        .wrapping_sub(1 as usize)
                        as usize] = '\0' as i32 as ::core::ffi::c_char;
                    g_free(resolved as gpointer);
                    current_block = 10160457471091490206;
                } else {
                    current_block = 2838571290723028321;
                }
            }
        } else {
            current_block = 2838571290723028321;
        }
        match current_block {
            10160457471091490206 => {}
            _ => {
                strcpy(
                    &raw mut safe_c2rust_real_dev_root as *mut ::core::ffi::c_char,
                    b"/dev/root\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    }
    return &raw mut safe_c2rust_real_dev_root as *mut ::core::ffi::c_char;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const GLIB_LOCALSTATEDIR: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"/var/local\0") };
pub const GLIB_RUNSTATEDIR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"/run\0") };
