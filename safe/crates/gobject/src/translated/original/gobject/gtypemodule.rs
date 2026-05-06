// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
    pub type _GData;
    pub type _GTypeCValue;
    pub type _GTypePlugin;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_from_name(name: *const gchar) -> GType;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_register_static(
        parent_type: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_register_dynamic(
        parent_type: GType,
        type_name: *const gchar,
        plugin: *mut GTypePlugin,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_add_interface_dynamic(
        instance_type: GType,
        interface_type: GType,
        plugin: *mut GTypePlugin,
    );
    fn g_type_get_plugin(type_0: GType) -> *mut GTypePlugin;
    fn g_type_interface_get_plugin(instance_type: GType, interface_type: GType)
        -> *mut GTypePlugin;
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_type_plugin_get_type() -> GType;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_enum_complete_type_info(
        g_enum_type: GType,
        info: *mut GTypeInfo,
        const_values: *const GEnumValue,
    );
    fn g_flags_complete_type_info(
        g_flags_type: GType,
        info: *mut GTypeInfo,
        const_values: *const GFlagsValue,
    );
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeModule {
    pub parent_instance: GObject,
    pub use_count: guint,
    pub type_infos: *mut GSList,
    pub interface_infos: *mut GSList,
    pub name: *mut gchar,
}
pub type GTypeModule = _GTypeModule;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeModuleClass {
    pub parent_class: GObjectClass,
    pub load: Option<unsafe extern "C" fn(*mut GTypeModule) -> gboolean>,
    pub unload: Option<unsafe extern "C" fn(*mut GTypeModule) -> ()>,
    pub reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub reserved4: Option<unsafe extern "C" fn() -> ()>,
}
pub type GTypeModuleClass = _GTypeModuleClass;
pub type ModuleInterfaceInfo = _ModuleInterfaceInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ModuleInterfaceInfo {
    pub loaded: gboolean,
    pub instance_type: GType,
    pub interface_type: GType,
    pub info: GInterfaceInfo,
}
pub type ModuleTypeInfo = _ModuleTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ModuleTypeInfo {
    pub loaded: gboolean,
    pub type_0: GType,
    pub parent_type: GType,
    pub info: GTypeInfo,
}
#[inline(always)]
unsafe extern "C" fn g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
}
static mut parent_class: gpointer =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
unsafe extern "C" fn g_type_module_dispose(mut object: *mut GObject) {
    let mut module: *mut GTypeModule = object as *mut ::core::ffi::c_void as *mut GTypeModule;
    if !(*module).type_infos.is_null() || !(*module).interface_infos.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtypemodule.c:103: unsolicitated invocation of g_object_run_dispose() on GTypeModule\0"
                as *const u8 as *const gchar,
        );
        g_object_ref(object as gpointer);
    }
    (*(parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn g_type_module_finalize(mut object: *mut GObject) {
    let mut module: *mut GTypeModule = object as *mut ::core::ffi::c_void as *mut GTypeModule;
    g_free((*module).name as gpointer);
    if (*module).type_infos.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtypemodule.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            119 as ::core::ffi::c_int,
            b"g_type_module_finalize\0" as *const u8 as *const ::core::ffi::c_char,
            b"module->type_infos == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*module).interface_infos.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtypemodule.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            120 as ::core::ffi::c_int,
            b"g_type_module_finalize\0" as *const u8 as *const ::core::ffi::c_char,
            b"module->interface_infos == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn g_type_module_class_init(mut class: *mut GTypeModuleClass) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    parent_class = g_type_class_peek_parent(class as gpointer) as *mut GObjectClass as gpointer;
    (*gobject_class).dispose =
        Some(g_type_module_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).finalize =
        Some(g_type_module_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn g_type_module_iface_init(mut iface: *mut GTypePluginClass) {
    (*iface).use_plugin =
        Some(g_type_module_use_plugin as unsafe extern "C" fn(*mut GTypePlugin) -> ())
            as GTypePluginUse;
    (*iface).unuse_plugin = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut GTypeModule) -> ()>,
        Option<unsafe extern "C" fn(*mut GTypePlugin) -> ()>,
    >(Some(
        g_type_module_unuse as unsafe extern "C" fn(*mut GTypeModule) -> (),
    )) as GTypePluginUnuse;
    (*iface).complete_type_info = Some(
        g_type_module_complete_type_info
            as unsafe extern "C" fn(
                *mut GTypePlugin,
                GType,
                *mut GTypeInfo,
                *mut GTypeValueTable,
            ) -> (),
    ) as GTypePluginCompleteTypeInfo;
    (*iface).complete_interface_info = Some(
        g_type_module_complete_interface_info
            as unsafe extern "C" fn(*mut GTypePlugin, GType, GType, *mut GInterfaceInfo) -> (),
    ) as GTypePluginCompleteInterfaceInfo;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_get_type() -> GType {
    static mut type_module_type: GType = 0 as GType;
    if type_module_type == 0 {
        let type_module_info: GTypeInfo = _GTypeInfo {
            class_size: ::core::mem::size_of::<GTypeModuleClass>() as guint16,
            base_init: None,
            base_finalize: None,
            class_init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTypeModuleClass) -> ()>,
                GClassInitFunc,
            >(Some(
                g_type_module_class_init as unsafe extern "C" fn(*mut GTypeModuleClass) -> (),
            )),
            class_finalize: None,
            class_data: ::core::ptr::null::<::core::ffi::c_void>(),
            instance_size: ::core::mem::size_of::<GTypeModule>() as guint16,
            n_preallocs: 0 as guint16,
            instance_init: None,
            value_table: ::core::ptr::null::<GTypeValueTable>(),
        };
        let iface_info: GInterfaceInfo = _GInterfaceInfo {
            interface_init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTypePluginClass) -> ()>,
                GInterfaceInitFunc,
            >(Some(
                g_type_module_iface_init as unsafe extern "C" fn(*mut GTypePluginClass) -> (),
            )),
            interface_finalize: None,
            interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        type_module_type = g_type_register_static(
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            g_intern_static_string(b"GTypeModule\0" as *const u8 as *const gchar),
            &raw const type_module_info,
            G_TYPE_FLAG_ABSTRACT,
        );
        g_type_add_interface_static(
            type_module_type,
            g_type_plugin_get_type(),
            &raw const iface_info,
        );
    }
    return type_module_type;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_set_name(
    mut module: *mut GTypeModule,
    mut name: *const gchar,
) {
    if ({
        let mut __inst: *mut GTypeInstance = module as *mut GTypeInstance;
        let mut __t: GType = g_type_module_get_type();
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
            b"g_type_module_set_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_TYPE_MODULE (module)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*module).name as gpointer);
    (*module).name = g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
}
unsafe extern "C" fn g_type_module_find_type_info(
    mut module: *mut GTypeModule,
    mut type_0: GType,
) -> *mut ModuleTypeInfo {
    let mut tmp_list: *mut GSList = (*module).type_infos;
    while !tmp_list.is_null() {
        let mut type_info: *mut ModuleTypeInfo = (*tmp_list).data as *mut ModuleTypeInfo;
        if (*type_info).type_0 == type_0 {
            return type_info;
        }
        tmp_list = (*tmp_list).next;
    }
    return ::core::ptr::null_mut::<ModuleTypeInfo>();
}
unsafe extern "C" fn g_type_module_find_interface_info(
    mut module: *mut GTypeModule,
    mut instance_type: GType,
    mut interface_type: GType,
) -> *mut ModuleInterfaceInfo {
    let mut tmp_list: *mut GSList = (*module).interface_infos;
    while !tmp_list.is_null() {
        let mut interface_info: *mut ModuleInterfaceInfo =
            (*tmp_list).data as *mut ModuleInterfaceInfo;
        if (*interface_info).instance_type == instance_type
            && (*interface_info).interface_type == interface_type
        {
            return interface_info;
        }
        tmp_list = (*tmp_list).next;
    }
    return ::core::ptr::null_mut::<ModuleInterfaceInfo>();
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_use(mut module: *mut GTypeModule) -> gboolean {
    if ({
        let mut __inst: *mut GTypeInstance = module as *mut GTypeInstance;
        let mut __t: GType = g_type_module_get_type();
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
            b"g_type_module_use\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_TYPE_MODULE (module)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    (*module).use_count = (*module).use_count.wrapping_add(1);
    if (*module).use_count == 1 as guint {
        let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
        if (*((*(module as *mut GTypeInstance)).g_class as *mut GTypeModuleClass))
            .load
            .expect("non-null function pointer")(module)
            == 0
        {
            (*module).use_count = (*module).use_count.wrapping_sub(1);
            return 0 as gboolean;
        }
        tmp_list = (*module).type_infos;
        while !tmp_list.is_null() {
            let mut type_info: *mut ModuleTypeInfo = (*tmp_list).data as *mut ModuleTypeInfo;
            if (*type_info).loaded == 0 {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"plugin '%s' failed to register type '%s'\0" as *const u8 as *const gchar,
                    if !(*module).name.is_null() {
                        (*module).name as *const gchar
                    } else {
                        b"(unknown)\0" as *const u8 as *const gchar
                    },
                    g_type_name((*type_info).type_0),
                );
                (*module).use_count = (*module).use_count.wrapping_sub(1);
                return 0 as gboolean;
            }
            tmp_list = (*tmp_list).next;
        }
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_unuse(mut module: *mut GTypeModule) {
    if ({
        let mut __inst: *mut GTypeInstance = module as *mut GTypeInstance;
        let mut __t: GType = g_type_module_get_type();
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
            b"g_type_module_unuse\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_TYPE_MODULE (module)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*module).use_count > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_unuse\0" as *const u8 as *const ::core::ffi::c_char,
            b"module->use_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*module).use_count = (*module).use_count.wrapping_sub(1);
    if (*module).use_count == 0 as guint {
        let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
        (*((*(module as *mut GTypeInstance)).g_class as *mut GTypeModuleClass))
            .unload
            .expect("non-null function pointer")(module);
        tmp_list = (*module).type_infos;
        while !tmp_list.is_null() {
            let mut type_info: *mut ModuleTypeInfo = (*tmp_list).data as *mut ModuleTypeInfo;
            (*type_info).loaded = 0 as ::core::ffi::c_int as gboolean;
            tmp_list = (*tmp_list).next;
        }
    }
}
unsafe extern "C" fn g_type_module_use_plugin(mut plugin: *mut GTypePlugin) {
    let mut module: *mut GTypeModule = plugin as *mut ::core::ffi::c_void as *mut GTypeModule;
    if g_type_module_use(module) == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Fatal error - Could not reload previously loaded plugin '%s'\0" as *const u8
                as *const gchar,
            if !(*module).name.is_null() {
                (*module).name as *const gchar
            } else {
                b"(unknown)\0" as *const u8 as *const gchar
            },
        );
        loop {}
    }
}
unsafe extern "C" fn g_type_module_complete_type_info(
    mut plugin: *mut GTypePlugin,
    mut g_type: GType,
    mut info: *mut GTypeInfo,
    mut value_table: *mut GTypeValueTable,
) {
    let mut module: *mut GTypeModule = plugin as *mut ::core::ffi::c_void as *mut GTypeModule;
    let mut module_type_info: *mut ModuleTypeInfo = g_type_module_find_type_info(module, g_type);
    *info = (*module_type_info).info;
    if !(*module_type_info).info.value_table.is_null() {
        *value_table = *(*module_type_info).info.value_table;
    }
}
unsafe extern "C" fn g_type_module_complete_interface_info(
    mut plugin: *mut GTypePlugin,
    mut instance_type: GType,
    mut interface_type: GType,
    mut info: *mut GInterfaceInfo,
) {
    let mut module: *mut GTypeModule = plugin as *mut ::core::ffi::c_void as *mut GTypeModule;
    let mut module_interface_info: *mut ModuleInterfaceInfo =
        g_type_module_find_interface_info(module, instance_type, interface_type);
    *info = (*module_interface_info).info;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_register_type(
    mut module: *mut GTypeModule,
    mut parent_type: GType,
    mut type_name: *const gchar,
    mut type_info: *const GTypeInfo,
    mut flags: GTypeFlags,
) -> GType {
    let mut module_type_info: *mut ModuleTypeInfo = ::core::ptr::null_mut::<ModuleTypeInfo>();
    let mut type_0: GType = 0;
    if !type_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_register_type\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !type_info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_register_type\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if module.is_null() {
        return g_type_register_static_simple(
            parent_type,
            type_name,
            (*type_info).class_size as guint,
            (*type_info).class_init,
            (*type_info).instance_size as guint,
            (*type_info).instance_init,
            flags,
        );
    }
    type_0 = g_type_from_name(type_name);
    if type_0 != 0 {
        let mut old_plugin: *mut GTypePlugin = g_type_get_plugin(type_0);
        if old_plugin != module as *mut ::core::ffi::c_void as *mut GTypePlugin {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"Two different plugins tried to register '%s'.\0" as *const u8 as *const gchar,
                type_name,
            );
            return 0 as GType;
        }
    }
    if type_0 != 0 {
        module_type_info = g_type_module_find_type_info(module, type_0);
        if (*module_type_info).parent_type != parent_type {
            let mut parent_type_name: *const gchar = g_type_name(parent_type);
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"Type '%s' recreated with different parent type.(was '%s', now '%s')\0"
                    as *const u8 as *const gchar,
                type_name,
                g_type_name((*module_type_info).parent_type),
                if !parent_type_name.is_null() {
                    parent_type_name as *const ::core::ffi::c_char
                } else {
                    b"(unknown)\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            return 0 as GType;
        }
        if !(*module_type_info).info.value_table.is_null() {
            g_free((*module_type_info).info.value_table as *mut GTypeValueTable as gpointer);
        }
    } else {
        module_type_info = g_malloc_n(
            1 as gsize,
            ::core::mem::size_of::<ModuleTypeInfo>() as gsize,
        ) as *mut ModuleTypeInfo;
        (*module_type_info).parent_type = parent_type;
        (*module_type_info).type_0 = g_type_register_dynamic(
            parent_type,
            type_name,
            module as *mut ::core::ffi::c_void as *mut GTypePlugin,
            flags,
        );
        (*module).type_infos = g_slist_prepend((*module).type_infos, module_type_info as gpointer);
    }
    (*module_type_info).loaded = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    (*module_type_info).info = *type_info;
    if !(*type_info).value_table.is_null() {
        (*module_type_info).info.value_table = g_memdup2(
            (*type_info).value_table as gconstpointer,
            ::core::mem::size_of::<GTypeValueTable>() as gsize,
        ) as *const GTypeValueTable;
    }
    return (*module_type_info).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_add_interface(
    mut module: *mut GTypeModule,
    mut instance_type: GType,
    mut interface_type: GType,
    mut interface_info: *const GInterfaceInfo,
) {
    let mut module_interface_info: *mut ModuleInterfaceInfo =
        ::core::ptr::null_mut::<ModuleInterfaceInfo>();
    if !interface_info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_add_interface\0" as *const u8 as *const ::core::ffi::c_char,
            b"interface_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if module.is_null() {
        g_type_add_interface_static(instance_type, interface_type, interface_info);
        return;
    }
    if instance_type == interface_type || g_type_is_a(instance_type, interface_type) != 0 {
        let mut old_plugin: *mut GTypePlugin =
            g_type_interface_get_plugin(instance_type, interface_type);
        if old_plugin.is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"Interface '%s' for '%s' was previously registered statically or for a parent type.\0"
                    as *const u8 as *const gchar,
                g_type_name(interface_type),
                g_type_name(instance_type),
            );
            return;
        } else if old_plugin != module as *mut ::core::ffi::c_void as *mut GTypePlugin {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"Two different plugins tried to register interface '%s' for '%s'.\0" as *const u8
                    as *const gchar,
                g_type_name(interface_type),
                g_type_name(instance_type),
            );
            return;
        }
        module_interface_info =
            g_type_module_find_interface_info(module, instance_type, interface_type);
        if !module_interface_info.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtypemodule.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                506 as ::core::ffi::c_int,
                b"g_type_module_add_interface\0" as *const u8 as *const ::core::ffi::c_char,
                b"module_interface_info\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        module_interface_info = g_malloc_n(
            1 as gsize,
            ::core::mem::size_of::<ModuleInterfaceInfo>() as gsize,
        ) as *mut ModuleInterfaceInfo;
        (*module_interface_info).instance_type = instance_type;
        (*module_interface_info).interface_type = interface_type;
        g_type_add_interface_dynamic(
            instance_type,
            interface_type,
            module as *mut ::core::ffi::c_void as *mut GTypePlugin,
        );
        (*module).interface_infos =
            g_slist_prepend((*module).interface_infos, module_interface_info as gpointer);
    }
    (*module_interface_info).loaded =
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    (*module_interface_info).info = *interface_info;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_register_enum(
    mut module: *mut GTypeModule,
    mut name: *const gchar,
    mut const_static_values: *const GEnumValue,
) -> GType {
    let mut enum_type_info: GTypeInfo = _GTypeInfo {
        class_size: 0 as guint16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0,
        n_preallocs: 0,
        instance_init: None,
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    if module.is_null()
        || ({
            let mut __inst: *mut GTypeInstance = module as *mut GTypeInstance;
            let mut __t: GType = g_type_module_get_type();
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
            b"g_type_module_register_enum\0" as *const u8 as *const ::core::ffi::c_char,
            b"module == NULL || G_IS_TYPE_MODULE (module)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_register_enum\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !const_static_values.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_register_enum\0" as *const u8 as *const ::core::ffi::c_char,
            b"const_static_values != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    g_enum_complete_type_info(
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        &raw mut enum_type_info,
        const_static_values,
    );
    return g_type_module_register_type(
        module as *mut ::core::ffi::c_void as *mut GTypeModule,
        ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        name,
        &raw mut enum_type_info,
        G_TYPE_FLAG_NONE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_type_module_register_flags(
    mut module: *mut GTypeModule,
    mut name: *const gchar,
    mut const_static_values: *const GFlagsValue,
) -> GType {
    let mut flags_type_info: GTypeInfo = _GTypeInfo {
        class_size: 0 as guint16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0,
        n_preallocs: 0,
        instance_init: None,
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    if module.is_null()
        || ({
            let mut __inst: *mut GTypeInstance = module as *mut GTypeInstance;
            let mut __t: GType = g_type_module_get_type();
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
            b"g_type_module_register_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"module == NULL || G_IS_TYPE_MODULE (module)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_register_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !const_static_values.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_module_register_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"const_static_values != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    g_flags_complete_type_info(
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        &raw mut flags_type_info,
        const_static_values,
    );
    return g_type_module_register_type(
        module as *mut ::core::ffi::c_void as *mut GTypeModule,
        ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        name,
        &raw mut flags_type_info,
        G_TYPE_FLAG_NONE,
    );
}
