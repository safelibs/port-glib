extern "C" {
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_ascii_toupper(c: gchar) -> gchar;
    fn g_strcanon(string: *mut gchar, valid_chars: *const gchar, substitutor: gchar) -> *mut gchar;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_ascii_strdown(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strjoinv(separator: *const gchar, str_array: *mut *mut gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_strv_equal(strv1: *const *const gchar, strv2: *const *const gchar) -> gboolean;
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
    fn g_string_printf(string: *mut GString, format: *const gchar, ...);
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    fn getpwnam_r(
        __name: *const ::core::ffi::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn getuid() -> __uid_t;
    fn gethostname(__name: *mut ::core::ffi::c_char, __len: size_t) -> ::core::ffi::c_int;
    fn getauxval(__type: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_error_quark() -> GQuark;
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_get_current_dir() -> *mut gchar;
    fn g_dngettext(
        domain: *const gchar,
        msgid: *const gchar,
        msgid_plural: *const gchar,
        n: gulong,
    ) -> *const gchar;
    fn g_shell_unquote(quoted_string: *const gchar, error: *mut *mut GError) -> *mut gchar;
    fn __lsan_enable();
    fn __lsan_ignore_object(p: *const ::core::ffi::c_void);
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn glib_pgettext(msgctxtid: *const gchar, msgidoffset: gsize) -> *const gchar;
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
pub type goffset = gint64;
pub type guintptr = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UserDatabaseEntry {
    pub user_name: *mut gchar,
    pub real_name: *mut gchar,
    pub home_dir: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
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
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [::core::ffi::c_char; 65],
    pub nodename: [::core::ffi::c_char; 65],
    pub release: [::core::ffi::c_char; 65],
    pub version: [::core::ffi::c_char; 65],
    pub machine: [::core::ffi::c_char; 65],
    pub domainname: [::core::ffi::c_char; 65],
}
pub type GStrv = *mut *mut gchar;
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GQuark = guint32;
pub const G_FILE_ERROR_NOENT: C2RustUnnamed_0 = 4;
pub const G_USER_N_DIRECTORIES: GUserDirectory = 8;
pub type GUserDirectory = ::core::ffi::c_uint;
pub const G_USER_DIRECTORY_VIDEOS: GUserDirectory = 7;
pub const G_USER_DIRECTORY_TEMPLATES: GUserDirectory = 6;
pub const G_USER_DIRECTORY_PUBLIC_SHARE: GUserDirectory = 5;
pub const G_USER_DIRECTORY_PICTURES: GUserDirectory = 4;
pub const G_USER_DIRECTORY_MUSIC: GUserDirectory = 3;
pub const G_USER_DIRECTORY_DOWNLOAD: GUserDirectory = 2;
pub const G_USER_DIRECTORY_DOCUMENTS: GUserDirectory = 1;
pub const G_USER_DIRECTORY_DESKTOP: GUserDirectory = 0;
pub type GFormatSizeFlags = ::core::ffi::c_uint;
pub const G_FORMAT_SIZE_ONLY_UNIT: GFormatSizeFlags = 16;
pub const G_FORMAT_SIZE_ONLY_VALUE: GFormatSizeFlags = 8;
pub const G_FORMAT_SIZE_BITS: GFormatSizeFlags = 4;
pub const G_FORMAT_SIZE_IEC_UNITS: GFormatSizeFlags = 2;
pub const G_FORMAT_SIZE_LONG_FORMAT: GFormatSizeFlags = 1;
pub const G_FORMAT_SIZE_DEFAULT: GFormatSizeFlags = 0;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub const FORMAT_BYTES_IEC: FormatIndex = 1;
pub type FormatIndex = ::core::ffi::c_uint;
pub const FORMAT_BITS_IEC: FormatIndex = 3;
pub const FORMAT_BITS: FormatIndex = 2;
pub const FORMAT_BYTES: FormatIndex = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Format {
    pub factor: guint64,
    pub string: [::core::ffi::c_char; 10],
}
pub type GVoidFunc = Option<unsafe extern "C" fn() -> ()>;
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_FILE_ERROR_FAILED: C2RustUnnamed_0 = 24;
pub const G_FILE_ERROR_NOSYS: C2RustUnnamed_0 = 23;
pub const G_FILE_ERROR_PERM: C2RustUnnamed_0 = 22;
pub const G_FILE_ERROR_IO: C2RustUnnamed_0 = 21;
pub const G_FILE_ERROR_INTR: C2RustUnnamed_0 = 20;
pub const G_FILE_ERROR_AGAIN: C2RustUnnamed_0 = 19;
pub const G_FILE_ERROR_PIPE: C2RustUnnamed_0 = 18;
pub const G_FILE_ERROR_INVAL: C2RustUnnamed_0 = 17;
pub const G_FILE_ERROR_BADF: C2RustUnnamed_0 = 16;
pub const G_FILE_ERROR_NFILE: C2RustUnnamed_0 = 15;
pub const G_FILE_ERROR_MFILE: C2RustUnnamed_0 = 14;
pub const G_FILE_ERROR_NOMEM: C2RustUnnamed_0 = 13;
pub const G_FILE_ERROR_NOSPC: C2RustUnnamed_0 = 12;
pub const G_FILE_ERROR_LOOP: C2RustUnnamed_0 = 11;
pub const G_FILE_ERROR_FAULT: C2RustUnnamed_0 = 10;
pub const G_FILE_ERROR_TXTBSY: C2RustUnnamed_0 = 9;
pub const G_FILE_ERROR_ROFS: C2RustUnnamed_0 = 8;
pub const G_FILE_ERROR_NODEV: C2RustUnnamed_0 = 7;
pub const G_FILE_ERROR_NXIO: C2RustUnnamed_0 = 6;
pub const G_FILE_ERROR_NOTDIR: C2RustUnnamed_0 = 5;
pub const G_FILE_ERROR_NAMETOOLONG: C2RustUnnamed_0 = 3;
pub const G_FILE_ERROR_ACCES: C2RustUnnamed_0 = 2;
pub const G_FILE_ERROR_ISDIR: C2RustUnnamed_0 = 1;
pub const G_FILE_ERROR_EXIST: C2RustUnnamed_0 = 0;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_set_application_name\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const HOST_NAME_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const G_MAXLONG: ::core::ffi::c_long = LONG_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const GLIB_SIZEOF_LONG: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXSSIZE: ::core::ffi::c_long = G_MAXLONG;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
pub const G_SEARCHPATH_SEPARATOR: ::core::ffi::c_int = ':' as i32;
pub const G_SEARCHPATH_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b":\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_bit_nth_lsf_impl(mut mask: gulong, mut nth_bit: gint) -> gint {
    if ({
        let mut _g_boolean_var_0: ::core::ffi::c_int = 0;
        if nth_bit < -(1 as ::core::ffi::c_int) {
            _g_boolean_var_0 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_0 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_0
    }) as ::core::ffi::c_long
        != 0
    {
        nth_bit = -(1 as ::core::ffi::c_int) as gint;
    }
    while nth_bit < GLIB_SIZEOF_LONG * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        nth_bit += 1;
        if mask as ::core::ffi::c_ulong & (1 as ::core::ffi::c_ulong) << nth_bit != 0 {
            return nth_bit;
        }
    }
    return -(1 as gint);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_bit_nth_msf_impl(mut mask: gulong, mut nth_bit: gint) -> gint {
    if nth_bit < 0 as ::core::ffi::c_int
        || ({
            let mut _g_boolean_var_1: ::core::ffi::c_int = 0;
            if nth_bit > 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int {
                _g_boolean_var_1 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_1 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_1
        }) as ::core::ffi::c_long
            != 0
    {
        nth_bit = (GLIB_SIZEOF_LONG * 8 as ::core::ffi::c_int) as gint;
    }
    while nth_bit > 0 as ::core::ffi::c_int {
        nth_bit -= 1;
        if mask as ::core::ffi::c_ulong & (1 as ::core::ffi::c_ulong) << nth_bit != 0 {
            return nth_bit;
        }
    }
    return -(1 as gint);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_bit_storage_impl(mut number: gulong) -> guint {
    return if ({
        let mut _g_boolean_var_2: ::core::ffi::c_int = 0;
        if number != 0 {
            _g_boolean_var_2 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_2 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_2
    }) as ::core::ffi::c_long
        != 0
    {
        ((GLIB_SIZEOF_LONG as guint)
            .wrapping_mul(8 as guint)
            .wrapping_sub(1 as guint)
            ^ number.leading_zeros() as i32 as guint)
            .wrapping_add(1 as guint)
    } else {
        1 as guint
    };
}
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
pub const AT_SECURE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_leak_sanitizer_is_supported() -> gboolean {
    return (Some(__lsan_enable as unsafe extern "C" fn() -> ()).is_some()
        && Some(__lsan_ignore_object as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ())
            .is_some()) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_ignore_leak(mut p: gconstpointer) {
    if !p.is_null()
        && Some(__lsan_ignore_object as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ())
            .is_some()
    {
        __lsan_ignore_object(p as *const ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_ignore_strv_leak(mut strv: GStrv) {
    let mut item: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if safe_c2rust_g_leak_sanitizer_is_supported() == 0 {
        return;
    }
    if !strv.is_null() {
        safe_c2rust_g_ignore_leak(strv as gconstpointer);
        item = strv as *mut *mut gchar;
        while !(*item).is_null() {
            safe_c2rust_g_ignore_leak(*item as gconstpointer);
            item = item.offset(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atexit(mut func: GVoidFunc) {
    let mut result: gint = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    result = atexit(func) as gint;
    errsv = *__errno_location();
    if result != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Could not register atexit() function: %s\0" as *const u8 as *const gchar,
            g_strerror(errsv as gint),
        );
        loop {}
    }
}
unsafe extern "C" fn safe_c2rust_my_strchrnul(mut str: *const gchar, mut c: gchar) -> *mut gchar {
    let mut p: *mut gchar = str as *mut gchar;
    while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != c as ::core::ffi::c_int {
        p = p.offset(1);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_find_program_in_path(
    mut program: *const gchar,
) -> *mut gchar {
    return safe_c2rust_g_find_program_for_path(
        program as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_find_program_for_path(
    mut program: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut working_dir: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut original_path: *const ::core::ffi::c_char = path;
    let mut original_program: *const ::core::ffi::c_char = program;
    let mut program_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut freeme: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0;
    let mut pathlen: gsize = 0;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !program.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"program != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !working_dir.is_null() && g_path_is_absolute(program as *const gchar) == 0 {
        program_path = g_build_filename(working_dir as *const gchar, program, NULL_0)
            as *mut ::core::ffi::c_char;
        program = program_path;
    }
    if g_path_is_absolute(program as *const gchar) != 0
        || !strchr(original_program, G_DIR_SEPARATOR).is_null()
    {
        if g_file_test(program as *const gchar, G_FILE_TEST_IS_EXECUTABLE) != 0
            && g_file_test(program as *const gchar, G_FILE_TEST_IS_DIR) == 0
        {
            let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
            if g_path_is_absolute(program as *const gchar) != 0 {
                out = safe_c2rust_g_strdup_inline(program) as *mut gchar;
            } else {
                let mut cwd: *mut ::core::ffi::c_char =
                    g_get_current_dir() as *mut ::core::ffi::c_char;
                out = g_build_filename(cwd, program, NULL_0);
                g_free(cwd as gpointer);
            }
            g_free(program_path as gpointer);
            return safe_c2rust_g_steal_pointer(&raw mut out as gpointer)
                as *mut ::core::ffi::c_char;
        } else {
            let mut _pp: *mut *mut ::core::ffi::c_char = &raw mut program_path;
            let mut _ptr: *mut ::core::ffi::c_char = *_pp;
            *_pp = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !_ptr.is_null() {
                g_free(_ptr as gpointer);
            }
            if g_path_is_absolute(original_program as *const gchar) != 0 {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
    }
    program = original_program;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if original_path.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        path = g_getenv(b"PATH\0" as *const u8 as *const gchar) as *const ::core::ffi::c_char;
    } else {
        path = original_path;
    }
    if path.is_null() {
        path = b"/bin:/usr/bin:.\0" as *const u8 as *const ::core::ffi::c_char;
    }
    len = strlen(program).wrapping_add(1 as size_t) as gsize;
    pathlen = strlen(path) as gsize;
    name = g_malloc(pathlen.wrapping_add(len).wrapping_add(1 as gsize)) as *mut gchar;
    freeme = name;
    memcpy(
        name.offset(pathlen as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        program as *const ::core::ffi::c_void,
        len as size_t,
    );
    name = name.offset(pathlen as isize);
    *name = G_DIR_SEPARATOR as gchar;
    p = path as *const gchar;
    loop {
        let mut startp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut startp_path: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        path = p as *const ::core::ffi::c_char;
        p = safe_c2rust_my_strchrnul(path as *const gchar, G_SEARCHPATH_SEPARATOR as gchar);
        if p == path {
            startp = name.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
        } else {
            startp = memcpy(
                name.offset(-(p.offset_from(path) as ::core::ffi::c_long as isize))
                    as *mut ::core::ffi::c_void,
                path as *const ::core::ffi::c_void,
                p.offset_from(path) as ::core::ffi::c_long as size_t,
            ) as *mut ::core::ffi::c_char;
        }
        if !working_dir.is_null() && g_path_is_absolute(startp) == 0 {
            startp_path = g_build_filename(working_dir as *const gchar, startp, NULL_0)
                as *mut ::core::ffi::c_char;
            startp = startp_path;
        }
        if g_file_test(startp, G_FILE_TEST_IS_EXECUTABLE) != 0
            && g_file_test(startp, G_FILE_TEST_IS_DIR) == 0
        {
            let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
            if g_path_is_absolute(startp) != 0 {
                ret = safe_c2rust_g_strdup_inline(startp) as *mut gchar;
            } else {
                let mut cwd_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
                cwd_0 = g_get_current_dir();
                ret = g_build_filename(cwd_0, startp, NULL_0);
                g_free(cwd_0 as gpointer);
            }
            g_free(program_path as gpointer);
            g_free(startp_path as gpointer);
            g_free(freeme as gpointer);
            return ret as *mut ::core::ffi::c_char;
        }
        g_free(startp_path as gpointer);
        let fresh6 = p;
        p = p.offset(1);
        if !(*fresh6 as ::core::ffi::c_int != '\0' as i32) {
            break;
        }
    }
    g_free(program_path as gpointer);
    g_free(freeme as gpointer);
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bit_nth_lsf(mut mask: gulong, mut nth_bit: gint) -> gint {
    return safe_c2rust_g_bit_nth_lsf_impl(mask, nth_bit);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bit_nth_msf(mut mask: gulong, mut nth_bit: gint) -> gint {
    return safe_c2rust_g_bit_nth_msf_impl(mask, nth_bit);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bit_storage(mut number: gulong) -> guint {
    return safe_c2rust_g_bit_storage_impl(number);
}
static mut safe_c2rust_g__g_utils_global_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_g_user_data_dir: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_g_system_data_dirs: *mut *mut gchar =
    ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
static mut safe_c2rust_g_user_cache_dir: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_g_user_config_dir: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_g_user_state_dir: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_g_user_runtime_dir: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
static mut safe_c2rust_g_system_config_dirs: *mut *mut gchar =
    ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
static mut safe_c2rust_g_user_special_dirs: *mut *mut gchar =
    ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
static mut safe_c2rust_g_tmp_dir: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
unsafe extern "C" fn safe_c2rust_g_get_user_database_entry() -> *mut UserDatabaseEntry {
    static mut safe_c2rust_entry: *mut UserDatabaseEntry =
        ::core::ptr::null::<UserDatabaseEntry>() as *mut UserDatabaseEntry;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_entry;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut UserDatabaseEntry =
                ::core::ptr::null_mut::<UserDatabaseEntry>();
            let mut gapg_temp_atomic: *mut *mut UserDatabaseEntry = &raw mut safe_c2rust_entry;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut safe_c2rust_entry as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        static mut safe_c2rust_e: UserDatabaseEntry = UserDatabaseEntry {
            user_name: ::core::ptr::null::<gchar>() as *mut gchar,
            real_name: ::core::ptr::null::<gchar>() as *mut gchar,
            home_dir: ::core::ptr::null::<gchar>() as *mut gchar,
        };
        let mut pw: *mut passwd = ::core::ptr::null_mut::<passwd>();
        let mut buffer: gpointer = NULL_0;
        let mut error: gint = 0;
        let mut logname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut pwd: passwd = passwd {
            pw_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_uid: 0,
            pw_gid: 0,
            pw_gecos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_shell: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let mut bufsize: glong = sysconf(_SC_GETPW_R_SIZE_MAX as ::core::ffi::c_int) as glong;
        if bufsize < 0 as glong {
            bufsize = 64 as glong;
        }
        logname = g_getenv(b"LOGNAME\0" as *const u8 as *const gchar) as *const ::core::ffi::c_char;
        loop {
            g_free(buffer);
            buffer = g_malloc((bufsize + 6 as glong) as gsize);
            *__errno_location() = 0 as ::core::ffi::c_int;
            if !logname.is_null() {
                error = getpwnam_r(
                    logname,
                    &raw mut pwd,
                    buffer as *mut ::core::ffi::c_char,
                    bufsize as size_t,
                    &raw mut pw,
                ) as gint;
                if pw.is_null() || (*pw).pw_uid != getuid() {
                    error = getpwuid_r(
                        getuid(),
                        &raw mut pwd,
                        buffer as *mut ::core::ffi::c_char,
                        bufsize as size_t,
                        &raw mut pw,
                    ) as gint;
                }
            } else {
                error = getpwuid_r(
                    getuid(),
                    &raw mut pwd,
                    buffer as *mut ::core::ffi::c_char,
                    bufsize as size_t,
                    &raw mut pw,
                ) as gint;
            }
            error = (if error < 0 as ::core::ffi::c_int {
                *__errno_location()
            } else {
                error as ::core::ffi::c_int
            }) as gint;
            if pw.is_null() {
                if error == 0 as ::core::ffi::c_int || error == ENOENT {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"getpwuid_r(): failed due to unknown user id (%lu)\0" as *const u8
                            as *const gchar,
                        getuid() as gulong,
                    );
                    break;
                } else if bufsize > (32 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as glong
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"getpwuid_r(): failed due to: %s.\0" as *const u8 as *const gchar,
                        g_strerror(error),
                    );
                    break;
                } else {
                    bufsize *= 2 as glong;
                }
            }
            if !pw.is_null() {
                break;
            }
        }
        if pw.is_null() {
            pw = getpwuid(getuid());
        }
        if !pw.is_null() {
            safe_c2rust_e.user_name = safe_c2rust_g_strdup_inline((*pw).pw_name) as *mut gchar;
            if !(*pw).pw_gecos.is_null()
                && *(*pw).pw_gecos as ::core::ffi::c_int != '\0' as i32
                && !(*pw).pw_name.is_null()
            {
                let mut gecos_fields: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
                let mut name_parts: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
                let mut uppercase_pw_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
                gecos_fields = g_strsplit(
                    (*pw).pw_gecos,
                    b",\0" as *const u8 as *const gchar,
                    0 as gint,
                );
                name_parts = g_strsplit(
                    *gecos_fields.offset(0 as ::core::ffi::c_int as isize),
                    b"&\0" as *const u8 as *const gchar,
                    0 as gint,
                );
                uppercase_pw_name = safe_c2rust_g_strdup_inline((*pw).pw_name) as *mut gchar;
                *uppercase_pw_name.offset(0 as ::core::ffi::c_int as isize) =
                    g_ascii_toupper(*uppercase_pw_name.offset(0 as ::core::ffi::c_int as isize));
                safe_c2rust_e.real_name = g_strjoinv(uppercase_pw_name, name_parts);
                g_strfreev(gecos_fields);
                g_strfreev(name_parts);
                g_free(uppercase_pw_name as gpointer);
            }
            if safe_c2rust_e.home_dir.is_null() {
                safe_c2rust_e.home_dir = safe_c2rust_g_strdup_inline((*pw).pw_dir) as *mut gchar;
            }
        }
        g_free(buffer);
        if safe_c2rust_e.user_name.is_null() {
            safe_c2rust_e.user_name = safe_c2rust_g_strdup_inline(
                b"somebody\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
        if safe_c2rust_e.real_name.is_null() {
            safe_c2rust_e.real_name = safe_c2rust_g_strdup_inline(
                b"Unknown\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_entry = &raw mut safe_c2rust_e;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_entry as *mut ::core::ffi::c_void,
            &raw mut safe_c2rust_e as guintptr as gpointer,
        );
    }
    return safe_c2rust_entry;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_user_name() -> *const gchar {
    let mut entry: *mut UserDatabaseEntry = ::core::ptr::null_mut::<UserDatabaseEntry>();
    entry = safe_c2rust_g_get_user_database_entry();
    return (*entry).user_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_real_name() -> *const gchar {
    let mut entry: *mut UserDatabaseEntry = ::core::ptr::null_mut::<UserDatabaseEntry>();
    entry = safe_c2rust_g_get_user_database_entry();
    return (*entry).real_name;
}
static mut safe_c2rust_g_home_dir: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
unsafe extern "C" fn safe_c2rust_g_build_home_dir() -> *mut gchar {
    let mut home_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    home_dir = safe_c2rust_g_strdup_inline(
        g_getenv(b"HOME\0" as *const u8 as *const gchar) as *const ::core::ffi::c_char
    ) as *mut gchar;
    if home_dir.is_null() {
        let mut entry: *mut UserDatabaseEntry = safe_c2rust_g_get_user_database_entry();
        home_dir = safe_c2rust_g_strdup_inline((*entry).home_dir) as *mut gchar;
    }
    if home_dir.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Could not find home directory: $HOME is not set, and user database could not be read.\0"
                as *const u8 as *const gchar,
        );
        home_dir = safe_c2rust_g_strdup_inline(b"/\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    return safe_c2rust_g_steal_pointer(&raw mut home_dir as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_home_dir() -> *const gchar {
    let mut home_dir: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_home_dir.is_null() {
        safe_c2rust_g_home_dir = safe_c2rust_g_build_home_dir();
    }
    home_dir = safe_c2rust_g_home_dir;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return home_dir;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unset_cached_tmp_dir() {
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    safe_c2rust_g_ignore_leak(safe_c2rust_g_tmp_dir as gconstpointer);
    safe_c2rust_g_tmp_dir = ::core::ptr::null_mut::<gchar>();
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_tmp_dir() -> *const gchar {
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_tmp_dir.is_null() {
        let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        tmp =
            safe_c2rust_g_strdup_inline(g_getenv(b"G_TEST_TMPDIR\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char) as *mut gchar;
        if tmp.is_null() || *tmp as ::core::ffi::c_int == '\0' as i32 {
            g_free(tmp as gpointer);
            tmp =
                safe_c2rust_g_strdup_inline(g_getenv(b"TMPDIR\0" as *const u8 as *const gchar)
                    as *const ::core::ffi::c_char) as *mut gchar;
        }
        if tmp.is_null() || *tmp as ::core::ffi::c_int == '\0' as i32 {
            let mut k: gsize = 0;
            g_free(tmp as gpointer);
            tmp = safe_c2rust_g_strdup_inline(b"/tmp\0" as *const u8 as *const ::core::ffi::c_char)
                as *mut gchar;
            k = strlen(tmp) as gsize;
            if k > 1 as gsize
                && *tmp.offset(k.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
                    == G_DIR_SEPARATOR
            {
                *tmp.offset(k.wrapping_sub(1 as gsize) as isize) = '\0' as i32 as gchar;
            }
        }
        if tmp.is_null() || *tmp as ::core::ffi::c_int == '\0' as i32 {
            g_free(tmp as gpointer);
            tmp = safe_c2rust_g_strdup_inline(b"/tmp\0" as *const u8 as *const ::core::ffi::c_char)
                as *mut gchar;
        }
        safe_c2rust_g_tmp_dir =
            safe_c2rust_g_steal_pointer(&raw mut tmp as gpointer) as *mut gchar as *mut gchar;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return safe_c2rust_g_tmp_dir;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_host_name() -> *const gchar {
    static mut safe_c2rust_hostname: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_hostname;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut gapg_temp_atomic: *mut *mut gchar = &raw mut safe_c2rust_hostname;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut safe_c2rust_hostname as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut failed: gboolean = 0;
        let mut utmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut size: gsize = 0;
        let size_large: gsize = (256 as ::core::ffi::c_int as gsize).wrapping_mul(256 as gsize);
        let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut max: glong = 0;
        max = sysconf(_SC_HOST_NAME_MAX as ::core::ffi::c_int) as glong;
        if max > 0 as glong && max as gsize <= G_MAXSIZE.wrapping_sub(1 as ::core::ffi::c_ulong) {
            size = (max as gsize).wrapping_add(1 as gsize);
        } else {
            size = (HOST_NAME_MAX + 1 as ::core::ffi::c_int) as gsize;
        }
        tmp = g_malloc(size) as *mut gchar;
        failed = (gethostname(tmp as *mut ::core::ffi::c_char, size as size_t)
            == -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int as gboolean;
        if failed != 0 && size < size_large {
            g_free(tmp as gpointer);
            tmp = g_malloc(size_large) as *mut gchar;
            failed = (gethostname(tmp as *mut ::core::ffi::c_char, size_large as size_t)
                == -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int
                as gboolean;
        }
        if failed != 0 {
            let mut _pp: *mut *mut gchar = &raw mut tmp;
            let mut _ptr: *mut gchar = *_pp;
            *_pp = ::core::ptr::null_mut::<gchar>();
            if !_ptr.is_null() {
                g_free(_ptr as gpointer);
            }
        }
        utmp = tmp;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_hostname = (if failed != 0 {
                safe_c2rust_g_strdup_inline(
                    b"localhost\0" as *const u8 as *const ::core::ffi::c_char,
                )
            } else {
                utmp as *mut ::core::ffi::c_char
            }) as *mut gchar;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_hostname as *mut ::core::ffi::c_void,
            (if failed != 0 {
                safe_c2rust_g_strdup_inline(
                    b"localhost\0" as *const u8 as *const ::core::ffi::c_char,
                )
            } else {
                utmp as *mut ::core::ffi::c_char
            }) as guintptr as gpointer,
        );
    }
    return safe_c2rust_hostname;
}
static mut safe_c2rust_g_prgname: *const gchar = ::core::ptr::null::<gchar>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_prgname() -> *const gchar {
    return ({
        let mut gapg_temp_newval: *const gchar = ::core::ptr::null::<gchar>();
        let mut gapg_temp_atomic: *mut *const gchar = &raw mut safe_c2rust_g_prgname;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_prgname(mut prgname: *const gchar) {
    prgname = g_intern_string(prgname);
    let mut gaps_temp_atomic: *mut *const gchar = &raw mut safe_c2rust_g_prgname;
    let mut gaps_temp_newval: *const gchar = prgname as *const gchar;
    if 0 as ::core::ffi::c_int != 0 {
        safe_c2rust_g_prgname;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_prgname_once(mut prgname: *const gchar) -> gboolean {
    prgname = g_intern_string(prgname);
    return ({
        let mut gapcae_oldval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_prgname;
        } else {
        };
        let fresh7 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            &raw mut safe_c2rust_g_prgname,
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *const gchar),
            prgname,
        );
        *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *const gchar) = fresh7.0;
        if fresh7.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    });
}
static mut safe_c2rust_g_application_name: *mut gchar = ::core::ptr::null::<gchar>() as *mut gchar;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_application_name() -> *const gchar {
    let mut retval: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    retval = ({
        let mut gapg_temp_newval: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut gapg_temp_atomic: *mut *mut gchar = &raw mut safe_c2rust_g_application_name;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *const ::core::ffi::c_char;
    if !retval.is_null() {
        return retval as *const gchar;
    }
    return safe_c2rust_g_get_prgname();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_application_name(mut application_name: *const gchar) {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !application_name.is_null() {
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
            b"application_name\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    name = safe_c2rust_g_strdup_inline(application_name as *const ::core::ffi::c_char);
    if ({
        let mut gapcae_oldval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_application_name;
        } else {
        };
        let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            &raw mut safe_c2rust_g_application_name,
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut gchar),
            name,
        );
        *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut gchar) = fresh0.0;
        if fresh0.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    }) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"g_set_application_name() called multiple times\0" as *const u8 as *const gchar,
        );
        g_free(name as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_get_os_info_from_os_release(
    mut key_name: *const gchar,
    mut buffer: *const gchar,
) -> *mut gchar {
    let mut lines: GStrv = ::core::ptr::null_mut::<*mut gchar>();
    let mut prefix: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: size_t = 0;
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    lines = g_strsplit(buffer, b"\n\0" as *const u8 as *const gchar, -(1 as gint)) as GStrv;
    prefix = g_strdup_printf(b"%s=\0" as *const u8 as *const gchar, key_name);
    i = 0 as size_t;
    while !(*lines.offset(i as isize)).is_null() {
        let mut line: *const gchar = *lines.offset(i as isize);
        let mut value: *const gchar = ::core::ptr::null::<gchar>();
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = line as *const ::core::ffi::c_char;
                let __prefix: *const ::core::ffi::c_char = prefix;
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
            g_str_has_prefix(line, prefix)
        } != 0
        {
            value = line.offset(strlen(prefix) as isize);
            result = g_shell_unquote(value, ::core::ptr::null_mut::<*mut GError>());
            if result.is_null() {
                result =
                    safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as *mut gchar;
            }
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    g_strfreev(lines as *mut *mut gchar);
    g_free(prefix as gpointer);
    if result.is_null() {
        if strcmp(
            key_name as *const ::core::ffi::c_char,
            b"NAME\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return safe_c2rust_g_strdup_inline(
                b"Linux\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
        if strcmp(
            key_name as *const ::core::ffi::c_char,
            b"ID\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return safe_c2rust_g_strdup_inline(
                b"linux\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
        if strcmp(
            key_name as *const ::core::ffi::c_char,
            b"PRETTY_NAME\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return safe_c2rust_g_strdup_inline(
                b"Linux\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
    }
    return safe_c2rust_g_steal_pointer(&raw mut result as gpointer) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_get_os_info_from_uname(mut key_name: *const gchar) -> *mut gchar {
    let mut info: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    if uname(&raw mut info) == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<gchar>();
    }
    if strcmp(
        key_name as *const ::core::ffi::c_char,
        b"NAME\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_g_strdup_inline(&raw mut info.sysname as *mut ::core::ffi::c_char)
            as *mut gchar;
    } else if strcmp(
        key_name as *const ::core::ffi::c_char,
        b"VERSION\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_g_strdup_inline(&raw mut info.release as *mut ::core::ffi::c_char)
            as *mut gchar;
    } else if strcmp(
        key_name as *const ::core::ffi::c_char,
        b"PRETTY_NAME\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return g_strdup_printf(
            b"%s %s\0" as *const u8 as *const gchar,
            &raw mut info.sysname as *mut ::core::ffi::c_char,
            &raw mut info.release as *mut ::core::ffi::c_char,
        );
    } else if strcmp(
        key_name as *const ::core::ffi::c_char,
        b"ID\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut result: *mut gchar = g_ascii_strdown(
            &raw mut info.sysname as *mut ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
        g_strcanon(
            result,
            b"abcdefghijklmnopqrstuvwxyz0123456789_-.\0" as *const u8 as *const gchar,
            '_' as i32 as gchar,
        );
        return safe_c2rust_g_steal_pointer(&raw mut result as gpointer) as *mut gchar;
    } else if strcmp(
        key_name as *const ::core::ffi::c_char,
        b"VERSION_ID\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut result_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if strcmp(
            &raw mut info.sysname as *mut ::core::ffi::c_char,
            b"NetBSD\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut len: gssize = G_MAXSSIZE;
            let mut c: *const gchar = ::core::ptr::null::<gchar>();
            c = strchr(
                &raw mut info.release as *mut ::core::ffi::c_char,
                '-' as i32,
            );
            if !c.is_null() {
                len = (if len
                    < c.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                {
                    len as ::core::ffi::c_long
                } else {
                    c.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                }) as gssize;
            }
            c = strchr(
                &raw mut info.release as *mut ::core::ffi::c_char,
                '_' as i32,
            );
            if !c.is_null() {
                len = (if len
                    < c.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                {
                    len as ::core::ffi::c_long
                } else {
                    c.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                }) as gssize;
            }
            if len == G_MAXSSIZE {
                len = -(1 as ::core::ffi::c_int) as gssize;
            }
            result_0 = g_ascii_strdown(&raw mut info.release as *mut ::core::ffi::c_char, len);
        } else if strcmp(
            &raw mut info.sysname as *mut ::core::ffi::c_char,
            b"GNU\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut len_0: gssize = -(1 as ::core::ffi::c_int) as gssize;
            let mut c_0: *const gchar = strchr(
                &raw mut info.release as *mut ::core::ffi::c_char,
                '/' as i32,
            );
            if !c_0.is_null() {
                len_0 = c_0.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as gssize;
            }
            result_0 = g_ascii_strdown(&raw mut info.release as *mut ::core::ffi::c_char, len_0);
        } else if (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char =
                    &raw mut info.sysname as *mut ::core::ffi::c_char;
                let __prefix: *const ::core::ffi::c_char =
                    b"GNU/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_13
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
                &raw mut info.sysname as *mut ::core::ffi::c_char,
                b"GNU/\0" as *const u8 as *const gchar,
            )
        }) != 0
            || strcmp(
                &raw mut info.sysname as *mut ::core::ffi::c_char,
                b"FreeBSD\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut info.sysname as *mut ::core::ffi::c_char,
                b"DragonFly\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            let mut len_1: gssize = G_MAXSSIZE;
            let mut c_1: *const gchar = ::core::ptr::null::<gchar>();
            c_1 = strchr(
                &raw mut info.release as *mut ::core::ffi::c_char,
                '-' as i32,
            );
            if !c_1.is_null() {
                len_1 = (if len_1
                    < c_1.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                {
                    len_1 as ::core::ffi::c_long
                } else {
                    c_1.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                }) as gssize;
            }
            c_1 = strchr(
                &raw mut info.release as *mut ::core::ffi::c_char,
                '(' as i32,
            );
            if !c_1.is_null() {
                len_1 = (if len_1
                    < c_1.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                {
                    len_1 as ::core::ffi::c_long
                } else {
                    c_1.offset_from(&raw mut info.release as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                }) as gssize;
            }
            if len_1 == G_MAXSSIZE {
                len_1 = -(1 as ::core::ffi::c_int) as gssize;
            }
            result_0 = g_ascii_strdown(&raw mut info.release as *mut ::core::ffi::c_char, len_1);
        } else {
            result_0 = g_ascii_strdown(
                &raw mut info.release as *mut ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        }
        g_strcanon(
            result_0,
            b"abcdefghijklmnopqrstuvwxyz0123456789_-.\0" as *const u8 as *const gchar,
            '_' as i32 as gchar,
        );
        return safe_c2rust_g_steal_pointer(&raw mut result_0 as gpointer) as *mut gchar;
    } else {
        return ::core::ptr::null_mut::<gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_os_info(mut key_name: *const gchar) -> *mut gchar {
    let os_release_files: [*const gchar; 2] = [
        b"/etc/os-release\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/lib/os-release\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: gsize = 0;
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !key_name.is_null() {
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
            b"key_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    i = 0 as gsize;
    while (i as usize)
        < (::core::mem::size_of::<[*const gchar; 2]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const gchar>() as usize)
    {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut file_missing: gboolean = 0;
        if g_file_get_contents(
            os_release_files[i as usize],
            &raw mut buffer,
            ::core::ptr::null_mut::<gsize>(),
            &raw mut error,
        ) != 0
        {
            break;
        }
        file_missing = g_error_matches(
            error,
            g_file_error_quark(),
            G_FILE_ERROR_NOENT as ::core::ffi::c_int as gint,
        );
        g_clear_error(&raw mut error);
        if file_missing == 0 {
            return ::core::ptr::null_mut::<gchar>();
        }
        i = i.wrapping_add(1);
    }
    if !buffer.is_null() {
        result = safe_c2rust_get_os_info_from_os_release(key_name, buffer);
    } else {
        result = safe_c2rust_get_os_info_from_uname(key_name);
    }
    g_free(buffer as gpointer);
    return safe_c2rust_g_steal_pointer(&raw mut result as gpointer) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_set_str_if_different(
    mut global_str: *mut *mut gchar,
    mut type_0: *const gchar,
    mut new_value: *const gchar,
) {
    if (*global_str).is_null()
        || !(strcmp(
            new_value as *const ::core::ffi::c_char,
            *global_str as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"g_set_user_dirs: Setting %s to %s\0" as *const u8 as *const gchar,
            type_0,
            new_value,
        );
        safe_c2rust_g_ignore_leak(*global_str as gconstpointer);
        *global_str =
            safe_c2rust_g_strdup_inline(new_value as *const ::core::ffi::c_char) as *mut gchar;
    }
}
unsafe extern "C" fn safe_c2rust_set_strv_if_different(
    mut global_strv: *mut *mut *mut gchar,
    mut type_0: *const gchar,
    mut new_value: *const *const gchar,
) {
    if (*global_strv).is_null() || g_strv_equal(new_value, *global_strv as *const *const gchar) == 0
    {
        let mut new_value_str: *mut gchar = g_strjoinv(
            b":\0" as *const u8 as *const gchar,
            new_value as *mut *mut gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"g_set_user_dirs: Setting %s to %s\0" as *const u8 as *const gchar,
            type_0,
            new_value_str,
        );
        g_free(new_value_str as gpointer);
        safe_c2rust_g_ignore_strv_leak(*global_strv);
        *global_strv = g_strdupv(new_value as *mut *mut gchar);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_set_user_dirs(
    mut first_dir_type: *const gchar,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    let mut dir_type: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    args_0 = args.clone();
    dir_type = first_dir_type;
    while !dir_type.is_null() {
        let mut dir_value: gconstpointer = args_0.arg::<gconstpointer>();
        if ({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if !dir_value.is_null() {
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
                b"../original/glib/gutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                1836 as ::core::ffi::c_int,
                G_STRFUNC,
                b"dir_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_str_if_different(
                &raw mut safe_c2rust_g_home_dir,
                dir_type,
                dir_value as *const gchar,
            );
        } else if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"XDG_CACHE_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_str_if_different(
                &raw mut safe_c2rust_g_user_cache_dir,
                dir_type,
                dir_value as *const gchar,
            );
        } else if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"XDG_CONFIG_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_strv_if_different(
                &raw mut safe_c2rust_g_system_config_dirs,
                dir_type,
                dir_value as *const *const gchar,
            );
        } else if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"XDG_CONFIG_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_str_if_different(
                &raw mut safe_c2rust_g_user_config_dir,
                dir_type,
                dir_value as *const gchar,
            );
        } else if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"XDG_DATA_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_strv_if_different(
                &raw mut safe_c2rust_g_system_data_dirs,
                dir_type,
                dir_value as *const *const gchar,
            );
        } else if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"XDG_DATA_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_str_if_different(
                &raw mut safe_c2rust_g_user_data_dir,
                dir_type,
                dir_value as *const gchar,
            );
        } else if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"XDG_STATE_HOME\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_str_if_different(
                &raw mut safe_c2rust_g_user_state_dir,
                dir_type,
                dir_value as *const gchar,
            );
        } else if strcmp(
            dir_type as *const ::core::ffi::c_char,
            b"XDG_RUNTIME_DIR\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_set_str_if_different(
                &raw mut safe_c2rust_g_user_runtime_dir,
                dir_type,
                dir_value as *const gchar,
            );
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                1855 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        dir_type = args_0.arg::<*const gchar>();
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
}
unsafe extern "C" fn safe_c2rust_g_build_user_data_dir() -> *mut gchar {
    let mut data_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut data_dir_env: *const gchar = g_getenv(b"XDG_DATA_HOME\0" as *const u8 as *const gchar);
    if !data_dir_env.is_null()
        && *data_dir_env.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        data_dir =
            safe_c2rust_g_strdup_inline(data_dir_env as *const ::core::ffi::c_char) as *mut gchar;
    }
    if data_dir.is_null() || *data_dir.offset(0 as ::core::ffi::c_int as isize) == 0 {
        let mut home_dir: *mut gchar = safe_c2rust_g_build_home_dir();
        g_free(data_dir as gpointer);
        data_dir = g_build_filename(
            home_dir,
            b".local\0" as *const u8 as *const ::core::ffi::c_char,
            b"share\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        );
        g_free(home_dir as gpointer);
    }
    return safe_c2rust_g_steal_pointer(&raw mut data_dir as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_user_data_dir() -> *const gchar {
    let mut user_data_dir: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_user_data_dir.is_null() {
        safe_c2rust_g_user_data_dir = safe_c2rust_g_build_user_data_dir();
    }
    user_data_dir = safe_c2rust_g_user_data_dir;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return user_data_dir;
}
unsafe extern "C" fn safe_c2rust_g_build_user_config_dir() -> *mut gchar {
    let mut config_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut config_dir_env: *const gchar =
        g_getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const gchar);
    if !config_dir_env.is_null()
        && *config_dir_env.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        config_dir =
            safe_c2rust_g_strdup_inline(config_dir_env as *const ::core::ffi::c_char) as *mut gchar;
    }
    if config_dir.is_null() || *config_dir.offset(0 as ::core::ffi::c_int as isize) == 0 {
        let mut home_dir: *mut gchar = safe_c2rust_g_build_home_dir();
        config_dir = g_build_filename(
            home_dir,
            b".config\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        );
        g_free(home_dir as gpointer);
    }
    return safe_c2rust_g_steal_pointer(&raw mut config_dir as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_user_config_dir() -> *const gchar {
    let mut user_config_dir: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_user_config_dir.is_null() {
        safe_c2rust_g_user_config_dir = safe_c2rust_g_build_user_config_dir();
    }
    user_config_dir = safe_c2rust_g_user_config_dir;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return user_config_dir;
}
unsafe extern "C" fn safe_c2rust_g_build_user_cache_dir() -> *mut gchar {
    let mut cache_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut cache_dir_env: *const gchar =
        g_getenv(b"XDG_CACHE_HOME\0" as *const u8 as *const gchar);
    if !cache_dir_env.is_null()
        && *cache_dir_env.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        cache_dir =
            safe_c2rust_g_strdup_inline(cache_dir_env as *const ::core::ffi::c_char) as *mut gchar;
    }
    if cache_dir.is_null() || *cache_dir.offset(0 as ::core::ffi::c_int as isize) == 0 {
        let mut home_dir: *mut gchar = safe_c2rust_g_build_home_dir();
        cache_dir = g_build_filename(
            home_dir,
            b".cache\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        );
        g_free(home_dir as gpointer);
    }
    return safe_c2rust_g_steal_pointer(&raw mut cache_dir as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_user_cache_dir() -> *const gchar {
    let mut user_cache_dir: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_user_cache_dir.is_null() {
        safe_c2rust_g_user_cache_dir = safe_c2rust_g_build_user_cache_dir();
    }
    user_cache_dir = safe_c2rust_g_user_cache_dir;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return user_cache_dir;
}
unsafe extern "C" fn safe_c2rust_g_build_user_state_dir() -> *mut gchar {
    let mut state_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut state_dir_env: *const gchar =
        g_getenv(b"XDG_STATE_HOME\0" as *const u8 as *const gchar);
    if !state_dir_env.is_null()
        && *state_dir_env.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        state_dir =
            safe_c2rust_g_strdup_inline(state_dir_env as *const ::core::ffi::c_char) as *mut gchar;
    }
    if state_dir.is_null() || *state_dir.offset(0 as ::core::ffi::c_int as isize) == 0 {
        let mut home_dir: *mut gchar = safe_c2rust_g_build_home_dir();
        state_dir = g_build_filename(
            home_dir,
            b".local/state\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        );
        g_free(home_dir as gpointer);
    }
    return safe_c2rust_g_steal_pointer(&raw mut state_dir as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_user_state_dir() -> *const gchar {
    let mut user_state_dir: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_user_state_dir.is_null() {
        safe_c2rust_g_user_state_dir = safe_c2rust_g_build_user_state_dir();
    }
    user_state_dir = safe_c2rust_g_user_state_dir;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return user_state_dir;
}
unsafe extern "C" fn safe_c2rust_g_build_user_runtime_dir() -> *mut gchar {
    let mut runtime_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut runtime_dir_env: *const gchar =
        g_getenv(b"XDG_RUNTIME_DIR\0" as *const u8 as *const gchar);
    if !runtime_dir_env.is_null()
        && *runtime_dir_env.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        runtime_dir = safe_c2rust_g_strdup_inline(runtime_dir_env as *const ::core::ffi::c_char)
            as *mut gchar;
    } else {
        runtime_dir = safe_c2rust_g_build_user_cache_dir();
        mkdir(runtime_dir, 0o700 as __mode_t);
    }
    return safe_c2rust_g_steal_pointer(&raw mut runtime_dir as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_user_runtime_dir() -> *const gchar {
    let mut user_runtime_dir: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_user_runtime_dir.is_null() {
        safe_c2rust_g_user_runtime_dir = safe_c2rust_g_build_user_runtime_dir();
    }
    user_runtime_dir = safe_c2rust_g_user_runtime_dir;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return user_runtime_dir;
}
unsafe extern "C" fn safe_c2rust_load_user_special_dirs() {
    let mut config_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut config_file: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut data: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n_lines: gint = 0;
    let mut i: gint = 0;
    config_dir = safe_c2rust_g_build_user_config_dir();
    config_file = g_build_filename(
        config_dir,
        b"user-dirs.dirs\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    g_free(config_dir as gpointer);
    if g_file_get_contents(
        config_file,
        &raw mut data,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        g_free(config_file as gpointer);
        return;
    }
    lines = g_strsplit(data, b"\n\0" as *const u8 as *const gchar, -(1 as gint));
    n_lines = g_strv_length(lines) as gint;
    g_free(data as gpointer);
    let mut current_block_60: u64;
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_lines {
        let mut buffer: *mut gchar = *lines.offset(i as isize);
        let mut d: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut len: gint = 0;
        let mut is_relative: gboolean = FALSE;
        let mut directory: GUserDirectory = G_USER_DIRECTORY_DESKTOP;
        len = strlen(buffer) as gint;
        if len > 0 as ::core::ffi::c_int
            && *buffer.offset((len as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == '\n' as i32
        {
            *buffer.offset((len as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                0 as gchar;
        }
        p = buffer;
        while *p as ::core::ffi::c_int == ' ' as i32 || *p as ::core::ffi::c_int == '\t' as i32 {
            p = p.offset(1);
        }
        if strncmp(
            p,
            b"XDG_DESKTOP_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_DESKTOP_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_DESKTOP;
            p = p.offset(
                strlen(b"XDG_DESKTOP_DIR\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
            current_block_60 = 3934796541983872331;
        } else if strncmp(
            p,
            b"XDG_DOCUMENTS_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_DOCUMENTS_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_DOCUMENTS;
            p = p.offset(
                strlen(b"XDG_DOCUMENTS_DIR\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
            current_block_60 = 3934796541983872331;
        } else if strncmp(
            p,
            b"XDG_DOWNLOAD_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_DOWNLOAD_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_DOWNLOAD;
            p = p.offset(
                strlen(b"XDG_DOWNLOAD_DIR\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
            current_block_60 = 3934796541983872331;
        } else if strncmp(
            p,
            b"XDG_MUSIC_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_MUSIC_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_MUSIC;
            p = p.offset(
                strlen(b"XDG_MUSIC_DIR\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
            current_block_60 = 3934796541983872331;
        } else if strncmp(
            p,
            b"XDG_PICTURES_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_PICTURES_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_PICTURES;
            p = p.offset(
                strlen(b"XDG_PICTURES_DIR\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
            current_block_60 = 3934796541983872331;
        } else if strncmp(
            p,
            b"XDG_PUBLICSHARE_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_PUBLICSHARE_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_PUBLIC_SHARE;
            p = p.offset(
                strlen(b"XDG_PUBLICSHARE_DIR\0" as *const u8 as *const ::core::ffi::c_char)
                    as isize,
            );
            current_block_60 = 3934796541983872331;
        } else if strncmp(
            p,
            b"XDG_TEMPLATES_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_TEMPLATES_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_TEMPLATES;
            p = p.offset(
                strlen(b"XDG_TEMPLATES_DIR\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
            current_block_60 = 3934796541983872331;
        } else if strncmp(
            p,
            b"XDG_VIDEOS_DIR\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"XDG_VIDEOS_DIR\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            directory = G_USER_DIRECTORY_VIDEOS;
            p = p.offset(
                strlen(b"XDG_VIDEOS_DIR\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
            current_block_60 = 3934796541983872331;
        } else {
            current_block_60 = 13513818773234778473;
        }
        match current_block_60 {
            3934796541983872331 => {
                while *p as ::core::ffi::c_int == ' ' as i32
                    || *p as ::core::ffi::c_int == '\t' as i32
                {
                    p = p.offset(1);
                }
                if !(*p as ::core::ffi::c_int != '=' as i32) {
                    p = p.offset(1);
                    while *p as ::core::ffi::c_int == ' ' as i32
                        || *p as ::core::ffi::c_int == '\t' as i32
                    {
                        p = p.offset(1);
                    }
                    if !(*p as ::core::ffi::c_int != '"' as i32) {
                        p = p.offset(1);
                        if strncmp(
                            p,
                            b"$HOME\0" as *const u8 as *const ::core::ffi::c_char,
                            5 as size_t,
                        ) == 0 as ::core::ffi::c_int
                        {
                            p = p.offset(5 as ::core::ffi::c_int as isize);
                            is_relative = TRUE as gboolean;
                            current_block_60 = 7990025728955927862;
                        } else if *p as ::core::ffi::c_int != '/' as i32 {
                            current_block_60 = 13513818773234778473;
                        } else {
                            current_block_60 = 7990025728955927862;
                        }
                        match current_block_60 {
                            13513818773234778473 => {}
                            _ => {
                                d = strrchr(p, '"' as i32) as *mut gchar;
                                if !d.is_null() {
                                    *d = 0 as gchar;
                                    d = p;
                                    len = strlen(d) as gint;
                                    if *d.offset(
                                        (len as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == '/' as i32
                                    {
                                        *d.offset(
                                            (len as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                                as isize,
                                        ) = 0 as gchar;
                                    }
                                    if is_relative != 0 {
                                        let mut home_dir: *mut gchar =
                                            safe_c2rust_g_build_home_dir();
                                        let ref mut fresh3 = *safe_c2rust_g_user_special_dirs
                                            .offset(directory as isize);
                                        *fresh3 = g_build_filename(home_dir, d, NULL_0);
                                        g_free(home_dir as gpointer);
                                    } else {
                                        let ref mut fresh4 = *safe_c2rust_g_user_special_dirs
                                            .offset(directory as isize);
                                        *fresh4 = safe_c2rust_g_strdup_inline(d) as *mut gchar;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    g_strfreev(lines);
    g_free(config_file as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_reload_user_special_dirs_cache() {
    let mut i: ::core::ffi::c_int = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if !safe_c2rust_g_user_special_dirs.is_null() {
        let mut old_g_user_special_dirs: *mut *mut ::core::ffi::c_char =
            safe_c2rust_g_user_special_dirs as *mut *mut ::core::ffi::c_char;
        let mut old_val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        safe_c2rust_g_user_special_dirs = ({
            let mut __n: gsize = G_USER_N_DIRECTORIES as ::core::ffi::c_int as gsize;
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
        safe_c2rust_load_user_special_dirs();
        i = 0 as ::core::ffi::c_int;
        while i < G_USER_N_DIRECTORIES as ::core::ffi::c_int {
            old_val = *old_g_user_special_dirs.offset(i as isize);
            if (*safe_c2rust_g_user_special_dirs.offset(i as isize)).is_null() {
                let ref mut fresh1 = *safe_c2rust_g_user_special_dirs.offset(i as isize);
                *fresh1 = old_val as *mut gchar;
            } else if g_strcmp0(old_val, *safe_c2rust_g_user_special_dirs.offset(i as isize))
                == 0 as ::core::ffi::c_int
            {
                g_free(*safe_c2rust_g_user_special_dirs.offset(i as isize) as gpointer);
                let ref mut fresh2 = *safe_c2rust_g_user_special_dirs.offset(i as isize);
                *fresh2 = old_val as *mut gchar;
            } else {
                g_free(old_val as gpointer);
            }
            i += 1;
        }
        g_free(old_g_user_special_dirs as gpointer);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_user_special_dir(
    mut directory: GUserDirectory,
) -> *const gchar {
    let mut user_special_dir: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if directory as ::core::ffi::c_uint
            >= G_USER_DIRECTORY_DESKTOP as ::core::ffi::c_int as ::core::ffi::c_uint
            && (directory as ::core::ffi::c_uint)
                < G_USER_N_DIRECTORIES as ::core::ffi::c_int as ::core::ffi::c_uint
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
            b"directory >= G_USER_DIRECTORY_DESKTOP && directory < G_USER_N_DIRECTORIES\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if safe_c2rust_g_user_special_dirs.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_user_special_dirs = ({
            let mut __n: gsize = G_USER_N_DIRECTORIES as ::core::ffi::c_int as gsize;
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
        safe_c2rust_load_user_special_dirs();
        if (*safe_c2rust_g_user_special_dirs
            .offset(G_USER_DIRECTORY_DESKTOP as ::core::ffi::c_int as isize))
        .is_null()
        {
            let mut home_dir: *mut gchar = safe_c2rust_g_build_home_dir();
            let ref mut fresh5 = *safe_c2rust_g_user_special_dirs
                .offset(G_USER_DIRECTORY_DESKTOP as ::core::ffi::c_int as isize);
            *fresh5 = g_build_filename(
                home_dir,
                b"Desktop\0" as *const u8 as *const ::core::ffi::c_char,
                NULL_0,
            );
            g_free(home_dir as gpointer);
        }
    }
    user_special_dir = *safe_c2rust_g_user_special_dirs.offset(directory as isize);
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return user_special_dir;
}
unsafe extern "C" fn safe_c2rust_g_build_system_data_dirs() -> *mut *mut gchar {
    let mut data_dir_vector: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut data_dirs: *mut gchar =
        g_getenv(b"XDG_DATA_DIRS\0" as *const u8 as *const gchar) as *mut gchar;
    if data_dirs.is_null() || *data_dirs.offset(0 as ::core::ffi::c_int as isize) == 0 {
        data_dirs = b"/usr/local/share/:/usr/share/\0" as *const u8 as *const ::core::ffi::c_char
            as *mut gchar;
    }
    data_dir_vector = g_strsplit(
        data_dirs,
        G_SEARCHPATH_SEPARATOR_S.as_ptr() as *const gchar,
        0 as gint,
    );
    return safe_c2rust_g_steal_pointer(&raw mut data_dir_vector as gpointer) as *mut *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_system_data_dirs() -> *const *const gchar {
    let mut system_data_dirs: *const *const gchar = ::core::ptr::null::<*const gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_system_data_dirs.is_null() {
        safe_c2rust_g_system_data_dirs = safe_c2rust_g_build_system_data_dirs();
    }
    system_data_dirs = safe_c2rust_g_system_data_dirs as *const *const gchar;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return system_data_dirs;
}
unsafe extern "C" fn safe_c2rust_g_build_system_config_dirs() -> *mut *mut gchar {
    let mut conf_dir_vector: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut conf_dirs: *const gchar = g_getenv(b"XDG_CONFIG_DIRS\0" as *const u8 as *const gchar);
    if conf_dirs.is_null() || *conf_dirs.offset(0 as ::core::ffi::c_int as isize) == 0 {
        conf_dirs = b"/etc/xdg\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    conf_dir_vector = g_strsplit(
        conf_dirs,
        G_SEARCHPATH_SEPARATOR_S.as_ptr() as *const gchar,
        0 as gint,
    );
    return safe_c2rust_g_steal_pointer(&raw mut conf_dir_vector as gpointer) as *mut *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_system_config_dirs() -> *const *const gchar {
    let mut system_config_dirs: *const *const gchar = ::core::ptr::null::<*const gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__g_utils_global_lock);
    if safe_c2rust_g_system_config_dirs.is_null() {
        safe_c2rust_g_system_config_dirs = safe_c2rust_g_build_system_config_dirs();
    }
    system_config_dirs = safe_c2rust_g_system_config_dirs as *const *const gchar;
    g_mutex_unlock(&raw mut safe_c2rust_g__g_utils_global_lock);
    return system_config_dirs;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_nullify_pointer(mut nullify_location: *mut gpointer) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !nullify_location.is_null() {
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
            b"nullify_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *nullify_location = NULL_0 as gpointer;
}
pub const KILOBYTE_FACTOR: ::core::ffi::c_long = 1000 as ::core::ffi::c_long;
pub const MEGABYTE_FACTOR: ::core::ffi::c_long = KILOBYTE_FACTOR * KILOBYTE_FACTOR;
pub const GIGABYTE_FACTOR: ::core::ffi::c_long = MEGABYTE_FACTOR * KILOBYTE_FACTOR;
pub const TERABYTE_FACTOR: ::core::ffi::c_long = GIGABYTE_FACTOR * KILOBYTE_FACTOR;
pub const PETABYTE_FACTOR: ::core::ffi::c_long = TERABYTE_FACTOR * KILOBYTE_FACTOR;
pub const EXABYTE_FACTOR: ::core::ffi::c_long = PETABYTE_FACTOR * KILOBYTE_FACTOR;
pub const KIBIBYTE_FACTOR: ::core::ffi::c_long = 1024 as ::core::ffi::c_long;
pub const MEBIBYTE_FACTOR: ::core::ffi::c_long = KIBIBYTE_FACTOR * KIBIBYTE_FACTOR;
pub const GIBIBYTE_FACTOR: ::core::ffi::c_long = MEBIBYTE_FACTOR * KIBIBYTE_FACTOR;
pub const TEBIBYTE_FACTOR: ::core::ffi::c_long = GIBIBYTE_FACTOR * KIBIBYTE_FACTOR;
pub const PEBIBYTE_FACTOR: ::core::ffi::c_long = TEBIBYTE_FACTOR * KIBIBYTE_FACTOR;
pub const EXBIBYTE_FACTOR: ::core::ffi::c_long = PEBIBYTE_FACTOR * KIBIBYTE_FACTOR;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_format_size(mut size: guint64) -> *mut gchar {
    return safe_c2rust_g_format_size_full(size, G_FORMAT_SIZE_DEFAULT);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_format_size_full(
    mut size: guint64,
    mut flags: GFormatSizeFlags,
) -> *mut gchar {
    let formats: [[Format; 6]; 4] = [
        [
            Format {
                factor: KILOBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"kB\0\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: MEGABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"MB\0\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: GIGABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"GB\0\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: TERABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"TB\0\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: PETABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"PB\0\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: EXABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"EB\0\0\0\0\0\0\0\0",
                ),
            },
        ],
        [
            Format {
                factor: KIBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"KiB\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: MEBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"MiB\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: GIBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"GiB\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: TEBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"TiB\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: PEBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"PiB\0\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: EXBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"EiB\0\0\0\0\0\0\0",
                ),
            },
        ],
        [
            Format {
                factor: KILOBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"kbit\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: MEGABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Mbit\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: GIGABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Gbit\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: TERABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Tbit\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: PETABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Pbit\0\0\0\0\0\0",
                ),
            },
            Format {
                factor: EXABYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Ebit\0\0\0\0\0\0",
                ),
            },
        ],
        [
            Format {
                factor: KIBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Kibit\0\0\0\0\0",
                ),
            },
            Format {
                factor: MEBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Mibit\0\0\0\0\0",
                ),
            },
            Format {
                factor: GIBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Gibit\0\0\0\0\0",
                ),
            },
            Format {
                factor: TEBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Tibit\0\0\0\0\0",
                ),
            },
            Format {
                factor: PEBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Pibit\0\0\0\0\0",
                ),
            },
            Format {
                factor: EXBIBYTE_FACTOR as guint64,
                string: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"Eibit\0\0\0\0\0",
                ),
            },
        ],
    ];
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut index: FormatIndex = FORMAT_BYTES;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & (G_FORMAT_SIZE_LONG_FORMAT as ::core::ffi::c_int
                | G_FORMAT_SIZE_ONLY_VALUE as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_FORMAT_SIZE_LONG_FORMAT as ::core::ffi::c_int
                | G_FORMAT_SIZE_ONLY_VALUE as ::core::ffi::c_int)
                as ::core::ffi::c_uint
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
            b"(flags & (G_FORMAT_SIZE_LONG_FORMAT | G_FORMAT_SIZE_ONLY_VALUE)) != (G_FORMAT_SIZE_LONG_FORMAT | G_FORMAT_SIZE_ONLY_VALUE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & (G_FORMAT_SIZE_LONG_FORMAT as ::core::ffi::c_int
                | G_FORMAT_SIZE_ONLY_UNIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_FORMAT_SIZE_LONG_FORMAT as ::core::ffi::c_int
                | G_FORMAT_SIZE_ONLY_UNIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
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
            b"(flags & (G_FORMAT_SIZE_LONG_FORMAT | G_FORMAT_SIZE_ONLY_UNIT)) != (G_FORMAT_SIZE_LONG_FORMAT | G_FORMAT_SIZE_ONLY_UNIT)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & (G_FORMAT_SIZE_ONLY_VALUE as ::core::ffi::c_int
                | G_FORMAT_SIZE_ONLY_UNIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_FORMAT_SIZE_ONLY_VALUE as ::core::ffi::c_int
                | G_FORMAT_SIZE_ONLY_UNIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
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
            b"(flags & (G_FORMAT_SIZE_ONLY_VALUE | G_FORMAT_SIZE_ONLY_UNIT)) != (G_FORMAT_SIZE_ONLY_VALUE | G_FORMAT_SIZE_ONLY_UNIT)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    string = g_string_new(::core::ptr::null::<gchar>());
    match flags as ::core::ffi::c_uint
        & !(G_FORMAT_SIZE_LONG_FORMAT as ::core::ffi::c_int
            | G_FORMAT_SIZE_ONLY_VALUE as ::core::ffi::c_int
            | G_FORMAT_SIZE_ONLY_UNIT as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        0 => {
            index = FORMAT_BYTES;
        }
        2 => {
            index = FORMAT_BYTES_IEC;
        }
        4 => {
            index = FORMAT_BITS;
        }
        6 => {
            index = FORMAT_BITS_IEC;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                3046 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    if size < formats[index as usize][0 as ::core::ffi::c_int as usize].factor {
        let mut units: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if index as ::core::ffi::c_uint == FORMAT_BYTES as ::core::ffi::c_int as ::core::ffi::c_uint
            || index as ::core::ffi::c_uint
                == FORMAT_BYTES_IEC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            units = g_dngettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"byte\0" as *const u8 as *const gchar,
                b"bytes\0" as *const u8 as *const gchar,
                size as guint as gulong,
            ) as *const ::core::ffi::c_char;
        } else {
            units = g_dngettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"bit\0" as *const u8 as *const gchar,
                b"bits\0" as *const u8 as *const gchar,
                size as guint as gulong,
            ) as *const ::core::ffi::c_char;
        }
        if flags as ::core::ffi::c_uint
            & G_FORMAT_SIZE_ONLY_UNIT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = units;
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        __val,
                        if ({
                            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_22
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
                    string,
                    units,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else if flags as ::core::ffi::c_uint
            & G_FORMAT_SIZE_ONLY_VALUE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            g_string_printf(
                string,
                glib_pgettext(
                    b"format-size\x04%u\0" as *const u8 as *const gchar,
                    (strlen(b"format-size\0" as *const u8 as *const ::core::ffi::c_char) as gsize)
                        .wrapping_add(1 as gsize),
                ),
                size as guint,
            );
        } else {
            g_string_printf(
                string,
                glib_pgettext(
                    b"format-size\x04%u %s\0" as *const u8 as *const gchar,
                    (strlen(b"format-size\0" as *const u8 as *const ::core::ffi::c_char) as gsize)
                        .wrapping_add(1 as gsize),
                ),
                size as guint,
                units,
            );
        }
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GFormatSizeFlags>(
            flags as ::core::ffi::c_uint
                & !(G_FORMAT_SIZE_LONG_FORMAT as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    } else {
        let n: gsize = (::core::mem::size_of::<[Format; 6]>() as gsize)
            .wrapping_div(::core::mem::size_of::<Format>() as gsize);
        let mut units_0: *const gchar = ::core::ptr::null::<gchar>();
        let mut value: gdouble = 0.;
        let mut i: gsize = 0;
        let mut f: *const Format = (&raw const *(&raw const formats as *const [Format; 6])
            .offset(index as isize) as *const Format)
            .offset(n.wrapping_sub(1 as gsize) as isize)
            as *const Format;
        i = 1 as gsize;
        while i < n {
            if size < formats[index as usize][i as usize].factor {
                f = (&raw const *(&raw const formats as *const [Format; 6]).offset(index as isize)
                    as *const Format)
                    .offset(i.wrapping_sub(1 as gsize) as isize)
                    as *const Format;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        units_0 = glib_gettext(&raw const (*f).string as *const gchar);
        value = size as gdouble / (*f).factor as gdouble;
        if flags as ::core::ffi::c_uint
            & G_FORMAT_SIZE_ONLY_UNIT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = units_0 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        __val,
                        if ({
                            let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_23
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
                    string,
                    units_0 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else if flags as ::core::ffi::c_uint
            & G_FORMAT_SIZE_ONLY_VALUE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            g_string_printf(
                string,
                glib_pgettext(
                    b"format-size\x04%.1f\0" as *const u8 as *const gchar,
                    (strlen(b"format-size\0" as *const u8 as *const ::core::ffi::c_char) as gsize)
                        .wrapping_add(1 as gsize),
                ),
                value,
            );
        } else {
            g_string_printf(
                string,
                glib_pgettext(
                    b"format-size\x04%.1f\xC2\xA0%s\0" as *const u8 as *const gchar,
                    (strlen(b"format-size\0" as *const u8 as *const ::core::ffi::c_char) as gsize)
                        .wrapping_add(1 as gsize),
                ),
                value,
                units_0,
            );
        }
    }
    if flags as ::core::ffi::c_uint
        & G_FORMAT_SIZE_LONG_FORMAT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut plural_form: guint = (if size < 1000 as guint64 {
            size
        } else {
            size.wrapping_rem(1000 as guint64)
                .wrapping_add(1000 as guint64)
        }) as guint;
        let mut translated_format: *const gchar = ::core::ptr::null::<gchar>();
        let mut formatted_number: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if index as ::core::ffi::c_uint == FORMAT_BYTES as ::core::ffi::c_int as ::core::ffi::c_uint
            || index as ::core::ffi::c_uint
                == FORMAT_BYTES_IEC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            translated_format = g_dngettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"%s byte\0" as *const u8 as *const gchar,
                b"%s bytes\0" as *const u8 as *const gchar,
                plural_form as gulong,
            );
        } else {
            translated_format = g_dngettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"%s bit\0" as *const u8 as *const gchar,
                b"%s bits\0" as *const u8 as *const gchar,
                plural_form as gulong,
            );
        }
        formatted_number = g_strdup_printf(b"%'lu\0" as *const u8 as *const gchar, size);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b" (\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_24
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
                string,
                b" (\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_string_append_printf(string, translated_format, formatted_number);
        g_free(formatted_number as gpointer);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b")\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_25
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
                string,
                b")\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(string, 0 as gboolean)
        } else {
            g_string_free_and_steal(string)
        }
    } else {
        g_string_free(string, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_format_size_for_display(mut size: goffset) -> *mut gchar {
    if size < KIBIBYTE_FACTOR {
        return g_strdup_printf(
            g_dngettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"%u byte\0" as *const u8 as *const gchar,
                b"%u bytes\0" as *const u8 as *const gchar,
                size as guint as gulong,
            ),
            size as guint,
        );
    } else {
        let mut displayed_size: gdouble = 0.;
        if size < MEBIBYTE_FACTOR {
            displayed_size = size as gdouble / KIBIBYTE_FACTOR as gdouble;
            return g_strdup_printf(
                glib_gettext(b"%.1f KB\0" as *const u8 as *const gchar),
                displayed_size,
            );
        } else if size < GIBIBYTE_FACTOR {
            displayed_size = size as gdouble / MEBIBYTE_FACTOR as gdouble;
            return g_strdup_printf(
                glib_gettext(b"%.1f MB\0" as *const u8 as *const gchar),
                displayed_size,
            );
        } else if size < TEBIBYTE_FACTOR {
            displayed_size = size as gdouble / GIBIBYTE_FACTOR as gdouble;
            return g_strdup_printf(
                glib_gettext(b"%.1f GB\0" as *const u8 as *const gchar),
                displayed_size,
            );
        } else if size < PEBIBYTE_FACTOR {
            displayed_size = size as gdouble / TEBIBYTE_FACTOR as gdouble;
            return g_strdup_printf(
                glib_gettext(b"%.1f TB\0" as *const u8 as *const gchar),
                displayed_size,
            );
        } else if size < EXBIBYTE_FACTOR {
            displayed_size = size as gdouble / PEBIBYTE_FACTOR as gdouble;
            return g_strdup_printf(
                glib_gettext(b"%.1f PB\0" as *const u8 as *const gchar),
                displayed_size,
            );
        } else {
            displayed_size = size as gdouble / EXBIBYTE_FACTOR as gdouble;
            return g_strdup_printf(
                glib_gettext(b"%.1f EB\0" as *const u8 as *const gchar),
                displayed_size,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_check_setuid() -> gboolean {
    let mut value: ::core::ffi::c_ulong = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    *__errno_location() = 0 as ::core::ffi::c_int;
    value = getauxval(AT_SECURE as ::core::ffi::c_ulong);
    errsv = *__errno_location();
    if errsv != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"getauxval () failed: %s\0" as *const u8 as *const gchar,
            g_strerror(errsv as gint),
        );
        loop {}
    }
    return value as gboolean;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const GETTEXT_PACKAGE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"glib20\0") };
