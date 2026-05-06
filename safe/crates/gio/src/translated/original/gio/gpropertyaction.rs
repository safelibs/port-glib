use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAction;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handlers_disconnect_matched(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> guint;
    fn g_variant_type_get_gtype() -> GType;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_class_find_property(
        oclass: *mut GObjectClass,
        property_name: *const gchar,
    ) -> *mut GParamSpec;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_set(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_set_property(
        object: *mut GObject,
        property_name: *const gchar,
        value: *const GValue,
    );
    fn g_object_get_property(object: *mut GObject, property_name: *const gchar, value: *mut GValue);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_unref(object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_variant(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        type_0: *const GVariantType,
        default_value: *mut GVariant,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_value_take_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_settings_set_mapping(
        value: *const GValue,
        expected_type: *const GVariantType,
        user_data: gpointer,
    ) -> *mut GVariant;
    fn g_settings_get_mapping(
        value: *mut GValue,
        variant: *mut GVariant,
        user_data: gpointer,
    ) -> gboolean;
    fn g_action_get_type() -> GType;
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GQuark = guint32;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GSignalMatchType = ::core::ffi::c_uint;
pub const G_SIGNAL_MATCH_UNBLOCKED: GSignalMatchType = 32;
pub const G_SIGNAL_MATCH_DATA: GSignalMatchType = 16;
pub const G_SIGNAL_MATCH_FUNC: GSignalMatchType = 8;
pub const G_SIGNAL_MATCH_CLOSURE: GSignalMatchType = 4;
pub const G_SIGNAL_MATCH_DETAIL: GSignalMatchType = 2;
pub const G_SIGNAL_MATCH_ID: GSignalMatchType = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecInt {
    pub parent_instance: GParamSpec,
    pub minimum: gint,
    pub maximum: gint,
    pub default_value: gint,
}
pub type GParamSpecInt = _GParamSpecInt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecUInt {
    pub parent_instance: GParamSpec,
    pub minimum: guint,
    pub maximum: guint,
    pub default_value: guint,
}
pub type GParamSpecUInt = _GParamSpecUInt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecFloat {
    pub parent_instance: GParamSpec,
    pub minimum: gfloat,
    pub maximum: gfloat,
    pub default_value: gfloat,
    pub epsilon: gfloat,
}
pub type GParamSpecFloat = _GParamSpecFloat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecDouble {
    pub parent_instance: GParamSpec,
    pub minimum: gdouble,
    pub maximum: gdouble,
    pub default_value: gdouble,
    pub epsilon: gdouble,
}
pub type GParamSpecDouble = _GParamSpecDouble;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPropertyAction {
    pub parent_instance: GObject,
    pub name: *mut gchar,
    pub object: gpointer,
    pub pspec: *mut GParamSpec,
    pub state_type: *const GVariantType,
    pub invert_boolean: gboolean,
}
pub type GPropertyAction = _GPropertyAction;
pub type GAction = _GAction;
pub type GPropertyActionClass = GObjectClass;
pub const PROP_INVERT_BOOLEAN: C2RustUnnamed_0 = 8;
pub const PROP_PROPERTY_NAME: C2RustUnnamed_0 = 7;
pub const PROP_OBJECT: C2RustUnnamed_0 = 6;
pub const PROP_STATE: C2RustUnnamed_0 = 5;
pub const PROP_STATE_TYPE: C2RustUnnamed_0 = 4;
pub const PROP_ENABLED: C2RustUnnamed_0 = 3;
pub const PROP_PARAMETER_TYPE: C2RustUnnamed_0 = 2;
pub const PROP_NAME: C2RustUnnamed_0 = 1;
pub type GActionInterface = _GActionInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionInterface {
    pub g_iface: GTypeInterface,
    pub get_name: Option<unsafe extern "C" fn(*mut GAction) -> *const gchar>,
    pub get_parameter_type: Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>,
    pub get_state_type: Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>,
    pub get_state_hint: Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>,
    pub get_enabled: Option<unsafe extern "C" fn(*mut GAction) -> gboolean>,
    pub get_state: Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>,
    pub change_state: Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>,
    pub activate: Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_NONE: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_VARIANT_TYPE_BOOLEAN: *const GVariantType =
    b"b\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INT32: *const GVariantType =
    b"i\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_UINT32: *const GVariantType =
    b"u\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_DOUBLE: *const GVariantType =
    b"d\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_ANY: *const GVariantType =
    b"*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT: GType = 24;
pub const G_TYPE_UINT: GType = 28;
pub const G_TYPE_ENUM: GType = ((12 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_FLOAT: GType = 56;
pub const G_TYPE_DOUBLE: GType = 60;
pub const G_TYPE_STRING: GType = 64;
pub const G_TYPE_OBJECT: GType = ((20 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_GPropertyAction_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_property_action_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_property_action_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GPropertyAction_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GPropertyAction_private_offset,
        );
    }
    safe_c2rust_g_property_action_class_init(klass as *mut GPropertyActionClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_property_action_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GPropertyAction\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GPropertyActionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_property_action_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GPropertyAction>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GPropertyAction) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_property_action_init
                    as unsafe extern "C" fn(*mut GPropertyAction) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_property_action_iface_init
                as unsafe extern "C" fn(*mut GActionInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_action_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_property_action_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_property_action_get_type_once();
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
static mut safe_c2rust_g_property_action_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_property_action_get_invert_boolean(
    mut action: *mut GAction,
) -> gboolean {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    return (*paction).invert_boolean;
}
unsafe extern "C" fn safe_c2rust_g_property_action_get_name(
    mut action: *mut GAction,
) -> *const gchar {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    return (*paction).name;
}
unsafe extern "C" fn safe_c2rust_g_property_action_get_parameter_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    return if (*(*paction).pspec).value_type == G_TYPE_BOOLEAN {
        ::core::ptr::null::<GVariantType>()
    } else {
        (*paction).state_type
    };
}
unsafe extern "C" fn safe_c2rust_g_property_action_get_state_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    return (*paction).state_type;
}
unsafe extern "C" fn safe_c2rust_g_property_action_get_state_hint(
    mut action: *mut GAction,
) -> *mut GVariant {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    if (*(*paction).pspec).value_type == G_TYPE_INT {
        let mut pspec: *mut GParamSpecInt = (*paction).pspec as *mut GParamSpecInt;
        return g_variant_new(
            b"(ii)\0" as *const u8 as *const gchar,
            (*pspec).minimum,
            (*pspec).maximum,
        );
    } else if (*(*paction).pspec).value_type == G_TYPE_UINT {
        let mut pspec_0: *mut GParamSpecUInt = (*paction).pspec as *mut GParamSpecUInt;
        return g_variant_new(
            b"(uu)\0" as *const u8 as *const gchar,
            (*pspec_0).minimum,
            (*pspec_0).maximum,
        );
    } else if (*(*paction).pspec).value_type == G_TYPE_FLOAT {
        let mut pspec_1: *mut GParamSpecFloat = (*paction).pspec as *mut GParamSpecFloat;
        return g_variant_new(
            b"(dd)\0" as *const u8 as *const gchar,
            (*pspec_1).minimum as ::core::ffi::c_double,
            (*pspec_1).maximum as ::core::ffi::c_double,
        );
    } else if (*(*paction).pspec).value_type == G_TYPE_DOUBLE {
        let mut pspec_2: *mut GParamSpecDouble = (*paction).pspec as *mut GParamSpecDouble;
        return g_variant_new(
            b"(dd)\0" as *const u8 as *const gchar,
            (*pspec_2).minimum,
            (*pspec_2).maximum,
        );
    }
    return ::core::ptr::null_mut::<GVariant>();
}
unsafe extern "C" fn safe_c2rust_g_property_action_get_enabled(
    mut action: *mut GAction,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_property_action_set_state(
    mut paction: *mut GPropertyAction,
    mut variant: *mut GVariant,
) {
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    g_value_init(&raw mut value, (*(*paction).pspec).value_type);
    g_settings_get_mapping(&raw mut value, variant, NULL);
    if (*(*paction).pspec).value_type == G_TYPE_BOOLEAN && (*paction).invert_boolean != 0 {
        g_value_set_boolean(
            &raw mut value,
            (g_value_get_boolean(&raw mut value) == 0) as ::core::ffi::c_int,
        );
    }
    g_object_set_property(
        (*paction).object as *mut GObject,
        (*(*paction).pspec).name,
        &raw mut value,
    );
    g_value_unset(&raw mut value);
}
unsafe extern "C" fn safe_c2rust_g_property_action_change_state(
    mut action: *mut GAction,
    mut value: *mut GVariant,
) {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_variant_is_of_type(value, (*paction).state_type) != 0 {
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
            b"g_variant_is_of_type (value, paction->state_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_property_action_set_state(paction, value);
}
unsafe extern "C" fn safe_c2rust_g_property_action_get_state(
    mut action: *mut GAction,
) -> *mut GVariant {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    g_value_init(&raw mut value, (*(*paction).pspec).value_type);
    g_object_get_property(
        (*paction).object as *mut GObject,
        (*(*paction).pspec).name,
        &raw mut value,
    );
    if (*(*paction).pspec).value_type == G_TYPE_BOOLEAN && (*paction).invert_boolean != 0 {
        g_value_set_boolean(
            &raw mut value,
            (g_value_get_boolean(&raw mut value) == 0) as ::core::ffi::c_int,
        );
    }
    result = g_settings_set_mapping(&raw mut value, (*paction).state_type, NULL);
    g_value_unset(&raw mut value);
    return g_variant_ref_sink(result);
}
unsafe extern "C" fn safe_c2rust_g_property_action_activate(
    mut action: *mut GAction,
    mut parameter: *mut GVariant,
) {
    let mut paction: *mut GPropertyAction =
        action as *mut ::core::ffi::c_void as *mut GPropertyAction;
    if (*(*paction).pspec).value_type == G_TYPE_BOOLEAN {
        let mut value: gboolean = 0;
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if (*(*paction).pspec).value_type
                == ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && parameter.is_null()
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
                b"paction->pspec->value_type == G_TYPE_BOOLEAN && parameter == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        g_object_get(
            (*paction).object,
            (*(*paction).pspec).name,
            &raw mut value,
            NULL,
        );
        value = (value == 0) as ::core::ffi::c_int as gboolean;
        g_object_set((*paction).object, (*(*paction).pspec).name, value, NULL);
    } else {
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if !parameter.is_null() && g_variant_is_of_type(parameter, (*paction).state_type) != 0 {
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
                b"parameter != NULL && g_variant_is_of_type (parameter, paction->state_type)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_c2rust_g_property_action_set_state(paction, parameter);
    };
}
unsafe extern "C" fn safe_c2rust_g_property_action_determine_type(
    mut pspec: *mut GParamSpec,
) -> *const GVariantType {
    if g_type_fundamental((*pspec).value_type) == G_TYPE_ENUM {
        return G_VARIANT_TYPE_STRING;
    }
    match (*pspec).value_type {
        G_TYPE_BOOLEAN => return G_VARIANT_TYPE_BOOLEAN,
        G_TYPE_INT => return G_VARIANT_TYPE_INT32,
        G_TYPE_UINT => return G_VARIANT_TYPE_UINT32,
        G_TYPE_DOUBLE | G_TYPE_FLOAT => return G_VARIANT_TYPE_DOUBLE,
        G_TYPE_STRING => return G_VARIANT_TYPE_STRING,
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"Unable to use GPropertyAction with property '%s::%s' of type '%s'\0" as *const u8
                    as *const gchar,
                g_type_name((*pspec).owner_type),
                (*pspec).name,
                g_type_name((*pspec).value_type),
            );
            return ::core::ptr::null::<GVariantType>();
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_property_action_notify(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut user_data: gpointer,
) {
    let mut paction: *mut GPropertyAction = user_data as *mut GPropertyAction;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if object == (*paction).object as *mut GObject {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gpropertyaction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            293 as ::core::ffi::c_int,
            G_STRFUNC,
            b"object == paction->object\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if pspec == (*paction).pspec {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gpropertyaction.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            294 as ::core::ffi::c_int,
            G_STRFUNC,
            b"pspec == paction->pspec\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_object_notify(
        paction as *mut ::core::ffi::c_void as *mut GObject,
        b"state\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_property_action_set_property_name(
    mut paction: *mut GPropertyAction,
    mut property_name: *const gchar,
) {
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut detailed: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if property_name.is_null()
            || *property_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
        {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Attempted to use an empty property name for GPropertyAction\0" as *const u8
                as *const gchar,
        );
        return;
    }
    pspec = g_object_class_find_property(
        (*((*paction).object as *mut GTypeInstance)).g_class as *mut GObjectClass,
        property_name,
    );
    if pspec.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Attempted to use non-existent property '%s::%s' for GPropertyAction\0" as *const u8
                as *const gchar,
            g_type_name((*(*((*paction).object as *mut GTypeInstance)).g_class).g_type),
            property_name,
        );
        return;
    }
    if !((*pspec).flags as ::core::ffi::c_int) & G_PARAM_READABLE as ::core::ffi::c_int != 0
        || !((*pspec).flags as ::core::ffi::c_int) & G_PARAM_WRITABLE as ::core::ffi::c_int != 0
        || (*pspec).flags as ::core::ffi::c_int & G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Property '%s::%s' used with GPropertyAction must be readable, writable, and not construct-only\0"
                as *const u8 as *const gchar,
            g_type_name((*(*((*paction).object as *mut GTypeInstance)).g_class).g_type),
            property_name,
        );
        return;
    }
    (*paction).pspec = pspec;
    detailed = g_strconcat(
        b"notify::\0" as *const u8 as *const gchar,
        (*(*paction).pspec).name,
        NULL,
    );
    (*paction).state_type = safe_c2rust_g_property_action_determine_type((*paction).pspec);
    g_signal_connect_data(
        (*paction).object,
        detailed,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_property_action_notify
                as unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> (),
        )),
        paction as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_free(detailed as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_property_action_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut paction: *mut GPropertyAction =
        object as *mut ::core::ffi::c_void as *mut GPropertyAction;
    match prop_id {
        1 => {
            (*paction).name = g_value_dup_string(value);
        }
        6 => {
            (*paction).object = g_value_dup_object(value);
        }
        7 => {
            safe_c2rust_g_property_action_set_property_name(paction, g_value_get_string(value));
        }
        8 => {
            (*paction).invert_boolean = g_value_get_boolean(value);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gpropertyaction.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                366 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_property_action_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut action: *mut GAction = object as *mut ::core::ffi::c_void as *mut GAction;
    match prop_id {
        1 => {
            g_value_set_string(value, safe_c2rust_g_property_action_get_name(action));
        }
        2 => {
            g_value_set_boxed(
                value,
                safe_c2rust_g_property_action_get_parameter_type(action) as gconstpointer,
            );
        }
        3 => {
            g_value_set_boolean(value, safe_c2rust_g_property_action_get_enabled(action));
        }
        4 => {
            g_value_set_boxed(
                value,
                safe_c2rust_g_property_action_get_state_type(action) as gconstpointer,
            );
        }
        5 => {
            g_value_take_variant(value, safe_c2rust_g_property_action_get_state(action));
        }
        8 => {
            g_value_set_boolean(
                value,
                safe_c2rust_g_property_action_get_invert_boolean(action),
            );
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gpropertyaction.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                405 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_property_action_dispose(mut object: *mut GObject) {
    let mut paction: *mut GPropertyAction =
        object as *mut ::core::ffi::c_void as *mut GPropertyAction;
    if !(*paction).object.is_null() {
        g_signal_handlers_disconnect_matched(
            (*paction).object,
            (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
                as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ::core::ptr::null_mut::<GClosure>(),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> ()>,
                gpointer,
            >(Some(
                safe_c2rust_g_property_action_notify
                    as unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> (),
            )),
            paction as gpointer,
        );
        let mut _pp: *mut gpointer = &raw mut (*paction).object;
        let mut _ptr: gpointer = *_pp;
        *_pp = NULL as gpointer;
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    }
    (*(safe_c2rust_g_property_action_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_property_action_finalize(mut object: *mut GObject) {
    let mut paction: *mut GPropertyAction =
        object as *mut ::core::ffi::c_void as *mut GPropertyAction;
    g_free((*paction).name as gpointer);
    (*(safe_c2rust_g_property_action_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_property_action_init(mut property: *mut GPropertyAction) {}
unsafe extern "C" fn safe_c2rust_g_property_action_iface_init(mut iface: *mut GActionInterface) {
    (*iface).get_name = Some(
        safe_c2rust_g_property_action_get_name
            as unsafe extern "C" fn(*mut GAction) -> *const gchar,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *const gchar>;
    (*iface).get_parameter_type = Some(
        safe_c2rust_g_property_action_get_parameter_type
            as unsafe extern "C" fn(*mut GAction) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>;
    (*iface).get_state_type = Some(
        safe_c2rust_g_property_action_get_state_type
            as unsafe extern "C" fn(*mut GAction) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>;
    (*iface).get_state_hint = Some(
        safe_c2rust_g_property_action_get_state_hint
            as unsafe extern "C" fn(*mut GAction) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>;
    (*iface).get_enabled = Some(
        safe_c2rust_g_property_action_get_enabled as unsafe extern "C" fn(*mut GAction) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> gboolean>;
    (*iface).get_state = Some(
        safe_c2rust_g_property_action_get_state
            as unsafe extern "C" fn(*mut GAction) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>;
    (*iface).change_state = Some(
        safe_c2rust_g_property_action_change_state
            as unsafe extern "C" fn(*mut GAction, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>;
    (*iface).activate = Some(
        safe_c2rust_g_property_action_activate
            as unsafe extern "C" fn(*mut GAction, *mut GVariant) -> (),
    ) as Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_property_action_class_init(
    mut class: *mut GPropertyActionClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).set_property = Some(
        safe_c2rust_g_property_action_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_property_action_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).dispose =
        Some(safe_c2rust_g_property_action_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_property_action_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        object_class,
        PROP_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PARAMETER_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"parameter-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_variant_type_get_gtype(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_ENABLED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"enabled\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            TRUE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_STATE_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"state-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_variant_type_get_gtype(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_STATE as ::core::ffi::c_int as guint,
        g_param_spec_variant(
            b"state\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_VARIANT_TYPE_ANY,
            ::core::ptr::null_mut::<GVariant>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_OBJECT as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"object\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_TYPE_OBJECT,
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PROPERTY_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"property-name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_INVERT_BOOLEAN as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"invert-boolean\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_property_action_new(
    mut name: *const gchar,
    mut object: gpointer,
    mut property_name: *const gchar,
) -> *mut GPropertyAction {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPropertyAction>();
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if g_type_check_instance_is_fundamentally_a(
            object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
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
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPropertyAction>();
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !property_name.is_null() {
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
            b"property_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPropertyAction>();
    }
    return g_object_new(
        safe_c2rust_g_property_action_get_type(),
        b"name\0" as *const u8 as *const gchar,
        name,
        b"object\0" as *const u8 as *const ::core::ffi::c_char,
        object,
        b"property-name\0" as *const u8 as *const ::core::ffi::c_char,
        property_name,
        NULL,
    ) as *mut GPropertyAction;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
