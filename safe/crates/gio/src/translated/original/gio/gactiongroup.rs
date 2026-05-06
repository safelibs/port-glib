use ::c2rust_bitfields;
extern "C" {
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GActionGroup;
    fn g_quark_try_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_name(type_0: GType) -> *const gchar;
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
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn _g_cclosure_marshal_VOID__STRING_BOOLEAN(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__STRING_BOOLEANv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_VOID__STRING_VARIANT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__STRING_VARIANTv(
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
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
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
pub type GActionGroup = _GActionGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionGroupInterface {
    pub g_iface: GTypeInterface,
    pub has_action: Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean>,
    pub list_actions: Option<unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar>,
    pub get_action_enabled:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean>,
    pub get_action_parameter_type:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType>,
    pub get_action_state_type:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType>,
    pub get_action_state_hint:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant>,
    pub get_action_state:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant>,
    pub change_action_state:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>,
    pub activate_action:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>,
    pub action_added: Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> ()>,
    pub action_removed: Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> ()>,
    pub action_enabled_changed:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean) -> ()>,
    pub action_state_changed:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>,
    pub query_action: Option<
        unsafe extern "C" fn(
            *mut GActionGroup,
            *const gchar,
            *mut gboolean,
            *mut *const GVariantType,
            *mut *const GVariantType,
            *mut *mut GVariant,
            *mut *mut GVariant,
        ) -> gboolean,
    >,
}
pub type GActionGroupInterface = _GActionGroupInterface;
pub const SIGNAL_ACTION_STATE_CHANGED: C2RustUnnamed_0 = 3;
pub const SIGNAL_ACTION_ENABLED_CHANGED: C2RustUnnamed_0 = 2;
pub const SIGNAL_ACTION_REMOVED: C2RustUnnamed_0 = 1;
pub const SIGNAL_ACTION_ADDED: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const NR_SIGNALS: C2RustUnnamed_0 = 4;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_get_type() -> GType {
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
            g_intern_static_string(b"GActionGroup\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GActionGroupInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GActionGroupInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_action_group_default_init
                        as unsafe extern "C" fn(*mut GActionGroupInterface) -> (),
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
static mut safe_c2rust_g_action_group_signals: [guint; 4] = [0; 4];
unsafe extern "C" fn safe_c2rust_g_action_group_real_has_action(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> gboolean {
    return safe_c2rust_g_action_group_query_action(
        action_group,
        action_name,
        ::core::ptr::null_mut::<gboolean>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_action_group_real_get_action_enabled(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> gboolean {
    let mut enabled: gboolean = 0;
    if safe_c2rust_g_action_group_query_action(
        action_group,
        action_name,
        &raw mut enabled,
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
    ) == 0
    {
        return FALSE;
    }
    return enabled;
}
unsafe extern "C" fn safe_c2rust_g_action_group_real_get_action_parameter_type(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *const GVariantType {
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    if safe_c2rust_g_action_group_query_action(
        action_group,
        action_name,
        ::core::ptr::null_mut::<gboolean>(),
        &raw mut type_0,
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
    ) == 0
    {
        return ::core::ptr::null::<GVariantType>();
    }
    return type_0;
}
unsafe extern "C" fn safe_c2rust_g_action_group_real_get_action_state_type(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *const GVariantType {
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    if safe_c2rust_g_action_group_query_action(
        action_group,
        action_name,
        ::core::ptr::null_mut::<gboolean>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        &raw mut type_0,
        ::core::ptr::null_mut::<*mut GVariant>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
    ) == 0
    {
        return ::core::ptr::null::<GVariantType>();
    }
    return type_0;
}
unsafe extern "C" fn safe_c2rust_g_action_group_real_get_action_state_hint(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *mut GVariant {
    let mut hint: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if safe_c2rust_g_action_group_query_action(
        action_group,
        action_name,
        ::core::ptr::null_mut::<gboolean>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        &raw mut hint,
        ::core::ptr::null_mut::<*mut GVariant>(),
    ) == 0
    {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return hint;
}
unsafe extern "C" fn safe_c2rust_g_action_group_real_get_action_state(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *mut GVariant {
    let mut state: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if safe_c2rust_g_action_group_query_action(
        action_group,
        action_name,
        ::core::ptr::null_mut::<gboolean>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*const GVariantType>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
        &raw mut state,
    ) == 0
    {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return state;
}
unsafe extern "C" fn safe_c2rust_g_action_group_real_query_action(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut enabled: *mut gboolean,
    mut parameter_type: *mut *const GVariantType,
    mut state_type: *mut *const GVariantType,
    mut state_hint: *mut *mut GVariant,
    mut state: *mut *mut GVariant,
) -> gboolean {
    let mut iface: *mut GActionGroupInterface = g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*iface).has_action
            == Some(
                safe_c2rust_g_action_group_real_has_action
                    as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean,
            )
            || (*iface).get_action_enabled
                == Some(
                    safe_c2rust_g_action_group_real_get_action_enabled
                        as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean,
                )
            || (*iface).get_action_parameter_type
                == Some(
                    safe_c2rust_g_action_group_real_get_action_parameter_type
                        as unsafe extern "C" fn(
                            *mut GActionGroup,
                            *const gchar,
                        ) -> *const GVariantType,
                )
            || (*iface).get_action_state_type
                == Some(
                    safe_c2rust_g_action_group_real_get_action_state_type
                        as unsafe extern "C" fn(
                            *mut GActionGroup,
                            *const gchar,
                        ) -> *const GVariantType,
                )
            || (*iface).get_action_state_hint
                == Some(
                    safe_c2rust_g_action_group_real_get_action_state_hint
                        as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant,
                )
            || (*iface).get_action_state
                == Some(
                    safe_c2rust_g_action_group_real_get_action_state
                        as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant,
                )
        {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Class '%s' implements GActionGroup interface without overriding query_action() method -- bailing out to avoid infinite recursion.\0"
                as *const u8 as *const gchar,
            g_type_name((*(*(action_group as *mut GTypeInstance)).g_class).g_type),
        );
        return FALSE;
    }
    if Some((*iface).has_action.expect("non-null function pointer"))
        .expect("non-null function pointer")(action_group, action_name)
        == 0
    {
        return FALSE;
    }
    if !enabled.is_null() {
        *enabled = Some(
            (*iface)
                .get_action_enabled
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(action_group, action_name);
    }
    if !parameter_type.is_null() {
        *parameter_type = Some(
            (*iface)
                .get_action_parameter_type
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(action_group, action_name);
    }
    if !state_type.is_null() {
        *state_type = Some(
            (*iface)
                .get_action_state_type
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(action_group, action_name);
    }
    if !state_hint.is_null() {
        *state_hint = Some(
            (*iface)
                .get_action_state_hint
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(action_group, action_name);
    }
    if !state.is_null() {
        *state = Some(
            (*iface)
                .get_action_state
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(action_group, action_name);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_action_group_default_init(
    mut iface: *mut GActionGroupInterface,
) {
    (*iface).has_action = Some(
        safe_c2rust_g_action_group_real_has_action
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean>;
    (*iface).get_action_enabled = Some(
        safe_c2rust_g_action_group_real_get_action_enabled
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean>;
    (*iface).get_action_parameter_type = Some(
        safe_c2rust_g_action_group_real_get_action_parameter_type
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType>;
    (*iface).get_action_state_type = Some(
        safe_c2rust_g_action_group_real_get_action_state_type
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType>;
    (*iface).get_action_state_hint = Some(
        safe_c2rust_g_action_group_real_get_action_state_hint
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant>;
    (*iface).get_action_state = Some(
        safe_c2rust_g_action_group_real_get_action_state
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant>;
    (*iface).query_action = Some(
        safe_c2rust_g_action_group_real_query_action
            as unsafe extern "C" fn(
                *mut GActionGroup,
                *const gchar,
                *mut gboolean,
                *mut *const GVariantType,
                *mut *const GVariantType,
                *mut *mut GVariant,
                *mut *mut GVariant,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GActionGroup,
                *const gchar,
                *mut gboolean,
                *mut *const GVariantType,
                *mut *const GVariantType,
                *mut *mut GVariant,
                *mut *mut GVariant,
            ) -> gboolean,
        >;
    safe_c2rust_g_action_group_signals[SIGNAL_ACTION_ADDED as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"action-added\0" as *const u8 as *const gchar),
            safe_c2rust_g_action_group_get_type(),
            (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_DETAILED as ::core::ffi::c_int)
                as GSignalFlags,
            88 as ::core::ffi::c_ulong as glong as guint,
            None,
            NULL,
            None,
            G_TYPE_NONE,
            1 as guint,
            G_TYPE_STRING,
        );
    safe_c2rust_g_action_group_signals[SIGNAL_ACTION_REMOVED as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"action-removed\0" as *const u8 as *const gchar),
            safe_c2rust_g_action_group_get_type(),
            (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_DETAILED as ::core::ffi::c_int)
                as GSignalFlags,
            96 as ::core::ffi::c_ulong as glong as guint,
            None,
            NULL,
            None,
            G_TYPE_NONE,
            1 as guint,
            G_TYPE_STRING,
        );
    safe_c2rust_g_action_group_signals
        [SIGNAL_ACTION_ENABLED_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"action-enabled-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_action_group_get_type(),
        (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_DETAILED as ::core::ffi::c_int)
            as GSignalFlags,
        104 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        Some(
            _g_cclosure_marshal_VOID__STRING_BOOLEAN
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
        G_TYPE_STRING,
        G_TYPE_BOOLEAN,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_g_action_group_signals
            [SIGNAL_ACTION_ENABLED_CHANGED as ::core::ffi::c_int as usize],
        (*(iface as *mut GTypeInterface)).g_type,
        Some(
            _g_cclosure_marshal_VOID__STRING_BOOLEANv
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
    safe_c2rust_g_action_group_signals
        [SIGNAL_ACTION_STATE_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"action-state-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_action_group_get_type(),
        (G_SIGNAL_RUN_LAST as ::core::ffi::c_int
            | G_SIGNAL_DETAILED as ::core::ffi::c_int
            | G_SIGNAL_MUST_COLLECT as ::core::ffi::c_int) as GSignalFlags,
        112 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        Some(
            _g_cclosure_marshal_VOID__STRING_VARIANT
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
        G_TYPE_STRING,
        G_TYPE_VARIANT,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_g_action_group_signals
            [SIGNAL_ACTION_STATE_CHANGED as ::core::ffi::c_int as usize],
        (*(iface as *mut GTypeInterface)).g_type,
        Some(
            _g_cclosure_marshal_VOID__STRING_VARIANTv
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
pub unsafe extern "C" fn safe_c2rust_g_action_group_list_actions(
    mut action_group: *mut GActionGroup,
) -> *mut *mut gchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .list_actions
        .expect("non-null function pointer")(action_group);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_has_action(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .has_action
        .expect("non-null function pointer")(action_group, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_get_action_parameter_type(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *const GVariantType {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .get_action_parameter_type
        .expect("non-null function pointer")(action_group, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_get_action_state_type(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *const GVariantType {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .get_action_state_type
        .expect("non-null function pointer")(action_group, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_get_action_state_hint(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .get_action_state_hint
        .expect("non-null function pointer")(action_group, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_get_action_enabled(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .get_action_enabled
        .expect("non-null function pointer")(action_group, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_get_action_state(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .get_action_state
        .expect("non-null function pointer")(action_group, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_change_action_state(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .change_action_state
        .expect("non-null function pointer")(action_group, action_name, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_activate_action(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut parameter: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .activate_action
        .expect("non-null function pointer")(action_group, action_name, parameter);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_action_added(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_signal_emit(
        action_group as gpointer,
        safe_c2rust_g_action_group_signals[SIGNAL_ACTION_ADDED as ::core::ffi::c_int as usize],
        g_quark_try_string(action_name),
        action_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_action_removed(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
) {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_signal_emit(
        action_group as gpointer,
        safe_c2rust_g_action_group_signals[SIGNAL_ACTION_REMOVED as ::core::ffi::c_int as usize],
        g_quark_try_string(action_name),
        action_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_action_enabled_changed(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut enabled: gboolean,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    enabled = (enabled != 0) as ::core::ffi::c_int as gboolean;
    g_signal_emit(
        action_group as gpointer,
        safe_c2rust_g_action_group_signals
            [SIGNAL_ACTION_ENABLED_CHANGED as ::core::ffi::c_int as usize],
        g_quark_try_string(action_name),
        action_name,
        enabled,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_action_state_changed(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut state: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_group as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_group_get_type();
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
            b"G_IS_ACTION_GROUP (action_group)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !action_name.is_null() {
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
            b"action_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_signal_emit(
        action_group as gpointer,
        safe_c2rust_g_action_group_signals
            [SIGNAL_ACTION_STATE_CHANGED as ::core::ffi::c_int as usize],
        g_quark_try_string(action_name),
        action_name,
        state,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_group_query_action(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut enabled: *mut gboolean,
    mut parameter_type: *mut *const GVariantType,
    mut state_type: *mut *const GVariantType,
    mut state_hint: *mut *mut GVariant,
    mut state: *mut *mut GVariant,
) -> gboolean {
    return (*(g_type_interface_peek(
        (*(action_group as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_group_get_type(),
    ) as *mut GActionGroupInterface))
        .query_action
        .expect("non-null function pointer")(
        action_group,
        action_name,
        enabled,
        parameter_type,
        state_type,
        state_hint,
        state,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
