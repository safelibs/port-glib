use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GKeyFile;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAppLaunchContextPrivate;
    pub type _GAppInfo;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GIcon;
    pub type _GTask;
    pub type _GDBusMessage;
    pub type _GDBusConnection;
    pub type _GWakeup;
    pub type _GDesktopAppInfoLookup;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn getpid() -> __pid_t;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_steal(array: *mut GPtrArray, len: *mut gsize) -> *mut gpointer;
    fn g_ptr_array_new_full(
        reserved_size: guint,
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_set_size(array: *mut GPtrArray, length: gint);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_sort_values(array: *mut GPtrArray, compare_func: GCompareFunc);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_get_prgname() -> *const gchar;
    fn g_get_user_data_dir() -> *const gchar;
    fn g_get_user_config_dir() -> *const gchar;
    fn g_get_system_data_dirs() -> *const *const gchar;
    fn g_get_system_config_dirs() -> *const *const gchar;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_filename_display_name(filename: *const gchar) -> *mut gchar;
    fn g_dir_open(path: *const gchar, flags: guint, error: *mut *mut GError) -> *mut GDir;
    fn g_dir_read_name(dir: *mut GDir) -> *const gchar;
    fn g_dir_close(dir: *mut GDir);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_get_environ() -> *mut *mut gchar;
    fn g_environ_getenv(envp: *mut *mut gchar, variable: *const gchar) -> *const gchar;
    fn g_environ_setenv(
        envp: *mut *mut gchar,
        variable: *const gchar,
        value: *const gchar,
        overwrite: gboolean,
    ) -> *mut *mut gchar;
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_file_set_contents_full(
        filename: *const gchar,
        contents: *const gchar,
        length: gssize,
        flags: GFileSetContentsFlags,
        mode: ::core::ffi::c_int,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_mkstemp(tmpl: *mut gchar) -> gint;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_mkdir_with_parents(pathname: *const gchar, mode: gint) -> gint;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_list_copy_deep(list: *mut GList, func: GCopyFunc, user_data: gpointer) -> *mut GList;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_child_watch_add(pid: GPid, function: GChildWatchFunc, data: gpointer) -> guint;
    fn g_timeout_add_seconds_full(
        priority: gint,
        interval: guint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    ) -> guint;
    fn g_source_remove(tag: guint) -> gboolean;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_tolower(c: gchar) -> gchar;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_ascii_strdown(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_str_tokenize_and_fold(
        string: *const gchar,
        translit_locale: *const gchar,
        ascii_alternates: *mut *mut *mut gchar,
    ) -> *mut *mut gchar;
    fn g_strv_contains(strv: *const *const gchar, str: *const gchar) -> gboolean;
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
    fn g_key_file_new() -> *mut GKeyFile;
    fn g_key_file_ref(key_file: *mut GKeyFile) -> *mut GKeyFile;
    fn g_key_file_unref(key_file: *mut GKeyFile);
    fn g_key_file_free(key_file: *mut GKeyFile);
    fn g_key_file_load_from_file(
        key_file: *mut GKeyFile,
        file: *const gchar,
        flags: GKeyFileFlags,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_key_file_to_data(
        key_file: *mut GKeyFile,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_key_file_get_start_group(key_file: *mut GKeyFile) -> *mut gchar;
    fn g_key_file_get_keys(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut *mut gchar;
    fn g_key_file_has_group(key_file: *mut GKeyFile, group_name: *const gchar) -> gboolean;
    fn g_key_file_has_key(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_key_file_get_string(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_key_file_set_string(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        string: *const gchar,
    );
    fn g_key_file_get_locale_string(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        locale: *const gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_key_file_get_boolean(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_key_file_set_boolean(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        value: gboolean,
    );
    fn g_key_file_get_string_list(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut *mut gchar;
    fn g_key_file_set_string_list(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        list: *const *const gchar,
        length: gsize,
    );
    fn g_key_file_get_locale_string_list(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        locale: *const gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut *mut gchar;
    fn g_key_file_remove_key(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_new_int32(value: gint32) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_take_string(string: *mut gchar) -> *mut GVariant;
    fn g_variant_new_bytestring(string: *const gchar) -> *mut GVariant;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_open(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_dict_init(dict: *mut GVariantDict, from_asv: *mut GVariant);
    fn g_variant_dict_lookup(
        dict: *mut GVariantDict,
        key: *const gchar,
        format_string: *const gchar,
        ...
    ) -> gboolean;
    fn g_variant_dict_clear(dict: *mut GVariantDict);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
    fn g_shell_quote(unquoted_string: *const gchar) -> *mut gchar;
    fn g_shell_parse_argv(
        command_line: *const gchar,
        argcp: *mut gint,
        argvp: *mut *mut *mut gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_spawn_error_quark() -> GQuark;
    fn g_spawn_async(
        working_directory: *const gchar,
        argv: *mut *mut gchar,
        envp: *mut *mut gchar,
        flags: GSpawnFlags,
        child_setup: GSpawnChildSetupFunc,
        user_data: gpointer,
        child_pid: *mut GPid,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_spawn_async_with_fds(
        working_directory: *const gchar,
        argv: *mut *mut gchar,
        envp: *mut *mut gchar,
        flags: GSpawnFlags,
        child_setup: GSpawnChildSetupFunc,
        user_data: gpointer,
        child_pid: *mut GPid,
        stdin_fd: gint,
        stdout_fd: gint,
        stderr_fd: gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_spawn_check_wait_status(wait_status: gint, error: *mut *mut GError) -> gboolean;
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
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_signal_handlers_disconnect_matched(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> guint;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn _g_unix_content_type_unalias(type_0: *const ::core::ffi::c_char)
        -> *mut ::core::ffi::c_char;
    fn _g_unix_content_type_get_parents(
        type_0: *const ::core::ffi::c_char,
    ) -> *mut *mut ::core::ffi::c_char;
    fn g_app_info_get_type() -> GType;
    fn g_app_info_supports_uris(appinfo: *mut GAppInfo) -> gboolean;
    fn g_app_launch_context_get_environment(
        context: *mut GAppLaunchContext,
    ) -> *mut *mut ::core::ffi::c_char;
    fn g_app_launch_context_get_startup_notify_id(
        context: *mut GAppLaunchContext,
        info: *mut GAppInfo,
        files: *mut GList,
    ) -> *mut ::core::ffi::c_char;
    fn g_app_launch_context_launch_failed(
        context: *mut GAppLaunchContext,
        startup_notify_id: *const ::core::ffi::c_char,
    );
    fn g_bus_get(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_bus_get_finish(res: *mut GAsyncResult, error: *mut *mut GError) -> *mut GDBusConnection;
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_flush(
        connection: *mut GDBusConnection,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_flush_finish(
        connection: *mut GDBusConnection,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_connection_send_message(
        connection: *mut GDBusConnection,
        message: *mut GDBusMessage,
        flags: GDBusSendMessageFlags,
        out_serial: *mut guint32,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_connection_call(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_finish(
        connection: *mut GDBusConnection,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_error_strip_remote_error(error: *mut GError) -> gboolean;
    fn g_dbus_message_new_signal(
        path: *const gchar,
        interface_: *const gchar,
        signal: *const gchar,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_set_body(message: *mut GDBusMessage, body: *mut GVariant);
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_file_new_for_path(path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_new_for_uri(uri: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_get_path(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_file_get_uri(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_file_icon_new(file: *mut GFile) -> *mut GIcon;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_file_monitor_cancel(monitor: *mut GFileMonitor) -> gboolean;
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
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_task_had_error(task: *mut GTask) -> gboolean;
    fn g_themed_icon_new(iconname: *const ::core::ffi::c_char) -> *mut GIcon;
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn g_access(filename: *const gchar, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_app_info_monitor_fire();
    fn g_local_file_monitor_new_in_worker(
        pathname: *const gchar,
        is_directory: gboolean,
        flags: GFileMonitorFlags,
        callback: GFileMonitorCallback,
        user_data: gpointer,
        destroy_user_data: GClosureNotify,
        error: *mut *mut GError,
    ) -> *mut GFileMonitor;
    fn g_document_portal_add_documents(
        uris: *mut GList,
        app_id: *const ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> *mut GList;
}
pub type size_t = usize;
pub type __pid_t = ::core::ffi::c_int;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
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
pub type guchar = ::core::ffi::c_uchar;
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
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type gatomicrefcount = gint;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
pub type GDir = _GDir;
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
pub type GFileSetContentsFlags = ::core::ffi::c_uint;
pub const G_FILE_SET_CONTENTS_ONLY_EXISTING: GFileSetContentsFlags = 4;
pub const G_FILE_SET_CONTENTS_DURABLE: GFileSetContentsFlags = 2;
pub const G_FILE_SET_CONTENTS_CONSISTENT: GFileSetContentsFlags = 1;
pub const G_FILE_SET_CONTENTS_NONE: GFileSetContentsFlags = 0;
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
pub type GChildWatchFunc = Option<unsafe extern "C" fn(GPid, gint, gpointer) -> ()>;
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
pub type GKeyFile = _GKeyFile;
pub type GKeyFileFlags = ::core::ffi::c_uint;
pub const G_KEY_FILE_KEEP_TRANSLATIONS: GKeyFileFlags = 2;
pub const G_KEY_FILE_KEEP_COMMENTS: GKeyFileFlags = 1;
pub const G_KEY_FILE_NONE: GKeyFileFlags = 0;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: C2RustUnnamed_1,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantDict {
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub s: C2RustUnnamed_3,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub asv: *mut GVariant,
    pub partial_magic: gsize,
    pub y: [guintptr; 14],
}
pub type GVariantDict = _GVariantDict;
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
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const G_SPAWN_ERROR_FAILED: C2RustUnnamed_4 = 19;
pub const G_SPAWN_ERROR_LIBBAD: C2RustUnnamed_4 = 18;
pub const G_SPAWN_ERROR_ISDIR: C2RustUnnamed_4 = 17;
pub const G_SPAWN_ERROR_INVAL: C2RustUnnamed_4 = 16;
pub const G_SPAWN_ERROR_MFILE: C2RustUnnamed_4 = 15;
pub const G_SPAWN_ERROR_NFILE: C2RustUnnamed_4 = 14;
pub const G_SPAWN_ERROR_IO: C2RustUnnamed_4 = 13;
pub const G_SPAWN_ERROR_TXTBUSY: C2RustUnnamed_4 = 12;
pub const G_SPAWN_ERROR_LOOP: C2RustUnnamed_4 = 11;
pub const G_SPAWN_ERROR_NOTDIR: C2RustUnnamed_4 = 10;
pub const G_SPAWN_ERROR_NOMEM: C2RustUnnamed_4 = 9;
pub const G_SPAWN_ERROR_NOENT: C2RustUnnamed_4 = 8;
pub const G_SPAWN_ERROR_NAMETOOLONG: C2RustUnnamed_4 = 7;
pub const G_SPAWN_ERROR_NOEXEC: C2RustUnnamed_4 = 6;
pub const G_SPAWN_ERROR_2BIG: C2RustUnnamed_4 = 5;
pub const G_SPAWN_ERROR_TOO_BIG: C2RustUnnamed_4 = 5;
pub const G_SPAWN_ERROR_PERM: C2RustUnnamed_4 = 4;
pub const G_SPAWN_ERROR_ACCES: C2RustUnnamed_4 = 3;
pub const G_SPAWN_ERROR_CHDIR: C2RustUnnamed_4 = 2;
pub const G_SPAWN_ERROR_READ: C2RustUnnamed_4 = 1;
pub const G_SPAWN_ERROR_FORK: C2RustUnnamed_4 = 0;
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
    pub data: [C2RustUnnamed_5; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
pub type GSignalMatchType = ::core::ffi::c_uint;
pub const G_SIGNAL_MATCH_UNBLOCKED: GSignalMatchType = 32;
pub const G_SIGNAL_MATCH_DATA: GSignalMatchType = 16;
pub const G_SIGNAL_MATCH_FUNC: GSignalMatchType = 8;
pub const G_SIGNAL_MATCH_CLOSURE: GSignalMatchType = 4;
pub const G_SIGNAL_MATCH_DETAIL: GSignalMatchType = 2;
pub const G_SIGNAL_MATCH_ID: GSignalMatchType = 1;
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
pub type GAppInfoCreateFlags = ::core::ffi::c_uint;
pub const G_APP_INFO_CREATE_SUPPORTS_STARTUP_NOTIFICATION: GAppInfoCreateFlags = 4;
pub const G_APP_INFO_CREATE_SUPPORTS_URIS: GAppInfoCreateFlags = 2;
pub const G_APP_INFO_CREATE_NEEDS_TERMINAL: GAppInfoCreateFlags = 1;
pub const G_APP_INFO_CREATE_NONE: GAppInfoCreateFlags = 0;
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
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusSendMessageFlags = ::core::ffi::c_uint;
pub const G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL: GDBusSendMessageFlags = 1;
pub const G_DBUS_SEND_MESSAGE_FLAGS_NONE: GDBusSendMessageFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppLaunchContext {
    pub parent_instance: GObject,
    pub priv_0: *mut GAppLaunchContextPrivate,
}
pub type GAppLaunchContextPrivate = _GAppLaunchContextPrivate;
pub type GAppLaunchContext = _GAppLaunchContext;
pub type GAppInfo = _GAppInfo;
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
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
pub type GFile = _GFile;
pub type GIcon = _GIcon;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusMessage = _GDBusMessage;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppInfoIface {
    pub g_iface: GTypeInterface,
    pub dup: Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut GAppInfo>,
    pub equal: Option<unsafe extern "C" fn(*mut GAppInfo, *mut GAppInfo) -> gboolean>,
    pub get_id: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_name: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_description: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_executable: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_icon: Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut GIcon>,
    pub launch: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *mut GList,
            *mut GAppLaunchContext,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub supports_uris: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub supports_files: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub launch_uris: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *mut GList,
            *mut GAppLaunchContext,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub should_show: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub set_as_default_for_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_as_default_for_extension: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub add_supports_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_remove_supports_type: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub remove_supports_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_delete: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub do_delete: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub get_commandline: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_display_name: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub set_as_last_used_for_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub get_supported_types:
        Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut *const ::core::ffi::c_char>,
    pub launch_uris_async: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *mut GList,
            *mut GAppLaunchContext,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub launch_uris_finish: Option<
        unsafe extern "C" fn(*mut GAppInfo, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
}
pub type GAppInfoIface = _GAppInfoIface;
pub type GDesktopAppInfo = _GDesktopAppInfo;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GDesktopAppInfo {
    pub parent_instance: GObject,
    pub desktop_id: *mut ::core::ffi::c_char,
    pub filename: *mut ::core::ffi::c_char,
    pub app_id: *mut ::core::ffi::c_char,
    pub keyfile: *mut GKeyFile,
    pub name: *mut ::core::ffi::c_char,
    pub generic_name: *mut ::core::ffi::c_char,
    pub fullname: *mut ::core::ffi::c_char,
    pub comment: *mut ::core::ffi::c_char,
    pub icon_name: *mut ::core::ffi::c_char,
    pub icon: *mut GIcon,
    pub keywords: *mut *mut ::core::ffi::c_char,
    pub only_show_in: *mut *mut ::core::ffi::c_char,
    pub not_show_in: *mut *mut ::core::ffi::c_char,
    pub try_exec: *mut ::core::ffi::c_char,
    pub exec: *mut ::core::ffi::c_char,
    pub binary: *mut ::core::ffi::c_char,
    pub path: *mut ::core::ffi::c_char,
    pub categories: *mut ::core::ffi::c_char,
    pub startup_wm_class: *mut ::core::ffi::c_char,
    pub mime_types: *mut *mut ::core::ffi::c_char,
    pub actions: *mut *mut ::core::ffi::c_char,
    #[bitfield(name = "nodisplay", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "hidden", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "terminal", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "startup_notify", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "no_fuse", ty = "guint", bits = "4..=4")]
    pub nodisplay_hidden_terminal_startup_notify_no_fuse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GDesktopAppInfoClass = _GDesktopAppInfoClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDesktopAppInfoClass {
    pub parent_class: GObjectClass,
}
pub const PROP_FILENAME: C2RustUnnamed_8 = 1;
pub type UpdateMimeFlags = ::core::ffi::c_uint;
pub const UPDATE_MIME_SET_LAST_USED: UpdateMimeFlags = 16;
pub const UPDATE_MIME_REMOVE: UpdateMimeFlags = 8;
pub const UPDATE_MIME_SET_NON_DEFAULT: UpdateMimeFlags = 4;
pub const UPDATE_MIME_SET_DEFAULT: UpdateMimeFlags = 2;
pub const UPDATE_MIME_NONE: UpdateMimeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DesktopFileDir {
    pub ref_count: gatomicrefcount,
    pub path: *mut gchar,
    pub alternatively_watching: *mut gchar,
    pub is_config: gboolean,
    pub is_setup: gboolean,
    pub monitor: *mut GFileMonitor,
    pub poll_source_id: guint,
    pub poll_snapshot: *mut gchar,
    pub app_names: *mut GHashTable,
    pub mime_tweaks: *mut GHashTable,
    pub memory_index: *mut GHashTable,
    pub memory_implementations: *mut GHashTable,
}
pub type DirType = ::core::ffi::c_uint;
pub const MIMETYPE_DIR: DirType = 2;
pub const APP_DIR: DirType = 1;
pub const CONF_DIR: DirType = 0;
pub type GDesktopAppLaunchCallback =
    Option<unsafe extern "C" fn(*mut GDesktopAppInfo, GPid, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LaunchUrisData {
    pub uris: *mut GList,
    pub context: *mut GAppLaunchContext,
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
pub struct C2RustUnnamed_6 {
    pub exec: *const ::core::ffi::c_char,
    pub exec_arg: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LaunchUrisWithDBusData {
    pub info: *mut GDesktopAppInfo,
    pub launch_context: *mut GAppLaunchContext,
    pub callback: GAsyncReadyCallback,
    pub startup_id: *mut gchar,
    pub user_data: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub in_0: *mut ::core::ffi::c_char,
    pub out: *mut *mut GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnindexedMimeTweaks {
    pub additions: *mut *mut gchar,
    pub removals: *mut *mut gchar,
    pub defaults: *mut *mut gchar,
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
pub type GDesktopAppInfoLookup = _GDesktopAppInfoLookup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDesktopAppInfoLookupIface {
    pub g_iface: GTypeInterface,
    pub get_default_for_uri_scheme: Option<
        unsafe extern "C" fn(
            *mut GDesktopAppInfoLookup,
            *const ::core::ffi::c_char,
        ) -> *mut GAppInfo,
    >,
}
pub type GDesktopAppInfoLookupIface = _GDesktopAppInfoLookupIface;
pub type GDesktopAppInfoLookupInterface = GDesktopAppInfoLookupIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_result {
    pub app_name: *const gchar,
    pub category: gint,
    pub match_type: gint,
}
pub type MemoryIndexEntry = _MemoryIndexEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MemoryIndexEntry {
    pub app_name: *const gchar,
    pub match_category: gint,
    pub next: *mut MemoryIndexEntry,
}
pub type MatchType = ::core::ffi::c_uint;
pub const MATCH_TYPE_SUBSTRING: MatchType = 2;
pub const MATCH_TYPE_PREFIX: MatchType = 1;
pub type MemoryIndex = GHashTable;
pub const DESKTOP_KEY_Exec: C2RustUnnamed_9 = 0;
pub const DESKTOP_KEY_X_GNOME_FullName: C2RustUnnamed_9 = 4;
pub const DESKTOP_KEY_Name: C2RustUnnamed_9 = 3;
pub const DESKTOP_KEY_Keywords: C2RustUnnamed_9 = 2;
pub const DESKTOP_KEY_GenericName: C2RustUnnamed_9 = 1;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const N_DESKTOP_KEYS: C2RustUnnamed_9 = 5;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_SEARCHPATH_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b":\0") };
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
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
pub const G_KEY_FILE_DESKTOP_GROUP: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"Desktop Entry\0") };
pub const G_KEY_FILE_DESKTOP_KEY_TYPE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Type\0") };
pub const G_KEY_FILE_DESKTOP_KEY_VERSION: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Version\0") };
pub const G_KEY_FILE_DESKTOP_KEY_NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Name\0") };
pub const G_KEY_FILE_DESKTOP_KEY_NO_DISPLAY: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"NoDisplay\0") };
pub const G_KEY_FILE_DESKTOP_KEY_COMMENT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Comment\0") };
pub const G_KEY_FILE_DESKTOP_KEY_ICON: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Icon\0") };
pub const G_KEY_FILE_DESKTOP_KEY_HIDDEN: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"Hidden\0") };
pub const G_KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"OnlyShowIn\0") };
pub const G_KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"NotShowIn\0") };
pub const G_KEY_FILE_DESKTOP_KEY_TRY_EXEC: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"TryExec\0") };
pub const G_KEY_FILE_DESKTOP_KEY_EXEC: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Exec\0") };
pub const G_KEY_FILE_DESKTOP_KEY_PATH: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Path\0") };
pub const G_KEY_FILE_DESKTOP_KEY_TERMINAL: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Terminal\0") };
pub const G_KEY_FILE_DESKTOP_KEY_MIME_TYPE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"MimeType\0") };
pub const G_KEY_FILE_DESKTOP_KEY_CATEGORIES: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"Categories\0") };
pub const G_KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"StartupNotify\0") };
pub const G_KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"DBusActivatable\0") };
pub const G_KEY_FILE_DESKTOP_KEY_ACTIONS: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Actions\0") };
pub const G_KEY_FILE_DESKTOP_TYPE_APPLICATION: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Application\0") };
pub const G_VARIANT_TYPE_ARRAY: *const GVariantType =
    b"a*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_TUPLE: *const GVariantType =
    b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING_ARRAY: *const GVariantType =
    b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_VARDICT: *const GVariantType =
    b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[inline]
unsafe extern "C" fn safe_c2rust_g_set_object(
    mut object_ptr: *mut *mut GObject,
    mut new_object: *mut GObject,
) -> gboolean {
    let mut old_object: *mut GObject = *object_ptr;
    if old_object == new_object {
        return FALSE;
    }
    if !new_object.is_null() {
        g_object_ref(new_object as gpointer);
    }
    *object_ptr = new_object;
    if !old_object.is_null() {
        g_object_unref(old_object as gpointer);
    }
    return TRUE;
}
pub const DEFAULT_APPLICATIONS_GROUP: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"Default Applications\0")
};
pub const ADDED_ASSOCIATIONS_GROUP: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"Added Associations\0")
};
pub const REMOVED_ASSOCIATIONS_GROUP: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"Removed Associations\0")
};
pub const MIME_CACHE_GROUP: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"MIME Cache\0") };
pub const GENERIC_NAME_KEY: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"GenericName\0") };
pub const FULL_NAME_KEY: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"X-GNOME-FullName\0")
};
pub const KEYWORDS_KEY: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Keywords\0") };
pub const STARTUP_WM_CLASS_KEY: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"StartupWMClass\0") };
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_desktop_app_info_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDesktopAppInfo_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDesktopAppInfo_private_offset,
        );
    }
    safe_c2rust_g_desktop_app_info_class_init(klass as *mut GDesktopAppInfoClass);
}
static mut safe_c2rust_g_desktop_app_info_parent_class: gpointer = NULL_1;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDesktopAppInfo\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDesktopAppInfoClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_desktop_app_info_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDesktopAppInfo>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDesktopAppInfo) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_desktop_app_info_init
                    as unsafe extern "C" fn(*mut GDesktopAppInfo) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GAppInfoIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_desktop_app_info_iface_init
                as unsafe extern "C" fn(*mut GAppInfoIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_app_info_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDesktopAppInfo_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_desktop_app_info_get_type_once();
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
static mut safe_c2rust_desktop_file_dirs: *mut GPtrArray =
    ::core::ptr::null::<GPtrArray>() as *mut GPtrArray;
static mut safe_c2rust_desktop_file_dirs_config_dir: *const gchar = ::core::ptr::null::<gchar>();
static mut safe_c2rust_desktop_file_dir_user_config: *mut DesktopFileDir =
    ::core::ptr::null::<DesktopFileDir>() as *mut DesktopFileDir;
static mut safe_c2rust_desktop_file_dir_user_data: *mut DesktopFileDir =
    ::core::ptr::null::<DesktopFileDir>() as *mut DesktopFileDir;
static mut safe_c2rust_desktop_file_dir_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_gio_launch_desktop_path: *const gchar = ::core::ptr::null::<gchar>();
unsafe extern "C" fn safe_c2rust_desktop_file_dir_ref(
    mut dir: *mut DesktopFileDir,
) -> *mut DesktopFileDir {
    g_atomic_ref_count_inc(&raw mut (*dir).ref_count);
    return dir;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unref(mut dir: *mut DesktopFileDir) {
    if g_atomic_ref_count_dec(&raw mut (*dir).ref_count) != 0 {
        safe_c2rust_desktop_file_dir_reset(dir);
        g_free((*dir).path as gpointer);
        g_free(dir as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_get_alternative_dir(
    mut dir: *mut DesktopFileDir,
) -> *mut gchar {
    let mut parent: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if g_access((*dir).path, R_OK | X_OK) == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<gchar>();
    }
    parent = g_path_get_dirname((*dir).path);
    while g_access(parent, R_OK | X_OK) != 0 as ::core::ffi::c_int {
        let mut tmp: *mut gchar = parent;
        parent = g_path_get_dirname(tmp);
        if strcmp(
            parent as *const ::core::ffi::c_char,
            tmp as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            g_free(tmp as gpointer);
            break;
        } else {
            g_free(tmp as gpointer);
        }
    }
    return parent;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_changed(
    mut monitor: *mut GFileMonitor,
    mut file: *mut GFile,
    mut other_file: *mut GFile,
    mut event_type: GFileMonitorEvent,
    mut user_data: gpointer,
) {
    let mut dir: *mut DesktopFileDir = user_data as *mut DesktopFileDir;
    let mut do_nothing: gboolean = FALSE;
    g_mutex_lock(&raw mut safe_c2rust_desktop_file_dir_lock);
    if !(*dir).alternatively_watching.is_null() {
        let mut alternative_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
        alternative_dir = safe_c2rust_desktop_file_dir_get_alternative_dir(dir);
        do_nothing = (!alternative_dir.is_null()
            && strcmp(
                (*dir).alternatively_watching as *const ::core::ffi::c_char,
                alternative_dir as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        g_free(alternative_dir as gpointer);
    }
    if do_nothing == 0 {
        safe_c2rust_desktop_file_dir_reset(dir);
    }
    g_mutex_unlock(&raw mut safe_c2rust_desktop_file_dir_lock);
    if do_nothing == 0 {
        g_app_info_monitor_fire();
    }
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_poll_snapshot(
    mut dir: *mut DesktopFileDir,
) -> *mut gchar {
    let mut watch_dir: *const gchar = if !(*dir).alternatively_watching.is_null() {
        (*dir).alternatively_watching
    } else {
        (*dir).path
    };
    let mut snapshot: *mut GString = g_string_new(::core::ptr::null::<gchar>());
    let mut entries: *mut GPtrArray =
        g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    let mut handle: *mut GDir =
        g_dir_open(watch_dir, 0 as guint, ::core::ptr::null_mut::<*mut GError>());
    if !handle.is_null() {
        loop {
            let mut name: *const gchar = g_dir_read_name(handle);
            if name.is_null() {
                break;
            }
            g_ptr_array_add(
                entries,
                safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as gpointer,
            );
        }
        g_dir_close(handle);
    }
    g_ptr_array_sort_values(
        entries,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >,
            GCompareFunc,
        >(Some(
            g_strcmp0
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )),
    );
    let mut i: guint = 0 as guint;
    while i < (*entries).len {
        let mut name: *const gchar = *(*entries).pdata.offset(i as isize) as *const gchar;
        g_string_append_len(snapshot, name, strlen(name as *const ::core::ffi::c_char) as gssize);
        g_string_insert_c(snapshot, -(1 as ::core::ffi::c_int) as gssize, '\n' as i32 as gchar);
        if g_str_has_suffix(
            name,
            b".desktop\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            let mut filename: *mut gchar = g_build_filename(watch_dir, name, NULL_1);
            let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut length: gsize = 0 as gsize;
            if g_file_get_contents(
                filename,
                &raw mut contents,
                &raw mut length,
                ::core::ptr::null_mut::<*mut GError>(),
            ) != 0
            {
                g_string_append_len(snapshot, contents, length as gssize);
                g_free(contents as gpointer);
            }
            g_free(filename as gpointer);
            g_string_insert_c(
                snapshot,
                -(1 as ::core::ffi::c_int) as gssize,
                '\n' as i32 as gchar,
            );
        }
        i = i.wrapping_add(1);
    }
    g_ptr_array_unref(entries);
    return g_string_free(snapshot, FALSE) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_poll_unref(mut data: gpointer) {
    safe_c2rust_desktop_file_dir_unref(data as *mut DesktopFileDir);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_poll(mut data: gpointer) -> gboolean {
    let mut dir: *mut DesktopFileDir = data as *mut DesktopFileDir;
    let mut snapshot: *mut gchar = safe_c2rust_desktop_file_dir_poll_snapshot(dir);
    let mut changed: gboolean = (g_strcmp0(
        snapshot as *const ::core::ffi::c_char,
        (*dir).poll_snapshot as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int) as gboolean;
    g_free(snapshot as gpointer);
    if changed != 0 {
        g_mutex_lock(&raw mut safe_c2rust_desktop_file_dir_lock);
        (*dir).poll_source_id = 0 as guint;
        safe_c2rust_desktop_file_dir_reset(dir);
        g_mutex_unlock(&raw mut safe_c2rust_desktop_file_dir_lock);
        g_app_info_monitor_fire();
        return FALSE as gboolean;
    }
    return TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_app_name_is_masked(
    mut dir: *mut DesktopFileDir,
    mut app_name: *const gchar,
) -> gboolean {
    let mut i: guint = 0;
    i = 0 as guint;
    while i < (*safe_c2rust_desktop_file_dirs).len {
        let mut i_dir: *mut DesktopFileDir =
            *(*safe_c2rust_desktop_file_dirs).pdata.offset(i as isize) as *mut DesktopFileDir;
        if dir == i_dir {
            return FALSE;
        }
        if !(*i_dir).app_names.is_null()
            && g_hash_table_contains((*i_dir).app_names, app_name as gconstpointer) != 0
        {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_validate_xdg_desktop(mut desktop: *const gchar) -> gboolean {
    let mut i: gsize = 0;
    i = 0 as gsize;
    while *desktop.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
        if *desktop.offset(i as isize) as ::core::ffi::c_int != '-' as i32
            && *desktop.offset(i as isize) as ::core::ffi::c_int != '_' as i32
            && !(*safe_c2rust_g_ascii_table.offset(*desktop.offset(i as isize) as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_ALNUM as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
        {
            return FALSE;
        }
        i = i.wrapping_add(1);
    }
    if i == 0 as gsize {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_valid_current_desktops(
    mut value: *const ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut tmp: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut i: gsize = 0;
    let mut valid_desktops: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if value.is_null() {
        value = g_getenv(b"XDG_CURRENT_DESKTOP\0" as *const u8 as *const gchar)
            as *const ::core::ffi::c_char;
    }
    if value.is_null() {
        value = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    tmp = g_strsplit(
        value as *const gchar,
        G_SEARCHPATH_SEPARATOR_S.as_ptr() as *const gchar,
        0 as gint,
    ) as *mut *mut ::core::ffi::c_char;
    valid_desktops = g_ptr_array_new_full(
        g_strv_length(tmp as *mut *mut gchar).wrapping_add(1 as guint),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    i = 0 as gsize;
    while !(*tmp.offset(i as isize)).is_null() {
        if safe_c2rust_validate_xdg_desktop(*tmp.offset(i as isize)) != 0 {
            g_ptr_array_add(valid_desktops, *tmp.offset(i as isize) as gpointer);
        } else {
            g_free(*tmp.offset(i as isize) as gpointer);
        }
        i = i.wrapping_add(1);
    }
    g_ptr_array_add(valid_desktops, NULL_1);
    g_free(tmp as gpointer);
    tmp = g_ptr_array_steal(valid_desktops, ::core::ptr::null_mut::<gsize>())
        as *mut *mut ::core::ffi::c_char;
    g_ptr_array_unref(valid_desktops);
    return tmp;
}
unsafe extern "C" fn safe_c2rust_get_lowercase_current_desktops() -> *const *const gchar {
    static mut safe_c2rust_result: *mut *mut gchar =
        ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_result;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            let mut gapg_temp_atomic: *mut *mut *mut gchar = &raw mut safe_c2rust_result;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut safe_c2rust_result as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut tmp: *mut *mut ::core::ffi::c_char =
            safe_c2rust_get_valid_current_desktops(::core::ptr::null::<::core::ffi::c_char>());
        let mut i: gsize = 0;
        let mut j: gsize = 0;
        i = 0 as gsize;
        while !(*tmp.offset(i as isize)).is_null() {
            j = 0 as gsize;
            while *(*tmp.offset(i as isize)).offset(j as isize) != 0 {
                *(*tmp.offset(i as isize)).offset(j as isize) =
                    g_ascii_tolower(*(*tmp.offset(i as isize)).offset(j as isize) as gchar)
                        as ::core::ffi::c_char;
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_result = tmp as *mut *mut gchar;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_result as *mut ::core::ffi::c_void,
            tmp as guintptr as gpointer,
        );
    }
    return safe_c2rust_result as *mut *const gchar;
}
unsafe extern "C" fn safe_c2rust_get_current_desktops(
    mut value: *const gchar,
) -> *const *const gchar {
    static mut safe_c2rust_result: *mut *mut gchar =
        ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_result;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            let mut gapg_temp_atomic: *mut *mut *mut gchar = &raw mut safe_c2rust_result;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut safe_c2rust_result as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut tmp: *mut *mut ::core::ffi::c_char =
            safe_c2rust_get_valid_current_desktops(value as *const ::core::ffi::c_char);
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_result = tmp as *mut *mut gchar;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_result as *mut ::core::ffi::c_void,
            tmp as guintptr as gpointer,
        );
    }
    return safe_c2rust_result as *mut *const gchar;
}
unsafe extern "C" fn safe_c2rust_add_to_table_if_appropriate(
    mut apps: *mut GHashTable,
    mut app_name: *const gchar,
    mut info: *mut GDesktopAppInfo,
) {
    if info.is_null() {
        return;
    }
    if (*info).hidden() != 0 {
        g_object_unref(info as gpointer);
        return;
    }
    g_free((*info).desktop_id as gpointer);
    (*info).desktop_id = safe_c2rust_g_strdup_inline(app_name as *const ::core::ffi::c_char);
    g_hash_table_insert(
        apps,
        safe_c2rust_g_strdup_inline((*info).desktop_id) as gpointer,
        info as gpointer,
    );
}
#[no_mangle]
pub static mut safe_c2rust_desktop_key_match_category: [gchar; 5] = [
    2 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    5 as ::core::ffi::c_int as gchar,
];
#[no_mangle]
pub static mut safe_c2rust_exec_key_match_blocklist: [*const ::core::ffi::c_char; 12] = [
    b"bash\0" as *const u8 as *const ::core::ffi::c_char,
    b"env\0" as *const u8 as *const ::core::ffi::c_char,
    b"flatpak\0" as *const u8 as *const ::core::ffi::c_char,
    b"gjs\0" as *const u8 as *const ::core::ffi::c_char,
    b"pkexec\0" as *const u8 as *const ::core::ffi::c_char,
    b"python\0" as *const u8 as *const ::core::ffi::c_char,
    b"python2\0" as *const u8 as *const ::core::ffi::c_char,
    b"python3\0" as *const u8 as *const ::core::ffi::c_char,
    b"sh\0" as *const u8 as *const ::core::ffi::c_char,
    b"wine\0" as *const u8 as *const ::core::ffi::c_char,
    b"wine64\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn safe_c2rust_desktop_key_get_name(mut key_id: guint) -> *mut gchar {
    match key_id {
        0 => return b"Exec\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        1 => return GENERIC_NAME_KEY.as_ptr() as *mut gchar,
        2 => return KEYWORDS_KEY.as_ptr() as *mut gchar,
        3 => return b"Name\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        4 => return FULL_NAME_KEY.as_ptr() as *mut gchar,
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                491 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
static mut safe_c2rust_static_token_results: *mut search_result =
    ::core::ptr::null::<search_result>() as *mut search_result;
static mut safe_c2rust_static_token_results_size: gint = 0;
static mut safe_c2rust_static_token_results_allocated: gint = 0;
static mut safe_c2rust_static_search_results: *mut search_result =
    ::core::ptr::null::<search_result>() as *mut search_result;
static mut safe_c2rust_static_search_results_size: gint = 0;
static mut safe_c2rust_static_search_results_allocated: gint = 0;
static mut safe_c2rust_static_total_results: *mut search_result =
    ::core::ptr::null::<search_result>() as *mut search_result;
static mut safe_c2rust_static_total_results_size: gint = 0;
static mut safe_c2rust_static_total_results_allocated: gint = 0;
unsafe extern "C" fn safe_c2rust_compare_results(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gint {
    let mut ra: *const search_result = a as *const search_result;
    let mut rb: *const search_result = b as *const search_result;
    if (*ra).app_name < (*rb).app_name {
        return -(1 as gint);
    } else if (*ra).app_name > (*rb).app_name {
        return 1 as gint;
    } else {
        if (*ra).match_type != (*rb).match_type {
            return (*ra).match_type - (*rb).match_type;
        }
        return (*ra).category - (*rb).category;
    };
}
unsafe extern "C" fn safe_c2rust_compare_categories(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gint {
    let mut ra: *const search_result = a as *const search_result;
    let mut rb: *const search_result = b as *const search_result;
    if (*ra).match_type != (*rb).match_type {
        return (*ra).match_type - (*rb).match_type;
    }
    return (*ra).category - (*rb).category;
}
unsafe extern "C" fn safe_c2rust_add_token_result(
    mut app_name: *const gchar,
    mut category: guint16,
    mut match_type: guint16,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if safe_c2rust_static_token_results_size == safe_c2rust_static_token_results_allocated {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_static_token_results_allocated = (if 16 as ::core::ffi::c_int
            > safe_c2rust_static_token_results_allocated as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int
        {
            16 as ::core::ffi::c_int
        } else {
            safe_c2rust_static_token_results_allocated as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int
        }) as gint;
        safe_c2rust_static_token_results = ({
            let mut __n: gsize = safe_c2rust_static_token_results_allocated as gsize;
            let mut __s: gsize = ::core::mem::size_of::<search_result>() as gsize;
            let mut __p: gpointer = safe_c2rust_static_token_results as gpointer;
            if __s == 1 as gsize {
                __p = g_realloc(__p, __n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_realloc(__p, __n.wrapping_mul(__s));
            } else {
                __p = g_realloc_n(__p, __n, __s);
            }
            __p
        }) as *mut search_result;
    }
    let ref mut fresh38 = (*safe_c2rust_static_token_results
        .offset(safe_c2rust_static_token_results_size as isize))
    .app_name;
    *fresh38 = app_name;
    (*safe_c2rust_static_token_results.offset(safe_c2rust_static_token_results_size as isize))
        .category = category as gint;
    (*safe_c2rust_static_token_results.offset(safe_c2rust_static_token_results_size as isize))
        .match_type = match_type as gint;
    safe_c2rust_static_token_results_size += 1;
}
unsafe extern "C" fn safe_c2rust_merge_token_results(mut first: gboolean) {
    if safe_c2rust_static_token_results_size != 0 as ::core::ffi::c_int {
        qsort(
            safe_c2rust_static_token_results as *mut ::core::ffi::c_void,
            safe_c2rust_static_token_results_size as size_t,
            ::core::mem::size_of::<search_result>() as size_t,
            Some(
                safe_c2rust_compare_results
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
            ),
        );
    }
    if first != 0 {
        let mut last_name: *const gchar = ::core::ptr::null::<gchar>();
        let mut i: gint = 0;
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if safe_c2rust_static_search_results_allocated < safe_c2rust_static_token_results_size {
                _g_boolean_var_12 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_12 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_12
        }) as ::core::ffi::c_long
            != 0
        {
            safe_c2rust_static_search_results_allocated =
                safe_c2rust_static_token_results_allocated;
            safe_c2rust_static_search_results = ({
                let mut __n: gsize = safe_c2rust_static_search_results_allocated as gsize;
                let mut __s: gsize = ::core::mem::size_of::<search_result>() as gsize;
                let mut __p: gpointer = safe_c2rust_static_search_results as gpointer;
                if __s == 1 as gsize {
                    __p = g_realloc(__p, __n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_realloc(__p, __n.wrapping_mul(__s));
                } else {
                    __p = g_realloc_n(__p, __n, __s);
                }
                __p
            }) as *mut search_result;
        }
        i = 0 as ::core::ffi::c_int as gint;
        while i < safe_c2rust_static_token_results_size {
            if !((*safe_c2rust_static_token_results.offset(i as isize)).app_name == last_name) {
                last_name = (*safe_c2rust_static_token_results.offset(i as isize)).app_name;
                let fresh36 = safe_c2rust_static_search_results_size;
                safe_c2rust_static_search_results_size = safe_c2rust_static_search_results_size + 1;
                *safe_c2rust_static_search_results.offset(fresh36 as isize) =
                    *safe_c2rust_static_token_results.offset(i as isize);
            }
            i += 1;
        }
    } else {
        let mut last_name_0: *const gchar = ::core::ptr::null::<gchar>();
        let mut i_0: gint = 0;
        let mut j: gint = 0 as gint;
        let mut k: gint = 0 as gint;
        i_0 = 0 as ::core::ffi::c_int as gint;
        while i_0 < safe_c2rust_static_token_results_size {
            if !((*safe_c2rust_static_token_results.offset(i_0 as isize)).app_name == last_name_0) {
                last_name_0 = (*safe_c2rust_static_token_results.offset(i_0 as isize)).app_name;
                while k < safe_c2rust_static_search_results_size
                    && (*safe_c2rust_static_search_results.offset(k as isize)).app_name
                        < (*safe_c2rust_static_token_results.offset(i_0 as isize)).app_name
                {
                    k += 1;
                }
                if k < safe_c2rust_static_search_results_size
                    && (*safe_c2rust_static_search_results.offset(k as isize)).app_name
                        == (*safe_c2rust_static_token_results.offset(i_0 as isize)).app_name
                {
                    let ref mut fresh37 =
                        (*safe_c2rust_static_search_results.offset(j as isize)).app_name;
                    *fresh37 = (*safe_c2rust_static_search_results.offset(k as isize)).app_name;
                    (*safe_c2rust_static_search_results.offset(j as isize)).category =
                        if (*safe_c2rust_static_search_results.offset(k as isize)).category
                            > (*safe_c2rust_static_token_results.offset(i_0 as isize)).category
                        {
                            (*safe_c2rust_static_search_results.offset(k as isize)).category
                        } else {
                            (*safe_c2rust_static_token_results.offset(i_0 as isize)).category
                        };
                    (*safe_c2rust_static_search_results.offset(j as isize)).match_type =
                        if (*safe_c2rust_static_search_results.offset(k as isize)).match_type
                            > (*safe_c2rust_static_token_results.offset(i_0 as isize)).match_type
                        {
                            (*safe_c2rust_static_search_results.offset(k as isize)).match_type
                        } else {
                            (*safe_c2rust_static_token_results.offset(i_0 as isize)).match_type
                        };
                    j += 1;
                }
            }
            i_0 += 1;
        }
        safe_c2rust_static_search_results_size = j;
    }
    safe_c2rust_static_token_results_size = 0 as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn safe_c2rust_reset_total_search_results() {
    safe_c2rust_static_total_results_size = 0 as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn safe_c2rust_sort_total_search_results() {
    if safe_c2rust_static_total_results_size != 0 as ::core::ffi::c_int {
        qsort(
            safe_c2rust_static_total_results as *mut ::core::ffi::c_void,
            safe_c2rust_static_total_results_size as size_t,
            ::core::mem::size_of::<search_result>() as size_t,
            Some(
                safe_c2rust_compare_categories
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
            ),
        );
    }
}
unsafe extern "C" fn safe_c2rust_merge_directory_results() {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if safe_c2rust_static_total_results_size + safe_c2rust_static_search_results_size
            > safe_c2rust_static_total_results_allocated
        {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_static_total_results_allocated =
            (if 16 as ::core::ffi::c_int > safe_c2rust_static_total_results_allocated {
                16 as ::core::ffi::c_int
            } else {
                safe_c2rust_static_total_results_allocated as ::core::ffi::c_int
            }) as gint;
        while safe_c2rust_static_total_results_allocated
            < safe_c2rust_static_total_results_size + safe_c2rust_static_search_results_size
        {
            safe_c2rust_static_total_results_allocated *= 2 as ::core::ffi::c_int;
        }
        safe_c2rust_static_total_results = ({
            let mut __n: gsize = safe_c2rust_static_total_results_allocated as gsize;
            let mut __s: gsize = ::core::mem::size_of::<search_result>() as gsize;
            let mut __p: gpointer = safe_c2rust_static_total_results as gpointer;
            if __s == 1 as gsize {
                __p = g_realloc(__p, __n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_realloc(__p, __n.wrapping_mul(__s));
            } else {
                __p = g_realloc_n(__p, __n, __s);
            }
            __p
        }) as *mut search_result;
    }
    if safe_c2rust_static_search_results_size != 0 as ::core::ffi::c_int {
        memcpy(
            safe_c2rust_static_total_results.offset(safe_c2rust_static_total_results_size as isize)
                as *mut ::core::ffi::c_void,
            safe_c2rust_static_search_results as *const ::core::ffi::c_void,
            (safe_c2rust_static_search_results_size as size_t)
                .wrapping_mul(::core::mem::size_of::<search_result>() as size_t),
        );
    }
    safe_c2rust_static_total_results_size += safe_c2rust_static_search_results_size;
    safe_c2rust_static_search_results_size = 0 as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn safe_c2rust_get_apps_from_dir(
    mut apps: *mut *mut GHashTable,
    mut dirname: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut basename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    dir = g_dir_open(
        dirname as *const gchar,
        0 as guint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if dir.is_null() {
        return;
    }
    loop {
        basename = g_dir_read_name(dir) as *const ::core::ffi::c_char;
        if basename.is_null() {
            break;
        }
        let mut filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
        filename = g_build_filename(dirname as *const gchar, basename, NULL_1);
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename;
                let __suffix: *const ::core::ffi::c_char =
                    b".desktop\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_14
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
            g_str_has_suffix(
                basename as *const gchar,
                b".desktop\0" as *const u8 as *const gchar,
            )
        } != 0
        {
            let mut app_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            app_name = g_strconcat(prefix as *const gchar, basename, NULL_1);
            if (*apps).is_null() {
                *apps = g_hash_table_new_full(
                    Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                    Some(
                        g_str_equal
                            as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
                    ),
                    Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                    Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                );
            }
            g_hash_table_insert(
                *apps,
                app_name as gpointer,
                safe_c2rust_g_strdup_inline(filename) as gpointer,
            );
        } else if g_file_test(filename, G_FILE_TEST_IS_DIR) != 0 {
            let mut subprefix: *mut gchar = ::core::ptr::null_mut::<gchar>();
            subprefix = g_strconcat(
                prefix as *const gchar,
                basename,
                b"-\0" as *const u8 as *const ::core::ffi::c_char,
                NULL_1,
            );
            safe_c2rust_get_apps_from_dir(apps, filename, subprefix);
            g_free(subprefix as gpointer);
        }
        g_free(filename as gpointer);
    }
    g_dir_close(dir);
}
unsafe extern "C" fn safe_c2rust_free_mime_tweaks(mut data: gpointer) {
    let mut tweaks: *mut UnindexedMimeTweaks = data as *mut UnindexedMimeTweaks;
    g_strfreev((*tweaks).additions);
    g_strfreev((*tweaks).removals);
    g_strfreev((*tweaks).defaults);
    g_slice_free1(
        ::core::mem::size_of::<UnindexedMimeTweaks>() as gsize,
        tweaks as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_get_tweaks(
    mut dir: *mut DesktopFileDir,
    mut mime_type: *const gchar,
) -> *mut UnindexedMimeTweaks {
    let mut tweaks: *mut UnindexedMimeTweaks = ::core::ptr::null_mut::<UnindexedMimeTweaks>();
    let mut unaliased_type: *mut gchar = ::core::ptr::null_mut::<gchar>();
    unaliased_type =
        _g_unix_content_type_unalias(mime_type as *const ::core::ffi::c_char) as *mut gchar;
    tweaks = g_hash_table_lookup((*dir).mime_tweaks, unaliased_type as gconstpointer)
        as *mut UnindexedMimeTweaks;
    if tweaks.is_null() {
        tweaks = ({
            let mut __s: gsize = ::core::mem::size_of::<UnindexedMimeTweaks>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut UnindexedMimeTweaks;
        g_hash_table_insert(
            (*dir).mime_tweaks,
            unaliased_type as gpointer,
            tweaks as gpointer,
        );
    } else {
        g_free(unaliased_type as gpointer);
    }
    return tweaks;
}
unsafe extern "C" fn safe_c2rust_expand_strv(
    mut strv_ptr: *mut *mut *mut gchar,
    mut to_add: *mut *mut gchar,
    mut blocklist: *const *mut gchar,
) {
    let mut current_block: u64;
    let mut strv_len: guint = 0;
    let mut add_len: guint = 0;
    let mut strv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: guint = 0;
    let mut j: guint = 0;
    if (*strv_ptr).is_null() {
        *strv_ptr = to_add;
        return;
    }
    strv = *strv_ptr;
    strv_len = g_strv_length(strv);
    add_len = g_strv_length(to_add);
    strv = ({
        let mut __n: gsize = strv_len.wrapping_add(add_len).wrapping_add(1 as guint) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
        let mut __p: gpointer = strv as gpointer;
        if __s == 1 as gsize {
            __p = g_realloc(__p, __n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_realloc(__p, __n.wrapping_mul(__s));
        } else {
            __p = g_realloc_n(__p, __n, __s);
        }
        __p
    }) as *mut *mut gchar;
    i = 0 as guint;
    while !(*to_add.offset(i as isize)).is_null() {
        if !blocklist.is_null() {
            j = 0 as guint;
            loop {
                if (*blocklist.offset(j as isize)).is_null() {
                    current_block = 12209867499936983673;
                    break;
                }
                if strcmp(
                    *to_add.offset(i as isize) as *const ::core::ffi::c_char,
                    *blocklist.offset(j as isize) as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    current_block = 13262583935794996699;
                    break;
                }
                j = j.wrapping_add(1);
            }
        } else {
            current_block = 12209867499936983673;
        }
        match current_block {
            12209867499936983673 => {
                j = 0 as guint;
                loop {
                    if !(j < strv_len) {
                        current_block = 10048703153582371463;
                        break;
                    }
                    if strcmp(
                        *to_add.offset(i as isize) as *const ::core::ffi::c_char,
                        *strv.offset(j as isize) as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        current_block = 13262583935794996699;
                        break;
                    }
                    j = j.wrapping_add(1);
                }
                match current_block {
                    13262583935794996699 => {}
                    _ => {
                        let fresh28 = strv_len;
                        strv_len = strv_len.wrapping_add(1);
                        let ref mut fresh29 = *strv.offset(fresh28 as isize);
                        *fresh29 = *to_add.offset(i as isize);
                        current_block = 4906268039856690917;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            13262583935794996699 => {
                g_free(*to_add.offset(i as isize) as gpointer);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    let ref mut fresh30 = *strv.offset(strv_len as isize);
    *fresh30 = ::core::ptr::null_mut::<gchar>();
    *strv_ptr = strv;
    g_free(to_add as gpointer);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_read_mimeapps_list(
    mut dir: *mut DesktopFileDir,
    mut filename: *const gchar,
    mut added_group: *const gchar,
    mut tweaks_permitted: gboolean,
) {
    let mut tweaks: *mut UnindexedMimeTweaks = ::core::ptr::null_mut::<UnindexedMimeTweaks>();
    let mut desktop_file_ids: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut key_file: *mut GKeyFile = ::core::ptr::null_mut::<GKeyFile>();
    let mut mime_types: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: ::core::ffi::c_int = 0;
    key_file = g_key_file_new();
    if g_key_file_load_from_file(
        key_file,
        filename,
        G_KEY_FILE_NONE,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        g_key_file_free(key_file);
        return;
    }
    mime_types = g_key_file_get_keys(
        key_file,
        added_group,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !mime_types.is_null() && tweaks_permitted == 0 {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s contains a [%s] group, but it is not permitted here.  Only the non-desktop-specific mimeapps.list file may add or remove associations.\0"
                as *const u8 as *const gchar,
            filename,
            added_group,
        );
        g_strfreev(mime_types);
        mime_types = ::core::ptr::null_mut::<*mut gchar>();
    }
    if !mime_types.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*mime_types.offset(i as isize)).is_null() {
            desktop_file_ids = g_key_file_get_string_list(
                key_file,
                added_group,
                *mime_types.offset(i as isize),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            ) as *mut *mut ::core::ffi::c_char;
            if !desktop_file_ids.is_null() {
                tweaks = safe_c2rust_desktop_file_dir_unindexed_get_tweaks(
                    dir,
                    *mime_types.offset(i as isize),
                );
                safe_c2rust_expand_strv(
                    &raw mut (*tweaks).additions,
                    desktop_file_ids as *mut *mut gchar,
                    (*tweaks).removals,
                );
            }
            i += 1;
        }
        g_strfreev(mime_types);
    }
    mime_types = g_key_file_get_keys(
        key_file,
        REMOVED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !mime_types.is_null() && tweaks_permitted == 0 {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s contains a [%s] group, but it is not permitted here.  Only the non-desktop-specific mimeapps.list file may add or remove associations.\0"
                as *const u8 as *const gchar,
            filename,
            b"Removed Associations\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_strfreev(mime_types);
        mime_types = ::core::ptr::null_mut::<*mut gchar>();
    }
    if !mime_types.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*mime_types.offset(i as isize)).is_null() {
            desktop_file_ids = g_key_file_get_string_list(
                key_file,
                REMOVED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
                *mime_types.offset(i as isize),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            ) as *mut *mut ::core::ffi::c_char;
            if !desktop_file_ids.is_null() {
                tweaks = safe_c2rust_desktop_file_dir_unindexed_get_tweaks(
                    dir,
                    *mime_types.offset(i as isize),
                );
                safe_c2rust_expand_strv(
                    &raw mut (*tweaks).removals,
                    desktop_file_ids as *mut *mut gchar,
                    (*tweaks).additions,
                );
            }
            i += 1;
        }
        g_strfreev(mime_types);
    }
    mime_types = g_key_file_get_keys(
        key_file,
        DEFAULT_APPLICATIONS_GROUP.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !mime_types.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*mime_types.offset(i as isize)).is_null() {
            desktop_file_ids = g_key_file_get_string_list(
                key_file,
                DEFAULT_APPLICATIONS_GROUP.as_ptr() as *const gchar,
                *mime_types.offset(i as isize),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            ) as *mut *mut ::core::ffi::c_char;
            if !desktop_file_ids.is_null() {
                tweaks = safe_c2rust_desktop_file_dir_unindexed_get_tweaks(
                    dir,
                    *mime_types.offset(i as isize),
                );
                safe_c2rust_expand_strv(
                    &raw mut (*tweaks).defaults,
                    desktop_file_ids as *mut *mut gchar,
                    ::core::ptr::null::<*mut gchar>(),
                );
            }
            i += 1;
        }
        g_strfreev(mime_types);
    }
    g_key_file_free(key_file);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_read_mimeapps_lists(
    mut dir: *mut DesktopFileDir,
) {
    let mut desktops: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: gint = 0;
    (*dir).mime_tweaks = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(safe_c2rust_free_mime_tweaks as unsafe extern "C" fn(gpointer) -> ()),
    );
    desktops = safe_c2rust_get_lowercase_current_desktops();
    i = 0 as ::core::ffi::c_int as gint;
    while !(*desktops.offset(i as isize)).is_null() {
        filename = g_strdup_printf(
            b"%s/%s-mimeapps.list\0" as *const u8 as *const gchar,
            (*dir).path,
            *desktops.offset(i as isize),
        );
        safe_c2rust_desktop_file_dir_unindexed_read_mimeapps_list(
            dir,
            filename,
            ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
            FALSE,
        );
        g_free(filename as gpointer);
        i += 1;
    }
    filename = g_strdup_printf(
        b"%s/mimeapps.list\0" as *const u8 as *const gchar,
        (*dir).path,
    );
    safe_c2rust_desktop_file_dir_unindexed_read_mimeapps_list(
        dir,
        filename,
        ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
        TRUE,
    );
    g_free(filename as gpointer);
    if (*dir).is_config != 0 {
        return;
    }
    filename = g_strdup_printf(
        b"%s/defaults.list\0" as *const u8 as *const gchar,
        (*dir).path,
    );
    safe_c2rust_desktop_file_dir_unindexed_read_mimeapps_list(
        dir,
        filename,
        ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
        FALSE,
    );
    g_free(filename as gpointer);
    filename = g_strdup_printf(
        b"%s/mimeinfo.cache\0" as *const u8 as *const gchar,
        (*dir).path,
    );
    safe_c2rust_desktop_file_dir_unindexed_read_mimeapps_list(
        dir,
        filename,
        MIME_CACHE_GROUP.as_ptr() as *const gchar,
        TRUE,
    );
    g_free(filename as gpointer);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_init(mut dir: *mut DesktopFileDir) {
    if (*dir).is_config == 0 {
        safe_c2rust_get_apps_from_dir(
            &raw mut (*dir).app_names,
            (*dir).path,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_desktop_file_dir_unindexed_read_mimeapps_lists(dir);
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_new_from_filename_unlocked(
    mut filename: *const ::core::ffi::c_char,
) -> *mut GDesktopAppInfo {
    let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    info = g_object_new(
        safe_c2rust_g_desktop_app_info_get_type(),
        b"filename\0" as *const u8 as *const gchar,
        filename,
        NULL_1,
    ) as *mut GDesktopAppInfo;
    if safe_c2rust_g_desktop_app_info_load_file(info) == 0 {
        let mut _pp: *mut *mut GDesktopAppInfo = &raw mut info;
        let mut _ptr: *mut GDesktopAppInfo = *_pp;
        *_pp = ::core::ptr::null_mut::<GDesktopAppInfo>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    }
    return info;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_get_app(
    mut dir: *mut DesktopFileDir,
    mut desktop_id: *const gchar,
) -> *mut GDesktopAppInfo {
    let mut filename: *const gchar = ::core::ptr::null::<gchar>();
    filename = g_hash_table_lookup((*dir).app_names, desktop_id as gconstpointer) as *const gchar;
    if filename.is_null() {
        return ::core::ptr::null_mut::<GDesktopAppInfo>();
    }
    return safe_c2rust_g_desktop_app_info_new_from_filename_unlocked(
        filename as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_get_all(
    mut dir: *mut DesktopFileDir,
    mut apps: *mut GHashTable,
) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut app_name: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut filename: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*dir).app_names.is_null() {
        return;
    }
    g_hash_table_iter_init(&raw mut iter, (*dir).app_names);
    while g_hash_table_iter_next(&raw mut iter, &raw mut app_name, &raw mut filename) != 0 {
        if safe_c2rust_desktop_file_dir_app_name_is_masked(dir, app_name as *const gchar) != 0 {
            continue;
        }
        safe_c2rust_add_to_table_if_appropriate(
            apps,
            app_name as *const gchar,
            safe_c2rust_g_desktop_app_info_new_from_filename_unlocked(
                filename as *const ::core::ffi::c_char,
            ),
        );
    }
}
unsafe extern "C" fn safe_c2rust_memory_index_entry_free(mut data: gpointer) {
    let mut mie: *mut MemoryIndexEntry = data as *mut MemoryIndexEntry;
    while !mie.is_null() {
        let mut next: *mut MemoryIndexEntry = (*mie).next;
        g_slice_free1(
            ::core::mem::size_of::<MemoryIndexEntry>() as gsize,
            mie as gpointer,
        );
        mie = next;
    }
}
unsafe extern "C" fn safe_c2rust_memory_index_add_token(
    mut mi: *mut MemoryIndex,
    mut token: *const gchar,
    mut match_category: gint,
    mut app_name: *const gchar,
) {
    let mut mie: *mut MemoryIndexEntry = ::core::ptr::null_mut::<MemoryIndexEntry>();
    let mut first: *mut MemoryIndexEntry = ::core::ptr::null_mut::<MemoryIndexEntry>();
    mie =
        g_slice_alloc(::core::mem::size_of::<MemoryIndexEntry>() as gsize) as *mut MemoryIndexEntry;
    (*mie).app_name = app_name;
    (*mie).match_category = match_category;
    first =
        g_hash_table_lookup(mi as *mut GHashTable, token as gconstpointer) as *mut MemoryIndexEntry;
    if !first.is_null() {
        (*mie).next = (*first).next;
        (*first).next = mie;
    } else {
        (*mie).next = ::core::ptr::null_mut::<MemoryIndexEntry>();
        g_hash_table_insert(
            mi as *mut GHashTable,
            safe_c2rust_g_strdup_inline(token as *const ::core::ffi::c_char) as gpointer,
            mie as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_memory_index_add_string(
    mut mi: *mut MemoryIndex,
    mut string: *const gchar,
    mut match_category: gint,
    mut app_name: *const gchar,
) {
    let mut tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut alternates: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gint = 0;
    tokens = g_str_tokenize_and_fold(string, ::core::ptr::null::<gchar>(), &raw mut alternates);
    i = 0 as ::core::ffi::c_int as gint;
    while !(*tokens.offset(i as isize)).is_null() {
        safe_c2rust_memory_index_add_token(
            mi,
            *tokens.offset(i as isize),
            match_category,
            app_name,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int as gint;
    while !(*alternates.offset(i as isize)).is_null() {
        safe_c2rust_memory_index_add_token(
            mi,
            *alternates.offset(i as isize),
            match_category,
            app_name,
        );
        i += 1;
    }
    g_strfreev(alternates);
    g_strfreev(tokens);
}
unsafe extern "C" fn safe_c2rust_memory_index_new() -> *mut MemoryIndex {
    return g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(safe_c2rust_memory_index_entry_free as unsafe extern "C" fn(gpointer) -> ()),
    ) as *mut MemoryIndex;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_setup_search(
    mut dir: *mut DesktopFileDir,
) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut app: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut path: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*dir).memory_index = safe_c2rust_memory_index_new() as *mut GHashTable;
    (*dir).memory_implementations = safe_c2rust_memory_index_new() as *mut GHashTable;
    if (*dir).app_names.is_null() {
        return;
    }
    g_hash_table_iter_init(&raw mut iter, (*dir).app_names);
    while g_hash_table_iter_next(&raw mut iter, &raw mut app, &raw mut path) != 0 {
        let mut key_file: *mut GKeyFile = ::core::ptr::null_mut::<GKeyFile>();
        if safe_c2rust_desktop_file_dir_app_name_is_masked(dir, app as *const gchar) != 0 {
            continue;
        }
        key_file = g_key_file_new();
        if g_key_file_load_from_file(
            key_file,
            path as *const gchar,
            G_KEY_FILE_NONE,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != 0
            && g_key_file_get_boolean(
                key_file,
                b"Desktop Entry\0" as *const u8 as *const gchar,
                b"Hidden\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<*mut GError>(),
            ) == 0
        {
            let mut implements: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            let mut i: gsize = 0;
            i = 0 as gsize;
            while (i as usize)
                < (::core::mem::size_of::<[gchar; 5]>() as usize)
                    .wrapping_div(::core::mem::size_of::<gchar>() as usize)
            {
                let mut value: *const gchar = ::core::ptr::null::<gchar>();
                let mut raw: *mut gchar = ::core::ptr::null_mut::<gchar>();
                if !(safe_c2rust_desktop_key_match_category[i as usize] == 0) {
                    raw = g_key_file_get_locale_string(
                        key_file,
                        b"Desktop Entry\0" as *const u8 as *const gchar,
                        safe_c2rust_desktop_key_get_name(i as guint),
                        ::core::ptr::null::<gchar>(),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                    value = raw;
                    if i == DESKTOP_KEY_Exec as ::core::ffi::c_int as gsize && !raw.is_null() {
                        let mut space: *mut gchar = ::core::ptr::null_mut::<gchar>();
                        let mut slash: *mut gchar = ::core::ptr::null_mut::<gchar>();
                        space = raw.offset(strcspn(
                            raw,
                            b" \t\n\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as isize);
                        *space = '\0' as i32 as gchar;
                        slash = strrchr(raw, '/' as i32) as *mut gchar;
                        if !slash.is_null() {
                            value = slash.offset(1 as ::core::ffi::c_int as isize);
                        }
                        if g_strv_contains(
                            &raw const safe_c2rust_exec_key_match_blocklist as *const *const gchar,
                            value,
                        ) != 0
                        {
                            value = ::core::ptr::null::<gchar>();
                        }
                    }
                    if !value.is_null() {
                        safe_c2rust_memory_index_add_string(
                            (*dir).memory_index as *mut MemoryIndex,
                            value,
                            safe_c2rust_desktop_key_match_category[i as usize] as gint,
                            app as *const gchar,
                        );
                    }
                    g_free(raw as gpointer);
                }
                i = i.wrapping_add(1);
            }
            implements = g_key_file_get_string_list(
                key_file,
                b"Desktop Entry\0" as *const u8 as *const gchar,
                b"Implements\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            i = 0 as gsize;
            while !implements.is_null() && !(*implements.offset(i as isize)).is_null() {
                safe_c2rust_memory_index_add_token(
                    (*dir).memory_implementations as *mut MemoryIndex,
                    *implements.offset(i as isize),
                    0 as gint,
                    app as *const gchar,
                );
                i = i.wrapping_add(1);
            }
            g_strfreev(implements);
        }
        g_key_file_free(key_file);
    }
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_search(
    mut dir: *mut DesktopFileDir,
    mut search_token: *const gchar,
) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !search_token.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1249 as ::core::ffi::c_int,
            G_STRFUNC,
            b"search_token != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*dir).memory_index.is_null() {
        safe_c2rust_desktop_file_dir_unindexed_setup_search(dir);
    }
    g_hash_table_iter_init(&raw mut iter, (*dir).memory_index);
    while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut value) != 0 {
        let mut mie: *mut MemoryIndexEntry = value as *mut MemoryIndexEntry;
        let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut match_type: MatchType = 0 as MatchType;
        p = strstr(
            key as *const ::core::ffi::c_char,
            search_token as *const ::core::ffi::c_char,
        );
        if p.is_null() {
            continue;
        }
        if p == key as *const ::core::ffi::c_char
            && *search_token as ::core::ffi::c_int != '\0' as i32
        {
            match_type = MATCH_TYPE_PREFIX;
        } else {
            match_type = MATCH_TYPE_SUBSTRING;
        }
        while !mie.is_null() {
            safe_c2rust_add_token_result(
                (*mie).app_name,
                (*mie).match_category as guint16,
                match_type as guint16,
            );
            mie = (*mie).next;
        }
    }
}
unsafe extern "C" fn safe_c2rust_array_contains(
    mut array: *mut GPtrArray,
    mut str: *const gchar,
) -> gboolean {
    let mut i: guint = 0;
    i = 0 as guint;
    while i < (*array).len {
        if strcmp(
            *(*array).pdata.offset(i as isize) as *const ::core::ffi::c_char,
            str as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_mime_lookup(
    mut dir: *mut DesktopFileDir,
    mut mime_type: *const gchar,
    mut hits: *mut GPtrArray,
    mut blocklist: *mut GPtrArray,
) {
    let mut tweaks: *mut UnindexedMimeTweaks = ::core::ptr::null_mut::<UnindexedMimeTweaks>();
    let mut i: gint = 0;
    tweaks = g_hash_table_lookup((*dir).mime_tweaks, mime_type as gconstpointer)
        as *mut UnindexedMimeTweaks;
    if tweaks.is_null() {
        return;
    }
    if !(*tweaks).additions.is_null() {
        i = 0 as ::core::ffi::c_int as gint;
        while !(*(*tweaks).additions.offset(i as isize)).is_null() {
            let mut app_name: *mut gchar = *(*tweaks).additions.offset(i as isize);
            if safe_c2rust_desktop_file_dir_app_name_is_masked(dir, app_name) == 0
                && safe_c2rust_array_contains(blocklist, app_name) == 0
                && safe_c2rust_array_contains(hits, app_name) == 0
            {
                g_ptr_array_add(hits, app_name as gpointer);
            }
            i += 1;
        }
    }
    if !(*tweaks).removals.is_null() {
        i = 0 as ::core::ffi::c_int as gint;
        while !(*(*tweaks).removals.offset(i as isize)).is_null() {
            let mut app_name_0: *mut gchar = *(*tweaks).removals.offset(i as isize);
            if safe_c2rust_desktop_file_dir_app_name_is_masked(dir, app_name_0) == 0
                && safe_c2rust_array_contains(blocklist, app_name_0) == 0
                && safe_c2rust_array_contains(hits, app_name_0) == 0
            {
                g_ptr_array_add(blocklist, app_name_0 as gpointer);
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_default_lookup(
    mut dir: *mut DesktopFileDir,
    mut mime_type: *const gchar,
    mut results: *mut GPtrArray,
) {
    let mut tweaks: *mut UnindexedMimeTweaks = ::core::ptr::null_mut::<UnindexedMimeTweaks>();
    let mut i: gint = 0;
    tweaks = g_hash_table_lookup((*dir).mime_tweaks, mime_type as gconstpointer)
        as *mut UnindexedMimeTweaks;
    if tweaks.is_null() || (*tweaks).defaults.is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int as gint;
    while !(*(*tweaks).defaults.offset(i as isize)).is_null() {
        let mut app_name: *mut gchar = *(*tweaks).defaults.offset(i as isize);
        if safe_c2rust_array_contains(results, app_name) == 0 {
            g_ptr_array_add(results, app_name as gpointer);
        }
        i += 1;
    }
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_unindexed_get_implementations(
    mut dir: *mut DesktopFileDir,
    mut results: *mut *mut GList,
    mut interface: *const gchar,
) {
    let mut mie: *mut MemoryIndexEntry = ::core::ptr::null_mut::<MemoryIndexEntry>();
    if (*dir).memory_index.is_null() {
        safe_c2rust_desktop_file_dir_unindexed_setup_search(dir);
    }
    mie = g_hash_table_lookup((*dir).memory_implementations, interface as gconstpointer)
        as *mut MemoryIndexEntry;
    while !mie.is_null() {
        *results = g_list_prepend(
            *results,
            safe_c2rust_g_strdup_inline((*mie).app_name as *const ::core::ffi::c_char) as gpointer,
        );
        mie = (*mie).next;
    }
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_new(
    mut data_dir: *const gchar,
) -> *mut DesktopFileDir {
    let mut dir: *mut DesktopFileDir = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<DesktopFileDir>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut DesktopFileDir;
    g_atomic_ref_count_init(&raw mut (*dir).ref_count);
    (*dir).path = g_build_filename(
        data_dir,
        b"applications\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    );
    return safe_c2rust_g_steal_pointer(&raw mut dir as gpointer) as *mut DesktopFileDir;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_new_for_config(
    mut config_dir: *const gchar,
) -> *mut DesktopFileDir {
    let mut dir: *mut DesktopFileDir = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<DesktopFileDir>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut DesktopFileDir;
    g_atomic_ref_count_init(&raw mut (*dir).ref_count);
    (*dir).path =
        safe_c2rust_g_strdup_inline(config_dir as *const ::core::ffi::c_char) as *mut gchar;
    (*dir).is_config = TRUE as gboolean;
    return safe_c2rust_g_steal_pointer(&raw mut dir as gpointer) as *mut DesktopFileDir;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_reset(mut dir: *mut DesktopFileDir) {
    if !(*dir).alternatively_watching.is_null() {
        g_free((*dir).alternatively_watching as gpointer);
        (*dir).alternatively_watching = ::core::ptr::null_mut::<gchar>();
    }
    if !(*dir).monitor.is_null() {
        g_signal_handlers_disconnect_matched(
            (*dir).monitor as gpointer,
            (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
                as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ::core::ptr::null_mut::<GClosure>(),
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
                gpointer,
            >(Some(
                safe_c2rust_desktop_file_dir_changed
                    as unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
            )),
            dir as gpointer,
        );
        g_file_monitor_cancel((*dir).monitor);
        g_object_unref((*dir).monitor as gpointer);
        (*dir).monitor = ::core::ptr::null_mut::<GFileMonitor>();
    }
    if (*dir).poll_source_id != 0 as guint {
        g_source_remove((*dir).poll_source_id);
        (*dir).poll_source_id = 0 as guint;
    }
    if !(*dir).poll_snapshot.is_null() {
        g_free((*dir).poll_snapshot as gpointer);
        (*dir).poll_snapshot = ::core::ptr::null_mut::<gchar>();
    }
    if !(*dir).app_names.is_null() {
        g_hash_table_unref((*dir).app_names);
        (*dir).app_names = ::core::ptr::null_mut::<GHashTable>();
    }
    if !(*dir).memory_index.is_null() {
        g_hash_table_unref((*dir).memory_index);
        (*dir).memory_index = ::core::ptr::null_mut::<GHashTable>();
    }
    if !(*dir).mime_tweaks.is_null() {
        g_hash_table_unref((*dir).mime_tweaks);
        (*dir).mime_tweaks = ::core::ptr::null_mut::<GHashTable>();
    }
    if !(*dir).memory_implementations.is_null() {
        g_hash_table_unref((*dir).memory_implementations);
        (*dir).memory_implementations = ::core::ptr::null_mut::<GHashTable>();
    }
    (*dir).is_setup = FALSE as gboolean;
}
unsafe extern "C" fn safe_c2rust_closure_notify_cb(mut data: gpointer, mut closure: *mut GClosure) {
    let mut dir: *mut DesktopFileDir = data as *mut DesktopFileDir;
    safe_c2rust_desktop_file_dir_unref(dir);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_init(mut dir: *mut DesktopFileDir) {
    let mut watch_dir: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*dir).is_setup == 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1480 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!dir->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*dir).alternatively_watching.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1482 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!dir->alternatively_watching\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*dir).monitor.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1483 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!dir->monitor\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*dir).alternatively_watching = safe_c2rust_desktop_file_dir_get_alternative_dir(dir);
    watch_dir = if !(*dir).alternatively_watching.is_null() {
        (*dir).alternatively_watching
    } else {
        (*dir).path
    };
    (*dir).monitor = g_local_file_monitor_new_in_worker(
        watch_dir,
        TRUE,
        G_FILE_MONITOR_NONE,
        Some(
            safe_c2rust_desktop_file_dir_changed
                as unsafe extern "C" fn(
                    *mut GFileMonitor,
                    *mut GFile,
                    *mut GFile,
                    GFileMonitorEvent,
                    gpointer,
                ) -> (),
        ),
        safe_c2rust_desktop_file_dir_ref(dir) as gpointer,
        Some(safe_c2rust_closure_notify_cb as unsafe extern "C" fn(gpointer, *mut GClosure) -> ()),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if (*dir).monitor.is_null() {
        (*dir).poll_snapshot = safe_c2rust_desktop_file_dir_poll_snapshot(dir);
        (*dir).poll_source_id = g_timeout_add_seconds_full(
            0 as gint,
            1 as guint,
            Some(safe_c2rust_desktop_file_dir_poll as unsafe extern "C" fn(gpointer) -> gboolean),
            safe_c2rust_desktop_file_dir_ref(dir) as gpointer,
            Some(safe_c2rust_desktop_file_dir_poll_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    safe_c2rust_desktop_file_dir_unindexed_init(dir);
    (*dir).is_setup = TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_get_app(
    mut dir: *mut DesktopFileDir,
    mut desktop_id: *const gchar,
) -> *mut GDesktopAppInfo {
    if (*dir).app_names.is_null() {
        return ::core::ptr::null_mut::<GDesktopAppInfo>();
    }
    return safe_c2rust_desktop_file_dir_unindexed_get_app(dir, desktop_id);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_get_all(
    mut dir: *mut DesktopFileDir,
    mut apps: *mut GHashTable,
) {
    safe_c2rust_desktop_file_dir_unindexed_get_all(dir, apps);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_mime_lookup(
    mut dir: *mut DesktopFileDir,
    mut mime_type: *const gchar,
    mut hits: *mut GPtrArray,
    mut blocklist: *mut GPtrArray,
) {
    safe_c2rust_desktop_file_dir_unindexed_mime_lookup(dir, mime_type, hits, blocklist);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_default_lookup(
    mut dir: *mut DesktopFileDir,
    mut mime_type: *const gchar,
    mut results: *mut GPtrArray,
) {
    safe_c2rust_desktop_file_dir_unindexed_default_lookup(dir, mime_type, results);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_search(
    mut dir: *mut DesktopFileDir,
    mut search_token: *const gchar,
) {
    safe_c2rust_desktop_file_dir_unindexed_search(dir, search_token);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dir_get_implementations(
    mut dir: *mut DesktopFileDir,
    mut results: *mut *mut GList,
    mut interface: *const gchar,
) {
    safe_c2rust_desktop_file_dir_unindexed_get_implementations(dir, results, interface);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dirs_lock() {
    let mut i: guint = 0;
    let mut user_config_dir: *const gchar = g_get_user_config_dir();
    g_mutex_lock(&raw mut safe_c2rust_desktop_file_dir_lock);
    if !safe_c2rust_desktop_file_dirs_config_dir.is_null()
        && g_strcmp0(
            safe_c2rust_desktop_file_dirs_config_dir as *const ::core::ffi::c_char,
            user_config_dir as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"%s: Resetting desktop app info dirs from %s to %s\0" as *const u8 as *const gchar,
            b"desktop_file_dirs_lock\0" as *const u8 as *const ::core::ffi::c_char,
            safe_c2rust_desktop_file_dirs_config_dir,
            user_config_dir,
        );
        g_ptr_array_set_size(safe_c2rust_desktop_file_dirs, 0 as gint);
        let mut _pp: *mut *mut DesktopFileDir = &raw mut safe_c2rust_desktop_file_dir_user_config;
        let mut _ptr: *mut DesktopFileDir = *_pp;
        *_pp = ::core::ptr::null_mut::<DesktopFileDir>();
        if !_ptr.is_null() {
            safe_c2rust_desktop_file_dir_unref(_ptr as *mut DesktopFileDir);
        }
        let mut _pp_0: *mut *mut DesktopFileDir = &raw mut safe_c2rust_desktop_file_dir_user_data;
        let mut _ptr_0: *mut DesktopFileDir = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<DesktopFileDir>();
        if !_ptr_0.is_null() {
            safe_c2rust_desktop_file_dir_unref(_ptr_0 as *mut DesktopFileDir);
        }
    }
    if safe_c2rust_desktop_file_dirs.is_null() || (*safe_c2rust_desktop_file_dirs).len == 0 as guint
    {
        let mut dirs: *const *const ::core::ffi::c_char =
            ::core::ptr::null::<*const ::core::ffi::c_char>();
        let mut i_0: gint = 0;
        if safe_c2rust_desktop_file_dirs.is_null() {
            safe_c2rust_desktop_file_dirs =
                g_ptr_array_new_with_free_func(::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut DesktopFileDir) -> ()>,
                    GDestroyNotify,
                >(Some(
                    safe_c2rust_desktop_file_dir_unref
                        as unsafe extern "C" fn(*mut DesktopFileDir) -> (),
                )));
        }
        safe_c2rust_desktop_file_dir_user_config =
            safe_c2rust_desktop_file_dir_new_for_config(user_config_dir);
        g_ptr_array_add(
            safe_c2rust_desktop_file_dirs,
            safe_c2rust_desktop_file_dir_ref(safe_c2rust_desktop_file_dir_user_config) as gpointer,
        );
        dirs = g_get_system_config_dirs() as *const *const ::core::ffi::c_char;
        i_0 = 0 as ::core::ffi::c_int as gint;
        while !(*dirs.offset(i_0 as isize)).is_null() {
            g_ptr_array_add(
                safe_c2rust_desktop_file_dirs,
                safe_c2rust_desktop_file_dir_new_for_config(
                    *dirs.offset(i_0 as isize) as *const gchar
                ) as gpointer,
            );
            i_0 += 1;
        }
        safe_c2rust_desktop_file_dir_user_data =
            safe_c2rust_desktop_file_dir_new(g_get_user_data_dir());
        g_ptr_array_add(
            safe_c2rust_desktop_file_dirs,
            safe_c2rust_desktop_file_dir_ref(safe_c2rust_desktop_file_dir_user_data) as gpointer,
        );
        dirs = g_get_system_data_dirs() as *const *const ::core::ffi::c_char;
        i_0 = 0 as ::core::ffi::c_int as gint;
        while !(*dirs.offset(i_0 as isize)).is_null() {
            g_ptr_array_add(
                safe_c2rust_desktop_file_dirs,
                safe_c2rust_desktop_file_dir_new(*dirs.offset(i_0 as isize) as *const gchar)
                    as gpointer,
            );
            i_0 += 1;
        }
        safe_c2rust_desktop_file_dirs_config_dir = user_config_dir;
    }
    i = 0 as guint;
    while i < (*safe_c2rust_desktop_file_dirs).len {
        if (*(*(*safe_c2rust_desktop_file_dirs).pdata.offset(i as isize) as *mut DesktopFileDir))
            .is_setup
            == 0
        {
            safe_c2rust_desktop_file_dir_init(
                *(*safe_c2rust_desktop_file_dirs).pdata.offset(i as isize) as *mut DesktopFileDir,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_desktop_file_dirs_unlock() {
    g_mutex_unlock(&raw mut safe_c2rust_desktop_file_dir_lock);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dirs_invalidate_user_config() {
    g_mutex_lock(&raw mut safe_c2rust_desktop_file_dir_lock);
    if !safe_c2rust_desktop_file_dir_user_config.is_null() {
        safe_c2rust_desktop_file_dir_reset(safe_c2rust_desktop_file_dir_user_config);
    }
    g_mutex_unlock(&raw mut safe_c2rust_desktop_file_dir_lock);
}
unsafe extern "C" fn safe_c2rust_desktop_file_dirs_invalidate_user_data() {
    g_mutex_lock(&raw mut safe_c2rust_desktop_file_dir_lock);
    if !safe_c2rust_desktop_file_dir_user_data.is_null() {
        safe_c2rust_desktop_file_dir_reset(safe_c2rust_desktop_file_dir_user_data);
    }
    g_mutex_unlock(&raw mut safe_c2rust_desktop_file_dir_lock);
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_finalize(mut object: *mut GObject) {
    let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    info = object as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    g_free((*info).desktop_id as gpointer);
    g_free((*info).filename as gpointer);
    if !(*info).keyfile.is_null() {
        g_key_file_unref((*info).keyfile);
    }
    g_free((*info).name as gpointer);
    g_free((*info).generic_name as gpointer);
    g_free((*info).fullname as gpointer);
    g_free((*info).comment as gpointer);
    g_free((*info).icon_name as gpointer);
    if !(*info).icon.is_null() {
        g_object_unref((*info).icon as gpointer);
    }
    g_strfreev((*info).keywords as *mut *mut gchar);
    g_strfreev((*info).only_show_in as *mut *mut gchar);
    g_strfreev((*info).not_show_in as *mut *mut gchar);
    g_free((*info).try_exec as gpointer);
    g_free((*info).exec as gpointer);
    g_free((*info).binary as gpointer);
    g_free((*info).path as gpointer);
    g_free((*info).categories as gpointer);
    g_free((*info).startup_wm_class as gpointer);
    g_strfreev((*info).mime_types as *mut *mut gchar);
    g_free((*info).app_id as gpointer);
    g_strfreev((*info).actions as *mut *mut gchar);
    (*(safe_c2rust_g_desktop_app_info_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GDesktopAppInfo =
        object as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    match prop_id {
        1 => {
            (*self_0).filename = g_value_dup_string(value) as *mut ::core::ffi::c_char;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1744 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GDesktopAppInfo =
        object as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    match prop_id {
        1 => {
            g_value_set_string(value, (*self_0).filename);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1763 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_class_init(
    mut klass: *mut GDesktopAppInfoClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_desktop_app_info_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_desktop_app_info_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_desktop_app_info_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_FILENAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"filename\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int)
                as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_init(mut local: *mut GDesktopAppInfo) {}
unsafe extern "C" fn safe_c2rust_binary_from_exec(
    mut exec: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = exec;
    while *p as ::core::ffi::c_int == ' ' as i32 {
        p = p.offset(1);
    }
    start = p;
    while *p as ::core::ffi::c_int != ' ' as i32
        && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    return g_strndup(
        start as *const gchar,
        p.offset_from(start) as ::core::ffi::c_long as gsize,
    ) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_desktop_id_for_filename(
    mut self_0: *mut GDesktopAppInfo,
) -> *mut ::core::ffi::c_char {
    let mut i: guint = 0;
    let mut desktop_id: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !(*self_0).filename.is_null() {
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
            b"self->filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    i = 0 as guint;
    while i < (*safe_c2rust_desktop_file_dirs).len {
        let mut dir: *mut DesktopFileDir =
            *(*safe_c2rust_desktop_file_dirs).pdata.offset(i as isize) as *mut DesktopFileDir;
        let mut app_names: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        let mut iter: GHashTableIter = _GHashTableIter {
            dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy4: 0,
            dummy5: 0,
            dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        app_names = (*dir).app_names;
        if !app_names.is_null() {
            g_hash_table_iter_init(&raw mut iter, app_names);
            while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut value) != 0 {
                if !(strcmp(value as *const ::core::ffi::c_char, (*self_0).filename) == 0) {
                    continue;
                }
                desktop_id =
                    safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as *mut gchar;
                break;
            }
            if !desktop_id.is_null() {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if desktop_id.is_null() {
        desktop_id = g_path_get_basename((*self_0).filename);
    }
    return safe_c2rust_g_steal_pointer(&raw mut desktop_id as gpointer)
        as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_load_from_keyfile(
    mut info: *mut GDesktopAppInfo,
    mut key_file: *mut GKeyFile,
) -> gboolean {
    let mut start_group: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut type_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut try_exec: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut exec: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bus_activatable: gboolean = 0;
    start_group = g_key_file_get_start_group(key_file) as *mut ::core::ffi::c_char;
    if start_group.is_null()
        || strcmp(start_group, G_KEY_FILE_DESKTOP_GROUP.as_ptr()) != 0 as ::core::ffi::c_int
    {
        g_free(start_group as gpointer);
        return FALSE;
    }
    g_free(start_group as gpointer);
    type_0 = g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_TYPE.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    if type_0.is_null()
        || strcmp(type_0, G_KEY_FILE_DESKTOP_TYPE_APPLICATION.as_ptr()) != 0 as ::core::ffi::c_int
    {
        g_free(type_0 as gpointer);
        return FALSE;
    }
    g_free(type_0 as gpointer);
    path = g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_PATH.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    try_exec = g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_TRY_EXEC.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    if !try_exec.is_null()
        && *try_exec.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        t = (*glib__private__())
            .g_find_program_for_path
            .expect("non-null function pointer")(
            try_exec,
            ::core::ptr::null::<::core::ffi::c_char>(),
            path,
        );
        if t.is_null() {
            g_free(path as gpointer);
            g_free(try_exec as gpointer);
            return FALSE;
        }
        g_free(t as gpointer);
    }
    exec = g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_EXEC.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    if !exec.is_null()
        && *exec.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        let mut argc: gint = 0;
        let mut argv: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        if g_shell_parse_argv(
            exec,
            &raw mut argc,
            &raw mut argv,
            ::core::ptr::null_mut::<*mut GError>(),
        ) == 0
        {
            g_free(path as gpointer);
            g_free(exec as gpointer);
            g_free(try_exec as gpointer);
            return FALSE;
        } else {
            let mut t_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if ({
                let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                if argc > 0 as ::core::ffi::c_int {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1941 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"argc > 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            t_0 = (*glib__private__())
                .g_find_program_for_path
                .expect("non-null function pointer")(
                *argv.offset(0 as ::core::ffi::c_int as isize),
                ::core::ptr::null::<::core::ffi::c_char>(),
                path,
            );
            g_strfreev(argv as *mut *mut gchar);
            if t_0.is_null() {
                g_free(path as gpointer);
                g_free(exec as gpointer);
                g_free(try_exec as gpointer);
                return FALSE;
            }
            g_free(t_0 as gpointer);
        }
    }
    (*info).name = g_key_file_get_locale_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_NAME.as_ptr() as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    (*info).generic_name = g_key_file_get_locale_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        GENERIC_NAME_KEY.as_ptr() as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    (*info).fullname = g_key_file_get_locale_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        FULL_NAME_KEY.as_ptr() as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    (*info).keywords = g_key_file_get_locale_string_list(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        KEYWORDS_KEY.as_ptr() as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut *mut ::core::ffi::c_char;
    (*info).comment = g_key_file_get_locale_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_COMMENT.as_ptr() as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    (*info).set_nodisplay(
        (g_key_file_get_boolean(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            G_KEY_FILE_DESKTOP_KEY_NO_DISPLAY.as_ptr() as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != FALSE) as ::core::ffi::c_int as guint as guint,
    );
    (*info).icon_name = g_key_file_get_locale_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_ICON.as_ptr() as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    (*info).only_show_in = g_key_file_get_string_list(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut *mut ::core::ffi::c_char;
    (*info).not_show_in = g_key_file_get_string_list(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut *mut ::core::ffi::c_char;
    (*info).try_exec = try_exec;
    (*info).exec = exec;
    (*info).path = safe_c2rust_g_steal_pointer(&raw mut path as gpointer)
        as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
    (*info).set_terminal(
        (g_key_file_get_boolean(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            G_KEY_FILE_DESKTOP_KEY_TERMINAL.as_ptr() as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != FALSE) as ::core::ffi::c_int as guint as guint,
    );
    (*info).set_startup_notify(
        (g_key_file_get_boolean(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            G_KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY.as_ptr() as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != FALSE) as ::core::ffi::c_int as guint as guint,
    );
    (*info).set_no_fuse(
        (g_key_file_get_boolean(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            b"X-GIO-NoFuse\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != FALSE) as ::core::ffi::c_int as guint as guint,
    );
    (*info).set_hidden(
        (g_key_file_get_boolean(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            G_KEY_FILE_DESKTOP_KEY_HIDDEN.as_ptr() as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        ) != FALSE) as ::core::ffi::c_int as guint as guint,
    );
    (*info).categories = g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_CATEGORIES.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    (*info).startup_wm_class = g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        STARTUP_WM_CLASS_KEY.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    (*info).mime_types = g_key_file_get_string_list(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_MIME_TYPE.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut *mut ::core::ffi::c_char;
    bus_activatable = g_key_file_get_boolean(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    (*info).actions = g_key_file_get_string_list(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_ACTIONS.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut *mut ::core::ffi::c_char;
    if (*info).actions.is_null() {
        (*info).actions = ({
            let mut __n: gsize = (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
        }) as *mut *mut gchar as *mut *mut ::core::ffi::c_char;
    }
    (*info).icon = ::core::ptr::null_mut::<GIcon>();
    if !(*info).icon_name.is_null() {
        if g_path_is_absolute((*info).icon_name) != 0 {
            let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
            file = g_file_new_for_path((*info).icon_name);
            (*info).icon = g_file_icon_new(file);
            g_object_unref(file as gpointer);
        } else {
            let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            p = strrchr((*info).icon_name, '.' as i32);
            if !p.is_null()
                && (strcmp(p, b".png\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(p, b".xpm\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    || strcmp(p, b".svg\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int)
            {
                *p = 0 as ::core::ffi::c_char;
            }
            (*info).icon = g_themed_icon_new((*info).icon_name);
        }
    }
    if !(*info).exec.is_null() {
        (*info).binary = safe_c2rust_binary_from_exec((*info).exec);
    }
    if !(*info).path.is_null()
        && *(*info).path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
    {
        g_free((*info).path as gpointer);
        (*info).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if bus_activatable != 0 && !(*info).filename.is_null() {
        let mut basename: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut last_dot: *mut gchar = ::core::ptr::null_mut::<gchar>();
        basename = g_path_get_basename((*info).filename);
        last_dot = strrchr(basename, '.' as i32) as *mut gchar;
        if !last_dot.is_null()
            && strcmp(
                last_dot as *const ::core::ffi::c_char,
                b".desktop\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            *last_dot = '\0' as i32 as gchar;
            if g_dbus_is_name(basename) != 0
                && *basename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != ':' as i32
            {
                (*info).app_id = safe_c2rust_g_strdup_inline(basename);
            }
        }
        g_free(basename as gpointer);
    }
    if !(*info).filename.is_null() {
        (*info).desktop_id = safe_c2rust_g_desktop_app_info_get_desktop_id_for_filename(info);
    }
    (*info).keyfile = g_key_file_ref(key_file);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_load_file(
    mut self_0: *mut GDesktopAppInfo,
) -> gboolean {
    let mut key_file: *mut GKeyFile = ::core::ptr::null_mut::<GKeyFile>();
    let mut retval: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !(*self_0).filename.is_null() {
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
            b"self->filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    key_file = g_key_file_new();
    if g_key_file_load_from_file(
        key_file,
        (*self_0).filename,
        G_KEY_FILE_NONE,
        ::core::ptr::null_mut::<*mut GError>(),
    ) != 0
    {
        retval = safe_c2rust_g_desktop_app_info_load_from_keyfile(self_0, key_file);
    }
    g_key_file_unref(key_file);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_new_from_keyfile(
    mut key_file: *mut GKeyFile,
) -> *mut GDesktopAppInfo {
    let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    info = g_object_new(
        safe_c2rust_g_desktop_app_info_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDesktopAppInfo;
    (*info).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    safe_c2rust_desktop_file_dirs_lock();
    if safe_c2rust_g_desktop_app_info_load_from_keyfile(info, key_file) == 0 {
        let mut _pp: *mut *mut GDesktopAppInfo = &raw mut info;
        let mut _ptr: *mut GDesktopAppInfo = *_pp;
        *_pp = ::core::ptr::null_mut::<GDesktopAppInfo>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    }
    safe_c2rust_desktop_file_dirs_unlock();
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_new_from_filename(
    mut filename: *const ::core::ffi::c_char,
) -> *mut GDesktopAppInfo {
    let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    safe_c2rust_desktop_file_dirs_lock();
    info = safe_c2rust_g_desktop_app_info_new_from_filename_unlocked(filename);
    safe_c2rust_desktop_file_dirs_unlock();
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_new(
    mut desktop_id: *const ::core::ffi::c_char,
) -> *mut GDesktopAppInfo {
    let mut appinfo: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    let mut i: guint = 0;
    safe_c2rust_desktop_file_dirs_lock();
    i = 0 as guint;
    while i < (*safe_c2rust_desktop_file_dirs).len {
        appinfo = safe_c2rust_desktop_file_dir_get_app(
            *(*safe_c2rust_desktop_file_dirs).pdata.offset(i as isize) as *mut DesktopFileDir,
            desktop_id as *const gchar,
        );
        if !appinfo.is_null() {
            break;
        }
        i = i.wrapping_add(1);
    }
    safe_c2rust_desktop_file_dirs_unlock();
    if appinfo.is_null() {
        return ::core::ptr::null_mut::<GDesktopAppInfo>();
    }
    g_free((*appinfo).desktop_id as gpointer);
    (*appinfo).desktop_id = safe_c2rust_g_strdup_inline(desktop_id);
    if safe_c2rust_g_desktop_app_info_get_is_hidden(appinfo) != 0 {
        g_object_unref(appinfo as gpointer);
        appinfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    }
    return appinfo;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_dup(
    mut appinfo: *mut GAppInfo,
) -> *mut GAppInfo {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    let mut new_info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    new_info = g_object_new(
        safe_c2rust_g_desktop_app_info_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDesktopAppInfo;
    (*new_info).filename = safe_c2rust_g_strdup_inline((*info).filename);
    (*new_info).desktop_id = safe_c2rust_g_strdup_inline((*info).desktop_id);
    if !(*info).keyfile.is_null() {
        (*new_info).keyfile = g_key_file_ref((*info).keyfile);
    }
    (*new_info).name = safe_c2rust_g_strdup_inline((*info).name);
    (*new_info).generic_name = safe_c2rust_g_strdup_inline((*info).generic_name);
    (*new_info).fullname = safe_c2rust_g_strdup_inline((*info).fullname);
    (*new_info).keywords =
        g_strdupv((*info).keywords as *mut *mut gchar) as *mut *mut ::core::ffi::c_char;
    (*new_info).comment = safe_c2rust_g_strdup_inline((*info).comment);
    (*new_info).set_nodisplay((*info).nodisplay() as guint);
    (*new_info).icon_name = safe_c2rust_g_strdup_inline((*info).icon_name);
    if !(*info).icon.is_null() {
        (*new_info).icon = g_object_ref((*info).icon as gpointer) as *mut GIcon as *mut GIcon;
    }
    (*new_info).only_show_in =
        g_strdupv((*info).only_show_in as *mut *mut gchar) as *mut *mut ::core::ffi::c_char;
    (*new_info).not_show_in =
        g_strdupv((*info).not_show_in as *mut *mut gchar) as *mut *mut ::core::ffi::c_char;
    (*new_info).try_exec = safe_c2rust_g_strdup_inline((*info).try_exec);
    (*new_info).exec = safe_c2rust_g_strdup_inline((*info).exec);
    (*new_info).binary = safe_c2rust_g_strdup_inline((*info).binary);
    (*new_info).path = safe_c2rust_g_strdup_inline((*info).path);
    (*new_info).app_id = safe_c2rust_g_strdup_inline((*info).app_id);
    (*new_info).set_hidden((*info).hidden() as guint);
    (*new_info).set_terminal((*info).terminal() as guint);
    (*new_info).set_startup_notify((*info).startup_notify() as guint);
    return new_info as *mut ::core::ffi::c_void as *mut GAppInfo;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_equal(
    mut appinfo1: *mut GAppInfo,
    mut appinfo2: *mut GAppInfo,
) -> gboolean {
    let mut info1: *mut GDesktopAppInfo =
        appinfo1 as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    let mut info2: *mut GDesktopAppInfo =
        appinfo2 as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if (*info1).desktop_id.is_null() || (*info2).desktop_id.is_null() {
        return (info1 == info2) as ::core::ffi::c_int;
    }
    return (strcmp((*info1).desktop_id, (*info2).desktop_id) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_id(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (*info).desktop_id;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_name(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if (*info).name.is_null() {
        return glib_gettext(b"Unnamed\0" as *const u8 as *const gchar)
            as *const ::core::ffi::c_char;
    }
    return (*info).name;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_display_name(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if (*info).fullname.is_null() {
        return safe_c2rust_g_desktop_app_info_get_name(appinfo);
    }
    return (*info).fullname;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_is_hidden(
    mut info: *mut GDesktopAppInfo,
) -> gboolean {
    return (*info).hidden() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_filename(
    mut info: *mut GDesktopAppInfo,
) -> *const ::core::ffi::c_char {
    return (*info).filename;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_description(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (*info).comment;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_executable(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (*info).binary;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_commandline(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (*info).exec;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_icon(
    mut appinfo: *mut GAppInfo,
) -> *mut GIcon {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (*info).icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_categories(
    mut info: *mut GDesktopAppInfo,
) -> *const ::core::ffi::c_char {
    return (*info).categories;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_keywords(
    mut info: *mut GDesktopAppInfo,
) -> *const *const ::core::ffi::c_char {
    return (*info).keywords as *const *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_generic_name(
    mut info: *mut GDesktopAppInfo,
) -> *const ::core::ffi::c_char {
    return (*info).generic_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_nodisplay(
    mut info: *mut GDesktopAppInfo,
) -> gboolean {
    return (*info).nodisplay() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_show_in(
    mut info: *mut GDesktopAppInfo,
    mut desktop_env: *const gchar,
) -> gboolean {
    let mut specified_envs: [*const gchar; 2] = [desktop_env, ::core::ptr::null::<gchar>()];
    let mut envs: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !desktop_env.is_null() {
        envs = &raw mut specified_envs as *mut *const gchar;
    } else {
        envs = safe_c2rust_get_current_desktops(::core::ptr::null::<gchar>());
    }
    i = 0 as ::core::ffi::c_int as gint;
    while !(*envs.offset(i as isize)).is_null() {
        let mut j: gint = 0;
        if !(*info).only_show_in.is_null() {
            j = 0 as ::core::ffi::c_int as gint;
            while !(*(*info).only_show_in.offset(j as isize)).is_null() {
                if strcmp(
                    *(*info).only_show_in.offset(j as isize) as *const ::core::ffi::c_char,
                    *envs.offset(i as isize) as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    return TRUE;
                }
                j += 1;
            }
        }
        if !(*info).not_show_in.is_null() {
            j = 0 as ::core::ffi::c_int as gint;
            while !(*(*info).not_show_in.offset(j as isize)).is_null() {
                if strcmp(
                    *(*info).not_show_in.offset(j as isize) as *const ::core::ffi::c_char,
                    *envs.offset(i as isize) as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    return FALSE;
                }
                j += 1;
            }
        }
        i += 1;
    }
    return ((*info).only_show_in == NULL_1 as *mut *mut ::core::ffi::c_char) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_expand_macro_single(
    mut macro_0: ::core::ffi::c_char,
    mut uri: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    file = g_file_new_for_uri(uri);
    match macro_0 as ::core::ffi::c_int {
        117 | 85 => {
            result = g_shell_quote(uri as *const gchar) as *mut ::core::ffi::c_char;
        }
        102 | 70 => {
            path = g_file_get_path(file);
            if !path.is_null() {
                result = g_shell_quote(path) as *mut ::core::ffi::c_char;
            }
        }
        100 | 68 => {
            path = g_file_get_path(file);
            if !path.is_null() {
                name = g_path_get_dirname(path) as *mut ::core::ffi::c_char;
                result = g_shell_quote(name) as *mut ::core::ffi::c_char;
                g_free(name as gpointer);
            }
        }
        110 | 78 => {
            path = g_file_get_path(file);
            if !path.is_null() {
                name = g_path_get_basename(path) as *mut ::core::ffi::c_char;
                result = g_shell_quote(name) as *mut ::core::ffi::c_char;
                g_free(name as gpointer);
            }
        }
        _ => {}
    }
    g_object_unref(file as gpointer);
    g_free(path as gpointer);
    return result;
}
unsafe extern "C" fn safe_c2rust_expand_macro_uri(
    mut macro_0: ::core::ffi::c_char,
    mut uri: *const ::core::ffi::c_char,
    mut force_file_uri: gboolean,
    mut force_file_uri_macro: ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut expanded: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !uri.is_null() {
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
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if force_file_uri == 0 || !strchr(uri, '#' as i32).is_null() {
        expanded = safe_c2rust_expand_macro_single(macro_0, uri);
    } else {
        expanded = safe_c2rust_expand_macro_single(force_file_uri_macro, uri);
        if expanded.is_null() {
            expanded = safe_c2rust_expand_macro_single(macro_0, uri);
        }
    }
    return expanded;
}
unsafe extern "C" fn safe_c2rust_expand_macro(
    mut macro_0: ::core::ffi::c_char,
    mut exec: *mut GString,
    mut info: *mut GDesktopAppInfo,
    mut uri_list: *mut *mut GList,
) {
    let mut uris: *mut GList = *uri_list;
    let mut expanded: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut force_file_uri: gboolean = 0;
    let mut force_file_uri_macro: ::core::ffi::c_char = 0;
    let mut uri: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !exec.is_null() {
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
            b"exec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    force_file_uri_macro = macro_0;
    force_file_uri = FALSE as gboolean;
    if (*info).no_fuse() == 0 {
        match macro_0 as ::core::ffi::c_int {
            117 => {
                force_file_uri_macro = 'f' as i32 as ::core::ffi::c_char;
                force_file_uri = TRUE as gboolean;
            }
            85 => {
                force_file_uri_macro = 'F' as i32 as ::core::ffi::c_char;
                force_file_uri = TRUE as gboolean;
            }
            _ => {}
        }
    }
    match macro_0 as ::core::ffi::c_int {
        117 | 102 | 100 | 110 => {
            if !uris.is_null() {
                uri = (*uris).data as *const ::core::ffi::c_char;
                expanded = safe_c2rust_expand_macro_uri(
                    macro_0,
                    uri,
                    force_file_uri,
                    force_file_uri_macro,
                );
                if !expanded.is_null() {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char = expanded;
                            safe_c2rust_g_string_append_len_inline(
                                exec,
                                __val,
                                if ({
                                    let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_27
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            exec,
                            expanded,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    g_free(expanded as gpointer);
                }
                uris = (*uris).next;
            }
        }
        85 | 70 | 68 | 78 => {
            while !uris.is_null() {
                uri = (*uris).data as *const ::core::ffi::c_char;
                expanded = safe_c2rust_expand_macro_uri(
                    macro_0,
                    uri,
                    force_file_uri,
                    force_file_uri_macro,
                );
                if !expanded.is_null() {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char = expanded;
                            safe_c2rust_g_string_append_len_inline(
                                exec,
                                __val,
                                if ({
                                    let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_28 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_28 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_28
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            exec,
                            expanded,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    g_free(expanded as gpointer);
                }
                uris = (*uris).next;
                if !uris.is_null() && !expanded.is_null() {
                    safe_c2rust_g_string_append_c_inline(exec, ' ' as i32 as gchar);
                }
            }
        }
        105 => {
            if !(*info).icon_name.is_null() {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"--icon \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            exec,
                            __val,
                            if ({
                                let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_29 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_29 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_29
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
                        exec,
                        b"--icon \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                expanded = g_shell_quote((*info).icon_name) as *mut ::core::ffi::c_char;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = expanded;
                        safe_c2rust_g_string_append_len_inline(
                            exec,
                            __val,
                            if ({
                                let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_30 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_30 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_30
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
                        exec,
                        expanded,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                g_free(expanded as gpointer);
            }
        }
        99 => {
            if !(*info).name.is_null() {
                expanded = g_shell_quote((*info).name) as *mut ::core::ffi::c_char;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = expanded;
                        safe_c2rust_g_string_append_len_inline(
                            exec,
                            __val,
                            if ({
                                let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_31 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_31 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_31
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
                        exec,
                        expanded,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                g_free(expanded as gpointer);
            }
        }
        107 => {
            if !(*info).filename.is_null() {
                expanded = g_shell_quote((*info).filename) as *mut ::core::ffi::c_char;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = expanded;
                        safe_c2rust_g_string_append_len_inline(
                            exec,
                            __val,
                            if ({
                                let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_32 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_32 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_32
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
                        exec,
                        expanded,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                g_free(expanded as gpointer);
            }
        }
        37 => {
            safe_c2rust_g_string_append_c_inline(exec, '%' as i32 as gchar);
        }
        109 | _ => {}
    }
    *uri_list = uris;
}
unsafe extern "C" fn safe_c2rust_expand_application_parameters(
    mut info: *mut GDesktopAppInfo,
    mut exec_line: *const gchar,
    mut uris: *mut *mut GList,
    mut argc: *mut ::core::ffi::c_int,
    mut argv: *mut *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut uri_list: *mut GList = *uris;
    let mut p: *const ::core::ffi::c_char = exec_line as *const ::core::ffi::c_char;
    let mut expanded_exec: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut res: gboolean = 0;
    if exec_line.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Desktop file didn\xE2\x80\x99t specify Exec field\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    expanded_exec = g_string_new(::core::ptr::null::<gchar>());
    while *p != 0 {
        if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '%' as i32
            && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        {
            safe_c2rust_expand_macro(
                *p.offset(1 as ::core::ffi::c_int as isize),
                expanded_exec,
                info,
                uris,
            );
            p = p.offset(1);
        } else {
            safe_c2rust_g_string_append_c_inline(expanded_exec, *p);
        }
        p = p.offset(1);
    }
    if uri_list == *uris && !uri_list.is_null() {
        safe_c2rust_g_string_append_c_inline(expanded_exec, ' ' as i32 as gchar);
        safe_c2rust_expand_macro('f' as i32 as ::core::ffi::c_char, expanded_exec, info, uris);
    }
    res = g_shell_parse_argv(
        (*expanded_exec).str_0,
        argc as *mut gint,
        argv as *mut *mut *mut gchar,
        error,
    );
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                expanded_exec,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(expanded_exec);
        };
    } else {
        g_string_free(
            expanded_exec,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
    return res;
}
unsafe extern "C" fn safe_c2rust_prepend_terminal_to_vector(
    mut argc: *mut ::core::ffi::c_int,
    mut argv: *mut *mut *mut ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut working_dir: *const ::core::ffi::c_char,
) -> gboolean {
    let mut real_argv: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut real_argc: size_t = 0;
    let mut i: size_t = 0;
    let mut term_argc: size_t = 0;
    let mut found_terminal: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut the_argv: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut term_arg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    static mut safe_c2rust_known_terminals: [C2RustUnnamed_6; 13] = [
        C2RustUnnamed_6 {
            exec: b"xdg-terminal-exec\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        C2RustUnnamed_6 {
            exec: b"kgx\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"gnome-terminal\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"--\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"mate-terminal\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-x\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"xfce4-terminal\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-x\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"tilix\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"konsole\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"x-terminal-emulator\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"nxterm\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"color-xterm\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"rxvt\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"dtterm\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
        C2RustUnnamed_6 {
            exec: b"xterm\0" as *const u8 as *const ::core::ffi::c_char,
            exec_arg: b"-e\0" as *const u8 as *const ::core::ffi::c_char,
        },
    ];
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !argc.is_null() {
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
            b"argc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !argv.is_null() {
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
            b"argv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*argv).is_null() {
        *argc = 0 as ::core::ffi::c_int;
    }
    the_argv = *argv;
    if *argc < 0 as ::core::ffi::c_int {
        *argc = 0 as ::core::ffi::c_int;
        while !(*the_argv.offset(*argc as isize)).is_null() {
            *argc += 1;
        }
    }
    i = 0 as size_t;
    found_terminal = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while i
        < (::core::mem::size_of::<[C2RustUnnamed_6; 13]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_6>() as usize)
    {
        found_terminal = (*glib__private__())
            .g_find_program_for_path
            .expect("non-null function pointer")(
            safe_c2rust_known_terminals[i as usize].exec,
            path,
            working_dir,
        );
        if !found_terminal.is_null() {
            term_arg = safe_c2rust_known_terminals[i as usize].exec_arg;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if found_terminal.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"Couldn\xE2\x80\x99t find a known terminal\0" as *const u8 as *const gchar,
        );
        return FALSE;
    }
    term_argc = (if !term_arg.is_null() {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as size_t;
    real_argc = term_argc.wrapping_add(*argc as size_t);
    real_argv = ({
        let mut __n: gsize = real_argc.wrapping_add(1 as size_t) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut ::core::ffi::c_char;
    i = 0 as size_t;
    let fresh21 = i;
    i = i.wrapping_add(1);
    let ref mut fresh22 = *real_argv.offset(fresh21 as isize);
    *fresh22 = found_terminal;
    if !term_arg.is_null() {
        let fresh23 = i;
        i = i.wrapping_add(1);
        let ref mut fresh24 = *real_argv.offset(fresh23 as isize);
        *fresh24 = safe_c2rust_g_strdup_inline(term_arg);
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if i == term_argc {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2755 as ::core::ffi::c_int,
            G_STRFUNC,
            b"i == term_argc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < *argc {
        let fresh25 = i;
        i = i.wrapping_add(1);
        let ref mut fresh26 = *real_argv.offset(fresh25 as isize);
        *fresh26 = *the_argv.offset(j as isize);
        j += 1;
    }
    let ref mut fresh27 = *real_argv.offset(i as isize);
    *fresh27 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_free(*argv as gpointer);
    *argv = real_argv;
    *argc = real_argc as ::core::ffi::c_int;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_create_files_for_uris(mut uris: *mut GList) -> *mut GList {
    let mut res: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut iter: *mut GList = ::core::ptr::null_mut::<GList>();
    res = ::core::ptr::null_mut::<GList>();
    iter = uris;
    while !iter.is_null() {
        let mut file: *mut GFile = g_file_new_for_uri((*iter).data as *mut ::core::ffi::c_char);
        res = g_list_prepend(res, file as gpointer);
        iter = (*iter).next;
    }
    return g_list_reverse(res);
}
unsafe extern "C" fn safe_c2rust_notify_desktop_launch(
    mut session_bus: *mut GDBusConnection,
    mut info: *mut GDesktopAppInfo,
    mut pid: ::core::ffi::c_long,
    mut display: *const ::core::ffi::c_char,
    mut sn_id: *const ::core::ffi::c_char,
    mut uris: *mut GList,
) {
    let mut msg: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut uri_variant: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut extras_variant: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut iter: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut desktop_file_id: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut gio_desktop_file: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    if session_bus.is_null() {
        return;
    }
    g_variant_builder_init(
        &raw mut uri_variant,
        g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
    );
    iter = uris;
    while !iter.is_null() {
        g_variant_builder_add(
            &raw mut uri_variant,
            b"s\0" as *const u8 as *const gchar,
            (*iter).data,
        );
        iter = (*iter).next;
    }
    g_variant_builder_init(
        &raw mut extras_variant,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    if !sn_id.is_null()
        && g_utf8_validate(
            sn_id as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
    {
        g_variant_builder_add(
            &raw mut extras_variant,
            b"{sv}\0" as *const u8 as *const gchar,
            b"startup-id\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new(b"s\0" as *const u8 as *const gchar, sn_id),
        );
    }
    gio_desktop_file = g_getenv(b"GIO_LAUNCHED_DESKTOP_FILE\0" as *const u8 as *const gchar)
        as *const ::core::ffi::c_char;
    if !gio_desktop_file.is_null() {
        g_variant_builder_add(
            &raw mut extras_variant,
            b"{sv}\0" as *const u8 as *const gchar,
            b"origin-desktop-file\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_bytestring(gio_desktop_file as *const gchar),
        );
    }
    if !g_get_prgname().is_null() {
        g_variant_builder_add(
            &raw mut extras_variant,
            b"{sv}\0" as *const u8 as *const gchar,
            b"origin-prgname\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_bytestring(g_get_prgname()),
        );
    }
    g_variant_builder_add(
        &raw mut extras_variant,
        b"{sv}\0" as *const u8 as *const gchar,
        b"origin-pid\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new(b"x\0" as *const u8 as *const gchar, getpid() as gint64),
    );
    if !(*info).filename.is_null() {
        desktop_file_id = (*info).filename;
    } else if !(*info).desktop_id.is_null() {
        desktop_file_id = (*info).desktop_id;
    } else {
        desktop_file_id = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    msg = g_dbus_message_new_signal(
        b"/org/gtk/gio/DesktopAppInfo\0" as *const u8 as *const gchar,
        b"org.gtk.gio.DesktopAppInfo\0" as *const u8 as *const gchar,
        b"Launched\0" as *const u8 as *const gchar,
    );
    g_dbus_message_set_body(
        msg,
        g_variant_new(
            b"(@aysxasa{sv})\0" as *const u8 as *const gchar,
            g_variant_new_bytestring(desktop_file_id as *const gchar),
            if !display.is_null() {
                display
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            pid,
            &raw mut uri_variant,
            &raw mut extras_variant,
        ),
    );
    g_dbus_connection_send_message(
        session_bus,
        msg,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(msg as gpointer);
}
unsafe extern "C" fn safe_c2rust_emit_launch_started(
    mut context: *mut GAppLaunchContext,
    mut info: *mut GDesktopAppInfo,
    mut startup_id: *const gchar,
) {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !startup_id.is_null() {
        g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_ARRAY);
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"startup-notification-id\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_string(startup_id),
        );
        platform_data = g_variant_ref_sink(g_variant_builder_end(&raw mut builder));
    }
    g_signal_emit_by_name(
        context as gpointer,
        b"launch-started\0" as *const u8 as *const gchar,
        info,
        platform_data,
    );
    let mut _pp: *mut *mut GVariant = &raw mut platform_data;
    let mut _ptr: *mut GVariant = *_pp;
    *_pp = ::core::ptr::null_mut::<GVariant>();
    if !_ptr.is_null() {
        g_variant_unref(_ptr as *mut GVariant);
    }
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris_with_spawn(
    mut info: *mut GDesktopAppInfo,
    mut session_bus: *mut GDBusConnection,
    mut exec_line: *const gchar,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut spawn_flags: GSpawnFlags,
    mut user_setup: GSpawnChildSetupFunc,
    mut user_setup_data: gpointer,
    mut pid_callback: GDesktopAppLaunchCallback,
    mut pid_callback_data: gpointer,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut completed: gboolean = FALSE;
    let mut old_uris: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut dup_uris: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut argv: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut envp: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut argc: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !launch_context.is_null() {
        envp = g_app_launch_context_get_environment(launch_context);
    } else {
        envp = g_get_environ() as *mut *mut ::core::ffi::c_char;
    }
    dup_uris = uris;
    loop {
        let mut pid: GPid = 0;
        let mut launched_uris: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut iter: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut sn_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut wrapped_argv: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut i: ::core::ffi::c_int = 0;
        old_uris = dup_uris;
        if safe_c2rust_expand_application_parameters(
            info,
            exec_line,
            &raw mut dup_uris,
            &raw mut argc,
            &raw mut argv,
            error,
        ) == 0
        {
            current_block = 14250230629648704018;
            break;
        }
        launched_uris = ::core::ptr::null_mut::<GList>();
        iter = old_uris;
        while !iter.is_null() && iter != dup_uris {
            launched_uris = g_list_prepend(launched_uris, (*iter).data);
            iter = (*iter).next;
        }
        launched_uris = g_list_reverse(launched_uris);
        if (*info).terminal() as ::core::ffi::c_int != 0
            && safe_c2rust_prepend_terminal_to_vector(
                &raw mut argc,
                &raw mut argv,
                g_environ_getenv(
                    envp as *mut *mut gchar,
                    b"PATH\0" as *const u8 as *const gchar,
                ) as *const ::core::ffi::c_char,
                (*info).path,
            ) == 0
        {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unable to find terminal required for application\0" as *const u8
                        as *const gchar,
                ),
            );
            current_block = 14250230629648704018;
            break;
        } else {
            if !(*info).filename.is_null() {
                envp = g_environ_setenv(
                    envp as *mut *mut gchar,
                    b"GIO_LAUNCHED_DESKTOP_FILE\0" as *const u8 as *const gchar,
                    (*info).filename,
                    TRUE,
                ) as *mut *mut ::core::ffi::c_char;
            }
            sn_id = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !launch_context.is_null() {
                let mut launched_files: *mut GList =
                    safe_c2rust_create_files_for_uris(launched_uris);
                if (*info).startup_notify() != 0 {
                    sn_id = g_app_launch_context_get_startup_notify_id(
                        launch_context,
                        info as *mut ::core::ffi::c_void as *mut GAppInfo,
                        launched_files,
                    );
                    if !sn_id.is_null() {
                        envp = g_environ_setenv(
                            envp as *mut *mut gchar,
                            b"DESKTOP_STARTUP_ID\0" as *const u8 as *const gchar,
                            sn_id,
                            TRUE,
                        ) as *mut *mut ::core::ffi::c_char;
                        envp = g_environ_setenv(
                            envp as *mut *mut gchar,
                            b"XDG_ACTIVATION_TOKEN\0" as *const u8 as *const gchar,
                            sn_id,
                            TRUE,
                        ) as *mut *mut ::core::ffi::c_char;
                    }
                }
                g_list_free_full(
                    launched_files,
                    Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
                );
                safe_c2rust_emit_launch_started(launch_context, info, sn_id);
            }
            if ({
                let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
                if argc > 0 as ::core::ffi::c_int {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2968 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"argc > 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if g_path_is_absolute(*argv.offset(0 as ::core::ffi::c_int as isize)) == 0
                || g_file_test(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    G_FILE_TEST_IS_EXECUTABLE,
                ) == 0
                || g_file_test(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    G_FILE_TEST_IS_DIR,
                ) != 0
            {
                let mut program: *mut ::core::ffi::c_char =
                    safe_c2rust_g_steal_pointer(argv.offset(0 as ::core::ffi::c_int as isize)
                        as *mut *mut ::core::ffi::c_char
                        as gpointer) as *mut ::core::ffi::c_char;
                let mut program_path: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if g_path_is_absolute(program) == 0 {
                    let mut env_path: *const ::core::ffi::c_char = g_environ_getenv(
                        envp as *mut *mut gchar,
                        b"PATH\0" as *const u8 as *const gchar,
                    )
                        as *const ::core::ffi::c_char;
                    program_path = (*glib__private__())
                        .g_find_program_for_path
                        .expect("non-null function pointer")(
                        program, env_path, (*info).path
                    );
                }
                if !program_path.is_null() {
                    let ref mut fresh17 = *argv.offset(0 as ::core::ffi::c_int as isize);
                    *fresh17 = safe_c2rust_g_steal_pointer(&raw mut program_path as gpointer)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    g_free(program as gpointer);
                } else {
                    if !sn_id.is_null() {
                        g_app_launch_context_launch_failed(launch_context, sn_id);
                    }
                    g_set_error(
                        error,
                        g_spawn_error_quark(),
                        G_SPAWN_ERROR_NOENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Program \xE2\x80\x98%s\xE2\x80\x99 not found in $PATH\0" as *const u8
                                as *const gchar,
                        ),
                        program,
                    );
                    g_free(program as gpointer);
                    let mut _pp: *mut *mut ::core::ffi::c_char = &raw mut sn_id;
                    let mut _ptr: *mut ::core::ffi::c_char = *_pp;
                    *_pp = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if !_ptr.is_null() {
                        g_free(_ptr as gpointer);
                    }
                    let mut _list: *mut GList = ::core::ptr::null_mut::<GList>();
                    _list = launched_uris;
                    if !_list.is_null() {
                        launched_uris = ::core::ptr::null_mut::<GList>();
                        if !NULL_1.is_null() {
                            g_list_free_full(_list, None);
                        } else {
                            g_list_free(_list);
                        }
                    }
                    current_block = 14250230629648704018;
                    break;
                }
            }
            if ({
                if 0 as ::core::ffi::c_int != 0 {
                    safe_c2rust_gio_launch_desktop_path;
                } else {
                };
                (({
                    let mut gapg_temp_newval: *const gchar = ::core::ptr::null::<gchar>();
                    let mut gapg_temp_atomic: *mut *const gchar =
                        &raw mut safe_c2rust_gio_launch_desktop_path;
                    *&raw mut gapg_temp_newval =
                        crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                    gapg_temp_newval
                })
                .is_null()
                    && g_once_init_enter_pointer(
                        &raw mut safe_c2rust_gio_launch_desktop_path as *mut ::core::ffi::c_void,
                    ) != 0) as ::core::ffi::c_int
            }) != 0
            {
                let mut tmp: *const gchar = ::core::ptr::null::<gchar>();
                let mut is_setuid: gboolean = (*glib__private__())
                    .g_check_setuid
                    .expect("non-null function pointer")(
                );
                if is_setuid == 0 {
                    tmp = g_getenv(b"GIO_LAUNCH_DESKTOP\0" as *const u8 as *const gchar);
                }
                if tmp.is_null()
                    && g_file_test(
                        GIO_LAUNCH_DESKTOP.as_ptr() as *const gchar,
                        G_FILE_TEST_IS_EXECUTABLE,
                    ) != 0
                {
                    tmp = GIO_LAUNCH_DESKTOP.as_ptr() as *const gchar;
                }
                if tmp.is_null() {
                    tmp = b"gio-launch-desktop\0" as *const u8 as *const ::core::ffi::c_char
                        as *const gchar;
                }
                if 0 as ::core::ffi::c_int != 0 {
                    safe_c2rust_gio_launch_desktop_path = tmp;
                } else {
                };
                g_once_init_leave_pointer(
                    &raw mut safe_c2rust_gio_launch_desktop_path as *mut ::core::ffi::c_void,
                    tmp as guintptr as gpointer,
                );
            }
            wrapped_argv = ({
                let mut __n: gsize = (argc + 2 as ::core::ffi::c_int) as gsize;
                let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc_n(__n, __s);
                }
                __p
            }) as *mut *mut ::core::ffi::c_char;
            let ref mut fresh18 = *wrapped_argv.offset(0 as ::core::ffi::c_int as isize);
            *fresh18 = safe_c2rust_g_strdup_inline(
                safe_c2rust_gio_launch_desktop_path as *const ::core::ffi::c_char,
            );
            i = 0 as ::core::ffi::c_int;
            while i < argc {
                let ref mut fresh19 = *wrapped_argv.offset((i + 1 as ::core::ffi::c_int) as isize);
                *fresh19 = safe_c2rust_g_steal_pointer(argv.offset(i as isize)
                    as *mut *mut ::core::ffi::c_char
                    as gpointer) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                i += 1;
            }
            let ref mut fresh20 = *wrapped_argv.offset((i + 1 as ::core::ffi::c_int) as isize);
            *fresh20 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            g_free(argv as gpointer);
            argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            if g_spawn_async_with_fds(
                (*info).path,
                wrapped_argv as *mut *mut gchar,
                envp as *mut *mut gchar,
                spawn_flags,
                user_setup,
                user_setup_data,
                &raw mut pid,
                stdin_fd,
                stdout_fd,
                stderr_fd,
                error,
            ) == 0
            {
                if !sn_id.is_null() {
                    g_app_launch_context_launch_failed(launch_context, sn_id);
                }
                g_free(sn_id as gpointer);
                g_list_free(launched_uris);
                let mut _pp_0: *mut *mut *mut ::core::ffi::c_char = &raw mut wrapped_argv;
                let mut _ptr_0: *mut *mut ::core::ffi::c_char = *_pp_0;
                *_pp_0 = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                if !_ptr_0.is_null() {
                    g_strfreev(_ptr_0 as *mut *mut gchar);
                }
                current_block = 14250230629648704018;
                break;
            } else {
                if pid_callback.is_some() {
                    pid_callback.expect("non-null function pointer")(info, pid, pid_callback_data);
                }
                if !launch_context.is_null() {
                    let mut builder: GVariantBuilder = _GVariantBuilder {
                        u: C2RustUnnamed_0 {
                            s: C2RustUnnamed_1 {
                                partial_magic: 0,
                                type_0: ::core::ptr::null::<GVariantType>(),
                                y: [0; 14],
                            },
                        },
                    };
                    let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                    g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_ARRAY);
                    g_variant_builder_add(
                        &raw mut builder,
                        b"{sv}\0" as *const u8 as *const gchar,
                        b"pid\0" as *const u8 as *const ::core::ffi::c_char,
                        g_variant_new_int32(pid as gint32),
                    );
                    if !sn_id.is_null() {
                        g_variant_builder_add(
                            &raw mut builder,
                            b"{sv}\0" as *const u8 as *const gchar,
                            b"startup-notification-id\0" as *const u8 as *const ::core::ffi::c_char,
                            g_variant_new_string(sn_id),
                        );
                    }
                    platform_data = g_variant_ref_sink(g_variant_builder_end(&raw mut builder));
                    g_signal_emit_by_name(
                        launch_context as gpointer,
                        b"launched\0" as *const u8 as *const gchar,
                        info,
                        platform_data,
                    );
                    g_variant_unref(platform_data);
                }
                safe_c2rust_notify_desktop_launch(
                    session_bus,
                    info,
                    pid as ::core::ffi::c_long,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    sn_id,
                    launched_uris,
                );
                g_free(sn_id as gpointer);
                g_list_free(launched_uris);
                g_strfreev(wrapped_argv as *mut *mut gchar);
                wrapped_argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                if dup_uris.is_null() {
                    current_block = 1765866445182206997;
                    break;
                }
            }
        }
    }
    match current_block {
        1765866445182206997 => {
            completed = TRUE as gboolean;
        }
        _ => {}
    }
    g_strfreev(argv as *mut *mut gchar);
    g_strfreev(envp as *mut *mut gchar);
    return completed;
}
unsafe extern "C" fn safe_c2rust_object_path_from_appid(mut appid: *const gchar) -> *mut gchar {
    let mut appid_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut iter: *mut gchar = ::core::ptr::null_mut::<gchar>();
    appid_path = g_strconcat(b"/\0" as *const u8 as *const gchar, appid, NULL_1);
    iter = appid_path;
    while *iter != 0 {
        if *iter as ::core::ffi::c_int == '.' as i32 {
            *iter = '/' as i32 as gchar;
        }
        if *iter as ::core::ffi::c_int == '-' as i32 {
            *iter = '_' as i32 as gchar;
        }
        iter = iter.offset(1);
    }
    return appid_path;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_make_platform_data(
    mut info: *mut GDesktopAppInfo,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
) -> *mut GVariant {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_VARDICT);
    if !launch_context.is_null() {
        let mut launched_files: *mut GList = safe_c2rust_create_files_for_uris(uris);
        if (*info).startup_notify() != 0 {
            let mut sn_id: *mut gchar = ::core::ptr::null_mut::<gchar>();
            sn_id = g_app_launch_context_get_startup_notify_id(
                launch_context,
                info as *mut ::core::ffi::c_void as *mut GAppInfo,
                launched_files,
            ) as *mut gchar;
            if !sn_id.is_null() {
                g_variant_builder_add(
                    &raw mut builder,
                    b"{sv}\0" as *const u8 as *const gchar,
                    b"desktop-startup-id\0" as *const u8 as *const ::core::ffi::c_char,
                    g_variant_new_string(sn_id),
                );
                g_variant_builder_add(
                    &raw mut builder,
                    b"{sv}\0" as *const u8 as *const gchar,
                    b"activation-token\0" as *const u8 as *const ::core::ffi::c_char,
                    g_variant_new_take_string(safe_c2rust_g_steal_pointer(
                        &raw mut sn_id as gpointer,
                    ) as *mut gchar),
                );
            }
        }
        g_list_free_full(
            launched_files,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_launch_uris_with_dbus_data_free(
    mut data: *mut LaunchUrisWithDBusData,
) {
    let mut _pp: *mut *mut GDesktopAppInfo = &raw mut (*data).info;
    let mut _ptr: *mut GDesktopAppInfo = *_pp;
    *_pp = ::core::ptr::null_mut::<GDesktopAppInfo>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GAppLaunchContext = &raw mut (*data).launch_context;
    let mut _ptr_0: *mut GAppLaunchContext = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GAppLaunchContext>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    g_free((*data).startup_id as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_uris_with_dbus_signal_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut LaunchUrisWithDBusData = user_data as *mut LaunchUrisWithDBusData;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    if !(*data).launch_context.is_null() {
        if g_task_had_error(result as *mut ::core::ffi::c_void as *mut GTask) != 0 {
            if !(*data).startup_id.is_null() {
                g_app_launch_context_launch_failed((*data).launch_context, (*data).startup_id);
            }
        } else {
            let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_ARRAY);
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                b"pid\0" as *const u8 as *const ::core::ffi::c_char,
                g_variant_new_int32(0 as gint32),
            );
            if !(*data).startup_id.is_null() {
                g_variant_builder_add(
                    &raw mut builder,
                    b"{sv}\0" as *const u8 as *const gchar,
                    b"startup-notification-id\0" as *const u8 as *const ::core::ffi::c_char,
                    g_variant_new_string((*data).startup_id),
                );
            }
            platform_data = g_variant_ref_sink(g_variant_builder_end(&raw mut builder));
            g_signal_emit_by_name(
                (*data).launch_context as gpointer,
                b"launched\0" as *const u8 as *const gchar,
                (*data).info,
                platform_data,
            );
            g_variant_unref(platform_data);
        }
    }
    if (*data).callback.is_some() {
        (*data).callback.expect("non-null function pointer")(object, result, (*data).user_data);
    } else if g_task_had_error(result as *mut ::core::ffi::c_void as *mut GTask) == 0 {
        g_variant_unref(g_dbus_connection_call_finish(
            object as *mut ::core::ffi::c_void as *mut GDBusConnection,
            result,
            ::core::ptr::null_mut::<*mut GError>(),
        ));
    }
    safe_c2rust_launch_uris_with_dbus_data_free(data);
}
unsafe extern "C" fn safe_c2rust_launch_uris_with_dbus(
    mut info: *mut GDesktopAppInfo,
    mut session_bus: *mut GDBusConnection,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut dict: GVariantDict = _GVariantDict {
        u: C2RustUnnamed_2 {
            s: C2RustUnnamed_3 {
                asv: ::core::ptr::null_mut::<GVariant>(),
                partial_magic: 0,
                y: [0; 14],
            },
        },
    };
    let mut object_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut data: *mut LaunchUrisWithDBusData = ::core::ptr::null_mut::<LaunchUrisWithDBusData>();
    g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_TUPLE);
    if !uris.is_null() {
        let mut iter: *mut GList = ::core::ptr::null_mut::<GList>();
        g_variant_builder_open(&raw mut builder, G_VARIANT_TYPE_STRING_ARRAY);
        iter = uris;
        while !iter.is_null() {
            g_variant_builder_add(
                &raw mut builder,
                b"s\0" as *const u8 as *const gchar,
                (*iter).data,
            );
            iter = (*iter).next;
        }
        g_variant_builder_close(&raw mut builder);
    }
    platform_data = safe_c2rust_g_desktop_app_info_make_platform_data(info, uris, launch_context);
    g_variant_builder_add_value(&raw mut builder, platform_data);
    object_path = safe_c2rust_object_path_from_appid((*info).app_id);
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LaunchUrisWithDBusData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LaunchUrisWithDBusData;
    (*data).info = g_object_ref(info as gpointer) as *mut GDesktopAppInfo as *mut GDesktopAppInfo;
    (*data).callback = callback;
    (*data).user_data = user_data;
    (*data).launch_context = (if !launch_context.is_null() {
        g_object_ref(launch_context as gpointer) as *mut GAppLaunchContext
    } else {
        ::core::ptr::null_mut::<GAppLaunchContext>()
    }) as *mut GAppLaunchContext;
    g_variant_dict_init(&raw mut dict, platform_data);
    g_variant_dict_lookup(
        &raw mut dict,
        b"desktop-startup-id\0" as *const u8 as *const gchar,
        b"s\0" as *const u8 as *const gchar,
        &raw mut (*data).startup_id,
    );
    if !launch_context.is_null() {
        safe_c2rust_emit_launch_started(launch_context, info, (*data).startup_id);
    }
    g_dbus_connection_call(
        session_bus,
        (*info).app_id,
        object_path,
        b"org.freedesktop.Application\0" as *const u8 as *const gchar,
        if !uris.is_null() {
            b"Open\0" as *const u8 as *const gchar
        } else {
            b"Activate\0" as *const u8 as *const gchar
        },
        g_variant_builder_end(&raw mut builder),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        Some(
            safe_c2rust_launch_uris_with_dbus_signal_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LaunchUrisWithDBusData
            as gpointer,
    );
    g_free(object_path as gpointer);
    g_variant_dict_clear(&raw mut dict);
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris_with_dbus(
    mut info: *mut GDesktopAppInfo,
    mut session_bus: *mut GDBusConnection,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) -> gboolean {
    let mut ruris: *mut GList = uris;
    let mut app_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !info.is_null() {
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
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    app_id = safe_c2rust_g_desktop_app_info_get_string(
        info,
        b"X-Flatpak\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !app_id.is_null() && *app_id as ::core::ffi::c_int != 0 {
        ruris =
            g_document_portal_add_documents(uris, app_id, ::core::ptr::null_mut::<*mut GError>());
        if ruris.is_null() {
            ruris = uris;
        }
    }
    safe_c2rust_launch_uris_with_dbus(
        info,
        session_bus,
        ruris,
        launch_context,
        cancellable,
        callback,
        user_data,
    );
    if ruris != uris {
        g_list_free_full(ruris, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    }
    g_free(app_id as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris_internal(
    mut appinfo: *mut GAppInfo,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut spawn_flags: GSpawnFlags,
    mut user_setup: GSpawnChildSetupFunc,
    mut user_setup_data: gpointer,
    mut pid_callback: GDesktopAppLaunchCallback,
    mut pid_callback_data: gpointer,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    let mut session_bus: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut success: gboolean = TRUE;
    session_bus = g_bus_get_sync(
        G_BUS_TYPE_SESSION,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !session_bus.is_null() && !(*info).app_id.is_null() {
        safe_c2rust_g_desktop_app_info_launch_uris_with_dbus(
            info,
            session_bus,
            uris,
            launch_context,
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_1,
        );
    } else {
        success = safe_c2rust_g_desktop_app_info_launch_uris_with_spawn(
            info,
            session_bus,
            (*info).exec,
            uris,
            launch_context,
            spawn_flags,
            user_setup,
            user_setup_data,
            pid_callback,
            pid_callback_data,
            stdin_fd,
            stdout_fd,
            stderr_fd,
            error,
        );
    }
    if !session_bus.is_null() {
        g_dbus_connection_flush(
            session_bus,
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_1,
        );
        g_object_unref(session_bus as gpointer);
    }
    return success;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris(
    mut appinfo: *mut GAppInfo,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_desktop_app_info_launch_uris_internal(
        appinfo,
        uris,
        launch_context,
        G_SPAWN_SEARCH_PATH,
        None,
        NULL_1,
        None,
        NULL_1,
        -(1 as gint),
        -(1 as gint),
        -(1 as gint),
        error,
    );
}
unsafe extern "C" fn safe_c2rust_launch_uris_data_free(mut data: *mut LaunchUrisData) {
    let mut _pp: *mut *mut GAppLaunchContext = &raw mut (*data).context;
    let mut _ptr: *mut GAppLaunchContext = *_pp;
    *_pp = ::core::ptr::null_mut::<GAppLaunchContext>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_list_free_full(
        (*data).uris,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_uris_with_dbus_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    ret = g_dbus_connection_call_finish(
        object as *mut ::core::ffi::c_void as *mut GDBusConnection,
        result,
        &raw mut local_error,
    );
    if !local_error.is_null() {
        g_dbus_error_strip_remote_error(local_error);
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer) as *mut GError,
        );
    } else {
        g_task_return_boolean(task, TRUE);
        g_variant_unref(ret);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_uris_flush_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    g_dbus_connection_flush_finish(
        object as *mut ::core::ffi::c_void as *mut GDBusConnection,
        result,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_task_return_boolean(task, TRUE);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_uris_bus_get_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut info: *mut GDesktopAppInfo = g_task_get_source_object(task) as *mut GDesktopAppInfo;
    let mut data: *mut LaunchUrisData = g_task_get_task_data(task) as *mut LaunchUrisData;
    let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
    let mut session_bus: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    session_bus = g_bus_get_finish(result, ::core::ptr::null_mut::<*mut GError>());
    if !session_bus.is_null() && !(*info).app_id.is_null() {
        safe_c2rust_g_desktop_app_info_launch_uris_with_dbus(
            info,
            session_bus,
            (*data).uris,
            (*data).context,
            cancellable,
            Some(
                safe_c2rust_launch_uris_with_dbus_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
    } else {
        safe_c2rust_g_desktop_app_info_launch_uris_with_spawn(
            info,
            session_bus,
            (*info).exec,
            (*data).uris,
            (*data).context,
            G_SPAWN_SEARCH_PATH,
            None,
            NULL_1,
            None,
            NULL_1,
            -(1 as gint),
            -(1 as gint),
            -(1 as gint),
            &raw mut local_error,
        );
        if !local_error.is_null() {
            g_task_return_error(
                task,
                safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer) as *mut GError,
            );
            g_object_unref(task as gpointer);
        } else if !session_bus.is_null() {
            g_dbus_connection_flush(
                session_bus,
                cancellable,
                Some(
                    safe_c2rust_launch_uris_flush_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
            );
        } else {
            g_task_return_boolean(task, TRUE);
            let mut _pp: *mut *mut GTask = &raw mut task;
            let mut _ptr: *mut GTask = *_pp;
            *_pp = ::core::ptr::null_mut::<GTask>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
        }
    }
    let mut _pp_0: *mut *mut GDBusConnection = &raw mut session_bus;
    let mut _ptr_0: *mut GDBusConnection = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GDBusConnection>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris_async(
    mut appinfo: *mut GAppInfo,
    mut uris: *mut GList,
    mut context: *mut GAppLaunchContext,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LaunchUrisData = ::core::ptr::null_mut::<LaunchUrisData>();
    task = g_task_new(appinfo as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GAppInfo,
                    *mut GList,
                    *mut GAppLaunchContext,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_desktop_app_info_launch_uris_async
                as unsafe extern "C" fn(
                    *mut GAppInfo,
                    *mut GList,
                    *mut GAppLaunchContext,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_desktop_app_info_launch_uris_async\0" as *const u8 as *const gchar,
        );
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LaunchUrisData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LaunchUrisData;
    (*data).uris = g_list_copy_deep(
        uris,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*const gchar) -> *mut gchar>, GCopyFunc>(
            Some(g_strdup as unsafe extern "C" fn(*const gchar) -> *mut gchar),
        ),
        NULL_1,
    );
    let mut _object_ptr: C2RustUnnamed_7 = C2RustUnnamed_7 {
        in_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    _object_ptr.in_0 = &raw mut (*data).context as *mut ::core::ffi::c_char;
    if 0 as ::core::ffi::c_int != 0 {
        (*data).context = context;
    } else {
    };
    safe_c2rust_g_set_object(_object_ptr.out, context as *mut GObject);
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LaunchUrisData as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut LaunchUrisData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_launch_uris_data_free as unsafe extern "C" fn(*mut LaunchUrisData) -> (),
        )),
    );
    g_bus_get(
        G_BUS_TYPE_SESSION,
        cancellable,
        Some(
            safe_c2rust_launch_uris_bus_get_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris_finish(
    mut appinfo: *mut GAppInfo,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, appinfo as gpointer) != 0 {
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
            b"g_task_is_valid (result, appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_supports_uris(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (!(*info).exec.is_null()
        && (!strstr(
            (*info).exec,
            b"%u\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
            || !strstr(
                (*info).exec,
                b"%U\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_supports_files(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (!(*info).exec.is_null()
        && (!strstr(
            (*info).exec,
            b"%f\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
            || !strstr(
                (*info).exec,
                b"%F\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch(
    mut appinfo: *mut GAppInfo,
    mut files: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut uris: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut uri: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: gboolean = 0;
    uris = ::core::ptr::null_mut::<GList>();
    while !files.is_null() {
        uri = g_file_get_uri((*files).data as *mut GFile);
        uris = g_list_prepend(uris, uri as gpointer);
        files = (*files).next;
    }
    uris = g_list_reverse(uris);
    res = safe_c2rust_g_desktop_app_info_launch_uris(appinfo, uris, launch_context, error);
    g_list_free_full(uris, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris_as_manager_with_fds(
    mut appinfo: *mut GDesktopAppInfo,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut spawn_flags: GSpawnFlags,
    mut user_setup: GSpawnChildSetupFunc,
    mut user_setup_data: gpointer,
    mut pid_callback: GDesktopAppLaunchCallback,
    mut pid_callback_data: gpointer,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_desktop_app_info_launch_uris_internal(
        appinfo as *mut GAppInfo,
        uris,
        launch_context,
        spawn_flags,
        user_setup,
        user_setup_data,
        pid_callback,
        pid_callback_data,
        stdin_fd,
        stdout_fd,
        stderr_fd,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_uris_as_manager(
    mut appinfo: *mut GDesktopAppInfo,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut spawn_flags: GSpawnFlags,
    mut user_setup: GSpawnChildSetupFunc,
    mut user_setup_data: gpointer,
    mut pid_callback: GDesktopAppLaunchCallback,
    mut pid_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_desktop_app_info_launch_uris_as_manager_with_fds(
        appinfo,
        uris,
        launch_context,
        spawn_flags,
        user_setup,
        user_setup_data,
        pid_callback,
        pid_callback_data,
        -(1 as gint),
        -(1 as gint),
        -(1 as gint),
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_set_desktop_env(
    mut desktop_env: *const gchar,
) {
    safe_c2rust_get_current_desktops(desktop_env);
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_should_show(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if (*info).nodisplay() != 0 {
        return FALSE;
    }
    return safe_c2rust_g_desktop_app_info_get_show_in(info, ::core::ptr::null::<gchar>());
}
unsafe extern "C" fn safe_c2rust_ensure_dir(
    mut type_0: DirType,
    mut error: *mut *mut GError,
) -> *mut ::core::ffi::c_char {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut display_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut errsv: ::core::ffi::c_int = 0;
    match type_0 as ::core::ffi::c_uint {
        0 => {
            path = g_build_filename(g_get_user_config_dir(), NULL_1) as *mut ::core::ffi::c_char;
        }
        1 => {
            path = g_build_filename(
                g_get_user_data_dir(),
                b"applications\0" as *const u8 as *const ::core::ffi::c_char,
                NULL_1,
            ) as *mut ::core::ffi::c_char;
        }
        2 => {
            path = g_build_filename(
                g_get_user_data_dir(),
                b"mime\0" as *const u8 as *const ::core::ffi::c_char,
                b"packages\0" as *const u8 as *const ::core::ffi::c_char,
                NULL_1,
            ) as *mut ::core::ffi::c_char;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                3719 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"%s: Ensuring %s\0" as *const u8 as *const gchar,
        b"ensure_dir\0" as *const u8 as *const ::core::ffi::c_char,
        path,
    );
    *__errno_location() = 0 as ::core::ffi::c_int;
    if g_mkdir_with_parents(path, 0o700 as gint) == 0 as ::core::ffi::c_int {
        return path;
    }
    errsv = *__errno_location();
    display_name = g_filename_display_name(path) as *mut ::core::ffi::c_char;
    if type_0 as ::core::ffi::c_uint == APP_DIR as ::core::ffi::c_int as ::core::ffi::c_uint {
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t create user application configuration folder %s: %s\0"
                    as *const u8 as *const gchar,
            ),
            display_name,
            g_strerror(errsv as gint),
        );
    } else {
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t create user MIME configuration folder %s: %s\0" as *const u8
                    as *const gchar,
            ),
            display_name,
            g_strerror(errsv as gint),
        );
    }
    g_free(display_name as gpointer);
    g_free(path as gpointer);
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_update_mimeapps_list(
    mut desktop_id: *const ::core::ffi::c_char,
    mut content_type: *const ::core::ffi::c_char,
    mut flags: UpdateMimeFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut dirname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut string: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key_file: *mut GKeyFile = ::core::ptr::null_mut::<GKeyFile>();
    let mut load_succeeded: gboolean = 0;
    let mut res: gboolean = 0;
    let mut old_list: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut list: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut length: gsize = 0;
    let mut data_size: gsize = 0;
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut content_types: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !(flags as ::core::ffi::c_uint
            & UPDATE_MIME_SET_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && flags as ::core::ffi::c_uint
                & UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0)
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdesktopappinfo.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            3762 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!((flags & UPDATE_MIME_SET_DEFAULT) && (flags & UPDATE_MIME_SET_NON_DEFAULT))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    dirname = safe_c2rust_ensure_dir(CONF_DIR, error);
    if dirname.is_null() {
        return FALSE;
    }
    filename = g_build_filename(
        dirname,
        b"mimeapps.list\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    ) as *mut ::core::ffi::c_char;
    g_free(dirname as gpointer);
    key_file = g_key_file_new();
    load_succeeded = g_key_file_load_from_file(
        key_file,
        filename,
        G_KEY_FILE_NONE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if load_succeeded == 0
        || g_key_file_has_group(key_file, ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar) == 0
            && g_key_file_has_group(
                key_file,
                REMOVED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
            ) == 0
            && g_key_file_has_group(
                key_file,
                DEFAULT_APPLICATIONS_GROUP.as_ptr() as *const gchar,
            ) == 0
    {
        g_key_file_free(key_file);
        key_file = g_key_file_new();
    }
    if !content_type.is_null() {
        content_types = ({
            let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut ::core::ffi::c_char;
        let ref mut fresh1 = *content_types.offset(0 as ::core::ffi::c_int as isize);
        *fresh1 = safe_c2rust_g_strdup_inline(content_type);
        let ref mut fresh2 = *content_types.offset(1 as ::core::ffi::c_int as isize);
        *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        content_types = g_key_file_get_keys(
            key_file,
            DEFAULT_APPLICATIONS_GROUP.as_ptr() as *const gchar,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut *mut ::core::ffi::c_char;
    }
    k = 0 as ::core::ffi::c_int;
    while !content_types.is_null() && !(*content_types.offset(k as isize)).is_null() {
        string = g_key_file_get_string(
            key_file,
            DEFAULT_APPLICATIONS_GROUP.as_ptr() as *const gchar,
            *content_types.offset(k as isize),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
        if g_strcmp0(string, desktop_id) != 0 as ::core::ffi::c_int
            && flags as ::core::ffi::c_uint
                & UPDATE_MIME_SET_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            g_free(string as gpointer);
            string = safe_c2rust_g_strdup_inline(desktop_id);
            flags = ::core::mem::transmute::<::core::ffi::c_uint, UpdateMimeFlags>(
                flags as ::core::ffi::c_uint
                    | UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        if string.is_null() || desktop_id.is_null() {
            g_key_file_remove_key(
                key_file,
                DEFAULT_APPLICATIONS_GROUP.as_ptr() as *const gchar,
                *content_types.offset(k as isize),
                ::core::ptr::null_mut::<*mut GError>(),
            );
        } else {
            g_key_file_set_string(
                key_file,
                DEFAULT_APPLICATIONS_GROUP.as_ptr() as *const gchar,
                *content_types.offset(k as isize),
                string,
            );
        }
        g_free(string as gpointer);
        k += 1;
    }
    if content_type.is_null() {
        g_strfreev(content_types as *mut *mut gchar);
        content_types = g_key_file_get_keys(
            key_file,
            ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut *mut ::core::ffi::c_char;
    }
    k = 0 as ::core::ffi::c_int;
    while !content_types.is_null() && !(*content_types.offset(k as isize)).is_null() {
        length = 0 as gsize;
        old_list = g_key_file_get_string_list(
            key_file,
            ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
            *content_types.offset(k as isize),
            &raw mut length,
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut *mut ::core::ffi::c_char;
        list = ({
            let mut __n: gsize = (1 as gsize).wrapping_add(length).wrapping_add(1 as gsize);
            let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        if flags as ::core::ffi::c_uint
            & UPDATE_MIME_SET_LAST_USED as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            if flags as ::core::ffi::c_uint
                & UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                flags = ::core::mem::transmute::<::core::ffi::c_uint, UpdateMimeFlags>(
                    flags as ::core::ffi::c_uint
                        ^ UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
            }
            let fresh3 = i;
            i = i + 1;
            let ref mut fresh4 = *list.offset(fresh3 as isize);
            *fresh4 = safe_c2rust_g_strdup_inline(desktop_id);
        }
        if !old_list.is_null() {
            j = 0 as ::core::ffi::c_int;
            while !(*old_list.offset(j as isize)).is_null() {
                if g_strcmp0(*old_list.offset(j as isize), desktop_id) != 0 as ::core::ffi::c_int {
                    let fresh5 = i;
                    i = i + 1;
                    let ref mut fresh6 = *list.offset(fresh5 as isize);
                    *fresh6 = safe_c2rust_g_strdup_inline(*old_list.offset(j as isize));
                } else if flags as ::core::ffi::c_uint
                    & UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    flags = ::core::mem::transmute::<::core::ffi::c_uint, UpdateMimeFlags>(
                        flags as ::core::ffi::c_uint
                            ^ UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int
                                as ::core::ffi::c_uint,
                    );
                    let fresh7 = i;
                    i = i + 1;
                    let ref mut fresh8 = *list.offset(fresh7 as isize);
                    *fresh8 = safe_c2rust_g_strdup_inline(*old_list.offset(j as isize));
                }
                j += 1;
            }
        }
        if flags as ::core::ffi::c_uint
            & UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            let fresh9 = i;
            i = i + 1;
            let ref mut fresh10 = *list.offset(fresh9 as isize);
            *fresh10 = safe_c2rust_g_strdup_inline(desktop_id);
        }
        let ref mut fresh11 = *list.offset(i as isize);
        *fresh11 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_strfreev(old_list as *mut *mut gchar);
        if (*list.offset(0 as ::core::ffi::c_int as isize)).is_null() || desktop_id.is_null() {
            g_key_file_remove_key(
                key_file,
                ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
                *content_types.offset(k as isize),
                ::core::ptr::null_mut::<*mut GError>(),
            );
        } else {
            g_key_file_set_string_list(
                key_file,
                ADDED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
                *content_types.offset(k as isize),
                list as *const *const gchar,
                i as gsize,
            );
        }
        g_strfreev(list as *mut *mut gchar);
        k += 1;
    }
    if content_type.is_null() {
        g_strfreev(content_types as *mut *mut gchar);
        content_types = g_key_file_get_keys(
            key_file,
            REMOVED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut *mut ::core::ffi::c_char;
    }
    k = 0 as ::core::ffi::c_int;
    while !content_types.is_null() && !(*content_types.offset(k as isize)).is_null() {
        length = 0 as gsize;
        old_list = g_key_file_get_string_list(
            key_file,
            REMOVED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
            *content_types.offset(k as isize),
            &raw mut length,
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut *mut ::core::ffi::c_char;
        list = ({
            let mut __n: gsize = (1 as gsize).wrapping_add(length).wrapping_add(1 as gsize);
            let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        if flags as ::core::ffi::c_uint
            & UPDATE_MIME_REMOVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            let fresh12 = i;
            i = i + 1;
            let ref mut fresh13 = *list.offset(fresh12 as isize);
            *fresh13 = safe_c2rust_g_strdup_inline(desktop_id);
        }
        if !old_list.is_null() {
            j = 0 as ::core::ffi::c_int;
            while !(*old_list.offset(j as isize)).is_null() {
                if g_strcmp0(*old_list.offset(j as isize), desktop_id) != 0 as ::core::ffi::c_int {
                    let fresh14 = i;
                    i = i + 1;
                    let ref mut fresh15 = *list.offset(fresh14 as isize);
                    *fresh15 = safe_c2rust_g_strdup_inline(*old_list.offset(j as isize));
                }
                j += 1;
            }
        }
        let ref mut fresh16 = *list.offset(i as isize);
        *fresh16 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_strfreev(old_list as *mut *mut gchar);
        if (*list.offset(0 as ::core::ffi::c_int as isize)).is_null() || desktop_id.is_null() {
            g_key_file_remove_key(
                key_file,
                REMOVED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
                *content_types.offset(k as isize),
                ::core::ptr::null_mut::<*mut GError>(),
            );
        } else {
            g_key_file_set_string_list(
                key_file,
                REMOVED_ASSOCIATIONS_GROUP.as_ptr() as *const gchar,
                *content_types.offset(k as isize),
                list as *const *const gchar,
                i as gsize,
            );
        }
        g_strfreev(list as *mut *mut gchar);
        k += 1;
    }
    g_strfreev(content_types as *mut *mut gchar);
    data = g_key_file_to_data(key_file, &raw mut data_size, error) as *mut ::core::ffi::c_char;
    g_key_file_free(key_file);
    res = g_file_set_contents_full(
        filename,
        data,
        data_size as gssize,
        (G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
            | G_FILE_SET_CONTENTS_ONLY_EXISTING as ::core::ffi::c_int)
            as GFileSetContentsFlags,
        0o600 as ::core::ffi::c_int,
        error,
    );
    safe_c2rust_desktop_file_dirs_invalidate_user_config();
    g_free(filename as gpointer);
    g_free(data as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_set_as_last_used_for_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if safe_c2rust_g_desktop_app_info_ensure_saved(info, error) == 0 {
        return FALSE;
    }
    if (*info).desktop_id.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Application information lacks an identifier\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    return safe_c2rust_update_mimeapps_list(
        (*info).desktop_id,
        content_type,
        (UPDATE_MIME_SET_NON_DEFAULT as ::core::ffi::c_int
            | UPDATE_MIME_SET_LAST_USED as ::core::ffi::c_int) as UpdateMimeFlags,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_set_as_default_for_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if safe_c2rust_g_desktop_app_info_ensure_saved(info, error) == 0 {
        return FALSE;
    }
    if (*info).desktop_id.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Application information lacks an identifier\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    return safe_c2rust_update_mimeapps_list(
        (*info).desktop_id,
        content_type,
        UPDATE_MIME_SET_DEFAULT,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_update_program_done(
    mut pid: GPid,
    mut status: gint,
    mut data: gpointer,
) {
    g_spawn_check_wait_status(status, ::core::ptr::null_mut::<*mut GError>()) != 0;
}
unsafe extern "C" fn safe_c2rust_run_update_command(
    mut command: *mut ::core::ffi::c_char,
    mut subdir: *mut ::core::ffi::c_char,
) {
    let mut argv: [*mut ::core::ffi::c_char; 3] = [
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    let mut pid: GPid = 0 as GPid;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    argv[0 as ::core::ffi::c_int as usize] = command;
    argv[1 as ::core::ffi::c_int as usize] =
        g_build_filename(g_get_user_data_dir(), subdir, NULL_1) as *mut ::core::ffi::c_char;
    if g_spawn_async(
        b"/\0" as *const u8 as *const gchar,
        &raw mut argv as *mut *mut gchar,
        ::core::ptr::null_mut::<*mut gchar>(),
        (G_SPAWN_SEARCH_PATH as ::core::ffi::c_int
            | G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int
            | G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int
            | G_SPAWN_DO_NOT_REAP_CHILD as ::core::ffi::c_int) as GSpawnFlags,
        None,
        NULL_1,
        &raw mut pid,
        &raw mut local_error,
    ) != 0
    {
        g_child_watch_add(
            pid,
            Some(
                safe_c2rust_update_program_done as unsafe extern "C" fn(GPid, gint, gpointer) -> (),
            ),
            NULL_1,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s\0" as *const u8 as *const gchar,
            (*local_error).message,
        );
        g_error_free(local_error);
    }
    g_free(argv[1 as ::core::ffi::c_int as usize] as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_set_as_default_for_extension(
    mut appinfo: *mut GAppInfo,
    mut extension: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mimetype: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dirname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: gboolean = 0;
    if safe_c2rust_g_desktop_app_info_ensure_saved(
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo,
        error,
    ) == 0
    {
        return FALSE;
    }
    dirname = safe_c2rust_ensure_dir(MIMETYPE_DIR, error);
    if dirname.is_null() {
        return FALSE;
    }
    basename = g_strdup_printf(
        b"user-extension-%s.xml\0" as *const u8 as *const gchar,
        extension,
    ) as *mut ::core::ffi::c_char;
    filename = g_build_filename(dirname, basename, NULL_1) as *mut ::core::ffi::c_char;
    g_free(basename as gpointer);
    g_free(dirname as gpointer);
    mimetype = g_strdup_printf(
        b"application/x-extension-%s\0" as *const u8 as *const gchar,
        extension,
    ) as *mut ::core::ffi::c_char;
    if g_file_test(filename, G_FILE_TEST_EXISTS) == 0 {
        let mut contents: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        contents = g_strdup_printf(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<mime-info xmlns=\"http://www.freedesktop.org/standards/shared-mime-info\">\n <mime-type type=\"%s\">\n  <comment>%s document</comment>\n  <glob pattern=\"*.%s\"/>\n </mime-type>\n</mime-info>\n\0"
                as *const u8 as *const gchar,
            mimetype,
            extension,
            extension,
        ) as *mut ::core::ffi::c_char;
        g_file_set_contents_full(
            filename,
            contents,
            -(1 as ::core::ffi::c_int) as gssize,
            (G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
                | G_FILE_SET_CONTENTS_ONLY_EXISTING as ::core::ffi::c_int)
                as GFileSetContentsFlags,
            0o600 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_free(contents as gpointer);
        safe_c2rust_run_update_command(
            b"update-mime-database\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"mime\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    g_free(filename as gpointer);
    res = safe_c2rust_g_desktop_app_info_set_as_default_for_type(appinfo, mimetype, error);
    g_free(mimetype as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_add_supports_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if safe_c2rust_g_desktop_app_info_ensure_saved(
        info as *mut ::core::ffi::c_void as *mut GDesktopAppInfo,
        error,
    ) == 0
    {
        return FALSE;
    }
    return safe_c2rust_update_mimeapps_list(
        (*info).desktop_id,
        content_type,
        UPDATE_MIME_SET_NON_DEFAULT,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_can_remove_supports_type(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_remove_supports_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if safe_c2rust_g_desktop_app_info_ensure_saved(
        info as *mut ::core::ffi::c_void as *mut GDesktopAppInfo,
        error,
    ) == 0
    {
        return FALSE;
    }
    return safe_c2rust_update_mimeapps_list(
        (*info).desktop_id,
        content_type,
        UPDATE_MIME_REMOVE,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_supported_types(
    mut appinfo: *mut GAppInfo,
) -> *mut *const ::core::ffi::c_char {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    return (*info).mime_types as *mut *const ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_ensure_saved(
    mut info: *mut GDesktopAppInfo,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut key_file: *mut GKeyFile = ::core::ptr::null_mut::<GKeyFile>();
    let mut dirname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut desktop_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data_size: gsize = 0;
    let mut fd: ::core::ffi::c_int = 0;
    let mut res: gboolean = 0;
    if !(*info).filename.is_null() {
        return TRUE;
    }
    dirname = safe_c2rust_ensure_dir(APP_DIR, error);
    if dirname.is_null() {
        return FALSE;
    }
    key_file = g_key_file_new();
    g_key_file_set_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        b"Encoding\0" as *const u8 as *const gchar,
        b"UTF-8\0" as *const u8 as *const gchar,
    );
    g_key_file_set_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_VERSION.as_ptr() as *const gchar,
        b"1.0\0" as *const u8 as *const gchar,
    );
    g_key_file_set_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_TYPE.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_TYPE_APPLICATION.as_ptr() as *const gchar,
    );
    if (*info).terminal() != 0 {
        g_key_file_set_boolean(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            G_KEY_FILE_DESKTOP_KEY_TERMINAL.as_ptr() as *const gchar,
            TRUE,
        );
    }
    if (*info).nodisplay() != 0 {
        g_key_file_set_boolean(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            G_KEY_FILE_DESKTOP_KEY_NO_DISPLAY.as_ptr() as *const gchar,
            TRUE,
        );
    }
    g_key_file_set_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_EXEC.as_ptr() as *const gchar,
        (*info).exec,
    );
    g_key_file_set_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_NAME.as_ptr() as *const gchar,
        (*info).name,
    );
    if !(*info).generic_name.is_null() {
        g_key_file_set_string(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            GENERIC_NAME_KEY.as_ptr() as *const gchar,
            (*info).generic_name,
        );
    }
    if !(*info).fullname.is_null() {
        g_key_file_set_string(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            FULL_NAME_KEY.as_ptr() as *const gchar,
            (*info).fullname,
        );
    }
    g_key_file_set_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_COMMENT.as_ptr() as *const gchar,
        (*info).comment,
    );
    g_key_file_set_boolean(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_NO_DISPLAY.as_ptr() as *const gchar,
        TRUE,
    );
    data = g_key_file_to_data(
        key_file,
        &raw mut data_size,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    g_key_file_free(key_file);
    desktop_id = g_strdup_printf(
        b"userapp-%s-XXXXXX.desktop\0" as *const u8 as *const gchar,
        (*info).name,
    ) as *mut ::core::ffi::c_char;
    filename = g_build_filename(dirname, desktop_id, NULL_1) as *mut ::core::ffi::c_char;
    g_free(desktop_id as gpointer);
    g_free(dirname as gpointer);
    fd = g_mkstemp(filename as *mut gchar) as ::core::ffi::c_int;
    if fd == -(1 as ::core::ffi::c_int) {
        let mut display_name: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        display_name = g_filename_display_name(filename) as *mut ::core::ffi::c_char;
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t create user desktop file %s\0" as *const u8 as *const gchar,
            ),
            display_name,
        );
        g_free(display_name as gpointer);
        g_free(filename as gpointer);
        g_free(data as gpointer);
        return FALSE;
    }
    desktop_id = g_path_get_basename(filename) as *mut ::core::ffi::c_char;
    g_close(fd as gint, ::core::ptr::null_mut::<*mut GError>());
    res = g_file_set_contents_full(
        filename,
        data,
        data_size as gssize,
        (G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
            | G_FILE_SET_CONTENTS_ONLY_EXISTING as ::core::ffi::c_int)
            as GFileSetContentsFlags,
        0o600 as ::core::ffi::c_int,
        error,
    );
    g_free(data as gpointer);
    if res == 0 {
        g_free(desktop_id as gpointer);
        g_free(filename as gpointer);
        return FALSE;
    }
    (*info).filename = filename;
    (*info).desktop_id = desktop_id;
    safe_c2rust_run_update_command(
        b"update-desktop-database\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"applications\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    safe_c2rust_desktop_file_dirs_invalidate_user_data();
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_can_delete(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if !(*info).filename.is_null() {
        if !strstr(
            (*info).filename,
            b"/userapp-\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            return (g_access((*info).filename, W_OK) == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_delete(mut appinfo: *mut GAppInfo) -> gboolean {
    let mut info: *mut GDesktopAppInfo =
        appinfo as *mut ::core::ffi::c_void as *mut GDesktopAppInfo;
    if !(*info).filename.is_null() {
        if remove((*info).filename) == 0 as ::core::ffi::c_int {
            safe_c2rust_update_mimeapps_list(
                (*info).desktop_id,
                ::core::ptr::null::<::core::ffi::c_char>(),
                UPDATE_MIME_NONE,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_free((*info).filename as gpointer);
            (*info).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
            g_free((*info).desktop_id as gpointer);
            (*info).desktop_id = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return TRUE;
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_create_from_commandline(
    mut commandline: *const ::core::ffi::c_char,
    mut application_name: *const ::core::ffi::c_char,
    mut flags: GAppInfoCreateFlags,
    mut error: *mut *mut GError,
) -> *mut GAppInfo {
    let mut split: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !commandline.is_null() {
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
            b"commandline\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    info = g_object_new(
        safe_c2rust_g_desktop_app_info_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDesktopAppInfo;
    (*info).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*info).desktop_id = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*info).set_terminal(
        (flags as ::core::ffi::c_uint
            & G_APP_INFO_CREATE_NEEDS_TERMINAL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
    );
    (*info).set_startup_notify(
        (flags as ::core::ffi::c_uint
            & G_APP_INFO_CREATE_SUPPORTS_STARTUP_NOTIFICATION as ::core::ffi::c_int
                as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
    );
    (*info).set_hidden(FALSE as guint as guint);
    if flags as ::core::ffi::c_uint
        & G_APP_INFO_CREATE_SUPPORTS_URIS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        (*info).exec = g_strconcat(
            commandline as *const gchar,
            b" %u\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_1,
        ) as *mut ::core::ffi::c_char;
    } else {
        (*info).exec = g_strconcat(
            commandline as *const gchar,
            b" %f\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_1,
        ) as *mut ::core::ffi::c_char;
    }
    (*info).set_nodisplay(TRUE as guint as guint);
    (*info).binary = safe_c2rust_binary_from_exec((*info).exec);
    if !application_name.is_null() {
        (*info).name = safe_c2rust_g_strdup_inline(application_name);
    } else {
        split = g_strsplit(
            commandline as *const gchar,
            b" \0" as *const u8 as *const gchar,
            2 as gint,
        ) as *mut *mut ::core::ffi::c_char;
        basename = (if !(*split.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            g_path_get_basename(*split.offset(0 as ::core::ffi::c_int as isize))
        } else {
            ::core::ptr::null_mut::<gchar>()
        }) as *mut ::core::ffi::c_char;
        g_strfreev(split as *mut *mut gchar);
        (*info).name = basename;
        if (*info).name.is_null() {
            (*info).name =
                safe_c2rust_g_strdup_inline(b"custom\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    (*info).comment = g_strdup_printf(
        glib_gettext(b"Custom definition for %s\0" as *const u8 as *const gchar),
        (*info).name,
    ) as *mut ::core::ffi::c_char;
    return info as *mut ::core::ffi::c_void as *mut GAppInfo;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_iface_init(mut iface: *mut GAppInfoIface) {
    (*iface).dup = Some(
        safe_c2rust_g_desktop_app_info_dup as unsafe extern "C" fn(*mut GAppInfo) -> *mut GAppInfo,
    ) as Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut GAppInfo>;
    (*iface).equal = Some(
        safe_c2rust_g_desktop_app_info_equal
            as unsafe extern "C" fn(*mut GAppInfo, *mut GAppInfo) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo, *mut GAppInfo) -> gboolean>;
    (*iface).get_id = Some(
        safe_c2rust_g_desktop_app_info_get_id
            as unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>;
    (*iface).get_name = Some(
        safe_c2rust_g_desktop_app_info_get_name
            as unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>;
    (*iface).get_description = Some(
        safe_c2rust_g_desktop_app_info_get_description
            as unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>;
    (*iface).get_executable = Some(
        safe_c2rust_g_desktop_app_info_get_executable
            as unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>;
    (*iface).get_icon = Some(
        safe_c2rust_g_desktop_app_info_get_icon
            as unsafe extern "C" fn(*mut GAppInfo) -> *mut GIcon,
    ) as Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut GIcon>;
    (*iface).launch = Some(
        safe_c2rust_g_desktop_app_info_launch
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *mut GList,
                *mut GAppLaunchContext,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *mut GList,
                *mut GAppLaunchContext,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).supports_uris = Some(
        safe_c2rust_g_desktop_app_info_supports_uris
            as unsafe extern "C" fn(*mut GAppInfo) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>;
    (*iface).supports_files = Some(
        safe_c2rust_g_desktop_app_info_supports_files
            as unsafe extern "C" fn(*mut GAppInfo) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>;
    (*iface).launch_uris = Some(
        safe_c2rust_g_desktop_app_info_launch_uris
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *mut GList,
                *mut GAppLaunchContext,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *mut GList,
                *mut GAppLaunchContext,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).launch_uris_async = Some(
        safe_c2rust_g_desktop_app_info_launch_uris_async
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *mut GList,
                *mut GAppLaunchContext,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *mut GList,
                *mut GAppLaunchContext,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).launch_uris_finish = Some(
        safe_c2rust_g_desktop_app_info_launch_uris_finish
            as unsafe extern "C" fn(*mut GAppInfo, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GAppInfo, *mut GAsyncResult, *mut *mut GError) -> gboolean,
        >;
    (*iface).should_show = Some(
        safe_c2rust_g_desktop_app_info_should_show
            as unsafe extern "C" fn(*mut GAppInfo) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>;
    (*iface).set_as_default_for_type = Some(
        safe_c2rust_g_desktop_app_info_set_as_default_for_type
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).set_as_default_for_extension = Some(
        safe_c2rust_g_desktop_app_info_set_as_default_for_extension
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).add_supports_type = Some(
        safe_c2rust_g_desktop_app_info_add_supports_type
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).can_remove_supports_type = Some(
        safe_c2rust_g_desktop_app_info_can_remove_supports_type
            as unsafe extern "C" fn(*mut GAppInfo) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>;
    (*iface).remove_supports_type = Some(
        safe_c2rust_g_desktop_app_info_remove_supports_type
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).can_delete = Some(
        safe_c2rust_g_desktop_app_info_can_delete
            as unsafe extern "C" fn(*mut GAppInfo) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>;
    (*iface).do_delete = Some(
        safe_c2rust_g_desktop_app_info_delete as unsafe extern "C" fn(*mut GAppInfo) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>;
    (*iface).get_commandline = Some(
        safe_c2rust_g_desktop_app_info_get_commandline
            as unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>;
    (*iface).get_display_name = Some(
        safe_c2rust_g_desktop_app_info_get_display_name
            as unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>;
    (*iface).set_as_last_used_for_type = Some(
        safe_c2rust_g_desktop_app_info_set_as_last_used_for_type
            as unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAppInfo,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).get_supported_types = Some(
        safe_c2rust_g_desktop_app_info_get_supported_types
            as unsafe extern "C" fn(*mut GAppInfo) -> *mut *const ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut *const ::core::ffi::c_char>;
}
unsafe extern "C" fn safe_c2rust_get_list_of_mimetypes(
    mut content_type: *const gchar,
    mut include_fallback: gboolean,
) -> *mut *mut gchar {
    let mut unaliased: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    array = g_ptr_array_new();
    unaliased =
        _g_unix_content_type_unalias(content_type as *const ::core::ffi::c_char) as *mut gchar;
    g_ptr_array_add(array, unaliased as gpointer);
    if include_fallback != 0 {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*array).len {
            let mut parents: *mut *mut gchar = _g_unix_content_type_get_parents(
                *(*array).pdata.offset(i as isize) as *const ::core::ffi::c_char,
            ) as *mut *mut gchar;
            let mut j: gint = 0;
            j = 0 as ::core::ffi::c_int as gint;
            while !(*parents.offset(j as isize)).is_null() {
                if safe_c2rust_array_contains(array, *parents.offset(j as isize)) == 0 {
                    g_ptr_array_add(array, *parents.offset(j as isize) as gpointer);
                } else {
                    g_free(*parents.offset(j as isize) as gpointer);
                }
                j += 1;
            }
            g_free(parents as gpointer);
            i = i.wrapping_add(1);
        }
    }
    g_ptr_array_add(array, NULL_1);
    return g_ptr_array_free(array, FALSE) as *mut *mut gchar;
}
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_desktop_ids_for_content_type(
    mut content_type: *const gchar,
    mut include_fallback: gboolean,
) -> *mut *mut gchar {
    let mut hits: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut blocklist: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut types: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: guint = 0;
    let mut j: guint = 0;
    hits = g_ptr_array_new();
    blocklist = g_ptr_array_new();
    types = safe_c2rust_get_list_of_mimetypes(content_type, include_fallback);
    safe_c2rust_desktop_file_dirs_lock();
    i = 0 as guint;
    while !(*types.offset(i as isize)).is_null() {
        j = 0 as guint;
        while j < (*safe_c2rust_desktop_file_dirs).len {
            safe_c2rust_desktop_file_dir_mime_lookup(
                *(*safe_c2rust_desktop_file_dirs).pdata.offset(j as isize) as *mut DesktopFileDir,
                *types.offset(i as isize),
                hits,
                blocklist,
            );
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as guint;
    while i < (*hits).len {
        let ref mut fresh31 = *(*hits).pdata.offset(i as isize);
        *fresh31 = safe_c2rust_g_strdup_inline(
            *(*hits).pdata.offset(i as isize) as *const ::core::ffi::c_char
        ) as gpointer;
        i = i.wrapping_add(1);
    }
    safe_c2rust_desktop_file_dirs_unlock();
    g_ptr_array_add(hits, NULL_1);
    g_ptr_array_free(blocklist, TRUE);
    g_strfreev(types);
    return g_ptr_array_free(hits, FALSE) as *mut *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_recommended_for_type(
    mut content_type: *const gchar,
) -> *mut GList {
    let mut desktop_ids: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut infos: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    desktop_ids =
        safe_c2rust_g_desktop_app_info_get_desktop_ids_for_content_type(content_type, FALSE);
    infos = ::core::ptr::null_mut::<GList>();
    i = 0 as ::core::ffi::c_int as gint;
    while !(*desktop_ids.offset(i as isize)).is_null() {
        let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
        info = safe_c2rust_g_desktop_app_info_new(*desktop_ids.offset(i as isize));
        if !info.is_null() {
            infos = g_list_prepend(infos, info as gpointer);
        }
        i += 1;
    }
    g_strfreev(desktop_ids);
    return g_list_reverse(infos);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_fallback_for_type(
    mut content_type: *const gchar,
) -> *mut GList {
    let mut recommended_ids: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut all_ids: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut infos: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    recommended_ids =
        safe_c2rust_g_desktop_app_info_get_desktop_ids_for_content_type(content_type, FALSE);
    all_ids = safe_c2rust_g_desktop_app_info_get_desktop_ids_for_content_type(content_type, TRUE);
    infos = ::core::ptr::null_mut::<GList>();
    i = 0 as ::core::ffi::c_int as gint;
    while !(*all_ids.offset(i as isize)).is_null() {
        let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
        let mut j: gint = 0;
        j = 0 as ::core::ffi::c_int as gint;
        while !(*recommended_ids.offset(j as isize)).is_null() {
            if strcmp(
                *all_ids.offset(i as isize) as *const ::core::ffi::c_char,
                *recommended_ids.offset(j as isize) as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                break;
            }
            j += 1;
        }
        if (*recommended_ids.offset(j as isize)).is_null() {
            info = safe_c2rust_g_desktop_app_info_new(*all_ids.offset(i as isize));
            if !info.is_null() {
                infos = g_list_prepend(infos, info as gpointer);
            }
        }
        i += 1;
    }
    g_strfreev(recommended_ids);
    g_strfreev(all_ids);
    return g_list_reverse(infos);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_all_for_type(
    mut content_type: *const ::core::ffi::c_char,
) -> *mut GList {
    let mut desktop_ids: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut infos: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    desktop_ids = safe_c2rust_g_desktop_app_info_get_desktop_ids_for_content_type(
        content_type as *const gchar,
        TRUE,
    );
    infos = ::core::ptr::null_mut::<GList>();
    i = 0 as ::core::ffi::c_int as gint;
    while !(*desktop_ids.offset(i as isize)).is_null() {
        let mut info: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
        info = safe_c2rust_g_desktop_app_info_new(*desktop_ids.offset(i as isize));
        if !info.is_null() {
            infos = g_list_prepend(infos, info as gpointer);
        }
        i += 1;
    }
    g_strfreev(desktop_ids);
    return g_list_reverse(infos);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_reset_type_associations(
    mut content_type: *const ::core::ffi::c_char,
) {
    safe_c2rust_update_mimeapps_list(
        ::core::ptr::null::<::core::ffi::c_char>(),
        content_type,
        UPDATE_MIME_NONE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_default_for_type(
    mut content_type: *const ::core::ffi::c_char,
    mut must_support_uris: gboolean,
) -> *mut GAppInfo {
    let mut blocklist: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut results: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    let mut types: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: guint = 0;
    let mut j: guint = 0;
    let mut k: guint = 0;
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    types = safe_c2rust_get_list_of_mimetypes(content_type as *const gchar, TRUE);
    blocklist = g_ptr_array_new();
    results = g_ptr_array_new();
    info = ::core::ptr::null_mut::<GAppInfo>();
    safe_c2rust_desktop_file_dirs_lock();
    i = 0 as guint;
    's_55: while !(*types.offset(i as isize)).is_null() {
        j = 0 as guint;
        while j < (*safe_c2rust_desktop_file_dirs).len {
            safe_c2rust_desktop_file_dir_default_lookup(
                *(*safe_c2rust_desktop_file_dirs).pdata.offset(j as isize) as *mut DesktopFileDir,
                *types.offset(i as isize),
                results,
            );
            j = j.wrapping_add(1);
        }
        j = 0 as guint;
        while j < (*safe_c2rust_desktop_file_dirs).len {
            safe_c2rust_desktop_file_dir_mime_lookup(
                *(*safe_c2rust_desktop_file_dirs).pdata.offset(j as isize) as *mut DesktopFileDir,
                *types.offset(i as isize),
                results,
                blocklist,
            );
            j = j.wrapping_add(1);
        }
        j = 0 as guint;
        while j < (*results).len {
            let mut desktop_id: *const gchar = *(*results).pdata.offset(j as isize) as *const gchar;
            k = 0 as guint;
            while k < (*safe_c2rust_desktop_file_dirs).len {
                info = safe_c2rust_desktop_file_dir_get_app(
                    *(*safe_c2rust_desktop_file_dirs).pdata.offset(k as isize)
                        as *mut DesktopFileDir,
                    desktop_id,
                ) as *mut GAppInfo;
                if !info.is_null() {
                    if must_support_uris == 0 || g_app_info_supports_uris(info) != 0 {
                        break 's_55;
                    }
                    let mut _pp: *mut *mut GAppInfo = &raw mut info;
                    let mut _ptr: *mut GAppInfo = *_pp;
                    *_pp = ::core::ptr::null_mut::<GAppInfo>();
                    if !_ptr.is_null() {
                        g_object_unref(_ptr as gpointer);
                    }
                }
                k = k.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        g_ptr_array_set_size(results, 0 as gint);
        i = i.wrapping_add(1);
    }
    safe_c2rust_desktop_file_dirs_unlock();
    g_ptr_array_unref(blocklist);
    g_ptr_array_unref(results);
    g_strfreev(types);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_default_for_uri_scheme(
    mut uri_scheme: *const ::core::ffi::c_char,
) -> *mut GAppInfo {
    let mut app_info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    let mut content_type: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut scheme_down: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !uri_scheme.is_null() && *uri_scheme as ::core::ffi::c_int != '\0' as i32 {
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
            b"uri_scheme != NULL && *uri_scheme != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    scheme_down = g_ascii_strdown(
        uri_scheme as *const gchar,
        -(1 as ::core::ffi::c_int) as gssize,
    ) as *mut ::core::ffi::c_char;
    content_type = g_strdup_printf(
        b"x-scheme-handler/%s\0" as *const u8 as *const gchar,
        scheme_down,
    ) as *mut ::core::ffi::c_char;
    g_free(scheme_down as gpointer);
    app_info = safe_c2rust_g_app_info_get_default_for_type(content_type, FALSE);
    g_free(content_type as gpointer);
    return app_info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_implementations(
    mut interface: *const gchar,
) -> *mut GList {
    let mut result: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ptr: *mut *mut GList = ::core::ptr::null_mut::<*mut GList>();
    let mut i: guint = 0;
    safe_c2rust_desktop_file_dirs_lock();
    i = 0 as guint;
    while i < (*safe_c2rust_desktop_file_dirs).len {
        safe_c2rust_desktop_file_dir_get_implementations(
            *(*safe_c2rust_desktop_file_dirs).pdata.offset(i as isize) as *mut DesktopFileDir,
            &raw mut result,
            interface,
        );
        i = i.wrapping_add(1);
    }
    safe_c2rust_desktop_file_dirs_unlock();
    ptr = &raw mut result;
    while !(*ptr).is_null() {
        let mut name: *mut gchar = (**ptr).data as *mut gchar;
        let mut app: *mut GDesktopAppInfo = ::core::ptr::null_mut::<GDesktopAppInfo>();
        app = safe_c2rust_g_desktop_app_info_new(name);
        g_free(name as gpointer);
        if !app.is_null() {
            (**ptr).data = app as gpointer;
            ptr = &raw mut (**ptr).next;
        } else {
            *ptr = g_list_delete_link(*ptr, *ptr);
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_search(
    mut search_string: *const gchar,
) -> *mut *mut *mut gchar {
    let mut search_tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut last_category: gint = -(1 as gint);
    let mut last_match_type: gint = -(1 as gint);
    let mut results: *mut *mut *mut gchar = ::core::ptr::null_mut::<*mut *mut gchar>();
    let mut n_groups: gint = 0 as gint;
    let mut start_of_group: gint = 0;
    let mut i: gint = 0;
    let mut j: gint = 0;
    let mut k: guint = 0;
    search_tokens = g_str_tokenize_and_fold(
        search_string,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut *mut gchar>(),
    );
    safe_c2rust_desktop_file_dirs_lock();
    safe_c2rust_reset_total_search_results();
    k = 0 as guint;
    while k < (*safe_c2rust_desktop_file_dirs).len {
        j = 0 as ::core::ffi::c_int as gint;
        while !(*search_tokens.offset(j as isize)).is_null() {
            safe_c2rust_desktop_file_dir_search(
                *(*safe_c2rust_desktop_file_dirs).pdata.offset(k as isize) as *mut DesktopFileDir,
                *search_tokens.offset(j as isize),
            );
            safe_c2rust_merge_token_results((j == 0 as ::core::ffi::c_int) as ::core::ffi::c_int);
            j += 1;
        }
        safe_c2rust_merge_directory_results();
        k = k.wrapping_add(1);
    }
    safe_c2rust_sort_total_search_results();
    i = 0 as ::core::ffi::c_int as gint;
    while i < safe_c2rust_static_total_results_size {
        if (*safe_c2rust_static_total_results.offset(i as isize)).category != last_category
            || (*safe_c2rust_static_total_results.offset(i as isize)).match_type != last_match_type
        {
            last_category = (*safe_c2rust_static_total_results.offset(i as isize)).category;
            last_match_type = (*safe_c2rust_static_total_results.offset(i as isize)).match_type;
            n_groups += 1;
        }
        i += 1;
    }
    results = ({
        let mut __n: gsize = (n_groups as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut *mut gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut *mut gchar;
    start_of_group = 0 as ::core::ffi::c_int as gint;
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_groups {
        let mut n_items_in_group: gint = 0 as gint;
        let mut this_category: gint = 0;
        let mut this_match_type: gint = 0;
        let mut j_0: gint = 0;
        this_category =
            (*safe_c2rust_static_total_results.offset(start_of_group as isize)).category;
        this_match_type =
            (*safe_c2rust_static_total_results.offset(start_of_group as isize)).match_type;
        while start_of_group + n_items_in_group < safe_c2rust_static_total_results_size
            && (*safe_c2rust_static_total_results
                .offset((start_of_group + n_items_in_group) as isize))
            .category
                == this_category
            && (*safe_c2rust_static_total_results
                .offset((start_of_group + n_items_in_group) as isize))
            .match_type
                == this_match_type
        {
            n_items_in_group += 1;
        }
        let ref mut fresh32 = *results.offset(i as isize);
        *fresh32 = ({
            let mut __n: gsize =
                (n_items_in_group as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
        j_0 = 0 as ::core::ffi::c_int as gint;
        while j_0 < n_items_in_group {
            let ref mut fresh33 = *(*results.offset(i as isize)).offset(j_0 as isize);
            *fresh33 = safe_c2rust_g_strdup_inline(
                (*safe_c2rust_static_total_results.offset((start_of_group + j_0) as isize)).app_name
                    as *const ::core::ffi::c_char,
            ) as *mut gchar;
            j_0 += 1;
        }
        let ref mut fresh34 = *(*results.offset(i as isize)).offset(j_0 as isize);
        *fresh34 = ::core::ptr::null_mut::<gchar>();
        start_of_group += n_items_in_group;
        i += 1;
    }
    let ref mut fresh35 = *results.offset(i as isize);
    *fresh35 = ::core::ptr::null_mut::<*mut gchar>();
    safe_c2rust_desktop_file_dirs_unlock();
    g_strfreev(search_tokens);
    return results;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_all() -> *mut GList {
    let mut apps: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut i: guint = 0;
    let mut infos: *mut GList = ::core::ptr::null_mut::<GList>();
    apps = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    safe_c2rust_desktop_file_dirs_lock();
    i = 0 as guint;
    while i < (*safe_c2rust_desktop_file_dirs).len {
        safe_c2rust_desktop_file_dir_get_all(
            *(*safe_c2rust_desktop_file_dirs).pdata.offset(i as isize) as *mut DesktopFileDir,
            apps,
        );
        i = i.wrapping_add(1);
    }
    safe_c2rust_desktop_file_dirs_unlock();
    infos = ::core::ptr::null_mut::<GList>();
    g_hash_table_iter_init(&raw mut iter, apps);
    while g_hash_table_iter_next(
        &raw mut iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut value,
    ) != 0
    {
        if !value.is_null() {
            infos = g_list_prepend(infos, value);
        }
    }
    g_hash_table_destroy(apps);
    return infos;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_lookup_get_type() -> GType {
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
            g_intern_static_string(b"GDesktopAppInfoLookup\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GDesktopAppInfoLookupInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GDesktopAppInfoLookupInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_desktop_app_info_lookup_default_init
                        as unsafe extern "C" fn(*mut GDesktopAppInfoLookupInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL_1),
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
unsafe extern "C" fn safe_c2rust_g_desktop_app_info_lookup_default_init(
    mut iface: *mut GDesktopAppInfoLookupInterface,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_lookup_get_default_for_uri_scheme(
    mut lookup: *mut GDesktopAppInfoLookup,
    mut uri_scheme: *const ::core::ffi::c_char,
) -> *mut GAppInfo {
    let mut iface: *mut GDesktopAppInfoLookupIface =
        ::core::ptr::null_mut::<GDesktopAppInfoLookupIface>();
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = lookup as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_lookup_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO_LOOKUP (lookup)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    iface = g_type_interface_peek(
        (*(lookup as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_desktop_app_info_lookup_get_type(),
    ) as *mut GDesktopAppInfoLookupIface;
    return Some(
        (*iface)
            .get_default_for_uri_scheme
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(lookup, uri_scheme);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_startup_wm_class(
    mut info: *mut GDesktopAppInfo,
) -> *const ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*info).startup_wm_class;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_string(
    mut info: *mut GDesktopAppInfo,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return g_key_file_get_string(
        (*info).keyfile,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        key as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_locale_string(
    mut info: *mut GDesktopAppInfo,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !key.is_null() && *key as ::core::ffi::c_int != '\0' as i32 {
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
            b"key != NULL && *key != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return g_key_file_get_locale_string(
        (*info).keyfile,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        key as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_boolean(
    mut info: *mut GDesktopAppInfo,
    mut key: *const ::core::ffi::c_char,
) -> gboolean {
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_key_file_get_boolean(
        (*info).keyfile,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        key as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_string_list(
    mut info: *mut GDesktopAppInfo,
    mut key: *const ::core::ffi::c_char,
    mut length: *mut gsize,
) -> *mut *mut gchar {
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    return g_key_file_get_string_list(
        (*info).keyfile,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        key as *const gchar,
        length,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_has_key(
    mut info: *mut GDesktopAppInfo,
    mut key: *const ::core::ffi::c_char,
) -> gboolean {
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_key_file_has_key(
        (*info).keyfile,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        key as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_list_actions(
    mut info: *mut GDesktopAppInfo,
) -> *const *const gchar {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<*const gchar>();
    }
    return (*info).actions as *mut *const gchar;
}
unsafe extern "C" fn safe_c2rust_app_info_has_action(
    mut info: *mut GDesktopAppInfo,
    mut action_name: *const gchar,
) -> gboolean {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while !(*(*info).actions.offset(i as isize)).is_null() {
        if strcmp(
            *(*info).actions.offset(i as isize) as *const ::core::ffi::c_char,
            action_name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_get_action_name(
    mut info: *mut GDesktopAppInfo,
    mut action_name: *const gchar,
) -> *mut gchar {
    let mut group_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if safe_c2rust_app_info_has_action(info, action_name) != 0 {
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
            b"app_info_has_action (info, action_name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    group_name = g_strdup_printf(
        b"Desktop Action %s\0" as *const u8 as *const gchar,
        action_name,
    );
    result = g_key_file_get_locale_string(
        (*info).keyfile,
        group_name,
        b"Name\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_free(group_name as gpointer);
    if result.is_null() {
        result =
            safe_c2rust_g_strdup_inline(glib_gettext(b"Unnamed\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char) as *mut gchar;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_desktop_app_info_launch_action(
    mut info: *mut GDesktopAppInfo,
    mut action_name: *const gchar,
    mut launch_context: *mut GAppLaunchContext,
) {
    let mut session_bus: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_desktop_app_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
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
            b"G_IS_DESKTOP_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if safe_c2rust_app_info_has_action(info, action_name) != 0 {
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
            b"app_info_has_action (info, action_name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    session_bus = g_bus_get_sync(
        G_BUS_TYPE_SESSION,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !session_bus.is_null() && !(*info).app_id.is_null() {
        let mut object_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
        object_path = safe_c2rust_object_path_from_appid((*info).app_id);
        g_dbus_connection_call(
            session_bus,
            (*info).app_id,
            object_path,
            b"org.freedesktop.Application\0" as *const u8 as *const gchar,
            b"ActivateAction\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(sav@a{sv})\0" as *const u8 as *const gchar,
                action_name,
                NULL_1,
                safe_c2rust_g_desktop_app_info_make_platform_data(
                    info,
                    ::core::ptr::null_mut::<GList>(),
                    launch_context,
                ),
            ),
            ::core::ptr::null::<GVariantType>(),
            G_DBUS_CALL_FLAGS_NONE,
            -(1 as gint),
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_1,
        );
        g_free(object_path as gpointer);
    } else {
        let mut group_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut exec_line: *mut gchar = ::core::ptr::null_mut::<gchar>();
        group_name = g_strdup_printf(
            b"Desktop Action %s\0" as *const u8 as *const gchar,
            action_name,
        );
        exec_line = g_key_file_get_string(
            (*info).keyfile,
            group_name,
            b"Exec\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_free(group_name as gpointer);
        if !exec_line.is_null() {
            safe_c2rust_g_desktop_app_info_launch_uris_with_spawn(
                info,
                session_bus,
                exec_line,
                ::core::ptr::null_mut::<GList>(),
                launch_context,
                G_SPAWN_SEARCH_PATH,
                None,
                NULL_1,
                None,
                NULL_1,
                -(1 as gint),
                -(1 as gint),
                -(1 as gint),
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
        g_free(exec_line as gpointer);
    }
    if !session_bus.is_null() {
        g_dbus_connection_flush(
            session_bus,
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_1,
        );
        g_object_unref(session_bus as gpointer);
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const GIO_LAUNCH_DESKTOP: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<[u8; 38], [::core::ffi::c_char; 38]>(
        *b"/usr/local/libexec/gio-launch-desktop\0",
    )
};
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
