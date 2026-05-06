extern "C" {
    pub type _GVariant;
    pub type _GRemoteActionGroup;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn g_action_group_get_type() -> GType;
}
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GVariant = _GVariant;
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
pub type GRemoteActionGroup = _GRemoteActionGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRemoteActionGroupInterface {
    pub g_iface: GTypeInterface,
    pub activate_action_full: Option<
        unsafe extern "C" fn(
            *mut GRemoteActionGroup,
            *const gchar,
            *mut GVariant,
            *mut GVariant,
        ) -> (),
    >,
    pub change_action_state_full: Option<
        unsafe extern "C" fn(
            *mut GRemoteActionGroup,
            *const gchar,
            *mut GVariant,
            *mut GVariant,
        ) -> (),
    >,
}
pub type GRemoteActionGroupInterface = _GRemoteActionGroupInterface;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_remote_action_group_get_type() -> GType {
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
            g_intern_static_string(b"GRemoteActionGroup\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GRemoteActionGroupInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GRemoteActionGroupInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_remote_action_group_default_init
                        as unsafe extern "C" fn(*mut GRemoteActionGroupInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if g_action_group_get_type() != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(g_define_type_id, g_action_group_get_type());
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
unsafe extern "C" fn safe_c2rust_g_remote_action_group_default_init(
    mut iface: *mut GRemoteActionGroupInterface,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_remote_action_group_activate_action_full(
    mut remote: *mut GRemoteActionGroup,
    mut action_name: *const gchar,
    mut parameter: *mut GVariant,
    mut platform_data: *mut GVariant,
) {
    (*(g_type_interface_peek(
        (*(remote as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_remote_action_group_get_type(),
    ) as *mut GRemoteActionGroupInterface))
        .activate_action_full
        .expect("non-null function pointer")(remote, action_name, parameter, platform_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_remote_action_group_change_action_state_full(
    mut remote: *mut GRemoteActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
    mut platform_data: *mut GVariant,
) {
    (*(g_type_interface_peek(
        (*(remote as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_remote_action_group_get_type(),
    ) as *mut GRemoteActionGroupInterface))
        .change_action_state_full
        .expect("non-null function pointer")(remote, action_name, value, platform_data);
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
