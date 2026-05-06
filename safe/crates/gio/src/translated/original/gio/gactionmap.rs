use ::c2rust_bitfields;
extern "C" {
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GActionMap;
    pub type _GSimpleAction;
    pub type _GAction;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_variant_type_string_is_valid(type_string: *const gchar) -> gboolean;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_parse(
        type_0: *const GVariantType,
        text: *const gchar,
        limit: *const gchar,
        endptr: *mut *const gchar,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_object_unref(object: gpointer);
    fn g_simple_action_new(
        name: *const gchar,
        parameter_type: *const GVariantType,
    ) -> *mut GSimpleAction;
    fn g_simple_action_new_stateful(
        name: *const gchar,
        parameter_type: *const GVariantType,
        state: *mut GVariant,
    ) -> *mut GSimpleAction;
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
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
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
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
pub type GConnectFlags = ::core::ffi::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
pub type GActionMap = _GActionMap;
pub type GSimpleAction = _GSimpleAction;
pub type GAction = _GAction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionMapInterface {
    pub g_iface: GTypeInterface,
    pub lookup_action: Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> *mut GAction>,
    pub add_action: Option<unsafe extern "C" fn(*mut GActionMap, *mut GAction) -> ()>,
    pub remove_action: Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> ()>,
}
pub type GActionMapInterface = _GActionMapInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionEntry {
    pub name: *const gchar,
    pub activate: Option<unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, gpointer) -> ()>,
    pub parameter_type: *const gchar,
    pub state: *const gchar,
    pub change_state:
        Option<unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, gpointer) -> ()>,
    pub padding: [gsize; 3],
}
pub type GActionEntry = _GActionEntry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_map_get_type() -> GType {
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
            g_intern_static_string(b"GActionMap\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GActionMapInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GActionMapInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_action_map_default_init
                        as unsafe extern "C" fn(*mut GActionMapInterface) -> (),
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
unsafe extern "C" fn safe_c2rust_g_action_map_default_init(mut iface: *mut GActionMapInterface) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_map_lookup_action(
    mut action_map: *mut GActionMap,
    mut action_name: *const gchar,
) -> *mut GAction {
    return (*(g_type_interface_peek(
        (*(action_map as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_map_get_type(),
    ) as *mut GActionMapInterface))
        .lookup_action
        .expect("non-null function pointer")(action_map, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_map_add_action(
    mut action_map: *mut GActionMap,
    mut action: *mut GAction,
) {
    (*(g_type_interface_peek(
        (*(action_map as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_map_get_type(),
    ) as *mut GActionMapInterface))
        .add_action
        .expect("non-null function pointer")(action_map, action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_map_remove_action(
    mut action_map: *mut GActionMap,
    mut action_name: *const gchar,
) {
    (*(g_type_interface_peek(
        (*(action_map as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_action_map_get_type(),
    ) as *mut GActionMapInterface))
        .remove_action
        .expect("non-null function pointer")(action_map, action_name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_map_add_action_entries(
    mut action_map: *mut GActionMap,
    mut entries: *const GActionEntry,
    mut n_entries: gint,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_map as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_map_get_type();
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
            b"G_IS_ACTION_MAP (action_map)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !entries.is_null() || n_entries == 0 as ::core::ffi::c_int {
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
            b"entries != NULL || n_entries == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_block_34: u64;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while if n_entries < 0 as ::core::ffi::c_int {
        ((*entries.offset(i as isize)).name != NULL as *const gchar) as ::core::ffi::c_int
    } else {
        (i < n_entries) as ::core::ffi::c_int
    } != 0
    {
        let mut entry: *const GActionEntry = entries.offset(i as isize) as *const GActionEntry;
        let mut parameter_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut action: *mut GSimpleAction = ::core::ptr::null_mut::<GSimpleAction>();
        if !(*entry).parameter_type.is_null() {
            if g_variant_type_string_is_valid((*entry).parameter_type) == 0 {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"g_action_map_add_entries: the type string '%s' given as the parameter type for action '%s' is not a valid GVariant type string.  This action will not be added.\0"
                        as *const u8 as *const gchar,
                    (*entry).parameter_type,
                    (*entry).name,
                );
                return;
            }
            parameter_type = g_variant_type_checked_((*entry).parameter_type);
        } else {
            parameter_type = ::core::ptr::null::<GVariantType>();
        }
        if !(*entry).state.is_null() {
            let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
            let mut state: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            state = g_variant_parse(
                ::core::ptr::null::<GVariantType>(),
                (*entry).state,
                ::core::ptr::null::<gchar>(),
                ::core::ptr::null_mut::<*const gchar>(),
                &raw mut error,
            );
            if state.is_null() {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"g_action_map_add_entries: GVariant could not parse the state value given for action '%s' ('%s'): %s.  This action will not be added.\0"
                        as *const u8 as *const gchar,
                    (*entry).name,
                    (*entry).state,
                    (*error).message,
                );
                g_error_free(error);
                current_block_34 = 12209867499936983673;
            } else {
                action = g_simple_action_new_stateful((*entry).name, parameter_type, state);
                g_variant_unref(state);
                current_block_34 = 13550086250199790493;
            }
        } else {
            action = g_simple_action_new((*entry).name, parameter_type);
            current_block_34 = 13550086250199790493;
        }
        match current_block_34 {
            13550086250199790493 => {
                if (*entry).activate.is_some() {
                    g_signal_connect_data(
                        action as gpointer,
                        b"activate\0" as *const u8 as *const gchar,
                        ::core::mem::transmute::<
                            Option<
                                unsafe extern "C" fn(
                                    *mut GSimpleAction,
                                    *mut GVariant,
                                    gpointer,
                                ) -> (),
                            >,
                            GCallback,
                        >((*entry).activate),
                        user_data,
                        None,
                        G_CONNECT_DEFAULT,
                    );
                }
                if (*entry).change_state.is_some() {
                    g_signal_connect_data(
                        action as gpointer,
                        b"change-state\0" as *const u8 as *const gchar,
                        ::core::mem::transmute::<
                            Option<
                                unsafe extern "C" fn(
                                    *mut GSimpleAction,
                                    *mut GVariant,
                                    gpointer,
                                ) -> (),
                            >,
                            GCallback,
                        >((*entry).change_state),
                        user_data,
                        None,
                        G_CONNECT_DEFAULT,
                    );
                }
                safe_c2rust_g_action_map_add_action(
                    action_map,
                    action as *mut ::core::ffi::c_void as *mut GAction,
                );
                g_object_unref(action as gpointer);
            }
            _ => {}
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_action_map_remove_action_entries(
    mut action_map: *mut GActionMap,
    mut entries: *const GActionEntry,
    mut n_entries: gint,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = action_map as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_action_map_get_type();
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
            b"G_IS_ACTION_MAP (action_map)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !entries.is_null() || n_entries == 0 as ::core::ffi::c_int {
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
            b"entries != NULL || n_entries == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while if n_entries < 0 as ::core::ffi::c_int {
        ((*entries.offset(i as isize)).name != NULL as *const gchar) as ::core::ffi::c_int
    } else {
        (i < n_entries) as ::core::ffi::c_int
    } != 0
    {
        safe_c2rust_g_action_map_remove_action(action_map, (*entries.offset(i as isize)).name);
        i += 1;
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
