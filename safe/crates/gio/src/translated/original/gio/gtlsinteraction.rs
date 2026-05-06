extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GIOStreamPrivate;
    pub type _GTask;
    pub type _GTlsConnectionPrivate;
    pub type _GTlsPasswordPrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_signal(cond: *mut GCond);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_iteration(context: *mut GMainContext, may_block: gboolean) -> gboolean;
    fn g_main_context_acquire(context: *mut GMainContext) -> gboolean;
    fn g_main_context_release(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_main_context_invoke(context: *mut GMainContext, function: GSourceFunc, data: gpointer);
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_cancellable_get_type() -> GType;
    fn g_tls_connection_get_type() -> GType;
    fn g_tls_password_get_type() -> GType;
    fn g_async_result_get_type() -> GType;
    fn g_async_result_is_tagged(res: *mut GAsyncResult, source_tag: gpointer) -> gboolean;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
}
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
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
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
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
pub type GTlsInteractionResult = ::core::ffi::c_uint;
pub const G_TLS_INTERACTION_FAILED: GTlsInteractionResult = 2;
pub const G_TLS_INTERACTION_HANDLED: GTlsInteractionResult = 1;
pub const G_TLS_INTERACTION_UNHANDLED: GTlsInteractionResult = 0;
pub type GTlsCertificateRequestFlags = ::core::ffi::c_uint;
pub const G_TLS_CERTIFICATE_REQUEST_NONE: GTlsCertificateRequestFlags = 0;
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
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GIOStream = _GIOStream;
pub type GTask = _GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GTlsConnectionPrivate,
}
pub type GTlsConnectionPrivate = _GTlsConnectionPrivate;
pub type GTlsConnection = _GTlsConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsInteraction {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsInteractionPrivate,
}
pub type GTlsInteractionPrivate = _GTlsInteractionPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsInteractionPrivate {
    pub context: *mut GMainContext,
}
pub type GTlsInteraction = _GTlsInteraction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsPassword {
    pub parent_instance: GObject,
    pub priv_0: *mut GTlsPasswordPrivate,
}
pub type GTlsPasswordPrivate = _GTlsPasswordPrivate;
pub type GTlsPassword = _GTlsPassword;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTlsInteractionClass {
    pub parent_class: GObjectClass,
    pub ask_password: Option<
        unsafe extern "C" fn(
            *mut GTlsInteraction,
            *mut GTlsPassword,
            *mut GCancellable,
            *mut *mut GError,
        ) -> GTlsInteractionResult,
    >,
    pub ask_password_async: Option<
        unsafe extern "C" fn(
            *mut GTlsInteraction,
            *mut GTlsPassword,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub ask_password_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsInteraction,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> GTlsInteractionResult,
    >,
    pub request_certificate: Option<
        unsafe extern "C" fn(
            *mut GTlsInteraction,
            *mut GTlsConnection,
            GTlsCertificateRequestFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> GTlsInteractionResult,
    >,
    pub request_certificate_async: Option<
        unsafe extern "C" fn(
            *mut GTlsInteraction,
            *mut GTlsConnection,
            GTlsCertificateRequestFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub request_certificate_finish: Option<
        unsafe extern "C" fn(
            *mut GTlsInteraction,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> GTlsInteractionResult,
    >,
    pub padding: [gpointer; 21],
}
pub type GTlsInteractionClass = _GTlsInteractionClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InvokeClosure {
    pub mutex: GMutex,
    pub interaction: *mut GTlsInteraction,
    pub argument: *mut GObject,
    pub cancellable: *mut GCancellable,
    pub callback: GAsyncReadyCallback,
    pub user_data: gpointer,
    pub result: GTlsInteractionResult,
    pub error: *mut GError,
    pub complete: gboolean,
    pub cond: GCond,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_g_tls_interaction_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_tls_interaction_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_tls_interaction_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GTlsInteraction_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GTlsInteraction_private_offset,
        );
    }
    safe_c2rust_g_tls_interaction_class_init(klass as *mut GTlsInteractionClass);
}
static mut safe_c2rust_GTlsInteraction_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_tls_interaction_get_instance_private(
    mut self_0: *mut GTlsInteraction,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GTlsInteraction_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_tls_interaction_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GTlsInteraction\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GTlsInteractionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_interaction_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GTlsInteraction>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTlsInteraction) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_tls_interaction_init
                    as unsafe extern "C" fn(*mut GTlsInteraction) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GTlsInteraction_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GTlsInteractionPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_tls_interaction_get_type_once();
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
unsafe extern "C" fn safe_c2rust_invoke_closure_free(mut data: gpointer) {
    let mut closure: *mut InvokeClosure = data as *mut InvokeClosure;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !closure.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            133 as ::core::ffi::c_int,
            G_STRFUNC,
            b"closure\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_object_unref((*closure).interaction as gpointer);
    let mut _pp: *mut *mut GObject = &raw mut (*closure).argument;
    let mut _ptr: *mut GObject = *_pp;
    *_pp = ::core::ptr::null_mut::<GObject>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GCancellable = &raw mut (*closure).cancellable;
    let mut _ptr_0: *mut GCancellable = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    g_cond_clear(&raw mut (*closure).cond);
    g_mutex_clear(&raw mut (*closure).mutex);
    g_clear_error(&raw mut (*closure).error);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*closure).callback.is_none() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            142 as ::core::ffi::c_int,
            G_STRFUNC,
            b"closure->callback == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*closure).user_data.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            143 as ::core::ffi::c_int,
            G_STRFUNC,
            b"closure->user_data == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_free(closure as gpointer);
}
unsafe extern "C" fn safe_c2rust_invoke_closure_new(
    mut interaction: *mut GTlsInteraction,
    mut argument: *mut GObject,
    mut cancellable: *mut GCancellable,
) -> *mut InvokeClosure {
    let mut closure: *mut InvokeClosure = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<InvokeClosure>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut InvokeClosure;
    (*closure).interaction =
        g_object_ref(interaction as gpointer) as *mut GTlsInteraction as *mut GTlsInteraction;
    (*closure).argument = (if !argument.is_null() {
        g_object_ref(argument as gpointer) as *mut GObject
    } else {
        ::core::ptr::null_mut::<GObject>()
    }) as *mut GObject;
    (*closure).cancellable = (if !cancellable.is_null() {
        g_object_ref(cancellable as gpointer) as *mut GCancellable
    } else {
        ::core::ptr::null_mut::<GCancellable>()
    }) as *mut GCancellable;
    g_mutex_init(&raw mut (*closure).mutex);
    g_cond_init(&raw mut (*closure).cond);
    (*closure).result = G_TLS_INTERACTION_UNHANDLED;
    return closure;
}
unsafe extern "C" fn safe_c2rust_invoke_closure_wait_and_free(
    mut closure: *mut InvokeClosure,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut result: GTlsInteractionResult = G_TLS_INTERACTION_UNHANDLED;
    g_mutex_lock(&raw mut (*closure).mutex);
    while (*closure).complete == 0 {
        g_cond_wait(&raw mut (*closure).cond, &raw mut (*closure).mutex);
    }
    g_mutex_unlock(&raw mut (*closure).mutex);
    if !(*closure).error.is_null() {
        g_propagate_error(error, (*closure).error);
        (*closure).error = ::core::ptr::null_mut::<GError>();
    }
    result = (*closure).result;
    safe_c2rust_invoke_closure_free(closure as gpointer);
    return result;
}
unsafe extern "C" fn safe_c2rust_invoke_closure_complete_and_free(
    mut interaction: *mut GTlsInteraction,
    mut closure: *mut InvokeClosure,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut result: GTlsInteractionResult = G_TLS_INTERACTION_UNHANDLED;
    let mut complete: gboolean = 0;
    if g_main_context_acquire((*(*interaction).priv_0).context) != 0 {
        loop {
            g_mutex_lock(&raw mut (*closure).mutex);
            complete = (*closure).complete;
            g_mutex_unlock(&raw mut (*closure).mutex);
            if complete != 0 {
                break;
            }
            g_main_context_iteration((*(*interaction).priv_0).context, TRUE);
        }
        g_main_context_release((*(*interaction).priv_0).context);
        if !(*closure).error.is_null() {
            g_propagate_error(error, (*closure).error);
            (*closure).error = ::core::ptr::null_mut::<GError>();
        }
        result = (*closure).result;
        safe_c2rust_invoke_closure_free(closure as gpointer);
    } else {
        result = safe_c2rust_invoke_closure_wait_and_free(closure, error);
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_g_tls_interaction_init(mut interaction: *mut GTlsInteraction) {
    (*interaction).priv_0 = safe_c2rust_g_tls_interaction_get_instance_private(interaction)
        as *mut GTlsInteractionPrivate;
    (*(*interaction).priv_0).context = g_main_context_ref_thread_default();
}
unsafe extern "C" fn safe_c2rust_g_tls_interaction_finalize(mut object: *mut GObject) {
    let mut interaction: *mut GTlsInteraction =
        object as *mut ::core::ffi::c_void as *mut GTlsInteraction;
    g_main_context_unref((*(*interaction).priv_0).context);
    (*(safe_c2rust_g_tls_interaction_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_tls_interaction_class_init(
    mut klass: *mut GTlsInteractionClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_tls_interaction_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_on_invoke_ask_password_sync(mut user_data: gpointer) -> gboolean {
    let mut closure: *mut InvokeClosure = user_data as *mut InvokeClosure;
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    g_mutex_lock(&raw mut (*closure).mutex);
    klass = (*((*closure).interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*klass).ask_password.is_some() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            270 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->ask_password\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*closure).result = (*klass).ask_password.expect("non-null function pointer")(
        (*closure).interaction,
        (*closure).argument as *mut ::core::ffi::c_void as *mut GTlsPassword,
        (*closure).cancellable,
        &raw mut (*closure).error,
    );
    (*closure).complete = TRUE as gboolean;
    g_cond_signal(&raw mut (*closure).cond);
    g_mutex_unlock(&raw mut (*closure).mutex);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_on_ask_password_complete(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut closure: *mut InvokeClosure = user_data as *mut InvokeClosure;
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    g_mutex_lock(&raw mut (*closure).mutex);
    klass = (*((*closure).interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*klass).ask_password_finish.is_some() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            295 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->ask_password_finish\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*closure).result = (*klass)
        .ask_password_finish
        .expect("non-null function pointer")(
        (*closure).interaction,
        result,
        &raw mut (*closure).error,
    );
    (*closure).complete = TRUE as gboolean;
    g_cond_signal(&raw mut (*closure).cond);
    g_mutex_unlock(&raw mut (*closure).mutex);
}
unsafe extern "C" fn safe_c2rust_on_invoke_ask_password_async_as_sync(
    mut user_data: gpointer,
) -> gboolean {
    let mut closure: *mut InvokeClosure = user_data as *mut InvokeClosure;
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    g_mutex_lock(&raw mut (*closure).mutex);
    klass = (*((*closure).interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*klass).ask_password_async.is_some() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            315 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->ask_password_async\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*klass)
        .ask_password_async
        .expect("non-null function pointer")(
        (*closure).interaction,
        (*closure).argument as *mut ::core::ffi::c_void as *mut GTlsPassword,
        (*closure).cancellable,
        Some(
            safe_c2rust_on_ask_password_complete
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        closure as gpointer,
    );
    (*closure).callback = None;
    (*closure).user_data = NULL as gpointer;
    g_mutex_unlock(&raw mut (*closure).mutex);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_invoke_ask_password(
    mut interaction: *mut GTlsInteraction,
    mut password: *mut GTlsPassword,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut result: GTlsInteractionResult = G_TLS_INTERACTION_UNHANDLED;
    let mut closure: *mut InvokeClosure = ::core::ptr::null_mut::<InvokeClosure>();
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).ask_password.is_some() {
        closure = safe_c2rust_invoke_closure_new(
            interaction,
            password as *mut ::core::ffi::c_void as *mut GObject,
            cancellable,
        );
        g_main_context_invoke(
            (*(*interaction).priv_0).context,
            Some(
                safe_c2rust_on_invoke_ask_password_sync
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            closure as gpointer,
        );
        result = safe_c2rust_invoke_closure_wait_and_free(closure, error);
    } else if (*klass).ask_password_async.is_some() {
        if ({
            let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
            if (*klass).ask_password_finish.is_some() {
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
                b"klass->ask_password_finish\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return G_TLS_INTERACTION_UNHANDLED;
        }
        closure = safe_c2rust_invoke_closure_new(
            interaction,
            password as *mut ::core::ffi::c_void as *mut GObject,
            cancellable,
        );
        g_main_context_invoke(
            (*(*interaction).priv_0).context,
            Some(
                safe_c2rust_on_invoke_ask_password_async_as_sync
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            closure as gpointer,
        );
        result = safe_c2rust_invoke_closure_complete_and_free(interaction, closure, error);
    } else {
        result = G_TLS_INTERACTION_UNHANDLED;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_ask_password(
    mut interaction: *mut GTlsInteraction,
    mut password: *mut GTlsPassword,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).ask_password.is_some() {
        return (*klass).ask_password.expect("non-null function pointer")(
            interaction,
            password,
            cancellable,
            error,
        );
    } else {
        return G_TLS_INTERACTION_UNHANDLED;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_ask_password_async(
    mut interaction: *mut GTlsInteraction,
    mut password: *mut GTlsPassword,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = password as *mut GTypeInstance;
            let mut __t: GType = g_tls_password_get_type();
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
            b"G_IS_TLS_PASSWORD (password)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).ask_password_async.is_some() {
        if ({
            let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
            if (*klass).ask_password_finish.is_some() {
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
                b"klass->ask_password_finish\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        (*klass)
            .ask_password_async
            .expect("non-null function pointer")(
            interaction,
            password,
            cancellable,
            callback,
            user_data,
        );
    } else {
        task = g_task_new(interaction as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GTlsInteraction,
                        *mut GTlsPassword,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_tls_interaction_ask_password_async
                    as unsafe extern "C" fn(
                        *mut GTlsInteraction,
                        *mut GTlsPassword,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        );
        if g_task_get_name(_task).is_null() {
            g_task_set_static_name(
                _task,
                b"g_tls_interaction_ask_password_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_int(
            task,
            G_TLS_INTERACTION_UNHANDLED as ::core::ffi::c_int as gssize,
        );
        g_object_unref(task as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_ask_password_finish(
    mut interaction: *mut GTlsInteraction,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).ask_password_finish.is_some() {
        if ({
            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
            if (*klass).ask_password_async.is_some() {
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
                b"klass->ask_password_async != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return G_TLS_INTERACTION_UNHANDLED;
        }
        return (*klass)
            .ask_password_finish
            .expect("non-null function pointer")(interaction, result, error);
    } else {
        if ({
            let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
            if g_async_result_is_tagged(
                result,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut GTlsInteraction,
                            *mut GTlsPassword,
                            *mut GCancellable,
                            GAsyncReadyCallback,
                            gpointer,
                        ) -> (),
                    >,
                    gpointer,
                >(Some(
                    safe_c2rust_g_tls_interaction_ask_password_async
                        as unsafe extern "C" fn(
                            *mut GTlsInteraction,
                            *mut GTlsPassword,
                            *mut GCancellable,
                            GAsyncReadyCallback,
                            gpointer,
                        ) -> (),
                )),
            ) != 0
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
                b"g_async_result_is_tagged (result, g_tls_interaction_ask_password_async)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return G_TLS_INTERACTION_UNHANDLED;
        }
        return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error)
            as GTlsInteractionResult;
    };
}
unsafe extern "C" fn safe_c2rust_on_invoke_request_certificate_sync(
    mut user_data: gpointer,
) -> gboolean {
    let mut closure: *mut InvokeClosure = user_data as *mut InvokeClosure;
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    g_mutex_lock(&raw mut (*closure).mutex);
    klass = (*((*closure).interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*klass).request_certificate.is_some() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            558 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->request_certificate != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*closure).result = (*klass)
        .request_certificate
        .expect("non-null function pointer")(
        (*closure).interaction,
        (*closure).argument as *mut ::core::ffi::c_void as *mut GTlsConnection,
        G_TLS_CERTIFICATE_REQUEST_NONE,
        (*closure).cancellable,
        &raw mut (*closure).error,
    );
    (*closure).complete = TRUE as gboolean;
    g_cond_signal(&raw mut (*closure).cond);
    g_mutex_unlock(&raw mut (*closure).mutex);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_on_request_certificate_complete(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut closure: *mut InvokeClosure = user_data as *mut InvokeClosure;
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    g_mutex_lock(&raw mut (*closure).mutex);
    klass = (*((*closure).interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*klass).request_certificate_finish.is_some() {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            584 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->request_certificate_finish != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*closure).result = (*klass)
        .request_certificate_finish
        .expect("non-null function pointer")(
        (*closure).interaction,
        result,
        &raw mut (*closure).error,
    );
    (*closure).complete = TRUE as gboolean;
    g_cond_signal(&raw mut (*closure).cond);
    g_mutex_unlock(&raw mut (*closure).mutex);
}
unsafe extern "C" fn safe_c2rust_on_invoke_request_certificate_async_as_sync(
    mut user_data: gpointer,
) -> gboolean {
    let mut closure: *mut InvokeClosure = user_data as *mut InvokeClosure;
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    g_mutex_lock(&raw mut (*closure).mutex);
    klass = (*((*closure).interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if (*klass).request_certificate_async.is_some() {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtlsinteraction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            603 as ::core::ffi::c_int,
            G_STRFUNC,
            b"klass->request_certificate_async\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*klass)
        .request_certificate_async
        .expect("non-null function pointer")(
        (*closure).interaction,
        (*closure).argument as *mut ::core::ffi::c_void as *mut GTlsConnection,
        G_TLS_CERTIFICATE_REQUEST_NONE,
        (*closure).cancellable,
        Some(
            safe_c2rust_on_request_certificate_complete
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        closure as gpointer,
    );
    (*closure).callback = None;
    (*closure).user_data = NULL as gpointer;
    g_mutex_unlock(&raw mut (*closure).mutex);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_invoke_request_certificate(
    mut interaction: *mut GTlsInteraction,
    mut connection: *mut GTlsConnection,
    mut flags: GTlsCertificateRequestFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut result: GTlsInteractionResult = G_TLS_INTERACTION_UNHANDLED;
    let mut closure: *mut InvokeClosure = ::core::ptr::null_mut::<InvokeClosure>();
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).request_certificate.is_some() {
        closure = safe_c2rust_invoke_closure_new(
            interaction,
            connection as *mut ::core::ffi::c_void as *mut GObject,
            cancellable,
        );
        g_main_context_invoke(
            (*(*interaction).priv_0).context,
            Some(
                safe_c2rust_on_invoke_request_certificate_sync
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            closure as gpointer,
        );
        result = safe_c2rust_invoke_closure_wait_and_free(closure, error);
    } else if (*klass).request_certificate_async.is_some() {
        if ({
            let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
            if (*klass).request_certificate_finish.is_some() {
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
                b"klass->request_certificate_finish\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return G_TLS_INTERACTION_UNHANDLED;
        }
        closure = safe_c2rust_invoke_closure_new(
            interaction,
            connection as *mut ::core::ffi::c_void as *mut GObject,
            cancellable,
        );
        g_main_context_invoke(
            (*(*interaction).priv_0).context,
            Some(
                safe_c2rust_on_invoke_request_certificate_async_as_sync
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            closure as gpointer,
        );
        result = safe_c2rust_invoke_closure_complete_and_free(interaction, closure, error);
    } else {
        result = G_TLS_INTERACTION_UNHANDLED;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_request_certificate(
    mut interaction: *mut GTlsInteraction,
    mut connection: *mut GTlsConnection,
    mut flags: GTlsCertificateRequestFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).request_certificate.is_some() {
        return (*klass)
            .request_certificate
            .expect("non-null function pointer")(
            interaction,
            connection,
            flags,
            cancellable,
            error,
        );
    } else {
        return G_TLS_INTERACTION_UNHANDLED;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_request_certificate_async(
    mut interaction: *mut GTlsInteraction,
    mut connection: *mut GTlsConnection,
    mut flags: GTlsCertificateRequestFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_tls_connection_get_type();
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
            b"G_IS_TLS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).request_certificate_async.is_some() {
        if ({
            let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
            if (*klass).request_certificate_finish.is_some() {
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
                b"klass->request_certificate_finish\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        (*klass)
            .request_certificate_async
            .expect("non-null function pointer")(
            interaction,
            connection,
            flags,
            cancellable,
            callback,
            user_data,
        );
    } else {
        task = g_task_new(interaction as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GTlsInteraction,
                        *mut GTlsConnection,
                        GTlsCertificateRequestFlags,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_tls_interaction_request_certificate_async
                    as unsafe extern "C" fn(
                        *mut GTlsInteraction,
                        *mut GTlsConnection,
                        GTlsCertificateRequestFlags,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        );
        if g_task_get_name(_task).is_null() {
            g_task_set_static_name(
                _task,
                b"g_tls_interaction_request_certificate_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_int(
            task,
            G_TLS_INTERACTION_UNHANDLED as ::core::ffi::c_int as gssize,
        );
        g_object_unref(task as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_request_certificate_finish(
    mut interaction: *mut GTlsInteraction,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> GTlsInteractionResult {
    let mut klass: *mut GTlsInteractionClass = ::core::ptr::null_mut::<GTlsInteractionClass>();
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interaction as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_tls_interaction_get_type();
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
            b"G_IS_TLS_INTERACTION (interaction)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_TLS_INTERACTION_UNHANDLED;
    }
    klass = (*(interaction as *mut GTypeInstance)).g_class as *mut GTlsInteractionClass;
    if (*klass).request_certificate_finish.is_some() {
        if ({
            let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
            if (*klass).request_certificate_async.is_some() {
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
                b"klass->request_certificate_async != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return G_TLS_INTERACTION_UNHANDLED;
        }
        return (*klass)
            .request_certificate_finish
            .expect("non-null function pointer")(interaction, result, error);
    } else {
        if ({
            let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
            if g_async_result_is_tagged(
                result,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut GTlsInteraction,
                            *mut GTlsConnection,
                            GTlsCertificateRequestFlags,
                            *mut GCancellable,
                            GAsyncReadyCallback,
                            gpointer,
                        ) -> (),
                    >,
                    gpointer,
                >(Some(
                    safe_c2rust_g_tls_interaction_request_certificate_async
                        as unsafe extern "C" fn(
                            *mut GTlsInteraction,
                            *mut GTlsConnection,
                            GTlsCertificateRequestFlags,
                            *mut GCancellable,
                            GAsyncReadyCallback,
                            gpointer,
                        ) -> (),
                )),
            ) != 0
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
                b"g_async_result_is_tagged (result, g_tls_interaction_request_certificate_async)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return G_TLS_INTERACTION_UNHANDLED;
        }
        return g_task_propagate_int(result as *mut ::core::ffi::c_void as *mut GTask, error)
            as GTlsInteractionResult;
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
