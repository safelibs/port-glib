// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
    pub type _GValue;
    pub type _GTypeCValue;
    pub type _GTypePlugin;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
    fn g_type_register_static(
        parent_type: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
}
pub type guint16 = ::core::ffi::c_ushort;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GType = gsize;
pub type GValue = _GValue;
pub type GTypeCValue = _GTypeCValue;
pub type GTypePlugin = _GTypePlugin;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInfo {
    pub class_size: guint16,
    pub base_init: GBaseInitFunc,
    pub base_finalize: GBaseFinalizeFunc,
    pub class_init: GClassInitFunc,
    pub class_finalize: GClassFinalizeFunc,
    pub class_data: gconstpointer,
    pub instance_size: guint16,
    pub n_preallocs: guint16,
    pub instance_init: GInstanceInitFunc,
    pub value_table: *const GTypeValueTable,
}
pub type GTypeValueTable = _GTypeValueTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeValueTable {
    pub value_init: GTypeValueInitFunc,
    pub value_free: GTypeValueFreeFunc,
    pub value_copy: GTypeValueCopyFunc,
    pub value_peek_pointer: GTypeValuePeekPointerFunc,
    pub collect_format: *const gchar,
    pub collect_value: GTypeValueCollectFunc,
    pub lcopy_format: *const gchar,
    pub lcopy_value: GTypeValueLCopyFunc,
}
pub type GTypeValueLCopyFunc =
    Option<unsafe extern "C" fn(*const GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValueCollectFunc =
    Option<unsafe extern "C" fn(*mut GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValuePeekPointerFunc = Option<unsafe extern "C" fn(*const GValue) -> gpointer>;
pub type GTypeValueCopyFunc = Option<unsafe extern "C" fn(*const GValue, *mut GValue) -> ()>;
pub type GTypeValueFreeFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GTypeValueInitFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GBaseFinalizeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBaseInitFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GTypeInfo = _GTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInterfaceInfo {
    pub interface_init: GInterfaceInitFunc,
    pub interface_finalize: GInterfaceFinalizeFunc,
    pub interface_data: gpointer,
}
pub type GInterfaceFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInfo = _GInterfaceInfo;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypePluginClass {
    pub base_iface: GTypeInterface,
    pub use_plugin: GTypePluginUse,
    pub unuse_plugin: GTypePluginUnuse,
    pub complete_type_info: GTypePluginCompleteTypeInfo,
    pub complete_interface_info: GTypePluginCompleteInterfaceInfo,
}
pub type GTypePluginCompleteInterfaceInfo =
    Option<unsafe extern "C" fn(*mut GTypePlugin, GType, GType, *mut GInterfaceInfo) -> ()>;
pub type GTypePluginCompleteTypeInfo = Option<
    unsafe extern "C" fn(*mut GTypePlugin, GType, *mut GTypeInfo, *mut GTypeValueTable) -> (),
>;
pub type GTypePluginUnuse = Option<unsafe extern "C" fn(*mut GTypePlugin) -> ()>;
pub type GTypePluginUse = Option<unsafe extern "C" fn(*mut GTypePlugin) -> ()>;
pub type GTypePluginClass = _GTypePluginClass;
#[no_mangle]
pub unsafe extern "C" fn g_type_plugin_get_type() -> GType {
    static mut type_plugin_type: GType = 0 as GType;
    if type_plugin_type == 0 {
        let type_plugin_info: GTypeInfo = _GTypeInfo {
            class_size: ::core::mem::size_of::<GTypePluginClass>() as guint16,
            base_init: None,
            base_finalize: None,
            class_init: None,
            class_finalize: None,
            class_data: ::core::ptr::null::<::core::ffi::c_void>(),
            instance_size: 0 as guint16,
            n_preallocs: 0 as guint16,
            instance_init: None,
            value_table: ::core::ptr::null::<GTypeValueTable>(),
        };
        type_plugin_type = g_type_register_static(
            ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            g_intern_static_string(b"GTypePlugin\0" as *const u8 as *const gchar),
            &raw const type_plugin_info,
            G_TYPE_FLAG_NONE,
        );
    }
    return type_plugin_type;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_plugin_use(mut plugin: *mut GTypePlugin) {
    let mut iface: *mut GTypePluginClass = ::core::ptr::null_mut::<GTypePluginClass>();
    if ({
        let mut __inst: *mut GTypeInstance = plugin as *mut GTypeInstance;
        let mut __t: GType = g_type_plugin_get_type();
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
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_plugin_use\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_TYPE_PLUGIN (plugin)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(plugin as *mut GTypeInstance)).g_class as gpointer,
        g_type_plugin_get_type(),
    ) as *mut GTypePluginClass;
    (*iface).use_plugin.expect("non-null function pointer")(plugin);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_plugin_unuse(mut plugin: *mut GTypePlugin) {
    let mut iface: *mut GTypePluginClass = ::core::ptr::null_mut::<GTypePluginClass>();
    if ({
        let mut __inst: *mut GTypeInstance = plugin as *mut GTypeInstance;
        let mut __t: GType = g_type_plugin_get_type();
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
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_plugin_unuse\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_TYPE_PLUGIN (plugin)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(plugin as *mut GTypeInstance)).g_class as gpointer,
        g_type_plugin_get_type(),
    ) as *mut GTypePluginClass;
    (*iface).unuse_plugin.expect("non-null function pointer")(plugin);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_plugin_complete_type_info(
    mut plugin: *mut GTypePlugin,
    mut g_type: GType,
    mut info: *mut GTypeInfo,
    mut value_table: *mut GTypeValueTable,
) {
    let mut iface: *mut GTypePluginClass = ::core::ptr::null_mut::<GTypePluginClass>();
    if ({
        let mut __inst: *mut GTypeInstance = plugin as *mut GTypeInstance;
        let mut __t: GType = g_type_plugin_get_type();
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
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_plugin_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_TYPE_PLUGIN (plugin)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_plugin_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !value_table.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_plugin_complete_type_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(plugin as *mut GTypeInstance)).g_class as gpointer,
        g_type_plugin_get_type(),
    ) as *mut GTypePluginClass;
    (*iface)
        .complete_type_info
        .expect("non-null function pointer")(plugin, g_type, info, value_table);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_plugin_complete_interface_info(
    mut plugin: *mut GTypePlugin,
    mut instance_type: GType,
    mut interface_type: GType,
    mut info: *mut GInterfaceInfo,
) {
    let mut iface: *mut GTypePluginClass = ::core::ptr::null_mut::<GTypePluginClass>();
    if ({
        let mut __inst: *mut GTypeInstance = plugin as *mut GTypeInstance;
        let mut __t: GType = g_type_plugin_get_type();
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
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_plugin_complete_interface_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_TYPE_PLUGIN (plugin)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_plugin_complete_interface_info\0" as *const u8 as *const ::core::ffi::c_char,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(plugin as *mut GTypeInstance)).g_class as gpointer,
        g_type_plugin_get_type(),
    ) as *mut GTypePluginClass;
    (*iface)
        .complete_interface_info
        .expect("non-null function pointer")(plugin, instance_type, interface_type, info);
}
