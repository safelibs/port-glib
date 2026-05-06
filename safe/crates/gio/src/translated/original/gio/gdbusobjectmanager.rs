use ::c2rust_bitfields;
extern "C" {
    pub type _GDBusInterface;
    pub type _GDBusObject;
    pub type _GDBusObjectManager;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
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
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        ...
    ) -> guint;
    fn g_signal_set_va_marshaller(
        signal_id: guint,
        instance_type: GType,
        va_marshaller: GSignalCVaMarshaller,
    );
    fn g_dbus_object_get_type() -> GType;
    fn g_dbus_interface_get_type() -> GType;
    fn g_dbus_is_interface_name(string: *const gchar) -> gboolean;
    fn _g_cclosure_marshal_VOID__OBJECT_OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__OBJECT_OBJECTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
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
pub type GVaClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        gpointer,
        ::core::ffi::VaList,
        gpointer,
        ::core::ffi::c_int,
        *mut GType,
    ) -> (),
>;
pub type GSignalFlags = ::core::ffi::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalCVaMarshaller = GVaClosureMarshal;
pub type GSignalAccumulator = Option<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
pub type GDBusInterface = _GDBusInterface;
pub type GDBusObject = _GDBusObject;
pub type GDBusObjectManager = _GDBusObjectManager;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerIface {
    pub parent_iface: GTypeInterface,
    pub get_object_path: Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *const gchar>,
    pub get_objects: Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *mut GList>,
    pub get_object:
        Option<unsafe extern "C" fn(*mut GDBusObjectManager, *const gchar) -> *mut GDBusObject>,
    pub get_interface: Option<
        unsafe extern "C" fn(
            *mut GDBusObjectManager,
            *const gchar,
            *const gchar,
        ) -> *mut GDBusInterface,
    >,
    pub object_added: Option<unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject) -> ()>,
    pub object_removed:
        Option<unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject) -> ()>,
    pub interface_added: Option<
        unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject, *mut GDBusInterface) -> (),
    >,
    pub interface_removed: Option<
        unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject, *mut GDBusInterface) -> (),
    >,
}
pub type GDBusObjectManagerIface = _GDBusObjectManagerIface;
pub type GDBusObjectManagerInterface = GDBusObjectManagerIface;
pub const INTERFACE_REMOVED: C2RustUnnamed_0 = 3;
pub const INTERFACE_ADDED: C2RustUnnamed_0 = 2;
pub const OBJECT_REMOVED: C2RustUnnamed_0 = 1;
pub const OBJECT_ADDED: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const N_SIGNALS: C2RustUnnamed_0 = 4;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_get_type() -> GType {
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
            g_intern_static_string(b"GDBusObjectManager\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GDBusObjectManagerInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GDBusObjectManagerIface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_dbus_object_manager_default_init
                        as unsafe extern "C" fn(*mut GDBusObjectManagerIface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
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
static mut safe_c2rust_signals: [guint; 4] = [0; 4];
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_default_init(
    mut iface: *mut GDBusObjectManagerIface,
) {
    safe_c2rust_signals[OBJECT_ADDED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"object-added\0" as *const u8 as *const gchar),
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        48 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_dbus_object_get_type(),
    );
    safe_c2rust_signals[OBJECT_REMOVED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"object-removed\0" as *const u8 as *const gchar),
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        56 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_dbus_object_get_type(),
    );
    safe_c2rust_signals[INTERFACE_ADDED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"interface-added\0" as *const u8 as *const gchar),
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        64 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_OBJECT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        2 as guint,
        g_dbus_object_get_type(),
        g_dbus_interface_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[INTERFACE_ADDED as ::core::ffi::c_int as usize],
        (*(iface as *mut GTypeInterface)).g_type,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_OBJECTv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
    safe_c2rust_signals[INTERFACE_REMOVED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"interface-removed\0" as *const u8 as *const gchar),
        (*(iface as *mut GTypeInterface)).g_type,
        G_SIGNAL_RUN_LAST,
        72 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_OBJECT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        2 as guint,
        g_dbus_object_get_type(),
        g_dbus_interface_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[INTERFACE_REMOVED as ::core::ffi::c_int as usize],
        (*(iface as *mut GTypeInterface)).g_type,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_OBJECTv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_get_object_path(
    mut manager: *mut GDBusObjectManager,
) -> *const gchar {
    let mut iface: *mut GDBusObjectManagerIface = g_type_interface_peek(
        (*(manager as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dbus_object_manager_get_type(),
    ) as *mut GDBusObjectManagerIface;
    return (*iface).get_object_path.expect("non-null function pointer")(manager);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_get_objects(
    mut manager: *mut GDBusObjectManager,
) -> *mut GList {
    let mut iface: *mut GDBusObjectManagerIface = g_type_interface_peek(
        (*(manager as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dbus_object_manager_get_type(),
    ) as *mut GDBusObjectManagerIface;
    return (*iface).get_objects.expect("non-null function pointer")(manager);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_get_object(
    mut manager: *mut GDBusObjectManager,
    mut object_path: *const gchar,
) -> *mut GDBusObject {
    let mut iface: *mut GDBusObjectManagerIface = g_type_interface_peek(
        (*(manager as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dbus_object_manager_get_type(),
    ) as *mut GDBusObjectManagerIface;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObject>();
    }
    return (*iface).get_object.expect("non-null function pointer")(manager, object_path);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_get_interface(
    mut manager: *mut GDBusObjectManager,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
) -> *mut GDBusInterface {
    let mut iface: *mut GDBusObjectManagerIface = g_type_interface_peek(
        (*(manager as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_dbus_object_manager_get_type(),
    ) as *mut GDBusObjectManagerIface;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterface>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterface>();
    }
    return (*iface).get_interface.expect("non-null function pointer")(
        manager,
        object_path,
        interface_name,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
