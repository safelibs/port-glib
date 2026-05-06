use ::c2rust_bitfields;
extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GMainContext;
    pub type _GAppInfo;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GFileEnumeratorPrivate;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileInputStreamPrivate;
    pub type _GFileOutputStreamPrivate;
    pub type _GFileIOStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GMount;
    pub type _GMountOperationPrivate;
    pub type _GTask;
    pub type _GFileDescriptorBased;
    pub type _GLocalFile;
    pub type _GLocalFileOutputStreamPrivate;
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn copy_file_range(
        __infd: ::core::ffi::c_int,
        __pinoff: *mut __off64_t,
        __outfd: ::core::ffi::c_int,
        __poutoff: *mut __off64_t,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
    fn splice(
        __fdin: ::core::ffi::c_int,
        __offin: *mut __off64_t,
        __fdout: ::core::ffi::c_int,
        __offout: *mut __off64_t,
        __len: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> __ssize_t;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_byte_array_new() -> *mut GByteArray;
    fn g_byte_array_free(array: *mut GByteArray, free_segment: gboolean) -> *mut guint8;
    fn g_byte_array_set_size(array: *mut GByteArray, length: guint) -> *mut GByteArray;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_static(data: gconstpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_bytes_ref(bytes: *mut GBytes) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_file_error_quark() -> GQuark;
    fn g_file_open_tmp(
        tmpl: *const gchar,
        name_used: *mut *mut gchar,
        error: *mut *mut GError,
    ) -> gint;
    fn g_dir_make_tmp(tmpl: *const gchar, error: *mut *mut GError) -> *mut gchar;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_build_filenamev(args: *mut *mut gchar) -> *mut gchar;
    fn g_build_filename_valist(
        first_element: *const gchar,
        args: *mut ::core::ffi::VaList,
    ) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_get_current_dir() -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_main_context_invoke_full(
        context: *mut GMainContext,
        priority: gint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_strfreev(str_array: *mut *mut gchar);
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
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_uri_unescape_string(
        escaped_string: *const ::core::ffi::c_char,
        illegal_characters: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_get_qdata(object: *mut GObject, quark: GQuark) -> gpointer;
    fn g_object_replace_qdata(
        object: *mut GObject,
        quark: GQuark,
        oldval: gpointer,
        newval: gpointer,
        destroy: GDestroyNotify,
        old_destroy: *mut GDestroyNotify,
    ) -> gboolean;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn g_unix_open_pipe(fds: *mut gint, flags: gint, error: *mut *mut GError) -> gboolean;
    fn g_vfs_get_file_for_path(vfs: *mut GVfs, path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_vfs_get_file_for_uri(vfs: *mut GVfs, uri: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_vfs_parse_name(vfs: *mut GVfs, parse_name: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_vfs_get_default() -> *mut GVfs;
    fn g_task_get_type() -> GType;
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
    fn g_task_set_check_cancellable(task: *mut GTask, check_cancellable: gboolean);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_context(task: *mut GTask) -> *mut GMainContext;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_source_tag(task: *mut GTask) -> gpointer;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
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
    fn g_task_return_error_if_cancelled(task: *mut GTask) -> gboolean;
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_file_attribute_info_list_new() -> *mut GFileAttributeInfoList;
    fn g_file_attribute_info_list_unref(list: *mut GFileAttributeInfoList);
    fn g_file_info_get_type() -> GType;
    fn g_file_info_dup(other: *mut GFileInfo) -> *mut GFileInfo;
    fn g_file_info_has_attribute(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_file_info_list_attributes(
        info: *mut GFileInfo,
        name_space: *const ::core::ffi::c_char,
    ) -> *mut *mut ::core::ffi::c_char;
    fn g_file_info_get_attribute_string(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn g_file_info_clear_status(info: *mut GFileInfo);
    fn g_file_info_get_file_type(info: *mut GFileInfo) -> GFileType;
    fn g_file_info_get_content_type(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn g_file_info_get_size(info: *mut GFileInfo) -> goffset;
    fn g_file_info_get_symlink_target(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn g_file_info_get_etag(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn _g_file_attribute_value_peek_as_pointer(attr: *mut GFileAttributeValue) -> gpointer;
    fn _g_file_info_get_attribute_value(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> *mut GFileAttributeValue;
    fn g_app_info_get_default_for_type(
        content_type: *const ::core::ffi::c_char,
        must_support_uris: gboolean,
    ) -> *mut GAppInfo;
    fn g_app_info_get_default_for_type_async(
        content_type: *const ::core::ffi::c_char,
        must_support_uris: gboolean,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_app_info_get_default_for_type_finish(
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GAppInfo;
    fn g_app_info_get_default_for_uri_scheme(
        uri_scheme: *const ::core::ffi::c_char,
    ) -> *mut GAppInfo;
    fn g_app_info_get_default_for_uri_scheme_async(
        uri_scheme: *const ::core::ffi::c_char,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_app_info_get_default_for_uri_scheme_finish(
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GAppInfo;
    fn g_async_result_get_type() -> GType;
    fn g_async_result_get_user_data(res: *mut GAsyncResult) -> gpointer;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_async_result_is_tagged(res: *mut GAsyncResult, source_tag: gpointer) -> gboolean;
    fn g_input_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_close(
        stream: *mut GInputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_input_stream_close_async(
        stream: *mut GInputStream,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_input_stream_close_finish(
        stream: *mut GInputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_output_stream_write(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
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
    fn g_output_stream_close_async(
        stream: *mut GOutputStream,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_output_stream_close_finish(
        stream: *mut GOutputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_cancellable_get_type() -> GType;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_input_stream_query_info(
        stream: *mut GFileInputStream,
        attributes: *const ::core::ffi::c_char,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_input_stream_query_info_async(
        stream: *mut GFileInputStream,
        attributes: *const ::core::ffi::c_char,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_file_input_stream_query_info_finish(
        stream: *mut GFileInputStream,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_io_error_from_file_error(file_error: GFileError) -> GIOErrorEnum;
    fn g_file_output_stream_get_etag(stream: *mut GFileOutputStream) -> *mut ::core::ffi::c_char;
    fn g_resources_lookup_data(
        path: *const ::core::ffi::c_char,
        lookup_flags: GResourceLookupFlags,
        error: *mut *mut GError,
    ) -> *mut GBytes;
    fn g_file_descriptor_based_get_type() -> GType;
    fn g_file_descriptor_based_get_fd(fd_based: *mut GFileDescriptorBased) -> ::core::ffi::c_int;
    fn _g_poll_file_monitor_new(file: *mut GFile) -> *mut GFileMonitor;
    fn _g_local_file_output_stream_new(fd: ::core::ffi::c_int) -> *mut GFileOutputStream;
    fn _g_local_file_output_stream_create(
        filename: *const ::core::ffi::c_char,
        readable: gboolean,
        flags: GFileCreateFlags,
        reference_info: *mut GFileInfo,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileOutputStream;
    fn _g_local_file_output_stream_replace(
        filename: *const ::core::ffi::c_char,
        readable: gboolean,
        etag: *const ::core::ffi::c_char,
        create_backup: gboolean,
        flags: GFileCreateFlags,
        reference_info: *mut GFileInfo,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileOutputStream;
    fn _g_local_file_io_stream_new(
        output_stream: *mut GLocalFileOutputStream,
    ) -> *mut GFileIOStream;
    fn _g_local_file_get_type() -> GType;
    fn _g_local_file_get_filename(file: *mut GLocalFile) -> *const ::core::ffi::c_char;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __loff_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type size_t = usize;
pub type loff_t = __loff_t;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
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
pub type GFileError = ::core::ffi::c_uint;
pub const G_FILE_ERROR_FAILED: GFileError = 24;
pub const G_FILE_ERROR_NOSYS: GFileError = 23;
pub const G_FILE_ERROR_PERM: GFileError = 22;
pub const G_FILE_ERROR_IO: GFileError = 21;
pub const G_FILE_ERROR_INTR: GFileError = 20;
pub const G_FILE_ERROR_AGAIN: GFileError = 19;
pub const G_FILE_ERROR_PIPE: GFileError = 18;
pub const G_FILE_ERROR_INVAL: GFileError = 17;
pub const G_FILE_ERROR_BADF: GFileError = 16;
pub const G_FILE_ERROR_NFILE: GFileError = 15;
pub const G_FILE_ERROR_MFILE: GFileError = 14;
pub const G_FILE_ERROR_NOMEM: GFileError = 13;
pub const G_FILE_ERROR_NOSPC: GFileError = 12;
pub const G_FILE_ERROR_LOOP: GFileError = 11;
pub const G_FILE_ERROR_FAULT: GFileError = 10;
pub const G_FILE_ERROR_TXTBSY: GFileError = 9;
pub const G_FILE_ERROR_ROFS: GFileError = 8;
pub const G_FILE_ERROR_NODEV: GFileError = 7;
pub const G_FILE_ERROR_NXIO: GFileError = 6;
pub const G_FILE_ERROR_NOTDIR: GFileError = 5;
pub const G_FILE_ERROR_NOENT: GFileError = 4;
pub const G_FILE_ERROR_NAMETOOLONG: GFileError = 3;
pub const G_FILE_ERROR_ACCES: GFileError = 2;
pub const G_FILE_ERROR_ISDIR: GFileError = 1;
pub const G_FILE_ERROR_EXIST: GFileError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GMainContext = _GMainContext;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
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
pub type GType = gsize;
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
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GFileAttributeType = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_TYPE_STRINGV: GFileAttributeType = 9;
pub const G_FILE_ATTRIBUTE_TYPE_OBJECT: GFileAttributeType = 8;
pub const G_FILE_ATTRIBUTE_TYPE_INT64: GFileAttributeType = 7;
pub const G_FILE_ATTRIBUTE_TYPE_UINT64: GFileAttributeType = 6;
pub const G_FILE_ATTRIBUTE_TYPE_INT32: GFileAttributeType = 5;
pub const G_FILE_ATTRIBUTE_TYPE_UINT32: GFileAttributeType = 4;
pub const G_FILE_ATTRIBUTE_TYPE_BOOLEAN: GFileAttributeType = 3;
pub const G_FILE_ATTRIBUTE_TYPE_BYTE_STRING: GFileAttributeType = 2;
pub const G_FILE_ATTRIBUTE_TYPE_STRING: GFileAttributeType = 1;
pub const G_FILE_ATTRIBUTE_TYPE_INVALID: GFileAttributeType = 0;
pub type GFileAttributeInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED: GFileAttributeInfoFlags = 2;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE: GFileAttributeInfoFlags = 1;
pub const G_FILE_ATTRIBUTE_INFO_NONE: GFileAttributeInfoFlags = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING: C2RustUnnamed_0 = 2;
pub const G_FILE_ATTRIBUTE_STATUS_SET: C2RustUnnamed_0 = 1;
pub const G_FILE_ATTRIBUTE_STATUS_UNSET: C2RustUnnamed_0 = 0;
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
pub type GFileCreateFlags = ::core::ffi::c_uint;
pub const G_FILE_CREATE_REPLACE_DESTINATION: GFileCreateFlags = 2;
pub const G_FILE_CREATE_PRIVATE: GFileCreateFlags = 1;
pub const G_FILE_CREATE_NONE: GFileCreateFlags = 0;
pub type GFileMeasureFlags = ::core::ffi::c_uint;
pub const G_FILE_MEASURE_NO_XDEV: GFileMeasureFlags = 8;
pub const G_FILE_MEASURE_APPARENT_SIZE: GFileMeasureFlags = 4;
pub const G_FILE_MEASURE_REPORT_ANY_ERROR: GFileMeasureFlags = 2;
pub const G_FILE_MEASURE_NONE: GFileMeasureFlags = 0;
pub type GMountMountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_MOUNT_NONE: GMountMountFlags = 0;
pub type GMountUnmountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_UNMOUNT_FORCE: GMountUnmountFlags = 1;
pub const G_MOUNT_UNMOUNT_NONE: GMountUnmountFlags = 0;
pub type GDriveStartFlags = ::core::ffi::c_uint;
pub const G_DRIVE_START_NONE: GDriveStartFlags = 0;
pub type GFileCopyFlags = ::core::ffi::c_uint;
pub const G_FILE_COPY_TARGET_DEFAULT_MODIFIED_TIME: GFileCopyFlags = 64;
pub const G_FILE_COPY_TARGET_DEFAULT_PERMS: GFileCopyFlags = 32;
pub const G_FILE_COPY_NO_FALLBACK_FOR_MOVE: GFileCopyFlags = 16;
pub const G_FILE_COPY_ALL_METADATA: GFileCopyFlags = 8;
pub const G_FILE_COPY_NOFOLLOW_SYMLINKS: GFileCopyFlags = 4;
pub const G_FILE_COPY_BACKUP: GFileCopyFlags = 2;
pub const G_FILE_COPY_OVERWRITE: GFileCopyFlags = 1;
pub const G_FILE_COPY_NONE: GFileCopyFlags = 0;
pub type GFileMonitorFlags = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_WATCH_MOVES: GFileMonitorFlags = 8;
pub const G_FILE_MONITOR_WATCH_HARD_LINKS: GFileMonitorFlags = 4;
pub const G_FILE_MONITOR_SEND_MOVED: GFileMonitorFlags = 2;
pub const G_FILE_MONITOR_WATCH_MOUNTS: GFileMonitorFlags = 1;
pub const G_FILE_MONITOR_NONE: GFileMonitorFlags = 0;
pub type GFileType = ::core::ffi::c_uint;
pub const G_FILE_TYPE_MOUNTABLE: GFileType = 6;
pub const G_FILE_TYPE_SHORTCUT: GFileType = 5;
pub const G_FILE_TYPE_SPECIAL: GFileType = 4;
pub const G_FILE_TYPE_SYMBOLIC_LINK: GFileType = 3;
pub const G_FILE_TYPE_DIRECTORY: GFileType = 2;
pub const G_FILE_TYPE_REGULAR: GFileType = 1;
pub const G_FILE_TYPE_UNKNOWN: GFileType = 0;
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
pub type GResourceLookupFlags = ::core::ffi::c_uint;
pub const G_RESOURCE_LOOKUP_FLAGS_NONE: GResourceLookupFlags = 0;
pub type GAppInfo = _GAppInfo;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileEnumerator {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileEnumeratorPrivate,
}
pub type GFileEnumeratorPrivate = _GFileEnumeratorPrivate;
pub type GFileEnumerator = _GFileEnumerator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfo {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: GFileAttributeType,
    pub flags: GFileAttributeInfoFlags,
}
pub type GFileAttributeInfo = _GFileAttributeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfoList {
    pub infos: *mut GFileAttributeInfo,
    pub n_infos: ::core::ffi::c_int,
}
pub type GFileAttributeInfoList = _GFileAttributeInfoList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInputStream {
    pub parent_instance: GInputStream,
    pub priv_0: *mut GFileInputStreamPrivate,
}
pub type GFileInputStreamPrivate = _GFileInputStreamPrivate;
pub type GFileInputStream = _GFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GFileOutputStreamPrivate,
}
pub type GFileOutputStreamPrivate = _GFileOutputStreamPrivate;
pub type GFileOutputStream = _GFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIOStream {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GFileIOStreamPrivate,
}
pub type GFileIOStreamPrivate = _GFileIOStreamPrivate;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GFileIOStream = _GFileIOStream;
pub type GMount = _GMount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountOperation {
    pub parent_instance: GObject,
    pub priv_0: *mut GMountOperationPrivate,
}
pub type GMountOperationPrivate = _GMountOperationPrivate;
pub type GMountOperation = _GMountOperation;
pub type GTask = _GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfs {
    pub parent_instance: GObject,
}
pub type GVfs = _GVfs;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GFileProgressCallback = Option<unsafe extern "C" fn(goffset, goffset, gpointer) -> ()>;
pub type GFileReadMoreCallback =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, goffset, gpointer) -> gboolean>;
pub type GFileMeasureProgressCallback =
    Option<unsafe extern "C" fn(gboolean, guint64, guint64, guint64, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIface {
    pub g_iface: GTypeInterface,
    pub dup: Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>,
    pub hash: Option<unsafe extern "C" fn(*mut GFile) -> guint>,
    pub equal: Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>,
    pub is_native: Option<unsafe extern "C" fn(*mut GFile) -> gboolean>,
    pub has_uri_scheme:
        Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean>,
    pub get_uri_scheme: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_basename: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_path: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_uri: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_parse_name: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_parent: Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>,
    pub prefix_matches: Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>,
    pub get_relative_path:
        Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char>,
    pub resolve_relative_path:
        Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_child_for_display_name: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> *mut GFile,
    >,
    pub enumerate_children: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileEnumerator,
    >,
    pub enumerate_children_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub enumerate_children_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileEnumerator,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
    >,
    pub query_filesystem_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_filesystem_info_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_filesystem_info_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
    >,
    pub find_enclosing_mount: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GMount,
    >,
    pub find_enclosing_mount_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub find_enclosing_mount_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GMount,
    >,
    pub set_display_name: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFile,
    >,
    pub set_display_name_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub set_display_name_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile>,
    pub query_settable_attributes: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileAttributeInfoList,
    >,
    pub _query_settable_attributes_async: Option<unsafe extern "C" fn() -> ()>,
    pub _query_settable_attributes_finish: Option<unsafe extern "C" fn() -> ()>,
    pub query_writable_namespaces: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileAttributeInfoList,
    >,
    pub _query_writable_namespaces_async: Option<unsafe extern "C" fn() -> ()>,
    pub _query_writable_namespaces_finish: Option<unsafe extern "C" fn() -> ()>,
    pub set_attribute: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileAttributeType,
            gpointer,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_attributes_from_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_attributes_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub set_attributes_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GFileInfo,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub read_fn: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInputStream,
    >,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub read_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInputStream,
    >,
    pub append_to: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub append_to_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub append_to_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub create: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub create_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub create_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub replace: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub replace_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub replace_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub delete_file:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub delete_file_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub delete_file_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub trash:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub trash_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub trash_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub make_directory:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub make_directory_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub make_directory_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub make_symbolic_link: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub make_symbolic_link_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub make_symbolic_link_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub copy_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub copy_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub move_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub move_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub mount_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub mount_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile>,
    pub unmount_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub mount_enclosing_volume: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub mount_enclosing_volume_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub monitor_dir: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMonitorFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileMonitor,
    >,
    pub monitor_file: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMonitorFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileMonitor,
    >,
    pub open_readwrite: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub open_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub open_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub create_readwrite: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileIOStream,
    >,
    pub create_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub create_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub replace_readwrite: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileIOStream,
    >,
    pub replace_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub replace_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub start_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GDriveStartFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub start_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub stop_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub stop_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub supports_thread_contexts: gboolean,
    pub unmount_mountable_with_operation: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_mountable_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject_mountable_with_operation: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_mountable_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub poll_mountable: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, GAsyncReadyCallback, gpointer) -> (),
    >,
    pub poll_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub measure_disk_usage: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMeasureFlags,
            *mut GCancellable,
            GFileMeasureProgressCallback,
            gpointer,
            *mut guint64,
            *mut guint64,
            *mut guint64,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub measure_disk_usage_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMeasureFlags,
            gint,
            *mut GCancellable,
            GFileMeasureProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub measure_disk_usage_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut guint64,
            *mut guint64,
            *mut guint64,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
pub type GFileIface = _GFileIface;
pub type GFileInterface = GFileIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeasureResult {
    pub disk_usage: guint64,
    pub num_dirs: guint64,
    pub num_files: guint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeasureProgress {
    pub callback: GFileMeasureProgressCallback,
    pub user_data: gpointer,
    pub reporting: gboolean,
    pub current_size: guint64,
    pub num_dirs: guint64,
    pub num_files: guint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeasureTaskData {
    pub flags: GFileMeasureFlags,
    pub progress_callback: GFileMeasureProgressCallback,
    pub progress_data: gpointer,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyProgressData {
    pub data: *mut CopyAsyncData,
    pub current_num_bytes: goffset,
    pub total_num_bytes: goffset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyAsyncData {
    pub source: *mut GFile,
    pub destination: *mut GFile,
    pub flags: GFileCopyFlags,
    pub progress_cb: GFileProgressCallback,
    pub progress_cb_data: gpointer,
}
pub type GFileDescriptorBased = _GFileDescriptorBased;
pub type GLocalFile = _GLocalFile;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GFileAttributeValue {
    #[bitfield(name = "type_0", ty = "guint", bits = "0..=7")]
    #[bitfield(name = "status", ty = "guint", bits = "8..=15")]
    pub type_0_status: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub boolean: gboolean,
    pub int32: gint32,
    pub uint32: guint32,
    pub int64: gint64,
    pub uint64: guint64,
    pub string: *mut ::core::ffi::c_char,
    pub obj: *mut GObject,
    pub stringv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReplaceRWAsyncData {
    pub etag: *mut ::core::ffi::c_char,
    pub make_backup: gboolean,
    pub flags: GFileCreateFlags,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MoveProgressData {
    pub data: *mut MoveAsyncData,
    pub current_num_bytes: goffset,
    pub total_num_bytes: goffset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MoveAsyncData {
    pub source: *mut GFile,
    pub destination: *mut GFile,
    pub flags: GFileCopyFlags,
    pub progress_cb: GFileProgressCallback,
    pub progress_cb_data: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReplaceAsyncData {
    pub stream: *mut GFileOutputStream,
    pub etag: *mut ::core::ffi::c_char,
    pub make_backup: gboolean,
    pub flags: GFileCreateFlags,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SetInfoAsyncData {
    pub flags: GFileQueryInfoFlags,
    pub info: *mut GFileInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryInfoAsyncData {
    pub attributes: *mut ::core::ffi::c_char,
    pub flags: GFileQueryInfoFlags,
}
pub type GLocalFileOutputStream = _GLocalFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileOutputStream {
    pub parent_instance: GFileOutputStream,
    pub priv_0: *mut GLocalFileOutputStreamPrivate,
}
pub type GLocalFileOutputStreamPrivate = _GLocalFileOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NewTmpAsyncData {
    pub file: *mut GFile,
    pub iostream: *mut GFileIOStream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadContentsData {
    pub task: *mut GTask,
    pub read_more_callback: GFileReadMoreCallback,
    pub content: *mut GByteArray,
    pub pos: gsize,
    pub etag: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReplaceContentsData {
    pub task: *mut GTask,
    pub content: *mut GBytes,
    pub pos: gsize,
    pub etag: *mut ::core::ffi::c_char,
    pub failed: gboolean,
}
pub const _IOC_NRBITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const _IOC_TYPEBITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const _IOC_SIZEBITS: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const _IOC_NRSHIFT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _IOC_TYPESHIFT: ::core::ffi::c_int = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: ::core::ffi::c_int = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: ::core::ffi::c_int = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EXDEV: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const BTRFS_IOC_CLONE: usize = ((1 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0x94 as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((9 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint)
    as usize
    | (::core::mem::size_of::<::core::ffi::c_int>() as usize) << _IOC_SIZESHIFT;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_SETPIPE_SZ: ::core::ffi::c_int = 1031 as ::core::ffi::c_int;
pub const F_GETPIPE_SZ: ::core::ffi::c_int = 1032 as ::core::ffi::c_int;
pub const SPLICE_F_MORE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_FILE_ATTRIBUTE_STANDARD_TYPE: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::type\0") };
pub const G_FILE_ATTRIBUTE_STANDARD_FAST_CONTENT_TYPE: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"standard::fast-content-type\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_SIZE: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::size\0") };
pub const G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"standard::symlink-target\0")
};
pub const G_FILE_ATTRIBUTE_ETAG_VALUE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"etag::value\0") };
pub const G_FILE_ATTRIBUTE_UNIX_MODE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"unix::mode\0") };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GFile\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GFileInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GFileIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_file_default_init as unsafe extern "C" fn(*mut GFileIface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL_0),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_g_file_default_init(mut iface: *mut GFileIface) {
    (*iface).enumerate_children_async = Some(
        safe_c2rust_g_file_real_enumerate_children_async
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).enumerate_children_finish = Some(
        safe_c2rust_g_file_real_enumerate_children_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileEnumerator,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileEnumerator,
        >;
    (*iface).set_display_name_async = Some(
        safe_c2rust_g_file_real_set_display_name_async
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).set_display_name_finish = Some(
        safe_c2rust_g_file_real_set_display_name_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile,
        >;
    (*iface).query_info_async = Some(
        safe_c2rust_g_file_real_query_info_async
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).query_info_finish = Some(
        safe_c2rust_g_file_real_query_info_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
        >;
    (*iface).query_filesystem_info_async = Some(
        safe_c2rust_g_file_real_query_filesystem_info_async
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).query_filesystem_info_finish = Some(
        safe_c2rust_g_file_real_query_filesystem_info_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
        >;
    (*iface).set_attributes_async = Some(
        safe_c2rust_g_file_real_set_attributes_async
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GFileInfo,
                GFileQueryInfoFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GFileInfo,
                GFileQueryInfoFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).set_attributes_finish = Some(
        safe_c2rust_g_file_real_set_attributes_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GFileInfo,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GFileInfo,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).read_async = Some(
        safe_c2rust_g_file_real_read_async
            as unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).read_finish = Some(
        safe_c2rust_g_file_real_read_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileInputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileInputStream,
        >;
    (*iface).append_to_async = Some(
        safe_c2rust_g_file_real_append_to_async
            as unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).append_to_finish = Some(
        safe_c2rust_g_file_real_append_to_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
        >;
    (*iface).create_async = Some(
        safe_c2rust_g_file_real_create_async
            as unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).create_finish = Some(
        safe_c2rust_g_file_real_create_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
        >;
    (*iface).replace_async = Some(
        safe_c2rust_g_file_real_replace_async
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).replace_finish = Some(
        safe_c2rust_g_file_real_replace_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
        >;
    (*iface).delete_file_async = Some(
        safe_c2rust_g_file_real_delete_async
            as unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).delete_file_finish = Some(
        safe_c2rust_g_file_real_delete_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).trash_async = Some(
        safe_c2rust_g_file_real_trash_async
            as unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).trash_finish = Some(
        safe_c2rust_g_file_real_trash_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).move_async = Some(
        safe_c2rust_g_file_real_move_async
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GFile,
                GFileCopyFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GFileProgressCallback,
                gpointer,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GFile,
                GFileCopyFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GFileProgressCallback,
                gpointer,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).move_finish = Some(
        safe_c2rust_g_file_real_move_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).make_directory_async = Some(
        safe_c2rust_g_file_real_make_directory_async
            as unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).make_directory_finish = Some(
        safe_c2rust_g_file_real_make_directory_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).make_symbolic_link_async = Some(
        safe_c2rust_g_file_real_make_symbolic_link_async
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).make_symbolic_link_finish = Some(
        safe_c2rust_g_file_real_make_symbolic_link_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).open_readwrite_async = Some(
        safe_c2rust_g_file_real_open_readwrite_async
            as unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).open_readwrite_finish = Some(
        safe_c2rust_g_file_real_open_readwrite_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
        >;
    (*iface).create_readwrite_async = Some(
        safe_c2rust_g_file_real_create_readwrite_async
            as unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).create_readwrite_finish = Some(
        safe_c2rust_g_file_real_create_readwrite_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
        >;
    (*iface).replace_readwrite_async = Some(
        safe_c2rust_g_file_real_replace_readwrite_async
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).replace_readwrite_finish = Some(
        safe_c2rust_g_file_real_replace_readwrite_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
        >;
    (*iface).find_enclosing_mount_async = Some(
        safe_c2rust_g_file_real_find_enclosing_mount_async
            as unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).find_enclosing_mount_finish = Some(
        safe_c2rust_g_file_real_find_enclosing_mount_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GMount,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GMount,
        >;
    (*iface).set_attributes_from_info = Some(
        safe_c2rust_g_file_real_set_attributes_from_info
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GFileInfo,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GFileInfo,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).copy_async = Some(
        safe_c2rust_g_file_real_copy_async
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GFile,
                GFileCopyFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GFileProgressCallback,
                gpointer,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GFile,
                GFileCopyFlags,
                ::core::ffi::c_int,
                *mut GCancellable,
                GFileProgressCallback,
                gpointer,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).copy_finish = Some(
        safe_c2rust_g_file_real_copy_finish
            as unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).measure_disk_usage = Some(
        safe_c2rust_g_file_real_measure_disk_usage
            as unsafe extern "C" fn(
                *mut GFile,
                GFileMeasureFlags,
                *mut GCancellable,
                GFileMeasureProgressCallback,
                gpointer,
                *mut guint64,
                *mut guint64,
                *mut guint64,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileMeasureFlags,
                *mut GCancellable,
                GFileMeasureProgressCallback,
                gpointer,
                *mut guint64,
                *mut guint64,
                *mut guint64,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).measure_disk_usage_async = Some(
        safe_c2rust_g_file_real_measure_disk_usage_async
            as unsafe extern "C" fn(
                *mut GFile,
                GFileMeasureFlags,
                gint,
                *mut GCancellable,
                GFileMeasureProgressCallback,
                gpointer,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileMeasureFlags,
                gint,
                *mut GCancellable,
                GFileMeasureProgressCallback,
                gpointer,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).measure_disk_usage_finish = Some(
        safe_c2rust_g_file_real_measure_disk_usage_finish
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut guint64,
                *mut guint64,
                *mut guint64,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GAsyncResult,
                *mut guint64,
                *mut guint64,
                *mut guint64,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_is_native(mut file: *mut GFile) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).is_native.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_has_uri_scheme(
    mut file: *mut GFile,
    mut uri_scheme: *const ::core::ffi::c_char,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !uri_scheme.is_null() {
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
            b"uri_scheme != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).has_uri_scheme.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, uri_scheme);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_uri_scheme(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).get_uri_scheme.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_basename(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).get_basename.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_path(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).get_path.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
unsafe extern "C" fn safe_c2rust_file_peek_path_generic(
    mut file: *mut GFile,
) -> *const ::core::ffi::c_char {
    let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    static mut safe_c2rust__file_path_quark: GQuark = 0 as GQuark;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if safe_c2rust__file_path_quark != 0 {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        == 0 as ::core::ffi::c_long
    {
        safe_c2rust__file_path_quark =
            g_quark_from_static_string(b"gio-file-path\0" as *const u8 as *const gchar);
    }
    while FALSE == 0 {
        let mut new_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
        path = g_object_get_qdata(file as *mut GObject, safe_c2rust__file_path_quark)
            as *const ::core::ffi::c_char;
        if !path.is_null() {
            break;
        }
        new_path = safe_c2rust_g_file_get_path(file) as *mut gchar;
        if new_path.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        if g_object_replace_qdata(
            file as *mut GObject,
            safe_c2rust__file_path_quark,
            NULL_0,
            new_path as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ),
            ::core::ptr::null_mut::<GDestroyNotify>(),
        ) != 0
        {
            path = new_path;
            break;
        } else {
            g_free(new_path as gpointer);
        }
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_peek_path(
    mut file: *mut GFile,
) -> *const ::core::ffi::c_char {
    if ({
        let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
        let mut __t: GType = _g_local_file_get_type();
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
        return _g_local_file_get_filename(file as *mut GLocalFile);
    }
    return safe_c2rust_file_peek_path_generic(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_uri(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).get_uri.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_parse_name(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).get_parse_name.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_dup(mut file: *mut GFile) -> *mut GFile {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).dup.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_hash(mut file: gconstpointer) -> guint {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).hash.expect("non-null function pointer"))
        .expect("non-null function pointer")(file as *mut GFile);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_equal(
    mut file1: *mut GFile,
    mut file2: *mut GFile,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file1 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file2 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file2)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if file1 == file2 {
        return TRUE;
    }
    if (*(*(file1 as *mut GTypeInstance)).g_class).g_type
        != (*(*(file2 as *mut GTypeInstance)).g_class).g_type
    {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file1 as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).equal.expect("non-null function pointer"))
        .expect("non-null function pointer")(file1, file2);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_parent(mut file: *mut GFile) -> *mut GFile {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).get_parent.expect("non-null function pointer"))
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_has_parent(
    mut file: *mut GFile,
    mut parent: *mut GFile,
) -> gboolean {
    let mut actual_parent: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut result: gboolean = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if parent.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = parent as *mut GTypeInstance;
                let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"parent == NULL || G_IS_FILE (parent)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    actual_parent = safe_c2rust_g_file_get_parent(file);
    if !actual_parent.is_null() {
        if !parent.is_null() {
            result = safe_c2rust_g_file_equal(parent, actual_parent);
        } else {
            result = TRUE as gboolean;
        }
        g_object_unref(actual_parent as gpointer);
    } else {
        result = FALSE as gboolean;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_child(
    mut file: *mut GFile,
    mut name: *const ::core::ffi::c_char,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if g_path_is_absolute(name as *const gchar) == 0 {
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
            b"!g_path_is_absolute (name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return safe_c2rust_g_file_resolve_relative_path(file, name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_child_for_display_name(
    mut file: *mut GFile,
    mut display_name: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !display_name.is_null() {
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
            b"display_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .get_child_for_display_name
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, display_name, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_has_prefix(
    mut file: *mut GFile,
    mut prefix: *mut GFile,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = prefix as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (prefix)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*(file as *mut GTypeInstance)).g_class).g_type
        != (*(*(prefix as *mut GTypeInstance)).g_class).g_type
    {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).prefix_matches.expect("non-null function pointer"))
        .expect("non-null function pointer")(prefix, file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_relative_path(
    mut parent: *mut GFile,
    mut descendant: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = parent as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (parent)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = descendant as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (descendant)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*(*(parent as *mut GTypeInstance)).g_class).g_type
        != (*(*(descendant as *mut GTypeInstance)).g_class).g_type
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(parent as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .get_relative_path
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(parent, descendant);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_resolve_relative_path(
    mut file: *mut GFile,
    mut relative_path: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !relative_path.is_null() {
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
            b"relative_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .resolve_relative_path
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, relative_path);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerate_children(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileEnumerator {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).enumerate_children.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    return Some(
        (*iface)
            .enumerate_children
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, attributes, flags, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerate_children_async(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .enumerate_children_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        attributes,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_enumerate_children_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileEnumerator {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .enumerate_children_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_exists(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
) -> gboolean {
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE(file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    info = safe_c2rust_g_file_query_info(
        file,
        G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr(),
        G_FILE_QUERY_INFO_NONE,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !info.is_null() {
        g_object_unref(info as gpointer);
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_file_type(
    mut file: *mut GFile,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
) -> GFileType {
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut file_type: GFileType = G_FILE_TYPE_UNKNOWN;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE(file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_FILE_TYPE_UNKNOWN;
    }
    info = safe_c2rust_g_file_query_info(
        file,
        G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr(),
        flags,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !info.is_null() {
        file_type = g_file_info_get_file_type(info);
        g_object_unref(info as gpointer);
    } else {
        file_type = G_FILE_TYPE_UNKNOWN;
    }
    return file_type;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_info(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).query_info.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    return Some((*iface).query_info.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file, attributes, flags, cancellable, error
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_info_async(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .query_info_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        attributes,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_info_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .query_info_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_filesystem_info(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).query_filesystem_info.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    return Some(
        (*iface)
            .query_filesystem_info
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, attributes, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_filesystem_info_async(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .query_filesystem_info_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        attributes,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_filesystem_info_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .query_filesystem_info_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_find_enclosing_mount(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GMount {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMount>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GMount>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).find_enclosing_mount.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(b"Containing mount does not exist\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GMount>();
    }
    return Some(
        (*iface)
            .find_enclosing_mount
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_find_enclosing_mount_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .find_enclosing_mount_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, io_priority, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_find_enclosing_mount_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GMount {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMount>();
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMount>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GMount>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .find_enclosing_mount_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_read(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).read_fn.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    return Some((*iface).read_fn.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_append_to(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).append_to.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    return Some((*iface).append_to.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, flags, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_create(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).create.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    return Some((*iface).create.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, flags, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).replace.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if !etag.is_null() && *etag as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        etag = ::core::ptr::null::<::core::ffi::c_char>();
    }
    return Some((*iface).replace.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file, etag, make_backup, flags, cancellable, error
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_open_readwrite(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).open_readwrite.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    return Some((*iface).open_readwrite.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_create_readwrite(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).create_readwrite.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    return Some(
        (*iface)
            .create_readwrite
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, flags, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_readwrite(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).replace_readwrite.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    return Some(
        (*iface)
            .replace_readwrite
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file, etag, make_backup, flags, cancellable, error
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_read_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some((*iface).read_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file, io_priority, cancellable, callback, user_data
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_read_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).read_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_append_to_async(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some((*iface).append_to_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_append_to_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .append_to_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_create_async(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some((*iface).create_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_create_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).create_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_async(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some((*iface).replace_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        etag,
        make_backup,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).replace_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_open_readwrite_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .open_readwrite_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, io_priority, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_open_readwrite_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .open_readwrite_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_create_readwrite_async(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .create_readwrite_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_create_readwrite_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .create_readwrite_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_readwrite_async(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .replace_readwrite_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        etag,
        make_backup,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_readwrite_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .replace_readwrite_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
unsafe extern "C" fn safe_c2rust_copy_symlink(
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut target: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut tried_delete: gboolean = 0;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut file_type: GFileType = G_FILE_TYPE_UNKNOWN;
    tried_delete = FALSE as gboolean;
    loop {
        my_error = ::core::ptr::null_mut::<GError>();
        if safe_c2rust_g_file_make_symbolic_link(
            destination,
            target,
            cancellable,
            &raw mut my_error,
        ) == 0
        {
            if tried_delete == 0
                && flags as ::core::ffi::c_uint
                    & G_FILE_COPY_OVERWRITE as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                && (*my_error).domain == g_io_error_quark()
                && (*my_error).code == G_IO_ERROR_EXISTS as ::core::ffi::c_int
            {
                g_clear_error(&raw mut my_error);
                info = safe_c2rust_g_file_query_info(
                    destination,
                    G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr(),
                    G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS,
                    cancellable,
                    &raw mut my_error,
                );
                if !info.is_null() {
                    file_type = g_file_info_get_file_type(info);
                    g_object_unref(info as gpointer);
                    if file_type as ::core::ffi::c_uint
                        == G_FILE_TYPE_DIRECTORY as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        g_set_error_literal(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_IS_DIRECTORY as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Can\xE2\x80\x99t copy over directory\0" as *const u8
                                    as *const gchar,
                            ),
                        );
                        return FALSE;
                    }
                }
                if safe_c2rust_g_file_delete(destination, cancellable, error) == 0 {
                    return FALSE;
                }
                tried_delete = TRUE as gboolean;
            } else {
                g_propagate_error(error, my_error);
                return FALSE;
            }
        } else {
            return TRUE;
        }
    }
}
unsafe extern "C" fn safe_c2rust_open_source_for_copy(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInputStream {
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GFileInputStream = ::core::ptr::null_mut::<GFileInputStream>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut file_type: GFileType = G_FILE_TYPE_UNKNOWN;
    my_error = ::core::ptr::null_mut::<GError>();
    ret = safe_c2rust_g_file_read(source, cancellable, &raw mut my_error);
    if !ret.is_null() {
        return ret;
    }
    if (*my_error).domain == g_io_error_quark()
        && (*my_error).code == G_IO_ERROR_IS_DIRECTORY as ::core::ffi::c_int
    {
        g_error_free(my_error);
        my_error = ::core::ptr::null_mut::<GError>();
        info = safe_c2rust_g_file_query_info(
            destination,
            G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr(),
            G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS,
            cancellable,
            &raw mut my_error,
        );
        if !info.is_null()
            && g_file_info_has_attribute(info, G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr()) != 0
        {
            file_type = g_file_info_get_file_type(info);
            g_object_unref(info as gpointer);
            if flags as ::core::ffi::c_uint
                & G_FILE_COPY_OVERWRITE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                if file_type as ::core::ffi::c_uint
                    == G_FILE_TYPE_DIRECTORY as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_WOULD_MERGE as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Can\xE2\x80\x99t copy directory over directory\0" as *const u8
                                as *const gchar,
                        ),
                    );
                    return ::core::ptr::null_mut::<GFileInputStream>();
                }
            } else {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_EXISTS as ::core::ffi::c_int as gint,
                    glib_gettext(b"Target file exists\0" as *const u8 as *const gchar),
                );
                return ::core::ptr::null_mut::<GFileInputStream>();
            }
        } else {
            let mut _pp: *mut *mut GFileInfo = &raw mut info;
            let mut _ptr: *mut GFileInfo = *_pp;
            *_pp = ::core::ptr::null_mut::<GFileInfo>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
            if !my_error.is_null()
                && g_error_matches(
                    my_error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                ) == 0
            {
                g_propagate_error(error, my_error);
                return ::core::ptr::null_mut::<GFileInputStream>();
            }
            g_clear_error(&raw mut my_error);
        }
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_WOULD_RECURSE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t recursively copy directory\0" as *const u8 as *const gchar,
            ),
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    g_propagate_error(error, my_error);
    return ::core::ptr::null_mut::<GFileInputStream>();
}
unsafe extern "C" fn safe_c2rust_should_copy(
    mut info: *mut GFileAttributeInfo,
    mut copy_all_attributes: gboolean,
    mut skip_perms: gboolean,
    mut skip_modified_time: gboolean,
) -> gboolean {
    if skip_perms != 0
        && strcmp(
            (*info).name,
            b"unix::mode\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || skip_modified_time != 0
            && strncmp(
                (*info).name,
                b"time::modified\0" as *const u8 as *const ::core::ffi::c_char,
                14 as size_t,
            ) == 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    if copy_all_attributes != 0 {
        return ((*info).flags as ::core::ffi::c_uint
            & G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED as ::core::ffi::c_int as ::core::ffi::c_uint)
            as gboolean;
    }
    return ((*info).flags as ::core::ffi::c_uint
        & G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_build_attribute_list_for_copy(
    mut file: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut attributes: *mut GFileAttributeInfoList =
        ::core::ptr::null_mut::<GFileAttributeInfoList>();
    let mut namespaces: *mut GFileAttributeInfoList =
        ::core::ptr::null_mut::<GFileAttributeInfoList>();
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut first: gboolean = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut copy_all_attributes: gboolean = 0;
    let mut skip_perms: gboolean = 0;
    let mut skip_modified_time: gboolean = 0;
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
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
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
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
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    copy_all_attributes = (flags as ::core::ffi::c_uint
        & G_FILE_COPY_ALL_METADATA as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    skip_perms = (flags as ::core::ffi::c_uint
        & G_FILE_COPY_TARGET_DEFAULT_PERMS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as gboolean;
    skip_modified_time = (flags as ::core::ffi::c_uint
        & G_FILE_COPY_TARGET_DEFAULT_MODIFIED_TIME as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as gboolean;
    attributes = safe_c2rust_g_file_query_settable_attributes(
        file,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !(g_cancellable_set_error_if_cancelled(cancellable, error) != 0) {
        namespaces = safe_c2rust_g_file_query_writable_namespaces(
            file,
            cancellable,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !(g_cancellable_set_error_if_cancelled(cancellable, error) != 0) {
            if !(attributes.is_null() && namespaces.is_null()) {
                first = TRUE as gboolean;
                s = g_string_new(b"\0" as *const u8 as *const gchar);
                first = FALSE as gboolean;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"standard::size\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            s,
                            __val,
                            if ({
                                let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_86 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_86 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_86
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
                        s,
                        b"standard::size\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                if !attributes.is_null() {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*attributes).n_infos {
                        if safe_c2rust_should_copy(
                            (*attributes).infos.offset(i as isize) as *mut GFileAttributeInfo,
                            copy_all_attributes,
                            skip_perms,
                            skip_modified_time,
                        ) != 0
                        {
                            if first != 0 {
                                first = FALSE as gboolean;
                            } else {
                                safe_c2rust_g_string_append_c_inline(s, ',' as i32 as gchar);
                            }
                            if 0 != 0 {
                                ({
                                    let __val: *const ::core::ffi::c_char =
                                        (*(*attributes).infos.offset(i as isize)).name;
                                    safe_c2rust_g_string_append_len_inline(
                                        s,
                                        __val,
                                        if ({
                                            let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
                                            if !__val.is_null() {
                                                _g_boolean_var_87 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_87 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_87
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            strlen(__val.offset(__val.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as gssize
                                        } else {
                                            -(1 as ::core::ffi::c_int) as gssize
                                        },
                                    );
                                });
                            } else {
                                safe_c2rust_g_string_append_len_inline(
                                    s,
                                    (*(*attributes).infos.offset(i as isize)).name,
                                    -(1 as ::core::ffi::c_int) as gssize,
                                );
                            };
                        }
                        i += 1;
                    }
                }
                if !namespaces.is_null() {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*namespaces).n_infos {
                        if safe_c2rust_should_copy(
                            (*namespaces).infos.offset(i as isize) as *mut GFileAttributeInfo,
                            copy_all_attributes,
                            FALSE,
                            FALSE,
                        ) != 0
                        {
                            if first != 0 {
                                first = FALSE as gboolean;
                            } else {
                                safe_c2rust_g_string_append_c_inline(s, ',' as i32 as gchar);
                            }
                            if 0 != 0 {
                                ({
                                    let __val: *const ::core::ffi::c_char =
                                        (*(*namespaces).infos.offset(i as isize)).name;
                                    safe_c2rust_g_string_append_len_inline(
                                        s,
                                        __val,
                                        if ({
                                            let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
                                            if !__val.is_null() {
                                                _g_boolean_var_88 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_88 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_88
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            strlen(__val.offset(__val.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as gssize
                                        } else {
                                            -(1 as ::core::ffi::c_int) as gssize
                                        },
                                    );
                                });
                            } else {
                                safe_c2rust_g_string_append_len_inline(
                                    s,
                                    (*(*namespaces).infos.offset(i as isize)).name,
                                    -(1 as ::core::ffi::c_int) as gssize,
                                );
                            };
                            if 0 != 0 {
                                ({
                                    let __val: *const ::core::ffi::c_char =
                                        b"::*\0" as *const u8 as *const ::core::ffi::c_char;
                                    safe_c2rust_g_string_append_len_inline(
                                        s,
                                        __val,
                                        if ({
                                            let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
                                            if !__val.is_null() {
                                                _g_boolean_var_89 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_89 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_89
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            strlen(__val.offset(__val.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as gssize
                                        } else {
                                            -(1 as ::core::ffi::c_int) as gssize
                                        },
                                    );
                                });
                            } else {
                                safe_c2rust_g_string_append_len_inline(
                                    s,
                                    b"::*\0" as *const u8 as *const ::core::ffi::c_char,
                                    -(1 as ::core::ffi::c_int) as gssize,
                                );
                            };
                        }
                        i += 1;
                    }
                }
                ret = (if 0 != 0 {
                    if 0 as ::core::ffi::c_int != 0 {
                        g_string_free(s, 0 as gboolean)
                    } else {
                        g_string_free_and_steal(s)
                    }
                } else {
                    g_string_free(s, 0 as gboolean)
                }) as *mut ::core::ffi::c_char;
                s = ::core::ptr::null_mut::<GString>();
            }
        }
    }
    if !s.is_null() {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(s);
            };
        } else {
            g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
    }
    if !attributes.is_null() {
        g_file_attribute_info_list_unref(attributes);
    }
    if !namespaces.is_null() {
        g_file_attribute_info_list_unref(namespaces);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_copy_attributes(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut attrs_to_read: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: gboolean = 0;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut source_nofollow_symlinks: gboolean = 0;
    attrs_to_read =
        safe_c2rust_g_file_build_attribute_list_for_copy(destination, flags, cancellable, error);
    if attrs_to_read.is_null() {
        return FALSE;
    }
    source_nofollow_symlinks = (flags as ::core::ffi::c_uint
        & G_FILE_COPY_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    info = safe_c2rust_g_file_query_info(
        source,
        attrs_to_read,
        (if source_nofollow_symlinks != 0 {
            G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as GFileQueryInfoFlags,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_free(attrs_to_read as gpointer);
    res = TRUE as gboolean;
    if !info.is_null() {
        res = safe_c2rust_g_file_set_attributes_from_info(
            destination,
            info,
            G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS,
            cancellable,
            error,
        );
        g_object_unref(info as gpointer);
    }
    return res;
}
pub const STREAM_BUFFER_SIZE: usize = ((1024 as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
    as usize)
    .wrapping_sub((2 as usize).wrapping_mul(::core::mem::size_of::<gpointer>() as usize));
unsafe extern "C" fn safe_c2rust_copy_stream_with_progress(
    mut in_0: *mut GInputStream,
    mut out: *mut GOutputStream,
    mut source: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut n_read: gssize = 0;
    let mut n_written: gsize = 0;
    let mut current_size: goffset = 0;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: gboolean = 0;
    let mut total_size: goffset = 0;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    total_size = -(1 as ::core::ffi::c_int) as goffset;
    if progress_callback.is_some() {
        info = g_file_input_stream_query_info(
            in_0 as *mut ::core::ffi::c_void as *mut GFileInputStream,
            G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr(),
            cancellable,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !info.is_null() {
            if g_file_info_has_attribute(info, G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr()) != 0 {
                total_size = g_file_info_get_size(info);
            }
            g_object_unref(info as gpointer);
        }
        if total_size == -(1 as ::core::ffi::c_int) as goffset {
            info = safe_c2rust_g_file_query_info(
                source,
                G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr(),
                G_FILE_QUERY_INFO_NONE,
                cancellable,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if !info.is_null() {
                if g_file_info_has_attribute(info, G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr()) != 0 {
                    total_size = g_file_info_get_size(info);
                }
                g_object_unref(info as gpointer);
            }
        }
    }
    if total_size == -(1 as ::core::ffi::c_int) as goffset {
        total_size = 0 as goffset;
    }
    buffer = g_malloc0(STREAM_BUFFER_SIZE as gsize) as *mut ::core::ffi::c_char;
    current_size = 0 as goffset;
    res = TRUE as gboolean;
    while FALSE == 0 {
        n_read = g_input_stream_read(
            in_0,
            buffer as *mut ::core::ffi::c_void,
            STREAM_BUFFER_SIZE as gsize,
            cancellable,
            error,
        );
        if n_read == -(1 as ::core::ffi::c_int) as gssize {
            res = FALSE as gboolean;
            break;
        } else {
            if n_read == 0 as gssize {
                break;
            }
            current_size += n_read as ::core::ffi::c_long;
            res = g_output_stream_write_all(
                out,
                buffer as *const ::core::ffi::c_void,
                n_read as gsize,
                &raw mut n_written,
                cancellable,
                error,
            );
            if res == 0 {
                break;
            }
            if progress_callback.is_some() {
                progress_callback.expect("non-null function pointer")(
                    current_size,
                    total_size,
                    progress_callback_data,
                );
            }
        }
    }
    g_free(buffer as gpointer);
    if progress_callback.is_some() {
        progress_callback.expect("non-null function pointer")(
            current_size,
            total_size,
            progress_callback_data,
        );
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_do_copy_file_range(
    mut fd_in: ::core::ffi::c_int,
    mut off_in: *mut loff_t,
    mut fd_out: ::core::ffi::c_int,
    mut off_out: *mut loff_t,
    mut len: size_t,
    mut bytes_transferred: *mut size_t,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut result: ssize_t = 0;
    loop {
        result = copy_file_range(
            fd_in,
            off_in as *mut __off64_t,
            fd_out,
            off_out as *mut __off64_t,
            len,
            0 as ::core::ffi::c_uint,
        );
        if result == -(1 as ::core::ffi::c_int) as ssize_t {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            if !(errsv == EINTR) {
                if errsv == ENOSYS || errsv == EINVAL || errsv == EOPNOTSUPP || errsv == EXDEV {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Copy file range not supported\0" as *const u8 as *const gchar,
                        ),
                    );
                } else {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        g_io_error_from_errno(errsv as gint) as gint,
                        glib_gettext(b"Error splicing file: %s\0" as *const u8 as *const gchar),
                        g_strerror(errsv as gint),
                    );
                }
                return FALSE;
            }
        }
        if !(result == -(1 as ::core::ffi::c_int) as ssize_t) {
            break;
        }
    }
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if result >= 0 as ssize_t {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            3064 as ::core::ffi::c_int,
            G_STRFUNC,
            b"result >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *bytes_transferred = result as size_t;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_copy_file_range_with_progress(
    mut in_0: *mut GInputStream,
    mut in_info: *mut GFileInfo,
    mut out: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut total_size: goffset = 0;
    let mut last_notified_size: goffset = 0;
    let mut copy_len: size_t = 0;
    let mut offset_in: loff_t = 0;
    let mut offset_out: loff_t = 0;
    let mut fd_in: ::core::ffi::c_int = 0;
    let mut fd_out: ::core::ffi::c_int = 0;
    fd_in = g_file_descriptor_based_get_fd(
        in_0 as *mut ::core::ffi::c_void as *mut GFileDescriptorBased,
    );
    fd_out = g_file_descriptor_based_get_fd(
        out as *mut ::core::ffi::c_void as *mut GFileDescriptorBased,
    );
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if g_file_info_has_attribute(
            in_info,
            b"standard::size\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            3088 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_file_info_has_attribute (in_info, G_FILE_ATTRIBUTE_STANDARD_SIZE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    total_size = g_file_info_get_size(in_info);
    if total_size == 0 as goffset {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Copy file range not supported\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    offset_out = 0 as loff_t;
    offset_in = offset_out;
    copy_len = total_size as size_t;
    last_notified_size = 0 as goffset;
    while copy_len > 0 as size_t {
        let mut n_copied: size_t = 0;
        if g_cancellable_set_error_if_cancelled(cancellable, error) != 0
            || safe_c2rust_do_copy_file_range(
                fd_in,
                &raw mut offset_in,
                fd_out,
                &raw mut offset_out,
                copy_len,
                &raw mut n_copied,
                error,
            ) == 0
        {
            return FALSE;
        }
        if n_copied == 0 as size_t {
            break;
        }
        if ({
            let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
            if n_copied <= copy_len {
                _g_boolean_var_92 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_92 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_92
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                3121 as ::core::ffi::c_int,
                G_STRFUNC,
                b"n_copied <= copy_len\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        copy_len = copy_len.wrapping_sub(n_copied);
        if progress_callback.is_some() {
            progress_callback.expect("non-null function pointer")(
                offset_in as goffset,
                total_size,
                progress_callback_data,
            );
            last_notified_size = total_size;
        }
    }
    if progress_callback.is_some() && last_notified_size != total_size {
        progress_callback.expect("non-null function pointer")(
            offset_in as goffset,
            total_size,
            progress_callback_data,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_do_splice(
    mut fd_in: ::core::ffi::c_int,
    mut off_in: *mut loff_t,
    mut fd_out: ::core::ffi::c_int,
    mut off_out: *mut loff_t,
    mut len: size_t,
    mut bytes_transferd: *mut ::core::ffi::c_long,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut result: ::core::ffi::c_long = 0;
    loop {
        result = splice(
            fd_in,
            off_in as *mut __off64_t,
            fd_out,
            off_out as *mut __off64_t,
            len,
            SPLICE_F_MORE as ::core::ffi::c_uint,
        ) as ::core::ffi::c_long;
        if result == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            if errsv == EINTR {
                continue;
            }
            if errsv == ENOSYS || errsv == EINVAL || errsv == EOPNOTSUPP {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                    glib_gettext(b"Splice not supported\0" as *const u8 as *const gchar),
                );
            } else {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    g_io_error_from_errno(errsv as gint) as gint,
                    glib_gettext(b"Error splicing file: %s\0" as *const u8 as *const gchar),
                    g_strerror(errsv as gint),
                );
            }
            return FALSE;
        } else {
            *bytes_transferd = result;
            return TRUE;
        }
    }
}
unsafe extern "C" fn safe_c2rust_splice_stream_with_progress(
    mut in_0: *mut GInputStream,
    mut in_info: *mut GFileInfo,
    mut out: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut buffer: [::core::ffi::c_int; 2] =
        [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)];
    let mut buffer_size: ::core::ffi::c_int = 0;
    let mut res: gboolean = 0;
    let mut total_size: goffset = 0;
    let mut offset_in: loff_t = 0;
    let mut offset_out: loff_t = 0;
    let mut fd_in: ::core::ffi::c_int = 0;
    let mut fd_out: ::core::ffi::c_int = 0;
    fd_in = g_file_descriptor_based_get_fd(
        in_0 as *mut ::core::ffi::c_void as *mut GFileDescriptorBased,
    );
    fd_out = g_file_descriptor_based_get_fd(
        out as *mut ::core::ffi::c_void as *mut GFileDescriptorBased,
    );
    if g_unix_open_pipe(&raw mut buffer as *mut gint, O_CLOEXEC, error) == 0 {
        return FALSE;
    }
    buffer_size = fcntl(
        buffer[1 as ::core::ffi::c_int as usize],
        F_SETPIPE_SZ,
        1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int,
    );
    if buffer_size <= 0 as ::core::ffi::c_int {
        buffer_size = fcntl(buffer[1 as ::core::ffi::c_int as usize], F_GETPIPE_SZ);
        if buffer_size <= 0 as ::core::ffi::c_int {
            buffer_size = 1024 as ::core::ffi::c_int * 64 as ::core::ffi::c_int;
        }
    }
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if buffer_size > 0 as ::core::ffi::c_int {
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            3217 as ::core::ffi::c_int,
            G_STRFUNC,
            b"buffer_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    total_size = -(1 as ::core::ffi::c_int) as goffset;
    if progress_callback.is_some() {
        if ({
            let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
            if g_file_info_has_attribute(
                in_info,
                b"standard::size\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
            {
                _g_boolean_var_94 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_94 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_94
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                3223 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_file_info_has_attribute (in_info, G_FILE_ATTRIBUTE_STANDARD_SIZE)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        total_size = g_file_info_get_size(in_info);
    }
    if total_size == -(1 as ::core::ffi::c_int) as goffset {
        total_size = 0 as goffset;
    }
    offset_out = 0 as loff_t;
    offset_in = offset_out;
    res = FALSE as gboolean;
    's_126: loop {
        if !(FALSE == 0) {
            current_block = 8180496224585318153;
            break;
        }
        let mut n_read: ::core::ffi::c_long = 0;
        let mut n_written: ::core::ffi::c_long = 0;
        if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
            current_block = 8180496224585318153;
            break;
        }
        if safe_c2rust_do_splice(
            fd_in,
            &raw mut offset_in,
            buffer[1 as ::core::ffi::c_int as usize],
            ::core::ptr::null_mut::<loff_t>(),
            buffer_size as size_t,
            &raw mut n_read,
            error,
        ) == 0
        {
            current_block = 8180496224585318153;
            break;
        }
        if n_read == 0 as ::core::ffi::c_long {
            res = TRUE as gboolean;
            current_block = 8180496224585318153;
            break;
        } else {
            while n_read > 0 as ::core::ffi::c_long {
                if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
                    current_block = 2967229571043969209;
                    break 's_126;
                }
                if safe_c2rust_do_splice(
                    buffer[0 as ::core::ffi::c_int as usize],
                    ::core::ptr::null_mut::<loff_t>(),
                    fd_out,
                    &raw mut offset_out,
                    n_read as size_t,
                    &raw mut n_written,
                    error,
                ) == 0
                {
                    current_block = 2967229571043969209;
                    break 's_126;
                }
                n_read -= n_written;
            }
            if progress_callback.is_some() {
                progress_callback.expect("non-null function pointer")(
                    offset_in as goffset,
                    total_size,
                    progress_callback_data,
                );
            }
        }
    }
    match current_block {
        8180496224585318153 => {
            if progress_callback.is_some() {
                progress_callback.expect("non-null function pointer")(
                    offset_in as goffset,
                    total_size,
                    progress_callback_data,
                );
            }
            if !(g_close(buffer[0 as ::core::ffi::c_int as usize], error) == 0) {
                buffer[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
                if !(g_close(buffer[1 as ::core::ffi::c_int as usize], error) == 0) {
                    buffer[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
                }
            }
        }
        _ => {}
    }
    if buffer[0 as ::core::ffi::c_int as usize] != -(1 as ::core::ffi::c_int) {
        g_close(
            buffer[0 as ::core::ffi::c_int as usize],
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if buffer[1 as ::core::ffi::c_int as usize] != -(1 as ::core::ffi::c_int) {
        g_close(
            buffer[1 as ::core::ffi::c_int as usize],
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_btrfs_reflink_with_progress(
    mut in_0: *mut GInputStream,
    mut in_info: *mut GFileInfo,
    mut out: *mut GOutputStream,
    mut info: *mut GFileInfo,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut total_size: goffset = 0;
    let mut fd_in: ::core::ffi::c_int = 0;
    let mut fd_out: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    fd_in = g_file_descriptor_based_get_fd(
        in_0 as *mut ::core::ffi::c_void as *mut GFileDescriptorBased,
    );
    fd_out = g_file_descriptor_based_get_fd(
        out as *mut ::core::ffi::c_void as *mut GFileDescriptorBased,
    );
    total_size = -(1 as ::core::ffi::c_int) as goffset;
    if progress_callback.is_some() {
        if ({
            let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
            if g_file_info_has_attribute(
                in_info,
                b"standard::size\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
            {
                _g_boolean_var_95 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_95 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_95
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                3306 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_file_info_has_attribute (in_info, G_FILE_ATTRIBUTE_STANDARD_SIZE)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        total_size = g_file_info_get_size(in_info);
    }
    if total_size == -(1 as ::core::ffi::c_int) as goffset {
        total_size = 0 as goffset;
    }
    ret = ioctl(fd_out, BTRFS_IOC_CLONE as ::core::ffi::c_ulong, fd_in);
    errsv = *__errno_location();
    if ret < 0 as ::core::ffi::c_int {
        if errsv == EXDEV {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Copy (reflink/clone) between mounts is not supported\0" as *const u8
                        as *const gchar,
                ),
            );
        } else if errsv == EINVAL {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Copy (reflink/clone) is not supported or invalid\0" as *const u8
                        as *const gchar,
                ),
            );
        } else {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Copy (reflink/clone) is not supported or didn\xE2\x80\x99t work\0"
                        as *const u8 as *const gchar,
                ),
            );
        }
        return FALSE;
    }
    if progress_callback.is_some() {
        progress_callback.expect("non-null function pointer")(
            total_size,
            total_size,
            progress_callback_data,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_file_copy_fallback(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = FALSE;
    let mut file_in: *mut GFileInputStream = ::core::ptr::null_mut::<GFileInputStream>();
    let mut in_0: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut out: *mut GOutputStream = ::core::ptr::null_mut::<GOutputStream>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut target: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut attrs_to_read: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut do_set_attributes: gboolean = FALSE;
    let mut create_flags: GFileCreateFlags = G_FILE_CREATE_NONE;
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    info = safe_c2rust_g_file_query_info(
        source,
        b"standard::type,standard::symlink-target\0" as *const u8 as *const ::core::ffi::c_char,
        G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS,
        cancellable,
        error,
    );
    if !info.is_null() {
        if g_file_info_has_attribute(info, G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr()) == 0 {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(b"Cannot retrieve attribute %s\0" as *const u8 as *const gchar),
                G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr(),
            );
        } else {
            if flags as ::core::ffi::c_uint
                & G_FILE_COPY_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && g_file_info_get_file_type(info) as ::core::ffi::c_uint
                    == G_FILE_TYPE_SYMBOLIC_LINK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if g_file_info_has_attribute(
                    info,
                    G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET.as_ptr(),
                ) == 0
                {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Cannot retrieve attribute %s\0" as *const u8 as *const gchar,
                        ),
                        G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET.as_ptr(),
                    );
                    current_block = 9168604710158400597;
                } else {
                    target = g_file_info_get_symlink_target(info);
                    if !target.is_null() {
                        if safe_c2rust_copy_symlink(destination, flags, cancellable, target, error)
                            == 0
                        {
                            current_block = 9168604710158400597;
                        } else {
                            ret = TRUE as gboolean;
                            current_block = 9168604710158400597;
                        }
                    } else {
                        current_block = 7149356873433890176;
                    }
                }
            } else if g_file_info_get_file_type(info) as ::core::ffi::c_uint
                == G_FILE_TYPE_SPECIAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Can\xE2\x80\x99t copy special file\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 9168604710158400597;
            } else {
                current_block = 7149356873433890176;
            }
            match current_block {
                9168604710158400597 => {}
                _ => {
                    file_in = safe_c2rust_open_source_for_copy(
                        source,
                        destination,
                        flags,
                        cancellable,
                        error,
                    );
                    if !file_in.is_null() {
                        in_0 = file_in as *mut ::core::ffi::c_void as *mut GInputStream;
                        attrs_to_read = safe_c2rust_g_file_build_attribute_list_for_copy(
                            destination,
                            flags,
                            cancellable,
                            error,
                        );
                        if !attrs_to_read.is_null() {
                            g_object_unref(info as gpointer);
                            info = g_file_input_stream_query_info(
                                file_in,
                                attrs_to_read,
                                cancellable,
                                &raw mut tmp_error,
                            );
                            if info.is_null() {
                                if g_error_matches(
                                    tmp_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                                ) != 0
                                {
                                    g_clear_error(&raw mut tmp_error);
                                    info = safe_c2rust_g_file_query_info(
                                        source,
                                        attrs_to_read,
                                        G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS,
                                        cancellable,
                                        error,
                                    );
                                    current_block = 13550086250199790493;
                                } else {
                                    g_free(attrs_to_read as gpointer);
                                    g_propagate_error(error, tmp_error);
                                    current_block = 9168604710158400597;
                                }
                            } else {
                                current_block = 13550086250199790493;
                            }
                            match current_block {
                                9168604710158400597 => {}
                                _ => {
                                    g_free(attrs_to_read as gpointer);
                                    if !info.is_null() {
                                        do_set_attributes = TRUE as gboolean;
                                        create_flags = G_FILE_CREATE_NONE;
                                        if flags as ::core::ffi::c_uint
                                            & G_FILE_COPY_TARGET_DEFAULT_PERMS as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            == 0
                                            && g_file_info_has_attribute(
                                                info,
                                                G_FILE_ATTRIBUTE_UNIX_MODE.as_ptr(),
                                            ) != 0
                                            && ({
                                                let mut __inst: *mut GTypeInstance =
                                                    destination as *mut GTypeInstance;
                                                let mut __t: GType = _g_local_file_get_type();
                                                let mut __r: gboolean = 0;
                                                if __inst.is_null() {
                                                    __r = FALSE as gboolean;
                                                } else if !(*__inst).g_class.is_null()
                                                    && (*(*__inst).g_class).g_type == __t
                                                {
                                                    __r = TRUE as gboolean;
                                                } else {
                                                    __r = g_type_check_instance_is_a(__inst, __t);
                                                }
                                                __r
                                            }) == 0
                                        {
                                            create_flags = ::core::mem::transmute::<
                                                ::core::ffi::c_uint,
                                                GFileCreateFlags,
                                            >(
                                                create_flags as ::core::ffi::c_uint
                                                    | G_FILE_CREATE_PRIVATE as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                            );
                                        }
                                        if flags as ::core::ffi::c_uint
                                            & G_FILE_COPY_OVERWRITE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            != 0
                                        {
                                            create_flags = ::core::mem::transmute::<
                                                ::core::ffi::c_uint,
                                                GFileCreateFlags,
                                            >(
                                                create_flags as ::core::ffi::c_uint
                                                    | G_FILE_CREATE_REPLACE_DESTINATION
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                            );
                                        }
                                        if ({
                                            let mut __inst: *mut GTypeInstance =
                                                destination as *mut GTypeInstance;
                                            let mut __t: GType = _g_local_file_get_type();
                                            let mut __r: gboolean = 0;
                                            if __inst.is_null() {
                                                __r = FALSE as gboolean;
                                            } else if !(*__inst).g_class.is_null()
                                                && (*(*__inst).g_class).g_type == __t
                                            {
                                                __r = TRUE as gboolean;
                                            } else {
                                                __r = g_type_check_instance_is_a(__inst, __t);
                                            }
                                            __r
                                        }) != 0
                                        {
                                            if flags as ::core::ffi::c_uint
                                                & G_FILE_COPY_OVERWRITE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                                != 0
                                            {
                                                out = _g_local_file_output_stream_replace(
                                                    _g_local_file_get_filename(
                                                        destination as *mut ::core::ffi::c_void
                                                            as *mut GLocalFile,
                                                    ),
                                                    FALSE,
                                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                                    (flags as ::core::ffi::c_uint
                                                        & G_FILE_COPY_BACKUP as ::core::ffi::c_int
                                                            as ::core::ffi::c_uint)
                                                        as gboolean,
                                                    create_flags,
                                                    if flags as ::core::ffi::c_uint
                                                        & G_FILE_COPY_TARGET_DEFAULT_PERMS
                                                            as ::core::ffi::c_int
                                                            as ::core::ffi::c_uint
                                                        != 0
                                                    {
                                                        ::core::ptr::null_mut::<GFileInfo>()
                                                    } else {
                                                        info
                                                    },
                                                    cancellable,
                                                    error,
                                                )
                                                    as *mut GOutputStream;
                                            } else {
                                                out = _g_local_file_output_stream_create(
                                                    _g_local_file_get_filename(
                                                        destination as *mut ::core::ffi::c_void
                                                            as *mut GLocalFile,
                                                    ),
                                                    FALSE,
                                                    create_flags,
                                                    if flags as ::core::ffi::c_uint
                                                        & G_FILE_COPY_TARGET_DEFAULT_PERMS
                                                            as ::core::ffi::c_int
                                                            as ::core::ffi::c_uint
                                                        != 0
                                                    {
                                                        ::core::ptr::null_mut::<GFileInfo>()
                                                    } else {
                                                        info
                                                    },
                                                    cancellable,
                                                    error,
                                                )
                                                    as *mut GOutputStream;
                                            }
                                        } else if flags as ::core::ffi::c_uint
                                            & G_FILE_COPY_OVERWRITE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            != 0
                                        {
                                            out = safe_c2rust_g_file_replace(
                                                destination,
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                (flags as ::core::ffi::c_uint
                                                    & G_FILE_COPY_BACKUP as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint)
                                                    as gboolean,
                                                create_flags,
                                                cancellable,
                                                error,
                                            )
                                                as *mut GOutputStream;
                                        } else {
                                            out = safe_c2rust_g_file_create(
                                                destination,
                                                create_flags,
                                                cancellable,
                                                error,
                                            )
                                                as *mut GOutputStream;
                                        }
                                        if !out.is_null() {
                                            if ({
                                                let mut __inst: *mut GTypeInstance =
                                                    in_0 as *mut GTypeInstance;
                                                let mut __t: GType =
                                                    g_file_descriptor_based_get_type();
                                                let mut __r: gboolean = 0;
                                                if __inst.is_null() {
                                                    __r = FALSE as gboolean;
                                                } else if !(*__inst).g_class.is_null()
                                                    && (*(*__inst).g_class).g_type == __t
                                                {
                                                    __r = TRUE as gboolean;
                                                } else {
                                                    __r = g_type_check_instance_is_a(__inst, __t);
                                                }
                                                __r
                                            }) != 0
                                                && ({
                                                    let mut __inst: *mut GTypeInstance =
                                                        out as *mut GTypeInstance;
                                                    let mut __t: GType =
                                                        g_file_descriptor_based_get_type();
                                                    let mut __r: gboolean = 0;
                                                    if __inst.is_null() {
                                                        __r = FALSE as gboolean;
                                                    } else if !(*__inst).g_class.is_null()
                                                        && (*(*__inst).g_class).g_type == __t
                                                    {
                                                        __r = TRUE as gboolean;
                                                    } else {
                                                        __r =
                                                            g_type_check_instance_is_a(__inst, __t);
                                                    }
                                                    __r
                                                }) != 0
                                            {
                                                let mut reflink_err: *mut GError =
                                                    ::core::ptr::null_mut::<GError>();
                                                if safe_c2rust_btrfs_reflink_with_progress(
                                                    in_0,
                                                    info,
                                                    out,
                                                    info,
                                                    cancellable,
                                                    progress_callback,
                                                    progress_callback_data,
                                                    &raw mut reflink_err,
                                                ) == 0
                                                {
                                                    if g_error_matches(
                                                        reflink_err,
                                                        g_io_error_quark(),
                                                        G_IO_ERROR_NOT_SUPPORTED
                                                            as ::core::ffi::c_int
                                                            as gint,
                                                    ) != 0
                                                    {
                                                        g_clear_error(&raw mut reflink_err);
                                                        current_block = 7990025728955927862;
                                                    } else {
                                                        g_propagate_error(error, reflink_err);
                                                        current_block = 9168604710158400597;
                                                    }
                                                } else {
                                                    ret = TRUE as gboolean;
                                                    current_block = 9168604710158400597;
                                                }
                                            } else {
                                                current_block = 7990025728955927862;
                                            }
                                            match current_block {
                                                9168604710158400597 => {}
                                                _ => {
                                                    if ({
                                                        let mut __inst: *mut GTypeInstance =
                                                            in_0 as *mut GTypeInstance;
                                                        let mut __t: GType =
                                                            g_file_descriptor_based_get_type();
                                                        let mut __r: gboolean = 0;
                                                        if __inst.is_null() {
                                                            __r = FALSE as gboolean;
                                                        } else if !(*__inst).g_class.is_null()
                                                            && (*(*__inst).g_class).g_type == __t
                                                        {
                                                            __r = TRUE as gboolean;
                                                        } else {
                                                            __r = g_type_check_instance_is_a(
                                                                __inst, __t,
                                                            );
                                                        }
                                                        __r
                                                    }) != 0
                                                        && ({
                                                            let mut __inst: *mut GTypeInstance =
                                                                out as *mut GTypeInstance;
                                                            let mut __t: GType =
                                                                g_file_descriptor_based_get_type();
                                                            let mut __r: gboolean = 0;
                                                            if __inst.is_null() {
                                                                __r = FALSE as gboolean;
                                                            } else if !(*__inst).g_class.is_null()
                                                                && (*(*__inst).g_class).g_type
                                                                    == __t
                                                            {
                                                                __r = TRUE as gboolean;
                                                            } else {
                                                                __r = g_type_check_instance_is_a(
                                                                    __inst, __t,
                                                                );
                                                            }
                                                            __r
                                                        }) != 0
                                                    {
                                                        let mut copy_file_range_error: *mut GError =
                                                            ::core::ptr::null_mut::<GError>();
                                                        if safe_c2rust_copy_file_range_with_progress(
                                                            in_0,
                                                            info,
                                                            out,
                                                            cancellable,
                                                            progress_callback,
                                                            progress_callback_data,
                                                            &raw mut copy_file_range_error,
                                                        ) != 0
                                                        {
                                                            ret = TRUE as gboolean;
                                                            current_block = 9168604710158400597;
                                                        } else if g_error_matches(
                                                            copy_file_range_error,
                                                            g_io_error_quark(),
                                                            G_IO_ERROR_NOT_SUPPORTED
                                                                as ::core::ffi::c_int
                                                                as gint,
                                                        ) == 0
                                                        {
                                                            g_propagate_error(
                                                                error,
                                                                safe_c2rust_g_steal_pointer(
                                                                    &raw mut copy_file_range_error
                                                                        as gpointer,
                                                                )
                                                                    as *mut GError,
                                                            );
                                                            current_block = 9168604710158400597;
                                                        } else {
                                                            g_clear_error(
                                                                &raw mut copy_file_range_error,
                                                            );
                                                            current_block = 168769493162332264;
                                                        }
                                                    } else {
                                                        current_block = 168769493162332264;
                                                    }
                                                    match current_block {
                                                        9168604710158400597 => {}
                                                        _ => {
                                                            if ({
                                                                let mut __inst: *mut GTypeInstance =
                                                                    in_0 as *mut GTypeInstance;
                                                                let mut __t: GType = g_file_descriptor_based_get_type();
                                                                let mut __r: gboolean = 0;
                                                                if __inst.is_null() {
                                                                    __r = FALSE as gboolean;
                                                                } else if !(*__inst)
                                                                    .g_class
                                                                    .is_null()
                                                                    && (*(*__inst).g_class).g_type
                                                                        == __t
                                                                {
                                                                    __r = TRUE as gboolean;
                                                                } else {
                                                                    __r =
                                                                        g_type_check_instance_is_a(
                                                                            __inst, __t,
                                                                        );
                                                                }
                                                                __r
                                                            }) != 0
                                                                && ({
                                                                    let mut __inst: *mut GTypeInstance = out
                                                                        as *mut GTypeInstance;
                                                                    let mut __t: GType = g_file_descriptor_based_get_type();
                                                                    let mut __r: gboolean = 0;
                                                                    if __inst.is_null() {
                                                                        __r = FALSE as gboolean;
                                                                    } else if !(*__inst)
                                                                        .g_class
                                                                        .is_null()
                                                                        && (*(*__inst).g_class)
                                                                            .g_type
                                                                            == __t
                                                                    {
                                                                        __r = TRUE as gboolean;
                                                                    } else {
                                                                        __r = g_type_check_instance_is_a(__inst, __t);
                                                                    }
                                                                    __r
                                                                }) != 0
                                                            {
                                                                let mut splice_err: *mut GError =
                                                                    ::core::ptr::null_mut::<GError>(
                                                                    );
                                                                if safe_c2rust_splice_stream_with_progress(
                                                                    in_0,
                                                                    info,
                                                                    out,
                                                                    cancellable,
                                                                    progress_callback,
                                                                    progress_callback_data,
                                                                    &raw mut splice_err,
                                                                ) == 0
                                                                {
                                                                    if g_error_matches(
                                                                        splice_err,
                                                                        g_io_error_quark(),
                                                                        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                                                                    ) != 0
                                                                    {
                                                                        g_clear_error(&raw mut splice_err);
                                                                        current_block = 15669289850109000831;
                                                                    } else {
                                                                        g_propagate_error(error, splice_err);
                                                                        current_block = 9168604710158400597;
                                                                    }
                                                                } else {
                                                                    ret = TRUE as gboolean;
                                                                    current_block = 9168604710158400597;
                                                                }
                                                            } else {
                                                                current_block =
                                                                    15669289850109000831;
                                                            }
                                                            match current_block {
                                                                9168604710158400597 => {}
                                                                _ => {
                                                                    if !(safe_c2rust_copy_stream_with_progress(
                                                                        in_0,
                                                                        out,
                                                                        source,
                                                                        cancellable,
                                                                        progress_callback,
                                                                        progress_callback_data,
                                                                        error,
                                                                    ) == 0)
                                                                    {
                                                                        ret = TRUE as gboolean;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !in_0.is_null() {
        g_input_stream_close(in_0, cancellable, ::core::ptr::null_mut::<*mut GError>());
        g_object_unref(in_0 as gpointer);
    }
    if !out.is_null() {
        if g_output_stream_close(
            out,
            cancellable,
            if ret != 0 {
                error
            } else {
                ::core::ptr::null_mut::<*mut GError>()
            },
        ) == 0
        {
            ret = FALSE as gboolean;
        }
        g_object_unref(out as gpointer);
    }
    if ret != 0 && do_set_attributes != 0 {
        safe_c2rust_g_file_set_attributes_from_info(
            destination,
            info,
            G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS,
            cancellable,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    let mut _pp: *mut *mut GFileInfo = &raw mut info;
    let mut _ptr: *mut GFileInfo = *_pp;
    *_pp = ::core::ptr::null_mut::<GFileInfo>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_copy(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = destination as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (destination)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(destination as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).copy.is_some() {
        my_error = ::core::ptr::null_mut::<GError>();
        res = Some((*iface).copy.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            source,
            destination,
            flags,
            cancellable,
            progress_callback,
            progress_callback_data,
            &raw mut my_error,
        );
        if res != 0 {
            return TRUE;
        }
        if (*my_error).domain != g_io_error_quark()
            || (*my_error).code != G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int
        {
            g_propagate_error(error, my_error);
            return FALSE;
        } else {
            g_clear_error(&raw mut my_error);
        }
    }
    if (*(*(source as *mut GTypeInstance)).g_class).g_type
        != (*(*(destination as *mut GTypeInstance)).g_class).g_type
    {
        iface = g_type_interface_peek(
            (*(source as *mut GTypeInstance)).g_class as gpointer,
            safe_c2rust_g_file_get_type(),
        ) as *mut GFileIface;
        if (*iface).copy.is_some() {
            my_error = ::core::ptr::null_mut::<GError>();
            res = Some((*iface).copy.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                source,
                destination,
                flags,
                cancellable,
                progress_callback,
                progress_callback_data,
                &raw mut my_error,
            );
            if res != 0 {
                return TRUE;
            }
            if (*my_error).domain != g_io_error_quark()
                || (*my_error).code != G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int
            {
                g_propagate_error(error, my_error);
                return FALSE;
            } else {
                g_clear_error(&raw mut my_error);
            }
        }
    }
    return safe_c2rust_file_copy_fallback(
        source,
        destination,
        flags,
        cancellable,
        progress_callback,
        progress_callback_data,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_copy_async(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = destination as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (destination)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(source as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some((*iface).copy_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        source,
        destination,
        flags,
        io_priority,
        cancellable,
        progress_callback,
        progress_callback_data,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_copy_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).copy_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_move(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = destination as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_103 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_103 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_103
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (destination)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(destination as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).move_0.is_some() {
        my_error = ::core::ptr::null_mut::<GError>();
        res = Some((*iface).move_0.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            source,
            destination,
            flags,
            cancellable,
            progress_callback,
            progress_callback_data,
            &raw mut my_error,
        );
        if res != 0 {
            return TRUE;
        }
        if (*my_error).domain != g_io_error_quark()
            || (*my_error).code != G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int
        {
            g_propagate_error(error, my_error);
            return FALSE;
        } else {
            g_clear_error(&raw mut my_error);
        }
    }
    if (*(*(source as *mut GTypeInstance)).g_class).g_type
        != (*(*(destination as *mut GTypeInstance)).g_class).g_type
    {
        iface = g_type_interface_peek(
            (*(source as *mut GTypeInstance)).g_class as gpointer,
            safe_c2rust_g_file_get_type(),
        ) as *mut GFileIface;
        if (*iface).move_0.is_some() {
            my_error = ::core::ptr::null_mut::<GError>();
            res = Some((*iface).move_0.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                source,
                destination,
                flags,
                cancellable,
                progress_callback,
                progress_callback_data,
                &raw mut my_error,
            );
            if res != 0 {
                return TRUE;
            }
            if (*my_error).domain != g_io_error_quark()
                || (*my_error).code != G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int
            {
                g_propagate_error(error, my_error);
                return FALSE;
            } else {
                g_clear_error(&raw mut my_error);
            }
        }
    }
    if flags as ::core::ffi::c_uint
        & G_FILE_COPY_NO_FALLBACK_FOR_MOVE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    flags = ::core::mem::transmute::<::core::ffi::c_uint, GFileCopyFlags>(
        flags as ::core::ffi::c_uint
            | (G_FILE_COPY_ALL_METADATA as ::core::ffi::c_int
                | G_FILE_COPY_NOFOLLOW_SYMLINKS as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
    );
    if safe_c2rust_g_file_copy(
        source,
        destination,
        flags,
        cancellable,
        progress_callback,
        progress_callback_data,
        error,
    ) == 0
    {
        return FALSE;
    }
    return safe_c2rust_g_file_delete(source, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_move_async(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_104 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_104 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_104
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = destination as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_105 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_105 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_105
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (destination)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
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
            _g_boolean_var_106 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_106 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_106
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
    iface = g_type_interface_peek(
        (*(source as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some((*iface).move_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        source,
        destination,
        flags,
        io_priority,
        cancellable,
        progress_callback,
        progress_callback_data,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_move_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_107 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_107 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_107
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
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
            _g_boolean_var_108 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_108 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_108
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
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_109 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_109 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_109
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
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).move_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_make_directory(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_110 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_110 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_110
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).make_directory.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return Some((*iface).make_directory.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_make_directory_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_111 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_111 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_111
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .make_directory_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, io_priority, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_make_directory_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_112 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_112 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_112
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
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
            _g_boolean_var_113 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_113 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_113
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
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .make_directory_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_make_directory_with_parents(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut work_file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_114 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_114 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_114
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    safe_c2rust_g_file_make_directory(file, cancellable, &raw mut my_error);
    if g_error_matches(
        my_error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
    ) == 0
    {
        if !my_error.is_null() {
            g_propagate_error(error, my_error);
        }
        return (my_error == NULL_0 as *mut GError) as ::core::ffi::c_int;
    }
    work_file = g_object_ref(file as gpointer) as *mut GFile as *mut GFile;
    while g_error_matches(
        my_error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
    ) != 0
    {
        let mut parent_file: *mut GFile = ::core::ptr::null_mut::<GFile>();
        parent_file = safe_c2rust_g_file_get_parent(work_file);
        if parent_file.is_null() {
            break;
        }
        g_clear_error(&raw mut my_error);
        safe_c2rust_g_file_make_directory(parent_file, cancellable, &raw mut my_error);
        if g_error_matches(
            my_error,
            g_io_error_quark(),
            G_IO_ERROR_EXISTS as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_clear_error(&raw mut my_error);
        }
        g_object_unref(work_file as gpointer);
        work_file = g_object_ref(parent_file as gpointer) as *mut GFile as *mut GFile;
        if g_error_matches(
            my_error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
        ) != 0
        {
            list = g_list_prepend(list, parent_file as gpointer);
        } else {
            g_object_unref(parent_file as gpointer);
        }
    }
    l = list;
    while my_error.is_null() && !l.is_null() {
        safe_c2rust_g_file_make_directory((*l).data as *mut GFile, cancellable, &raw mut my_error);
        if g_error_matches(
            my_error,
            g_io_error_quark(),
            G_IO_ERROR_EXISTS as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_clear_error(&raw mut my_error);
        }
        l = (*l).next;
    }
    if !work_file.is_null() {
        g_object_unref(work_file as gpointer);
    }
    while !list.is_null() {
        g_object_unref((*list).data as *mut GFile as gpointer);
        list = g_list_remove(list, (*list).data as gconstpointer);
    }
    if !my_error.is_null() {
        g_propagate_error(error, my_error);
        return FALSE;
    }
    return safe_c2rust_g_file_make_directory(file, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_make_symbolic_link(
    mut file: *mut GFile,
    mut symlink_value: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_115 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_115 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_115
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if !symlink_value.is_null() {
            _g_boolean_var_116 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_116 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_116
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"symlink_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    if *symlink_value as ::core::ffi::c_int == '\0' as i32 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid symlink value given\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).make_symbolic_link.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Symbolic links not supported\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return Some(
        (*iface)
            .make_symbolic_link
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, symlink_value, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_make_symbolic_link_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut symlink_value: *const ::core::ffi::c_char = task_data as *const ::core::ffi::c_char;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_file_make_symbolic_link(
        object as *mut GFile,
        symlink_value,
        cancellable,
        &raw mut error,
    ) != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_make_symbolic_link_async(
    mut file: *mut GFile,
    mut symlink_value: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_117: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_117 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_117 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_117
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_118: ::core::ffi::c_int = 0;
        if !symlink_value.is_null() {
            _g_boolean_var_118 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_118 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_118
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"symlink_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_119: ::core::ffi::c_int = 0;
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
            _g_boolean_var_119 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_119 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_119
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
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_make_symbolic_link_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
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
            b"g_file_real_make_symbolic_link_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_strdup_inline(symlink_value) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_make_symbolic_link_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_make_symbolic_link_async(
    mut file: *mut GFile,
    mut symlink_value: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_120: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_120 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_120 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_120
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_121: ::core::ffi::c_int = 0;
        if !symlink_value.is_null() {
            _g_boolean_var_121 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_121 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_121
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"symlink_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_122: ::core::ffi::c_int = 0;
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
            _g_boolean_var_122 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_122 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_122
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
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if ({
        let mut _g_boolean_var_123: ::core::ffi::c_int = 0;
        if (*iface).make_symbolic_link_async.is_some() {
            _g_boolean_var_123 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_123 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_123
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4421 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->make_symbolic_link_async != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    Some(
        (*iface)
            .make_symbolic_link_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        symlink_value,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_g_file_real_make_symbolic_link_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_124: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, file as gpointer) != 0 {
            _g_boolean_var_124 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_124 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_124
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_make_symbolic_link_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_125: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_125 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_125 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_125
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_126: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_126 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_126 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_126
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
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if ({
        let mut _g_boolean_var_127: ::core::ffi::c_int = 0;
        if (*iface).make_symbolic_link_finish.is_some() {
            _g_boolean_var_127 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_127 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_127
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4461 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface->make_symbolic_link_finish != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return Some(
        (*iface)
            .make_symbolic_link_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_delete(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_128: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_128 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_128 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_128
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).delete_file.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return Some((*iface).delete_file.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_delete_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_129: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_129 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_129 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_129
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .delete_file_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, io_priority, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_delete_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_130: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_130 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_130 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_130
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_131: ::core::ffi::c_int = 0;
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
            _g_boolean_var_131 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_131 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_131
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
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .delete_file_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_trash(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_132: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_132 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_132 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_132
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).trash.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Trash not supported\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return Some((*iface).trash.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_trash_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_133: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_133 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_133 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_133
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some((*iface).trash_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file, io_priority, cancellable, callback, user_data
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_trash_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_134: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_134 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_134 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_134
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_135: ::core::ffi::c_int = 0;
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
            _g_boolean_var_135 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_135 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_135
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
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some((*iface).trash_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_display_name(
    mut file: *mut GFile,
    mut display_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_136: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_136 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_136 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_136
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_137: ::core::ffi::c_int = 0;
        if !display_name.is_null() {
            _g_boolean_var_137 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_137 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_137
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"display_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if !strchr(display_name as *const ::core::ffi::c_char, G_DIR_SEPARATOR).is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"File names cannot contain \xE2\x80\x9C%c\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            G_DIR_SEPARATOR,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFile>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .set_display_name
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        display_name as *const ::core::ffi::c_char,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_display_name_async(
    mut file: *mut GFile,
    mut display_name: *const gchar,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_138: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_138 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_138 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_138
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_139: ::core::ffi::c_int = 0;
        if !display_name.is_null() {
            _g_boolean_var_139 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_139 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_139
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"display_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .set_display_name_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        display_name as *const ::core::ffi::c_char,
        io_priority as ::core::ffi::c_int,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_display_name_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_140: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_140 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_140 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_140
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_141: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            _g_boolean_var_141 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_141 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_141
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GFile>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .set_display_name_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_settable_attributes(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileAttributeInfoList {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut list: *mut GFileAttributeInfoList = ::core::ptr::null_mut::<GFileAttributeInfoList>();
    if ({
        let mut _g_boolean_var_142: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_142 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_142 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_142
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeInfoList>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileAttributeInfoList>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).query_settable_attributes.is_none() {
        return g_file_attribute_info_list_new();
    }
    my_error = ::core::ptr::null_mut::<GError>();
    list = Some(
        (*iface)
            .query_settable_attributes
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, cancellable, &raw mut my_error);
    if list.is_null() {
        if (*my_error).domain == g_io_error_quark()
            && (*my_error).code == G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int
        {
            list = g_file_attribute_info_list_new();
            g_error_free(my_error);
        } else {
            g_propagate_error(error, my_error);
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_writable_namespaces(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileAttributeInfoList {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut list: *mut GFileAttributeInfoList = ::core::ptr::null_mut::<GFileAttributeInfoList>();
    if ({
        let mut _g_boolean_var_143: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_143 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_143 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_143
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeInfoList>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileAttributeInfoList>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).query_writable_namespaces.is_none() {
        return g_file_attribute_info_list_new();
    }
    my_error = ::core::ptr::null_mut::<GError>();
    list = Some(
        (*iface)
            .query_writable_namespaces
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, cancellable, &raw mut my_error);
    if list.is_null() {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4917 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        list = g_file_attribute_info_list_new();
    }
    if !my_error.is_null() {
        if (*my_error).domain == g_io_error_quark()
            && (*my_error).code == G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int
        {
            g_error_free(my_error);
        } else {
            g_propagate_error(error, my_error);
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attribute(
    mut file: *mut GFile,
    mut attribute: *const gchar,
    mut type_0: GFileAttributeType,
    mut value_p: gpointer,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_144: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_144 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_144 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_144
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_145: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_145 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_145 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_145
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).set_attribute.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return Some((*iface).set_attribute.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        attribute as *const ::core::ffi::c_char,
        type_0,
        value_p,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attributes_from_info(
    mut file: *mut GFile,
    mut info: *mut GFileInfo,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_146: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_146 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_146 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_146
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_147: ::core::ffi::c_int = 0;
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
            _g_boolean_var_147 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_147 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_147
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return FALSE;
    }
    g_file_info_clear_status(info);
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .set_attributes_from_info
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, info, flags, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_file_real_set_attributes_from_info(
    mut file: *mut GFile,
    mut info: *mut GFileInfo,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut attributes: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut res: gboolean = 0;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    res = TRUE as gboolean;
    attributes = g_file_info_list_attributes(info, ::core::ptr::null::<::core::ffi::c_char>());
    i = 0 as ::core::ffi::c_int;
    while !(*attributes.offset(i as isize)).is_null() {
        value = _g_file_info_get_attribute_value(info, *attributes.offset(i as isize));
        if !((*value).status() as ::core::ffi::c_int
            != G_FILE_ATTRIBUTE_STATUS_UNSET as ::core::ffi::c_int)
        {
            if safe_c2rust_g_file_set_attribute(
                file,
                *attributes.offset(i as isize),
                (*value).type_0() as GFileAttributeType,
                _g_file_attribute_value_peek_as_pointer(value),
                flags,
                cancellable,
                error,
            ) == 0
            {
                (*value).set_status(
                    G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING as ::core::ffi::c_int as guint as guint,
                );
                res = FALSE as gboolean;
                error = ::core::ptr::null_mut::<*mut GError>();
            } else {
                (*value).set_status(
                    G_FILE_ATTRIBUTE_STATUS_SET as ::core::ffi::c_int as guint as guint,
                );
            }
        }
        i += 1;
    }
    g_strfreev(attributes as *mut *mut gchar);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attributes_async(
    mut file: *mut GFile,
    mut info: *mut GFileInfo,
    mut flags: GFileQueryInfoFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_148: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_148 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_148 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_148
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_149: ::core::ffi::c_int = 0;
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
            _g_boolean_var_149 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_149 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_149
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    Some(
        (*iface)
            .set_attributes_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        file,
        info,
        flags,
        io_priority,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attributes_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut info: *mut *mut GFileInfo,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_150: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_150 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_150 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_150
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_151: ::core::ffi::c_int = 0;
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
            _g_boolean_var_151 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_151 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_151
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
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .set_attributes_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, info, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attribute_string(
    mut file: *mut GFile,
    mut attribute: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_set_attribute(
        file,
        attribute as *const gchar,
        G_FILE_ATTRIBUTE_TYPE_STRING,
        value as gpointer,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attribute_byte_string(
    mut file: *mut GFile,
    mut attribute: *const gchar,
    mut value: *const gchar,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_set_attribute(
        file,
        attribute,
        G_FILE_ATTRIBUTE_TYPE_BYTE_STRING,
        value as gpointer,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attribute_uint32(
    mut file: *mut GFile,
    mut attribute: *const gchar,
    mut value: guint32,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_set_attribute(
        file,
        attribute,
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        &raw mut value as gpointer,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attribute_int32(
    mut file: *mut GFile,
    mut attribute: *const gchar,
    mut value: gint32,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_set_attribute(
        file,
        attribute,
        G_FILE_ATTRIBUTE_TYPE_INT32,
        &raw mut value as gpointer,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attribute_uint64(
    mut file: *mut GFile,
    mut attribute: *const gchar,
    mut value: guint64,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_set_attribute(
        file,
        attribute,
        G_FILE_ATTRIBUTE_TYPE_UINT64,
        &raw mut value as gpointer,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_attribute_int64(
    mut file: *mut GFile,
    mut attribute: *const gchar,
    mut value: gint64,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_set_attribute(
        file,
        attribute,
        G_FILE_ATTRIBUTE_TYPE_INT64,
        &raw mut value as gpointer,
        flags,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_mount_mountable(
    mut file: *mut GFile,
    mut flags: GMountMountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_152: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_152 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_152 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_152
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).mount_mountable.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GMountMountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_mount_mountable
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GMountMountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).mount_mountable.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        flags,
        mount_operation,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_mount_mountable_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_153: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_153 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_153 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_153
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_154: ::core::ffi::c_int = 0;
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
            _g_boolean_var_154 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_154 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_154
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return ::core::ptr::null_mut::<GFile>();
    } else if g_async_result_is_tagged(
        result,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    GMountMountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_mount_mountable
                as unsafe extern "C" fn(
                    *mut GFile,
                    GMountMountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
            as *mut GFile;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .mount_mountable_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_unmount_mountable(
    mut file: *mut GFile,
    mut flags: GMountUnmountFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_155: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_155 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_155 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_155
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).unmount_mountable.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_unmount_mountable_with_operation
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some(
        (*iface)
            .unmount_mountable
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, flags, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_unmount_mountable_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_156: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_156 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_156 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_156
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_157: ::core::ffi::c_int = 0;
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
            _g_boolean_var_157 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_157 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_157
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
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_unmount_mountable_with_operation
                as unsafe extern "C" fn(
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .unmount_mountable_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_unmount_mountable_with_operation(
    mut file: *mut GFile,
    mut flags: GMountUnmountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_158: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_158 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_158 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_158
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).unmount_mountable.is_none() && (*iface).unmount_mountable_with_operation.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_unmount_mountable_with_operation
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*iface).unmount_mountable_with_operation.is_some() {
        Some(
            (*iface)
                .unmount_mountable_with_operation
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            file,
            flags,
            mount_operation,
            cancellable,
            callback,
            user_data,
        );
    } else {
        Some(
            (*iface)
                .unmount_mountable
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(file, flags, cancellable, callback, user_data);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_unmount_mountable_with_operation_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_159: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_159 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_159 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_159
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_160: ::core::ffi::c_int = 0;
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
            _g_boolean_var_160 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_160 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_160
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
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_unmount_mountable_with_operation
                as unsafe extern "C" fn(
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).unmount_mountable_with_operation_finish.is_some() {
        return Some(
            (*iface)
                .unmount_mountable_with_operation_finish
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(file, result, error);
    } else {
        return Some(
            (*iface)
                .unmount_mountable_finish
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(file, result, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_eject_mountable(
    mut file: *mut GFile,
    mut flags: GMountUnmountFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_161: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_161 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_161 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_161
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).eject_mountable.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_eject_mountable_with_operation
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).eject_mountable.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, flags, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_eject_mountable_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_162: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_162 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_162 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_162
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_163: ::core::ffi::c_int = 0;
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
            _g_boolean_var_163 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_163 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_163
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
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_eject_mountable_with_operation
                as unsafe extern "C" fn(
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .eject_mountable_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_eject_mountable_with_operation(
    mut file: *mut GFile,
    mut flags: GMountUnmountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_164: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_164 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_164 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_164
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).eject_mountable.is_none() && (*iface).eject_mountable_with_operation.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_eject_mountable_with_operation
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*iface).eject_mountable_with_operation.is_some() {
        Some(
            (*iface)
                .eject_mountable_with_operation
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            file,
            flags,
            mount_operation,
            cancellable,
            callback,
            user_data,
        );
    } else {
        Some((*iface).eject_mountable.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            file, flags, cancellable, callback, user_data
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_eject_mountable_with_operation_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_165: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_165 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_165 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_165
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_166: ::core::ffi::c_int = 0;
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
            _g_boolean_var_166 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_166 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_166
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
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_eject_mountable_with_operation
                as unsafe extern "C" fn(
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).eject_mountable_with_operation_finish.is_some() {
        return Some(
            (*iface)
                .eject_mountable_with_operation_finish
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(file, result, error);
    } else {
        return Some(
            (*iface)
                .eject_mountable_finish
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(file, result, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_monitor_directory(
    mut file: *mut GFile,
    mut flags: GFileMonitorFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    let mut monitor: *mut GFileMonitor = ::core::ptr::null_mut::<GFileMonitor>();
    if ({
        let mut _g_boolean_var_167: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_167 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_167 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_167
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileMonitor>();
    }
    if ({
        let mut _g_boolean_var_168: ::core::ffi::c_int = 0;
        if !(flags as ::core::ffi::c_uint)
            & G_FILE_MONITOR_WATCH_HARD_LINKS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            _g_boolean_var_168 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_168 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_168
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"~flags & G_FILE_MONITOR_WATCH_HARD_LINKS\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileMonitor>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileMonitor>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).monitor_dir.is_none() {
        return _g_poll_file_monitor_new(file);
    }
    monitor = Some((*iface).monitor_dir.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        flags,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if monitor.is_null() {
        monitor = _g_poll_file_monitor_new(file);
    }
    return monitor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_monitor_file(
    mut file: *mut GFile,
    mut flags: GFileMonitorFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    let mut monitor: *mut GFileMonitor = ::core::ptr::null_mut::<GFileMonitor>();
    if ({
        let mut _g_boolean_var_169: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_169 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_169 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_169
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileMonitor>();
    }
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileMonitor>();
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    monitor = ::core::ptr::null_mut::<GFileMonitor>();
    if (*iface).monitor_file.is_some() {
        monitor = Some((*iface).monitor_file.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            file,
            flags,
            cancellable,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if monitor.is_null() {
        monitor = _g_poll_file_monitor_new(file);
    }
    return monitor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_monitor(
    mut file: *mut GFile,
    mut flags: GFileMonitorFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    if safe_c2rust_g_file_query_file_type(file, G_FILE_QUERY_INFO_NONE, cancellable)
        as ::core::ffi::c_uint
        == G_FILE_TYPE_DIRECTORY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return safe_c2rust_g_file_monitor_directory(
            file,
            (flags as ::core::ffi::c_uint
                & !(G_FILE_MONITOR_WATCH_HARD_LINKS as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as GFileMonitorFlags,
            cancellable,
            error,
        );
    } else {
        return safe_c2rust_g_file_monitor_file(file, flags, cancellable, error);
    };
}
unsafe extern "C" fn safe_c2rust_query_info_data_free(mut data: *mut QueryInfoAsyncData) {
    g_free((*data).attributes as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_query_info_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut QueryInfoAsyncData = task_data as *mut QueryInfoAsyncData;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    info = safe_c2rust_g_file_query_info(
        object as *mut GFile,
        (*data).attributes,
        (*data).flags,
        cancellable,
        &raw mut error,
    );
    if !info.is_null() {
        g_task_return_pointer(
            task,
            info as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_query_info_async(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut QueryInfoAsyncData = ::core::ptr::null_mut::<QueryInfoAsyncData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<QueryInfoAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut QueryInfoAsyncData;
    (*data).attributes = safe_c2rust_g_strdup_inline(attributes);
    (*data).flags = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    GFileQueryInfoFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_query_info_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    GFileQueryInfoFlags,
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
            b"g_file_real_query_info_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut QueryInfoAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_query_info_data_free as unsafe extern "C" fn(*mut QueryInfoAsyncData) -> (),
        )),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_query_info_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_query_info_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    if ({
        let mut _g_boolean_var_170: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_170 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_170 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_170
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileInfo;
}
unsafe extern "C" fn safe_c2rust_query_filesystem_info_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut attributes: *const ::core::ffi::c_char = task_data as *const ::core::ffi::c_char;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    info = safe_c2rust_g_file_query_filesystem_info(
        object as *mut GFile,
        attributes,
        cancellable,
        &raw mut error,
    );
    if !info.is_null() {
        g_task_return_pointer(
            task,
            info as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_query_filesystem_info_async(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_query_filesystem_info_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
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
            b"g_file_real_query_filesystem_info_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_strdup_inline(attributes) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_query_filesystem_info_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_query_filesystem_info_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    if ({
        let mut _g_boolean_var_171: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_171 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_171 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_171
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileInfo;
}
unsafe extern "C" fn safe_c2rust_enumerate_children_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut QueryInfoAsyncData = task_data as *mut QueryInfoAsyncData;
    let mut enumerator: *mut GFileEnumerator = ::core::ptr::null_mut::<GFileEnumerator>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    enumerator = safe_c2rust_g_file_enumerate_children(
        object as *mut GFile,
        (*data).attributes,
        (*data).flags,
        cancellable,
        &raw mut error,
    );
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            enumerator as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_enumerate_children_async(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut QueryInfoAsyncData = ::core::ptr::null_mut::<QueryInfoAsyncData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<QueryInfoAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut QueryInfoAsyncData;
    (*data).attributes = safe_c2rust_g_strdup_inline(attributes);
    (*data).flags = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    GFileQueryInfoFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_enumerate_children_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    GFileQueryInfoFlags,
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
            b"g_file_real_enumerate_children_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut QueryInfoAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_query_info_data_free as unsafe extern "C" fn(*mut QueryInfoAsyncData) -> (),
        )),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_enumerate_children_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_enumerate_children_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileEnumerator {
    if ({
        let mut _g_boolean_var_172: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_172 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_172 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_172
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileEnumerator;
}
unsafe extern "C" fn safe_c2rust_open_read_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GFileInputStream = ::core::ptr::null_mut::<GFileInputStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_read(object as *mut GFile, cancellable, &raw mut error);
    if !stream.is_null() {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_read_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_read_async
                as unsafe extern "C" fn(
                    *mut GFile,
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
            b"g_file_real_read_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_open_read_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_read_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileInputStream {
    if ({
        let mut _g_boolean_var_173: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_173 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_173 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_173
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileInputStream;
}
unsafe extern "C" fn safe_c2rust_append_to_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut GFileCreateFlags = task_data as *mut GFileCreateFlags;
    let mut stream: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_append_to(
        source_object as *mut GFile,
        *data,
        cancellable,
        &raw mut error,
    );
    if !stream.is_null() {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_append_to_async(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut GFileCreateFlags = ::core::ptr::null_mut::<GFileCreateFlags>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GFileCreateFlags>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GFileCreateFlags;
    *data = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    GFileCreateFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_append_to_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    GFileCreateFlags,
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
            b"g_file_real_append_to_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_append_to_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_append_to_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    if ({
        let mut _g_boolean_var_174: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_174 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_174 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_174
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileOutputStream;
}
unsafe extern "C" fn safe_c2rust_create_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut GFileCreateFlags = task_data as *mut GFileCreateFlags;
    let mut stream: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_create(
        source_object as *mut GFile,
        *data,
        cancellable,
        &raw mut error,
    );
    if !stream.is_null() {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_create_async(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut GFileCreateFlags = ::core::ptr::null_mut::<GFileCreateFlags>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GFileCreateFlags>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GFileCreateFlags;
    *data = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    GFileCreateFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_create_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    GFileCreateFlags,
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
            b"g_file_real_create_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_create_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_create_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    if ({
        let mut _g_boolean_var_175: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_175 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_175 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_175
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileOutputStream;
}
unsafe extern "C" fn safe_c2rust_replace_async_data_free(mut data: *mut ReplaceAsyncData) {
    if !(*data).stream.is_null() {
        g_object_unref((*data).stream as gpointer);
    }
    g_free((*data).etag as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_replace_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut data: *mut ReplaceAsyncData = task_data as *mut ReplaceAsyncData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_replace(
        source_object as *mut GFile,
        (*data).etag,
        (*data).make_backup,
        (*data).flags,
        cancellable,
        &raw mut error,
    );
    if !stream.is_null() {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_replace_async(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut ReplaceAsyncData = ::core::ptr::null_mut::<ReplaceAsyncData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ReplaceAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ReplaceAsyncData;
    (*data).etag = safe_c2rust_g_strdup_inline(etag);
    (*data).make_backup = make_backup;
    (*data).flags = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    gboolean,
                    GFileCreateFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_replace_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    gboolean,
                    GFileCreateFlags,
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
            b"g_file_real_replace_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ReplaceAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_replace_async_data_free
                as unsafe extern "C" fn(*mut ReplaceAsyncData) -> (),
        )),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_replace_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_replace_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    if ({
        let mut _g_boolean_var_176: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_176 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_176 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_176
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileOutputStream;
}
unsafe extern "C" fn safe_c2rust_delete_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_file_delete(object as *mut GFile, cancellable, &raw mut error) != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_delete_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_delete_async
                as unsafe extern "C" fn(
                    *mut GFile,
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
            b"g_file_real_delete_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_delete_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_delete_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_177: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_177 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_177 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_177
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_trash_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_file_trash(object as *mut GFile, cancellable, &raw mut error) != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_trash_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_trash_async
                as unsafe extern "C" fn(
                    *mut GFile,
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
            b"g_file_real_trash_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_trash_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_trash_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_178: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_178 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_178 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_178
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_move_async_data_free(mut data: *mut MoveAsyncData) {
    g_object_unref((*data).source as gpointer);
    g_object_unref((*data).destination as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<MoveAsyncData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_move_async_progress_in_main(mut user_data: gpointer) -> gboolean {
    let mut progress: *mut MoveProgressData = user_data as *mut MoveProgressData;
    let mut data: *mut MoveAsyncData = (*progress).data;
    (*data).progress_cb.expect("non-null function pointer")(
        (*progress).current_num_bytes,
        (*progress).total_num_bytes,
        (*data).progress_cb_data,
    );
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_move_async_progress_callback(
    mut current_num_bytes: goffset,
    mut total_num_bytes: goffset,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut MoveAsyncData = g_task_get_task_data(task) as *mut MoveAsyncData;
    let mut progress: *mut MoveProgressData = ::core::ptr::null_mut::<MoveProgressData>();
    progress = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<MoveProgressData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut MoveProgressData;
    (*progress).data = data;
    (*progress).current_num_bytes = current_num_bytes;
    (*progress).total_num_bytes = total_num_bytes;
    g_main_context_invoke_full(
        g_task_get_context(task),
        g_task_get_priority(task),
        Some(safe_c2rust_move_async_progress_in_main as unsafe extern "C" fn(gpointer) -> gboolean),
        safe_c2rust_g_steal_pointer(&raw mut progress as gpointer) as *mut MoveProgressData
            as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_move_async_thread(
    mut task: *mut GTask,
    mut source: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut MoveAsyncData = task_data as *mut MoveAsyncData;
    let mut result: gboolean = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    result = safe_c2rust_g_file_move(
        (*data).source,
        (*data).destination,
        (*data).flags,
        cancellable,
        if (*data).progress_cb.is_some() {
            Some(
                safe_c2rust_move_async_progress_callback
                    as unsafe extern "C" fn(goffset, goffset, gpointer) -> (),
            )
        } else {
            None
        },
        task as gpointer,
        &raw mut error,
    );
    if result != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_move_async(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut MoveAsyncData = ::core::ptr::null_mut::<MoveAsyncData>();
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<MoveAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut MoveAsyncData;
    (*data).source = g_object_ref(source as gpointer) as *mut GFile as *mut GFile;
    (*data).destination = g_object_ref(destination as gpointer) as *mut GFile as *mut GFile;
    (*data).flags = flags;
    (*data).progress_cb = progress_callback;
    (*data).progress_cb_data = progress_callback_data;
    task = g_task_new(source as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *mut GFile,
                    GFileCopyFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GFileProgressCallback,
                    gpointer,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_move_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *mut GFile,
                    GFileCopyFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GFileProgressCallback,
                    gpointer,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_real_move_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut MoveAsyncData as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut MoveAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_move_async_data_free as unsafe extern "C" fn(*mut MoveAsyncData) -> (),
        )),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_move_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_move_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_179: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, file as gpointer) != 0 {
            _g_boolean_var_179 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_179 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_179
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_make_directory_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_file_make_directory(object as *mut GFile, cancellable, &raw mut error) != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_make_directory_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_make_directory_async
                as unsafe extern "C" fn(
                    *mut GFile,
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
            b"g_file_real_make_directory_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_make_directory_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_make_directory_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_180: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_180 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_180 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_180
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_open_readwrite_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GFileIOStream = ::core::ptr::null_mut::<GFileIOStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_open_readwrite(object as *mut GFile, cancellable, &raw mut error);
    if stream.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_open_readwrite_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_open_readwrite_async
                as unsafe extern "C" fn(
                    *mut GFile,
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
            b"g_file_real_open_readwrite_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_open_readwrite_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_open_readwrite_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    if ({
        let mut _g_boolean_var_181: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_181 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_181 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_181
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileIOStream;
}
unsafe extern "C" fn safe_c2rust_create_readwrite_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut GFileCreateFlags = task_data as *mut GFileCreateFlags;
    let mut stream: *mut GFileIOStream = ::core::ptr::null_mut::<GFileIOStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_create_readwrite(
        object as *mut GFile,
        *data,
        cancellable,
        &raw mut error,
    );
    if stream.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_create_readwrite_async(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut GFileCreateFlags = ::core::ptr::null_mut::<GFileCreateFlags>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GFileCreateFlags>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GFileCreateFlags;
    *data = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    GFileCreateFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_create_readwrite_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    GFileCreateFlags,
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
            b"g_file_real_create_readwrite_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_create_readwrite_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_create_readwrite_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    if ({
        let mut _g_boolean_var_182: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_182 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_182 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_182
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileIOStream;
}
unsafe extern "C" fn safe_c2rust_replace_rw_async_data_free(mut data: *mut ReplaceRWAsyncData) {
    g_free((*data).etag as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_replace_readwrite_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut stream: *mut GFileIOStream = ::core::ptr::null_mut::<GFileIOStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut data: *mut ReplaceRWAsyncData = task_data as *mut ReplaceRWAsyncData;
    stream = safe_c2rust_g_file_replace_readwrite(
        object as *mut GFile,
        (*data).etag,
        (*data).make_backup,
        (*data).flags,
        cancellable,
        &raw mut error,
    );
    if stream.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_replace_readwrite_async(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut ReplaceRWAsyncData = ::core::ptr::null_mut::<ReplaceRWAsyncData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ReplaceRWAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ReplaceRWAsyncData;
    (*data).etag = safe_c2rust_g_strdup_inline(etag);
    (*data).make_backup = make_backup;
    (*data).flags = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    gboolean,
                    GFileCreateFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_replace_readwrite_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    gboolean,
                    GFileCreateFlags,
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
            b"g_file_real_replace_readwrite_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ReplaceRWAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_replace_rw_async_data_free
                as unsafe extern "C" fn(*mut ReplaceRWAsyncData) -> (),
        )),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_replace_readwrite_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_replace_readwrite_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    if ({
        let mut _g_boolean_var_183: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_183 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_183 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_183
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFileIOStream;
}
unsafe extern "C" fn safe_c2rust_set_display_name_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut name: *mut ::core::ffi::c_char = task_data as *mut ::core::ffi::c_char;
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    file = safe_c2rust_g_file_set_display_name(
        object as *mut GFile,
        name,
        cancellable,
        &raw mut error,
    );
    if file.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            file as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_set_display_name_async(
    mut file: *mut GFile,
    mut display_name: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_set_display_name_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *const ::core::ffi::c_char,
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
            b"g_file_real_set_display_name_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_strdup_inline(display_name) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_set_display_name_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_set_display_name_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_184: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_184 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_184 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_184
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFile;
}
unsafe extern "C" fn safe_c2rust_set_info_data_free(mut data: *mut SetInfoAsyncData) {
    if !(*data).info.is_null() {
        g_object_unref((*data).info as gpointer);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_set_info_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut SetInfoAsyncData = task_data as *mut SetInfoAsyncData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_file_set_attributes_from_info(
        object as *mut GFile,
        (*data).info,
        (*data).flags,
        cancellable,
        &raw mut error,
    ) != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_set_attributes_async(
    mut file: *mut GFile,
    mut info: *mut GFileInfo,
    mut flags: GFileQueryInfoFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut SetInfoAsyncData = ::core::ptr::null_mut::<SetInfoAsyncData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<SetInfoAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut SetInfoAsyncData;
    (*data).info = g_file_info_dup(info);
    (*data).flags = flags;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *mut GFileInfo,
                    GFileQueryInfoFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_set_attributes_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *mut GFileInfo,
                    GFileQueryInfoFlags,
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
            b"g_file_real_set_attributes_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut SetInfoAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_set_info_data_free as unsafe extern "C" fn(*mut SetInfoAsyncData) -> (),
        )),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_set_info_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_set_attributes_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut info: *mut *mut GFileInfo,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut data: *mut SetInfoAsyncData = ::core::ptr::null_mut::<SetInfoAsyncData>();
    if ({
        let mut _g_boolean_var_185: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_185 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_185 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_185
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    data = g_task_get_task_data(res as *mut ::core::ffi::c_void as *mut GTask)
        as *mut SetInfoAsyncData;
    if !info.is_null() {
        *info = g_object_ref((*data).info as gpointer) as *mut GFileInfo as *mut GFileInfo;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_find_enclosing_mount_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut mount: *mut GMount = ::core::ptr::null_mut::<GMount>();
    mount =
        safe_c2rust_g_file_find_enclosing_mount(object as *mut GFile, cancellable, &raw mut error);
    if mount.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            mount as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_find_enclosing_mount_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_find_enclosing_mount_async
                as unsafe extern "C" fn(
                    *mut GFile,
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
            b"g_file_real_find_enclosing_mount_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_find_enclosing_mount_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_find_enclosing_mount_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GMount {
    if ({
        let mut _g_boolean_var_186: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_186 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_186 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_186
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMount>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GMount;
}
unsafe extern "C" fn safe_c2rust_copy_async_data_free(mut data: *mut CopyAsyncData) {
    g_object_unref((*data).source as gpointer);
    g_object_unref((*data).destination as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<CopyAsyncData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_copy_async_progress_in_main(mut user_data: gpointer) -> gboolean {
    let mut progress: *mut CopyProgressData = user_data as *mut CopyProgressData;
    let mut data: *mut CopyAsyncData = (*progress).data;
    (*data).progress_cb.expect("non-null function pointer")(
        (*progress).current_num_bytes,
        (*progress).total_num_bytes,
        (*data).progress_cb_data,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_copy_async_progress_callback(
    mut current_num_bytes: goffset,
    mut total_num_bytes: goffset,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut CopyAsyncData = g_task_get_task_data(task) as *mut CopyAsyncData;
    let mut progress: *mut CopyProgressData = ::core::ptr::null_mut::<CopyProgressData>();
    progress = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<CopyProgressData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut CopyProgressData;
    (*progress).data = data;
    (*progress).current_num_bytes = current_num_bytes;
    (*progress).total_num_bytes = total_num_bytes;
    g_main_context_invoke_full(
        g_task_get_context(task),
        g_task_get_priority(task),
        Some(safe_c2rust_copy_async_progress_in_main as unsafe extern "C" fn(gpointer) -> gboolean),
        progress as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_copy_async_thread(
    mut task: *mut GTask,
    mut source: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut CopyAsyncData = task_data as *mut CopyAsyncData;
    let mut result: gboolean = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    result = safe_c2rust_g_file_copy(
        (*data).source,
        (*data).destination,
        (*data).flags,
        cancellable,
        if (*data).progress_cb.is_some() {
            Some(
                safe_c2rust_copy_async_progress_callback
                    as unsafe extern "C" fn(goffset, goffset, gpointer) -> (),
            )
        } else {
            None
        },
        task as gpointer,
        &raw mut error,
    );
    if result != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_copy_async(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut CopyAsyncData = ::core::ptr::null_mut::<CopyAsyncData>();
    data = g_slice_alloc(::core::mem::size_of::<CopyAsyncData>() as gsize) as *mut CopyAsyncData;
    (*data).source = g_object_ref(source as gpointer) as *mut GFile as *mut GFile;
    (*data).destination = g_object_ref(destination as gpointer) as *mut GFile as *mut GFile;
    (*data).flags = flags;
    (*data).progress_cb = progress_callback;
    (*data).progress_cb_data = progress_callback_data;
    task = g_task_new(source as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *mut GFile,
                    GFileCopyFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GFileProgressCallback,
                    gpointer,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_copy_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *mut GFile,
                    GFileCopyFlags,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GFileProgressCallback,
                    gpointer,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_real_copy_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut CopyAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_copy_async_data_free as unsafe extern "C" fn(*mut CopyAsyncData) -> (),
        )),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_copy_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_copy_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_187: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_187 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_187 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_187
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_for_path(
    mut path: *const ::core::ffi::c_char,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_188: ::core::ffi::c_int = 0;
        if !path.is_null() {
            _g_boolean_var_188 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_188 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_188
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return g_vfs_get_file_for_path(g_vfs_get_default(), path);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_for_uri(
    mut uri: *const ::core::ffi::c_char,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_189: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_189 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_189 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_189
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return g_vfs_get_file_for_uri(g_vfs_get_default(), uri);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_tmp(
    mut tmpl: *const ::core::ffi::c_char,
    mut iostream: *mut *mut GFileIOStream,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut fd: gint = 0;
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut output: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    if ({
        let mut _g_boolean_var_190: ::core::ffi::c_int = 0;
        if !iostream.is_null() {
            _g_boolean_var_190 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_190 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_190
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"iostream != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    fd = g_file_open_tmp(tmpl as *const gchar, &raw mut path, error);
    if fd == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<GFile>();
    }
    file = safe_c2rust_g_file_new_for_path(path);
    output = _g_local_file_output_stream_new(fd as ::core::ffi::c_int);
    *iostream = _g_local_file_io_stream_new(
        output as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream,
    );
    g_object_unref(output as gpointer);
    g_free(path as gpointer);
    return file;
}
unsafe extern "C" fn safe_c2rust_new_tmp_data_free(mut data: *mut NewTmpAsyncData) {
    let mut _pp: *mut *mut GFile = &raw mut (*data).file;
    let mut _ptr: *mut GFile = *_pp;
    *_pp = ::core::ptr::null_mut::<GFile>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GFileIOStream = &raw mut (*data).iostream;
    let mut _ptr_0: *mut GFileIOStream = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GFileIOStream>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_new_tmp_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut tmpl: *const ::core::ffi::c_char = task_data as *const ::core::ffi::c_char;
    let mut iostream: *mut GFileIOStream = ::core::ptr::null_mut::<GFileIOStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut return_data: *mut NewTmpAsyncData = ::core::ptr::null_mut::<NewTmpAsyncData>();
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    file = safe_c2rust_g_file_new_tmp(tmpl, &raw mut iostream, &raw mut error);
    if file.is_null() {
        let mut error_code: ::core::ffi::c_int = G_IO_ERROR_FAILED as ::core::ffi::c_int;
        if (*error).domain == g_io_error_quark() {
            g_task_return_error(
                task,
                safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
            );
            return;
        }
        if (*error).domain == g_file_error_quark() {
            error_code =
                g_io_error_from_file_error((*error).code as GFileError) as ::core::ffi::c_int;
        }
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            error_code as gint,
            glib_gettext(
                b"Failed to create a temporary directory for template \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                    as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char,
            tmpl,
            (*error).message,
        );
        g_clear_error(&raw mut error);
        return;
    }
    return_data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<NewTmpAsyncData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut NewTmpAsyncData;
    (*return_data).file =
        safe_c2rust_g_steal_pointer(&raw mut file as gpointer) as *mut GFile as *mut GFile;
    (*return_data).iostream = safe_c2rust_g_steal_pointer(&raw mut iostream as gpointer)
        as *mut GFileIOStream as *mut GFileIOStream;
    g_task_return_pointer(
        task,
        safe_c2rust_g_steal_pointer(&raw mut return_data as gpointer) as *mut NewTmpAsyncData
            as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut NewTmpAsyncData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_new_tmp_data_free as unsafe extern "C" fn(*mut NewTmpAsyncData) -> (),
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_tmp_async(
    mut tmpl: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_191: ::core::ffi::c_int = 0;
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
            _g_boolean_var_191 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_191 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_191
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
    task = g_task_new(NULL_0, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_new_tmp_async
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
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
            b"g_file_new_tmp_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_strdup_inline(tmpl) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_set_check_cancellable(task, TRUE);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_new_tmp_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_tmp_finish(
    mut result: *mut GAsyncResult,
    mut iostream: *mut *mut GFileIOStream,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut data: *mut NewTmpAsyncData = ::core::ptr::null_mut::<NewTmpAsyncData>();
    if ({
        let mut _g_boolean_var_192: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            result as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
        {
            _g_boolean_var_192 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_192 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_192
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_193: ::core::ffi::c_int = 0;
        if g_task_get_source_tag(result as *mut ::core::ffi::c_void as *mut GTask)
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_new_tmp_async
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            ))
        {
            _g_boolean_var_193 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_193 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_193
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_get_source_tag (G_TASK (result)) == g_file_new_tmp_async\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_194: ::core::ffi::c_int = 0;
        if !iostream.is_null() {
            _g_boolean_var_194 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_194 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_194
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"iostream != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_195: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_195 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_195 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_195
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    data = g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut NewTmpAsyncData;
    if data.is_null() {
        *iostream = ::core::ptr::null_mut::<GFileIOStream>();
        return ::core::ptr::null_mut::<GFile>();
    }
    file =
        safe_c2rust_g_steal_pointer(&raw mut (*data).file as gpointer) as *mut GFile as *mut GFile;
    *iostream = safe_c2rust_g_steal_pointer(&raw mut (*data).iostream as gpointer)
        as *mut GFileIOStream as *mut GFileIOStream;
    safe_c2rust_new_tmp_data_free(data);
    return file;
}
unsafe extern "C" fn safe_c2rust_new_tmp_dir_async_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmpl: *const ::core::ffi::c_char = task_data as *const ::core::ffi::c_char;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    path = g_dir_make_tmp(tmpl as *const gchar, &raw mut error);
    if path.is_null() {
        let mut error_code: ::core::ffi::c_int = G_IO_ERROR_FAILED as ::core::ffi::c_int;
        if (*error).domain == g_io_error_quark() {
            g_task_return_error(
                task,
                safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
            );
            return;
        }
        if (*error).domain == g_file_error_quark() {
            error_code =
                g_io_error_from_file_error((*error).code as GFileError) as ::core::ffi::c_int;
        }
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            error_code as gint,
            glib_gettext(
                b"Failed to create a temporary directory for template \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                    as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char,
            tmpl,
            (*error).message,
        );
        g_clear_error(&raw mut error);
        return;
    }
    g_task_return_pointer(
        task,
        safe_c2rust_g_file_new_for_path(path) as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_free(path as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_tmp_dir_async(
    mut tmpl: *const ::core::ffi::c_char,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_196: ::core::ffi::c_int = 0;
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
            _g_boolean_var_196 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_196 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_196
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
    task = g_task_new(NULL_0, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_new_tmp_dir_async
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
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
            b"g_file_new_tmp_dir_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_strdup_inline(tmpl) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority as gint);
    g_task_set_check_cancellable(task, TRUE);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_new_tmp_dir_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_tmp_dir_finish(
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_197: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            result as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
        {
            _g_boolean_var_197 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_197 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_197
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_198: ::core::ffi::c_int = 0;
        if g_task_get_source_tag(result as *mut ::core::ffi::c_void as *mut GTask)
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_new_tmp_dir_async
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            ))
        {
            _g_boolean_var_198 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_198 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_198
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_get_source_tag (G_TASK (result)) == g_file_new_tmp_dir_async\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_199: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_199 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_199 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_199
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GFile;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_parse_name(
    mut parse_name: *const ::core::ffi::c_char,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_200: ::core::ffi::c_int = 0;
        if !parse_name.is_null() {
            _g_boolean_var_200 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_200 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_200
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"parse_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return g_vfs_parse_name(g_vfs_get_default(), parse_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_build_filename(
    mut first_element: *const gchar,
    mut args: ...
) -> *mut GFile {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut args_0: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_201: ::core::ffi::c_int = 0;
        if !first_element.is_null() {
            _g_boolean_var_201 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_201 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_201
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"first_element != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    args_0 = args.clone();
    str = g_build_filename_valist(first_element, &raw mut args_0);
    file = safe_c2rust_g_file_new_for_path(str);
    g_free(str as gpointer);
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_build_filenamev(
    mut args: *const *const gchar,
) -> *mut GFile {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    str = g_build_filenamev(args as *mut *mut gchar);
    file = safe_c2rust_g_file_new_for_path(str);
    g_free(str as gpointer);
    return file;
}
unsafe extern "C" fn safe_c2rust_is_valid_scheme_character(mut c: ::core::ffi::c_char) -> gboolean {
    return (*safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALNUM as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || c as ::core::ffi::c_int == '+' as i32
        || c as ::core::ffi::c_int == '-' as i32
        || c as ::core::ffi::c_int == '.' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_has_valid_scheme(mut uri: *const ::core::ffi::c_char) -> gboolean {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = uri;
    if !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALPHA as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int)
    {
        return FALSE;
    }
    loop {
        p = p.offset(1);
        if !(safe_c2rust_is_valid_scheme_character(*p) != 0) {
            break;
        }
    }
    return (*p as ::core::ffi::c_int == ':' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_new_for_cmdline_arg(
    mut arg: *const gchar,
    mut cwd: *const gchar,
) -> *mut GFile {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if g_path_is_absolute(arg) != 0 {
        return safe_c2rust_g_file_new_for_path(arg as *const ::core::ffi::c_char);
    }
    if safe_c2rust_has_valid_scheme(arg as *const ::core::ffi::c_char) != 0 {
        return safe_c2rust_g_file_new_for_uri(arg as *const ::core::ffi::c_char);
    }
    if cwd.is_null() {
        let mut current_dir: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        current_dir = g_get_current_dir() as *mut ::core::ffi::c_char;
        filename = g_build_filename(current_dir, arg, NULL_0) as *mut ::core::ffi::c_char;
        g_free(current_dir as gpointer);
    } else {
        filename = g_build_filename(cwd, arg, NULL_0) as *mut ::core::ffi::c_char;
    }
    file = safe_c2rust_g_file_new_for_path(filename);
    g_free(filename as gpointer);
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_for_commandline_arg(
    mut arg: *const ::core::ffi::c_char,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_202: ::core::ffi::c_int = 0;
        if !arg.is_null() {
            _g_boolean_var_202 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_202 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_202
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"arg != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return safe_c2rust_new_for_cmdline_arg(arg as *const gchar, ::core::ptr::null::<gchar>());
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_new_for_commandline_arg_and_cwd(
    mut arg: *const gchar,
    mut cwd: *const gchar,
) -> *mut GFile {
    if ({
        let mut _g_boolean_var_203: ::core::ffi::c_int = 0;
        if !arg.is_null() {
            _g_boolean_var_203 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_203 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_203
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"arg != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_204: ::core::ffi::c_int = 0;
        if !cwd.is_null() {
            _g_boolean_var_204 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_204 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_204
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cwd != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    return safe_c2rust_new_for_cmdline_arg(arg, cwd);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_mount_enclosing_volume(
    mut location: *mut GFile,
    mut flags: GMountMountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_205: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = location as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_205 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_205 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_205
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (location)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(location as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).mount_enclosing_volume.is_none() {
        g_task_report_new_error(
            location as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GMountMountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_mount_enclosing_volume
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GMountMountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"volume doesn\xE2\x80\x99t implement mount\0" as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char,
        );
        return;
    }
    Some(
        (*iface)
            .mount_enclosing_volume
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        location,
        flags,
        mount_operation,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_mount_enclosing_volume_finish(
    mut location: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_206: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = location as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_206 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_206 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_206
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (location)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_207: ::core::ffi::c_int = 0;
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
            _g_boolean_var_207 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_207 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_207
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
                    *mut GFile,
                    GMountMountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_mount_enclosing_volume
                as unsafe extern "C" fn(
                    *mut GFile,
                    GMountMountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(location as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .mount_enclosing_volume_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(location, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_default_handler(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GAppInfo {
    let mut uri_scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut content_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut appinfo: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    uri_scheme = safe_c2rust_g_file_get_uri_scheme(file);
    if !uri_scheme.is_null()
        && *uri_scheme.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        appinfo = g_app_info_get_default_for_uri_scheme(uri_scheme);
        g_free(uri_scheme as gpointer);
        if !appinfo.is_null() {
            return appinfo;
        }
    } else {
        g_free(uri_scheme as gpointer);
    }
    info = safe_c2rust_g_file_query_info(
        file,
        b"standard::content-type,standard::fast-content-type\0" as *const u8
            as *const ::core::ffi::c_char,
        G_FILE_QUERY_INFO_NONE,
        cancellable,
        error,
    );
    if info.is_null() {
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    appinfo = ::core::ptr::null_mut::<GAppInfo>();
    content_type = g_file_info_get_content_type(info);
    if content_type.is_null() {
        content_type = g_file_info_get_attribute_string(
            info,
            G_FILE_ATTRIBUTE_STANDARD_FAST_CONTENT_TYPE.as_ptr(),
        );
    }
    if !content_type.is_null() {
        path = safe_c2rust_g_file_get_path(file);
        appinfo = g_app_info_get_default_for_type(
            content_type,
            (path == NULL_0 as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
        );
        g_free(path as gpointer);
    }
    g_object_unref(info as gpointer);
    if !appinfo.is_null() {
        return appinfo;
    }
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        glib_gettext(
            b"No application is registered as handling this file\0" as *const u8 as *const gchar,
        ),
    );
    return ::core::ptr::null_mut::<GAppInfo>();
}
unsafe extern "C" fn safe_c2rust_query_default_handler_query_app_info_for_type_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut appinfo: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    appinfo = g_app_info_get_default_for_type_finish(result, &raw mut error);
    if !appinfo.is_null() {
        g_task_return_pointer(
            task,
            safe_c2rust_g_steal_pointer(&raw mut appinfo as gpointer) as *mut GAppInfo as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else if g_error_matches(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
    ) != 0
    {
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            (*error).message,
        );
    } else {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    }
    g_clear_error(&raw mut error);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_query_default_handler_query_info_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut file: *mut GFile = object as *mut ::core::ffi::c_void as *mut GFile;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut content_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    info = safe_c2rust_g_file_query_info_finish(file, result, &raw mut error);
    if info.is_null() {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
        g_object_unref(task as gpointer);
        return;
    }
    content_type = g_file_info_get_content_type(info);
    if content_type.is_null() {
        content_type = g_file_info_get_attribute_string(
            info,
            G_FILE_ATTRIBUTE_STANDARD_FAST_CONTENT_TYPE.as_ptr(),
        );
    }
    if !content_type.is_null() {
        let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        path = safe_c2rust_g_file_get_path(file);
        g_app_info_get_default_for_type_async(
            content_type,
            (path == NULL_0 as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
            cancellable,
            Some(
                safe_c2rust_query_default_handler_query_app_info_for_type_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
        g_free(path as gpointer);
    } else {
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No application is registered as handling this file\0" as *const u8
                    as *const gchar,
            ) as *const ::core::ffi::c_char,
        );
    }
    g_object_unref(info as gpointer);
    let mut _pp: *mut *mut GTask = &raw mut task;
    let mut _ptr: *mut GTask = *_pp;
    *_pp = ::core::ptr::null_mut::<GTask>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_on_query_default_handler_for_uri_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut app_info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    app_info = g_app_info_get_default_for_uri_scheme_finish(
        result,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !app_info.is_null() {
        g_task_return_pointer(
            task,
            safe_c2rust_g_steal_pointer(&raw mut app_info as gpointer) as *mut GAppInfo as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_g_file_query_info_async(
            g_task_get_source_object(task) as *mut GFile,
            b"standard::content-type,standard::fast-content-type\0" as *const u8
                as *const ::core::ffi::c_char,
            G_FILE_QUERY_INFO_NONE,
            g_task_get_priority(task) as ::core::ffi::c_int,
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_query_default_handler_query_info_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_default_handler_async(
    mut file: *mut GFile,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut uri_scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_query_default_handler_async
                as unsafe extern "C" fn(
                    *mut GFile,
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
            b"g_file_query_default_handler_async\0" as *const u8 as *const gchar,
        );
    }
    uri_scheme = safe_c2rust_g_file_get_uri_scheme(file);
    if !uri_scheme.is_null()
        && *uri_scheme.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        g_app_info_get_default_for_uri_scheme_async(
            uri_scheme,
            cancellable,
            Some(
                safe_c2rust_on_query_default_handler_for_uri_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
        g_free(uri_scheme as gpointer);
        return;
    }
    safe_c2rust_g_file_query_info_async(
        file,
        b"standard::content-type,standard::fast-content-type\0" as *const u8
            as *const ::core::ffi::c_char,
        G_FILE_QUERY_INFO_NONE,
        io_priority,
        cancellable,
        Some(
            safe_c2rust_query_default_handler_query_info_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
    );
    g_free(uri_scheme as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_default_handler_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GAppInfo {
    if ({
        let mut _g_boolean_var_208: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_208 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_208 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_208
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    if ({
        let mut _g_boolean_var_209: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, file as gpointer) != 0 {
            _g_boolean_var_209 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_209 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_209
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GAppInfo;
}
pub const GET_CONTENT_BLOCK_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_contents(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut contents: *mut *mut ::core::ffi::c_char,
    mut length: *mut gsize,
    mut etag_out: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut in_0: *mut GFileInputStream = ::core::ptr::null_mut::<GFileInputStream>();
    let mut content: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    let mut pos: gsize = 0;
    let mut res: gssize = 0;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    if ({
        let mut _g_boolean_var_210: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_210 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_210 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_210
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_211: ::core::ffi::c_int = 0;
        if !contents.is_null() {
            _g_boolean_var_211 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_211 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_211
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"contents != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    in_0 = safe_c2rust_g_file_read(file, cancellable, error);
    if in_0.is_null() {
        return FALSE;
    }
    content = g_byte_array_new();
    pos = 0 as gsize;
    g_byte_array_set_size(
        content,
        pos.wrapping_add(GET_CONTENT_BLOCK_SIZE as gsize)
            .wrapping_add(1 as gsize) as guint,
    );
    loop {
        res = g_input_stream_read(
            in_0 as *mut ::core::ffi::c_void as *mut GInputStream,
            (*content).data.offset(pos as isize) as *mut ::core::ffi::c_void,
            GET_CONTENT_BLOCK_SIZE as gsize,
            cancellable,
            error,
        );
        if !(res > 0 as gssize) {
            break;
        }
        pos = pos.wrapping_add(res as gsize);
        g_byte_array_set_size(
            content,
            pos.wrapping_add(GET_CONTENT_BLOCK_SIZE as gsize)
                .wrapping_add(1 as gsize) as guint,
        );
    }
    if !etag_out.is_null() {
        *etag_out = ::core::ptr::null_mut::<::core::ffi::c_char>();
        info = g_file_input_stream_query_info(
            in_0,
            G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr(),
            cancellable,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !info.is_null() {
            *etag_out =
                if g_file_info_has_attribute(info, G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr()) != 0 {
                    safe_c2rust_g_strdup_inline(g_file_info_get_etag(info))
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                };
            g_object_unref(info as gpointer);
        }
    }
    g_input_stream_close(
        in_0 as *mut ::core::ffi::c_void as *mut GInputStream,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(in_0 as gpointer);
    if res < 0 as gssize {
        g_byte_array_free(content, TRUE);
        return FALSE;
    }
    if !length.is_null() {
        *length = pos;
    }
    *(*content).data.offset(pos as isize) = 0 as guint8;
    *contents = g_byte_array_free(content, FALSE) as *mut ::core::ffi::c_char;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_load_contents_data_free(mut data: *mut LoadContentsData) {
    if !(*data).content.is_null() {
        g_byte_array_free((*data).content, TRUE);
    }
    g_free((*data).etag as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_load_contents_close_callback(
    mut obj: *mut GObject,
    mut close_res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GInputStream = obj as *mut ::core::ffi::c_void as *mut GInputStream;
    let mut data: *mut LoadContentsData = user_data as *mut LoadContentsData;
    g_input_stream_close_finish(stream, close_res, ::core::ptr::null_mut::<*mut GError>());
    g_object_unref(stream as gpointer);
    g_task_return_boolean((*data).task, TRUE);
    g_object_unref((*data).task as gpointer);
}
unsafe extern "C" fn safe_c2rust_load_contents_fstat_callback(
    mut obj: *mut GObject,
    mut stat_res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GInputStream = obj as *mut ::core::ffi::c_void as *mut GInputStream;
    let mut data: *mut LoadContentsData = user_data as *mut LoadContentsData;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    info = g_file_input_stream_query_info_finish(
        stream as *mut ::core::ffi::c_void as *mut GFileInputStream,
        stat_res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !info.is_null() {
        (*data).etag = if g_file_info_has_attribute(info, G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr()) != 0
        {
            safe_c2rust_g_strdup_inline(g_file_info_get_etag(info))
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        g_object_unref(info as gpointer);
    }
    g_input_stream_close_async(
        stream,
        0 as ::core::ffi::c_int,
        g_task_get_cancellable((*data).task),
        Some(
            safe_c2rust_load_contents_close_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_load_contents_read_callback(
    mut obj: *mut GObject,
    mut read_res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GInputStream = obj as *mut ::core::ffi::c_void as *mut GInputStream;
    let mut data: *mut LoadContentsData = user_data as *mut LoadContentsData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut read_size: gssize = 0;
    read_size = g_input_stream_read_finish(stream, read_res, &raw mut error);
    if read_size < 0 as gssize {
        g_task_return_error((*data).task, error);
        g_object_unref((*data).task as gpointer);
        g_input_stream_close_async(
            stream,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_0,
        );
        g_object_unref(stream as gpointer);
    } else if read_size == 0 as gssize {
        g_file_input_stream_query_info_async(
            stream as *mut ::core::ffi::c_void as *mut GFileInputStream,
            G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr(),
            0 as ::core::ffi::c_int,
            g_task_get_cancellable((*data).task),
            Some(
                safe_c2rust_load_contents_fstat_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            data as gpointer,
        );
    } else if read_size > 0 as gssize {
        (*data).pos = (*data).pos.wrapping_add(read_size as gsize);
        g_byte_array_set_size(
            (*data).content,
            (*data).pos.wrapping_add(GET_CONTENT_BLOCK_SIZE as gsize) as guint,
        );
        if (*data).read_more_callback.is_some()
            && (*data)
                .read_more_callback
                .expect("non-null function pointer")(
                (*(*data).content).data as *mut ::core::ffi::c_char,
                (*data).pos as goffset,
                g_async_result_get_user_data(
                    (*data).task as *mut ::core::ffi::c_void as *mut GAsyncResult,
                ),
            ) == 0
        {
            g_file_input_stream_query_info_async(
                stream as *mut ::core::ffi::c_void as *mut GFileInputStream,
                G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr(),
                0 as ::core::ffi::c_int,
                g_task_get_cancellable((*data).task),
                Some(
                    safe_c2rust_load_contents_fstat_callback
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                data as gpointer,
            );
        } else {
            g_input_stream_read_async(
                stream,
                (*(*data).content).data.offset((*data).pos as isize) as *mut ::core::ffi::c_void,
                GET_CONTENT_BLOCK_SIZE as gsize,
                0 as ::core::ffi::c_int,
                g_task_get_cancellable((*data).task),
                Some(
                    safe_c2rust_load_contents_read_callback
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                data as gpointer,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_load_contents_open_callback(
    mut obj: *mut GObject,
    mut open_res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut file: *mut GFile = obj as *mut ::core::ffi::c_void as *mut GFile;
    let mut stream: *mut GFileInputStream = ::core::ptr::null_mut::<GFileInputStream>();
    let mut data: *mut LoadContentsData = user_data as *mut LoadContentsData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_read_finish(file, open_res, &raw mut error);
    if !stream.is_null() {
        g_byte_array_set_size(
            (*data).content,
            (*data).pos.wrapping_add(GET_CONTENT_BLOCK_SIZE as gsize) as guint,
        );
        g_input_stream_read_async(
            stream as *mut ::core::ffi::c_void as *mut GInputStream,
            (*(*data).content).data.offset((*data).pos as isize) as *mut ::core::ffi::c_void,
            GET_CONTENT_BLOCK_SIZE as gsize,
            0 as ::core::ffi::c_int,
            g_task_get_cancellable((*data).task),
            Some(
                safe_c2rust_load_contents_read_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            data as gpointer,
        );
    } else {
        g_task_return_error((*data).task, error);
        g_object_unref((*data).task as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_partial_contents_async(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut read_more_callback: GFileReadMoreCallback,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut LoadContentsData = ::core::ptr::null_mut::<LoadContentsData>();
    if ({
        let mut _g_boolean_var_212: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_212 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_212 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_212
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LoadContentsData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LoadContentsData;
    (*data).read_more_callback = read_more_callback;
    (*data).content = g_byte_array_new();
    (*data).task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = (*data).task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *mut GCancellable,
                    GFileReadMoreCallback,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_load_partial_contents_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *mut GCancellable,
                    GFileReadMoreCallback,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_load_partial_contents_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        (*data).task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut LoadContentsData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_load_contents_data_free
                as unsafe extern "C" fn(*mut LoadContentsData) -> (),
        )),
    );
    safe_c2rust_g_file_read_async(
        file,
        0 as ::core::ffi::c_int,
        g_task_get_cancellable((*data).task),
        Some(
            safe_c2rust_load_contents_open_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_partial_contents_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut contents: *mut *mut ::core::ffi::c_char,
    mut length: *mut gsize,
    mut etag_out: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LoadContentsData = ::core::ptr::null_mut::<LoadContentsData>();
    if ({
        let mut _g_boolean_var_213: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_213 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_213 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_213
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_214: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_214 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_214 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_214
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_215: ::core::ffi::c_int = 0;
        if !contents.is_null() {
            _g_boolean_var_215 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_215 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_215
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"contents != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    task = res as *mut ::core::ffi::c_void as *mut GTask;
    if g_task_propagate_boolean(task, error) == 0 {
        if !length.is_null() {
            *length = 0 as gsize;
        }
        return FALSE;
    }
    data = g_task_get_task_data(task) as *mut LoadContentsData;
    if !length.is_null() {
        *length = (*data).pos;
    }
    if !etag_out.is_null() {
        *etag_out = (*data).etag;
        (*data).etag = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    g_byte_array_set_size(
        (*data).content,
        (*data).pos.wrapping_add(1 as gsize) as guint,
    );
    *(*(*data).content).data.offset((*data).pos as isize) = 0 as guint8;
    *contents = g_byte_array_free((*data).content, FALSE) as *mut ::core::ffi::c_char;
    (*data).content = ::core::ptr::null_mut::<GByteArray>();
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_contents_async(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_g_file_load_partial_contents_async(file, cancellable, None, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_contents_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut contents: *mut *mut ::core::ffi::c_char,
    mut length: *mut gsize,
    mut etag_out: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_load_partial_contents_finish(
        file, res, contents, length, etag_out, error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_contents(
    mut file: *mut GFile,
    mut contents: *const ::core::ffi::c_char,
    mut length: gsize,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut new_etag: *mut *mut ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut out: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut pos: gsize = 0;
    let mut remainder: gsize = 0;
    let mut res: gssize = -(1 as ::core::ffi::c_int) as gssize;
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_216: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_216 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_216 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_216
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_217: ::core::ffi::c_int = 0;
        if !contents.is_null() {
            _g_boolean_var_217 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_217 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_217
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"contents != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    out = safe_c2rust_g_file_replace(file, etag, make_backup, flags, cancellable, error);
    if out.is_null() {
        return FALSE;
    }
    pos = 0 as gsize;
    remainder = length;
    while remainder > 0 as gsize && {
        res = g_output_stream_write(
            out as *mut ::core::ffi::c_void as *mut GOutputStream,
            contents.offset(pos as isize) as *const ::core::ffi::c_void,
            (if remainder < 8192 as gsize {
                remainder
            } else {
                8192 as gsize
            }),
            cancellable,
            error,
        );
        res > 0 as gssize
    } {
        pos = pos.wrapping_add(res as gsize);
        remainder = remainder.wrapping_sub(res as gsize);
    }
    if remainder > 0 as gsize && res < 0 as gssize {
        g_output_stream_close(
            out as *mut ::core::ffi::c_void as *mut GOutputStream,
            cancellable,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(out as gpointer);
        return FALSE;
    }
    ret = g_output_stream_close(
        out as *mut ::core::ffi::c_void as *mut GOutputStream,
        cancellable,
        error,
    );
    if !new_etag.is_null() {
        *new_etag = g_file_output_stream_get_etag(out);
    }
    g_object_unref(out as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_replace_contents_data_free(mut data: *mut ReplaceContentsData) {
    g_bytes_unref((*data).content);
    g_free((*data).etag as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_replace_contents_close_callback(
    mut obj: *mut GObject,
    mut close_res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GOutputStream = obj as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut data: *mut ReplaceContentsData = user_data as *mut ReplaceContentsData;
    g_output_stream_close_finish(stream, close_res, ::core::ptr::null_mut::<*mut GError>());
    if (*data).failed == 0 {
        (*data).etag = g_file_output_stream_get_etag(
            stream as *mut ::core::ffi::c_void as *mut GFileOutputStream,
        );
        g_task_return_boolean((*data).task, TRUE);
    }
    g_object_unref((*data).task as gpointer);
}
unsafe extern "C" fn safe_c2rust_replace_contents_write_callback(
    mut obj: *mut GObject,
    mut read_res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut stream: *mut GOutputStream = obj as *mut ::core::ffi::c_void as *mut GOutputStream;
    let mut data: *mut ReplaceContentsData = user_data as *mut ReplaceContentsData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut write_size: gssize = 0;
    write_size = g_output_stream_write_finish(stream, read_res, &raw mut error);
    if write_size <= 0 as gssize {
        if write_size < 0 as gssize {
            (*data).failed = TRUE as gboolean;
            g_task_return_error((*data).task, error);
        }
        g_output_stream_close_async(
            stream,
            0 as ::core::ffi::c_int,
            g_task_get_cancellable((*data).task),
            Some(
                safe_c2rust_replace_contents_close_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            data as gpointer,
        );
    } else if write_size > 0 as gssize {
        let mut content: *const gchar = ::core::ptr::null::<gchar>();
        let mut length: gsize = 0;
        content = g_bytes_get_data((*data).content, &raw mut length) as *const gchar;
        (*data).pos = (*data).pos.wrapping_add(write_size as gsize);
        if (*data).pos >= length {
            g_output_stream_close_async(
                stream,
                0 as ::core::ffi::c_int,
                g_task_get_cancellable((*data).task),
                Some(
                    safe_c2rust_replace_contents_close_callback
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                data as gpointer,
            );
        } else {
            g_output_stream_write_async(
                stream,
                content.offset((*data).pos as isize) as *const ::core::ffi::c_void,
                length.wrapping_sub((*data).pos),
                0 as ::core::ffi::c_int,
                g_task_get_cancellable((*data).task),
                Some(
                    safe_c2rust_replace_contents_write_callback
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                data as gpointer,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_replace_contents_open_callback(
    mut obj: *mut GObject,
    mut open_res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut file: *mut GFile = obj as *mut ::core::ffi::c_void as *mut GFile;
    let mut stream: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut data: *mut ReplaceContentsData = user_data as *mut ReplaceContentsData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_file_replace_finish(file, open_res, &raw mut error);
    if !stream.is_null() {
        let mut content: *const gchar = ::core::ptr::null::<gchar>();
        let mut length: gsize = 0;
        content = g_bytes_get_data((*data).content, &raw mut length) as *const gchar;
        g_output_stream_write_async(
            stream as *mut ::core::ffi::c_void as *mut GOutputStream,
            content.offset((*data).pos as isize) as *const ::core::ffi::c_void,
            length.wrapping_sub((*data).pos),
            0 as ::core::ffi::c_int,
            g_task_get_cancellable((*data).task),
            Some(
                safe_c2rust_replace_contents_write_callback
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            data as gpointer,
        );
        g_object_unref(stream as gpointer);
    } else {
        g_task_return_error((*data).task, error);
        g_object_unref((*data).task as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_contents_async(
    mut file: *mut GFile,
    mut contents: *const ::core::ffi::c_char,
    mut length: gsize,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    bytes = g_bytes_new_static(contents as gconstpointer, length);
    safe_c2rust_g_file_replace_contents_bytes_async(
        file,
        bytes,
        etag,
        make_backup,
        flags,
        cancellable,
        callback,
        user_data,
    );
    g_bytes_unref(bytes);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_contents_bytes_async(
    mut file: *mut GFile,
    mut contents: *mut GBytes,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: *mut ReplaceContentsData = ::core::ptr::null_mut::<ReplaceContentsData>();
    if ({
        let mut _g_boolean_var_218: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_218 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_218 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_218
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_219: ::core::ffi::c_int = 0;
        if !contents.is_null() {
            _g_boolean_var_219 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_219 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_219
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"contents != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ReplaceContentsData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ReplaceContentsData;
    (*data).content = g_bytes_ref(contents);
    (*data).task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = (*data).task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *mut GBytes,
                    *const ::core::ffi::c_char,
                    gboolean,
                    GFileCreateFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_replace_contents_bytes_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *mut GBytes,
                    *const ::core::ffi::c_char,
                    gboolean,
                    GFileCreateFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_replace_contents_bytes_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        (*data).task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ReplaceContentsData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_replace_contents_data_free
                as unsafe extern "C" fn(*mut ReplaceContentsData) -> (),
        )),
    );
    safe_c2rust_g_file_replace_async(
        file,
        etag,
        make_backup,
        flags,
        0 as ::core::ffi::c_int,
        g_task_get_cancellable((*data).task),
        Some(
            safe_c2rust_replace_contents_open_callback
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        data as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_replace_contents_finish(
    mut file: *mut GFile,
    mut res: *mut GAsyncResult,
    mut new_etag: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut ReplaceContentsData = ::core::ptr::null_mut::<ReplaceContentsData>();
    if ({
        let mut _g_boolean_var_220: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_220 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_220 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_220
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_221: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, file as gpointer) != 0 {
            _g_boolean_var_221 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_221 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_221
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    task = res as *mut ::core::ffi::c_void as *mut GTask;
    if g_task_propagate_boolean(task, error) == 0 {
        return FALSE;
    }
    data = g_task_get_task_data(task) as *mut ReplaceContentsData;
    if !new_etag.is_null() {
        *new_etag = (*data).etag;
        (*data).etag = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_file_real_measure_disk_usage(
    mut file: *mut GFile,
    mut flags: GFileMeasureFlags,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileMeasureProgressCallback,
    mut progress_data: gpointer,
    mut disk_usage: *mut guint64,
    mut num_dirs: *mut guint64,
    mut num_files: *mut guint64,
    mut error: *mut *mut GError,
) -> gboolean {
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        b"Operation not supported for the current backend.\0" as *const u8 as *const gchar,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_measure_disk_usage_invoke_progress(
    mut user_data: gpointer,
) -> gboolean {
    let mut progress: *mut MeasureProgress = user_data as *mut MeasureProgress;
    Some((*progress).callback.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*progress).reporting,
        (*progress).current_size,
        (*progress).num_dirs,
        (*progress).num_files,
        (*progress).user_data,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_measure_disk_usage_progress(
    mut reporting: gboolean,
    mut current_size: guint64,
    mut num_dirs: guint64,
    mut num_files: guint64,
    mut user_data: gpointer,
) {
    let mut progress: MeasureProgress = MeasureProgress {
        callback: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        reporting: 0,
        current_size: 0,
        num_dirs: 0,
        num_files: 0,
    };
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut MeasureTaskData = ::core::ptr::null_mut::<MeasureTaskData>();
    data = g_task_get_task_data(task) as *mut MeasureTaskData;
    progress.callback = (*data).progress_callback;
    progress.user_data = (*data).progress_data;
    progress.reporting = reporting;
    progress.current_size = current_size;
    progress.num_dirs = num_dirs;
    progress.num_files = num_files;
    g_main_context_invoke_full(
        g_task_get_context(task),
        g_task_get_priority(task),
        Some(
            safe_c2rust_measure_disk_usage_invoke_progress
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        g_memdup2(
            &raw mut progress as gconstpointer,
            ::core::mem::size_of::<MeasureProgress>() as gsize,
        ),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_measure_disk_usage_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut MeasureTaskData = task_data as *mut MeasureTaskData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: MeasureResult = MeasureResult {
        disk_usage: 0 as guint64,
        num_dirs: 0,
        num_files: 0,
    };
    if safe_c2rust_g_file_measure_disk_usage(
        source_object as *mut GFile,
        (*data).flags,
        cancellable,
        if (*data).progress_callback.is_some() {
            Some(
                safe_c2rust_measure_disk_usage_progress
                    as unsafe extern "C" fn(gboolean, guint64, guint64, guint64, gpointer) -> (),
            )
        } else {
            None
        },
        task as gpointer,
        &raw mut result.disk_usage,
        &raw mut result.num_dirs,
        &raw mut result.num_files,
        &raw mut error,
    ) != 0
    {
        g_task_return_pointer(
            task,
            g_memdup2(
                &raw mut result as gconstpointer,
                ::core::mem::size_of::<MeasureResult>() as gsize,
            ),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
unsafe extern "C" fn safe_c2rust_g_file_real_measure_disk_usage_async(
    mut file: *mut GFile,
    mut flags: GFileMeasureFlags,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileMeasureProgressCallback,
    mut progress_data: gpointer,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut data: MeasureTaskData = MeasureTaskData {
        flags: G_FILE_MEASURE_NONE,
        progress_callback: None,
        progress_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    data.flags = flags;
    data.progress_callback = progress_callback;
    data.progress_data = progress_data;
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    GFileMeasureFlags,
                    gint,
                    *mut GCancellable,
                    GFileMeasureProgressCallback,
                    gpointer,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_real_measure_disk_usage_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    GFileMeasureFlags,
                    gint,
                    *mut GCancellable,
                    GFileMeasureProgressCallback,
                    gpointer,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_real_measure_disk_usage_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        g_memdup2(
            &raw mut data as gconstpointer,
            ::core::mem::size_of::<MeasureTaskData>() as gsize,
        ),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_priority(task, io_priority);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_measure_disk_usage_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_file_real_measure_disk_usage_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut disk_usage: *mut guint64,
    mut num_dirs: *mut guint64,
    mut num_files: *mut guint64,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut measure_result: *mut MeasureResult = ::core::ptr::null_mut::<MeasureResult>();
    if ({
        let mut _g_boolean_var_222: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, file as gpointer) != 0 {
            _g_boolean_var_222 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_222 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_222
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    measure_result =
        g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
            as *mut MeasureResult;
    if measure_result.is_null() {
        return FALSE;
    }
    if !disk_usage.is_null() {
        *disk_usage = (*measure_result).disk_usage;
    }
    if !num_dirs.is_null() {
        *num_dirs = (*measure_result).num_dirs;
    }
    if !num_files.is_null() {
        *num_files = (*measure_result).num_files;
    }
    g_free(measure_result as gpointer);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_measure_disk_usage(
    mut file: *mut GFile,
    mut flags: GFileMeasureFlags,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileMeasureProgressCallback,
    mut progress_data: gpointer,
    mut disk_usage: *mut guint64,
    mut num_dirs: *mut guint64,
    mut num_files: *mut guint64,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_223: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_223 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_223 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_223
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_224: ::core::ffi::c_int = 0;
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
            _g_boolean_var_224 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_224 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_224
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
        let mut _g_boolean_var_225: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_225 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_225 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_225
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
    return (*(g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface))
        .measure_disk_usage
        .expect("non-null function pointer")(
        file,
        flags,
        cancellable,
        progress_callback,
        progress_data,
        disk_usage,
        num_dirs,
        num_files,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_measure_disk_usage_async(
    mut file: *mut GFile,
    mut flags: GFileMeasureFlags,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileMeasureProgressCallback,
    mut progress_data: gpointer,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_226: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_226 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_226 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_226
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_227: ::core::ffi::c_int = 0;
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
            _g_boolean_var_227 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_227 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_227
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
    (*(g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface))
        .measure_disk_usage_async
        .expect("non-null function pointer")(
        file,
        flags,
        io_priority,
        cancellable,
        progress_callback,
        progress_data,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_measure_disk_usage_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut disk_usage: *mut guint64,
    mut num_dirs: *mut guint64,
    mut num_files: *mut guint64,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_228: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_228 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_228 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_228
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_229: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_229 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_229 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_229
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
    return (*(g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface))
        .measure_disk_usage_finish
        .expect("non-null function pointer")(
        file, result, disk_usage, num_dirs, num_files, error
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_start_mountable(
    mut file: *mut GFile,
    mut flags: GDriveStartFlags,
    mut start_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_230: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_230 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_230 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_230
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).start_mountable.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GDriveStartFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_start_mountable
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GDriveStartFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).start_mountable.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        flags,
        start_operation,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_start_mountable_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_231: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_231 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_231 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_231
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_232: ::core::ffi::c_int = 0;
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
            _g_boolean_var_232 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_232 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_232
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
                    *mut GFile,
                    GDriveStartFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_start_mountable
                as unsafe extern "C" fn(
                    *mut GFile,
                    GDriveStartFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .start_mountable_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_stop_mountable(
    mut file: *mut GFile,
    mut flags: GMountUnmountFlags,
    mut mount_operation: *mut GMountOperation,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_233: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_233 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_233 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_233
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).stop_mountable.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_stop_mountable
                    as unsafe extern "C" fn(
                        *mut GFile,
                        GMountUnmountFlags,
                        *mut GMountOperation,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).stop_mountable.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        file,
        flags,
        mount_operation,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_stop_mountable_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_234: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_234 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_234 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_234
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_235: ::core::ffi::c_int = 0;
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
            _g_boolean_var_235 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_235 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_235
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
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_stop_mountable
                as unsafe extern "C" fn(
                    *mut GFile,
                    GMountUnmountFlags,
                    *mut GMountOperation,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .stop_mountable_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_poll_mountable(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_236: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_236 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_236 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_236
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    if (*iface).poll_mountable.is_none() {
        g_task_report_new_error(
            file as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFile,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_file_poll_mountable
                    as unsafe extern "C" fn(
                        *mut GFile,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Some((*iface).poll_mountable.expect("non-null function pointer"))
        .expect("non-null function pointer")(file, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_poll_mountable_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_237: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_237 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_237 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_237
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_238: ::core::ffi::c_int = 0;
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
            _g_boolean_var_238 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_238 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_238
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
                    *mut GFile,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_poll_mountable
                as unsafe extern "C" fn(
                    *mut GFile,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return Some(
        (*iface)
            .poll_mountable_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(file, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_supports_thread_contexts(
    mut file: *mut GFile,
) -> gboolean {
    let mut iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    if ({
        let mut _g_boolean_var_239: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_239 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_239 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_239
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(file as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_file_get_type(),
    ) as *mut GFileIface;
    return (*iface).supports_thread_contexts;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_bytes(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut etag_out: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_240: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_240 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_240 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_240
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_241: ::core::ffi::c_int = 0;
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
            _g_boolean_var_241 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_241 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_241
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
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_242: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_242 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_242 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_242
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if !etag_out.is_null() {
        *etag_out = ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_g_file_has_uri_scheme(
        file,
        b"resource\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
        let mut uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut unescaped: *mut gchar = ::core::ptr::null_mut::<gchar>();
        uri = safe_c2rust_g_file_get_uri(file) as *mut gchar;
        unescaped = g_uri_unescape_string(
            uri.offset(
                strlen(b"resource://\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            ),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) as *mut gchar;
        g_free(uri as gpointer);
        bytes = g_resources_lookup_data(unescaped, G_RESOURCE_LOOKUP_FLAGS_NONE, error);
        g_free(unescaped as gpointer);
        return bytes;
    }
    if safe_c2rust_g_file_load_contents(
        file,
        cancellable,
        &raw mut contents,
        &raw mut len,
        etag_out as *mut *mut ::core::ffi::c_char,
        error,
    ) != 0
    {
        return g_bytes_new_take(
            safe_c2rust_g_steal_pointer(&raw mut contents as gpointer) as *mut gchar as gpointer,
            len,
        );
    }
    return ::core::ptr::null_mut::<GBytes>();
}
unsafe extern "C" fn safe_c2rust_g_file_load_bytes_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut file: *mut GFile = object as *mut ::core::ffi::c_void as *mut GFile;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut etag: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0 as gsize;
    safe_c2rust_g_file_load_contents_finish(
        file,
        result,
        &raw mut contents,
        &raw mut len,
        &raw mut etag,
        &raw mut error,
    );
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut etag as gpointer) as *mut gchar as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    if !error.is_null() {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    } else {
        g_task_return_pointer(
            task,
            g_bytes_new_take(
                safe_c2rust_g_steal_pointer(&raw mut contents as gpointer) as *mut gchar
                    as gpointer,
                len,
            ) as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GBytes) -> ()>, GDestroyNotify>(
                Some(g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> ()),
            ),
        );
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_bytes_async(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_243: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_243 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_243 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_243
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_244: ::core::ffi::c_int = 0;
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
            _g_boolean_var_244 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_244 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_244
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
    task = g_task_new(file as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GFile,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_file_load_bytes_async
                as unsafe extern "C" fn(
                    *mut GFile,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_file_load_bytes_async\0" as *const u8 as *const gchar,
        );
    }
    if safe_c2rust_g_file_has_uri_scheme(
        file,
        b"resource\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        safe_c2rust_g_file_load_contents_async(
            file,
            cancellable,
            Some(
                safe_c2rust_g_file_load_bytes_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
        return;
    }
    bytes = safe_c2rust_g_file_load_bytes(
        file,
        cancellable,
        ::core::ptr::null_mut::<*mut gchar>(),
        &raw mut error,
    );
    if bytes.is_null() {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    } else {
        g_task_return_pointer(
            task,
            safe_c2rust_g_steal_pointer(&raw mut bytes as gpointer) as *mut GBytes as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GBytes) -> ()>, GDestroyNotify>(
                Some(g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> ()),
            ),
        );
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_load_bytes_finish(
    mut file: *mut GFile,
    mut result: *mut GAsyncResult,
    mut etag_out: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if ({
        let mut _g_boolean_var_245: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = file as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_get_type();
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
            _g_boolean_var_245 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_245 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_245
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_246: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_task_get_type();
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
            _g_boolean_var_246 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_246 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_246
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_TASK (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_247: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            result as *mut ::core::ffi::c_void as *mut GTask as gpointer,
            file as gpointer,
        ) != 0
        {
            _g_boolean_var_247 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_247 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_247
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (G_TASK (result), file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_248: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_248 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_248 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_248
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    bytes = g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GBytes;
    if !etag_out.is_null() {
        *etag_out = safe_c2rust_g_strdup_inline(g_task_get_task_data(
            result as *mut ::core::ffi::c_void as *mut GTask,
        ) as *const ::core::ffi::c_char) as *mut gchar;
    }
    return bytes;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
