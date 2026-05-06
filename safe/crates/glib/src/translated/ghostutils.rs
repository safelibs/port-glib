extern "C" {
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_sized_new(
        zero_terminated: gboolean,
        clear_: gboolean,
        element_size: guint,
        reserved_size: guint,
    ) -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_array_insert_vals(
        array: *mut GArray,
        index_: guint,
        data: gconstpointer,
        len: guint,
    ) -> *mut GArray;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_unichar_isupper(c: gunichar) -> gboolean;
    fn g_unichar_type(c: gunichar) -> GUnicodeType;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_to_ucs4(
        str: *const gchar,
        len: glong,
        items_read: *mut glong,
        items_written: *mut glong,
        error: *mut *mut GError,
    ) -> *mut gunichar;
    fn g_utf8_strdown(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_utf8_normalize(str: *const gchar, len: gssize, mode: GNormalizeMode) -> *mut gchar;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_tolower(c: gchar) -> gchar;
    fn g_ascii_strncasecmp(s1: *const gchar, s2: *const gchar, n: gsize) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
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
    fn g_string_append_unichar(string: *mut GString, wc: gunichar) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
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
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
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
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
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
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub const G_ASCII_DIGIT: C2RustUnnamed_0 = 8;
pub const G_ASCII_XDIGIT: C2RustUnnamed_0 = 1024;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type gunichar = guint32;
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GQuark = guint32;
pub const G_UNICODE_NON_SPACING_MARK: GUnicodeType = 12;
pub const G_UNICODE_OTHER_SYMBOL: GUnicodeType = 26;
pub const G_UNICODE_SPACE_SEPARATOR: GUnicodeType = 29;
pub const G_UNICODE_PARAGRAPH_SEPARATOR: GUnicodeType = 28;
pub const G_UNICODE_LINE_SEPARATOR: GUnicodeType = 27;
pub const G_UNICODE_SURROGATE: GUnicodeType = 4;
pub const G_UNICODE_PRIVATE_USE: GUnicodeType = 3;
pub const G_UNICODE_UNASSIGNED: GUnicodeType = 2;
pub const G_UNICODE_FORMAT: GUnicodeType = 1;
pub const G_UNICODE_CONTROL: GUnicodeType = 0;
pub type GUnicodeType = ::core::ffi::c_uint;
pub const G_UNICODE_MATH_SYMBOL: GUnicodeType = 25;
pub const G_UNICODE_MODIFIER_SYMBOL: GUnicodeType = 24;
pub const G_UNICODE_CURRENCY_SYMBOL: GUnicodeType = 23;
pub const G_UNICODE_OPEN_PUNCTUATION: GUnicodeType = 22;
pub const G_UNICODE_OTHER_PUNCTUATION: GUnicodeType = 21;
pub const G_UNICODE_INITIAL_PUNCTUATION: GUnicodeType = 20;
pub const G_UNICODE_FINAL_PUNCTUATION: GUnicodeType = 19;
pub const G_UNICODE_CLOSE_PUNCTUATION: GUnicodeType = 18;
pub const G_UNICODE_DASH_PUNCTUATION: GUnicodeType = 17;
pub const G_UNICODE_CONNECT_PUNCTUATION: GUnicodeType = 16;
pub const G_UNICODE_OTHER_NUMBER: GUnicodeType = 15;
pub const G_UNICODE_LETTER_NUMBER: GUnicodeType = 14;
pub const G_UNICODE_DECIMAL_NUMBER: GUnicodeType = 13;
pub const G_UNICODE_ENCLOSING_MARK: GUnicodeType = 11;
pub const G_UNICODE_SPACING_MARK: GUnicodeType = 10;
pub const G_UNICODE_UPPERCASE_LETTER: GUnicodeType = 9;
pub const G_UNICODE_TITLECASE_LETTER: GUnicodeType = 8;
pub const G_UNICODE_OTHER_LETTER: GUnicodeType = 7;
pub const G_UNICODE_MODIFIER_LETTER: GUnicodeType = 6;
pub const G_UNICODE_LOWERCASE_LETTER: GUnicodeType = 5;
pub type GNormalizeMode = ::core::ffi::c_uint;
pub const G_NORMALIZE_NFKC: GNormalizeMode = 3;
pub const G_NORMALIZE_ALL_COMPOSE: GNormalizeMode = 3;
pub const G_NORMALIZE_NFKD: GNormalizeMode = 2;
pub const G_NORMALIZE_ALL: GNormalizeMode = 2;
pub const G_NORMALIZE_NFC: GNormalizeMode = 1;
pub const G_NORMALIZE_DEFAULT_COMPOSE: GNormalizeMode = 1;
pub const G_NORMALIZE_NFD: GNormalizeMode = 0;
pub const G_NORMALIZE_DEFAULT: GNormalizeMode = 0;
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_ASCII_UPPER: C2RustUnnamed_0 = 512;
pub const G_ASCII_SPACE: C2RustUnnamed_0 = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed_0 = 128;
pub const G_ASCII_PRINT: C2RustUnnamed_0 = 64;
pub const G_ASCII_LOWER: C2RustUnnamed_0 = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed_0 = 16;
pub const G_ASCII_CNTRL: C2RustUnnamed_0 = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed_0 = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed_0 = 1;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const HOST_NAME_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const IDNA_ACE_PREFIX: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"xn--\0") };
pub const IDNA_ACE_PREFIX_LEN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PUNYCODE_BASE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const PUNYCODE_TMIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PUNYCODE_TMAX: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const PUNYCODE_SKEW: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const PUNYCODE_DAMP: ::core::ffi::c_int = 700 as ::core::ffi::c_int;
pub const PUNYCODE_INITIAL_BIAS: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const PUNYCODE_INITIAL_N: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_encode_digit(mut dig: guint) -> gchar {
    if dig < 26 as guint {
        return dig.wrapping_add('a' as i32 as guint) as gchar;
    } else {
        return dig
            .wrapping_sub(26 as guint)
            .wrapping_add('0' as i32 as guint) as gchar;
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_decode_digit(mut dig: gchar) -> guint {
    if dig as ::core::ffi::c_int >= 'A' as i32 && dig as ::core::ffi::c_int <= 'Z' as i32 {
        return (dig as ::core::ffi::c_int - 'A' as i32) as guint;
    } else if dig as ::core::ffi::c_int >= 'a' as i32 && dig as ::core::ffi::c_int <= 'z' as i32 {
        return (dig as ::core::ffi::c_int - 'a' as i32) as guint;
    } else if dig as ::core::ffi::c_int >= '0' as i32 && dig as ::core::ffi::c_int <= '9' as i32 {
        return (dig as ::core::ffi::c_int - '0' as i32 + 26 as ::core::ffi::c_int) as guint;
    } else {
        return G_MAXUINT;
    };
}
unsafe extern "C" fn safe_c2rust_adapt(
    mut delta: guint,
    mut numpoints: guint,
    mut firsttime: gboolean,
) -> guint {
    let mut k: guint = 0;
    delta = if firsttime != 0 {
        delta.wrapping_div(PUNYCODE_DAMP as guint)
    } else {
        delta.wrapping_div(2 as guint)
    };
    delta = delta.wrapping_add(delta.wrapping_div(numpoints));
    k = 0 as guint;
    while delta
        > ((PUNYCODE_BASE - PUNYCODE_TMIN) * PUNYCODE_TMAX / 2 as ::core::ffi::c_int) as guint
    {
        delta = delta.wrapping_div((PUNYCODE_BASE - PUNYCODE_TMIN) as guint);
        k = k.wrapping_add(PUNYCODE_BASE as guint);
    }
    return k.wrapping_add(
        ((PUNYCODE_BASE - PUNYCODE_TMIN + 1 as ::core::ffi::c_int) as guint)
            .wrapping_mul(delta)
            .wrapping_div(delta.wrapping_add(PUNYCODE_SKEW as guint)),
    );
}
unsafe extern "C" fn safe_c2rust_punycode_encode(
    mut input_utf8: *const gchar,
    mut input_utf8_length: gsize,
    mut output: *mut GString,
) -> gboolean {
    let mut current_block: u64;
    let mut delta: guint = 0;
    let mut handled_chars: guint = 0;
    let mut num_basic_chars: guint = 0;
    let mut bias: guint = 0;
    let mut j: guint = 0;
    let mut q: guint = 0;
    let mut k: guint = 0;
    let mut t: guint = 0;
    let mut digit: guint = 0;
    let mut n: gunichar = 0;
    let mut m: gunichar = 0;
    let mut input: *mut gunichar = ::core::ptr::null_mut::<gunichar>();
    let mut written_chars: glong = 0;
    let mut input_length: gsize = 0;
    let mut success: gboolean = FALSE;
    input = g_utf8_to_ucs4(
        input_utf8,
        input_utf8_length as glong,
        ::core::ptr::null_mut::<glong>(),
        &raw mut written_chars,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if input.is_null() {
        return FALSE;
    }
    input_length = (if written_chars > 0 as glong {
        written_chars
    } else {
        0 as glong
    }) as gsize;
    num_basic_chars = 0 as guint;
    j = num_basic_chars;
    while (j as gsize) < input_length {
        if *input.offset(j as isize) < 0x80 as guint {
            safe_c2rust_g_string_append_c_inline(
                output,
                g_ascii_tolower(*input.offset(j as isize) as gchar),
            );
            num_basic_chars = num_basic_chars.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    if num_basic_chars != 0 {
        safe_c2rust_g_string_append_c_inline(output, '-' as i32 as gchar);
    }
    handled_chars = num_basic_chars;
    delta = 0 as guint;
    bias = PUNYCODE_INITIAL_BIAS as guint;
    n = PUNYCODE_INITIAL_N as gunichar;
    's_74: loop {
        if !((handled_chars as gsize) < input_length) {
            current_block = 721385680381463314;
            break;
        }
        m = G_MAXUINT as gunichar;
        j = 0 as guint;
        while (j as gsize) < input_length {
            if *input.offset(j as isize) >= n && *input.offset(j as isize) < m {
                m = *input.offset(j as isize);
            }
            j = j.wrapping_add(1);
        }
        if m.wrapping_sub(n)
            > G_MAXUINT
                .wrapping_sub(delta)
                .wrapping_div(handled_chars.wrapping_add(1 as guint))
        {
            current_block = 16910374505238418596;
            break;
        }
        delta = (delta as ::core::ffi::c_uint).wrapping_add(
            (m as guint)
                .wrapping_sub(n as guint)
                .wrapping_mul(handled_chars.wrapping_add(1 as guint))
                as ::core::ffi::c_uint,
        ) as guint as guint;
        n = m;
        j = 0 as guint;
        while (j as gsize) < input_length {
            if *input.offset(j as isize) < n {
                delta = delta.wrapping_add(1);
                if delta == 0 as guint {
                    current_block = 16910374505238418596;
                    break 's_74;
                }
            } else if *input.offset(j as isize) == n {
                q = delta;
                k = PUNYCODE_BASE as guint;
                loop {
                    if k <= bias {
                        t = PUNYCODE_TMIN as guint;
                    } else if k >= bias.wrapping_add(PUNYCODE_TMAX as guint) {
                        t = PUNYCODE_TMAX as guint;
                    } else {
                        t = k.wrapping_sub(bias);
                    }
                    if q < t {
                        break;
                    }
                    digit = t.wrapping_add(
                        q.wrapping_sub(t)
                            .wrapping_rem((PUNYCODE_BASE as guint).wrapping_sub(t)),
                    );
                    safe_c2rust_g_string_append_c_inline(output, safe_c2rust_encode_digit(digit));
                    q = q
                        .wrapping_sub(t)
                        .wrapping_div((PUNYCODE_BASE as guint).wrapping_sub(t));
                    k = k.wrapping_add(PUNYCODE_BASE as guint);
                }
                safe_c2rust_g_string_append_c_inline(output, safe_c2rust_encode_digit(q));
                bias = safe_c2rust_adapt(
                    delta,
                    handled_chars.wrapping_add(1 as guint),
                    (handled_chars == num_basic_chars) as ::core::ffi::c_int,
                );
                delta = 0 as guint;
                handled_chars = handled_chars.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        delta = delta.wrapping_add(1);
        n = n.wrapping_add(1);
    }
    match current_block {
        721385680381463314 => {
            success = TRUE as gboolean;
        }
        _ => {}
    }
    g_free(input as gpointer);
    return success;
}
unsafe extern "C" fn safe_c2rust_remove_junk(mut str: *const gchar, mut len: gint) -> *mut gchar {
    let mut cleaned: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut ch: gunichar = 0;
    p = str;
    while if len == -(1 as ::core::ffi::c_int) {
        *p as ::core::ffi::c_int
    } else {
        (p < str.offset(len as isize)) as ::core::ffi::c_int
    } != 0
    {
        ch = g_utf8_get_char(p);
        if ch == 0xad as gunichar
            || ch == 0x1806 as gunichar
            || ch == 0x200b as gunichar
            || ch == 0x2060 as gunichar
            || ch == 0xfeff as gunichar
            || ch == 0x34f as gunichar
            || ch == 0x180b as gunichar
            || ch == 0x180c as gunichar
            || ch == 0x180d as gunichar
            || ch == 0x200c as gunichar
            || ch == 0x200d as gunichar
            || ch >= 0xfe00 as gunichar && ch <= 0xfe0f as gunichar
        {
            if cleaned.is_null() {
                cleaned = g_string_new(::core::ptr::null::<gchar>());
                safe_c2rust_g_string_append_len_inline(
                    cleaned,
                    str as *const ::core::ffi::c_char,
                    p.offset_from(str) as gssize,
                );
            }
        } else if !cleaned.is_null() {
            g_string_append_unichar(cleaned, ch);
        }
        p = p.offset(
            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
    }
    if !cleaned.is_null() {
        return if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(cleaned, 0 as gboolean)
            } else {
                g_string_free_and_steal(cleaned)
            }
        } else {
            g_string_free(cleaned, 0 as gboolean)
        };
    } else {
        return ::core::ptr::null_mut::<gchar>();
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_contains_uppercase_letters(
    mut str: *const gchar,
    mut len: gint,
) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    p = str;
    while if len == -(1 as ::core::ffi::c_int) {
        *p as ::core::ffi::c_int
    } else {
        (p < str.offset(len as isize)) as ::core::ffi::c_int
    } != 0
    {
        if g_unichar_isupper(g_utf8_get_char(p)) != 0 {
            return TRUE;
        }
        p = p.offset(
            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
    }
    return FALSE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_contains_non_ascii(
    mut str: *const gchar,
    mut len: gint,
) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    p = str;
    while if len == -(1 as ::core::ffi::c_int) {
        *p as ::core::ffi::c_int
    } else {
        (p < str.offset(len as isize)) as ::core::ffi::c_int
    } != 0
    {
        if *p as guchar as ::core::ffi::c_int > 0x80 as ::core::ffi::c_int {
            return TRUE;
        }
        p = p.offset(1);
    }
    return FALSE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_idna_is_prohibited(mut ch: gunichar) -> gboolean {
    match g_unichar_type(ch) as ::core::ffi::c_uint {
        0 | 1 | 2 | 3 | 4 | 27 | 28 | 29 => return TRUE,
        26 => {
            if ch == 0xfffc as gunichar
                || ch == 0xfffd as gunichar
                || ch >= 0x2ff0 as gunichar && ch <= 0x2ffb as gunichar
            {
                return TRUE;
            }
            return FALSE;
        }
        12 => {
            if ch == 0x340 as gunichar || ch == 0x341 as gunichar {
                return TRUE;
            }
            return FALSE;
        }
        _ => return FALSE,
    };
}
unsafe extern "C" fn safe_c2rust_nameprep(
    mut hostname: *const gchar,
    mut len: gint,
    mut is_unicode: *mut gboolean,
) -> *mut gchar {
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    name = safe_c2rust_remove_junk(hostname, len);
    if !name.is_null() {
        tmp = name;
        len = -(1 as ::core::ffi::c_int) as gint;
    } else {
        name = hostname as *mut gchar;
    }
    if safe_c2rust_contains_uppercase_letters(name, len) != 0 {
        name = g_utf8_strdown(name, len as gssize);
        g_free(tmp as gpointer);
        tmp = name;
        len = -(1 as ::core::ffi::c_int) as gint;
    }
    if safe_c2rust_contains_non_ascii(name, len) == 0 {
        *is_unicode = FALSE as gboolean;
        if name == hostname as *mut gchar {
            return if len == -(1 as ::core::ffi::c_int) {
                safe_c2rust_g_strdup_inline(hostname as *const ::core::ffi::c_char) as *mut gchar
            } else {
                g_strndup(hostname, len as gsize)
            };
        } else {
            return name;
        }
    }
    *is_unicode = TRUE as gboolean;
    name = g_utf8_normalize(name, len as gssize, G_NORMALIZE_NFKC);
    g_free(tmp as gpointer);
    tmp = name;
    if name.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_contains_uppercase_letters(name, -(1 as gint)) != 0 {
        name = g_utf8_strdown(name, -(1 as ::core::ffi::c_int) as gssize);
        g_free(tmp as gpointer);
        tmp = name;
    }
    p = name;
    while *p != 0 {
        if safe_c2rust_idna_is_prohibited(g_utf8_get_char(p)) != 0 {
            name = ::core::ptr::null_mut::<gchar>();
            g_free(tmp as gpointer);
            break;
        } else {
            p = p.offset(
                *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char as *mut gchar;
        }
    }
    return name;
}
unsafe extern "C" fn safe_c2rust_idna_end_of_label(mut str: *const gchar) -> *const gchar {
    while *str != 0 {
        if *str.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
            == '.' as i32
            || *str.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                == 0xe3 as ::core::ffi::c_int
                && *str.offset(1 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
                && *str.offset(2 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0x82 as ::core::ffi::c_int
            || *str.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                == 0xef as ::core::ffi::c_int
                && *str.offset(1 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0xbc as ::core::ffi::c_int
                && *str.offset(2 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0x8e as ::core::ffi::c_int
            || *str.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                == 0xef as ::core::ffi::c_int
                && *str.offset(1 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0xbd as ::core::ffi::c_int
                && *str.offset(2 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0xa1 as ::core::ffi::c_int
        {
            return str;
        }
        str = str.offset(
            *safe_c2rust_g_utf8_skip.offset(*(str as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
    }
    return str;
}
unsafe extern "C" fn safe_c2rust_get_hostname_max_length_bytes() -> gsize {
    let mut max: glong = sysconf(_SC_HOST_NAME_MAX as ::core::ffi::c_int) as glong;
    if max > 0 as glong {
        return max as gsize;
    }
    return HOST_NAME_MAX as gsize;
}
unsafe extern "C" fn safe_c2rust_strlen_greater_than(
    mut str: *const gchar,
    mut comparison_length: gsize,
) -> gboolean {
    let mut i: gsize = 0;
    i = 0 as gsize;
    while *str.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
        if i > comparison_length {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hostname_to_ascii(mut hostname: *const gchar) -> *mut gchar {
    let mut current_block: u64;
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut label: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut out: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut llen: gssize = 0;
    let mut oldlen: gssize = 0;
    let mut unicode: gboolean = 0;
    let mut hostname_max_length_bytes: gsize = safe_c2rust_get_hostname_max_length_bytes();
    if hostname_max_length_bytes <= G_MAXSIZE.wrapping_div(4 as ::core::ffi::c_ulong)
        && safe_c2rust_strlen_greater_than(
            hostname,
            (4 as gsize).wrapping_mul(
                (if 255 as gsize > hostname_max_length_bytes {
                    255 as gsize
                } else {
                    hostname_max_length_bytes
                }),
            ),
        ) != 0
    {
        return ::core::ptr::null_mut::<gchar>();
    }
    name = safe_c2rust_nameprep(hostname, -(1 as gint), &raw mut unicode);
    label = name;
    if name.is_null() || unicode == 0 {
        return name;
    }
    out = g_string_new(::core::ptr::null::<gchar>());
    loop {
        unicode = FALSE as gboolean;
        p = label;
        while *p as ::core::ffi::c_int != 0
            && !(*p.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                == '.' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0xe3 as ::core::ffi::c_int
                    && *p.offset(1 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                        == 0x80 as ::core::ffi::c_int
                    && *p.offset(2 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                        == 0x82 as ::core::ffi::c_int
                || *p.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0xef as ::core::ffi::c_int
                    && *p.offset(1 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                        == 0xbc as ::core::ffi::c_int
                    && *p.offset(2 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                        == 0x8e as ::core::ffi::c_int
                || *p.offset(0 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                    == 0xef as ::core::ffi::c_int
                    && *p.offset(1 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                        == 0xbd as ::core::ffi::c_int
                    && *p.offset(2 as ::core::ffi::c_int as isize) as guchar as ::core::ffi::c_int
                        == 0xa1 as ::core::ffi::c_int)
        {
            if *p as guchar as ::core::ffi::c_int > 0x80 as ::core::ffi::c_int {
                unicode = TRUE as gboolean;
            }
            p = p.offset(1);
        }
        oldlen = (*out).len as gssize;
        llen = p.offset_from(label) as ::core::ffi::c_long as gssize;
        if unicode != 0 {
            if strncmp(
                label,
                IDNA_ACE_PREFIX.as_ptr(),
                IDNA_ACE_PREFIX_LEN as size_t,
            ) == 0
            {
                current_block = 9222955515692011121;
                break;
            }
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"xn--\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        out,
                        __val,
                        if ({
                            let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_8 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_8 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_8
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
                    out,
                    b"xn--\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            if safe_c2rust_punycode_encode(label, llen as gsize, out) == 0 {
                current_block = 9222955515692011121;
                break;
            }
        } else {
            safe_c2rust_g_string_append_len_inline(out, label, llen);
        }
        if (*out).len.wrapping_sub(oldlen as gsize) > 63 as gsize {
            current_block = 9222955515692011121;
            break;
        }
        label = label.offset(llen as isize);
        if *label != 0 {
            label = label.offset(
                *safe_c2rust_g_utf8_skip.offset(*(label as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char as *mut gchar;
        }
        if *label != 0 {
            safe_c2rust_g_string_append_c_inline(out, '.' as i32 as gchar);
        }
        if !(*label != 0) {
            current_block = 11307063007268554308;
            break;
        }
    }
    match current_block {
        9222955515692011121 => {
            g_free(name as gpointer);
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(out, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                } else {
                    g_string_free_and_steal(out);
                };
            } else {
                g_string_free(out, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            };
            return ::core::ptr::null_mut::<gchar>();
        }
        _ => {
            g_free(name as gpointer);
            return if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(out, 0 as gboolean)
                } else {
                    g_string_free_and_steal(out)
                }
            } else {
                g_string_free(out, 0 as gboolean)
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hostname_is_non_ascii(
    mut hostname: *const gchar,
) -> gboolean {
    return safe_c2rust_contains_non_ascii(hostname, -(1 as gint));
}
unsafe extern "C" fn safe_c2rust_punycode_decode(
    mut input: *const gchar,
    mut input_length: gsize,
    mut output: *mut GString,
) -> gboolean {
    let mut current_block: u64;
    let mut output_chars: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut n: gunichar = 0;
    let mut i: guint = 0;
    let mut bias: guint = 0;
    let mut oldi: guint = 0;
    let mut w: guint = 0;
    let mut k: guint = 0;
    let mut digit: guint = 0;
    let mut t: guint = 0;
    let mut split: *const gchar = ::core::ptr::null::<gchar>();
    n = PUNYCODE_INITIAL_N as gunichar;
    i = 0 as guint;
    bias = PUNYCODE_INITIAL_BIAS as guint;
    split = input
        .offset(input_length as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while split > input && *split as ::core::ffi::c_int != '-' as i32 {
        split = split.offset(-1);
    }
    if split > input {
        output_chars = g_array_sized_new(
            FALSE,
            FALSE,
            ::core::mem::size_of::<gunichar>() as guint,
            split.offset_from(input) as ::core::ffi::c_long as guint,
        );
        input_length = input_length.wrapping_sub(
            (split.offset_from(input) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as gsize,
        );
        loop {
            if !(input < split) {
                current_block = 2968425633554183086;
                break;
            }
            let fresh1 = input;
            input = input.offset(1);
            let mut ch: gunichar = *fresh1 as gunichar;
            if !(ch < 0x80 as guint) {
                current_block = 249110143584110561;
                break;
            }
            g_array_append_vals(output_chars, &raw mut ch as gconstpointer, 1 as guint);
        }
        match current_block {
            249110143584110561 => {}
            _ => {
                input = input.offset(1);
                current_block = 4166486009154926805;
            }
        }
    } else {
        output_chars = g_array_new(FALSE, FALSE, ::core::mem::size_of::<gunichar>() as guint);
        current_block = 4166486009154926805;
    }
    '_fail: loop {
        match current_block {
            249110143584110561 => {
                g_array_free(output_chars, TRUE);
                return FALSE;
            }
            _ => {
                if input_length != 0 {
                    oldi = i;
                    w = 1 as guint;
                    k = PUNYCODE_BASE as guint;
                    loop {
                        let fresh2 = input_length;
                        input_length = input_length.wrapping_sub(1);
                        if fresh2 == 0 {
                            current_block = 249110143584110561;
                            continue '_fail;
                        }
                        let fresh3 = input;
                        input = input.offset(1);
                        digit = safe_c2rust_decode_digit(*fresh3);
                        if digit >= PUNYCODE_BASE as guint {
                            current_block = 249110143584110561;
                            continue '_fail;
                        }
                        if digit > G_MAXUINT.wrapping_sub(i).wrapping_div(w) {
                            current_block = 249110143584110561;
                            continue '_fail;
                        }
                        i = i.wrapping_add(digit.wrapping_mul(w));
                        if k <= bias {
                            t = PUNYCODE_TMIN as guint;
                        } else if k >= bias.wrapping_add(PUNYCODE_TMAX as guint) {
                            t = PUNYCODE_TMAX as guint;
                        } else {
                            t = k.wrapping_sub(bias);
                        }
                        if digit < t {
                            break;
                        }
                        if w > G_MAXUINT.wrapping_div((PUNYCODE_BASE as guint).wrapping_sub(t)) {
                            current_block = 249110143584110561;
                            continue '_fail;
                        }
                        w = w.wrapping_mul((PUNYCODE_BASE as guint).wrapping_sub(t));
                        k = k.wrapping_add(PUNYCODE_BASE as guint);
                    }
                    bias = safe_c2rust_adapt(
                        i.wrapping_sub(oldi),
                        (*output_chars).len.wrapping_add(1 as guint),
                        (oldi == 0 as guint) as ::core::ffi::c_int,
                    );
                    if i.wrapping_div((*output_chars).len.wrapping_add(1 as guint))
                        > G_MAXUINT.wrapping_sub(n)
                    {
                        current_block = 249110143584110561;
                        continue;
                    }
                    n = (n as ::core::ffi::c_uint)
                        .wrapping_add(i.wrapping_div((*output_chars).len.wrapping_add(1 as guint))
                            as ::core::ffi::c_uint) as gunichar as gunichar;
                    i = i.wrapping_rem((*output_chars).len.wrapping_add(1 as guint));
                    let fresh4 = i;
                    i = i.wrapping_add(1);
                    g_array_insert_vals(
                        output_chars,
                        fresh4,
                        &raw mut n as gconstpointer,
                        1 as guint,
                    );
                    current_block = 4166486009154926805;
                } else {
                    i = 0 as guint;
                    while i < (*output_chars).len {
                        g_string_append_unichar(
                            output,
                            *((*output_chars).data as *mut ::core::ffi::c_void as *mut gunichar)
                                .offset(i as isize),
                        );
                        i = i.wrapping_add(1);
                    }
                    g_array_free(output_chars, TRUE);
                    return TRUE;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hostname_to_unicode(
    mut hostname: *const gchar,
) -> *mut gchar {
    let mut out: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut llen: gssize = 0;
    let mut hostname_max_length_bytes: gsize = safe_c2rust_get_hostname_max_length_bytes();
    if hostname_max_length_bytes <= G_MAXSIZE.wrapping_div(4 as ::core::ffi::c_ulong)
        && safe_c2rust_strlen_greater_than(
            hostname,
            (4 as gsize).wrapping_mul(
                (if 255 as gsize > hostname_max_length_bytes {
                    255 as gsize
                } else {
                    hostname_max_length_bytes
                }),
            ),
        ) != 0
    {
        return ::core::ptr::null_mut::<gchar>();
    }
    out = g_string_new(::core::ptr::null::<gchar>());
    loop {
        llen = safe_c2rust_idna_end_of_label(hostname).offset_from(hostname) as ::core::ffi::c_long
            as gssize;
        if g_ascii_strncasecmp(
            hostname,
            IDNA_ACE_PREFIX.as_ptr() as *const gchar,
            IDNA_ACE_PREFIX_LEN as gsize,
        ) == 0
        {
            hostname = hostname.offset(IDNA_ACE_PREFIX_LEN as isize);
            llen -= IDNA_ACE_PREFIX_LEN as gssize;
            if safe_c2rust_punycode_decode(hostname, llen as gsize, out) == 0 {
                if 0 != 0 {
                    if 0 as ::core::ffi::c_int == 0 {
                        g_string_free(out, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                    } else {
                        g_string_free_and_steal(out);
                    };
                } else {
                    g_string_free(out, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                };
                return ::core::ptr::null_mut::<gchar>();
            }
        } else {
            let mut unicode: gboolean = 0;
            let mut canonicalized: *mut gchar =
                safe_c2rust_nameprep(hostname, llen as gint, &raw mut unicode);
            if canonicalized.is_null() {
                if 0 != 0 {
                    if 0 as ::core::ffi::c_int == 0 {
                        g_string_free(out, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                    } else {
                        g_string_free_and_steal(out);
                    };
                } else {
                    g_string_free(out, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                };
                return ::core::ptr::null_mut::<gchar>();
            }
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = canonicalized;
                    safe_c2rust_g_string_append_len_inline(
                        out,
                        __val,
                        if ({
                            let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_9 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_9 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_9
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
                    out,
                    canonicalized,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            g_free(canonicalized as gpointer);
        }
        hostname = hostname.offset(llen as isize);
        if *hostname != 0 {
            hostname = hostname.offset(
                *safe_c2rust_g_utf8_skip.offset(*(hostname as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
        }
        if *hostname != 0 {
            safe_c2rust_g_string_append_c_inline(out, '.' as i32 as gchar);
        }
        if !(*hostname != 0) {
            break;
        }
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(out, 0 as gboolean)
        } else {
            g_string_free_and_steal(out)
        }
    } else {
        g_string_free(out, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hostname_is_ascii_encoded(
    mut hostname: *const gchar,
) -> gboolean {
    loop {
        if g_ascii_strncasecmp(
            hostname,
            IDNA_ACE_PREFIX.as_ptr() as *const gchar,
            IDNA_ACE_PREFIX_LEN as gsize,
        ) == 0
        {
            return TRUE;
        }
        hostname = safe_c2rust_idna_end_of_label(hostname);
        if *hostname != 0 {
            hostname = hostname.offset(
                *safe_c2rust_g_utf8_skip.offset(*(hostname as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
        }
        if *hostname == 0 {
            return FALSE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hostname_is_ip_address(
    mut hostname: *const gchar,
) -> gboolean {
    let mut current_block: u64;
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut nsegments: gint = 0;
    let mut octet: gint = 0;
    p = hostname as *mut ::core::ffi::c_char as *mut gchar;
    if !strchr(p, ':' as i32).is_null() {
        let mut skipped: gboolean = 0;
        nsegments = 0 as ::core::ffi::c_int as gint;
        skipped = FALSE as gboolean;
        loop {
            if !(*p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != '%' as i32
                && nsegments < 8 as ::core::ffi::c_int)
            {
                current_block = 4495394744059808450;
                break;
            }
            if p != hostname as *mut ::core::ffi::c_char
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
                    && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ':' as i32
            {
                if *p as ::core::ffi::c_int != ':' as i32 {
                    return FALSE;
                }
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == ':' as i32 && skipped == 0 {
                skipped = TRUE as gboolean;
                nsegments += 1;
                if *p.offset(1 as ::core::ffi::c_int as isize) == 0 {
                    p = p.offset(1);
                }
            } else {
                end = p;
                while *safe_c2rust_g_ascii_table.offset(*end as guchar as isize)
                    as ::core::ffi::c_int
                    & G_ASCII_XDIGIT as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    end = end.offset(1);
                }
                if end == p || end > p.offset(4 as ::core::ffi::c_int as isize) {
                    return FALSE;
                }
                if *end as ::core::ffi::c_int == '.' as i32 {
                    if nsegments == 6 as ::core::ffi::c_int && skipped == 0
                        || nsegments <= 6 as ::core::ffi::c_int && skipped != 0
                    {
                        current_block = 2801561592510271240;
                        break;
                    }
                    return FALSE;
                } else {
                    nsegments += 1;
                    p = end;
                }
            }
        }
        match current_block {
            2801561592510271240 => {}
            _ => {
                return ((*p == 0
                    || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '%' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0)
                    && (nsegments == 8 as ::core::ffi::c_int || skipped != 0))
                    as ::core::ffi::c_int;
            }
        }
    }
    nsegments = 0 as ::core::ffi::c_int as gint;
    while nsegments < 4 as ::core::ffi::c_int {
        if nsegments != 0 as ::core::ffi::c_int {
            if *p as ::core::ffi::c_int != '.' as i32 {
                return FALSE;
            }
            p = p.offset(1);
        }
        octet = 0 as ::core::ffi::c_int as gint;
        if *p as ::core::ffi::c_int == '0' as i32 {
            end = p.offset(1 as ::core::ffi::c_int as isize);
        } else {
            end = p;
            while *safe_c2rust_g_ascii_table.offset(*end as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_DIGIT as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                octet = (10 as ::core::ffi::c_int * octet as ::core::ffi::c_int
                    + (*end as ::core::ffi::c_int - '0' as i32)) as gint;
                if octet > 255 as ::core::ffi::c_int {
                    break;
                }
                end = end.offset(1);
            }
        }
        if end == p
            || end > p.offset(3 as ::core::ffi::c_int as isize)
            || octet > 255 as ::core::ffi::c_int
        {
            return FALSE;
        }
        p = end;
        nsegments += 1;
    }
    return (*p == 0) as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
