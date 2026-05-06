use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAction;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_get_boolean(value: *mut GVariant) -> gboolean;
    fn g_variant_equal(one: gconstpointer, two: gconstpointer) -> gboolean;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
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
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_has_handler_pending(
        instance: gpointer,
        signal_id: guint,
        detail: GQuark,
        may_be_blocked: gboolean,
    ) -> gboolean;
    fn g_variant_type_get_gtype() -> GType;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_dup_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
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
    fn g_value_take_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_value_get_variant(value: *const GValue) -> *mut GVariant;
    fn g_value_dup_variant(value: *const GValue) -> *mut GVariant;
    fn g_action_get_type() -> GType;
}
pub type size_t = usize;
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
pub type GSignalAccumulator = Option<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
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
pub struct _GSimpleAction {
    pub parent_instance: GObject,
    pub name: *mut gchar,
    pub parameter_type: *mut GVariantType,
    pub enabled: gboolean,
    pub state: *mut GVariant,
    pub state_hint: *mut GVariant,
    pub state_set_already: gboolean,
}
pub type GSimpleAction = _GSimpleAction;
pub type GAction = _GAction;
pub type GSimpleActionClass = GObjectClass;
pub const PROP_STATE: C2RustUnnamed_0 = 5;
pub const PROP_STATE_TYPE: C2RustUnnamed_0 = 4;
pub const PROP_ENABLED: C2RustUnnamed_0 = 3;
pub const PROP_PARAMETER_TYPE: C2RustUnnamed_0 = 2;
pub const PROP_NAME: C2RustUnnamed_0 = 1;
pub const SIGNAL_CHANGE_STATE: C2RustUnnamed_1 = 0;
pub const SIGNAL_ACTIVATE: C2RustUnnamed_1 = 1;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const NR_SIGNALS: C2RustUnnamed_1 = 2;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
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
pub const G_VARIANT_TYPE_BOOLEAN: *const GVariantType =
    b"b\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_ANY: *const GVariantType =
    b"*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_simple_action_get_type_once();
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
static mut safe_c2rust_g_simple_action_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_simple_action_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_simple_action_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSimpleAction_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSimpleAction_private_offset,
        );
    }
    safe_c2rust_g_simple_action_class_init(klass as *mut GSimpleActionClass);
}
static mut safe_c2rust_GSimpleAction_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_simple_action_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSimpleAction\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSimpleActionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_action_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSimpleAction>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSimpleAction) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_action_init as unsafe extern "C" fn(*mut GSimpleAction) -> (),
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
            safe_c2rust_g_simple_action_iface_init
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
static mut safe_c2rust_g_simple_action_signals: [guint; 2] = [0; 2];
unsafe extern "C" fn safe_c2rust_g_simple_action_get_name(
    mut action: *mut GAction,
) -> *const gchar {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    return (*simple).name;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_get_parameter_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    return (*simple).parameter_type;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_get_state_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    if !(*simple).state.is_null() {
        return g_variant_get_type((*simple).state);
    } else {
        return ::core::ptr::null::<GVariantType>();
    };
}
unsafe extern "C" fn safe_c2rust_g_simple_action_get_state_hint(
    mut action: *mut GAction,
) -> *mut GVariant {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    if !(*simple).state_hint.is_null() {
        return g_variant_ref((*simple).state_hint);
    } else {
        return ::core::ptr::null_mut::<GVariant>();
    };
}
unsafe extern "C" fn safe_c2rust_g_simple_action_get_enabled(mut action: *mut GAction) -> gboolean {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    return (*simple).enabled;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_change_state(
    mut action: *mut GAction,
    mut value: *mut GVariant,
) {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    if g_signal_has_handler_pending(
        action as gpointer,
        safe_c2rust_g_simple_action_signals[SIGNAL_CHANGE_STATE as ::core::ffi::c_int as usize],
        0 as GQuark,
        TRUE,
    ) != 0
    {
        g_signal_emit(
            action as gpointer,
            safe_c2rust_g_simple_action_signals[SIGNAL_CHANGE_STATE as ::core::ffi::c_int as usize],
            0 as GQuark,
            value,
        );
    } else {
        safe_c2rust_g_simple_action_set_state(simple, value);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_set_state(
    mut simple: *mut GSimpleAction,
    mut value: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_action_get_type();
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
            b"G_IS_SIMPLE_ACTION (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut state_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
    state_type = if !(*simple).state.is_null() {
        g_variant_get_type((*simple).state)
    } else {
        ::core::ptr::null::<GVariantType>()
    };
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !state_type.is_null() {
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
            b"state_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if g_variant_is_of_type(value, state_type) != 0 {
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
            b"g_variant_is_of_type (value, state_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_variant_ref_sink(value);
    if (*simple).state.is_null()
        || g_variant_equal((*simple).state as gconstpointer, value as gconstpointer) == 0
    {
        if !(*simple).state.is_null() {
            g_variant_unref((*simple).state);
        }
        (*simple).state = g_variant_ref(value);
        g_object_notify(
            simple as *mut ::core::ffi::c_void as *mut GObject,
            b"state\0" as *const u8 as *const gchar,
        );
    }
    g_variant_unref(value);
}
unsafe extern "C" fn safe_c2rust_g_simple_action_get_state(
    mut action: *mut GAction,
) -> *mut GVariant {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    return if !(*simple).state.is_null() {
        g_variant_ref((*simple).state)
    } else {
        ::core::ptr::null_mut::<GVariant>()
    };
}
unsafe extern "C" fn safe_c2rust_g_simple_action_activate(
    mut action: *mut GAction,
    mut parameter: *mut GVariant,
) {
    let mut simple: *mut GSimpleAction = action as *mut ::core::ffi::c_void as *mut GSimpleAction;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if if (*simple).parameter_type.is_null() {
            (parameter == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut GVariant)
                as ::core::ffi::c_int
        } else {
            (!parameter.is_null() && g_variant_is_of_type(parameter, (*simple).parameter_type) != 0)
                as ::core::ffi::c_int
        } != 0
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
            b"simple->parameter_type == NULL ? parameter == NULL : (parameter != NULL && g_variant_is_of_type (parameter, simple->parameter_type))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !parameter.is_null() {
        g_variant_ref_sink(parameter);
    }
    if (*simple).enabled != 0 {
        if g_signal_has_handler_pending(
            action as gpointer,
            safe_c2rust_g_simple_action_signals[SIGNAL_ACTIVATE as ::core::ffi::c_int as usize],
            0 as GQuark,
            TRUE,
        ) != 0
        {
            g_signal_emit(
                action as gpointer,
                safe_c2rust_g_simple_action_signals[SIGNAL_ACTIVATE as ::core::ffi::c_int as usize],
                0 as GQuark,
                parameter,
            );
        } else if !(*simple).state.is_null() {
            if parameter.is_null()
                && g_variant_is_of_type((*simple).state, G_VARIANT_TYPE_BOOLEAN) != 0
            {
                let mut was_enabled: gboolean = g_variant_get_boolean((*simple).state);
                safe_c2rust_g_simple_action_change_state(
                    action,
                    g_variant_new_boolean((was_enabled == 0) as ::core::ffi::c_int),
                );
            } else if g_variant_is_of_type((*simple).state, g_variant_get_type(parameter)) != 0 {
                safe_c2rust_g_simple_action_change_state(action, parameter);
            }
        }
    }
    if !parameter.is_null() {
        g_variant_unref(parameter);
    }
}
unsafe extern "C" fn safe_c2rust_g_simple_action_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut action: *mut GSimpleAction = object as *mut ::core::ffi::c_void as *mut GSimpleAction;
    match prop_id {
        1 => {
            (*action).name = safe_c2rust_g_strdup_inline(
                g_value_get_string(value) as *const ::core::ffi::c_char
            ) as *mut gchar;
        }
        2 => {
            (*action).parameter_type = g_value_dup_boxed(value) as *mut GVariantType;
        }
        3 => {
            (*action).enabled = g_value_get_boolean(value);
        }
        5 => {
            if (*action).state_set_already == 0 {
                (*action).state = g_value_dup_variant(value);
                (*action).state_set_already = TRUE as gboolean;
            } else {
                safe_c2rust_g_simple_action_set_state(action, g_value_get_variant(value));
            }
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsimpleaction.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_simple_action_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut action: *mut GAction = object as *mut ::core::ffi::c_void as *mut GAction;
    match prop_id {
        1 => {
            g_value_set_string(value, safe_c2rust_g_simple_action_get_name(action));
        }
        2 => {
            g_value_set_boxed(
                value,
                safe_c2rust_g_simple_action_get_parameter_type(action) as gconstpointer,
            );
        }
        3 => {
            g_value_set_boolean(value, safe_c2rust_g_simple_action_get_enabled(action));
        }
        4 => {
            g_value_set_boxed(
                value,
                safe_c2rust_g_simple_action_get_state_type(action) as gconstpointer,
            );
        }
        5 => {
            g_value_take_variant(value, safe_c2rust_g_simple_action_get_state(action));
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsimpleaction.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                312 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_simple_action_finalize(mut object: *mut GObject) {
    let mut simple: *mut GSimpleAction = object as *mut ::core::ffi::c_void as *mut GSimpleAction;
    g_free((*simple).name as gpointer);
    if !(*simple).parameter_type.is_null() {
        g_variant_type_free((*simple).parameter_type);
    }
    if !(*simple).state.is_null() {
        g_variant_unref((*simple).state);
    }
    if !(*simple).state_hint.is_null() {
        g_variant_unref((*simple).state_hint);
    }
    (*(safe_c2rust_g_simple_action_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_simple_action_init(mut simple: *mut GSimpleAction) {
    (*simple).enabled = TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_iface_init(mut iface: *mut GActionInterface) {
    (*iface).get_name = Some(
        safe_c2rust_g_simple_action_get_name as unsafe extern "C" fn(*mut GAction) -> *const gchar,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *const gchar>;
    (*iface).get_parameter_type = Some(
        safe_c2rust_g_simple_action_get_parameter_type
            as unsafe extern "C" fn(*mut GAction) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>;
    (*iface).get_state_type = Some(
        safe_c2rust_g_simple_action_get_state_type
            as unsafe extern "C" fn(*mut GAction) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>;
    (*iface).get_state_hint = Some(
        safe_c2rust_g_simple_action_get_state_hint
            as unsafe extern "C" fn(*mut GAction) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>;
    (*iface).get_enabled = Some(
        safe_c2rust_g_simple_action_get_enabled as unsafe extern "C" fn(*mut GAction) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> gboolean>;
    (*iface).get_state = Some(
        safe_c2rust_g_simple_action_get_state
            as unsafe extern "C" fn(*mut GAction) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>;
    (*iface).change_state = Some(
        safe_c2rust_g_simple_action_change_state
            as unsafe extern "C" fn(*mut GAction, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>;
    (*iface).activate = Some(
        safe_c2rust_g_simple_action_activate
            as unsafe extern "C" fn(*mut GAction, *mut GVariant) -> (),
    ) as Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_class_init(mut class: *mut GSimpleActionClass) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).set_property = Some(
        safe_c2rust_g_simple_action_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_simple_action_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_simple_action_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_g_simple_action_signals[SIGNAL_ACTIVATE as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"activate\0" as *const u8 as *const gchar),
            safe_c2rust_g_simple_action_get_type(),
            (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_MUST_COLLECT as ::core::ffi::c_int)
                as GSignalFlags,
            0 as guint,
            None,
            NULL_0,
            None,
            G_TYPE_NONE,
            1 as guint,
            G_TYPE_VARIANT,
        );
    safe_c2rust_g_simple_action_signals[SIGNAL_CHANGE_STATE as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"change-state\0" as *const u8 as *const gchar),
            safe_c2rust_g_simple_action_get_type(),
            (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_MUST_COLLECT as ::core::ffi::c_int)
                as GSignalFlags,
            0 as guint,
            None,
            NULL_0,
            None,
            G_TYPE_NONE,
            1 as guint,
            G_TYPE_VARIANT,
        );
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
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
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
            (G_PARAM_READWRITE as ::core::ffi::c_int
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
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_set_enabled(
    mut simple: *mut GSimpleAction,
    mut enabled: gboolean,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_action_get_type();
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
            b"G_IS_SIMPLE_ACTION (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    enabled = (enabled != 0) as ::core::ffi::c_int as gboolean;
    if (*simple).enabled != enabled {
        (*simple).enabled = enabled;
        g_object_notify(
            simple as *mut ::core::ffi::c_void as *mut GObject,
            b"enabled\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_set_state_hint(
    mut simple: *mut GSimpleAction,
    mut state_hint: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_action_get_type();
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
            b"G_IS_SIMPLE_ACTION (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*simple).state_hint.is_null() {
        g_variant_unref((*simple).state_hint);
        (*simple).state_hint = ::core::ptr::null_mut::<GVariant>();
    }
    if !state_hint.is_null() {
        (*simple).state_hint = g_variant_ref(state_hint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_new(
    mut name: *const gchar,
    mut parameter_type: *const GVariantType,
) -> *mut GSimpleAction {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSimpleAction>();
    }
    return g_object_new(
        safe_c2rust_g_simple_action_get_type(),
        b"name\0" as *const u8 as *const gchar,
        name,
        b"parameter-type\0" as *const u8 as *const ::core::ffi::c_char,
        parameter_type,
        NULL_0,
    ) as *mut GSimpleAction;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_new_stateful(
    mut name: *const gchar,
    mut parameter_type: *const GVariantType,
    mut state: *mut GVariant,
) -> *mut GSimpleAction {
    return g_object_new(
        safe_c2rust_g_simple_action_get_type(),
        b"name\0" as *const u8 as *const gchar,
        name,
        b"parameter-type\0" as *const u8 as *const ::core::ffi::c_char,
        parameter_type,
        b"state\0" as *const u8 as *const ::core::ffi::c_char,
        state,
        NULL_0,
    ) as *mut GSimpleAction;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
