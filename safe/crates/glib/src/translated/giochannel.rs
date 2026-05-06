use ::c2rust_bitfields;
extern "C" {
    pub type _GIConv;
    pub type _GMainContext;
    pub type _GSourcePrivate;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_convert_error_quark() -> GQuark;
    fn g_iconv_open(to_codeset: *const gchar, from_codeset: *const gchar) -> GIConv;
    fn g_iconv(
        converter: GIConv,
        inbuf: *mut *mut gchar,
        inbytes_left: *mut gsize,
        outbuf: *mut *mut gchar,
        outbytes_left: *mut gsize,
    ) -> gsize;
    fn g_iconv_close(converter: GIConv) -> gint;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_get_char_validated(p: *const gchar, max_len: gssize) -> gunichar;
    fn g_unichar_to_utf8(c: gunichar, outbuf: *mut gchar) -> gint;
    fn g_utf8_validate_len(str: *const gchar, max_len: gsize, end: *mut *const gchar) -> gboolean;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_set_size(string: *mut GString, len: gsize) -> *mut GString;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_prepend_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_erase(string: *mut GString, pos: gssize, len: gssize) -> *mut GString;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_CONVERT_ERROR_EMBEDDED_NUL: C2RustUnnamed = 7;
pub const G_CONVERT_ERROR_NO_MEMORY: C2RustUnnamed = 6;
pub const G_CONVERT_ERROR_NOT_ABSOLUTE_PATH: C2RustUnnamed = 5;
pub const G_CONVERT_ERROR_BAD_URI: C2RustUnnamed = 4;
pub const G_CONVERT_ERROR_PARTIAL_INPUT: C2RustUnnamed = 3;
pub const G_CONVERT_ERROR_FAILED: C2RustUnnamed = 2;
pub const G_CONVERT_ERROR_ILLEGAL_SEQUENCE: C2RustUnnamed = 1;
pub const G_CONVERT_ERROR_NO_CONVERSION: C2RustUnnamed = 0;
pub type GIConv = *mut _GIConv;
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
pub type gunichar = guint32;
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
pub type GIOError = ::core::ffi::c_uint;
pub const G_IO_ERROR_UNKNOWN: GIOError = 3;
pub const G_IO_ERROR_INVAL: GIOError = 2;
pub const G_IO_ERROR_AGAIN: GIOError = 1;
pub const G_IO_ERROR_NONE: GIOError = 0;
pub type GIOChannelError = ::core::ffi::c_uint;
pub const G_IO_CHANNEL_ERROR_FAILED: GIOChannelError = 8;
pub const G_IO_CHANNEL_ERROR_PIPE: GIOChannelError = 7;
pub const G_IO_CHANNEL_ERROR_OVERFLOW: GIOChannelError = 6;
pub const G_IO_CHANNEL_ERROR_NXIO: GIOChannelError = 5;
pub const G_IO_CHANNEL_ERROR_NOSPC: GIOChannelError = 4;
pub const G_IO_CHANNEL_ERROR_ISDIR: GIOChannelError = 3;
pub const G_IO_CHANNEL_ERROR_IO: GIOChannelError = 2;
pub const G_IO_CHANNEL_ERROR_INVAL: GIOChannelError = 1;
pub const G_IO_CHANNEL_ERROR_FBIG: GIOChannelError = 0;
pub type GIOFunc =
    Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean>;
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
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_io_channel_ref\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4;
pub const EIO: ::core::ffi::c_int = 5;
pub const ENXIO: ::core::ffi::c_int = 6;
pub const E2BIG: ::core::ffi::c_int = 7;
pub const EBADF: ::core::ffi::c_int = 9;
pub const EFAULT: ::core::ffi::c_int = 14;
pub const EISDIR: ::core::ffi::c_int = 21;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EFBIG: ::core::ffi::c_int = 27;
pub const ENOSPC: ::core::ffi::c_int = 28;
pub const EPIPE: ::core::ffi::c_int = 32;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const G_MAXLONG: ::core::ffi::c_long = LONG_MAX;
pub const G_MAXSSIZE: ::core::ffi::c_long = G_MAXLONG;
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
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const G_IO_NICE_BUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAX_CHAR_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_init(mut channel: *mut GIOChannel) {
    (*channel).ref_count = 1 as ::core::ffi::c_int as gint;
    (*channel).encoding =
        safe_c2rust_g_strdup_inline(b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    (*channel).line_term = ::core::ptr::null_mut::<gchar>();
    (*channel).line_term_len = 0 as guint;
    (*channel).buf_size = G_IO_NICE_BUF_SIZE as gsize;
    (*channel).read_cd = -(1 as ::core::ffi::c_int) as GIConv;
    (*channel).write_cd = -(1 as ::core::ffi::c_int) as GIConv;
    (*channel).read_buf = ::core::ptr::null_mut::<GString>();
    (*channel).encoded_read_buf = ::core::ptr::null_mut::<GString>();
    (*channel).write_buf = ::core::ptr::null_mut::<GString>();
    (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
    (*channel).set_use_buffer(TRUE as guint as guint);
    (*channel).set_do_encode(FALSE as guint as guint);
    (*channel).set_close_on_unref(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_ref(
    mut channel: *mut GIOChannel,
) -> *mut GIOChannel {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !channel.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOChannel>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*channel).ref_count;
        (*channel).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*channel).ref_count, 1 as ::core::ffi::c_int);
    return channel;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_unref(mut channel: *mut GIOChannel) {
    let mut is_zero: gboolean = 0;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    is_zero = ({
        if 0 as ::core::ffi::c_int != 0 {
            (*channel).ref_count;
            (*channel).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*channel).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) as gboolean;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if is_zero != 0 {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        if (*channel).close_on_unref() != 0 {
            safe_c2rust_g_io_channel_shutdown(
                channel,
                TRUE,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        } else {
            safe_c2rust_g_io_channel_purge(channel);
        }
        g_free((*channel).encoding as gpointer);
        if (*channel).read_cd != -(1 as ::core::ffi::c_int) as GIConv {
            g_iconv_close((*channel).read_cd);
        }
        if (*channel).write_cd != -(1 as ::core::ffi::c_int) as GIConv {
            g_iconv_close((*channel).write_cd);
        }
        g_free((*channel).line_term as gpointer);
        if !(*channel).read_buf.is_null() {
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(
                        (*channel).read_buf,
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                } else {
                    g_string_free_and_steal((*channel).read_buf);
                };
            } else {
                g_string_free(
                    (*channel).read_buf,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            };
        }
        if !(*channel).write_buf.is_null() {
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(
                        (*channel).write_buf,
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                } else {
                    g_string_free_and_steal((*channel).write_buf);
                };
            } else {
                g_string_free(
                    (*channel).write_buf,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            };
        }
        if !(*channel).encoded_read_buf.is_null() {
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(
                        (*channel).encoded_read_buf,
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                } else {
                    g_string_free_and_steal((*channel).encoded_read_buf);
                };
            } else {
                g_string_free(
                    (*channel).encoded_read_buf,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            };
        }
        (*(*channel).funcs)
            .io_free
            .expect("non-null function pointer")(channel);
    }
}
unsafe extern "C" fn safe_c2rust_g_io_error_get_from_g_error(
    mut status: GIOStatus,
    mut err: *mut GError,
) -> GIOError {
    match status as ::core::ffi::c_uint {
        1 | 2 => return G_IO_ERROR_NONE,
        3 => return G_IO_ERROR_AGAIN,
        0 => {
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if !err.is_null() {
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
                    b"err != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return G_IO_ERROR_UNKNOWN;
            }
            if (*err).domain != safe_c2rust_g_io_channel_error_quark() {
                return G_IO_ERROR_UNKNOWN;
            }
            match (*err).code {
                1 => return G_IO_ERROR_INVAL,
                _ => return G_IO_ERROR_UNKNOWN,
            }
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_read(
    mut channel: *mut GIOChannel,
    mut buf: *mut gchar,
    mut count: gsize,
    mut bytes_read: *mut gsize,
) -> GIOError {
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut error: GIOError = G_IO_ERROR_NONE;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_ERROR_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !bytes_read.is_null() {
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
            b"bytes_read != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_ERROR_UNKNOWN;
    }
    if count == 0 as gsize {
        if !bytes_read.is_null() {
            *bytes_read = 0 as gsize;
        }
        return G_IO_ERROR_NONE;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_ERROR_UNKNOWN;
    }
    status = (*(*channel).funcs)
        .io_read
        .expect("non-null function pointer")(
        channel, buf, count, bytes_read, &raw mut err
    );
    error = safe_c2rust_g_io_error_get_from_g_error(status, err);
    if !err.is_null() {
        g_error_free(err);
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_write(
    mut channel: *mut GIOChannel,
    mut buf: *const gchar,
    mut count: gsize,
    mut bytes_written: *mut gsize,
) -> GIOError {
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut error: GIOError = G_IO_ERROR_NONE;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_ERROR_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !bytes_written.is_null() {
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
            b"bytes_written != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_ERROR_UNKNOWN;
    }
    status = (*(*channel).funcs)
        .io_write
        .expect("non-null function pointer")(
        channel, buf, count, bytes_written, &raw mut err
    );
    error = safe_c2rust_g_io_error_get_from_g_error(status, err);
    if !err.is_null() {
        g_error_free(err);
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_seek(
    mut channel: *mut GIOChannel,
    mut offset: gint64,
    mut type_0: GSeekType,
) -> GIOError {
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut error: GIOError = G_IO_ERROR_NONE;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_ERROR_UNKNOWN;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*channel).is_seekable() != 0 {
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
            b"channel->is_seekable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_ERROR_UNKNOWN;
    }
    match type_0 as ::core::ffi::c_uint {
        0 | 1 | 2 => {}
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"g_io_channel_seek: unknown seek type\0" as *const u8 as *const gchar,
            );
            return G_IO_ERROR_UNKNOWN;
        }
    }
    status = (*(*channel).funcs)
        .io_seek
        .expect("non-null function pointer")(channel, offset, type_0, &raw mut err);
    error = safe_c2rust_g_io_error_get_from_g_error(status, err);
    if !err.is_null() {
        g_error_free(err);
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_close(mut channel: *mut GIOChannel) {
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_io_channel_purge(channel);
    (*(*channel).funcs)
        .io_close
        .expect("non-null function pointer")(channel, &raw mut err);
    if !err.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error closing channel: %s\0" as *const u8 as *const gchar,
            (*err).message,
        );
        g_error_free(err);
    }
    (*channel).set_close_on_unref(FALSE as guint as guint);
    (*channel).set_is_readable(FALSE as guint as guint);
    (*channel).set_is_writeable(FALSE as guint as guint);
    (*channel).set_is_seekable(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_shutdown(
    mut channel: *mut GIOChannel,
    mut flush: gboolean,
    mut err: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut result: GIOStatus = G_IO_STATUS_ERROR;
    let mut tmperr: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if err.is_null() || (*err).is_null() {
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
            b"err == NULL || *err == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if !(*channel).write_buf.is_null() && (*(*channel).write_buf).len > 0 as gsize {
        if flush != 0 {
            let mut flags: GIOFlags = G_IO_FLAG_NONE;
            flags = safe_c2rust_g_io_channel_get_flags(channel);
            safe_c2rust_g_io_channel_set_flags(
                channel,
                (flags as ::core::ffi::c_uint
                    & !(G_IO_FLAG_NONBLOCK as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    as GIOFlags,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            result = safe_c2rust_g_io_channel_flush(channel, &raw mut tmperr);
        } else {
            result = G_IO_STATUS_NORMAL;
        }
        safe_c2rust_g_string_truncate_inline((*channel).write_buf, 0 as gsize);
    } else {
        result = G_IO_STATUS_NORMAL;
    }
    if (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != '\0' as i32
    {
        if flush != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Partial character at end of write buffer not flushed.\0" as *const u8
                    as *const gchar,
            );
        }
        (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
    }
    status = (*(*channel).funcs)
        .io_close
        .expect("non-null function pointer")(channel, err);
    (*channel).set_close_on_unref(FALSE as guint as guint);
    (*channel).set_is_readable(FALSE as guint as guint);
    (*channel).set_is_writeable(FALSE as guint as guint);
    (*channel).set_is_seekable(FALSE as guint as guint);
    if status as ::core::ffi::c_uint
        != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        g_clear_error(&raw mut tmperr);
        return status;
    } else if result as ::core::ffi::c_uint
        != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        g_propagate_error(err, tmperr);
        return result;
    } else {
        return G_IO_STATUS_NORMAL;
    };
}
unsafe extern "C" fn safe_c2rust_g_io_channel_purge(mut channel: *mut GIOChannel) {
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*channel).write_buf.is_null() && (*(*channel).write_buf).len > 0 as gsize {
        let mut flags: GIOFlags = G_IO_FLAG_NONE;
        flags = safe_c2rust_g_io_channel_get_flags(channel);
        safe_c2rust_g_io_channel_set_flags(
            channel,
            (flags as ::core::ffi::c_uint
                & !(G_IO_FLAG_NONBLOCK as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as GIOFlags,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        status = safe_c2rust_g_io_channel_flush(channel, &raw mut err);
        if !err.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Error flushing string: %s\0" as *const u8 as *const gchar,
                (*err).message,
            );
            g_error_free(err);
        }
    }
    if !(*channel).read_buf.is_null() {
        safe_c2rust_g_string_truncate_inline((*channel).read_buf, 0 as gsize);
    }
    if !(*channel).write_buf.is_null() {
        safe_c2rust_g_string_truncate_inline((*channel).write_buf, 0 as gsize);
    }
    if !(*channel).encoding.is_null() {
        if !(*channel).encoded_read_buf.is_null() {
            safe_c2rust_g_string_truncate_inline((*channel).encoded_read_buf, 0 as gsize);
        }
        if (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != '\0' as i32
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Partial character at end of write buffer not flushed.\0" as *const u8
                    as *const gchar,
            );
            (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_create_watch(
    mut channel: *mut GIOChannel,
    mut condition: GIOCondition,
) -> *mut GSource {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    return (*(*channel).funcs)
        .io_create_watch
        .expect("non-null function pointer")(channel, condition);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_add_watch_full(
    mut channel: *mut GIOChannel,
    mut priority: gint,
    mut condition: GIOCondition,
    mut func: GIOFunc,
    mut user_data: gpointer,
    mut notify: GDestroyNotify,
) -> guint {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut id: guint = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    source = safe_c2rust_g_io_create_watch(channel, condition);
    if priority != G_PRIORITY_DEFAULT {
        g_source_set_priority(source, priority);
    }
    g_source_set_callback(
        source,
        ::core::mem::transmute::<GIOFunc, GSourceFunc>(func),
        user_data,
        notify,
    );
    id = g_source_attach(source, ::core::ptr::null_mut::<GMainContext>());
    g_source_unref(source);
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_add_watch(
    mut channel: *mut GIOChannel,
    mut condition: GIOCondition,
    mut func: GIOFunc,
    mut user_data: gpointer,
) -> guint {
    return safe_c2rust_g_io_add_watch_full(
        channel,
        G_PRIORITY_DEFAULT,
        condition,
        func,
        user_data,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_get_buffer_condition(
    mut channel: *mut GIOChannel,
) -> GIOCondition {
    let mut condition: GIOCondition = 0 as GIOCondition;
    if !(*channel).encoding.is_null() {
        if !(*channel).encoded_read_buf.is_null() && (*(*channel).encoded_read_buf).len > 0 as gsize
        {
            condition = ::core::mem::transmute::<::core::ffi::c_uint, GIOCondition>(
                condition as ::core::ffi::c_uint
                    | G_IO_IN as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
    } else if !(*channel).read_buf.is_null() && (*(*channel).read_buf).len > 0 as gsize {
        condition = ::core::mem::transmute::<::core::ffi::c_uint, GIOCondition>(
            condition as ::core::ffi::c_uint | G_IO_IN as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if !(*channel).write_buf.is_null() && (*(*channel).write_buf).len < (*channel).buf_size {
        condition = ::core::mem::transmute::<::core::ffi::c_uint, GIOCondition>(
            condition as ::core::ffi::c_uint
                | G_IO_OUT as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return condition;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_error_from_errno(
    mut en: gint,
) -> GIOChannelError {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if en != 11 as ::core::ffi::c_int {
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
            b"en != EAGAIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_CHANNEL_ERROR_FAILED;
    }
    match en {
        EBADF => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Invalid file descriptor.\0" as *const u8 as *const gchar,
            );
            return G_IO_CHANNEL_ERROR_FAILED;
        }
        EFAULT => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Buffer outside valid address space.\0" as *const u8 as *const gchar,
            );
            return G_IO_CHANNEL_ERROR_FAILED;
        }
        EFBIG => return G_IO_CHANNEL_ERROR_FBIG,
        EINTR => return G_IO_CHANNEL_ERROR_FAILED,
        EINVAL => return G_IO_CHANNEL_ERROR_INVAL,
        EIO => return G_IO_CHANNEL_ERROR_IO,
        EISDIR => return G_IO_CHANNEL_ERROR_ISDIR,
        ENOSPC => return G_IO_CHANNEL_ERROR_NOSPC,
        ENXIO => return G_IO_CHANNEL_ERROR_NXIO,
        EOVERFLOW => return G_IO_CHANNEL_ERROR_OVERFLOW,
        EPIPE => return G_IO_CHANNEL_ERROR_PIPE,
        _ => return G_IO_CHANNEL_ERROR_FAILED,
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_set_buffer_size(
    mut channel: *mut GIOChannel,
    mut size: gsize,
) {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if size == 0 as gsize {
        size = G_IO_NICE_BUF_SIZE as gsize;
    }
    if size < MAX_CHAR_SIZE as gsize {
        size = MAX_CHAR_SIZE as gsize;
    }
    (*channel).buf_size = size;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_get_buffer_size(
    mut channel: *mut GIOChannel,
) -> gsize {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*channel).buf_size;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_set_line_term(
    mut channel: *mut GIOChannel,
    mut line_term: *const gchar,
    mut length: gint,
) {
    let mut length_unsigned: guint = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if line_term.is_null() || length != 0 as ::core::ffi::c_int {
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
            b"line_term == NULL || length != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if line_term.is_null() {
        length_unsigned = 0 as guint;
    } else if length >= 0 as ::core::ffi::c_int {
        length_unsigned = length as guint;
    } else {
        let mut length_size: gsize = strlen(line_term as *const ::core::ffi::c_char) as gsize;
        if ({
            let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
            if length_size
                <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as gsize
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
                b"length_size <= G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        length_unsigned = length_size as guint;
    }
    g_free((*channel).line_term as gpointer);
    (*channel).line_term = (if !line_term.is_null() {
        g_memdup2(line_term as gconstpointer, length_unsigned as gsize)
    } else {
        NULL_0
    }) as *mut gchar;
    (*channel).line_term_len = length_unsigned;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_get_line_term(
    mut channel: *mut GIOChannel,
    mut length: *mut gint,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if !length.is_null() {
        *length = (*channel).line_term_len as gint;
    }
    return (*channel).line_term;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_set_flags(
    mut channel: *mut GIOChannel,
    mut flags: GIOFlags,
    mut error: *mut *mut GError,
) -> GIOStatus {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    return Some(
        (*(*channel).funcs)
            .io_set_flags
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        channel,
        (flags as ::core::ffi::c_uint
            & G_IO_FLAG_SET_MASK as ::core::ffi::c_int as ::core::ffi::c_uint) as GIOFlags,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_get_flags(
    mut channel: *mut GIOChannel,
) -> GIOFlags {
    let mut flags: GIOFlags = G_IO_FLAG_NONE;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_FLAG_NONE;
    }
    flags = Some(
        (*(*channel).funcs)
            .io_get_flags
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(channel);
    if (*channel).is_seekable() != 0 {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GIOFlags>(
            flags as ::core::ffi::c_uint
                | G_IO_FLAG_IS_SEEKABLE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*channel).is_readable() != 0 {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GIOFlags>(
            flags as ::core::ffi::c_uint
                | G_IO_FLAG_IS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*channel).is_writeable() != 0 {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GIOFlags>(
            flags as ::core::ffi::c_uint
                | G_IO_FLAG_IS_WRITABLE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_set_close_on_unref(
    mut channel: *mut GIOChannel,
    mut do_close: gboolean,
) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*channel).set_close_on_unref(do_close as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_get_close_on_unref(
    mut channel: *mut GIOChannel,
) -> gboolean {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*channel).close_on_unref() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_seek_position(
    mut channel: *mut GIOChannel,
    mut offset: gint64,
    mut type_0: GSeekType,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if (*channel).is_seekable() != 0 {
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
            b"channel->is_seekable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    match type_0 as ::core::ffi::c_uint {
        0 => {
            if (*channel).use_buffer() != 0 {
                if (*channel).do_encode() as ::core::ffi::c_int != 0
                    && !(*channel).encoded_read_buf.is_null()
                    && (*(*channel).encoded_read_buf).len > 0 as gsize
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"Seek type G_SEEK_CUR not allowed for this channel's encoding.\0"
                            as *const u8 as *const gchar,
                    );
                    return G_IO_STATUS_ERROR;
                }
                if !(*channel).read_buf.is_null() {
                    offset = (offset as gsize).wrapping_sub((*(*channel).read_buf).len) as gint64
                        as gint64;
                }
                if !(*channel).encoded_read_buf.is_null() {
                    if ({
                        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
                        if (*(*channel).encoded_read_buf).len == 0 as gsize
                            || (*channel).do_encode() == 0
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
                            b"../original/glib/giochannel.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1112 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"channel->encoded_read_buf->len == 0 || !channel->do_encode\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    offset = (offset as gsize).wrapping_sub((*(*channel).encoded_read_buf).len)
                        as gint64 as gint64;
                }
            }
        }
        1 | 2 => {}
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"g_io_channel_seek_position: unknown seek type\0" as *const u8 as *const gchar,
            );
            return G_IO_STATUS_ERROR;
        }
    }
    if (*channel).use_buffer() != 0 {
        status = safe_c2rust_g_io_channel_flush(channel, error);
        if status as ::core::ffi::c_uint
            != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return status;
        }
    }
    status = (*(*channel).funcs)
        .io_seek
        .expect("non-null function pointer")(channel, offset, type_0, error);
    if status as ::core::ffi::c_uint
        == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*channel).use_buffer() as ::core::ffi::c_int != 0
    {
        if !(*channel).read_buf.is_null() {
            safe_c2rust_g_string_truncate_inline((*channel).read_buf, 0 as gsize);
        }
        if (*channel).read_cd != -(1 as ::core::ffi::c_int) as GIConv {
            g_iconv(
                (*channel).read_cd,
                ::core::ptr::null_mut::<*mut gchar>(),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut gchar>(),
                ::core::ptr::null_mut::<gsize>(),
            );
        }
        if (*channel).write_cd != -(1 as ::core::ffi::c_int) as GIConv {
            g_iconv(
                (*channel).write_cd,
                ::core::ptr::null_mut::<*mut gchar>(),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut gchar>(),
                ::core::ptr::null_mut::<gsize>(),
            );
        }
        if !(*channel).encoded_read_buf.is_null() {
            if ({
                let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
                if (*(*channel).encoded_read_buf).len == 0 as gsize || (*channel).do_encode() == 0 {
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
                    b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1153 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"channel->encoded_read_buf->len == 0 || !channel->do_encode\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_g_string_truncate_inline((*channel).encoded_read_buf, 0 as gsize);
        }
        if (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != '\0' as i32
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Partial character at end of write buffer not flushed.\0" as *const u8
                    as *const gchar,
            );
            (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_flush(
    mut channel: *mut GIOChannel,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut this_time: gsize = 1 as gsize;
    let mut bytes_written: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if (*channel).write_buf.is_null() || (*(*channel).write_buf).len == 0 as gsize {
        return G_IO_STATUS_NORMAL;
    }
    loop {
        if ({
            let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
            if this_time > 0 as gsize {
                _g_boolean_var_44 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_44 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_44
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                1193 as ::core::ffi::c_int,
                G_STRFUNC,
                b"this_time > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        status = (*(*channel).funcs)
            .io_write
            .expect("non-null function pointer")(
            channel,
            (*(*channel).write_buf).str_0.offset(bytes_written as isize),
            (*(*channel).write_buf).len.wrapping_sub(bytes_written),
            &raw mut this_time,
            error,
        );
        bytes_written = bytes_written.wrapping_add(this_time);
        if !(bytes_written < (*(*channel).write_buf).len
            && status as ::core::ffi::c_uint
                == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
    }
    g_string_erase((*channel).write_buf, 0 as gssize, bytes_written as gssize);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_set_buffered(
    mut channel: *mut GIOChannel,
    mut buffered: gboolean,
) {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*channel).encoding.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Need to have NULL encoding to set the buffering state of the channel.\0" as *const u8
                as *const gchar,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if (*channel).read_buf.is_null() || (*(*channel).read_buf).len == 0 as gsize {
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
            b"!channel->read_buf || channel->read_buf->len == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if (*channel).write_buf.is_null() || (*(*channel).write_buf).len == 0 as gsize {
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
            b"!channel->write_buf || channel->write_buf->len == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*channel).set_use_buffer(buffered as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_get_buffered(
    mut channel: *mut GIOChannel,
) -> gboolean {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*channel).use_buffer() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_set_encoding(
    mut channel: *mut GIOChannel,
    mut encoding: *const gchar,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut read_cd: GIConv = ::core::ptr::null_mut::<_GIConv>();
    let mut write_cd: GIConv = ::core::ptr::null_mut::<_GIConv>();
    let mut did_encode: gboolean = 0;
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if (*channel).do_encode() == 0
            || (*channel).encoded_read_buf.is_null()
            || (*(*channel).encoded_read_buf).len == 0 as gsize
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
            b"!channel->do_encode || !channel->encoded_read_buf || channel->encoded_read_buf->len == 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if (*channel).use_buffer() == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Need to set the channel buffered before setting the encoding.\0" as *const u8
                as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Assuming this is what you meant and acting accordingly.\0" as *const u8
                as *const gchar,
        );
        (*channel).set_use_buffer(TRUE as guint as guint);
    }
    if (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != '\0' as i32
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Partial character at end of write buffer not flushed.\0" as *const u8 as *const gchar,
        );
        (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
    }
    did_encode = (*channel).do_encode() as gboolean;
    if encoding.is_null()
        || strcmp(
            encoding as *const ::core::ffi::c_char,
            b"UTF8\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            encoding as *const ::core::ffi::c_char,
            b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        (*channel).set_do_encode(FALSE as guint as guint);
        write_cd = -(1 as ::core::ffi::c_int) as GIConv;
        read_cd = write_cd;
    } else {
        let mut err: gint = 0 as gint;
        let mut from_enc: *const gchar = ::core::ptr::null::<gchar>();
        let mut to_enc: *const gchar = ::core::ptr::null::<gchar>();
        if (*channel).is_readable() != 0 {
            read_cd = g_iconv_open(b"UTF-8\0" as *const u8 as *const gchar, encoding);
            if read_cd == -(1 as ::core::ffi::c_int) as GIConv {
                err = *__errno_location() as gint;
                from_enc = encoding;
                to_enc = b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
            }
        } else {
            read_cd = -(1 as ::core::ffi::c_int) as GIConv;
        }
        if (*channel).is_writeable() as ::core::ffi::c_int != 0 && err == 0 as ::core::ffi::c_int {
            write_cd = g_iconv_open(encoding, b"UTF-8\0" as *const u8 as *const gchar);
            if write_cd == -(1 as ::core::ffi::c_int) as GIConv {
                err = *__errno_location() as gint;
                from_enc = b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
                to_enc = encoding;
            }
        } else {
            write_cd = -(1 as ::core::ffi::c_int) as GIConv;
        }
        if err != 0 as ::core::ffi::c_int {
            if ({
                let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
                if !from_enc.is_null() {
                    _g_boolean_var_52 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_52 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_52
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1388 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"from_enc\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if ({
                let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
                if !to_enc.is_null() {
                    _g_boolean_var_53 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_53 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_53
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1389 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"to_enc\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if err == EINVAL {
                g_set_error(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_NO_CONVERSION as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Conversion from character set \xE2\x80\x9C%s\xE2\x80\x9D to \xE2\x80\x9C%s\xE2\x80\x9D is not supported\0"
                            as *const u8 as *const gchar,
                    ),
                    from_enc,
                    to_enc,
                );
            } else {
                g_set_error(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Could not open converter from \xE2\x80\x9C%s\xE2\x80\x9D to \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                            as *const u8 as *const gchar,
                    ),
                    from_enc,
                    to_enc,
                    g_strerror(err),
                );
            }
            if read_cd != -(1 as ::core::ffi::c_int) as GIConv {
                g_iconv_close(read_cd);
            }
            if write_cd != -(1 as ::core::ffi::c_int) as GIConv {
                g_iconv_close(write_cd);
            }
            return G_IO_STATUS_ERROR;
        }
        (*channel).set_do_encode(TRUE as guint as guint);
    }
    if (*channel).read_cd != -(1 as ::core::ffi::c_int) as GIConv {
        g_iconv_close((*channel).read_cd);
    }
    if (*channel).write_cd != -(1 as ::core::ffi::c_int) as GIConv {
        g_iconv_close((*channel).write_cd);
    }
    if !(*channel).encoded_read_buf.is_null() && (*(*channel).encoded_read_buf).len > 0 as gsize {
        if ({
            let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
            if did_encode == 0 {
                _g_boolean_var_54 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_54 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_54
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                1420 as ::core::ffi::c_int,
                G_STRFUNC,
                b"!did_encode\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_string_prepend_len(
            (*channel).read_buf,
            (*(*channel).encoded_read_buf).str_0,
            (*(*channel).encoded_read_buf).len as gssize,
        );
        safe_c2rust_g_string_truncate_inline((*channel).encoded_read_buf, 0 as gsize);
    }
    (*channel).read_cd = read_cd;
    (*channel).write_cd = write_cd;
    g_free((*channel).encoding as gpointer);
    (*channel).encoding =
        safe_c2rust_g_strdup_inline(encoding as *const ::core::ffi::c_char) as *mut gchar;
    return G_IO_STATUS_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_get_encoding(
    mut channel: *mut GIOChannel,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*channel).encoding;
}
unsafe extern "C" fn safe_c2rust_g_io_channel_fill_buffer(
    mut channel: *mut GIOChannel,
    mut err: *mut *mut GError,
) -> GIOStatus {
    let mut read_size: gsize = 0;
    let mut cur_len: gsize = 0;
    let mut oldlen: gsize = 0;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if (*channel).is_seekable() as ::core::ffi::c_int != 0
        && !(*channel).write_buf.is_null()
        && (*(*channel).write_buf).len > 0 as gsize
    {
        status = safe_c2rust_g_io_channel_flush(channel, err);
        if status as ::core::ffi::c_uint
            != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return status;
        }
    }
    if (*channel).is_seekable() as ::core::ffi::c_int != 0
        && (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != '\0' as i32
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Partial character at end of write buffer not flushed.\0" as *const u8 as *const gchar,
        );
        (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
    }
    if (*channel).read_buf.is_null() {
        (*channel).read_buf = g_string_sized_new((*channel).buf_size);
    }
    cur_len = (*(*channel).read_buf).len;
    g_string_set_size(
        (*channel).read_buf,
        (*(*channel).read_buf).len.wrapping_add((*channel).buf_size),
    );
    status = (*(*channel).funcs)
        .io_read
        .expect("non-null function pointer")(
        channel,
        (*(*channel).read_buf).str_0.offset(cur_len as isize),
        (*channel).buf_size,
        &raw mut read_size,
        err,
    );
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if status as ::core::ffi::c_uint
            == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
            || read_size == 0 as gsize
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
            1488 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(status == G_IO_STATUS_NORMAL) || (read_size == 0)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_g_string_truncate_inline((*channel).read_buf, read_size.wrapping_add(cur_len));
    if status as ::core::ffi::c_uint
        != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        && (status as ::core::ffi::c_uint
            != G_IO_STATUS_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*channel).read_buf).len == 0 as gsize)
    {
        return status;
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if (*(*channel).read_buf).len > 0 as gsize {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
            1496 as ::core::ffi::c_int,
            G_STRFUNC,
            b"channel->read_buf->len > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*channel).encoded_read_buf.is_null() {
        oldlen = (*(*channel).encoded_read_buf).len;
    } else {
        oldlen = 0 as gsize;
        if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf = g_string_sized_new((*channel).buf_size);
        }
    }
    if (*channel).do_encode() != 0 {
        let mut errnum: gsize = 0;
        let mut inbytes_left: gsize = 0;
        let mut outbytes_left: gsize = 0;
        let mut inbuf: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut outbuf: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut errval: ::core::ffi::c_int = 0;
        if ({
            let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
            if !(*channel).encoded_read_buf.is_null() {
                _g_boolean_var_58 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_58 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_58
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                1513 as ::core::ffi::c_int,
                G_STRFUNC,
                b"channel->encoded_read_buf\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        loop {
            inbytes_left = (*(*channel).read_buf).len;
            outbytes_left = if (*(*channel).read_buf).len
                > (*(*channel).encoded_read_buf)
                    .allocated_len
                    .wrapping_sub((*(*channel).encoded_read_buf).len)
                    .wrapping_sub(1 as gsize)
            {
                (*(*channel).read_buf).len
            } else {
                (*(*channel).encoded_read_buf)
                    .allocated_len
                    .wrapping_sub((*(*channel).encoded_read_buf).len)
                    .wrapping_sub(1 as gsize)
            };
            outbytes_left = if outbytes_left > 6 as gsize {
                outbytes_left
            } else {
                6 as gsize
            };
            inbuf = (*(*channel).read_buf).str_0;
            g_string_set_size(
                (*channel).encoded_read_buf,
                (*(*channel).encoded_read_buf)
                    .len
                    .wrapping_add(outbytes_left),
            );
            outbuf = (*(*channel).encoded_read_buf)
                .str_0
                .offset((*(*channel).encoded_read_buf).len as isize)
                .offset(-(outbytes_left as isize));
            errnum = g_iconv(
                (*channel).read_cd,
                &raw mut inbuf,
                &raw mut inbytes_left,
                &raw mut outbuf,
                &raw mut outbytes_left,
            );
            errval = *__errno_location();
            if ({
                let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
                if inbuf.offset(inbytes_left as isize)
                    == (*(*channel).read_buf)
                        .str_0
                        .offset((*(*channel).read_buf).len as isize)
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
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1534 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"inbuf + inbytes_left == channel->read_buf->str + channel->read_buf->len\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if ({
                let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
                if outbuf.offset(outbytes_left as isize)
                    == (*(*channel).encoded_read_buf)
                        .str_0
                        .offset((*(*channel).encoded_read_buf).len as isize)
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
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/giochannel.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1536 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"outbuf + outbytes_left == channel->encoded_read_buf->str + channel->encoded_read_buf->len\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            g_string_erase(
                (*channel).read_buf,
                0 as gssize,
                (*(*channel).read_buf).len.wrapping_sub(inbytes_left) as gssize,
            );
            safe_c2rust_g_string_truncate_inline(
                (*channel).encoded_read_buf,
                (*(*channel).encoded_read_buf)
                    .len
                    .wrapping_sub(outbytes_left),
            );
            if !(errnum == -(1 as ::core::ffi::c_int) as gsize) {
                break;
            }
            match errval {
                EINVAL => {
                    if oldlen == (*(*channel).encoded_read_buf).len
                        && status as ::core::ffi::c_uint
                            == G_IO_STATUS_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        status = G_IO_STATUS_EOF;
                    } else {
                        status = G_IO_STATUS_NORMAL;
                    }
                    break;
                }
                E2BIG => {
                    if ({
                        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
                        if inbuf != (*(*channel).read_buf).str_0 {
                            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_61
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/giochannel.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1556 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"inbuf != channel->read_buf->str\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                EILSEQ => {
                    if oldlen < (*(*channel).encoded_read_buf).len {
                        status = G_IO_STATUS_NORMAL;
                    } else {
                        g_set_error_literal(
                            err,
                            g_convert_error_quark(),
                            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Invalid byte sequence in conversion input\0" as *const u8
                                    as *const gchar,
                            ),
                        );
                        return G_IO_STATUS_ERROR;
                    }
                    break;
                }
                _ => {
                    if ({
                        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
                        if errval != 9 as ::core::ffi::c_int {
                            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_62
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/giochannel.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1570 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"errval != EBADF\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    g_set_error(
                        err,
                        g_convert_error_quark(),
                        G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
                        glib_gettext(b"Error during conversion: %s\0" as *const u8 as *const gchar),
                        g_strerror(errval as gint),
                    );
                    return G_IO_STATUS_ERROR;
                }
            }
        }
        if ({
            let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
            if status as ::core::ffi::c_uint
                != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*channel).encoded_read_buf).len > 0 as gsize
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                1577 as ::core::ffi::c_int,
                G_STRFUNC,
                b"(status != G_IO_STATUS_NORMAL) || (channel->encoded_read_buf->len > 0)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if !(*channel).encoding.is_null() {
        let mut nextchar: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut lastchar: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if ({
            let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
            if !(*channel).encoded_read_buf.is_null() {
                _g_boolean_var_64 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_64 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_64
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                1583 as ::core::ffi::c_int,
                G_STRFUNC,
                b"channel->encoded_read_buf\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        nextchar = (*(*channel).read_buf).str_0;
        lastchar = (*(*channel).read_buf)
            .str_0
            .offset((*(*channel).read_buf).len as isize);
        while nextchar < lastchar {
            let mut val_char: gunichar = 0;
            val_char =
                g_utf8_get_char_validated(nextchar, lastchar.offset_from(nextchar) as gssize);
            match val_char {
                4294967294 => {
                    lastchar = nextchar;
                }
                4294967295 => {
                    if oldlen < (*(*channel).encoded_read_buf).len {
                        status = G_IO_STATUS_NORMAL;
                    } else {
                        g_set_error_literal(
                            err,
                            g_convert_error_quark(),
                            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Invalid byte sequence in conversion input\0" as *const u8
                                    as *const gchar,
                            ),
                        );
                        status = G_IO_STATUS_ERROR;
                    }
                    lastchar = nextchar;
                }
                _ => {
                    nextchar = nextchar.offset(
                        *safe_c2rust_g_utf8_skip.offset(*(nextchar as *const guchar) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as *mut ::core::ffi::c_char as *mut gchar;
                }
            }
        }
        if lastchar > (*(*channel).read_buf).str_0 {
            let mut copy_len: gint =
                lastchar.offset_from((*(*channel).read_buf).str_0) as ::core::ffi::c_long as gint;
            safe_c2rust_g_string_append_len_inline(
                (*channel).encoded_read_buf,
                (*(*channel).read_buf).str_0,
                copy_len as gssize,
            );
            g_string_erase((*channel).read_buf, 0 as gssize, copy_len as gssize);
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_read_line(
    mut channel: *mut GIOChannel,
    mut str_return: *mut *mut gchar,
    mut length: *mut gsize,
    mut terminator_pos: *mut gsize,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut got_length: gsize = 0;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !str_return.is_null() {
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
            b"str_return != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if (*channel).is_readable() != 0 {
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
            b"channel->is_readable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    status = safe_c2rust_g_io_channel_read_line_backend(
        channel,
        &raw mut got_length,
        terminator_pos,
        error,
    );
    if !length.is_null()
        && status as ::core::ffi::c_uint
            != G_IO_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *length = got_length;
    }
    if status as ::core::ffi::c_uint
        == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut line: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if ({
            let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
            if !if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            }
            .is_null()
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                1679 as ::core::ffi::c_int,
                G_STRFUNC,
                b"USE_BUF (channel)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        line = g_memdup2(
            (*if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            })
            .str_0 as gconstpointer,
            got_length.wrapping_add(1 as gsize),
        ) as *mut gchar;
        *line.offset(got_length as isize) = '\0' as i32 as gchar;
        *str_return =
            safe_c2rust_g_steal_pointer(&raw mut line as gpointer) as *mut gchar as *mut gchar;
        g_string_erase(
            if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            },
            0 as gssize,
            got_length as gssize,
        );
    } else {
        *str_return = ::core::ptr::null_mut::<gchar>();
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_read_line_string(
    mut channel: *mut GIOChannel,
    mut buffer: *mut GString,
    mut terminator_pos: *mut gsize,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut length: gsize = 0;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !buffer.is_null() {
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
            b"buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if (*channel).is_readable() != 0 {
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
            b"channel->is_readable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if (*buffer).len > 0 as gsize {
        safe_c2rust_g_string_truncate_inline(buffer, 0 as gsize);
    }
    status =
        safe_c2rust_g_io_channel_read_line_backend(channel, &raw mut length, terminator_pos, error);
    if status as ::core::ffi::c_uint
        == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if ({
            let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
            if !if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            }
            .is_null()
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                1727 as ::core::ffi::c_int,
                G_STRFUNC,
                b"USE_BUF (channel)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        safe_c2rust_g_string_append_len_inline(
            buffer,
            (*if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            })
            .str_0,
            length as gssize,
        );
        g_string_erase(
            if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            },
            0 as gssize,
            length as gssize,
        );
    }
    return status;
}
unsafe extern "C" fn safe_c2rust_g_io_channel_read_line_backend(
    mut channel: *mut GIOChannel,
    mut length: *mut gsize,
    mut terminator_pos: *mut gsize,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut current_block: u64;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut checked_to: gsize = 0;
    let mut line_term_len: gsize = 0;
    let mut line_length: gsize = 0;
    let mut got_term_len: gsize = 0;
    let mut first_time: gboolean = TRUE;
    if (*channel).use_buffer() == 0 {
        g_set_error_literal(
            error,
            g_convert_error_quark(),
            G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t do a raw read in g_io_channel_read_line_string\0" as *const u8
                    as *const gchar,
            ),
        );
        return G_IO_STATUS_ERROR;
    }
    status = G_IO_STATUS_NORMAL;
    if !(*channel).line_term.is_null() {
        line_term_len = (*channel).line_term_len as gsize;
    } else {
        line_term_len = 3 as gsize;
    }
    checked_to = 0 as gsize;
    's_38: while FALSE == 0 {
        let mut nextchar: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut lastchar: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut use_buf: *mut GString = ::core::ptr::null_mut::<GString>();
        if first_time == 0
            || (if !(if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            })
            .is_null()
            {
                (*(if !(*channel).encoding.is_null() {
                    (*channel).encoded_read_buf
                } else {
                    (*channel).read_buf
                }))
                .len
            } else {
                0 as gsize
            }) == 0 as gsize
        {
            current_block = 17373778776584603617;
        } else {
            current_block = 17478428563724192186;
        }
        '_read_again: loop {
            match current_block {
                17373778776584603617 => {
                    status = safe_c2rust_g_io_channel_fill_buffer(channel, error);
                    match status as ::core::ffi::c_uint {
                        1 => {
                            if !((if !(if !(*channel).encoding.is_null() {
                                (*channel).encoded_read_buf
                            } else {
                                (*channel).read_buf
                            })
                            .is_null()
                            {
                                (*(if !(*channel).encoding.is_null() {
                                    (*channel).encoded_read_buf
                                } else {
                                    (*channel).read_buf
                                }))
                                .len
                            } else {
                                0 as gsize
                            }) == 0 as gsize)
                            {
                                current_block = 17478428563724192186;
                                continue;
                            }
                            first_time = FALSE as gboolean;
                            continue 's_38;
                        }
                        2 => {
                            if (if !(if !(*channel).encoding.is_null() {
                                (*channel).encoded_read_buf
                            } else {
                                (*channel).read_buf
                            })
                            .is_null()
                            {
                                (*(if !(*channel).encoding.is_null() {
                                    (*channel).encoded_read_buf
                                } else {
                                    (*channel).read_buf
                                }))
                                .len
                            } else {
                                0 as gsize
                            }) == 0 as gsize
                            {
                                if !length.is_null() {
                                    *length = 0 as gsize;
                                }
                                if !(*channel).encoding.is_null()
                                    && (*(*channel).read_buf).len != 0 as gsize
                                {
                                    g_set_error_literal(
                                        error,
                                        g_convert_error_quark(),
                                        G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"Leftover unconverted data in read buffer\0"
                                                as *const u8
                                                as *const gchar,
                                        ),
                                    );
                                    return G_IO_STATUS_ERROR;
                                } else {
                                    return G_IO_STATUS_EOF;
                                }
                            }
                            current_block = 17478428563724192186;
                        }
                        _ => {
                            if !length.is_null() {
                                *length = 0 as gsize;
                            }
                            return status;
                        }
                    }
                }
                _ => {
                    if ({
                        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
                        if (if !(if !(*channel).encoding.is_null() {
                            (*channel).encoded_read_buf
                        } else {
                            (*channel).read_buf
                        })
                        .is_null()
                        {
                            (*(if !(*channel).encoding.is_null() {
                                (*channel).encoded_read_buf
                            } else {
                                (*channel).read_buf
                            }))
                            .len
                        } else {
                            0 as gsize
                        }) != 0 as gsize
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
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/giochannel.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1812 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"BUF_LEN (USE_BUF (channel)) != 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    use_buf = if !(*channel).encoding.is_null() {
                        (*channel).encoded_read_buf
                    } else {
                        (*channel).read_buf
                    };
                    first_time = FALSE as gboolean;
                    lastchar = (*use_buf).str_0.offset((*use_buf).len as isize);
                    nextchar = (*use_buf).str_0.offset(checked_to as isize);
                    while nextchar < lastchar {
                        if !(*channel).line_term.is_null() {
                            if memcmp(
                                (*channel).line_term as *const ::core::ffi::c_void,
                                nextchar as *const ::core::ffi::c_void,
                                line_term_len as size_t,
                            ) == 0 as ::core::ffi::c_int
                            {
                                line_length = nextchar.offset_from((*use_buf).str_0)
                                    as ::core::ffi::c_long
                                    as gsize;
                                got_term_len = line_term_len;
                                break 's_38;
                            }
                        } else {
                            match *nextchar as ::core::ffi::c_int {
                                10 => {
                                    current_block = 11315337735135006795;
                                    match current_block {
                                        11315337735135006795 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        3090467812768480670 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        4239559463070363580 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            if nextchar
                                                == lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && status as ::core::ffi::c_uint
                                                    != G_IO_STATUS_EOF as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                && lastchar
                                                    == (*use_buf)
                                                        .str_0
                                                        .offset((*use_buf).len as isize)
                                            {
                                                current_block = 17373778776584603617;
                                                continue '_read_again;
                                            }
                                            if nextchar
                                                < lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && *nextchar
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32
                                            {
                                                got_term_len = 2 as gsize;
                                            } else {
                                                got_term_len = 1 as gsize;
                                            }
                                            break 's_38;
                                        }
                                        _ => {
                                            if strncmp(
                                                b"\xE2\x80\xA9\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                nextchar,
                                                3 as size_t,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                line_length = nextchar.offset_from((*use_buf).str_0)
                                                    as ::core::ffi::c_long
                                                    as gsize;
                                                got_term_len = 3 as gsize;
                                                break 's_38;
                                            }
                                        }
                                    }
                                }
                                13 => {
                                    current_block = 4239559463070363580;
                                    match current_block {
                                        11315337735135006795 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        3090467812768480670 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        4239559463070363580 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            if nextchar
                                                == lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && status as ::core::ffi::c_uint
                                                    != G_IO_STATUS_EOF as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                && lastchar
                                                    == (*use_buf)
                                                        .str_0
                                                        .offset((*use_buf).len as isize)
                                            {
                                                current_block = 17373778776584603617;
                                                continue '_read_again;
                                            }
                                            if nextchar
                                                < lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && *nextchar
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32
                                            {
                                                got_term_len = 2 as gsize;
                                            } else {
                                                got_term_len = 1 as gsize;
                                            }
                                            break 's_38;
                                        }
                                        _ => {
                                            if strncmp(
                                                b"\xE2\x80\xA9\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                nextchar,
                                                3 as size_t,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                line_length = nextchar.offset_from((*use_buf).str_0)
                                                    as ::core::ffi::c_long
                                                    as gsize;
                                                got_term_len = 3 as gsize;
                                                break 's_38;
                                            }
                                        }
                                    }
                                }
                                -30 => {
                                    current_block = 17116429213293874016;
                                    match current_block {
                                        11315337735135006795 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        3090467812768480670 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        4239559463070363580 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            if nextchar
                                                == lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && status as ::core::ffi::c_uint
                                                    != G_IO_STATUS_EOF as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                && lastchar
                                                    == (*use_buf)
                                                        .str_0
                                                        .offset((*use_buf).len as isize)
                                            {
                                                current_block = 17373778776584603617;
                                                continue '_read_again;
                                            }
                                            if nextchar
                                                < lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && *nextchar
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32
                                            {
                                                got_term_len = 2 as gsize;
                                            } else {
                                                got_term_len = 1 as gsize;
                                            }
                                            break 's_38;
                                        }
                                        _ => {
                                            if strncmp(
                                                b"\xE2\x80\xA9\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                nextchar,
                                                3 as size_t,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                line_length = nextchar.offset_from((*use_buf).str_0)
                                                    as ::core::ffi::c_long
                                                    as gsize;
                                                got_term_len = 3 as gsize;
                                                break 's_38;
                                            }
                                        }
                                    }
                                }
                                0 => {
                                    current_block = 3090467812768480670;
                                    match current_block {
                                        11315337735135006795 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        3090467812768480670 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            got_term_len = 1 as gsize;
                                            break 's_38;
                                        }
                                        4239559463070363580 => {
                                            line_length = nextchar.offset_from((*use_buf).str_0)
                                                as ::core::ffi::c_long
                                                as gsize;
                                            if nextchar
                                                == lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && status as ::core::ffi::c_uint
                                                    != G_IO_STATUS_EOF as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                && lastchar
                                                    == (*use_buf)
                                                        .str_0
                                                        .offset((*use_buf).len as isize)
                                            {
                                                current_block = 17373778776584603617;
                                                continue '_read_again;
                                            }
                                            if nextchar
                                                < lastchar
                                                    .offset(-(1 as ::core::ffi::c_int as isize))
                                                && *nextchar
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32
                                            {
                                                got_term_len = 2 as gsize;
                                            } else {
                                                got_term_len = 1 as gsize;
                                            }
                                            break 's_38;
                                        }
                                        _ => {
                                            if strncmp(
                                                b"\xE2\x80\xA9\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                nextchar,
                                                3 as size_t,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                line_length = nextchar.offset_from((*use_buf).str_0)
                                                    as ::core::ffi::c_long
                                                    as gsize;
                                                got_term_len = 3 as gsize;
                                                break 's_38;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        if !(*channel).encoding.is_null() {
                            nextchar = nextchar.offset(
                                *safe_c2rust_g_utf8_skip
                                    .offset(*(nextchar as *const guchar) as isize)
                                    as ::core::ffi::c_int as isize,
                            ) as *mut ::core::ffi::c_char
                                as *mut gchar;
                        } else {
                            nextchar = nextchar.offset(1);
                        };
                    }
                    if ({
                        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
                        if nextchar == lastchar {
                            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_76
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/giochannel.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1869 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"nextchar == lastchar\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if status as ::core::ffi::c_uint
                        == G_IO_STATUS_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        current_block = 8545136480011357681;
                        break;
                    } else {
                        current_block = 3546145585875536353;
                        break;
                    }
                }
            }
        }
        match current_block {
            3546145585875536353 => {
                if (*use_buf).len > line_term_len.wrapping_sub(1 as gsize) {
                    checked_to = (*use_buf)
                        .len
                        .wrapping_sub(line_term_len.wrapping_sub(1 as gsize));
                } else {
                    checked_to = 0 as gsize;
                }
            }
            _ => {
                if !(*channel).encoding.is_null() && (*(*channel).read_buf).len > 0 as gsize {
                    g_set_error_literal(
                        error,
                        g_convert_error_quark(),
                        G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Channel terminates in a partial character\0" as *const u8
                                as *const gchar,
                        ),
                    );
                    return G_IO_STATUS_ERROR;
                }
                line_length = (*use_buf).len;
                got_term_len = 0 as gsize;
                break;
            }
        }
    }
    if !terminator_pos.is_null() {
        *terminator_pos = line_length;
    }
    if !length.is_null() {
        *length = line_length.wrapping_add(got_term_len);
    }
    return G_IO_STATUS_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_read_to_end(
    mut channel: *mut GIOChannel,
    mut str_return: *mut *mut gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if (*channel).is_readable() != 0 {
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
            b"channel->is_readable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if !str_return.is_null() {
        *str_return = ::core::ptr::null_mut::<gchar>();
    }
    if !length.is_null() {
        *length = 0 as gsize;
    }
    if (*channel).use_buffer() == 0 {
        g_set_error_literal(
            error,
            g_convert_error_quark(),
            G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t do a raw read in g_io_channel_read_to_end\0" as *const u8
                    as *const gchar,
            ),
        );
        return G_IO_STATUS_ERROR;
    }
    loop {
        status = safe_c2rust_g_io_channel_fill_buffer(channel, error);
        if !(status as ::core::ffi::c_uint
            == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
    }
    if status as ::core::ffi::c_uint != G_IO_STATUS_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return status;
    }
    if !(*channel).encoding.is_null() && (*(*channel).read_buf).len > 0 as gsize {
        g_set_error_literal(
            error,
            g_convert_error_quark(),
            G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Channel terminates in a partial character\0" as *const u8 as *const gchar,
            ),
        );
        return G_IO_STATUS_ERROR;
    }
    if if !(*channel).encoding.is_null() {
        (*channel).encoded_read_buf
    } else {
        (*channel).read_buf
    }
    .is_null()
    {
        if !str_return.is_null() {
            *str_return =
                safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
                    as *mut gchar;
        }
    } else {
        if !length.is_null() {
            *length = (*if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            })
            .len;
        }
        if !str_return.is_null() {
            *str_return = if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(
                        if !(*channel).encoding.is_null() {
                            (*channel).encoded_read_buf
                        } else {
                            (*channel).read_buf
                        },
                        0 as gboolean,
                    )
                } else {
                    g_string_free_and_steal(if !(*channel).encoding.is_null() {
                        (*channel).encoded_read_buf
                    } else {
                        (*channel).read_buf
                    })
                }
            } else {
                g_string_free(
                    if !(*channel).encoding.is_null() {
                        (*channel).encoded_read_buf
                    } else {
                        (*channel).read_buf
                    },
                    0 as gboolean,
                )
            };
        } else {
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(
                        if !(*channel).encoding.is_null() {
                            (*channel).encoded_read_buf
                        } else {
                            (*channel).read_buf
                        },
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                } else {
                    g_string_free_and_steal(if !(*channel).encoding.is_null() {
                        (*channel).encoded_read_buf
                    } else {
                        (*channel).read_buf
                    });
                };
            } else {
                g_string_free(
                    if !(*channel).encoding.is_null() {
                        (*channel).encoded_read_buf
                    } else {
                        (*channel).read_buf
                    },
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            };
        }
        if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf = ::core::ptr::null_mut::<GString>();
        } else {
            (*channel).read_buf = ::core::ptr::null_mut::<GString>();
        }
    }
    return G_IO_STATUS_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_read_chars(
    mut channel: *mut GIOChannel,
    mut buf: *mut gchar,
    mut count: gsize,
    mut bytes_read: *mut gsize,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut got_bytes: gsize = 0;
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if (*channel).is_readable() != 0 {
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
            b"channel->is_readable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if count == 0 as gsize {
        if !bytes_read.is_null() {
            *bytes_read = 0 as gsize;
        }
        return G_IO_STATUS_NORMAL;
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if !buf.is_null() {
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
            b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if (*channel).use_buffer() == 0 {
        let mut tmp_bytes: gsize = 0;
        if ({
            let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
            if (*channel).read_buf.is_null() || (*(*channel).read_buf).len == 0 as gsize {
                _g_boolean_var_84 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_84 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_84
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2029 as ::core::ffi::c_int,
                G_STRFUNC,
                b"!channel->read_buf || channel->read_buf->len == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        status = (*(*channel).funcs)
            .io_read
            .expect("non-null function pointer")(
            channel, buf, count, &raw mut tmp_bytes, error
        );
        if !bytes_read.is_null() {
            *bytes_read = tmp_bytes;
        }
        return status;
    }
    status = G_IO_STATUS_NORMAL;
    while (if !(if !(*channel).encoding.is_null() {
        (*channel).encoded_read_buf
    } else {
        (*channel).read_buf
    })
    .is_null()
    {
        (*(if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf
        } else {
            (*channel).read_buf
        }))
        .len
    } else {
        0 as gsize
    }) < count
        && status as ::core::ffi::c_uint
            == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = safe_c2rust_g_io_channel_fill_buffer(channel, error);
    }
    if (if !(if !(*channel).encoding.is_null() {
        (*channel).encoded_read_buf
    } else {
        (*channel).read_buf
    })
    .is_null()
    {
        (*(if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf
        } else {
            (*channel).read_buf
        }))
        .len
    } else {
        0 as gsize
    }) == 0 as gsize
    {
        if ({
            let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
            if status as ::core::ffi::c_uint
                != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _g_boolean_var_85 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_85 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_85
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2048 as ::core::ffi::c_int,
                G_STRFUNC,
                b"status != G_IO_STATUS_NORMAL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if status as ::core::ffi::c_uint
            == G_IO_STATUS_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
            && !(*channel).encoding.is_null()
            && (if !(*channel).read_buf.is_null() {
                (*(*channel).read_buf).len
            } else {
                0 as gsize
            }) > 0 as gsize
        {
            g_set_error_literal(
                error,
                g_convert_error_quark(),
                G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Leftover unconverted data in read buffer\0" as *const u8 as *const gchar,
                ),
            );
            status = G_IO_STATUS_ERROR;
        }
        if !bytes_read.is_null() {
            *bytes_read = 0 as gsize;
        }
        return status;
    }
    if status as ::core::ffi::c_uint
        == G_IO_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        g_clear_error(error);
    }
    got_bytes = if count
        < (if !(if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf
        } else {
            (*channel).read_buf
        })
        .is_null()
        {
            (*(if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            }))
            .len
        } else {
            0 as gsize
        }) {
        count
    } else if !if !(*channel).encoding.is_null() {
        (*channel).encoded_read_buf
    } else {
        (*channel).read_buf
    }
    .is_null()
    {
        (*if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf
        } else {
            (*channel).read_buf
        })
        .len
    } else {
        0 as gsize
    };
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if got_bytes > 0 as gsize {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
            2070 as ::core::ffi::c_int,
            G_STRFUNC,
            b"got_bytes > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*channel).encoding.is_null() {
        let mut nextchar: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut prevchar: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if ({
            let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
            if (if !(*channel).encoding.is_null() {
                (*channel).encoded_read_buf
            } else {
                (*channel).read_buf
            }) == (*channel).encoded_read_buf
            {
                _g_boolean_var_87 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_87 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_87
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2077 as ::core::ffi::c_int,
                G_STRFUNC,
                b"USE_BUF (channel) == channel->encoded_read_buf\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        nextchar = (*(*channel).encoded_read_buf).str_0;
        loop {
            prevchar = nextchar;
            nextchar = nextchar.offset(
                *safe_c2rust_g_utf8_skip.offset(*(nextchar as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char as *mut gchar;
            if ({
                let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
                if nextchar != prevchar {
                    _g_boolean_var_88 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_88 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_88
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2085 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"nextchar != prevchar\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !(nextchar
                < (*(*channel).encoded_read_buf)
                    .str_0
                    .offset(got_bytes as isize))
            {
                break;
            }
        }
        if nextchar
            > (*(*channel).encoded_read_buf)
                .str_0
                .offset(got_bytes as isize)
        {
            got_bytes = prevchar.offset_from((*(*channel).encoded_read_buf).str_0)
                as ::core::ffi::c_long as gsize;
        }
        if ({
            let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
            if got_bytes > 0 as gsize || count < 6 as gsize {
                _g_boolean_var_89 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_89 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_89
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2092 as ::core::ffi::c_int,
                G_STRFUNC,
                b"got_bytes > 0 || count < 6\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    memcpy(
        buf as *mut ::core::ffi::c_void,
        (*if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf
        } else {
            (*channel).read_buf
        })
        .str_0 as *const ::core::ffi::c_void,
        got_bytes as size_t,
    );
    g_string_erase(
        if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf
        } else {
            (*channel).read_buf
        },
        0 as gssize,
        got_bytes as gssize,
    );
    if !bytes_read.is_null() {
        *bytes_read = got_bytes;
    }
    return G_IO_STATUS_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_read_unichar(
    mut channel: *mut GIOChannel,
    mut thechar: *mut gunichar,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_NORMAL;
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if !channel.is_null() {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if !(*channel).encoding.is_null() {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"channel->encoding != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if (*channel).is_readable() != 0 {
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"channel->is_readable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    while (if !(*channel).encoded_read_buf.is_null() {
        (*(*channel).encoded_read_buf).len
    } else {
        0 as gsize
    }) == 0 as gsize
        && status as ::core::ffi::c_uint
            == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = safe_c2rust_g_io_channel_fill_buffer(channel, error);
    }
    if (if !(if !(*channel).encoding.is_null() {
        (*channel).encoded_read_buf
    } else {
        (*channel).read_buf
    })
    .is_null()
    {
        (*(if !(*channel).encoding.is_null() {
            (*channel).encoded_read_buf
        } else {
            (*channel).read_buf
        }))
        .len
    } else {
        0 as gsize
    }) == 0 as gsize
    {
        if ({
            let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
            if status as ::core::ffi::c_uint
                != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
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
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2136 as ::core::ffi::c_int,
                G_STRFUNC,
                b"status != G_IO_STATUS_NORMAL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if status as ::core::ffi::c_uint
            == G_IO_STATUS_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
            && (if !(*channel).read_buf.is_null() {
                (*(*channel).read_buf).len
            } else {
                0 as gsize
            }) > 0 as gsize
        {
            g_set_error_literal(
                error,
                g_convert_error_quark(),
                G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Leftover unconverted data in read buffer\0" as *const u8 as *const gchar,
                ),
            );
            status = G_IO_STATUS_ERROR;
        }
        if !thechar.is_null() {
            *thechar = -(1 as ::core::ffi::c_int) as gunichar;
        }
        return status;
    }
    if status as ::core::ffi::c_uint
        == G_IO_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        g_clear_error(error);
    }
    if !thechar.is_null() {
        *thechar = g_utf8_get_char((*(*channel).encoded_read_buf).str_0);
    }
    g_string_erase(
        (*channel).encoded_read_buf,
        0 as gssize,
        ((*(*channel).encoded_read_buf).str_0.offset(
            *safe_c2rust_g_utf8_skip
                .offset(*((*(*channel).encoded_read_buf).str_0 as *const guchar) as isize)
                as ::core::ffi::c_int as isize,
        ) as *mut ::core::ffi::c_char)
            .offset_from((*(*channel).encoded_read_buf).str_0) as gssize,
    );
    return G_IO_STATUS_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_write_chars(
    mut channel: *mut GIOChannel,
    mut buf: *const gchar,
    mut count: gssize,
    mut bytes_written: *mut gsize,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut count_unsigned: gsize = 0;
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut wrote_bytes: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if !channel.is_null() {
            _g_boolean_var_95 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_95 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_95
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if !buf.is_null() || count == 0 as gssize {
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
            b"buf != NULL || count == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if (*channel).is_writeable() != 0 {
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
            b"channel->is_writeable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if count < 0 as gssize {
        count_unsigned = strlen(buf as *const ::core::ffi::c_char) as gsize;
    } else {
        count_unsigned = count as gsize;
    }
    if count_unsigned == 0 as gsize {
        if !bytes_written.is_null() {
            *bytes_written = 0 as gsize;
        }
        return G_IO_STATUS_NORMAL;
    }
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if count_unsigned > 0 as gsize {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
            2217 as ::core::ffi::c_int,
            G_STRFUNC,
            b"count_unsigned > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*channel).use_buffer() == 0 {
        let mut tmp_bytes: gsize = 0;
        if ({
            let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
            if (*channel).write_buf.is_null() || (*(*channel).write_buf).len == 0 as gsize {
                _g_boolean_var_100 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_100 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_100
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2225 as ::core::ffi::c_int,
                G_STRFUNC,
                b"!channel->write_buf || channel->write_buf->len == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
            if (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '\0' as i32
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2226 as ::core::ffi::c_int,
                G_STRFUNC,
                b"channel->partial_write_buf[0] == '\\0'\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        status = (*(*channel).funcs)
            .io_write
            .expect("non-null function pointer")(
            channel,
            buf,
            count_unsigned,
            &raw mut tmp_bytes,
            error,
        );
        if !bytes_written.is_null() {
            *bytes_written = tmp_bytes;
        }
        return status;
    }
    if (*channel).is_seekable() as ::core::ffi::c_int != 0
        && ((if !(*channel).read_buf.is_null() {
            (*(*channel).read_buf).len
        } else {
            0 as gsize
        }) > 0 as gsize
            || (if !(*channel).encoded_read_buf.is_null() {
                (*(*channel).encoded_read_buf).len
            } else {
                0 as gsize
            }) > 0 as gsize)
    {
        if (*channel).do_encode() as ::core::ffi::c_int != 0
            && (if !(*channel).encoded_read_buf.is_null() {
                (*(*channel).encoded_read_buf).len
            } else {
                0 as gsize
            }) > 0 as gsize
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Mixed reading and writing not allowed on encoded files\0" as *const u8
                    as *const gchar,
            );
            return G_IO_STATUS_ERROR;
        }
        status = safe_c2rust_g_io_channel_seek_position(channel, 0 as gint64, G_SEEK_CUR, error);
        if status as ::core::ffi::c_uint
            != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if !bytes_written.is_null() {
                *bytes_written = 0 as gsize;
            }
            return status;
        }
    }
    if (*channel).write_buf.is_null() {
        (*channel).write_buf = g_string_sized_new((*channel).buf_size);
    }
    while wrote_bytes < count_unsigned {
        let mut space_in_buf: gsize = 0;
        if (*(*channel).write_buf).len >= (*channel).buf_size.wrapping_sub(MAX_CHAR_SIZE as gsize) {
            let mut did_write: gsize = 0 as gsize;
            let mut this_time: gsize = 0;
            loop {
                status = (*(*channel).funcs)
                    .io_write
                    .expect("non-null function pointer")(
                    channel,
                    (*(*channel).write_buf).str_0.offset(did_write as isize),
                    (*(*channel).write_buf).len.wrapping_sub(did_write),
                    &raw mut this_time,
                    error,
                );
                did_write = did_write.wrapping_add(this_time);
                if !(status as ::core::ffi::c_uint
                    == G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
                    && did_write
                        < (if (*(*channel).write_buf).len < 10 as gsize {
                            (*(*channel).write_buf).len
                        } else {
                            10 as gsize
                        }))
                {
                    break;
                }
            }
            g_string_erase((*channel).write_buf, 0 as gssize, did_write as gssize);
            if status as ::core::ffi::c_uint
                != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if status as ::core::ffi::c_uint
                    == G_IO_STATUS_AGAIN as ::core::ffi::c_int as ::core::ffi::c_uint
                    && wrote_bytes > 0 as gsize
                {
                    status = G_IO_STATUS_NORMAL;
                }
                if !bytes_written.is_null() {
                    *bytes_written = wrote_bytes;
                }
                return status;
            }
        }
        space_in_buf = (if (*channel).buf_size
            > (*(*channel).write_buf)
                .allocated_len
                .wrapping_sub(1 as gsize)
        {
            (*channel).buf_size
        } else {
            (*(*channel).write_buf)
                .allocated_len
                .wrapping_sub(1 as gsize)
        })
        .wrapping_sub((*(*channel).write_buf).len);
        if ({
            let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
            if space_in_buf >= 10 as gsize {
                _g_boolean_var_102 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_102 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_102
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                2301 as ::core::ffi::c_int,
                G_STRFUNC,
                b"space_in_buf >= MAX_CHAR_SIZE\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*channel).encoding.is_null() {
            let mut write_this: gsize = if space_in_buf < count_unsigned.wrapping_sub(wrote_bytes) {
                space_in_buf
            } else {
                count_unsigned.wrapping_sub(wrote_bytes)
            };
            if write_this > G_MAXSSIZE as gsize {
                write_this = G_MAXSSIZE as gsize;
            }
            safe_c2rust_g_string_append_len_inline(
                (*channel).write_buf,
                buf as *const ::core::ffi::c_char,
                write_this as gssize,
            );
            buf = buf.offset(write_this as isize);
            wrote_bytes = wrote_bytes.wrapping_add(write_this);
        } else {
            let mut from_buf: *const gchar = ::core::ptr::null::<gchar>();
            let mut from_buf_len: gsize = 0;
            let mut from_buf_old_len: gsize = 0;
            let mut left_len: gsize = 0;
            let mut err: gsize = 0;
            let mut errnum: gint = 0;
            if (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != '\0' as i32
            {
                if ({
                    let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
                    if wrote_bytes == 0 as gsize {
                        _g_boolean_var_103 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_103 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_103
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/giochannel.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        2324 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"wrote_bytes == 0\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                from_buf = &raw mut (*channel).partial_write_buf as *mut gchar;
                from_buf_old_len =
                    strlen(&raw mut (*channel).partial_write_buf as *mut gchar) as gsize;
                if ({
                    let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
                    if from_buf_old_len > 0 as gsize {
                        _g_boolean_var_104 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_104 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_104
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/giochannel.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        2328 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"from_buf_old_len > 0\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                from_buf_len = if (6 as gsize) < from_buf_old_len.wrapping_add(count_unsigned) {
                    6 as gsize
                } else {
                    from_buf_old_len.wrapping_add(count_unsigned)
                };
                memcpy(
                    (&raw mut (*channel).partial_write_buf as *mut gchar)
                        .offset(from_buf_old_len as isize)
                        as *mut ::core::ffi::c_void,
                    buf as *const ::core::ffi::c_void,
                    (from_buf_len as size_t).wrapping_sub(from_buf_old_len as size_t),
                );
            } else {
                from_buf = buf;
                from_buf_len = count_unsigned.wrapping_sub(wrote_bytes);
                from_buf_old_len = 0 as gsize;
            }
            loop {
                if (*channel).do_encode() == 0 {
                    let mut badchar: *const gchar = ::core::ptr::null::<gchar>();
                    let mut try_len: gsize = if from_buf_len < space_in_buf {
                        from_buf_len
                    } else {
                        space_in_buf
                    };
                    if g_utf8_validate_len(from_buf, try_len, &raw mut badchar) == 0 {
                        let mut try_char: gunichar = 0;
                        let mut incomplete_len: gsize =
                            from_buf.offset(try_len as isize).offset_from(badchar)
                                as ::core::ffi::c_long as gsize;
                        left_len = from_buf.offset(from_buf_len as isize).offset_from(badchar)
                            as ::core::ffi::c_long as gsize;
                        try_char = g_utf8_get_char_validated(badchar, incomplete_len as gssize);
                        match try_char {
                            4294967294 => {
                                if ({
                                    let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
                                    if incomplete_len < 6 as gsize {
                                        _g_boolean_var_105 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_105 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_105
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                } else {
                                    g_assertion_message_expr(
                                        G_LOG_DOMAIN.as_ptr(),
                                        b"../original/glib/giochannel.c\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        2362 as ::core::ffi::c_int,
                                        G_STRFUNC,
                                        b"incomplete_len < 6\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                                if try_len == from_buf_len {
                                    errnum = EINVAL as gint;
                                    err = -(1 as ::core::ffi::c_int) as gsize;
                                } else {
                                    errnum = 0 as ::core::ffi::c_int as gint;
                                    err = 0 as ::core::ffi::c_int as gsize;
                                }
                            }
                            4294967295 => {
                                g_log(
                                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                                    G_LOG_LEVEL_WARNING,
                                    b"Invalid UTF-8 passed to g_io_channel_write_chars().\0"
                                        as *const u8
                                        as *const gchar,
                                );
                                errnum = EILSEQ as gint;
                                err = -(1 as ::core::ffi::c_int) as gsize;
                            }
                            _ => {
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"../original/glib/giochannel.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    2381 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                );
                            }
                        }
                    } else {
                        err = 0 as ::core::ffi::c_int as gsize;
                        errnum = 0 as ::core::ffi::c_int as gint;
                        left_len = from_buf_len.wrapping_sub(try_len);
                    }
                    safe_c2rust_g_string_append_len_inline(
                        (*channel).write_buf,
                        from_buf as *const ::core::ffi::c_char,
                        from_buf_len.wrapping_sub(left_len) as gssize,
                    );
                    from_buf = from_buf.offset(from_buf_len.wrapping_sub(left_len) as isize);
                } else {
                    let mut outbuf: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    left_len = from_buf_len;
                    g_string_set_size(
                        (*channel).write_buf,
                        (*(*channel).write_buf).len.wrapping_add(space_in_buf),
                    );
                    outbuf = (*(*channel).write_buf)
                        .str_0
                        .offset((*(*channel).write_buf).len as isize)
                        .offset(-(space_in_buf as isize));
                    err = g_iconv(
                        (*channel).write_cd,
                        &raw mut from_buf as *mut *mut gchar,
                        &raw mut left_len,
                        &raw mut outbuf,
                        &raw mut space_in_buf,
                    );
                    errnum = *__errno_location() as gint;
                    safe_c2rust_g_string_truncate_inline(
                        (*channel).write_buf,
                        (*(*channel).write_buf).len.wrapping_sub(space_in_buf),
                    );
                }
                if !(err == -(1 as ::core::ffi::c_int) as gsize) {
                    break;
                }
                match errnum {
                    EINVAL => {
                        if ({
                            let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
                            if left_len < 6 as gsize {
                                _g_boolean_var_106 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_106 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_106
                        }) as ::core::ffi::c_long
                            != 0
                        {
                        } else {
                            g_assertion_message_expr(
                                G_LOG_DOMAIN.as_ptr(),
                                b"../original/glib/giochannel.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                2418 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"left_len < 6\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        if from_buf_old_len == 0 as gsize {
                            memcpy(
                                &raw mut (*channel).partial_write_buf as *mut gchar
                                    as *mut ::core::ffi::c_void,
                                from_buf as *const ::core::ffi::c_void,
                                left_len as size_t,
                            );
                            (*channel).partial_write_buf[left_len as usize] = '\0' as i32 as gchar;
                            if !bytes_written.is_null() {
                                *bytes_written = count_unsigned;
                            }
                            return G_IO_STATUS_NORMAL;
                        }
                        if left_len == from_buf_len {
                            if ({
                                let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
                                if count_unsigned == from_buf_len.wrapping_sub(from_buf_old_len) {
                                    _g_boolean_var_107 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_107 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_107
                            }) as ::core::ffi::c_long
                                != 0
                            {
                            } else {
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"../original/glib/giochannel.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    2439 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"count_unsigned == from_buf_len - from_buf_old_len\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            (*channel).partial_write_buf[from_buf_len as usize] =
                                '\0' as i32 as gchar;
                            if !bytes_written.is_null() {
                                *bytes_written = count_unsigned;
                            }
                            return G_IO_STATUS_NORMAL;
                        }
                        if ({
                            let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
                            if from_buf_len.wrapping_sub(left_len) >= from_buf_old_len {
                                _g_boolean_var_108 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_108 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_108
                        }) as ::core::ffi::c_long
                            != 0
                        {
                        } else {
                            g_assertion_message_expr(
                                G_LOG_DOMAIN.as_ptr(),
                                b"../original/glib/giochannel.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                2449 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"from_buf_len - left_len >= from_buf_old_len\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        break;
                    }
                    E2BIG => {
                        if !(from_buf_len == left_len) {
                            break;
                        }
                        space_in_buf = space_in_buf.wrapping_add(MAX_CHAR_SIZE as gsize);
                    }
                    EILSEQ => {
                        g_set_error_literal(
                            error,
                            g_convert_error_quark(),
                            G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Invalid byte sequence in conversion input\0" as *const u8
                                    as *const gchar,
                            ),
                        );
                        if from_buf_old_len > 0 as gsize && from_buf_len == left_len {
                            g_log(
                                G_LOG_DOMAIN.as_ptr() as *const gchar,
                                G_LOG_LEVEL_WARNING,
                                b"Illegal sequence due to partial character at the end of a previous write.\0"
                                    as *const u8 as *const gchar,
                            );
                        } else {
                            if ({
                                let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
                                if from_buf_len >= left_len.wrapping_add(from_buf_old_len) {
                                    _g_boolean_var_109 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_109 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_109
                            }) as ::core::ffi::c_long
                                != 0
                            {
                            } else {
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"../original/glib/giochannel.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    2473 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"from_buf_len >= left_len + from_buf_old_len\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            wrote_bytes = wrote_bytes.wrapping_add(
                                from_buf_len
                                    .wrapping_sub(left_len)
                                    .wrapping_sub(from_buf_old_len),
                            );
                        }
                        if !bytes_written.is_null() {
                            *bytes_written = wrote_bytes;
                        }
                        (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] =
                            '\0' as i32 as gchar;
                        return G_IO_STATUS_ERROR;
                    }
                    _ => {
                        g_set_error(
                            error,
                            g_convert_error_quark(),
                            G_CONVERT_ERROR_FAILED as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Error during conversion: %s\0" as *const u8 as *const gchar,
                            ),
                            g_strerror(errnum),
                        );
                        if from_buf_len >= left_len.wrapping_add(from_buf_old_len) {
                            wrote_bytes = wrote_bytes.wrapping_add(
                                from_buf_len
                                    .wrapping_sub(left_len)
                                    .wrapping_sub(from_buf_old_len),
                            );
                        }
                        if !bytes_written.is_null() {
                            *bytes_written = wrote_bytes;
                        }
                        (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] =
                            '\0' as i32 as gchar;
                        return G_IO_STATUS_ERROR;
                    }
                }
            }
            if ({
                let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
                if from_buf_len.wrapping_sub(left_len) >= from_buf_old_len {
                    _g_boolean_var_110 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_110 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_110
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2492 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"from_buf_len - left_len >= from_buf_old_len\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            wrote_bytes = wrote_bytes.wrapping_add(
                from_buf_len
                    .wrapping_sub(left_len)
                    .wrapping_sub(from_buf_old_len),
            );
            if from_buf_old_len > 0 as gsize {
                buf = buf.offset(
                    from_buf_len
                        .wrapping_sub(left_len)
                        .wrapping_sub(from_buf_old_len) as isize,
                );
                (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] =
                    '\0' as i32 as gchar;
            } else {
                buf = from_buf;
            }
        }
    }
    if !bytes_written.is_null() {
        *bytes_written = count_unsigned;
    }
    return G_IO_STATUS_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_write_unichar(
    mut channel: *mut GIOChannel,
    mut thechar: gunichar,
    mut error: *mut *mut GError,
) -> GIOStatus {
    let mut status: GIOStatus = G_IO_STATUS_ERROR;
    let mut static_buf: [gchar; 6] = [0; 6];
    let mut char_len: gsize = 0;
    let mut wrote_len: gsize = 0;
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if !channel.is_null() {
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
            b"channel != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if !(*channel).encoding.is_null() {
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
            b"channel->encoding != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"(error == NULL) || (*error == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if (*channel).is_writeable() != 0 {
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
            b"channel->is_writeable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_IO_STATUS_ERROR;
    }
    char_len = g_unichar_to_utf8(thechar, &raw mut static_buf as *mut gchar) as gsize;
    if (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != '\0' as i32
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Partial character written before writing unichar.\0" as *const u8 as *const gchar,
        );
        (*channel).partial_write_buf[0 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
    }
    status = safe_c2rust_g_io_channel_write_chars(
        channel,
        &raw mut static_buf as *mut gchar,
        char_len as gssize,
        &raw mut wrote_len,
        error,
    );
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if wrote_len == char_len
            || status as ::core::ffi::c_uint
                != G_IO_STATUS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/giochannel.c\0" as *const u8 as *const ::core::ffi::c_char,
            2554 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wrote_len == char_len || status != G_IO_STATUS_NORMAL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_channel_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_116 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_116 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_116
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-io-channel-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const EOVERFLOW: ::core::ffi::c_int = 75;
pub const EILSEQ: ::core::ffi::c_int = 84;
