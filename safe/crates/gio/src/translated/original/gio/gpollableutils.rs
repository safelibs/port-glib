use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GPollableInputStream;
    pub type _GPollableOutputStream;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_add_child_source(source: *mut GSource, child_source: *mut GSource);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_closure_invoke(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
    );
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_source_set_dummy_callback(source: *mut GSource);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_input_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_output_stream_write(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_cancellable_source_new(cancellable: *mut GCancellable) -> *mut GSource;
    fn g_pollable_output_stream_get_type() -> GType;
    fn g_pollable_output_stream_write_nonblocking(
        stream: *mut GPollableOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_pollable_input_stream_get_type() -> GType;
    fn g_pollable_input_stream_read_nonblocking(
        stream: *mut GPollableInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
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
pub type GPollableInputStream = _GPollableInputStream;
pub type GPollableOutputStream = _GPollableOutputStream;
pub type GPollableSourceFunc = Option<unsafe extern "C" fn(*mut GObject, gpointer) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPollableSource {
    pub source: GSource,
    pub stream: *mut GObject,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_OBJECT: GType = ((20 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
unsafe extern "C" fn safe_c2rust_pollable_source_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut func: GPollableSourceFunc =
        ::core::mem::transmute::<GSourceFunc, GPollableSourceFunc>(callback);
    let mut pollable_source: *mut GPollableSource = source as *mut GPollableSource;
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
        (*pollable_source).stream,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_pollable_source_finalize(mut source: *mut GSource) {
    let mut pollable_source: *mut GPollableSource = source as *mut GPollableSource;
    g_object_unref((*pollable_source).stream as gpointer);
}
unsafe extern "C" fn safe_c2rust_pollable_source_closure_callback(
    mut stream: *mut GObject,
    mut data: gpointer,
) -> gboolean {
    let mut closure: *mut GClosure = data as *mut GClosure;
    let mut param: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
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
    g_value_init(&raw mut result_value, G_TYPE_BOOLEAN);
    g_value_init(&raw mut param, G_TYPE_OBJECT);
    g_value_set_object(&raw mut param, stream as gpointer);
    g_closure_invoke(
        closure,
        &raw mut result_value,
        1 as guint,
        &raw mut param,
        NULL,
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset(&raw mut result_value);
    g_value_unset(&raw mut param);
    return result;
}
static mut safe_c2rust_pollable_source_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: None,
        check: None,
        dispatch: Some(
            safe_c2rust_pollable_source_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: Some(
            safe_c2rust_pollable_source_finalize as unsafe extern "C" fn(*mut GSource) -> (),
        ),
        closure_callback: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, gpointer) -> gboolean>,
            GSourceFunc,
        >(Some(
            safe_c2rust_pollable_source_closure_callback
                as unsafe extern "C" fn(*mut GObject, gpointer) -> gboolean,
        )),
        closure_marshal: None,
    }
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pollable_source_new(
    mut pollable_stream: *mut GObject,
) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut pollable_source: *mut GPollableSource = ::core::ptr::null_mut::<GPollableSource>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = pollable_stream as *mut GTypeInstance;
            let mut __t: GType = g_pollable_input_stream_get_type();
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
            || ({
                let mut __inst: *mut GTypeInstance = pollable_stream as *mut GTypeInstance;
                let mut __t: GType = g_pollable_output_stream_get_type();
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
            b"G_IS_POLLABLE_INPUT_STREAM (pollable_stream) || G_IS_POLLABLE_OUTPUT_STREAM (pollable_stream)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    source = g_source_new(
        &raw mut safe_c2rust_pollable_source_funcs,
        ::core::mem::size_of::<GPollableSource>() as guint,
    );
    g_source_set_static_name(
        source,
        b"GPollableSource\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pollable_source = source as *mut GPollableSource;
    (*pollable_source).stream =
        g_object_ref(pollable_stream as gpointer) as *mut GObject as *mut GObject;
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pollable_source_new_full(
    mut pollable_stream: gpointer,
    mut child_source: *mut GSource,
    mut cancellable: *mut GCancellable,
) -> *mut GSource {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = pollable_stream as *mut GTypeInstance;
            let mut __t: GType = g_pollable_input_stream_get_type();
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
            || ({
                let mut __inst: *mut GTypeInstance = pollable_stream as *mut GTypeInstance;
                let mut __t: GType = g_pollable_output_stream_get_type();
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
            b"G_IS_POLLABLE_INPUT_STREAM (pollable_stream) || G_IS_POLLABLE_OUTPUT_STREAM (pollable_stream)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSource>();
    }
    source = safe_c2rust_g_pollable_source_new(pollable_stream as *mut GObject);
    if !child_source.is_null() {
        g_source_set_dummy_callback(child_source);
        g_source_add_child_source(source, child_source);
    }
    if !cancellable.is_null() {
        let mut cancellable_source: *mut GSource = g_cancellable_source_new(cancellable);
        g_source_set_dummy_callback(cancellable_source);
        g_source_add_child_source(source, cancellable_source);
        g_source_unref(cancellable_source);
    }
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pollable_stream_read(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut blocking: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    if blocking != 0 {
        return g_input_stream_read(stream, buffer, count, cancellable, error);
    } else {
        return g_pollable_input_stream_read_nonblocking(
            stream as *mut ::core::ffi::c_void as *mut GPollableInputStream,
            buffer,
            count,
            cancellable,
            error,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pollable_stream_write(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut blocking: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    if blocking != 0 {
        return g_output_stream_write(stream, buffer, count, cancellable, error);
    } else {
        return g_pollable_output_stream_write_nonblocking(
            stream as *mut ::core::ffi::c_void as *mut GPollableOutputStream,
            buffer,
            count,
            cancellable,
            error,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pollable_stream_write_all(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut blocking: gboolean,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut _bytes_written: gsize = 0;
    let mut res: gssize = 0;
    _bytes_written = 0 as gsize;
    while _bytes_written < count {
        res = safe_c2rust_g_pollable_stream_write(
            stream,
            (buffer as *mut ::core::ffi::c_char).offset(_bytes_written as isize)
                as *const ::core::ffi::c_void,
            count.wrapping_sub(_bytes_written),
            blocking,
            cancellable,
            error,
        );
        if res == -(1 as ::core::ffi::c_int) as gssize {
            if !bytes_written.is_null() {
                *bytes_written = _bytes_written;
            }
            return FALSE;
        }
        if res == 0 as gssize {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Write returned zero without error\0" as *const u8 as *const gchar,
            );
        }
        _bytes_written = _bytes_written.wrapping_add(res as gsize);
    }
    if !bytes_written.is_null() {
        *bytes_written = _bytes_written;
    }
    return TRUE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
