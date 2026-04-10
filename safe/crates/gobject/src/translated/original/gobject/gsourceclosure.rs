// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GIConv;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback_indirect(
        source: *mut GSource,
        callback_data: gpointer,
        callback_funcs: *mut GSourceCallbackFuncs,
    );
    static mut g_timeout_funcs: GSourceFuncs;
    static mut g_child_watch_funcs: GSourceFuncs;
    static mut g_idle_funcs: GSourceFuncs;
    static mut g_unix_signal_funcs: GSourceFuncs;
    static mut g_unix_fd_source_funcs: GSourceFuncs;
    fn g_io_channel_ref(channel: *mut GIOChannel) -> *mut GIOChannel;
    fn g_io_channel_unref(channel: *mut GIOChannel);
    static mut g_io_watch_funcs: GSourceFuncs;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
    fn g_closure_ref(closure: *mut GClosure) -> *mut GClosure;
    fn g_closure_sink(closure: *mut GClosure);
    fn g_closure_unref(closure: *mut GClosure);
    fn g_closure_new_simple(sizeof_closure: guint, data: gpointer) -> *mut GClosure;
    fn g_closure_add_invalidate_notifier(
        closure: *mut GClosure,
        notify_data: gpointer,
        notify_func: GClosureNotify,
    );
    fn g_closure_set_marshal(closure: *mut GClosure, marshal: GClosureMarshal);
    fn g_closure_set_meta_marshal(
        closure: *mut GClosure,
        marshal_data: gpointer,
        meta_marshal: GClosureMarshal,
    );
    fn g_closure_invoke(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
    );
    fn g_cclosure_marshal_generic(
        closure: *mut GClosure,
        return_gvalue: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_flags_register_static(
        name: *const gchar,
        const_static_values: *const GFlagsValue,
    ) -> GType;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_set_ulong(value: *mut GValue, v_ulong: gulong);
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type GPid = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCClosure {
    pub closure: GClosure,
    pub callback: gpointer,
}
pub type GCClosure = _GCClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GIOChannel) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GIOChannel) -> *mut GIOChannel>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GIOChannel) -> *mut GIOChannel>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
#[no_mangle]
pub unsafe extern "C" fn g_io_channel_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_io_channel_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_io_channel_get_type_once() -> GType {
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
        g_intern_static_string(b"GIOChannel\0" as *const u8 as *const gchar),
        C2RustUnnamed_1 {
            do_copy_type: Some(
                g_io_channel_ref as unsafe extern "C" fn(*mut GIOChannel) -> *mut GIOChannel,
            ),
        },
        C2RustUnnamed_0 {
            do_free_type: Some(g_io_channel_unref as unsafe extern "C" fn(*mut GIOChannel) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_io_condition_get_type() -> GType {
    static mut etype: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            etype;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut etype;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(&raw mut etype as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        static mut values: [GFlagsValue; 7] = [
            _GFlagsValue {
                value: G_IO_IN as ::core::ffi::c_int as guint,
                value_name: b"G_IO_IN\0" as *const u8 as *const gchar,
                value_nick: b"in\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_OUT as ::core::ffi::c_int as guint,
                value_name: b"G_IO_OUT\0" as *const u8 as *const gchar,
                value_nick: b"out\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_PRI as ::core::ffi::c_int as guint,
                value_name: b"G_IO_PRI\0" as *const u8 as *const gchar,
                value_nick: b"pri\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_ERR as ::core::ffi::c_int as guint,
                value_name: b"G_IO_ERR\0" as *const u8 as *const gchar,
                value_nick: b"err\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_HUP as ::core::ffi::c_int as guint,
                value_name: b"G_IO_HUP\0" as *const u8 as *const gchar,
                value_nick: b"hup\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_NVAL as ::core::ffi::c_int as guint,
                value_name: b"G_IO_NVAL\0" as *const u8 as *const gchar,
                value_nick: b"nval\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut type_id: GType = g_flags_register_static(
            b"GIOCondition\0" as *const u8 as *const gchar,
            &raw const values as *const GFlagsValue,
        );
        if 0 as ::core::ffi::c_int != 0 {
            etype = type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut etype as *mut ::core::ffi::c_void,
            type_id as gpointer,
        );
    }
    return etype;
}
unsafe extern "C" fn source_closure_marshal_BOOLEAN__VOID(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut callback: GSourceFunc = None;
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut v_return: gboolean = 0;
    if !return_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"source_closure_marshal_BOOLEAN__VOID\0" as *const u8 as *const ::core::ffi::c_char,
            b"return_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_param_values == 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"source_closure_marshal_BOOLEAN__VOID\0" as *const u8 as *const ::core::ffi::c_char,
            b"n_param_values == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    callback = ::core::mem::transmute::<*mut ::core::ffi::c_void, GSourceFunc>(
        if !marshal_data.is_null() {
            marshal_data as *mut ::core::ffi::c_void
        } else {
            (*cc).callback as *mut ::core::ffi::c_void
        },
    );
    v_return = callback.expect("non-null function pointer")((*closure).data);
    g_value_set_boolean(return_value, v_return);
}
unsafe extern "C" fn io_watch_closure_callback(
    mut channel: *mut GIOChannel,
    mut condition: GIOCondition,
    mut data: gpointer,
) -> gboolean {
    let mut closure: *mut GClosure = data as *mut GClosure;
    let mut params: [GValue; 2] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
    ];
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result: gboolean = 0;
    g_value_init(
        &raw mut result_value,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        g_io_channel_get_type(),
    );
    g_value_set_boxed(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        channel as gconstpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        g_io_condition_get_type(),
    );
    g_value_set_flags(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        condition as guint,
    );
    g_closure_invoke(
        closure,
        &raw mut result_value,
        2 as guint,
        &raw mut params as *mut GValue,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset(&raw mut result_value);
    g_value_unset(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    return result;
}
unsafe extern "C" fn g_child_watch_closure_callback(
    mut pid: GPid,
    mut status: gint,
    mut data: gpointer,
) -> gboolean {
    let mut closure: *mut GClosure = data as *mut GClosure;
    let mut params: [GValue; 2] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
    ];
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result: gboolean = 0;
    g_value_init(
        &raw mut result_value,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_set_ulong(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        pid as gulong,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_set_int(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        status,
    );
    g_closure_invoke(
        closure,
        &raw mut result_value,
        2 as guint,
        &raw mut params as *mut GValue,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset(&raw mut result_value);
    g_value_unset(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    return result;
}
unsafe extern "C" fn g_unix_fd_source_closure_callback(
    mut fd: ::core::ffi::c_int,
    mut condition: GIOCondition,
    mut data: gpointer,
) -> gboolean {
    let mut closure: *mut GClosure = data as *mut GClosure;
    let mut params: [GValue; 2] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
    ];
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result: gboolean = 0;
    g_value_init(
        &raw mut result_value,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_set_int(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        fd as gint,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        g_io_condition_get_type(),
    );
    g_value_set_flags(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        condition as guint,
    );
    g_closure_invoke(
        closure,
        &raw mut result_value,
        2 as guint,
        &raw mut params as *mut GValue,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset(&raw mut result_value);
    g_value_unset(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    return result;
}
unsafe extern "C" fn source_closure_callback(mut data: gpointer) -> gboolean {
    let mut closure: *mut GClosure = data as *mut GClosure;
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result: gboolean = 0;
    g_value_init(
        &raw mut result_value,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_closure_invoke(
        closure,
        &raw mut result_value,
        0 as guint,
        ::core::ptr::null::<GValue>(),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset(&raw mut result_value);
    return result;
}
unsafe extern "C" fn closure_callback_get(
    mut cb_data: gpointer,
    mut source: *mut GSource,
    mut func: *mut GSourceFunc,
    mut data: *mut gpointer,
) {
    let mut closure_callback: GSourceFunc = (*(*source).source_funcs).closure_callback;
    if closure_callback.is_none() {
        if (*source).source_funcs == &raw mut g_io_watch_funcs as *const GSourceFuncs {
            closure_callback = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean>,
                GSourceFunc,
            >(Some(
                io_watch_closure_callback
                    as unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean,
            ));
        } else if (*source).source_funcs == &raw mut g_child_watch_funcs as *const GSourceFuncs {
            closure_callback = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(GPid, gint, gpointer) -> gboolean>,
                GSourceFunc,
            >(Some(
                g_child_watch_closure_callback
                    as unsafe extern "C" fn(GPid, gint, gpointer) -> gboolean,
            ));
        } else if (*source).source_funcs == &raw mut g_unix_fd_source_funcs as *const GSourceFuncs {
            closure_callback = ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(::core::ffi::c_int, GIOCondition, gpointer) -> gboolean,
                >,
                GSourceFunc,
            >(Some(
                g_unix_fd_source_closure_callback
                    as unsafe extern "C" fn(::core::ffi::c_int, GIOCondition, gpointer) -> gboolean,
            ));
        } else if (*source).source_funcs == &raw mut g_timeout_funcs as *const GSourceFuncs
            || (*source).source_funcs == &raw mut g_unix_signal_funcs as *const GSourceFuncs
            || (*source).source_funcs == &raw mut g_idle_funcs as *const GSourceFuncs
        {
            closure_callback =
                Some(source_closure_callback as unsafe extern "C" fn(gpointer) -> gboolean)
                    as GSourceFunc;
        }
    }
    *func = closure_callback;
    *data = cb_data;
}
static mut closure_callback_funcs: GSourceCallbackFuncs = unsafe {
    _GSourceCallbackFuncs {
        ref_0: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GClosure) -> *mut GClosure>,
            Option<unsafe extern "C" fn(gpointer) -> ()>,
        >(Some(
            g_closure_ref as unsafe extern "C" fn(*mut GClosure) -> *mut GClosure,
        )),
        unref: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GClosure) -> ()>,
            Option<unsafe extern "C" fn(gpointer) -> ()>,
        >(Some(
            g_closure_unref as unsafe extern "C" fn(*mut GClosure) -> (),
        )),
        get: Some(
            closure_callback_get
                as unsafe extern "C" fn(
                    gpointer,
                    *mut GSource,
                    *mut GSourceFunc,
                    *mut gpointer,
                ) -> (),
        ),
    }
};
unsafe extern "C" fn closure_invalidated(mut user_data: gpointer, mut closure: *mut GClosure) {
    g_source_destroy(user_data as *mut GSource);
}
#[no_mangle]
pub unsafe extern "C" fn g_source_set_closure(
    mut source: *mut GSource,
    mut closure: *mut GClosure,
) {
    if !source.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_source_set_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_source_set_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*source).source_funcs).closure_callback.is_none()
        && (*source).source_funcs != &raw mut g_unix_fd_source_funcs as *const GSourceFuncs
        && (*source).source_funcs != &raw mut g_unix_signal_funcs as *const GSourceFuncs
        && (*source).source_funcs != &raw mut g_child_watch_funcs as *const GSourceFuncs
        && (*source).source_funcs != &raw mut g_io_watch_funcs as *const GSourceFuncs
        && (*source).source_funcs != &raw mut g_timeout_funcs as *const GSourceFuncs
        && (*source).source_funcs != &raw mut g_idle_funcs as *const GSourceFuncs
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsourceclosure.c:263: closure cannot be set on GSource without GSourceFuncs::closure_callback\0"
                as *const u8 as *const gchar,
        );
        return;
    }
    g_closure_ref(closure);
    g_closure_sink(closure);
    g_source_set_callback_indirect(source, closure as gpointer, &raw mut closure_callback_funcs);
    g_closure_add_invalidate_notifier(
        closure,
        source as gpointer,
        Some(closure_invalidated as unsafe extern "C" fn(gpointer, *mut GClosure) -> ()),
    );
    if (*closure).marshal.is_none() {
        let mut marshal: GClosureMarshal = ::core::mem::transmute::<
            GSourceDummyMarshal,
            GClosureMarshal,
        >((*(*source).source_funcs).closure_marshal);
        if marshal.is_some() {
            g_closure_set_marshal(closure, marshal);
        } else if (*source).source_funcs == &raw mut g_idle_funcs as *const GSourceFuncs
            || (*source).source_funcs == &raw mut g_unix_signal_funcs as *const GSourceFuncs
            || (*source).source_funcs == &raw mut g_timeout_funcs as *const GSourceFuncs
        {
            g_closure_set_marshal(
                closure,
                Some(
                    source_closure_marshal_BOOLEAN__VOID
                        as unsafe extern "C" fn(
                            *mut GClosure,
                            *mut GValue,
                            guint,
                            *const GValue,
                            gpointer,
                            gpointer,
                        ) -> (),
                ),
            );
        } else {
            g_closure_set_marshal(
                closure,
                Some(
                    g_cclosure_marshal_generic
                        as unsafe extern "C" fn(
                            *mut GClosure,
                            *mut GValue,
                            guint,
                            *const GValue,
                            gpointer,
                            gpointer,
                        ) -> (),
                ),
            );
        }
    }
}
unsafe extern "C" fn dummy_closure_marshal(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    if ({
        let mut __val: *const GValue = return_value as *const GValue;
        let mut __t: GType = ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
        g_value_set_boolean(
            return_value,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_source_set_dummy_callback(mut source: *mut GSource) {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    closure = g_closure_new_simple(
        ::core::mem::size_of::<GClosure>() as guint,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    g_closure_set_meta_marshal(
        closure,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(
            dummy_closure_marshal
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
    );
    g_source_set_closure(source, closure);
}
