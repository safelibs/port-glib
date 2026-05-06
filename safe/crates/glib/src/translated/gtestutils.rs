use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
use ::f128;
use ::num_traits;
use ::num_traits::ToPrimitive;
extern "C" {
    pub type _GRand;
    pub type _GDir;
    pub type _GTimer;
    pub type _GIConv;
    pub type _GSourcePrivate;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_get_prgname() -> *const gchar;
    fn g_snprintf(string: *mut gchar, n: gulong, format: *const gchar, ...) -> gint;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strsignal(signum: gint) -> *const gchar;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_ascii_strtoull(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> guint64;
    fn g_strchug(string: *mut gchar) -> *mut gchar;
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strescape(source: *const gchar, exceptions: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strjoinv(separator: *const gchar, str_array: *mut *mut gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_new_len(init: *const gchar, len: gssize) -> *mut GString;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_prepend(string: *mut GString, val: *const gchar) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_overwrite_len(
        string: *mut GString,
        pos: gsize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_erase(string: *mut GString, pos: gssize, len: gssize) -> *mut GString;
    fn g_string_printf(string: *mut GString, format: *const gchar, ...);
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_log_default_handler(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        message: *const gchar,
        unused_data: gpointer,
    );
    fn g_log_set_default_handler(log_func: GLogFunc, user_data: gpointer) -> GLogFunc;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_log_set_always_fatal(fatal_mask: GLogLevelFlags) -> GLogLevelFlags;
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
    fn g_print(format: *const gchar, ...);
    fn g_set_print_handler(func: GPrintFunc) -> GPrintFunc;
    fn g_printerr(format: *const gchar, ...);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
    fn g_slist_find_custom(
        list: *mut GSList,
        data: gconstpointer,
        func: GCompareFunc,
    ) -> *mut GSList;
    fn g_slist_last(list: *mut GSList) -> *mut GSList;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_test_log_set_fatal_handler(log_func: GTestLogFatalFunc, user_data: gpointer);
    fn g_file_error_quark() -> GQuark;
    fn g_file_error_from_errno(err_no: gint) -> GFileError;
    fn g_dir_make_tmp(tmpl: *const gchar, error: *mut *mut GError) -> *mut gchar;
    fn g_build_path(separator: *const gchar, first_element: *const gchar, ...) -> *mut gchar;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_build_filenamev(args: *mut *mut gchar) -> *mut gchar;
    fn g_mkdir_with_parents(pathname: *const gchar, mode: gint) -> gint;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_get_current_dir() -> *mut gchar;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    static mut safe_c2rust_stdout: *mut FILE;
    static mut safe_c2rust_stderr: *mut FILE;
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn prctl(__option: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> ::core::ffi::c_int;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_dir_open(path: *const gchar, flags: guint, error: *mut *mut GError) -> *mut GDir;
    fn g_dir_read_name(dir: *mut GDir) -> *const gchar;
    fn g_dir_close(dir: *mut GDir);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_setenv(variable: *const gchar, value: *const gchar, overwrite: gboolean) -> gboolean;
    fn g_main_context_new() -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    fn g_main_loop_run(loop_0: *mut GMainLoop);
    fn g_main_loop_quit(loop_0: *mut GMainLoop);
    fn g_main_loop_unref(loop_0: *mut GMainLoop);
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
    fn g_child_watch_source_new(pid: GPid) -> *mut GSource;
    fn g_timeout_source_new(interval: guint) -> *mut GSource;
    fn g_get_monotonic_time() -> gint64;
    fn g_io_channel_unref(channel: *mut GIOChannel);
    fn g_io_create_watch(channel: *mut GIOChannel, condition: GIOCondition) -> *mut GSource;
    fn g_io_channel_set_buffered(channel: *mut GIOChannel, buffered: gboolean);
    fn g_io_channel_set_encoding(
        channel: *mut GIOChannel,
        encoding: *const gchar,
        error: *mut *mut GError,
    ) -> GIOStatus;
    fn g_io_channel_set_close_on_unref(channel: *mut GIOChannel, do_close: gboolean);
    fn g_io_channel_read_chars(
        channel: *mut GIOChannel,
        buf: *mut gchar,
        count: gsize,
        bytes_read: *mut gsize,
        error: *mut *mut GError,
    ) -> GIOStatus;
    fn g_io_channel_unix_new(fd: ::core::ffi::c_int) -> *mut GIOChannel;
    fn g_pattern_match_simple(pattern: *const gchar, string: *const gchar) -> gboolean;
    fn g_rand_new_with_seed(seed: guint32) -> *mut GRand;
    fn g_rand_new_with_seed_array(seed: *const guint32, seed_length: guint) -> *mut GRand;
    fn g_rand_free(rand_: *mut GRand);
    fn g_rand_int(rand_: *mut GRand) -> guint32;
    fn g_rand_int_range(rand_: *mut GRand, begin: gint32, end: gint32) -> gint32;
    fn g_rand_double(rand_: *mut GRand) -> gdouble;
    fn g_rand_double_range(rand_: *mut GRand, begin: gdouble, end: gdouble) -> gdouble;
    fn g_random_int() -> guint32;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_spawn_async_with_pipes(
        working_directory: *const gchar,
        argv: *mut *mut gchar,
        envp: *mut *mut gchar,
        flags: GSpawnFlags,
        child_setup: GSpawnChildSetupFunc,
        user_data: gpointer,
        child_pid: *mut GPid,
        standard_input: *mut gint,
        standard_output: *mut gint,
        standard_error: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_timer_new() -> *mut GTimer;
    fn g_timer_destroy(timer: *mut GTimer);
    fn g_timer_start(timer: *mut GTimer);
    fn g_timer_stop(timer: *mut GTimer);
    fn g_timer_elapsed(timer: *mut GTimer, microseconds: *mut gulong) -> gdouble;
    fn g_rmdir(filename: *const gchar) -> ::core::ffi::c_int;
    fn g_set_user_dirs(first_dir_type: *const gchar, ...);
    fn _g_unset_cached_tmp_dir();
    fn g_set_prgname_once(prgname: *const gchar) -> gboolean;
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
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type GPid = ::core::ffi::c_int;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type ssize_t = isize;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GLogFunc =
    Option<unsafe extern "C" fn(*const gchar, GLogLevelFlags, *const gchar, gpointer) -> ()>;
pub type GPrintFunc = Option<unsafe extern "C" fn(*const gchar) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GTestCase {
    pub name: *mut gchar,
    pub fixture_size: guint,
    pub fixture_setup: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, gconstpointer) -> ()>,
    pub fixture_test: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, gconstpointer) -> ()>,
    pub fixture_teardown:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, gconstpointer) -> ()>,
    pub test_data: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GTestSuite {
    pub name: *mut gchar,
    pub suites: *mut GSList,
    pub cases: *mut GSList,
}
pub type GTestFunc = Option<unsafe extern "C" fn() -> ()>;
pub type GTestDataFunc = Option<unsafe extern "C" fn(gconstpointer) -> ()>;
pub type GTestFixtureFunc = Option<unsafe extern "C" fn(gpointer, gconstpointer) -> ()>;
pub type GTestLogType = ::core::ffi::c_uint;
pub const G_TEST_LOG_STOP_SUITE: GTestLogType = 11;
pub const G_TEST_LOG_START_SUITE: GTestLogType = 10;
pub const G_TEST_LOG_MESSAGE: GTestLogType = 9;
pub const G_TEST_LOG_MAX_RESULT: GTestLogType = 8;
pub const G_TEST_LOG_MIN_RESULT: GTestLogType = 7;
pub const G_TEST_LOG_STOP_CASE: GTestLogType = 6;
pub const G_TEST_LOG_START_CASE: GTestLogType = 5;
pub const G_TEST_LOG_SKIP_CASE: GTestLogType = 4;
pub const G_TEST_LOG_LIST_CASE: GTestLogType = 3;
pub const G_TEST_LOG_START_BINARY: GTestLogType = 2;
pub const G_TEST_LOG_ERROR: GTestLogType = 1;
pub const G_TEST_LOG_NONE: GTestLogType = 0;

#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log(
    lbit: GTestLogType,
    string1: *const gchar,
    string2: *const gchar,
    n_nums: guint,
    nums: *mut ::f128::f128,
) {
    let subtest_level: ::core::ffi::c_uint = if safe_c2rust_is_subtest() != 0 {
        1 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    };
    if safe_c2rust_g_default_print_func.is_none() {
        safe_c2rust_g_default_print_func = g_set_print_handler(Some(
            safe_c2rust_g_test_print_handler as unsafe extern "C" fn(*const gchar) -> (),
        ));
    }
    match lbit as ::core::ffi::c_uint {
        G_TEST_LOG_START_BINARY => {
            if safe_c2rust_test_tap_log != 0 {
                if safe_c2rust_is_subtest() == 0 {
                    safe_c2rust_g_test_tap_print(
                        0 as ::core::ffi::c_uint,
                        FALSE,
                        b"TAP version 13\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    safe_c2rust_g_test_tap_print(
                        subtest_level.saturating_sub(1),
                        TRUE,
                        b"Subtest: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        safe_c2rust_test_argv0,
                    );
                }
                g_print(
                    b"random seed: %s\n\0" as *const u8 as *const gchar,
                    string2,
                );
            } else if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
                g_print(
                    b"GTest: random seed: %s\n\0" as *const u8 as *const gchar,
                    string2,
                );
            }
        }
        G_TEST_LOG_START_SUITE => {
            if safe_c2rust_test_tap_log != 0 && !string1.is_null() {
                if *string1 as ::core::ffi::c_int != 0 {
                    g_print(
                        b"Start of %s tests\n\0" as *const u8 as *const gchar,
                        string1,
                    );
                } else if safe_c2rust_test_paths.is_null() {
                    safe_c2rust_g_test_tap_print(
                        subtest_level,
                        FALSE,
                        b"1..%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                        safe_c2rust_test_count as ::core::ffi::c_int,
                    );
                }
            }
        }
        G_TEST_LOG_STOP_SUITE => {
            if safe_c2rust_test_tap_log != 0 && !string1.is_null() {
                if *string1 as ::core::ffi::c_int != 0 {
                    g_print(
                        b"End of %s tests\n\0" as *const u8 as *const gchar,
                        string1,
                    );
                } else if !safe_c2rust_test_paths.is_null() {
                    safe_c2rust_g_test_tap_print(
                        subtest_level,
                        FALSE,
                        b"1..%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                        safe_c2rust_test_run_count as ::core::ffi::c_int,
                    );
                }
            }
        }
        G_TEST_LOG_STOP_CASE => {
            let result: GTestResult = if !nums.is_null()
                && n_nums > G_TEST_CASE_LARGS_RESULT as guint
            {
                (*nums.offset(G_TEST_CASE_LARGS_RESULT as isize))
                    .to_u32()
                    .unwrap_or(G_TEST_RUN_FAILURE)
            } else {
                G_TEST_RUN_FAILURE
            };
            let result_index: usize = if result <= G_TEST_RUN_INCOMPLETE {
                result as usize
            } else {
                G_TEST_RUN_FAILURE as usize
            };
            let timing: ::core::ffi::c_double = if !nums.is_null()
                && n_nums > G_TEST_CASE_LARGS_EXECUTION_TIME as guint
            {
                (*nums.offset(G_TEST_CASE_LARGS_EXECUTION_TIME as isize))
                    .to_f64()
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            let fail = result == G_TEST_RUN_FAILURE;
            if safe_c2rust_test_tap_log != 0 {
                let tap_output = if fail || result == G_TEST_RUN_INCOMPLETE {
                    g_string_new(b"not ok\0" as *const u8 as *const gchar)
                } else {
                    g_string_new(b"ok\0" as *const u8 as *const gchar)
                };
                if safe_c2rust_is_subtest() != 0 {
                    g_string_prepend(tap_output, TAP_SUBTEST_PREFIX.as_ptr());
                }
                g_string_append_printf(
                    tap_output,
                    b" %d %s\0" as *const u8 as *const gchar,
                    safe_c2rust_test_run_count as ::core::ffi::c_int,
                    string1,
                );
                if result == G_TEST_RUN_INCOMPLETE {
                    g_string_append_printf(
                        tap_output,
                        b" # TODO %s\0" as *const u8 as *const gchar,
                        if string2.is_null() {
                            b"\0" as *const u8 as *const gchar
                        } else {
                            string2
                        },
                    );
                } else if result == G_TEST_RUN_SKIPPED {
                    g_string_append_printf(
                        tap_output,
                        b" # SKIP %s\0" as *const u8 as *const gchar,
                        if string2.is_null() {
                            b"\0" as *const u8 as *const gchar
                        } else {
                            string2
                        },
                    );
                } else if result == G_TEST_RUN_FAILURE && !string2.is_null() {
                    g_string_append_printf(
                        tap_output,
                        b" - %s\0" as *const u8 as *const gchar,
                        string2,
                    );
                }
                safe_c2rust_g_string_append_c_inline(tap_output, '\n' as i32 as gchar);
                safe_c2rust_g_default_print_func
                    .expect("non-null function pointer")((*tap_output).str_0);
                g_string_free(tap_output, TRUE);
                if timing > 0.5f64 {
                    let slow_output = g_string_new(b"# \0" as *const u8 as *const gchar);
                    g_string_append_printf(
                        slow_output,
                        b"slow test %s executed in %0.2lf secs\n\0" as *const u8 as *const gchar,
                        string1,
                        timing,
                    );
                    safe_c2rust_g_default_print_func
                        .expect("non-null function pointer")((*slow_output).str_0);
                    g_string_free(slow_output, TRUE);
                }
            } else if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
                g_print(
                    b"GTest: result: %s\n\0" as *const u8 as *const gchar,
                    safe_c2rust_g_test_result_names[result_index],
                );
            } else if (*safe_c2rust_g_test_config_vars).test_quiet == 0
                && safe_c2rust_test_in_subprocess == 0
            {
                g_print(
                    b"%s\n\0" as *const u8 as *const gchar,
                    safe_c2rust_g_test_result_names[result_index],
                );
            }
            if fail && safe_c2rust_test_mode_fatal != 0 {
                if safe_c2rust_test_tap_log != 0 {
                    safe_c2rust_g_test_tap_print(
                        0 as ::core::ffi::c_uint,
                        FALSE,
                        b"Bail out!\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                abort();
            }
            if result == G_TEST_RUN_SKIPPED || result == G_TEST_RUN_INCOMPLETE {
                safe_c2rust_test_skipped_count = safe_c2rust_test_skipped_count.wrapping_add(1);
            }
        }
        G_TEST_LOG_SKIP_CASE => {
            if safe_c2rust_test_tap_log != 0 {
                safe_c2rust_g_test_tap_print(
                    subtest_level,
                    FALSE,
                    b"ok %d %s # SKIP\n\0" as *const u8 as *const ::core::ffi::c_char,
                    safe_c2rust_test_run_count as ::core::ffi::c_int,
                    string1,
                );
            }
        }
        G_TEST_LOG_MIN_RESULT => {
            if safe_c2rust_test_tap_log != 0 {
                g_print(b"min perf: %s\n\0" as *const u8 as *const gchar, string1);
            } else if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
                g_print(b"(MINPERF:%s)\n\0" as *const u8 as *const gchar, string1);
            }
        }
        G_TEST_LOG_MAX_RESULT => {
            if safe_c2rust_test_tap_log != 0 {
                g_print(b"max perf: %s\n\0" as *const u8 as *const gchar, string1);
            } else if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
                g_print(b"(MAXPERF:%s)\n\0" as *const u8 as *const gchar, string1);
            }
        }
        G_TEST_LOG_MESSAGE => {
            if safe_c2rust_test_tap_log != 0 {
                g_print(b"%s\n\0" as *const u8 as *const gchar, string1);
            } else if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
                g_print(b"(MSG: %s)\n\0" as *const u8 as *const gchar, string1);
            }
        }
        G_TEST_LOG_ERROR => {
            if safe_c2rust_test_tap_log != 0 {
                let mut message = if string1.is_null() {
                    ::core::ptr::null_mut::<gchar>()
                } else {
                    g_strdup(string1)
                };
                if !message.is_null() {
                    let mut line = message as *mut ::core::ffi::c_char;
                    loop {
                        line = strchr(line, '\n' as i32);
                        if line.is_null() {
                            break;
                        }
                        *line = ' ' as i32 as gchar;
                        line = line.offset(1);
                    }
                    message = g_strchomp(g_strchug(message));
                }
                if !safe_c2rust_test_run_name.is_null()
                    && *safe_c2rust_test_run_name as ::core::ffi::c_int != 0
                {
                    if !message.is_null() && *message as ::core::ffi::c_int != 0 {
                        safe_c2rust_g_test_tap_print(
                            subtest_level,
                            FALSE,
                            b"not ok %s - %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                            safe_c2rust_test_run_name,
                            message,
                        );
                    } else {
                        safe_c2rust_g_test_tap_print(
                            subtest_level,
                            FALSE,
                            b"not ok %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                            safe_c2rust_test_run_name,
                        );
                    }
                    g_free(message as gpointer);
                    message = ::core::ptr::null_mut::<gchar>();
                }
                if !message.is_null() && *message as ::core::ffi::c_int != 0 {
                    safe_c2rust_g_test_tap_print(
                        subtest_level,
                        FALSE,
                        b"Bail out! %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        message,
                    );
                } else {
                    safe_c2rust_g_test_tap_print(
                        subtest_level,
                        FALSE,
                        b"Bail out!\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                g_free(message as gpointer);
            } else if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
                g_print(b"(ERROR: %s)\n\0" as *const u8 as *const gchar, string1);
            }
        }
        _ => {}
    }
    let mut astrings: [*mut gchar; 3] = [
        ::core::ptr::null_mut::<gchar>(),
        ::core::ptr::null_mut::<gchar>(),
        ::core::ptr::null_mut::<gchar>(),
    ];
    let mut len: guint = 0;
    let mut msg = GTestLogMsg {
        log_type: lbit,
        n_strings: 0,
        strings: astrings.as_mut_ptr(),
        n_nums,
        nums,
    };
    if !string1.is_null() {
        astrings[0] = string1 as *mut gchar;
        msg.n_strings = 1;
        if !string2.is_null() {
            astrings[1] = string2 as *mut gchar;
            msg.n_strings = 2;
        }
    }
    let buffer = safe_c2rust_g_test_log_dump(&raw mut msg, &raw mut len);
    safe_c2rust_g_test_log_send(len, buffer);
    g_free(buffer as gpointer);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GTestConfig {
    pub test_initialized: gboolean,
    pub test_quick: gboolean,
    pub test_perf: gboolean,
    pub test_verbose: gboolean,
    pub test_quiet: gboolean,
    pub test_undefined: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GTestLogMsg {
    pub log_type: GTestLogType,
    pub n_strings: guint,
    pub strings: *mut *mut gchar,
    pub n_nums: guint,
    pub nums: *mut ::f128::f128,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GTestLogBuffer {
    pub data: *mut GString,
    pub msgs: *mut GSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub vuint64: guint64,
    pub vdouble: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub vdouble: ::core::ffi::c_double,
    pub vuint64: guint64,
}
pub type GTestResult = ::core::ffi::c_uint;
pub const G_TEST_RUN_INCOMPLETE: GTestResult = 3;
pub const G_TEST_RUN_FAILURE: GTestResult = 2;
pub const G_TEST_RUN_SKIPPED: GTestResult = 1;
pub const G_TEST_RUN_SUCCESS: GTestResult = 0;
pub const G_TEST_CASE_LARGS_EXECUTION_TIME: C2RustUnnamed_1 = 2;
pub const G_TEST_CASE_LARGS_RESULT: C2RustUnnamed_1 = 0;
pub type GRand = _GRand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type rlim_t = __rlim64_t;
pub type __rlimit_resource_t = __rlimit_resource;
pub type __rlimit_resource = ::core::ffi::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type GDir = _GDir;
pub type GTimer = _GTimer;
pub const G_TEST_CASE_LARGS_RUN_FORKS: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DestroyEntry {
    pub next: *mut DestroyEntry,
    pub destroy_func: GDestroyNotify,
    pub destroy_data: gpointer,
}
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
pub type GTestLogFatalFunc =
    Option<unsafe extern "C" fn(*const gchar, GLogLevelFlags, *const gchar, gpointer) -> gboolean>;
pub type GTestTrapFlags = ::core::ffi::c_uint;
pub const G_TEST_TRAP_INHERIT_STDIN: GTestTrapFlags = 512;
pub const G_TEST_TRAP_SILENCE_STDERR: GTestTrapFlags = 256;
pub const G_TEST_TRAP_SILENCE_STDOUT: GTestTrapFlags = 128;
pub const G_TEST_TRAP_DEFAULT: GTestTrapFlags = 0;
pub type GIOChannel = _GIOChannel;
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
pub type GIConv = *mut _GIConv;
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
pub type GIOCondition = ::core::ffi::c_uint;
pub const G_IO_NVAL: GIOCondition = 32;
pub const G_IO_HUP: GIOCondition = 16;
pub const G_IO_ERR: GIOCondition = 8;
pub const G_IO_PRI: GIOCondition = 2;
pub const G_IO_OUT: GIOCondition = 4;
pub const G_IO_IN: GIOCondition = 1;
pub type GSource = _GSource;
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
pub type GMainContext = _GMainContext;
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
pub type GSeekType = ::core::ffi::c_uint;
pub const G_SEEK_END: GSeekType = 2;
pub const G_SEEK_SET: GSeekType = 1;
pub const G_SEEK_CUR: GSeekType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaitForChildData {
    pub pid: GPid,
    pub loop_0: *mut GMainLoop,
    pub child_status: ::core::ffi::c_int,
    pub stdout_io: *mut GIOChannel,
    pub echo_stdout: gboolean,
    pub stdout_str: *mut GString,
    pub stderr_io: *mut GIOChannel,
    pub echo_stderr: gboolean,
    pub stderr_str: *mut GString,
}
pub type GMainLoop = _GMainLoop;
pub type FILE = _IO_FILE;
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
pub type GTestSubprocessFlags = ::core::ffi::c_uint;
pub const G_TEST_SUBPROCESS_INHERIT_STDERR: GTestSubprocessFlags = 4;
pub const G_TEST_SUBPROCESS_INHERIT_STDOUT: GTestSubprocessFlags = 2;
pub const G_TEST_SUBPROCESS_INHERIT_STDIN: GTestSubprocessFlags = 1;
pub const G_TEST_SUBPROCESS_DEFAULT: GTestSubprocessFlags = 0;
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
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type uint64_t = __uint64_t;
pub type GTestFileType = ::core::ffi::c_uint;
pub const G_TEST_BUILT: GTestFileType = 1;
pub const G_TEST_DIST: GTestFileType = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_TEST_CASE_LARGS_MAX: C2RustUnnamed_1 = 3;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_test_log_msg_free\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __WCOREFLAG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_truncate_inline(
    mut gstring: *mut GString,
    mut len: gsize,
) -> *mut GString {
    (*gstring).len = if len < (*gstring).len {
        len
    } else {
        (*gstring).len
    };
    *(*gstring).str_0.offset((*gstring).len as isize) = '\0' as i32 as gchar;
    return gstring;
}
pub const SIGALRM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const G_TEST_OPTION_ISOLATE_DIRS: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"isolate_dirs\0") };
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PR_SET_DUMPABLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TAP_SUBTEST_PREFIX: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"#    \0") };
#[no_mangle]
pub static mut safe_c2rust___glib_assert_msg: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut safe_c2rust_g_test_result_names: [*const ::core::ffi::c_char; 4] = [
    b"OK\0" as *const u8 as *const ::core::ffi::c_char,
    b"SKIP\0" as *const u8 as *const ::core::ffi::c_char,
    b"FAIL\0" as *const u8 as *const ::core::ffi::c_char,
    b"TODO\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut safe_c2rust_test_log_fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut safe_c2rust_test_mode_fatal: gboolean = TRUE;
static mut safe_c2rust_g_test_run_once: gboolean = TRUE;
static mut safe_c2rust_test_isolate_dirs: gboolean = FALSE;
static mut safe_c2rust_test_isolate_dirs_tmpdir: *mut gchar =
    ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_test_tmpdir: *const gchar = ::core::ptr::null::<gchar>();
static mut safe_c2rust_test_run_list: gboolean = FALSE;
static mut safe_c2rust_test_run_seedstr: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_g__test_run_rand_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_test_run_rand: *mut GRand = ::core::ptr::null::<GRand>() as *mut GRand;
static mut safe_c2rust_test_run_name: *mut gchar =
    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
static mut safe_c2rust_test_run_name_path: *mut gchar =
    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
static mut safe_c2rust_test_filename_free_list: *mut *mut GSList =
    ::core::ptr::null::<*mut GSList>() as *mut *mut GSList;
static mut safe_c2rust_test_run_forks: guint = 0 as guint;
static mut safe_c2rust_test_run_count: guint = 0 as guint;
static mut safe_c2rust_test_count: guint = 0 as guint;
static mut safe_c2rust_test_skipped_count: guint = 0 as guint;
static mut safe_c2rust_test_run_success: GTestResult = G_TEST_RUN_FAILURE;
static mut safe_c2rust_test_run_msg: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_test_startup_skip_count: guint = 0 as guint;
static mut safe_c2rust_test_user_timer: *mut GTimer = ::core::ptr::null::<GTimer>() as *mut GTimer;
static mut safe_c2rust_test_user_stamp: ::core::ffi::c_double =
    0 as ::core::ffi::c_int as ::core::ffi::c_double;
static mut safe_c2rust_test_paths: *mut GSList = ::core::ptr::null::<GSList>() as *mut GSList;
static mut safe_c2rust_test_prefix: gboolean = FALSE;
static mut safe_c2rust_test_prefix_extended: gboolean = FALSE;
static mut safe_c2rust_test_paths_skipped: *mut GSList =
    ::core::ptr::null::<GSList>() as *mut GSList;
static mut safe_c2rust_test_prefix_skipped: gboolean = FALSE;
static mut safe_c2rust_test_prefix_extended_skipped: gboolean = FALSE;
static mut safe_c2rust_test_suite_root: *mut GTestSuite =
    ::core::ptr::null::<GTestSuite>() as *mut GTestSuite;
static mut safe_c2rust_test_trap_last_status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut safe_c2rust_test_trap_last_pid: GPid = 0 as GPid;
static mut safe_c2rust_test_trap_last_subprocess: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut safe_c2rust_test_trap_last_stdout: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut safe_c2rust_test_trap_last_stderr: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut safe_c2rust_test_uri_base: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut safe_c2rust_test_debug_log: gboolean = FALSE;
static mut safe_c2rust_test_tap_log: gboolean = TRUE;
static mut safe_c2rust_test_nonfatal_assertions: gboolean = FALSE;
static mut safe_c2rust_test_destroy_queue: *mut DestroyEntry =
    ::core::ptr::null::<DestroyEntry>() as *mut DestroyEntry;
static mut safe_c2rust_test_argv0: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
static mut safe_c2rust_test_argv0_dirname: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut safe_c2rust_test_disted_files_dir: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
static mut safe_c2rust_test_built_files_dir: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
static mut safe_c2rust_test_initial_cwd: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut safe_c2rust_test_in_forked_child: gboolean = FALSE;
static mut safe_c2rust_test_in_subprocess: gboolean = FALSE;
static mut safe_c2rust_test_is_subtest: gboolean = FALSE;
static mut safe_c2rust_mutable_test_config_vars: GTestConfig = GTestConfig {
    test_initialized: FALSE,
    test_quick: TRUE,
    test_perf: FALSE,
    test_verbose: FALSE,
    test_quiet: FALSE,
    test_undefined: TRUE,
};
#[no_mangle]
pub static mut safe_c2rust_g_test_config_vars: *const GTestConfig =
    unsafe { &raw const safe_c2rust_mutable_test_config_vars as *mut GTestConfig };
static mut safe_c2rust_no_g_set_prgname: gboolean = FALSE;
static mut safe_c2rust_g_default_print_func: GPrintFunc = None;
#[inline]
unsafe extern "C" fn safe_c2rust_is_subtest() -> gboolean {
    return (safe_c2rust_test_is_subtest != 0
        || safe_c2rust_test_in_forked_child != 0
        || safe_c2rust_test_in_subprocess != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_test_print_handler_full(
    mut string: *const gchar,
    mut use_tap_format: gboolean,
    mut is_tap_comment: gboolean,
    mut subtest_level: ::core::ffi::c_uint,
) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            767 as ::core::ffi::c_int,
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if use_tap_format != 0 {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
        && !strchr(string as *const ::core::ffi::c_char, '\n' as i32).is_null()
    {
        static mut safe_c2rust_last_had_final_newline: gboolean = TRUE;
        let mut output: *mut GString = g_string_new_len(
            ::core::ptr::null::<gchar>(),
            strlen(string as *const ::core::ffi::c_char).wrapping_add(2 as size_t) as gssize,
        );
        let mut line: *const ::core::ffi::c_char = string as *const ::core::ffi::c_char;
        loop {
            let mut next: *const ::core::ffi::c_char = strchr(line, '\n' as i32);
            if safe_c2rust_last_had_final_newline != 0
                && (!next.is_null() || *line as ::core::ffi::c_int != '\0' as i32)
            {
                let mut l: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                while l < subtest_level {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                b"#    \0" as *const u8 as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                output,
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
                            output,
                            b"#    \0" as *const u8 as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    l = l.wrapping_add(1);
                }
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if is_tap_comment != 0 {
                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_12
                }) as ::core::ffi::c_long
                    != 0
                {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                b"# \0" as *const u8 as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                output,
                                __val,
                                if ({
                                    let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_13
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
                            output,
                            b"# \0" as *const u8 as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                }
            }
            if !next.is_null() {
                next = next.offset(1 as ::core::ffi::c_int as isize);
                safe_c2rust_g_string_append_len_inline(
                    output,
                    line,
                    next.offset_from(line) as gssize,
                );
            } else {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = line;
                        safe_c2rust_g_string_append_len_inline(
                            output,
                            __val,
                            if ({
                                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_14
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
                        output,
                        line,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                safe_c2rust_last_had_final_newline =
                    (*line as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int as gboolean;
            }
            line = next;
            if line.is_null() {
                break;
            }
        }
        safe_c2rust_g_default_print_func.expect("non-null function pointer")((*output).str_0);
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    safe_c2rust_g_steal_pointer(&raw mut output as gpointer) as *mut GString,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal(
                    safe_c2rust_g_steal_pointer(&raw mut output as gpointer) as *mut GString
                );
            };
        } else {
            g_string_free(
                safe_c2rust_g_steal_pointer(&raw mut output as gpointer) as *mut GString,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
    } else {
        safe_c2rust_g_default_print_func.expect("non-null function pointer")(string);
    };
}
unsafe extern "C" fn safe_c2rust_g_test_print_handler(mut string: *const gchar) {
    safe_c2rust_g_test_print_handler_full(
        string,
        safe_c2rust_test_tap_log,
        TRUE,
        (if safe_c2rust_is_subtest() != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn safe_c2rust_g_test_tap_print(
    mut subtest_level: ::core::ffi::c_uint,
    mut commented: gboolean,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    let mut string: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    args_0 = args.clone();
    string =
        g_strdup_vprintf(format as *const gchar, args_0.clone()) as *mut ::core::ffi::c_char;
    safe_c2rust_g_test_print_handler_full(string, TRUE, commented, subtest_level);
    g_free(string as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log_type_name(
    mut log_type: GTestLogType,
) -> *const ::core::ffi::c_char {
    match log_type as ::core::ffi::c_uint {
        0 => return b"none\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"error\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"binary\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"list\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"skip\0" as *const u8 as *const ::core::ffi::c_char,
        5 => return b"start\0" as *const u8 as *const ::core::ffi::c_char,
        6 => return b"stop\0" as *const u8 as *const ::core::ffi::c_char,
        7 => return b"minperf\0" as *const u8 as *const ::core::ffi::c_char,
        8 => return b"maxperf\0" as *const u8 as *const ::core::ffi::c_char,
        9 => return b"message\0" as *const u8 as *const ::core::ffi::c_char,
        10 => return b"start suite\0" as *const u8 as *const ::core::ffi::c_char,
        11 => return b"stop suite\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {}
    }
    return b"???\0" as *const u8 as *const ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_test_log_send(mut n_bytes: guint, mut buffer: *const guint8) {
    if safe_c2rust_test_log_fd >= 0 as ::core::ffi::c_int {
        let mut r: ::core::ffi::c_int = 0;
        loop {
            r = write(
                safe_c2rust_test_log_fd,
                buffer as *const ::core::ffi::c_void,
                n_bytes as size_t,
            ) as ::core::ffi::c_int;
            if !(r < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
    }
    if safe_c2rust_test_debug_log != 0 {
        let mut lbuffer: *mut GTestLogBuffer = safe_c2rust_g_test_log_buffer_new();
        let mut msg: *mut GTestLogMsg = ::core::ptr::null_mut::<GTestLogMsg>();
        let mut output: *mut GString = ::core::ptr::null_mut::<GString>();
        let mut ui: guint = 0;
        safe_c2rust_g_test_log_buffer_push(lbuffer, n_bytes, buffer);
        msg = safe_c2rust_g_test_log_buffer_pop(lbuffer);
        if !(({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if !msg.is_null() {
                _g_boolean_var_15 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_15 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_15
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                875 as ::core::ffi::c_int,
                G_STRFUNC,
                b"msg != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(({
            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
            if (*(*lbuffer).data).len == 0 as gsize {
                _g_boolean_var_16 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_16 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_16
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                876 as ::core::ffi::c_int,
                G_STRFUNC,
                b"lbuffer->data->len == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        safe_c2rust_g_test_log_buffer_free(lbuffer);
        output = g_string_new(::core::ptr::null::<gchar>());
        g_string_printf(
            output,
            b"{*LOG(%s)\0" as *const u8 as *const gchar,
            safe_c2rust_g_test_log_type_name((*msg).log_type),
        );
        ui = 0 as guint;
        while ui < (*msg).n_strings {
            g_string_append_printf(
                output,
                b":{%s}\0" as *const u8 as *const gchar,
                *(*msg).strings.offset(ui as isize),
            );
            ui = ui.wrapping_add(1);
        }
        if (*msg).n_nums != 0 {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b":(\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        output,
                        __val,
                        if ({
                            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_17
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
                    output,
                    b":(\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            ui = 0 as guint;
            while ui < (*msg).n_nums {
                if ::f128::f128::new((*(*msg).nums.offset(ui as isize)).to_i64().unwrap())
                    == *(*msg).nums.offset(ui as isize)
                {
                    g_string_append_printf(
                        output,
                        b"%s%ld\0" as *const u8 as *const gchar,
                        if ui != 0 {
                            b";\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        (*(*msg).nums.offset(ui as isize)).to_i64().unwrap(),
                    );
                } else {
                    g_string_append_printf(
                        output,
                        b"%s%.16g\0" as *const u8 as *const gchar,
                        if ui != 0 {
                            b";\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        (*(*msg).nums.offset(ui as isize)).to_f64().unwrap(),
                    );
                }
                ui = ui.wrapping_add(1);
            }
            safe_c2rust_g_string_append_c_inline(output, ')' as i32 as gchar);
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b":LOG*}\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    output,
                    __val,
                    if ({
                        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_18
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
                output,
                b":LOG*}\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_printerr(b"%s\n\0" as *const u8 as *const gchar, (*output).str_0);
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(output, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(output);
            };
        } else {
            g_string_free(output, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        safe_c2rust_g_test_log_msg_free(msg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_disable_crash_reporting() {
    let mut limit: rlimit = rlimit {
        rlim_cur: 0 as rlim_t,
        rlim_max: 0 as rlim_t,
    };
    setrlimit(RLIMIT_CORE, &raw mut limit);
    prctl(
        PR_SET_DUMPABLE,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn safe_c2rust_parse_args(
    mut argc_p: *mut gint,
    mut argv_p: *mut *mut *mut gchar,
) {
    let mut argc: guint = *argc_p as guint;
    let mut argv: *mut *mut gchar = *argv_p;
    let mut i: guint = 0;
    let mut e: guint = 0;
    safe_c2rust_test_argv0 = *argv.offset(0 as ::core::ffi::c_int as isize);
    safe_c2rust_test_initial_cwd = g_get_current_dir() as *mut ::core::ffi::c_char;
    i = 1 as guint;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"--g-fatal-warnings\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut fatal_mask: GLogLevelFlags = g_log_set_always_fatal(
                (G_LOG_FLAG_RECURSION as ::core::ffi::c_int
                    | G_LOG_LEVEL_ERROR as ::core::ffi::c_int) as GLogLevelFlags,
            );
            fatal_mask = (fatal_mask as ::core::ffi::c_int
                | G_LOG_LEVEL_WARNING as ::core::ffi::c_int
                | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int)
                as GLogLevelFlags;
            g_log_set_always_fatal(fatal_mask);
            let ref mut fresh30 = *argv.offset(i as isize);
            *fresh30 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            *argv.offset(i as isize),
            b"--keep-going\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"-k\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_test_mode_fatal = FALSE as gboolean;
            let ref mut fresh31 = *argv.offset(i as isize);
            *fresh31 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            *argv.offset(i as isize),
            b"--debug-log\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_test_debug_log = TRUE as gboolean;
            let ref mut fresh32 = *argv.offset(i as isize);
            *fresh32 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            *argv.offset(i as isize),
            b"--tap\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_test_tap_log = TRUE as gboolean;
            let ref mut fresh33 = *argv.offset(i as isize);
            *fresh33 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            b"--GTestLogFD\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"--GTestLogFD=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                13 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal: *mut gchar =
                (*argv.offset(i as isize)).offset(12 as ::core::ffi::c_int as isize);
            if *equal as ::core::ffi::c_int == '=' as i32 {
                safe_c2rust_test_log_fd = g_ascii_strtoull(
                    equal.offset(1 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<*mut gchar>(),
                    0 as guint,
                ) as ::core::ffi::c_int;
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh34 = i;
                i = i.wrapping_add(1);
                let ref mut fresh35 = *argv.offset(fresh34 as isize);
                *fresh35 = ::core::ptr::null_mut::<gchar>();
                safe_c2rust_test_log_fd = g_ascii_strtoull(
                    *argv.offset(i as isize),
                    ::core::ptr::null_mut::<*mut gchar>(),
                    0 as guint,
                ) as ::core::ffi::c_int;
            }
            let ref mut fresh36 = *argv.offset(i as isize);
            *fresh36 = ::core::ptr::null_mut::<gchar>();
            safe_c2rust_test_tap_log = FALSE as gboolean;
        } else if strcmp(
            b"--GTestSkipCount\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"--GTestSkipCount=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                17 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal_0: *mut gchar =
                (*argv.offset(i as isize)).offset(16 as ::core::ffi::c_int as isize);
            if *equal_0 as ::core::ffi::c_int == '=' as i32 {
                safe_c2rust_test_startup_skip_count = g_ascii_strtoull(
                    equal_0.offset(1 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<*mut gchar>(),
                    0 as guint,
                ) as guint;
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh37 = i;
                i = i.wrapping_add(1);
                let ref mut fresh38 = *argv.offset(fresh37 as isize);
                *fresh38 = ::core::ptr::null_mut::<gchar>();
                safe_c2rust_test_startup_skip_count = g_ascii_strtoull(
                    *argv.offset(i as isize),
                    ::core::ptr::null_mut::<*mut gchar>(),
                    0 as guint,
                ) as guint;
            }
            let ref mut fresh39 = *argv.offset(i as isize);
            *fresh39 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            b"--GTestSubprocess\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_test_in_subprocess = TRUE as gboolean;
            safe_c2rust_g_test_disable_crash_reporting();
            let ref mut fresh40 = *argv.offset(i as isize);
            *fresh40 = ::core::ptr::null_mut::<gchar>();
            safe_c2rust_test_tap_log = FALSE as gboolean;
        } else if strcmp(
            b"-p\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"-p=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal_1: *mut gchar =
                (*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize);
            if *equal_1 as ::core::ffi::c_int == '=' as i32 {
                safe_c2rust_test_paths = g_slist_prepend(
                    safe_c2rust_test_paths,
                    equal_1.offset(1 as ::core::ffi::c_int as isize) as gpointer,
                );
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh41 = i;
                i = i.wrapping_add(1);
                let ref mut fresh42 = *argv.offset(fresh41 as isize);
                *fresh42 = ::core::ptr::null_mut::<gchar>();
                safe_c2rust_test_paths =
                    g_slist_prepend(safe_c2rust_test_paths, *argv.offset(i as isize) as gpointer);
            }
            let ref mut fresh43 = *argv.offset(i as isize);
            *fresh43 = ::core::ptr::null_mut::<gchar>();
            if safe_c2rust_test_prefix_extended != 0 {
                printf(
                    b"do not mix [-r | --run-prefix] with '-p'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            safe_c2rust_test_prefix = TRUE as gboolean;
        } else if strcmp(
            b"-r\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"-r=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                b"--run-prefix\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
            ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"--run-prefix=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                13 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal_2: *mut gchar =
                (*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize);
            if *equal_2 as ::core::ffi::c_int == '=' as i32 {
                safe_c2rust_test_paths = g_slist_prepend(
                    safe_c2rust_test_paths,
                    equal_2.offset(1 as ::core::ffi::c_int as isize) as gpointer,
                );
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh44 = i;
                i = i.wrapping_add(1);
                let ref mut fresh45 = *argv.offset(fresh44 as isize);
                *fresh45 = ::core::ptr::null_mut::<gchar>();
                safe_c2rust_test_paths =
                    g_slist_prepend(safe_c2rust_test_paths, *argv.offset(i as isize) as gpointer);
            }
            let ref mut fresh46 = *argv.offset(i as isize);
            *fresh46 = ::core::ptr::null_mut::<gchar>();
            if safe_c2rust_test_prefix != 0 {
                printf(
                    b"do not mix [-r | --run-prefix] with '-p'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            safe_c2rust_test_prefix_extended = TRUE as gboolean;
        } else if strcmp(
            b"-s\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"-s=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal_3: *mut gchar =
                (*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize);
            if *equal_3 as ::core::ffi::c_int == '=' as i32 {
                safe_c2rust_test_paths_skipped = g_slist_prepend(
                    safe_c2rust_test_paths_skipped,
                    equal_3.offset(1 as ::core::ffi::c_int as isize) as gpointer,
                );
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh47 = i;
                i = i.wrapping_add(1);
                let ref mut fresh48 = *argv.offset(fresh47 as isize);
                *fresh48 = ::core::ptr::null_mut::<gchar>();
                safe_c2rust_test_paths_skipped = g_slist_prepend(
                    safe_c2rust_test_paths_skipped,
                    *argv.offset(i as isize) as gpointer,
                );
            }
            let ref mut fresh49 = *argv.offset(i as isize);
            *fresh49 = ::core::ptr::null_mut::<gchar>();
            if safe_c2rust_test_prefix_extended_skipped != 0 {
                printf(
                    b"do not mix [-x | --skip-prefix] with '-s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            safe_c2rust_test_prefix_skipped = TRUE as gboolean;
        } else if strcmp(
            b"-x\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"-x=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                b"--skip-prefix\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
            ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"--skip-prefix=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                14 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal_4: *mut gchar =
                (*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize);
            if *equal_4 as ::core::ffi::c_int == '=' as i32 {
                safe_c2rust_test_paths_skipped = g_slist_prepend(
                    safe_c2rust_test_paths_skipped,
                    equal_4.offset(1 as ::core::ffi::c_int as isize) as gpointer,
                );
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh50 = i;
                i = i.wrapping_add(1);
                let ref mut fresh51 = *argv.offset(fresh50 as isize);
                *fresh51 = ::core::ptr::null_mut::<gchar>();
                safe_c2rust_test_paths_skipped = g_slist_prepend(
                    safe_c2rust_test_paths_skipped,
                    *argv.offset(i as isize) as gpointer,
                );
            }
            let ref mut fresh52 = *argv.offset(i as isize);
            *fresh52 = ::core::ptr::null_mut::<gchar>();
            if safe_c2rust_test_prefix_skipped != 0 {
                printf(
                    b"do not mix [-x | --skip-prefix] with '-s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            safe_c2rust_test_prefix_extended_skipped = TRUE as gboolean;
        } else if strcmp(
            b"-m\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"-m=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal_5: *mut gchar =
                (*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize);
            let mut mode: *const gchar = b"\0" as *const u8 as *const gchar;
            if *equal_5 as ::core::ffi::c_int == '=' as i32 {
                mode = equal_5.offset(1 as ::core::ffi::c_int as isize);
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh53 = i;
                i = i.wrapping_add(1);
                let ref mut fresh54 = *argv.offset(fresh53 as isize);
                *fresh54 = ::core::ptr::null_mut::<gchar>();
                mode = *argv.offset(i as isize);
            }
            if strcmp(
                mode as *const ::core::ffi::c_char,
                b"perf\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                safe_c2rust_mutable_test_config_vars.test_perf = TRUE as gboolean;
            } else if strcmp(
                mode as *const ::core::ffi::c_char,
                b"slow\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                safe_c2rust_mutable_test_config_vars.test_quick = FALSE as gboolean;
            } else if strcmp(
                mode as *const ::core::ffi::c_char,
                b"thorough\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                safe_c2rust_mutable_test_config_vars.test_quick = FALSE as gboolean;
            } else if strcmp(
                mode as *const ::core::ffi::c_char,
                b"quick\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                safe_c2rust_mutable_test_config_vars.test_quick = TRUE as gboolean;
                safe_c2rust_mutable_test_config_vars.test_perf = FALSE as gboolean;
            } else if strcmp(
                mode as *const ::core::ffi::c_char,
                b"undefined\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                safe_c2rust_mutable_test_config_vars.test_undefined = TRUE as gboolean;
            } else if strcmp(
                mode as *const ::core::ffi::c_char,
                b"no-undefined\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                safe_c2rust_mutable_test_config_vars.test_undefined = FALSE as gboolean;
            } else {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"unknown test mode: -m %s\0" as *const u8 as *const gchar,
                    mode,
                );
                loop {}
            }
            let ref mut fresh55 = *argv.offset(i as isize);
            *fresh55 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            b"-q\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                b"--quiet\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
            ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_mutable_test_config_vars.test_quiet = TRUE as gboolean;
            safe_c2rust_mutable_test_config_vars.test_verbose = FALSE as gboolean;
            let ref mut fresh56 = *argv.offset(i as isize);
            *fresh56 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            b"--verbose\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_mutable_test_config_vars.test_quiet = FALSE as gboolean;
            safe_c2rust_mutable_test_config_vars.test_verbose = TRUE as gboolean;
            let ref mut fresh57 = *argv.offset(i as isize);
            *fresh57 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            b"-l\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_test_run_list = TRUE as gboolean;
            let ref mut fresh58 = *argv.offset(i as isize);
            *fresh58 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            b"--seed\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                b"--seed=\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
                7 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut equal_6: *mut gchar =
                (*argv.offset(i as isize)).offset(6 as ::core::ffi::c_int as isize);
            if *equal_6 as ::core::ffi::c_int == '=' as i32 {
                safe_c2rust_test_run_seedstr = equal_6.offset(1 as ::core::ffi::c_int as isize);
            } else if i.wrapping_add(1 as guint) < argc {
                let fresh59 = i;
                i = i.wrapping_add(1);
                let ref mut fresh60 = *argv.offset(fresh59 as isize);
                *fresh60 = ::core::ptr::null_mut::<gchar>();
                safe_c2rust_test_run_seedstr = *argv.offset(i as isize);
            }
            let ref mut fresh61 = *argv.offset(i as isize);
            *fresh61 = ::core::ptr::null_mut::<gchar>();
        } else if strcmp(
            b"-?\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                b"-h\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                b"--help\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(i as isize),
            ) == 0 as ::core::ffi::c_int
        {
            printf(
                b"Usage:\n  %s [OPTION...]\n\nHelp Options:\n  -h, --help                     Show help options\n\nTest Options:\n  --g-fatal-warnings             Make all warnings fatal\n  -l                             List test cases available in a test executable\n  -m {perf|slow|thorough|quick}  Execute tests according to mode\n  -m {undefined|no-undefined}    Execute tests according to mode\n  -p TESTPATH                    Only start test cases matching TESTPATH\n  -s TESTPATH                    Skip all tests matching TESTPATH\n  [-r | --run-prefix] PREFIX     Only start test cases (or suites) matching PREFIX (incompatible with -p).\n                                 Unlike the -p option (which only goes one level deep), this option would \n                                 run all tests path that have PREFIX at the beginning of their name.\n                                 Note that the prefix used should be a valid test path (and not a simple prefix).\n  [-x | --skip-prefix] PREFIX    Skip all tests matching PREFIX (incompatible with -s)\n                                 Unlike the -s option (which only skips the exact TESTPATH), this option will \n                                 skip all the tests that begins with PREFIX).\n  --seed=SEEDSTRING              Start tests with random seed SEEDSTRING\n  --debug-log                    debug test logging output\n  -q, --quiet                    Run tests quietly\n  --verbose                      Run tests verbosely\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            exit(0 as ::core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }
    safe_c2rust_test_paths = g_slist_reverse(safe_c2rust_test_paths);
    e = 0 as guint;
    i = 0 as guint;
    while i < argc {
        if !(*argv.offset(i as isize)).is_null() {
            let fresh62 = e;
            e = e.wrapping_add(1);
            let ref mut fresh63 = *argv.offset(fresh62 as isize);
            *fresh63 = *argv.offset(i as isize);
            if i >= e {
                let ref mut fresh64 = *argv.offset(i as isize);
                *fresh64 = ::core::ptr::null_mut::<gchar>();
            }
        }
        i = i.wrapping_add(1);
    }
    *argc_p = e as gint;
}
unsafe extern "C" fn safe_c2rust_rm_rf(mut path: *const gchar) {
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    let mut entry: *const gchar = ::core::ptr::null::<gchar>();
    dir = g_dir_open(path, 0 as guint, ::core::ptr::null_mut::<*mut GError>());
    if dir.is_null() {
        remove(path as *const ::core::ffi::c_char);
        return;
    }
    loop {
        entry = g_dir_read_name(dir);
        if entry.is_null() {
            break;
        }
        let mut sub_path: *mut gchar = g_build_filename(path, entry, NULL_0);
        safe_c2rust_rm_rf(sub_path);
        g_free(sub_path as gpointer);
    }
    g_dir_close(dir);
    g_rmdir(path);
}
unsafe extern "C" fn safe_c2rust_test_do_isolate_dirs(mut error: *mut *mut GError) -> gboolean {
    let mut subdir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut home_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut cache_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut config_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut state_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut data_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut runtime_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut config_dirs: [*mut gchar; 3] = [::core::ptr::null_mut::<gchar>(); 3];
    let mut data_dirs: [*mut gchar; 3] = [::core::ptr::null_mut::<gchar>(); 3];
    if safe_c2rust_test_isolate_dirs == 0 {
        return TRUE;
    }
    subdir = g_build_filename(
        safe_c2rust_test_tmpdir,
        safe_c2rust_test_run_name_path,
        b".dirs\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    runtime_dir = g_build_filename(
        subdir,
        b"runtime\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    if g_mkdir_with_parents(runtime_dir, 0o700 as gint) != 0 as ::core::ffi::c_int {
        let mut saved_errno: gint = *__errno_location();
        g_set_error(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(saved_errno) as gint,
            b"Failed to create XDG_RUNTIME_DIR \xE2\x80\x98%s\xE2\x80\x99: %s\0" as *const u8
                as *const gchar,
            runtime_dir,
            g_strerror(saved_errno),
        );
        g_free(runtime_dir as gpointer);
        g_free(subdir as gpointer);
        return FALSE;
    }
    home_dir = g_build_filename(
        subdir,
        b"home\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    cache_dir = g_build_filename(
        subdir,
        b"cache\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    config_dir = g_build_filename(
        subdir,
        b"config\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    data_dir = g_build_filename(
        subdir,
        b"data\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    state_dir = g_build_filename(
        subdir,
        b"state\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    config_dirs[0 as ::core::ffi::c_int as usize] = g_build_filename(
        subdir,
        b"system-config1\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    config_dirs[1 as ::core::ffi::c_int as usize] = g_build_filename(
        subdir,
        b"system-config2\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    config_dirs[2 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<gchar>();
    data_dirs[0 as ::core::ffi::c_int as usize] = g_build_filename(
        subdir,
        b"system-data1\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    data_dirs[1 as ::core::ffi::c_int as usize] = g_build_filename(
        subdir,
        b"system-data2\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    data_dirs[2 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<gchar>();
    g_set_user_dirs(
        b"HOME\0" as *const u8 as *const gchar,
        home_dir,
        b"XDG_CACHE_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        cache_dir,
        b"XDG_CONFIG_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut config_dirs as *mut *mut gchar,
        b"XDG_CONFIG_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        config_dir,
        b"XDG_DATA_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut data_dirs as *mut *mut gchar,
        b"XDG_DATA_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        data_dir,
        b"XDG_STATE_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        state_dir,
        b"XDG_RUNTIME_DIR\0" as *const u8 as *const ::core::ffi::c_char,
        runtime_dir,
        NULL_0,
    );
    g_free(runtime_dir as gpointer);
    g_free(state_dir as gpointer);
    g_free(data_dir as gpointer);
    g_free(config_dir as gpointer);
    g_free(cache_dir as gpointer);
    g_free(home_dir as gpointer);
    g_free(data_dirs[1 as ::core::ffi::c_int as usize] as gpointer);
    g_free(data_dirs[0 as ::core::ffi::c_int as usize] as gpointer);
    g_free(config_dirs[1 as ::core::ffi::c_int as usize] as gpointer);
    g_free(config_dirs[0 as ::core::ffi::c_int as usize] as gpointer);
    g_free(subdir as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_test_rm_isolate_dirs() {
    let mut subdir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if safe_c2rust_test_isolate_dirs == 0 {
        return;
    }
    subdir = g_build_filename(
        safe_c2rust_test_tmpdir,
        safe_c2rust_test_run_name_path,
        NULL_0,
    );
    safe_c2rust_rm_rf(subdir);
    g_free(subdir as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_init(
    mut argc: *mut ::core::ffi::c_int,
    mut argv: *mut *mut *mut ::core::ffi::c_char,
    mut args: ...
) {
    static mut safe_c2rust_seedstr: [::core::ffi::c_char; 37] = [0; 37];
    let mut args_0: ::core::ffi::VaList;
    let mut option: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut fatal_mask: GLogLevelFlags = g_log_set_always_fatal(
        (G_LOG_FLAG_RECURSION as ::core::ffi::c_int | G_LOG_LEVEL_ERROR as ::core::ffi::c_int)
            as GLogLevelFlags,
    );
    fatal_mask = (fatal_mask as ::core::ffi::c_int
        | G_LOG_LEVEL_WARNING as ::core::ffi::c_int
        | G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int) as GLogLevelFlags;
    g_log_set_always_fatal(fatal_mask);
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !argc.is_null() {
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
            b"argc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !argv.is_null() {
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
            b"argv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if (*safe_c2rust_g_test_config_vars).test_initialized == 0 as ::core::ffi::c_int {
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
            b"g_test_config_vars->test_initialized == FALSE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_mutable_test_config_vars.test_initialized = TRUE as gboolean;
    args_0 = args.clone();
    loop {
        option = args_0.arg::<*mut ::core::ffi::c_char>() as gpointer;
        if option.is_null() {
            break;
        }
        if safe_c2rust_g_strcmp0(
            option as *const ::core::ffi::c_char,
            b"no_g_set_prgname\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_no_g_set_prgname = TRUE as gboolean;
        } else if safe_c2rust_g_strcmp0(
            option as *const ::core::ffi::c_char,
            G_TEST_OPTION_ISOLATE_DIRS.as_ptr(),
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_test_isolate_dirs = TRUE as gboolean;
        }
    }
    safe_c2rust_parse_args(argc as *mut gint, argv as *mut *mut *mut gchar);
    if safe_c2rust_test_run_seedstr.is_null() {
        g_snprintf(
            &raw mut safe_c2rust_seedstr as *mut gchar,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>() as gulong,
            b"R02S%08x%08x%08x%08x\0" as *const u8 as *const gchar,
            g_random_int(),
            g_random_int(),
            g_random_int(),
            g_random_int(),
        );
        safe_c2rust_test_run_seedstr =
            &raw mut safe_c2rust_seedstr as *mut ::core::ffi::c_char as *mut gchar;
    }
    if g_get_prgname().is_null() && safe_c2rust_no_g_set_prgname == 0 {
        g_set_prgname_once(*(*argv).offset(0 as ::core::ffi::c_int as isize));
    }
    if !g_getenv(b"G_TEST_ROOT_PROCESS\0" as *const u8 as *const gchar).is_null() {
        safe_c2rust_test_is_subtest = TRUE as gboolean;
    } else if g_setenv(
        b"G_TEST_ROOT_PROCESS\0" as *const u8 as *const gchar,
        if !safe_c2rust_test_argv0.is_null() {
            safe_c2rust_test_argv0 as *const gchar
        } else {
            b"root\0" as *const u8 as *const gchar
        },
        TRUE,
    ) == 0
    {
        g_printerr(
            b"%s: Failed to set environment variable \xE2\x80\x98%s\xE2\x80\x99\n\0" as *const u8
                as *const gchar,
            safe_c2rust_test_argv0,
            b"G_TEST_ROOT_PROCESS\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if safe_c2rust_test_isolate_dirs != 0 {
        if g_getenv(b"G_TEST_TMPDIR\0" as *const u8 as *const gchar).is_null() {
            let mut test_prgname: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut tmpl: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
            test_prgname = g_path_get_basename(g_get_prgname());
            if *test_prgname as ::core::ffi::c_int == '\0' as i32 {
                g_free(test_prgname as gpointer);
                test_prgname = safe_c2rust_g_strdup_inline(
                    b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut gchar;
            }
            tmpl = g_strdup_printf(
                b"test_%s_XXXXXX\0" as *const u8 as *const gchar,
                test_prgname,
            );
            g_free(test_prgname as gpointer);
            safe_c2rust_test_isolate_dirs_tmpdir = g_dir_make_tmp(tmpl, &raw mut local_error);
            if !local_error.is_null() {
                g_printerr(
                    b"%s: Failed to create temporary directory: %s\n\0" as *const u8
                        as *const gchar,
                    *(*argv).offset(0 as ::core::ffi::c_int as isize),
                    (*local_error).message,
                );
                g_error_free(local_error);
                exit(1 as ::core::ffi::c_int);
            }
            g_free(tmpl as gpointer);
            if g_setenv(
                b"G_TEST_TMPDIR\0" as *const u8 as *const gchar,
                safe_c2rust_test_isolate_dirs_tmpdir,
                TRUE,
            ) == 0
            {
                g_printerr(
                    b"%s: Failed to set environment variable \xE2\x80\x98%s\xE2\x80\x99\n\0"
                        as *const u8 as *const gchar,
                    *(*argv).offset(0 as ::core::ffi::c_int as isize),
                    b"G_TEST_TMPDIR\0" as *const u8 as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            _g_unset_cached_tmp_dir();
            let mut overridden_environment_variables: [*const gchar; 7] = [
                b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
                b"XDG_CACHE_HOME\0" as *const u8 as *const ::core::ffi::c_char,
                b"XDG_CONFIG_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
                b"XDG_CONFIG_HOME\0" as *const u8 as *const ::core::ffi::c_char,
                b"XDG_DATA_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
                b"XDG_DATA_HOME\0" as *const u8 as *const ::core::ffi::c_char,
                b"XDG_RUNTIME_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            ];
            let mut i: gsize = 0;
            i = 0 as gsize;
            while (i as usize)
                < (::core::mem::size_of::<[*const gchar; 7]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*const gchar>() as usize)
            {
                if g_setenv(
                    overridden_environment_variables[i as usize],
                    b"/dev/null\0" as *const u8 as *const gchar,
                    TRUE,
                ) == 0
                {
                    g_printerr(
                        b"%s: Failed to set environment variable \xE2\x80\x98%s\xE2\x80\x99\n\0"
                            as *const u8 as *const gchar,
                        *(*argv).offset(0 as ::core::ffi::c_int as isize),
                        overridden_environment_variables[i as usize],
                    );
                    exit(1 as ::core::ffi::c_int);
                }
                i = i.wrapping_add(1);
            }
        }
        safe_c2rust_test_tmpdir = g_getenv(b"G_TEST_TMPDIR\0" as *const u8 as *const gchar);
    }
    let mut rg: *mut GRand = g_rand_new_with_seed(0xc8c49fb6 as guint32);
    let mut t1: guint32 = g_rand_int(rg);
    let mut t2: guint32 = g_rand_int(rg);
    let mut t3: guint32 = g_rand_int(rg);
    let mut t4: guint32 = g_rand_int(rg);
    if t1 != 0xfab39f9b as ::core::ffi::c_uint
        || t2 != 0xb948fb0e as ::core::ffi::c_uint
        || t3 != 0x3d31be26 as ::core::ffi::c_int as guint32
        || t4 != 0x43a19d66 as ::core::ffi::c_int as guint32
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"random numbers are not GRand-2.2 compatible, seeds may be broken (check $G_RANDOM_VERSION)\0"
                as *const u8 as *const gchar,
        );
    }
    g_rand_free(rg);
    safe_c2rust_test_run_seed(safe_c2rust_test_run_seedstr);
    g_log_set_default_handler(
        Some(
            safe_c2rust_gtest_default_log_handler
                as unsafe extern "C" fn(*const gchar, GLogLevelFlags, *const gchar, gpointer) -> (),
        ),
        NULL_0,
    );
    safe_c2rust_g_test_log(
        G_TEST_LOG_START_BINARY,
        g_get_prgname(),
        safe_c2rust_test_run_seedstr,
        0 as guint,
        ::core::ptr::null_mut::<::f128::f128>(),
    );
    safe_c2rust_test_argv0_dirname = if !safe_c2rust_test_argv0.is_null() {
        g_path_get_dirname(safe_c2rust_test_argv0 as *const gchar) as *mut ::core::ffi::c_char
    } else {
        safe_c2rust_g_strdup_inline(b".\0" as *const u8 as *const ::core::ffi::c_char)
    };
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = safe_c2rust_test_argv0_dirname;
            let __suffix: *const ::core::ffi::c_char =
                b"/.libs\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                if __str.is_null() || __suffix.is_null() {
                    _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_23
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
            safe_c2rust_test_argv0_dirname,
            b"/.libs\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        tmp = g_path_get_dirname(safe_c2rust_test_argv0_dirname);
        g_free(safe_c2rust_test_argv0_dirname as gpointer);
        safe_c2rust_test_argv0_dirname = tmp as *mut ::core::ffi::c_char;
    }
    safe_c2rust_test_disted_files_dir =
        g_getenv(b"G_TEST_SRCDIR\0" as *const u8 as *const gchar) as *const ::core::ffi::c_char;
    if safe_c2rust_test_disted_files_dir.is_null() {
        safe_c2rust_test_disted_files_dir = safe_c2rust_test_argv0_dirname;
    }
    safe_c2rust_test_built_files_dir =
        g_getenv(b"G_TEST_BUILDDIR\0" as *const u8 as *const gchar) as *const ::core::ffi::c_char;
    if safe_c2rust_test_built_files_dir.is_null() {
        safe_c2rust_test_built_files_dir = safe_c2rust_test_argv0_dirname;
    }
}
unsafe extern "C" fn safe_c2rust_test_cleanup() {
    let mut _pp: *mut *mut GRand = &raw mut safe_c2rust_test_run_rand;
    let mut _ptr: *mut GRand = *_pp;
    *_pp = ::core::ptr::null_mut::<GRand>();
    if !_ptr.is_null() {
        g_rand_free(_ptr as *mut GRand);
    }
    let mut _pp_0: *mut *mut ::core::ffi::c_char = &raw mut safe_c2rust_test_argv0_dirname;
    let mut _ptr_0: *mut ::core::ffi::c_char = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut ::core::ffi::c_char = &raw mut safe_c2rust_test_initial_cwd;
    let mut _ptr_1: *mut ::core::ffi::c_char = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr_1.is_null() {
        g_free(_ptr_1 as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_test_run_seed(mut rseed: *const gchar) {
    let mut seed_failed: guint = 0 as guint;
    if !safe_c2rust_test_run_rand.is_null() {
        g_rand_free(safe_c2rust_test_run_rand);
    }
    safe_c2rust_test_run_rand = ::core::ptr::null_mut::<GRand>();
    while !strchr(
        b" \t\x0B\r\n\x0C\0" as *const u8 as *const ::core::ffi::c_char,
        *rseed as ::core::ffi::c_int,
    )
    .is_null()
    {
        rseed = rseed.offset(1);
    }
    if strncmp(
        rseed as *const ::core::ffi::c_char,
        b"R02S\0" as *const u8 as *const ::core::ffi::c_char,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        let mut s: *const ::core::ffi::c_char = rseed.offset(4 as ::core::ffi::c_int as isize);
        if strlen(s) >= 32 as size_t {
            let mut seedarray: [guint32; 4] = [0; 4];
            let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut hexbuf: [gchar; 9] = [0 as ::core::ffi::c_int as gchar, 0, 0, 0, 0, 0, 0, 0, 0];
            memcpy(
                &raw mut hexbuf as *mut gchar as *mut ::core::ffi::c_void,
                s.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            seedarray[0 as ::core::ffi::c_int as usize] =
                g_ascii_strtoull(&raw mut hexbuf as *mut gchar, &raw mut p, 16 as guint) as guint32;
            seed_failed = seed_failed.wrapping_add(
                (!p.is_null() && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as guint,
            );
            memcpy(
                &raw mut hexbuf as *mut gchar as *mut ::core::ffi::c_void,
                s.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            seedarray[1 as ::core::ffi::c_int as usize] =
                g_ascii_strtoull(&raw mut hexbuf as *mut gchar, &raw mut p, 16 as guint) as guint32;
            seed_failed = seed_failed.wrapping_add(
                (!p.is_null() && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as guint,
            );
            memcpy(
                &raw mut hexbuf as *mut gchar as *mut ::core::ffi::c_void,
                s.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            seedarray[2 as ::core::ffi::c_int as usize] =
                g_ascii_strtoull(&raw mut hexbuf as *mut gchar, &raw mut p, 16 as guint) as guint32;
            seed_failed = seed_failed.wrapping_add(
                (!p.is_null() && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as guint,
            );
            memcpy(
                &raw mut hexbuf as *mut gchar as *mut ::core::ffi::c_void,
                s.offset(24 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                8 as size_t,
            );
            seedarray[3 as ::core::ffi::c_int as usize] =
                g_ascii_strtoull(&raw mut hexbuf as *mut gchar, &raw mut p, 16 as guint) as guint32;
            seed_failed = seed_failed.wrapping_add(
                (!p.is_null() && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as guint,
            );
            if seed_failed == 0 {
                safe_c2rust_test_run_rand =
                    g_rand_new_with_seed_array(&raw mut seedarray as *mut guint32, 4 as guint);
                return;
            }
        }
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_ERROR,
        b"Unknown or invalid random seed: %s\0" as *const u8 as *const gchar,
        rseed,
    );
    loop {}
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_rand_int() -> gint32 {
    let mut r: gint32 = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__test_run_rand_lock);
    r = g_rand_int(safe_c2rust_test_run_rand) as gint32;
    g_mutex_unlock(&raw mut safe_c2rust_g__test_run_rand_lock);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_rand_int_range(
    mut begin: gint32,
    mut end: gint32,
) -> gint32 {
    let mut r: gint32 = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__test_run_rand_lock);
    r = g_rand_int_range(safe_c2rust_test_run_rand, begin, end);
    g_mutex_unlock(&raw mut safe_c2rust_g__test_run_rand_lock);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_rand_double() -> ::core::ffi::c_double {
    let mut r: ::core::ffi::c_double = 0.;
    g_mutex_lock(&raw mut safe_c2rust_g__test_run_rand_lock);
    r = g_rand_double(safe_c2rust_test_run_rand) as ::core::ffi::c_double;
    g_mutex_unlock(&raw mut safe_c2rust_g__test_run_rand_lock);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_rand_double_range(
    mut range_start: ::core::ffi::c_double,
    mut range_end: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut r: ::core::ffi::c_double = 0.;
    g_mutex_lock(&raw mut safe_c2rust_g__test_run_rand_lock);
    r = g_rand_double_range(
        safe_c2rust_test_run_rand,
        range_start as gdouble,
        range_end as gdouble,
    ) as ::core::ffi::c_double;
    g_mutex_unlock(&raw mut safe_c2rust_g__test_run_rand_lock);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_timer_start() {
    if safe_c2rust_test_user_timer.is_null() {
        safe_c2rust_test_user_timer = g_timer_new();
    }
    safe_c2rust_test_user_stamp = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    g_timer_start(safe_c2rust_test_user_timer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_timer_elapsed() -> ::core::ffi::c_double {
    safe_c2rust_test_user_stamp = (if !safe_c2rust_test_user_timer.is_null() {
        g_timer_elapsed(
            safe_c2rust_test_user_timer,
            ::core::ptr::null_mut::<gulong>(),
        )
    } else {
        0 as ::core::ffi::c_int as gdouble
    }) as ::core::ffi::c_double;
    return safe_c2rust_test_user_stamp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_timer_last() -> ::core::ffi::c_double {
    return safe_c2rust_test_user_stamp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_minimized_result(
    mut minimized_quantity: ::core::ffi::c_double,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut largs: ::f128::f128 = ::f128::f128::new(minimized_quantity);
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    buffer = g_strdup_vprintf(format as *const gchar, args_0.clone());
    safe_c2rust_g_test_log(
        G_TEST_LOG_MIN_RESULT,
        buffer,
        ::core::ptr::null::<gchar>(),
        1 as guint,
        &raw mut largs,
    );
    g_free(buffer as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_maximized_result(
    mut maximized_quantity: ::core::ffi::c_double,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut largs: ::f128::f128 = ::f128::f128::new(maximized_quantity);
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    buffer = g_strdup_vprintf(format as *const gchar, args_0.clone());
    safe_c2rust_g_test_log(
        G_TEST_LOG_MAX_RESULT,
        buffer,
        ::core::ptr::null::<gchar>(),
        1 as guint,
        &raw mut largs,
    );
    g_free(buffer as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_message(
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    buffer = g_strdup_vprintf(format as *const gchar, args_0.clone());
    safe_c2rust_g_test_log(
        G_TEST_LOG_MESSAGE,
        buffer,
        ::core::ptr::null::<gchar>(),
        0 as guint,
        ::core::ptr::null_mut::<::f128::f128>(),
    );
    g_free(buffer as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_bug_base(mut uri_pattern: *const ::core::ffi::c_char) {
    g_free(safe_c2rust_test_uri_base as gpointer);
    safe_c2rust_test_uri_base = safe_c2rust_g_strdup_inline(uri_pattern);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_bug(mut bug_uri_snippet: *const ::core::ffi::c_char) {
    let mut c: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !bug_uri_snippet.is_null() {
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
            b"bug_uri_snippet != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = bug_uri_snippet;
            let __prefix: *const ::core::ffi::c_char =
                b"http:\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_25
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
            bug_uri_snippet as *const gchar,
            b"http:\0" as *const u8 as *const gchar,
        )
    }) != 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = bug_uri_snippet;
                let __prefix: *const ::core::ffi::c_char =
                    b"https:\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_26 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_26 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_26
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
                bug_uri_snippet as *const gchar,
                b"https:\0" as *const u8 as *const gchar,
            )
        }) != 0
    {
        safe_c2rust_g_test_message(
            b"Bug Reference: %s\0" as *const u8 as *const ::core::ffi::c_char,
            bug_uri_snippet,
        );
        return;
    }
    if !safe_c2rust_test_uri_base.is_null() {
        c = strstr(
            safe_c2rust_test_uri_base,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !c.is_null() {
        let mut b: *mut ::core::ffi::c_char = g_strndup(
            safe_c2rust_test_uri_base,
            c.offset_from(safe_c2rust_test_uri_base) as ::core::ffi::c_long as gsize,
        ) as *mut ::core::ffi::c_char;
        let mut s: *mut ::core::ffi::c_char = g_strconcat(
            b,
            bug_uri_snippet,
            c.offset(2 as ::core::ffi::c_int as isize),
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        g_free(b as gpointer);
        safe_c2rust_g_test_message(
            b"Bug Reference: %s\0" as *const u8 as *const ::core::ffi::c_char,
            s,
        );
        g_free(s as gpointer);
    } else {
        safe_c2rust_g_test_message(
            b"Bug Reference: %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            if !safe_c2rust_test_uri_base.is_null() {
                safe_c2rust_test_uri_base as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            bug_uri_snippet,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_summary(mut summary: *const ::core::ffi::c_char) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !summary.is_null() {
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
            b"summary != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if strchr(summary, '\n' as i32).is_null() {
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
            b"strchr (summary, '\\n') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if strchr(summary, '\r' as i32).is_null() {
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
            b"strchr (summary, '\\r') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_test_message(
        b"%s summary: %s\0" as *const u8 as *const ::core::ffi::c_char,
        safe_c2rust_test_run_name,
        summary,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_get_root() -> *mut GTestSuite {
    if safe_c2rust_test_suite_root.is_null() {
        safe_c2rust_test_suite_root =
            safe_c2rust_g_test_create_suite(b"root\0" as *const u8 as *const ::core::ffi::c_char);
        g_free((*safe_c2rust_test_suite_root).name as gpointer);
        (*safe_c2rust_test_suite_root).name =
            safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
                as *mut gchar;
    }
    return safe_c2rust_test_suite_root;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_run() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut suite: *mut GTestSuite = ::core::ptr::null_mut::<GTestSuite>();
    if atexit(Some(
        safe_c2rust_test_cleanup as unsafe extern "C" fn() -> (),
    )) != 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Unable to register test cleanup to be run at exit: %s\0" as *const u8 as *const gchar,
            g_strerror(errsv as gint),
        );
        loop {}
    }
    suite = safe_c2rust_g_test_get_root();
    if safe_c2rust_g_test_run_suite(suite) != 0 as ::core::ffi::c_int {
        ret = 1 as ::core::ffi::c_int;
    } else {
        if !safe_c2rust_test_isolate_dirs_tmpdir.is_null() {
            safe_c2rust_rm_rf(safe_c2rust_test_isolate_dirs_tmpdir);
            g_free(safe_c2rust_test_isolate_dirs_tmpdir as gpointer);
            safe_c2rust_test_isolate_dirs_tmpdir = ::core::ptr::null_mut::<gchar>();
        }
        if safe_c2rust_test_tap_log != 0 {
            ret = 0 as ::core::ffi::c_int;
        } else if safe_c2rust_test_run_count > 0 as guint
            && safe_c2rust_test_run_count == safe_c2rust_test_skipped_count
        {
            ret = 77 as ::core::ffi::c_int;
        } else {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    safe_c2rust_g_test_suite_free(suite);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_create_case(
    mut test_name: *const ::core::ffi::c_char,
    mut data_size: gsize,
    mut test_data: gconstpointer,
    mut data_setup: GTestFixtureFunc,
    mut data_test: GTestFixtureFunc,
    mut data_teardown: GTestFixtureFunc,
) -> *mut GTestCase {
    let mut tc: *mut GTestCase = ::core::ptr::null_mut::<GTestCase>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !test_name.is_null() {
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
            b"test_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestCase>();
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if strchr(test_name, '/' as i32).is_null() {
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
            b"strchr (test_name, '/') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestCase>();
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if *test_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
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
            b"test_name[0] != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestCase>();
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if data_test.is_some() {
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
            b"data_test != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestCase>();
    }
    tc = ({
        let mut __s: gsize = ::core::mem::size_of::<GTestCase>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GTestCase;
    (*tc).name = safe_c2rust_g_strdup_inline(test_name) as *mut gchar;
    (*tc).test_data = test_data as gpointer;
    (*tc).fixture_size = data_size as guint;
    (*tc).fixture_setup = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, gconstpointer) -> ()>,
    >(::core::mem::transmute::<
        GTestFixtureFunc,
        *mut ::core::ffi::c_void,
    >(data_setup));
    (*tc).fixture_test = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, gconstpointer) -> ()>,
    >(::core::mem::transmute::<
        GTestFixtureFunc,
        *mut ::core::ffi::c_void,
    >(data_test));
    (*tc).fixture_teardown = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, gconstpointer) -> ()>,
    >(::core::mem::transmute::<
        GTestFixtureFunc,
        *mut ::core::ffi::c_void,
    >(data_teardown));
    return tc;
}
unsafe extern "C" fn safe_c2rust_find_suite(mut l: gconstpointer, mut s: gconstpointer) -> gint {
    let mut suite: *const GTestSuite = l as *const GTestSuite;
    let mut str: *const gchar = s as *const gchar;
    return strcmp((*suite).name, str as *const ::core::ffi::c_char) as gint;
}
unsafe extern "C" fn safe_c2rust_find_case(mut l: gconstpointer, mut s: gconstpointer) -> gint {
    let mut tc: *const GTestCase = l as *const GTestCase;
    let mut str: *const gchar = s as *const gchar;
    return strcmp((*tc).name, str as *const ::core::ffi::c_char) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_add_vtable(
    mut testpath: *const ::core::ffi::c_char,
    mut data_size: gsize,
    mut test_data: gconstpointer,
    mut data_setup: GTestFixtureFunc,
    mut fixture_test_func: GTestFixtureFunc,
    mut data_teardown: GTestFixtureFunc,
) {
    let mut segments: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut ui: guint = 0;
    let mut suite: *mut GTestSuite = ::core::ptr::null_mut::<GTestSuite>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !testpath.is_null() {
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
            b"testpath != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if g_path_is_absolute(testpath as *const gchar) != 0 {
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
            b"g_path_is_absolute (testpath)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if fixture_test_func.is_some() {
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
            b"fixture_test_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if safe_c2rust_test_isolate_dirs == 0
            || strstr(testpath, b"/.\0" as *const u8 as *const ::core::ffi::c_char).is_null()
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
            b"!test_isolate_dirs || strstr (testpath, \"/.\") == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    suite = safe_c2rust_g_test_get_root();
    segments = g_strsplit(
        testpath as *const gchar,
        b"/\0" as *const u8 as *const gchar,
        -(1 as gint),
    );
    ui = 0 as guint;
    while !(*segments.offset(ui as isize)).is_null() {
        let mut seg: *const ::core::ffi::c_char = *segments.offset(ui as isize);
        let mut islast: gboolean = (*segments.offset(ui.wrapping_add(1 as guint) as isize)
            == NULL_0 as *mut gchar) as ::core::ffi::c_int;
        if islast != 0 && *seg.offset(0 as ::core::ffi::c_int as isize) == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"invalid test case path: %s\0" as *const u8 as *const gchar,
                testpath,
            );
            loop {}
        } else {
            if !(*seg.offset(0 as ::core::ffi::c_int as isize) == 0) {
                if islast == 0 {
                    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
                    let mut csuite: *mut GTestSuite = ::core::ptr::null_mut::<GTestSuite>();
                    l = g_slist_find_custom(
                        (*suite).suites,
                        seg as gconstpointer,
                        Some(
                            safe_c2rust_find_suite
                                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
                        ),
                    );
                    if !l.is_null() {
                        csuite = (*l).data as *mut GTestSuite;
                    } else {
                        csuite = safe_c2rust_g_test_create_suite(seg);
                        safe_c2rust_g_test_suite_add_suite(suite, csuite);
                    }
                    suite = csuite;
                } else {
                    let mut tc: *mut GTestCase = ::core::ptr::null_mut::<GTestCase>();
                    if !g_slist_find_custom(
                        (*suite).cases,
                        seg as gconstpointer,
                        Some(
                            safe_c2rust_find_case
                                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
                        ),
                    )
                    .is_null()
                    {
                        g_log(
                            G_LOG_DOMAIN.as_ptr() as *const gchar,
                            G_LOG_LEVEL_ERROR,
                            b"duplicate test case path: %s\0" as *const u8 as *const gchar,
                            testpath,
                        );
                        loop {}
                    }
                    tc = safe_c2rust_g_test_create_case(
                        seg,
                        data_size,
                        test_data,
                        data_setup,
                        fixture_test_func,
                        data_teardown,
                    );
                    safe_c2rust_g_test_suite_add(suite, tc);
                }
            }
            ui = ui.wrapping_add(1);
        }
    }
    g_strfreev(segments);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_fail() {
    safe_c2rust_test_run_success = G_TEST_RUN_FAILURE;
    let mut _pp: *mut *mut gchar = &raw mut safe_c2rust_test_run_msg;
    let mut _ptr: *mut gchar = *_pp;
    *_pp = ::core::ptr::null_mut::<gchar>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_fail_printf(
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    safe_c2rust_test_run_success = G_TEST_RUN_FAILURE;
    args_0 = args.clone();
    g_free(safe_c2rust_test_run_msg as gpointer);
    safe_c2rust_test_run_msg = g_strdup_vprintf(format as *const gchar, args_0.clone());
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_incomplete(mut msg: *const gchar) {
    safe_c2rust_test_run_success = G_TEST_RUN_INCOMPLETE;
    g_free(safe_c2rust_test_run_msg as gpointer);
    safe_c2rust_test_run_msg =
        safe_c2rust_g_strdup_inline(msg as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_incomplete_printf(
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    safe_c2rust_test_run_success = G_TEST_RUN_INCOMPLETE;
    args_0 = args.clone();
    g_free(safe_c2rust_test_run_msg as gpointer);
    safe_c2rust_test_run_msg = g_strdup_vprintf(format as *const gchar, args_0.clone());
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_skip(mut msg: *const gchar) {
    safe_c2rust_test_run_success = G_TEST_RUN_SKIPPED;
    g_free(safe_c2rust_test_run_msg as gpointer);
    safe_c2rust_test_run_msg =
        safe_c2rust_g_strdup_inline(msg as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_skip_printf(
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    safe_c2rust_test_run_success = G_TEST_RUN_SKIPPED;
    args_0 = args.clone();
    g_free(safe_c2rust_test_run_msg as gpointer);
    safe_c2rust_test_run_msg = g_strdup_vprintf(format as *const gchar, args_0.clone());
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_failed() -> gboolean {
    return (safe_c2rust_test_run_success as ::core::ffi::c_uint
        != G_TEST_RUN_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_set_nonfatal_assertions() {
    if (*safe_c2rust_g_test_config_vars).test_initialized == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_test_set_nonfatal_assertions called without g_test_init\0" as *const u8
                as *const gchar,
        );
        loop {}
    }
    safe_c2rust_test_nonfatal_assertions = TRUE as gboolean;
    safe_c2rust_test_mode_fatal = FALSE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_add_func(
    mut testpath: *const ::core::ffi::c_char,
    mut test_func: GTestFunc,
) {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !testpath.is_null() {
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
            b"testpath != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if *testpath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
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
            b"testpath[0] == '/'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if test_func.is_some() {
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
            b"test_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_test_add_vtable(
        testpath,
        0 as gsize,
        ::core::ptr::null::<::core::ffi::c_void>(),
        None,
        ::core::mem::transmute::<GTestFunc, GTestFixtureFunc>(test_func),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_add_data_func(
    mut testpath: *const ::core::ffi::c_char,
    mut test_data: gconstpointer,
    mut test_func: GTestDataFunc,
) {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !testpath.is_null() {
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
            b"testpath != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if *testpath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
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
            b"testpath[0] == '/'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if test_func.is_some() {
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
            b"test_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_test_add_vtable(
        testpath,
        0 as gsize,
        test_data,
        None,
        ::core::mem::transmute::<GTestDataFunc, GTestFixtureFunc>(test_func),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_add_data_func_full(
    mut testpath: *const ::core::ffi::c_char,
    mut test_data: gpointer,
    mut test_func: GTestDataFunc,
    mut data_free_func: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !testpath.is_null() {
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
            b"testpath != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if *testpath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
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
            b"testpath[0] == '/'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if test_func.is_some() {
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
            b"test_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_test_add_vtable(
        testpath,
        0 as gsize,
        test_data as gconstpointer,
        None,
        ::core::mem::transmute::<GTestDataFunc, GTestFixtureFunc>(test_func),
        ::core::mem::transmute::<GDestroyNotify, GTestFixtureFunc>(data_free_func),
    );
}
unsafe extern "C" fn safe_c2rust_g_test_suite_case_exists(
    mut suite: *mut GTestSuite,
    mut test_path: *const ::core::ffi::c_char,
) -> gboolean {
    let mut iter: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut slash: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tc: *mut GTestCase = ::core::ptr::null_mut::<GTestCase>();
    test_path = test_path.offset(1);
    slash = strchr(test_path, '/' as i32);
    if !slash.is_null() {
        iter = (*suite).suites;
        while !iter.is_null() {
            let mut child_suite: *mut GTestSuite = (*iter).data as *mut GTestSuite;
            if strncmp(
                (*child_suite).name,
                test_path,
                slash.offset_from(test_path) as ::core::ffi::c_long as size_t,
            ) == 0
            {
                if safe_c2rust_g_test_suite_case_exists(child_suite, slash) != 0 {
                    return TRUE;
                }
            }
            iter = (*iter).next;
        }
    } else {
        iter = (*suite).cases;
        while !iter.is_null() {
            tc = (*iter).data as *mut GTestCase;
            if strcmp((*tc).name, test_path) == 0 {
                return TRUE;
            }
            iter = (*iter).next;
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_create_suite(
    mut suite_name: *const ::core::ffi::c_char,
) -> *mut GTestSuite {
    let mut ts: *mut GTestSuite = ::core::ptr::null_mut::<GTestSuite>();
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !suite_name.is_null() {
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
            b"suite_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestSuite>();
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if strchr(suite_name, '/' as i32).is_null() {
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
            b"strchr (suite_name, '/') == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestSuite>();
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if *suite_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
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
            b"suite_name[0] != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestSuite>();
    }
    ts = ({
        let mut __s: gsize = ::core::mem::size_of::<GTestSuite>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GTestSuite;
    (*ts).name = safe_c2rust_g_strdup_inline(suite_name) as *mut gchar;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_suite_add(
    mut suite: *mut GTestSuite,
    mut test_case: *mut GTestCase,
) {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !suite.is_null() {
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
            b"suite != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !test_case.is_null() {
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
            b"test_case != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*suite).cases = g_slist_append((*suite).cases, test_case as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_suite_add_suite(
    mut suite: *mut GTestSuite,
    mut nestedsuite: *mut GTestSuite,
) {
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !suite.is_null() {
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
            b"suite != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !nestedsuite.is_null() {
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
            b"nestedsuite != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*suite).suites = g_slist_append((*suite).suites, nestedsuite as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_queue_free(mut gfree_pointer: gpointer) {
    if !gfree_pointer.is_null() {
        safe_c2rust_g_test_queue_destroy(
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            gfree_pointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_queue_destroy(
    mut destroy_func: GDestroyNotify,
    mut destroy_data: gpointer,
) {
    let mut dentry: *mut DestroyEntry = ::core::ptr::null_mut::<DestroyEntry>();
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if destroy_func.is_some() {
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
            b"destroy_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    dentry = ({
        let mut __s: gsize = ::core::mem::size_of::<DestroyEntry>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut DestroyEntry;
    (*dentry).destroy_func = destroy_func;
    (*dentry).destroy_data = destroy_data;
    (*dentry).next = safe_c2rust_test_destroy_queue;
    safe_c2rust_test_destroy_queue = dentry;
}
unsafe extern "C" fn safe_c2rust_test_has_prefix(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gint {
    let mut test_path_skipped_local: *const gchar = a as *const gchar;
    let mut test_run_name_local: *const gchar = b as *const gchar;
    if safe_c2rust_test_prefix_extended_skipped != 0 {
        if test_path_skipped_local.is_null() || test_run_name_local.is_null() {
            return FALSE;
        }
        return strncmp(
            test_run_name_local as *const ::core::ffi::c_char,
            test_path_skipped_local as *const ::core::ffi::c_char,
            strlen(test_path_skipped_local as *const ::core::ffi::c_char),
        ) as gint;
    }
    return safe_c2rust_g_strcmp0(
        test_run_name_local as *const ::core::ffi::c_char,
        test_path_skipped_local as *const ::core::ffi::c_char,
    ) as gint;
}
unsafe extern "C" fn safe_c2rust_test_case_run(
    mut tc: *mut GTestCase,
    mut test_run_name: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
) -> gboolean {
    let mut old_base: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut old_free_list: *mut *mut GSList = ::core::ptr::null_mut::<*mut GSList>();
    let mut filename_free_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut success: gboolean = G_TEST_RUN_SUCCESS as ::core::ffi::c_int as gboolean;
    let mut free_test_data: gboolean = TRUE;
    old_base = safe_c2rust_g_strdup_inline(safe_c2rust_test_uri_base) as *mut gchar;
    old_free_list = safe_c2rust_test_filename_free_list;
    safe_c2rust_test_filename_free_list = &raw mut filename_free_list;
    if safe_c2rust_test_should_run(test_run_name, path) == 0 {
        success = G_TEST_RUN_SKIPPED as ::core::ffi::c_int as gboolean;
    } else {
        safe_c2rust_test_run_count = safe_c2rust_test_run_count.wrapping_add(1);
        if safe_c2rust_test_run_count <= safe_c2rust_test_startup_skip_count {
            safe_c2rust_g_test_log(
                G_TEST_LOG_SKIP_CASE,
                test_run_name as *const gchar,
                ::core::ptr::null::<gchar>(),
                0 as guint,
                ::core::ptr::null_mut::<::f128::f128>(),
            );
        } else if safe_c2rust_test_run_list != 0 {
            g_print(b"%s\n\0" as *const u8 as *const gchar, test_run_name);
            safe_c2rust_g_test_log(
                G_TEST_LOG_LIST_CASE,
                test_run_name as *const gchar,
                ::core::ptr::null::<gchar>(),
                0 as guint,
                ::core::ptr::null_mut::<::f128::f128>(),
            );
        } else {
            let mut test_run_timer: *mut GTimer = g_timer_new();
            let mut largs: [::f128::f128; 3] = [::f128::f128::ZERO; 3];
            let mut fixture: *mut ::core::ffi::c_void =
                ::core::ptr::null_mut::<::core::ffi::c_void>();
            safe_c2rust_g_test_log(
                G_TEST_LOG_START_CASE,
                test_run_name as *const gchar,
                ::core::ptr::null::<gchar>(),
                0 as guint,
                ::core::ptr::null_mut::<::f128::f128>(),
            );
            safe_c2rust_test_run_forks = 0 as guint;
            safe_c2rust_test_run_success = G_TEST_RUN_SUCCESS;
            let mut _pp: *mut *mut gchar = &raw mut safe_c2rust_test_run_msg;
            let mut _ptr: *mut gchar = *_pp;
            *_pp = ::core::ptr::null_mut::<gchar>();
            if !_ptr.is_null() {
                g_free(_ptr as gpointer);
            }
            g_test_log_set_fatal_handler(None, NULL_0);
            if !safe_c2rust_test_paths_skipped.is_null()
                && !g_slist_find_custom(
                    safe_c2rust_test_paths_skipped,
                    test_run_name as gconstpointer,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>,
                        GCompareFunc,
                    >(Some(
                        safe_c2rust_test_has_prefix
                            as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
                    )),
                )
                .is_null()
            {
                safe_c2rust_g_test_skip(b"by request (-s option)\0" as *const u8 as *const gchar);
            } else {
                let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
                if safe_c2rust_test_do_isolate_dirs(&raw mut local_error) == 0 {
                    safe_c2rust_g_test_log(
                        G_TEST_LOG_ERROR,
                        (*local_error).message,
                        ::core::ptr::null::<gchar>(),
                        0 as guint,
                        ::core::ptr::null_mut::<::f128::f128>(),
                    );
                    safe_c2rust_g_test_fail();
                    g_error_free(local_error);
                } else {
                    g_timer_start(test_run_timer);
                    fixture = if (*tc).fixture_size != 0 {
                        g_malloc0((*tc).fixture_size as gsize) as *mut ::core::ffi::c_void
                    } else {
                        (*tc).test_data as *mut ::core::ffi::c_void
                    };
                    safe_c2rust_test_run_seed(safe_c2rust_test_run_seedstr);
                    if (*tc).fixture_setup.is_some() {
                        (*tc).fixture_setup.expect("non-null function pointer")(
                            fixture,
                            (*tc).test_data as gconstpointer,
                        );
                    }
                    (*tc).fixture_test.expect("non-null function pointer")(
                        fixture,
                        (*tc).test_data as gconstpointer,
                    );
                    safe_c2rust_test_trap_clear();
                    while !safe_c2rust_test_destroy_queue.is_null() {
                        let mut dentry: *mut DestroyEntry = safe_c2rust_test_destroy_queue;
                        safe_c2rust_test_destroy_queue = (*dentry).next;
                        (*dentry).destroy_func.expect("non-null function pointer")(
                            (*dentry).destroy_data,
                        );
                        g_slice_free1(
                            ::core::mem::size_of::<DestroyEntry>() as gsize,
                            dentry as gpointer,
                        );
                    }
                    if (*tc).fixture_teardown.is_some() {
                        (*tc).fixture_teardown.expect("non-null function pointer")(
                            fixture,
                            (*tc).test_data as gconstpointer,
                        );
                    }
                    free_test_data = FALSE as gboolean;
                    if (*tc).fixture_size != 0 {
                        g_free(fixture as gpointer);
                    }
                    g_timer_stop(test_run_timer);
                }
                safe_c2rust_test_rm_isolate_dirs();
            }
            success = safe_c2rust_test_run_success as gboolean;
            safe_c2rust_test_run_success = G_TEST_RUN_FAILURE;
            largs[G_TEST_CASE_LARGS_RESULT as ::core::ffi::c_int as usize] =
                ::f128::f128::new(success);
            largs[G_TEST_CASE_LARGS_RUN_FORKS as ::core::ffi::c_int as usize] =
                ::f128::f128::new(safe_c2rust_test_run_forks);
            largs[G_TEST_CASE_LARGS_EXECUTION_TIME as ::core::ffi::c_int as usize] =
                ::f128::f128::new(g_timer_elapsed(
                    test_run_timer,
                    ::core::ptr::null_mut::<gulong>(),
                ));
            safe_c2rust_g_test_log(
                G_TEST_LOG_STOP_CASE,
                test_run_name as *const gchar,
                safe_c2rust_test_run_msg,
                (::core::mem::size_of::<[::f128::f128; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<::f128::f128>() as usize)
                    as guint,
                &raw mut largs as *mut ::f128::f128,
            );
            let mut _pp_0: *mut *mut gchar = &raw mut safe_c2rust_test_run_msg;
            let mut _ptr_0: *mut gchar = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<gchar>();
            if !_ptr_0.is_null() {
                g_free(_ptr_0 as gpointer);
            }
            g_timer_destroy(test_run_timer);
        }
    }
    if free_test_data != 0 && (*tc).fixture_size == 0 as guint && (*tc).fixture_teardown.is_some() {
        (*tc).fixture_teardown.expect("non-null function pointer")(
            (*tc).test_data as *mut ::core::ffi::c_void,
            (*tc).test_data as gconstpointer,
        );
    }
    g_slist_free_full(
        filename_free_list,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    safe_c2rust_test_filename_free_list = old_free_list;
    g_free(safe_c2rust_test_uri_base as gpointer);
    safe_c2rust_test_uri_base = old_base as *mut ::core::ffi::c_char;
    return (success == G_TEST_RUN_SUCCESS as ::core::ffi::c_int
        || success == G_TEST_RUN_SKIPPED as ::core::ffi::c_int
        || success == G_TEST_RUN_INCOMPLETE as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_path_has_prefix(
    mut path: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> gboolean {
    let mut prefix_len: ::core::ffi::c_int = strlen(prefix) as ::core::ffi::c_int;
    return (strncmp(path, prefix, prefix_len as size_t) == 0 as ::core::ffi::c_int
        && (*path.offset(prefix_len as isize) as ::core::ffi::c_int == '\0' as i32
            || *path.offset(prefix_len as isize) as ::core::ffi::c_int == '/' as i32))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_test_should_run(
    mut test_path: *const ::core::ffi::c_char,
    mut cmp_path: *const ::core::ffi::c_char,
) -> gboolean {
    if !strstr(
        safe_c2rust_test_run_name,
        b"/subprocess\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        if safe_c2rust_g_strcmp0(test_path, cmp_path) == 0 as ::core::ffi::c_int {
            return TRUE;
        }
        if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
            if safe_c2rust_test_tap_log != 0 {
                g_print(
                    b"skipping: %s\n\0" as *const u8 as *const gchar,
                    safe_c2rust_test_run_name,
                );
            } else {
                g_print(
                    b"GTest: skipping: %s\n\0" as *const u8 as *const gchar,
                    safe_c2rust_test_run_name,
                );
            }
        }
        return FALSE;
    }
    return (cmp_path.is_null() || safe_c2rust_path_has_prefix(test_path, cmp_path) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_test_run_suite_internal(
    mut suite: *mut GTestSuite,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n_bad: guint = 0 as guint;
    let mut old_name: *mut gchar = safe_c2rust_test_run_name;
    let mut old_name_path: *mut gchar = safe_c2rust_test_run_name_path;
    let mut iter: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !suite.is_null() {
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
            b"suite != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    safe_c2rust_g_test_log(
        G_TEST_LOG_START_SUITE,
        (*suite).name,
        ::core::ptr::null::<gchar>(),
        0 as guint,
        ::core::ptr::null_mut::<::f128::f128>(),
    );
    iter = (*suite).cases;
    while !iter.is_null() {
        let mut tc: *mut GTestCase = (*iter).data as *mut GTestCase;
        safe_c2rust_test_run_name = g_build_path(
            b"/\0" as *const u8 as *const gchar,
            old_name,
            (*tc).name,
            NULL_0,
        );
        safe_c2rust_test_run_name_path = g_build_path(
            G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
            old_name_path,
            (*tc).name,
            NULL_0,
        );
        if safe_c2rust_test_case_run(tc, safe_c2rust_test_run_name, path) == 0 {
            n_bad = n_bad.wrapping_add(1);
        }
        g_free(safe_c2rust_test_run_name as gpointer);
        g_free(safe_c2rust_test_run_name_path as gpointer);
        iter = (*iter).next;
    }
    iter = (*suite).suites;
    while !iter.is_null() {
        let mut ts: *mut GTestSuite = (*iter).data as *mut GTestSuite;
        safe_c2rust_test_run_name = g_build_path(
            b"/\0" as *const u8 as *const gchar,
            old_name,
            (*ts).name,
            NULL_0,
        );
        safe_c2rust_test_run_name_path = g_build_path(
            G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
            old_name_path,
            (*ts).name,
            NULL_0,
        );
        if safe_c2rust_test_prefix_extended != 0 {
            if path.is_null() || safe_c2rust_path_has_prefix(safe_c2rust_test_run_name, path) != 0 {
                n_bad = n_bad.wrapping_add(safe_c2rust_g_test_run_suite_internal(
                    ts,
                    safe_c2rust_test_run_name,
                ) as guint);
            } else if path.is_null()
                || safe_c2rust_path_has_prefix(path, safe_c2rust_test_run_name) != 0
            {
                n_bad =
                    n_bad.wrapping_add(safe_c2rust_g_test_run_suite_internal(ts, path) as guint);
            }
        } else if path.is_null()
            || safe_c2rust_path_has_prefix(path, safe_c2rust_test_run_name) != 0
        {
            n_bad = n_bad.wrapping_add(safe_c2rust_g_test_run_suite_internal(ts, path) as guint);
        }
        g_free(safe_c2rust_test_run_name as gpointer);
        g_free(safe_c2rust_test_run_name_path as gpointer);
        iter = (*iter).next;
    }
    safe_c2rust_test_run_name = old_name;
    safe_c2rust_test_run_name_path = old_name_path;
    safe_c2rust_g_test_log(
        G_TEST_LOG_STOP_SUITE,
        (*suite).name,
        ::core::ptr::null::<gchar>(),
        0 as guint,
        ::core::ptr::null_mut::<::f128::f128>(),
    );
    return n_bad as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_test_suite_count(
    mut suite: *mut GTestSuite,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iter: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !suite.is_null() {
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
            b"suite != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    iter = (*suite).cases;
    while !iter.is_null() {
        let mut tc: *mut GTestCase = (*iter).data as *mut GTestCase;
        if strcmp(
            (*tc).name,
            b"subprocess\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            n += 1;
        }
        iter = (*iter).next;
    }
    iter = (*suite).suites;
    while !iter.is_null() {
        let mut ts: *mut GTestSuite = (*iter).data as *mut GTestSuite;
        if strcmp(
            (*ts).name,
            b"subprocess\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            n += safe_c2rust_g_test_suite_count(ts);
        }
        iter = (*iter).next;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_run_suite(
    mut suite: *mut GTestSuite,
) -> ::core::ffi::c_int {
    let mut n_bad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if safe_c2rust_g_test_run_once == (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int {
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
            b"g_test_run_once == TRUE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    safe_c2rust_g_test_run_once = FALSE as gboolean;
    safe_c2rust_test_count = safe_c2rust_g_test_suite_count(suite) as guint;
    safe_c2rust_test_run_name =
        g_strdup_printf(b"/%s\0" as *const u8 as *const gchar, (*suite).name);
    safe_c2rust_test_run_name_path = g_build_path(
        G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
        (*suite).name,
        NULL_0,
    );
    if !safe_c2rust_test_paths.is_null() {
        let mut iter: *mut GSList = ::core::ptr::null_mut::<GSList>();
        iter = safe_c2rust_test_paths;
        while !iter.is_null() {
            n_bad += safe_c2rust_g_test_run_suite_internal(
                suite,
                (*iter).data as *const ::core::ffi::c_char,
            );
            iter = (*iter).next;
        }
    } else {
        n_bad = safe_c2rust_g_test_run_suite_internal(
            suite,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    let mut _pp: *mut *mut gchar = &raw mut safe_c2rust_test_run_name;
    let mut _ptr: *mut gchar = *_pp;
    *_pp = ::core::ptr::null_mut::<gchar>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut gchar = &raw mut safe_c2rust_test_run_name_path;
    let mut _ptr_0: *mut gchar = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<gchar>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    return n_bad;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_case_free(mut test_case: *mut GTestCase) {
    g_free((*test_case).name as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GTestCase>() as gsize,
        test_case as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_suite_free(mut suite: *mut GTestSuite) {
    g_slist_free_full(
        (*suite).cases,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GTestCase) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_g_test_case_free as unsafe extern "C" fn(*mut GTestCase) -> ()),
        ),
    );
    g_free((*suite).name as gpointer);
    g_slist_free_full(
        (*suite).suites,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GTestSuite) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_g_test_suite_free as unsafe extern "C" fn(*mut GTestSuite) -> ()),
        ),
    );
    g_slice_free1(
        ::core::mem::size_of::<GTestSuite>() as gsize,
        suite as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_gtest_default_log_handler(
    mut log_domain: *const gchar,
    mut log_level: GLogLevelFlags,
    mut message: *const gchar,
    mut unused_data: gpointer,
) {
    let mut strv: [*const gchar; 16] = [::core::ptr::null::<gchar>(); 16];
    let mut fatal: gboolean = FALSE;
    let mut msg: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: guint = 0 as guint;
    if !log_domain.is_null() {
        let fresh17 = i;
        i = i.wrapping_add(1);
        strv[fresh17 as usize] = log_domain;
        let fresh18 = i;
        i = i.wrapping_add(1);
        strv[fresh18 as usize] = b"-\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_FATAL as ::core::ffi::c_int != 0 {
        let fresh19 = i;
        i = i.wrapping_add(1);
        strv[fresh19 as usize] =
            b"FATAL-\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        fatal = TRUE as gboolean;
    }
    if log_level as ::core::ffi::c_int & G_LOG_FLAG_RECURSION as ::core::ffi::c_int != 0 {
        let fresh20 = i;
        i = i.wrapping_add(1);
        strv[fresh20 as usize] =
            b"RECURSIVE-\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_ERROR as ::core::ffi::c_int != 0 {
        let fresh21 = i;
        i = i.wrapping_add(1);
        strv[fresh21 as usize] =
            b"ERROR\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_CRITICAL as ::core::ffi::c_int != 0 {
        let fresh22 = i;
        i = i.wrapping_add(1);
        strv[fresh22 as usize] =
            b"CRITICAL\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_WARNING as ::core::ffi::c_int != 0 {
        let fresh23 = i;
        i = i.wrapping_add(1);
        strv[fresh23 as usize] =
            b"WARNING\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_MESSAGE as ::core::ffi::c_int != 0 {
        let fresh24 = i;
        i = i.wrapping_add(1);
        strv[fresh24 as usize] =
            b"MESSAGE\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_INFO as ::core::ffi::c_int != 0 {
        let fresh25 = i;
        i = i.wrapping_add(1);
        strv[fresh25 as usize] =
            b"INFO\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if log_level as ::core::ffi::c_int & G_LOG_LEVEL_DEBUG as ::core::ffi::c_int != 0 {
        let fresh26 = i;
        i = i.wrapping_add(1);
        strv[fresh26 as usize] =
            b"DEBUG\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    let fresh27 = i;
    i = i.wrapping_add(1);
    strv[fresh27 as usize] = b": \0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    let fresh28 = i;
    i = i.wrapping_add(1);
    strv[fresh28 as usize] = message;
    let fresh29 = i;
    i = i.wrapping_add(1);
    strv[fresh29 as usize] = ::core::ptr::null::<gchar>();
    msg = g_strjoinv(
        b"\0" as *const u8 as *const gchar,
        &raw mut strv as *mut *const gchar as *mut *mut gchar,
    );
    safe_c2rust_g_test_log(
        (if fatal != 0 {
            G_TEST_LOG_ERROR as ::core::ffi::c_int
        } else {
            G_TEST_LOG_MESSAGE as ::core::ffi::c_int
        }) as GTestLogType,
        msg,
        ::core::ptr::null::<gchar>(),
        0 as guint,
        ::core::ptr::null_mut::<::f128::f128>(),
    );
    g_free(msg as gpointer);
    if safe_c2rust_test_tap_log == 0 {
        g_log_default_handler(log_domain, log_level, message, unused_data);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assertion_message(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut message: *const ::core::ffi::c_char,
) {
    let mut lstr: [::core::ffi::c_char; 32] = [0; 32];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if message.is_null() {
        message = b"code should not be reached\0" as *const u8 as *const ::core::ffi::c_char;
    }
    g_snprintf(
        &raw mut lstr as *mut gchar,
        32 as gulong,
        b"%d\0" as *const u8 as *const gchar,
        line,
    );
    s = g_strconcat(
        if !domain.is_null() {
            domain as *const gchar
        } else {
            b"\0" as *const u8 as *const gchar
        },
        if !domain.is_null()
            && *domain.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            b":\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        b"ERROR:\0" as *const u8 as *const ::core::ffi::c_char,
        file,
        b":\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut lstr as *mut ::core::ffi::c_char,
        b":\0" as *const u8 as *const ::core::ffi::c_char,
        func,
        if *func.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
            b":\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        b" \0" as *const u8 as *const ::core::ffi::c_char,
        message,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    if (*safe_c2rust_g_test_config_vars).test_initialized == 0 {
        g_print(b"%s\n\0" as *const u8 as *const gchar, s);
    }
    g_printerr(b"**\n%s\n\0" as *const u8 as *const gchar, s);
    if safe_c2rust_test_nonfatal_assertions != 0
        || safe_c2rust_test_in_subprocess != 0
        || safe_c2rust_test_in_forked_child != 0
    {
        safe_c2rust_g_test_log(
            G_TEST_LOG_MESSAGE,
            s,
            ::core::ptr::null::<gchar>(),
            0 as guint,
            ::core::ptr::null_mut::<::f128::f128>(),
        );
    } else {
        safe_c2rust_g_test_log(
            G_TEST_LOG_ERROR,
            s,
            ::core::ptr::null::<gchar>(),
            0 as guint,
            ::core::ptr::null_mut::<::f128::f128>(),
        );
    }
    if safe_c2rust_test_nonfatal_assertions != 0 {
        g_free(s as gpointer);
        safe_c2rust_g_test_fail();
        return;
    }
    if !safe_c2rust___glib_assert_msg.is_null() {
        free(safe_c2rust___glib_assert_msg as *mut ::core::ffi::c_void);
    }
    safe_c2rust___glib_assert_msg =
        malloc(strlen(s).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    strcpy(safe_c2rust___glib_assert_msg, s);
    crate::data::GLIB_ASSERT_MSG = safe_c2rust___glib_assert_msg as usize;
    g_free(s as gpointer);
    if safe_c2rust_test_in_subprocess != 0 {
        _exit(1 as ::core::ffi::c_int);
    } else {
        abort();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assertion_message_expr(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut expr: *const ::core::ffi::c_char,
) -> ! {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if expr.is_null() {
        s = safe_c2rust_g_strdup_inline(
            b"code should not be reached\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        s = g_strconcat(
            b"assertion failed: (\0" as *const u8 as *const gchar,
            expr,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
    }
    safe_c2rust_g_assertion_message(domain, file, line, func, s);
    g_free(s as gpointer);
    if safe_c2rust_test_in_subprocess != 0 {
        _exit(1 as ::core::ffi::c_int);
    } else {
        abort();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assertion_message_cmpint(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut expr: *const ::core::ffi::c_char,
    mut arg1: guint64,
    mut cmp: *const ::core::ffi::c_char,
    mut arg2: guint64,
    mut numtype: ::core::ffi::c_char,
) {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match numtype as ::core::ffi::c_int {
        105 => {
            s = g_strdup_printf(
                b"assertion failed (%s): (%li %s %li)\0" as *const u8 as *const gchar,
                expr,
                arg1 as int64_t,
                cmp,
                arg2 as int64_t,
            ) as *mut ::core::ffi::c_char;
        }
        117 => {
            s = g_strdup_printf(
                b"assertion failed (%s): (%lu %s %lu)\0" as *const u8 as *const gchar,
                expr,
                arg1 as uint64_t,
                cmp,
                arg2 as uint64_t,
            ) as *mut ::core::ffi::c_char;
        }
        120 => {
            s = g_strdup_printf(
                b"assertion failed (%s): (0x%08lx %s 0x%08lx)\0" as *const u8 as *const gchar,
                expr,
                arg1 as uint64_t,
                cmp,
                arg2 as uint64_t,
            ) as *mut ::core::ffi::c_char;
        }
        _ => {
            safe_c2rust_g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                3401 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    safe_c2rust_g_assertion_message(domain, file, line, func, s);
    g_free(s as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assertion_message_cmpnum(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut expr: *const ::core::ffi::c_char,
    mut arg1: ::f128::f128,
    mut cmp: *const ::core::ffi::c_char,
    mut arg2: ::f128::f128,
    mut numtype: ::core::ffi::c_char,
) {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match numtype as ::core::ffi::c_int {
        102 => {
            s = g_strdup_printf(
                b"assertion failed (%s): (%.9g %s %.9g)\0" as *const u8 as *const gchar,
                expr,
                arg1.to_f64().unwrap(),
                cmp,
                arg2.to_f64().unwrap(),
            ) as *mut ::core::ffi::c_char;
        }
        105 | 120 => {
            safe_c2rust_g_assertion_message_cmpint(
                domain,
                file,
                line,
                func,
                expr,
                arg1.to_u64().unwrap(),
                cmp,
                arg2.to_u64().unwrap(),
                numtype,
            );
        }
        _ => {
            safe_c2rust_g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                3431 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    safe_c2rust_g_assertion_message(domain, file, line, func, s);
    g_free(s as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assertion_message_cmpstr(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut expr: *const ::core::ffi::c_char,
    mut arg1: *const ::core::ffi::c_char,
    mut cmp: *const ::core::ffi::c_char,
    mut arg2: *const ::core::ffi::c_char,
) {
    let mut a1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut a2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    a1 = if !arg1.is_null() {
        t1 = g_strescape(arg1 as *const gchar, ::core::ptr::null::<gchar>())
            as *mut ::core::ffi::c_char;
        g_strconcat(
            b"\"\0" as *const u8 as *const gchar,
            t1,
            b"\"\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char
    } else {
        safe_c2rust_g_strdup_inline(b"NULL\0" as *const u8 as *const ::core::ffi::c_char)
    };
    a2 = if !arg2.is_null() {
        t2 = g_strescape(arg2 as *const gchar, ::core::ptr::null::<gchar>())
            as *mut ::core::ffi::c_char;
        g_strconcat(
            b"\"\0" as *const u8 as *const gchar,
            t2,
            b"\"\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char
    } else {
        safe_c2rust_g_strdup_inline(b"NULL\0" as *const u8 as *const ::core::ffi::c_char)
    };
    g_free(t1 as gpointer);
    g_free(t2 as gpointer);
    s = g_strdup_printf(
        b"assertion failed (%s): (%s %s %s)\0" as *const u8 as *const gchar,
        expr,
        a1,
        cmp,
        a2,
    ) as *mut ::core::ffi::c_char;
    g_free(a1 as gpointer);
    g_free(a2 as gpointer);
    safe_c2rust_g_assertion_message(domain, file, line, func, s);
    g_free(s as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assertion_message_cmpstrv(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut expr: *const ::core::ffi::c_char,
    mut arg1: *const *const ::core::ffi::c_char,
    mut arg2: *const *const ::core::ffi::c_char,
    mut first_wrong_idx: gsize,
) {
    let mut s1: *const ::core::ffi::c_char = *arg1.offset(first_wrong_idx as isize);
    let mut s2: *const ::core::ffi::c_char = *arg2.offset(first_wrong_idx as isize);
    let mut a1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut a2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    t1 = g_strescape(s1 as *const gchar, ::core::ptr::null::<gchar>()) as *mut ::core::ffi::c_char;
    a1 = g_strconcat(
        b"\"\0" as *const u8 as *const gchar,
        t1,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    t2 = g_strescape(s2 as *const gchar, ::core::ptr::null::<gchar>()) as *mut ::core::ffi::c_char;
    a2 = g_strconcat(
        b"\"\0" as *const u8 as *const gchar,
        t2,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    g_free(t1 as gpointer);
    g_free(t2 as gpointer);
    s = g_strdup_printf(
        b"assertion failed (%s): first differing element at index %lu: %s does not equal %s\0"
            as *const u8 as *const gchar,
        expr,
        first_wrong_idx,
        a1,
        a2,
    ) as *mut ::core::ffi::c_char;
    g_free(a1 as gpointer);
    g_free(a2 as gpointer);
    safe_c2rust_g_assertion_message(domain, file, line, func, s);
    g_free(s as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_assertion_message_error(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut expr: *const ::core::ffi::c_char,
    mut error: *const GError,
    mut error_domain: GQuark,
    mut error_code: ::core::ffi::c_int,
) {
    let mut gstring: *mut GString = ::core::ptr::null_mut::<GString>();
    gstring = g_string_new(b"assertion failed \0" as *const u8 as *const gchar);
    if error_domain != 0 {
        g_string_append_printf(
            gstring,
            b"(%s == (%s, %d)): \0" as *const u8 as *const gchar,
            expr,
            g_quark_to_string(error_domain),
            error_code,
        );
    } else {
        g_string_append_printf(
            gstring,
            b"(%s == NULL): \0" as *const u8 as *const gchar,
            expr,
        );
    }
    if !error.is_null() {
        g_string_append_printf(
            gstring,
            b"%s (%s, %d)\0" as *const u8 as *const gchar,
            (*error).message,
            g_quark_to_string((*error).domain),
            (*error).code,
        );
    } else {
        g_string_append_printf(gstring, b"%s is NULL\0" as *const u8 as *const gchar, expr);
    }
    safe_c2rust_g_assertion_message(domain, file, line, func, (*gstring).str_0);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                gstring,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(gstring);
        };
    } else {
        g_string_free(
            gstring,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strcmp0(
    mut str1: *const ::core::ffi::c_char,
    mut str2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if str1.is_null() {
        return -((str1 != str2) as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return (str1 != str2) as ::core::ffi::c_int;
    }
    return strcmp(str1, str2);
}
unsafe extern "C" fn safe_c2rust_test_trap_clear() {
    safe_c2rust_test_trap_last_status = 0 as ::core::ffi::c_int;
    safe_c2rust_test_trap_last_pid = 0 as ::core::ffi::c_int as GPid;
    let mut _pp: *mut *mut ::core::ffi::c_char = &raw mut safe_c2rust_test_trap_last_subprocess;
    let mut _ptr: *mut ::core::ffi::c_char = *_pp;
    *_pp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut ::core::ffi::c_char = &raw mut safe_c2rust_test_trap_last_stdout;
    let mut _ptr_0: *mut ::core::ffi::c_char = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut ::core::ffi::c_char = &raw mut safe_c2rust_test_trap_last_stderr;
    let mut _ptr_1: *mut ::core::ffi::c_char = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr_1.is_null() {
        g_free(_ptr_1 as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_safe_dup2(
    mut fd1: ::core::ffi::c_int,
    mut fd2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    loop {
        ret = dup2(fd1, fd2);
        if !(ret < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_check_complete(mut data: *mut WaitForChildData) {
    if (*data).child_status != -(1 as ::core::ffi::c_int)
        && (*data).stdout_io.is_null()
        && (*data).stderr_io.is_null()
    {
        g_main_loop_quit((*data).loop_0);
    }
}
unsafe extern "C" fn safe_c2rust_child_exited(
    mut pid: GPid,
    mut status: gint,
    mut user_data: gpointer,
) {
    let mut data: *mut WaitForChildData = user_data as *mut WaitForChildData;
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if status != -(1 as ::core::ffi::c_int) {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            3595 as ::core::ffi::c_int,
            G_STRFUNC,
            b"status != -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*data).child_status = status as ::core::ffi::c_int;
    safe_c2rust_check_complete(data);
}
unsafe extern "C" fn safe_c2rust_child_timeout(mut user_data: gpointer) -> gboolean {
    let mut data: *mut WaitForChildData = user_data as *mut WaitForChildData;
    kill((*data).pid as __pid_t, SIGALRM);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_child_read(
    mut io: *mut GIOChannel,
    mut cond: GIOCondition,
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut WaitForChildData = user_data as *mut WaitForChildData;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut nread: gsize = 0;
    let mut nwrote: gsize = 0;
    let mut total: gsize = 0;
    let mut buf: [gchar; 4096] = [0; 4096];
    let mut echo_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    status = g_io_channel_read_chars(
        io,
        &raw mut buf as *mut gchar,
        ::core::mem::size_of::<[gchar; 4096]>() as gsize,
        &raw mut nread,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if status as ::core::ffi::c_uint
        == G_IO_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        || status as ::core::ffi::c_uint
            == G_IO_STATUS_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if io == (*data).stdout_io {
            let mut _pp: *mut *mut GIOChannel = &raw mut (*data).stdout_io;
            let mut _ptr: *mut GIOChannel = *_pp;
            *_pp = ::core::ptr::null_mut::<GIOChannel>();
            if !_ptr.is_null() {
                g_io_channel_unref(_ptr as *mut GIOChannel);
            }
        } else {
            let mut _pp_0: *mut *mut GIOChannel = &raw mut (*data).stderr_io;
            let mut _ptr_0: *mut GIOChannel = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<GIOChannel>();
            if !_ptr_0.is_null() {
                g_io_channel_unref(_ptr_0 as *mut GIOChannel);
            }
        }
        safe_c2rust_check_complete(data);
        return FALSE;
    } else if status as ::core::ffi::c_uint
        == G_IO_STATUS_AGAIN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return TRUE;
    }
    if io == (*data).stdout_io {
        safe_c2rust_g_string_append_len_inline(
            (*data).stdout_str,
            &raw mut buf as *mut gchar,
            nread as gssize,
        );
        if (*data).echo_stdout != 0 {
            if ({
                let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
                if safe_c2rust_test_tap_log == 0 {
                    _g_boolean_var_59 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_59 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_59
            }) as ::core::ffi::c_long
                != 0
            {
                echo_file = safe_c2rust_stdout;
            }
        }
    } else {
        safe_c2rust_g_string_append_len_inline(
            (*data).stderr_str,
            &raw mut buf as *mut gchar,
            nread as gssize,
        );
        if (*data).echo_stderr != 0 {
            echo_file = safe_c2rust_stderr;
        }
    }
    if !echo_file.is_null() {
        total = 0 as gsize;
        while total < nread {
            let mut errsv: ::core::ffi::c_int = 0;
            nwrote = fwrite(
                (&raw mut buf as *mut gchar).offset(total as isize) as *const ::core::ffi::c_void,
                1 as size_t,
                (nread as size_t).wrapping_sub(total as size_t),
                echo_file,
            ) as gsize;
            errsv = *__errno_location();
            if nwrote == 0 as gsize {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"write failed: %s\0" as *const u8 as *const gchar,
                    g_strerror(errsv as gint),
                );
                loop {}
            }
            total = total.wrapping_add(nwrote);
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_wait_for_child(
    mut pid: GPid,
    mut stdout_fd: ::core::ffi::c_int,
    mut echo_stdout: gboolean,
    mut stderr_fd: ::core::ffi::c_int,
    mut echo_stderr: gboolean,
    mut timeout: guint64,
) {
    let mut data: WaitForChildData = WaitForChildData {
        pid: 0,
        loop_0: ::core::ptr::null_mut::<GMainLoop>(),
        child_status: 0,
        stdout_io: ::core::ptr::null_mut::<GIOChannel>(),
        echo_stdout: 0,
        stdout_str: ::core::ptr::null_mut::<GString>(),
        stderr_io: ::core::ptr::null_mut::<GIOChannel>(),
        echo_stderr: 0,
        stderr_str: ::core::ptr::null_mut::<GString>(),
    };
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    data.pid = pid;
    data.child_status = -(1 as ::core::ffi::c_int);
    context = g_main_context_new();
    data.loop_0 = g_main_loop_new(context, FALSE);
    source = g_child_watch_source_new(pid);
    g_source_set_callback(
        source,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(GPid, gint, gpointer) -> ()>,
            GSourceFunc,
        >(Some(
            safe_c2rust_child_exited as unsafe extern "C" fn(GPid, gint, gpointer) -> (),
        )),
        &raw mut data as gpointer,
        None,
    );
    g_source_attach(source, context);
    g_source_unref(source);
    data.echo_stdout = echo_stdout;
    data.stdout_str = g_string_new(::core::ptr::null::<gchar>());
    data.stdout_io = g_io_channel_unix_new(stdout_fd);
    g_io_channel_set_close_on_unref(data.stdout_io, TRUE);
    g_io_channel_set_encoding(
        data.stdout_io,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_io_channel_set_buffered(data.stdout_io, FALSE);
    source = g_io_create_watch(
        data.stdout_io,
        (G_IO_IN as ::core::ffi::c_int
            | G_IO_ERR as ::core::ffi::c_int
            | G_IO_HUP as ::core::ffi::c_int) as GIOCondition,
    );
    g_source_set_callback(
        source,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean>,
            GSourceFunc,
        >(Some(
            safe_c2rust_child_read
                as unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean,
        )),
        &raw mut data as gpointer,
        None,
    );
    g_source_attach(source, context);
    g_source_unref(source);
    data.echo_stderr = echo_stderr;
    data.stderr_str = g_string_new(::core::ptr::null::<gchar>());
    data.stderr_io = g_io_channel_unix_new(stderr_fd);
    g_io_channel_set_close_on_unref(data.stderr_io, TRUE);
    g_io_channel_set_encoding(
        data.stderr_io,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_io_channel_set_buffered(data.stderr_io, FALSE);
    source = g_io_create_watch(
        data.stderr_io,
        (G_IO_IN as ::core::ffi::c_int
            | G_IO_ERR as ::core::ffi::c_int
            | G_IO_HUP as ::core::ffi::c_int) as GIOCondition,
    );
    g_source_set_callback(
        source,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean>,
            GSourceFunc,
        >(Some(
            safe_c2rust_child_read
                as unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean,
        )),
        &raw mut data as gpointer,
        None,
    );
    g_source_attach(source, context);
    g_source_unref(source);
    if timeout != 0 {
        source = g_timeout_source_new(0 as guint);
        g_source_set_ready_time(
            source,
            (g_get_monotonic_time() as guint64).wrapping_add(timeout) as gint64,
        );
        g_source_set_callback(
            source,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gboolean>, GSourceFunc>(
                Some(safe_c2rust_child_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
            ),
            &raw mut data as gpointer,
            None,
        );
        g_source_attach(source, context);
        g_source_unref(source);
    }
    g_main_loop_run(data.loop_0);
    g_main_loop_unref(data.loop_0);
    g_main_context_unref(context);
    if echo_stdout != 0 && safe_c2rust_test_tap_log != 0 && (*data.stdout_str).len > 0 as gsize {
        let mut added_newline: gboolean = FALSE;
        if !safe_c2rust_test_trap_last_subprocess.is_null() {
            let prefix = g_strdup_printf(
                b"%s: \0" as *const u8 as *const gchar,
                safe_c2rust_test_trap_last_subprocess,
            );
            g_string_prepend(data.stdout_str, prefix);
            g_free(prefix as gpointer);
        }
        if *(*data.stdout_str)
            .str_0
            .offset((*data.stdout_str).len.wrapping_sub(1 as gsize) as isize)
            as ::core::ffi::c_int
            != '\n' as i32
        {
            safe_c2rust_g_string_append_c_inline(data.stdout_str, '\n' as i32 as gchar);
            added_newline = TRUE as gboolean;
        }
        safe_c2rust_g_test_print_handler_full(
            (*data.stdout_str).str_0,
            TRUE,
            TRUE,
            1 as ::core::ffi::c_uint,
        );
        if added_newline != 0 {
            safe_c2rust_g_string_truncate_inline(
                data.stdout_str,
                (*data.stdout_str).len.wrapping_sub(1 as gsize),
            );
        }
    }
    safe_c2rust_test_trap_last_pid = pid;
    safe_c2rust_test_trap_last_status = data.child_status;
    safe_c2rust_test_trap_last_stdout = (if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(data.stdout_str, 0 as gboolean)
        } else {
            g_string_free_and_steal(data.stdout_str)
        }
    } else {
        g_string_free(data.stdout_str, 0 as gboolean)
    }) as *mut ::core::ffi::c_char;
    safe_c2rust_test_trap_last_stderr = (if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(data.stderr_str, 0 as gboolean)
        } else {
            g_string_free_and_steal(data.stderr_str)
        }
    } else {
        g_string_free(data.stderr_str, 0 as gboolean)
    }) as *mut ::core::ffi::c_char;
    let mut _pp: *mut *mut GIOChannel = &raw mut data.stdout_io;
    let mut _ptr: *mut GIOChannel = *_pp;
    *_pp = ::core::ptr::null_mut::<GIOChannel>();
    if !_ptr.is_null() {
        g_io_channel_unref(_ptr as *mut GIOChannel);
    }
    let mut _pp_0: *mut *mut GIOChannel = &raw mut data.stderr_io;
    let mut _ptr_0: *mut GIOChannel = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GIOChannel>();
    if !_ptr_0.is_null() {
        g_io_channel_unref(_ptr_0 as *mut GIOChannel);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_trap_fork(
    mut usec_timeout: guint64,
    mut test_trap_flags: GTestTrapFlags,
) -> gboolean {
    let mut stdout_pipe: [::core::ffi::c_int; 2] =
        [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)];
    let mut stderr_pipe: [::core::ffi::c_int; 2] =
        [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)];
    let mut errsv: ::core::ffi::c_int = 0;
    safe_c2rust_test_trap_clear();
    if pipe(&raw mut stdout_pipe as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || pipe(&raw mut stderr_pipe as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        errsv = *__errno_location();
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"failed to create pipes to fork test program: %s\0" as *const u8 as *const gchar,
            g_strerror(errsv as gint),
        );
        loop {}
    }
    safe_c2rust_test_trap_last_pid = fork() as GPid;
    errsv = *__errno_location();
    if safe_c2rust_test_trap_last_pid < 0 as ::core::ffi::c_int {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"failed to fork test program: %s\0" as *const u8 as *const gchar,
            g_strerror(errsv as gint),
        );
        loop {}
    }
    if safe_c2rust_test_trap_last_pid == 0 as ::core::ffi::c_int {
        let mut fd0: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        safe_c2rust_test_in_forked_child = TRUE as gboolean;
        close(stdout_pipe[0 as ::core::ffi::c_int as usize]);
        close(stderr_pipe[0 as ::core::ffi::c_int as usize]);
        if test_trap_flags as ::core::ffi::c_uint
            & G_TEST_TRAP_INHERIT_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            fd0 = open(
                b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
                O_RDONLY,
                0 as ::core::ffi::c_int,
            );
            if fd0 < 0 as ::core::ffi::c_int {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"failed to open /dev/null for stdin redirection\0" as *const u8
                        as *const gchar,
                );
                loop {}
            }
        }
        if safe_c2rust_safe_dup2(
            stdout_pipe[1 as ::core::ffi::c_int as usize],
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
            || safe_c2rust_safe_dup2(
                stderr_pipe[1 as ::core::ffi::c_int as usize],
                2 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            || fd0 >= 0 as ::core::ffi::c_int
                && safe_c2rust_safe_dup2(fd0, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            errsv = *__errno_location();
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"failed to dup2() in forked test program: %s\0" as *const u8 as *const gchar,
                g_strerror(errsv as gint),
            );
            loop {}
        }
        if fd0 >= 3 as ::core::ffi::c_int {
            close(fd0);
        }
        if stdout_pipe[1 as ::core::ffi::c_int as usize] >= 3 as ::core::ffi::c_int {
            close(stdout_pipe[1 as ::core::ffi::c_int as usize]);
        }
        if stderr_pipe[1 as ::core::ffi::c_int as usize] >= 3 as ::core::ffi::c_int {
            close(stderr_pipe[1 as ::core::ffi::c_int as usize]);
        }
        safe_c2rust_g_test_disable_crash_reporting();
        return TRUE;
    } else {
        safe_c2rust_test_run_forks = safe_c2rust_test_run_forks.wrapping_add(1);
        close(stdout_pipe[1 as ::core::ffi::c_int as usize]);
        close(stderr_pipe[1 as ::core::ffi::c_int as usize]);
        safe_c2rust_wait_for_child(
            safe_c2rust_test_trap_last_pid,
            stdout_pipe[0 as ::core::ffi::c_int as usize],
            (test_trap_flags as ::core::ffi::c_uint
                & G_TEST_TRAP_SILENCE_STDOUT as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0) as ::core::ffi::c_int,
            stderr_pipe[0 as ::core::ffi::c_int as usize],
            (test_trap_flags as ::core::ffi::c_uint
                & G_TEST_TRAP_SILENCE_STDERR as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0) as ::core::ffi::c_int,
            usec_timeout,
        );
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_trap_subprocess(
    mut test_path: *const ::core::ffi::c_char,
    mut usec_timeout: guint64,
    mut test_flags: GTestSubprocessFlags,
) {
    safe_c2rust_g_test_trap_subprocess_with_envp(
        test_path,
        ::core::ptr::null::<*const ::core::ffi::c_char>(),
        usec_timeout,
        test_flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_trap_subprocess_with_envp(
    mut test_path: *const ::core::ffi::c_char,
    mut envp: *const *const ::core::ffi::c_char,
    mut usec_timeout: guint64,
    mut test_flags: GTestSubprocessFlags,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut argv: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut flags: GSpawnFlags = G_SPAWN_DEFAULT;
    let mut stdout_fd: ::core::ffi::c_int = 0;
    let mut stderr_fd: ::core::ffi::c_int = 0;
    let mut pid: GPid = 0;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if test_flags as ::core::ffi::c_uint
            & (G_TEST_TRAP_INHERIT_STDIN as ::core::ffi::c_int
                | G_TEST_TRAP_SILENCE_STDOUT as ::core::ffi::c_int
                | G_TEST_TRAP_SILENCE_STDERR as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
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
        safe_c2rust_g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4003 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(test_flags & (G_TEST_TRAP_INHERIT_STDIN | G_TEST_TRAP_SILENCE_STDOUT | G_TEST_TRAP_SILENCE_STDERR)) == 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !test_path.is_null() {
        if safe_c2rust_g_test_suite_case_exists(safe_c2rust_g_test_get_root(), test_path) == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"g_test_trap_subprocess: test does not exist: %s\0" as *const u8 as *const gchar,
                test_path,
            );
            loop {}
        }
    } else {
        test_path = safe_c2rust_test_run_name;
    }
    if (*safe_c2rust_g_test_config_vars).test_verbose != 0 {
        if safe_c2rust_test_tap_log != 0 {
            g_print(
                b"subprocess: %s\n\0" as *const u8 as *const gchar,
                test_path,
            );
        } else {
            g_print(
                b"GTest: subprocess: %s\n\0" as *const u8 as *const gchar,
                test_path,
            );
        }
    }
    safe_c2rust_test_trap_clear();
    safe_c2rust_test_trap_last_subprocess = safe_c2rust_g_strdup_inline(test_path);
    if safe_c2rust_test_argv0.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_test_trap_subprocess() requires argv0 to be passed to g_test_init()\0" as *const u8
                as *const gchar,
        );
        loop {}
    }
    argv = g_ptr_array_new();
    g_ptr_array_add(
        argv,
        safe_c2rust_test_argv0 as *mut ::core::ffi::c_char as gpointer,
    );
    g_ptr_array_add(
        argv,
        b"-q\0" as *const u8 as *const ::core::ffi::c_char as gpointer,
    );
    g_ptr_array_add(
        argv,
        b"-p\0" as *const u8 as *const ::core::ffi::c_char as gpointer,
    );
    g_ptr_array_add(argv, test_path as *mut ::core::ffi::c_char as gpointer);
    g_ptr_array_add(
        argv,
        b"--GTestSubprocess\0" as *const u8 as *const ::core::ffi::c_char as gpointer,
    );
    if safe_c2rust_test_log_fd != -(1 as ::core::ffi::c_int) {
        let mut log_fd_buf: [::core::ffi::c_char; 128] = [0; 128];
        g_ptr_array_add(
            argv,
            b"--GTestLogFD\0" as *const u8 as *const ::core::ffi::c_char as gpointer,
        );
        g_snprintf(
            &raw mut log_fd_buf as *mut gchar,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as gulong,
            b"%d\0" as *const u8 as *const gchar,
            safe_c2rust_test_log_fd,
        );
        g_ptr_array_add(
            argv,
            &raw mut log_fd_buf as *mut ::core::ffi::c_char as gpointer,
        );
    }
    g_ptr_array_add(argv, NULL_0);
    flags = G_SPAWN_DO_NOT_REAP_CHILD;
    if safe_c2rust_test_log_fd != -(1 as ::core::ffi::c_int) {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
            flags as ::core::ffi::c_uint
                | G_SPAWN_LEAVE_DESCRIPTORS_OPEN as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if test_flags as ::core::ffi::c_uint
        & G_TEST_TRAP_INHERIT_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GSpawnFlags>(
            flags as ::core::ffi::c_uint
                | G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if g_spawn_async_with_pipes(
        safe_c2rust_test_initial_cwd,
        (*argv).pdata as *mut *mut gchar,
        envp as *mut *mut gchar,
        flags,
        None,
        NULL_0,
        &raw mut pid,
        ::core::ptr::null_mut::<gint>(),
        &raw mut stdout_fd,
        &raw mut stderr_fd,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_test_trap_subprocess() failed: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        loop {}
    }
    g_ptr_array_free(argv, TRUE);
    safe_c2rust_wait_for_child(
        pid,
        stdout_fd,
        (test_flags as ::core::ffi::c_uint
            & G_TEST_SUBPROCESS_INHERIT_STDOUT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0) as ::core::ffi::c_int,
        stderr_fd,
        (test_flags as ::core::ffi::c_uint
            & G_TEST_SUBPROCESS_INHERIT_STDERR as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0) as ::core::ffi::c_int,
        usec_timeout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_subprocess() -> gboolean {
    return safe_c2rust_test_in_subprocess;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_trap_has_passed() -> gboolean {
    return (safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (safe_c2rust_test_trap_last_status & 0xff00 as ::core::ffi::c_int)
            >> 8 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_trap_reached_timeout() -> gboolean {
    return (((safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as ::core::ffi::c_schar as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
        && safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int == SIGALRM)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_log_child_output(mut process_id: *const gchar) -> gboolean {
    let mut escaped: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (safe_c2rust_test_trap_last_status & 0xff00 as ::core::ffi::c_int)
            >> 8 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            safe_c2rust_g_test_message(
                b"child process (%s) exit status: 0 (success)\0" as *const u8
                    as *const ::core::ffi::c_char,
                process_id,
            );
        } else {
            safe_c2rust_g_test_message(
                b"child process (%s) exit status: %d (error)\0" as *const u8
                    as *const ::core::ffi::c_char,
                process_id,
                (safe_c2rust_test_trap_last_status & 0xff00 as ::core::ffi::c_int)
                    >> 8 as ::core::ffi::c_int,
            );
        }
    } else if ((safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as ::core::ffi::c_schar as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
        && safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int == SIGALRM
    {
        safe_c2rust_g_test_message(
            b"child process (%s) timed out\0" as *const u8 as *const ::core::ffi::c_char,
            process_id,
        );
    } else if ((safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as ::core::ffi::c_schar as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        let mut maybe_dumped_core: *const gchar = b"\0" as *const u8 as *const gchar;
        if safe_c2rust_test_trap_last_status & __WCOREFLAG != 0 {
            maybe_dumped_core =
                b", core dumped\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        }
        safe_c2rust_g_test_message(
            b"child process (%s) killed by signal %d (%s)%s\0" as *const u8
                as *const ::core::ffi::c_char,
            process_id,
            safe_c2rust_test_trap_last_status & 0x7f as ::core::ffi::c_int,
            g_strsignal(safe_c2rust_test_trap_last_status as gint & 0x7f as gint),
            maybe_dumped_core,
        );
    } else {
        safe_c2rust_g_test_message(
            b"child process (%s) unknown wait status %d\0" as *const u8
                as *const ::core::ffi::c_char,
            process_id,
            safe_c2rust_test_trap_last_status,
        );
    }
    escaped = g_strescape(
        safe_c2rust_test_trap_last_stdout,
        ::core::ptr::null::<gchar>(),
    );
    safe_c2rust_g_test_message(
        b"child process (%s) stdout: \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        process_id,
        escaped,
    );
    g_free(escaped as gpointer);
    escaped = g_strescape(
        safe_c2rust_test_trap_last_stderr,
        ::core::ptr::null::<gchar>(),
    );
    safe_c2rust_g_test_message(
        b"child process (%s) stderr: \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        process_id,
        escaped,
    );
    g_free(escaped as gpointer);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_trap_assertions(
    mut domain: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut assertion_flags: guint64,
    mut pattern: *const ::core::ffi::c_char,
) {
    let mut must_pass: gboolean = (assertion_flags == 0 as guint64) as ::core::ffi::c_int;
    let mut must_fail: gboolean = (assertion_flags == 1 as guint64) as ::core::ffi::c_int;
    let mut match_result: gboolean =
        (0 as guint64 == assertion_flags & 1 as guint64) as ::core::ffi::c_int;
    let mut logged_child_output: gboolean = FALSE;
    let mut stdout_pattern: *const ::core::ffi::c_char = if assertion_flags & 2 as guint64 != 0 {
        pattern
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
    let mut stderr_pattern: *const ::core::ffi::c_char = if assertion_flags & 4 as guint64 != 0 {
        pattern
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
    let mut match_error: *const ::core::ffi::c_char = if match_result != 0 {
        b"failed to match\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"contains invalid match\0" as *const u8 as *const ::core::ffi::c_char
    };
    let mut process_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !safe_c2rust_test_trap_last_subprocess.is_null() {
        process_id = g_strdup_printf(
            b"%s [%d]\0" as *const u8 as *const gchar,
            safe_c2rust_test_trap_last_subprocess,
            safe_c2rust_test_trap_last_pid,
        ) as *mut ::core::ffi::c_char;
    } else if safe_c2rust_test_trap_last_pid != 0 as ::core::ffi::c_int {
        process_id = g_strdup_printf(
            b"%d\0" as *const u8 as *const gchar,
            safe_c2rust_test_trap_last_pid,
        ) as *mut ::core::ffi::c_char;
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_test_trap_ assertion with no trapped test\0" as *const u8 as *const gchar,
        );
        loop {}
    }
    if must_pass != 0 && safe_c2rust_g_test_trap_has_passed() == 0 {
        let mut msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        logged_child_output = (logged_child_output != 0
            || safe_c2rust_log_child_output(process_id) != 0)
            as ::core::ffi::c_int as gboolean;
        msg = g_strdup_printf(
            b"child process (%s) failed unexpectedly\0" as *const u8 as *const gchar,
            process_id,
        ) as *mut ::core::ffi::c_char;
        safe_c2rust_g_assertion_message(domain, file, line, func, msg);
        g_free(msg as gpointer);
    }
    if must_fail != 0 && safe_c2rust_g_test_trap_has_passed() != 0 {
        let mut msg_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        logged_child_output = (logged_child_output != 0
            || safe_c2rust_log_child_output(process_id) != 0)
            as ::core::ffi::c_int as gboolean;
        msg_0 = g_strdup_printf(
            b"child process (%s) did not fail as expected\0" as *const u8 as *const gchar,
            process_id,
        ) as *mut ::core::ffi::c_char;
        safe_c2rust_g_assertion_message(domain, file, line, func, msg_0);
        g_free(msg_0 as gpointer);
    }
    if !stdout_pattern.is_null()
        && match_result
            == (g_pattern_match_simple(
                stdout_pattern as *const gchar,
                safe_c2rust_test_trap_last_stdout,
            ) == 0) as ::core::ffi::c_int
    {
        let mut msg_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        logged_child_output = (logged_child_output != 0
            || safe_c2rust_log_child_output(process_id) != 0)
            as ::core::ffi::c_int as gboolean;
        safe_c2rust_g_test_message(
            b"stdout was:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
            safe_c2rust_test_trap_last_stdout,
        );
        msg_1 = g_strdup_printf(
            b"stdout of child process (%s) %s: %s\0" as *const u8 as *const gchar,
            process_id,
            match_error,
            stdout_pattern,
        ) as *mut ::core::ffi::c_char;
        safe_c2rust_g_assertion_message(domain, file, line, func, msg_1);
        g_free(msg_1 as gpointer);
    }
    if !stderr_pattern.is_null()
        && match_result
            == (g_pattern_match_simple(
                stderr_pattern as *const gchar,
                safe_c2rust_test_trap_last_stderr,
            ) == 0) as ::core::ffi::c_int
    {
        let mut msg_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        logged_child_output = (logged_child_output != 0
            || safe_c2rust_log_child_output(process_id) != 0)
            as ::core::ffi::c_int as gboolean;
        safe_c2rust_g_test_message(
            b"stderr was:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
            safe_c2rust_test_trap_last_stderr,
        );
        msg_2 = g_strdup_printf(
            b"stderr of child process (%s) %s: %s\0" as *const u8 as *const gchar,
            process_id,
            match_error,
            stderr_pattern,
        ) as *mut ::core::ffi::c_char;
        safe_c2rust_g_assertion_message(domain, file, line, func, msg_2);
        g_free(msg_2 as gpointer);
    }
    g_free(process_id as gpointer);
}
unsafe extern "C" fn safe_c2rust_gstring_overwrite_int(
    mut gstring: *mut GString,
    mut pos: guint,
    mut vuint: guint32,
) {
    vuint = ({
        let mut __v: guint32 = 0;
        let mut __x: guint32 = vuint;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
        } else {
            let fresh8 = &mut __v;
            let fresh9;
            let fresh10 = __x;
            asm!(
                "bswapl {0:e}\n", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh8,
                fresh10) => fresh9, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
        }
        __v
    });
    g_string_overwrite_len(
        gstring,
        pos as gsize,
        &raw mut vuint as *const gchar,
        4 as gssize,
    );
}
unsafe extern "C" fn safe_c2rust_gstring_append_int(mut gstring: *mut GString, mut vuint: guint32) {
    vuint = ({
        let mut __v: guint32 = 0;
        let mut __x: guint32 = vuint;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
        } else {
            let fresh14 = &mut __v;
            let fresh15;
            let fresh16 = __x;
            asm!(
                "bswapl {0:e}\n", inlateout(reg)
                c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16) => fresh15,
                options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
        }
        __v
    });
    safe_c2rust_g_string_append_len_inline(
        gstring,
        &raw mut vuint as *const ::core::ffi::c_char,
        4 as gssize,
    );
}
unsafe extern "C" fn safe_c2rust_gstring_append_double(
    mut gstring: *mut GString,
    mut vdouble: ::core::ffi::c_double,
) {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { vdouble: 0. };
    u.vdouble = vdouble;
    u.vuint64 = ({
        let mut __v: guint64 = 0;
        let mut __x: guint64 = u.vuint64;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                | (__x & 0xff0000000000 as ::core::ffi::c_ulong) >> 24 as ::core::ffi::c_int
                | (__x & 0xff000000000000 as ::core::ffi::c_ulong) >> 40 as ::core::ffi::c_int
                | (__x & 0xff00000000000000 as ::core::ffi::c_ulong) >> 56 as ::core::ffi::c_int;
        } else {
            let fresh11 = &mut __v;
            let fresh12;
            let fresh13 = __x;
            asm!(
                "bswapq {0}\n", inlateout(reg)
                c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13) => fresh12,
                options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
        }
        __v
    });
    safe_c2rust_g_string_append_len_inline(
        gstring,
        &raw mut u.vuint64 as *const ::core::ffi::c_char,
        8 as gssize,
    );
}
unsafe extern "C" fn safe_c2rust_g_test_log_dump(
    mut msg: *mut GTestLogMsg,
    mut len: *mut guint,
) -> *mut guint8 {
    let mut gstring: *mut GString = g_string_sized_new(1024 as gsize);
    let mut ui: guint = 0;
    safe_c2rust_gstring_append_int(gstring, 0 as guint32);
    safe_c2rust_gstring_append_int(gstring, (*msg).log_type as guint32);
    safe_c2rust_gstring_append_int(gstring, (*msg).n_strings as guint32);
    safe_c2rust_gstring_append_int(gstring, (*msg).n_nums as guint32);
    safe_c2rust_gstring_append_int(gstring, 0 as guint32);
    ui = 0 as guint;
    while ui < (*msg).n_strings {
        let mut l: guint = strlen(*(*msg).strings.offset(ui as isize)) as guint;
        safe_c2rust_gstring_append_int(gstring, l as guint32);
        safe_c2rust_g_string_append_len_inline(
            gstring,
            *(*msg).strings.offset(ui as isize),
            l as gssize,
        );
        ui = ui.wrapping_add(1);
    }
    ui = 0 as guint;
    while ui < (*msg).n_nums {
        safe_c2rust_gstring_append_double(
            gstring,
            (*(*msg).nums.offset(ui as isize)).to_f64().unwrap(),
        );
        ui = ui.wrapping_add(1);
    }
    *len = (*gstring).len as guint;
    safe_c2rust_gstring_overwrite_int(gstring, 0 as guint, *len);
    return (if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(gstring, 0 as gboolean)
        } else {
            g_string_free_and_steal(gstring)
        }
    } else {
        g_string_free(gstring, 0 as gboolean)
    }) as *mut guint8;
}
#[inline]
unsafe extern "C" fn safe_c2rust_net_double(mut ipointer: *mut *const gchar) -> ::f128::f128 {
    let mut u: C2RustUnnamed = C2RustUnnamed { vuint64: 0 };
    let mut aligned_int64: guint64 = 0;
    memcpy(
        &raw mut aligned_int64 as *mut ::core::ffi::c_void,
        *ipointer as *const ::core::ffi::c_void,
        8 as size_t,
    );
    *ipointer = (*ipointer).offset(8 as ::core::ffi::c_int as isize);
    u.vuint64 = ({
        let mut __v: guint64 = 0;
        let mut __x: guint64 = aligned_int64;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                | (__x & 0xff0000000000 as ::core::ffi::c_ulong) >> 24 as ::core::ffi::c_int
                | (__x & 0xff000000000000 as ::core::ffi::c_ulong) >> 40 as ::core::ffi::c_int
                | (__x & 0xff00000000000000 as ::core::ffi::c_ulong) >> 56 as ::core::ffi::c_int;
        } else {
            let fresh2 = &mut __v;
            let fresh3;
            let fresh4 = __x;
            asm!(
                "bswapq {0}\n", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh2,
                fresh4) => fresh3, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
        }
        __v
    });
    return ::f128::f128::new(u.vdouble);
}
#[inline]
unsafe extern "C" fn safe_c2rust_net_int(mut ipointer: *mut *const gchar) -> guint32 {
    let mut aligned_int: guint32 = 0;
    memcpy(
        &raw mut aligned_int as *mut ::core::ffi::c_void,
        *ipointer as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ipointer = (*ipointer).offset(4 as ::core::ffi::c_int as isize);
    return ({
        let mut __v: guint32 = 0;
        let mut __x: guint32 = aligned_int;
        if 0 != 0 {
            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
        } else {
            let fresh5 = &mut __v;
            let fresh6;
            let fresh7 = __x;
            asm!(
                "bswapl {0:e}\n", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh5,
                fresh7) => fresh6, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
        }
        __v
    });
}
unsafe extern "C" fn safe_c2rust_g_test_log_extract(mut tbuffer: *mut GTestLogBuffer) -> gboolean {
    let mut p: *const gchar = (*(*tbuffer).data).str_0;
    let mut msg: GTestLogMsg = GTestLogMsg {
        log_type: G_TEST_LOG_NONE,
        n_strings: 0,
        strings: ::core::ptr::null_mut::<*mut gchar>(),
        n_nums: 0,
        nums: ::core::ptr::null_mut::<::f128::f128>(),
    };
    let mut mlength: guint = 0;
    if (*(*tbuffer).data).len < (4 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as gsize {
        return FALSE;
    }
    mlength = safe_c2rust_net_int(&raw mut p) as guint;
    if (*(*tbuffer).data).len < mlength as gsize {
        return FALSE;
    }
    msg.log_type = safe_c2rust_net_int(&raw mut p) as GTestLogType;
    msg.n_strings = safe_c2rust_net_int(&raw mut p) as guint;
    msg.n_nums = safe_c2rust_net_int(&raw mut p) as guint;
    if safe_c2rust_net_int(&raw mut p) == 0 as guint32 {
        let mut ui: guint = 0;
        msg.strings = ({
            let mut __n: gsize = msg.n_strings.wrapping_add(1 as guint) as gsize;
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
        msg.nums = ({
            let mut __n: gsize = msg.n_nums as gsize;
            let mut __s: gsize = ::core::mem::size_of::<::f128::f128>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ::f128::f128;
        ui = 0 as guint;
        while ui < msg.n_strings {
            let mut sl: guint = safe_c2rust_net_int(&raw mut p) as guint;
            let ref mut fresh1 = *msg.strings.offset(ui as isize);
            *fresh1 = g_strndup(p, sl as gsize);
            p = p.offset(sl as isize);
            ui = ui.wrapping_add(1);
        }
        ui = 0 as guint;
        while ui < msg.n_nums {
            *msg.nums.offset(ui as isize) = safe_c2rust_net_double(&raw mut p);
            ui = ui.wrapping_add(1);
        }
        if p <= (*(*tbuffer).data).str_0.offset(mlength as isize) as *const gchar {
            g_string_erase((*tbuffer).data, 0 as gssize, mlength as gssize);
            (*tbuffer).msgs = g_slist_prepend(
                (*tbuffer).msgs,
                g_memdup2(
                    &raw mut msg as gconstpointer,
                    ::core::mem::size_of::<GTestLogMsg>() as gsize,
                ),
            );
            return TRUE;
        }
        g_free(msg.nums as gpointer);
        g_strfreev(msg.strings);
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_ERROR,
        b"corrupt log stream from test program\0" as *const u8 as *const gchar,
    );
    loop {}
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log_buffer_new() -> *mut GTestLogBuffer {
    let mut tb: *mut GTestLogBuffer = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GTestLogBuffer>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GTestLogBuffer;
    (*tb).data = g_string_sized_new(1024 as gsize);
    return tb;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log_buffer_free(mut tbuffer: *mut GTestLogBuffer) {
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !tbuffer.is_null() {
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
            b"tbuffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    while !(*tbuffer).msgs.is_null() {
        safe_c2rust_g_test_log_msg_free(safe_c2rust_g_test_log_buffer_pop(tbuffer));
    }
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                (*tbuffer).data,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal((*tbuffer).data);
        };
    } else {
        g_string_free(
            (*tbuffer).data,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
    g_free(tbuffer as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log_buffer_push(
    mut tbuffer: *mut GTestLogBuffer,
    mut n_bytes: guint,
    mut bytes: *const guint8,
) {
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if !tbuffer.is_null() {
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
            b"tbuffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_bytes != 0 {
        let mut more_messages: gboolean = 0;
        if ({
            let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
            if !bytes.is_null() {
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
                b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_c2rust_g_string_append_len_inline(
            (*tbuffer).data,
            bytes as *const ::core::ffi::c_char,
            n_bytes as gssize,
        );
        loop {
            more_messages = safe_c2rust_g_test_log_extract(tbuffer);
            if !(more_messages != 0) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log_buffer_pop(
    mut tbuffer: *mut GTestLogBuffer,
) -> *mut GTestLogMsg {
    let mut msg: *mut GTestLogMsg = ::core::ptr::null_mut::<GTestLogMsg>();
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !tbuffer.is_null() {
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
            b"tbuffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTestLogMsg>();
    }
    if !(*tbuffer).msgs.is_null() {
        let mut slist: *mut GSList = g_slist_last((*tbuffer).msgs);
        msg = (*slist).data as *mut GTestLogMsg;
        (*tbuffer).msgs = g_slist_delete_link((*tbuffer).msgs, slist);
    }
    return msg;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_log_msg_free(mut tmsg: *mut GTestLogMsg) {
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !tmsg.is_null() {
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
            b"tmsg != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_strfreev((*tmsg).strings);
    g_free((*tmsg).nums as gpointer);
    g_free(tmsg as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_test_build_filename_va(
    mut file_type: GTestFileType,
    mut first_path: *const gchar,
    mut ap: ::core::ffi::VaList,
) -> *mut gchar {
    let mut pathv: [*const gchar; 16] = [::core::ptr::null::<gchar>(); 16];
    let mut num_path_segments: gsize = 0;
    if file_type as ::core::ffi::c_uint == G_TEST_DIST as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pathv[0 as ::core::ffi::c_int as usize] = safe_c2rust_test_disted_files_dir as *const gchar;
    } else if file_type as ::core::ffi::c_uint
        == G_TEST_BUILT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pathv[0 as ::core::ffi::c_int as usize] = safe_c2rust_test_built_files_dir as *const gchar;
    } else {
        safe_c2rust_g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            4480 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    pathv[1 as ::core::ffi::c_int as usize] = first_path;
    num_path_segments = 2 as gsize;
    while (num_path_segments as usize)
        < (::core::mem::size_of::<[*const gchar; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const gchar>() as usize)
    {
        pathv[num_path_segments as usize] = ap.arg::<*const ::core::ffi::c_char>() as *const gchar;
        if pathv[num_path_segments as usize].is_null() {
            break;
        }
        num_path_segments = num_path_segments.wrapping_add(1);
    }
    let mut __n1: gint64 = num_path_segments as gint64;
    let mut __n2: gint64 = (::core::mem::size_of::<[*const gchar; 16]>() as usize)
        .wrapping_div(::core::mem::size_of::<*const gchar>() as usize)
        as gint64;
    if !(__n1 < __n2) {
        safe_c2rust_g_assertion_message_cmpint(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            4491 as ::core::ffi::c_int,
            G_STRFUNC,
            b"num_path_segments < G_N_ELEMENTS (pathv)\0" as *const u8
                as *const ::core::ffi::c_char,
            __n1 as guint64,
            b"<\0" as *const u8 as *const ::core::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as ::core::ffi::c_char,
        );
    }
    return g_build_filenamev(&raw mut pathv as *mut *const gchar as *mut *mut gchar);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_build_filename(
    mut file_type: GTestFileType,
    mut first_path: *const gchar,
    mut args: ...
) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if (*safe_c2rust_g_test_config_vars).test_initialized != 0 {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            4561 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_test_initialized ()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    ap = args.clone();
    result = safe_c2rust_g_test_build_filename_va(file_type, first_path, ap.clone());
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_get_dir(mut file_type: GTestFileType) -> *const gchar {
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if (*safe_c2rust_g_test_config_vars).test_initialized != 0 {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            4587 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_test_initialized ()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if file_type as ::core::ffi::c_uint == G_TEST_DIST as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return safe_c2rust_test_disted_files_dir as *const gchar;
    } else if file_type as ::core::ffi::c_uint
        == G_TEST_BUILT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return safe_c2rust_test_built_files_dir as *const gchar;
    }
    safe_c2rust_g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
        4594 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_get_filename(
    mut file_type: GTestFileType,
    mut first_path: *const gchar,
    mut args: ...
) -> *const gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if (*safe_c2rust_g_test_config_vars).test_initialized != 0 {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        safe_c2rust_g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtestutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            4629 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_test_initialized ()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if safe_c2rust_test_filename_free_list.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_test_get_filename() can only be used within testcase functions\0" as *const u8
                as *const gchar,
        );
        loop {}
    }
    ap = args.clone();
    result = safe_c2rust_g_test_build_filename_va(file_type, first_path, ap.clone());
    node = g_slist_prepend(::core::ptr::null_mut::<GSList>(), result as gpointer);
    loop {
        (*node).next = *safe_c2rust_test_filename_free_list;
        if !(({
            let mut gapcae_oldval: gpointer = (*node).next as gpointer;
            if 0 as ::core::ffi::c_int != 0 {
                *safe_c2rust_test_filename_free_list;
            } else {
            };
            let fresh65 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                safe_c2rust_test_filename_free_list,
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut GSList),
                node,
            );
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut GSList) = fresh65.0;
            if fresh65.1 as ::core::ffi::c_int != 0 {
                TRUE
            } else {
                FALSE
            }
        }) == 0)
        {
            break;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_get_path() -> *const ::core::ffi::c_char {
    return safe_c2rust_test_run_name;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
