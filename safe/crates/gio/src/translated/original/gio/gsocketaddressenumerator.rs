extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GSocketAddress;
    pub type _GTask;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_unref(object: gpointer);
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumerator {
    pub parent_instance: GObject,
}
pub type GSocketAddressEnumerator = _GSocketAddressEnumerator;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddressEnumeratorClass {
    pub parent_class: GObjectClass,
    pub next: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GSocketAddress,
    >,
    pub next_async: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub next_finish: Option<
        unsafe extern "C" fn(
            *mut GSocketAddressEnumerator,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GSocketAddress,
    >,
}
pub type GSocketAddressEnumeratorClass = _GSocketAddressEnumeratorClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_socket_address_enumerator_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSocketAddressEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSocketAddressEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_address_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSocketAddressEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSocketAddressEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_socket_address_enumerator_init
                    as unsafe extern "C" fn(*mut GSocketAddressEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust_g_socket_address_enumerator_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSocketAddressEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSocketAddressEnumerator_private_offset,
        );
    }
    safe_c2rust_g_socket_address_enumerator_class_init(klass as *mut GSocketAddressEnumeratorClass);
}
static mut safe_c2rust_GSocketAddressEnumerator_private_offset: gint = 0;
static mut safe_c2rust_g_socket_address_enumerator_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_init(
    mut enumerator: *mut GSocketAddressEnumerator,
) {
}
unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_class_init(
    mut enumerator_class: *mut GSocketAddressEnumeratorClass,
) {
    (*enumerator_class).next_async = Some(
        safe_c2rust_g_socket_address_enumerator_real_next_async
            as unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*enumerator_class).next_finish = Some(
        safe_c2rust_g_socket_address_enumerator_real_next_finish
            as unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketAddressEnumerator,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GSocketAddress,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_next(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut klass: *mut GSocketAddressEnumeratorClass =
        ::core::ptr::null_mut::<GSocketAddressEnumeratorClass>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_address_enumerator_get_type();
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
            b"G_IS_SOCKET_ADDRESS_ENUMERATOR (enumerator)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    klass = (*(enumerator as *mut GTypeInstance)).g_class as *mut GSocketAddressEnumeratorClass;
    return Some((*klass).next.expect("non-null function pointer"))
        .expect("non-null function pointer")(enumerator, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_real_next_async(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut address: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    task = g_task_new(
        enumerator as gpointer,
        ::core::ptr::null_mut::<GCancellable>(),
        callback,
        user_data,
    );
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GSocketAddressEnumerator,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_socket_address_enumerator_real_next_async
                as unsafe extern "C" fn(
                    *mut GSocketAddressEnumerator,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_socket_address_enumerator_real_next_async\0" as *const u8 as *const gchar,
        );
    }
    address = safe_c2rust_g_socket_address_enumerator_next(enumerator, cancellable, &raw mut error);
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            address as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_next_async(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut klass: *mut GSocketAddressEnumeratorClass =
        ::core::ptr::null_mut::<GSocketAddressEnumeratorClass>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_address_enumerator_get_type();
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
            b"G_IS_SOCKET_ADDRESS_ENUMERATOR (enumerator)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    klass = (*(enumerator as *mut GTypeInstance)).g_class as *mut GSocketAddressEnumeratorClass;
    Some((*klass).next_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(enumerator, cancellable, callback, user_data);
}
unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_real_next_finish(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, enumerator as gpointer) != 0 {
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
            b"g_task_is_valid (result, enumerator)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GSocketAddress;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_address_enumerator_next_finish(
    mut enumerator: *mut GSocketAddressEnumerator,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GSocketAddress {
    let mut klass: *mut GSocketAddressEnumeratorClass =
        ::core::ptr::null_mut::<GSocketAddressEnumeratorClass>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = enumerator as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_socket_address_enumerator_get_type();
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
            b"G_IS_SOCKET_ADDRESS_ENUMERATOR (enumerator)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSocketAddress>();
    }
    klass = (*(enumerator as *mut GTypeInstance)).g_class as *mut GSocketAddressEnumeratorClass;
    return Some((*klass).next_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(enumerator, result, error);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
