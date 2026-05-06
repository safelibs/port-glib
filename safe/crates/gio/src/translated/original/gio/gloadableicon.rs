extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GInputStream;
    pub type _GLoadableIcon;
    pub type _GTask;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_object_unref(object: gpointer);
    fn g_icon_get_type() -> GType;
    fn g_async_result_get_type() -> GType;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
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
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GInputStream = _GInputStream;
pub type GLoadableIcon = _GLoadableIcon;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLoadableIconIface {
    pub g_iface: GTypeInterface,
    pub load: Option<
        unsafe extern "C" fn(
            *mut GLoadableIcon,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GInputStream,
    >,
    pub load_async: Option<
        unsafe extern "C" fn(
            *mut GLoadableIcon,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub load_finish: Option<
        unsafe extern "C" fn(
            *mut GLoadableIcon,
            *mut GAsyncResult,
            *mut *mut ::core::ffi::c_char,
            *mut *mut GError,
        ) -> *mut GInputStream,
    >,
}
pub type GLoadableIconIface = _GLoadableIconIface;
pub type GLoadableIconInterface = GLoadableIconIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadData {
    pub size: ::core::ffi::c_int,
    pub type_0: *mut ::core::ffi::c_char,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_loadable_icon_get_type() -> GType {
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
            g_intern_static_string(b"GLoadableIcon\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GLoadableIconInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GLoadableIconIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_loadable_icon_default_init
                        as unsafe extern "C" fn(*mut GLoadableIconIface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if g_icon_get_type() != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(g_define_type_id, g_icon_get_type());
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
unsafe extern "C" fn safe_c2rust_g_loadable_icon_default_init(mut iface: *mut GLoadableIconIface) {
    (*iface).load_async = Some(
        safe_c2rust_g_loadable_icon_real_load_async
            as unsafe extern "C" fn(
                *mut GLoadableIcon,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GLoadableIcon,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).load_finish = Some(
        safe_c2rust_g_loadable_icon_real_load_finish
            as unsafe extern "C" fn(
                *mut GLoadableIcon,
                *mut GAsyncResult,
                *mut *mut ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GInputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GLoadableIcon,
                *mut GAsyncResult,
                *mut *mut ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GInputStream,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_loadable_icon_load(
    mut icon: *mut GLoadableIcon,
    mut size: ::core::ffi::c_int,
    mut type_0: *mut *mut ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GInputStream {
    let mut iface: *mut GLoadableIconIface = ::core::ptr::null_mut::<GLoadableIconIface>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_loadable_icon_get_type();
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
            b"G_IS_LOADABLE_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    iface = g_type_interface_peek(
        (*(icon as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_loadable_icon_get_type(),
    ) as *mut GLoadableIconIface;
    return Some((*iface).load.expect("non-null function pointer"))
        .expect("non-null function pointer")(icon, size, type_0, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_loadable_icon_load_async(
    mut icon: *mut GLoadableIcon,
    mut size: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GLoadableIconIface = ::core::ptr::null_mut::<GLoadableIconIface>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_loadable_icon_get_type();
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
            b"G_IS_LOADABLE_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(icon as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_loadable_icon_get_type(),
    ) as *mut GLoadableIconIface;
    Some((*iface).load_async.expect("non-null function pointer"))
        .expect("non-null function pointer")(icon, size, cancellable, callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_loadable_icon_load_finish(
    mut icon: *mut GLoadableIcon,
    mut res: *mut GAsyncResult,
    mut type_0: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GInputStream {
    let mut iface: *mut GLoadableIconIface = ::core::ptr::null_mut::<GLoadableIconIface>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_loadable_icon_get_type();
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
            b"G_IS_LOADABLE_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
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
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    if g_async_result_legacy_propagate_error(res, error) != 0 {
        return ::core::ptr::null_mut::<GInputStream>();
    }
    iface = g_type_interface_peek(
        (*(icon as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_loadable_icon_get_type(),
    ) as *mut GLoadableIconIface;
    return Some((*iface).load_finish.expect("non-null function pointer"))
        .expect("non-null function pointer")(icon, res, type_0, error);
}
unsafe extern "C" fn safe_c2rust_load_data_free(mut data: *mut LoadData) {
    g_free((*data).type_0 as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_load_async_thread(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut icon: *mut GLoadableIcon = source_object as *mut GLoadableIcon;
    let mut data: *mut LoadData = task_data as *mut LoadData;
    let mut iface: *mut GLoadableIconIface = ::core::ptr::null_mut::<GLoadableIconIface>();
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    iface = g_type_interface_peek(
        (*(icon as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_loadable_icon_get_type(),
    ) as *mut GLoadableIconIface;
    stream = (*iface).load.expect("non-null function pointer")(
        icon,
        (*data).size,
        &raw mut (*data).type_0,
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
unsafe extern "C" fn safe_c2rust_g_loadable_icon_real_load_async(
    mut icon: *mut GLoadableIcon,
    mut size: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LoadData = ::core::ptr::null_mut::<LoadData>();
    task = g_task_new(icon as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GLoadableIcon,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_loadable_icon_real_load_async
                as unsafe extern "C" fn(
                    *mut GLoadableIcon,
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
            b"g_loadable_icon_real_load_async\0" as *const u8 as *const gchar,
        );
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LoadData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LoadData;
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LoadData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_load_data_free as unsafe extern "C" fn(*mut LoadData) -> ()),
        ),
    );
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_load_async_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_loadable_icon_real_load_finish(
    mut icon: *mut GLoadableIcon,
    mut res: *mut GAsyncResult,
    mut type_0: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GInputStream {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LoadData = ::core::ptr::null_mut::<LoadData>();
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, icon as gpointer) != 0 {
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
            b"g_task_is_valid (res, icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    task = res as *mut ::core::ffi::c_void as *mut GTask;
    data = g_task_get_task_data(task) as *mut LoadData;
    stream = g_task_propagate_pointer(task, error) as *mut GInputStream;
    if !stream.is_null() && !type_0.is_null() {
        *type_0 = (*data).type_0;
        (*data).type_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return stream;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
