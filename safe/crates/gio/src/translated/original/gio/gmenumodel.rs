use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GMenuModelPrivate;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_ref(hash_table: *mut GHashTable) -> *mut GHashTable;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_get_va(
        value: *mut GVariant,
        format_string: *const gchar,
        endptr: *mut *const gchar,
        app: *mut ::core::ffi::VaList,
    );
    fn g_variant_check_format_string(
        value: *mut GVariant,
        format_string: *const gchar,
        copy_only: gboolean,
    ) -> gboolean;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn _g_cclosure_marshal_VOID__INT_INT_INT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__INT_INT_INTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type guint8 = ::core::ffi::c_uchar;
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
pub type va_list = __builtin_va_list;
pub type GQuark = guint32;
pub type GData = _GData;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
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
pub struct _GMenuModel {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuModelPrivate,
}
pub type GMenuModelPrivate = _GMenuModelPrivate;
pub type GMenuModel = _GMenuModel;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuModelClass {
    pub parent_class: GObjectClass,
    pub is_mutable: Option<unsafe extern "C" fn(*mut GMenuModel) -> gboolean>,
    pub get_n_items: Option<unsafe extern "C" fn(*mut GMenuModel) -> gint>,
    pub get_item_attributes:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>,
    pub iterate_item_attributes:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuAttributeIter>,
    pub get_item_attribute_value: Option<
        unsafe extern "C" fn(
            *mut GMenuModel,
            gint,
            *const gchar,
            *const GVariantType,
        ) -> *mut GVariant,
    >,
    pub get_item_links:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>,
    pub iterate_item_links:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuLinkIter>,
    pub get_item_link:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint, *const gchar) -> *mut GMenuModel>,
}
pub type GMenuLinkIter = _GMenuLinkIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuLinkIter {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuLinkIterPrivate,
}
pub type GMenuLinkIterPrivate = _GMenuLinkIterPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuLinkIterPrivate {
    pub name: GQuark,
    pub value: *mut GMenuModel,
    pub valid: gboolean,
}
pub type GMenuAttributeIter = _GMenuAttributeIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuAttributeIter {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuAttributeIterPrivate,
}
pub type GMenuAttributeIterPrivate = _GMenuAttributeIterPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuAttributeIterPrivate {
    pub name: GQuark,
    pub value: *mut GVariant,
    pub valid: gboolean,
}
pub type GMenuModelClass = _GMenuModelClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuAttributeIterClass {
    pub parent_class: GObjectClass,
    pub get_next: Option<
        unsafe extern "C" fn(
            *mut GMenuAttributeIter,
            *mut *const gchar,
            *mut *mut GVariant,
        ) -> gboolean,
    >,
}
pub type GMenuAttributeIterClass = _GMenuAttributeIterClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuLinkIterClass {
    pub parent_class: GObjectClass,
    pub get_next: Option<
        unsafe extern "C" fn(
            *mut GMenuLinkIter,
            *mut *const gchar,
            *mut *mut GMenuModel,
        ) -> gboolean,
    >,
}
pub type GMenuLinkIterClass = _GMenuLinkIterClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMenuLinkHashIter {
    pub parent_instance: GMenuLinkIter,
    pub iter: GHashTableIter,
    pub table: *mut GHashTable,
}
pub type GMenuLinkHashIterClass = GMenuLinkIterClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMenuAttributeHashIter {
    pub parent_instance: GMenuAttributeIter,
    pub iter: GHashTableIter,
    pub table: *mut GHashTable,
}
pub type GMenuAttributeHashIterClass = GMenuAttributeIterClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT: GType = ((6 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_GMenuLinkHashIter_private_offset: gint = 0;
static mut safe_c2rust_g_menu_link_hash_iter_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_menu_link_hash_iter_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_menu_link_hash_iter_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMenuLinkHashIter_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMenuLinkHashIter_private_offset,
        );
    }
    safe_c2rust_g_menu_link_hash_iter_class_init(klass as *mut GMenuLinkHashIterClass);
}
unsafe extern "C" fn safe_c2rust_g_menu_link_hash_iter_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_menu_link_hash_iter_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_menu_link_hash_iter_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        safe_c2rust_g_menu_link_iter_get_type(),
        g_intern_static_string(b"GMenuLinkHashIter\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMenuLinkHashIterClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_link_hash_iter_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMenuLinkHashIter>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenuLinkHashIter) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_link_hash_iter_init
                    as unsafe extern "C" fn(*mut GMenuLinkHashIter) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_menu_link_hash_iter_get_next(
    mut link_iter: *mut GMenuLinkIter,
    mut out_name: *mut *const gchar,
    mut value: *mut *mut GMenuModel,
) -> gboolean {
    let mut iter: *mut GMenuLinkHashIter = link_iter as *mut GMenuLinkHashIter;
    let mut keyptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut valueptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if g_hash_table_iter_next(&raw mut (*iter).iter, &raw mut keyptr, &raw mut valueptr) == 0 {
        return FALSE;
    }
    *out_name = keyptr as *const gchar;
    *value = g_object_ref(valueptr) as *mut GMenuModel;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_menu_link_hash_iter_finalize(mut object: *mut GObject) {
    let mut iter: *mut GMenuLinkHashIter = object as *mut GMenuLinkHashIter;
    g_hash_table_unref((*iter).table);
    (*(safe_c2rust_g_menu_link_hash_iter_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_menu_link_hash_iter_init(mut iter: *mut GMenuLinkHashIter) {}
unsafe extern "C" fn safe_c2rust_g_menu_link_hash_iter_class_init(
    mut class: *mut GMenuLinkHashIterClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize = Some(
        safe_c2rust_g_menu_link_hash_iter_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*class).get_next = Some(
        safe_c2rust_g_menu_link_hash_iter_get_next
            as unsafe extern "C" fn(
                *mut GMenuLinkIter,
                *mut *const gchar,
                *mut *mut GMenuModel,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMenuLinkIter,
                *mut *const gchar,
                *mut *mut GMenuModel,
            ) -> gboolean,
        >;
}
static mut safe_c2rust_g_menu_attribute_hash_iter_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_menu_attribute_hash_iter_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_menu_attribute_hash_iter_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMenuAttributeHashIter_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMenuAttributeHashIter_private_offset,
        );
    }
    safe_c2rust_g_menu_attribute_hash_iter_class_init(klass as *mut GMenuAttributeHashIterClass);
}
static mut safe_c2rust_GMenuAttributeHashIter_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_menu_attribute_hash_iter_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        safe_c2rust_g_menu_attribute_iter_get_type(),
        g_intern_static_string(b"GMenuAttributeHashIter\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMenuAttributeHashIterClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_attribute_hash_iter_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMenuAttributeHashIter>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenuAttributeHashIter) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_attribute_hash_iter_init
                    as unsafe extern "C" fn(*mut GMenuAttributeHashIter) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_hash_iter_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_menu_attribute_hash_iter_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_menu_attribute_hash_iter_get_next(
    mut attr_iter: *mut GMenuAttributeIter,
    mut name: *mut *const gchar,
    mut value: *mut *mut GVariant,
) -> gboolean {
    let mut iter: *mut GMenuAttributeHashIter = attr_iter as *mut GMenuAttributeHashIter;
    let mut keyptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut valueptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if g_hash_table_iter_next(&raw mut (*iter).iter, &raw mut keyptr, &raw mut valueptr) == 0 {
        return FALSE;
    }
    *name = keyptr as *const gchar;
    *value = g_variant_ref(valueptr as *mut GVariant);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_hash_iter_finalize(mut object: *mut GObject) {
    let mut iter: *mut GMenuAttributeHashIter = object as *mut GMenuAttributeHashIter;
    g_hash_table_unref((*iter).table);
    (*(safe_c2rust_g_menu_attribute_hash_iter_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_hash_iter_init(
    mut iter: *mut GMenuAttributeHashIter,
) {
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_hash_iter_class_init(
    mut class: *mut GMenuAttributeHashIterClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize = Some(
        safe_c2rust_g_menu_attribute_hash_iter_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*class).get_next = Some(
        safe_c2rust_g_menu_attribute_hash_iter_get_next
            as unsafe extern "C" fn(
                *mut GMenuAttributeIter,
                *mut *const gchar,
                *mut *mut GVariant,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMenuAttributeIter,
                *mut *const gchar,
                *mut *mut GVariant,
            ) -> gboolean,
        >;
}
static mut safe_c2rust_GMenuModel_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_menu_model_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_menu_model_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GMenuModel\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMenuModelClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_model_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMenuModel>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenuModel) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_model_init as unsafe extern "C" fn(*mut GMenuModel) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_menu_model_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_menu_model_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMenuModel_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GMenuModel_private_offset);
    }
    safe_c2rust_g_menu_model_class_init(klass as *mut GMenuModelClass);
}
static mut safe_c2rust_g_menu_model_parent_class: gpointer = NULL;
static mut safe_c2rust_g_menu_model_items_changed_signal: guint = 0;
unsafe extern "C" fn safe_c2rust_g_menu_model_real_iterate_item_attributes(
    mut model: *mut GMenuModel,
    mut item_index: gint,
) -> *mut GMenuAttributeIter {
    let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut result: *mut GMenuAttributeIter = ::core::ptr::null_mut::<GMenuAttributeIter>();
    (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .get_item_attributes
        .expect("non-null function pointer")(model, item_index, &raw mut table);
    if !table.is_null() {
        let mut iter: *mut GMenuAttributeHashIter = g_object_new(
            safe_c2rust_g_menu_attribute_hash_iter_get_type(),
            ::core::ptr::null::<gchar>(),
        ) as *mut GMenuAttributeHashIter;
        g_hash_table_iter_init(&raw mut (*iter).iter, table);
        (*iter).table = g_hash_table_ref(table);
        result = iter as *mut ::core::ffi::c_void as *mut GMenuAttributeIter;
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GMenuModel implementation '%s' doesn't override iterate_item_attributes() and fails to return valid values from get_item_attributes()\0"
                as *const u8 as *const gchar,
            g_type_name((*(*(model as *mut GTypeInstance)).g_class).g_type),
        );
        result = ::core::ptr::null_mut::<GMenuAttributeIter>();
    }
    if !table.is_null() {
        g_hash_table_unref(table);
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_g_menu_model_real_get_item_attribute_value(
    mut model: *mut GMenuModel,
    mut item_index: gint,
    mut attribute: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .get_item_attributes
        .expect("non-null function pointer")(model, item_index, &raw mut table);
    if !table.is_null() {
        value = g_hash_table_lookup(table, attribute as gconstpointer) as *mut GVariant;
        if !value.is_null() {
            if expected_type.is_null() || g_variant_is_of_type(value, expected_type) != 0 {
                value = g_variant_ref(value);
            } else {
                value = ::core::ptr::null_mut::<GVariant>();
            }
        }
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenumodel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            340 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if !table.is_null() {
        g_hash_table_unref(table);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_g_menu_model_real_iterate_item_links(
    mut model: *mut GMenuModel,
    mut item_index: gint,
) -> *mut GMenuLinkIter {
    let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut result: *mut GMenuLinkIter = ::core::ptr::null_mut::<GMenuLinkIter>();
    (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .get_item_links
        .expect("non-null function pointer")(model, item_index, &raw mut table);
    if !table.is_null() {
        let mut iter: *mut GMenuLinkHashIter = g_object_new(
            safe_c2rust_g_menu_link_hash_iter_get_type(),
            ::core::ptr::null::<gchar>(),
        ) as *mut GMenuLinkHashIter;
        g_hash_table_iter_init(&raw mut (*iter).iter, table);
        (*iter).table = g_hash_table_ref(table);
        result = iter as *mut ::core::ffi::c_void as *mut GMenuLinkIter;
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GMenuModel implementation '%s' doesn't override iterate_item_links() and fails to return valid values from get_item_links()\0"
                as *const u8 as *const gchar,
            g_type_name((*(*(model as *mut GTypeInstance)).g_class).g_type),
        );
        result = ::core::ptr::null_mut::<GMenuLinkIter>();
    }
    if !table.is_null() {
        g_hash_table_unref(table);
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_g_menu_model_real_get_item_link(
    mut model: *mut GMenuModel,
    mut item_index: gint,
    mut link: *const gchar,
) -> *mut GMenuModel {
    let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut value: *mut GMenuModel = ::core::ptr::null_mut::<GMenuModel>();
    (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .get_item_links
        .expect("non-null function pointer")(model, item_index, &raw mut table);
    if !table.is_null() {
        value = g_hash_table_lookup(table, link as gconstpointer) as *mut GMenuModel;
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenumodel.c\0" as *const u8
                as *const ::core::ffi::c_char,
            393 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if !value.is_null() {
        g_object_ref(value as gpointer);
    }
    if !table.is_null() {
        g_hash_table_unref(table);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_g_menu_model_init(mut model: *mut GMenuModel) {}
unsafe extern "C" fn safe_c2rust_g_menu_model_class_init(mut class: *mut GMenuModelClass) {
    (*class).iterate_item_attributes = Some(
        safe_c2rust_g_menu_model_real_iterate_item_attributes
            as unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuAttributeIter,
    )
        as Option<unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuAttributeIter>;
    (*class).get_item_attribute_value = Some(
        safe_c2rust_g_menu_model_real_get_item_attribute_value
            as unsafe extern "C" fn(
                *mut GMenuModel,
                gint,
                *const gchar,
                *const GVariantType,
            ) -> *mut GVariant,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMenuModel,
                gint,
                *const gchar,
                *const GVariantType,
            ) -> *mut GVariant,
        >;
    (*class).iterate_item_links = Some(
        safe_c2rust_g_menu_model_real_iterate_item_links
            as unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuLinkIter,
    )
        as Option<unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuLinkIter>;
    (*class).get_item_link = Some(
        safe_c2rust_g_menu_model_real_get_item_link
            as unsafe extern "C" fn(*mut GMenuModel, gint, *const gchar) -> *mut GMenuModel,
    )
        as Option<unsafe extern "C" fn(*mut GMenuModel, gint, *const gchar) -> *mut GMenuModel>;
    safe_c2rust_g_menu_model_items_changed_signal = g_signal_new(
        g_intern_static_string(b"items-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_menu_model_get_type(),
        G_SIGNAL_RUN_LAST,
        0 as guint,
        None,
        NULL,
        Some(
            _g_cclosure_marshal_VOID__INT_INT_INT
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
        3 as guint,
        G_TYPE_INT,
        G_TYPE_INT,
        G_TYPE_INT,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_g_menu_model_items_changed_signal,
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__INT_INT_INTv
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
pub unsafe extern "C" fn safe_c2rust_g_menu_model_is_mutable(
    mut model: *mut GMenuModel,
) -> gboolean {
    return (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .is_mutable
        .expect("non-null function pointer")(model);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_get_n_items(mut model: *mut GMenuModel) -> gint {
    return (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .get_n_items
        .expect("non-null function pointer")(model);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_iterate_item_attributes(
    mut model: *mut GMenuModel,
    mut item_index: gint,
) -> *mut GMenuAttributeIter {
    return (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .iterate_item_attributes
        .expect("non-null function pointer")(model, item_index);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_get_item_attribute_value(
    mut model: *mut GMenuModel,
    mut item_index: gint,
    mut attribute: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    return (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .get_item_attribute_value
        .expect("non-null function pointer")(
        model, item_index, attribute, expected_type
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_get_item_attribute(
    mut model: *mut GMenuModel,
    mut item_index: gint,
    mut attribute: *const gchar,
    mut format_string: *const gchar,
    mut args: ...
) -> gboolean {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    value = safe_c2rust_g_menu_model_get_item_attribute_value(
        model,
        item_index,
        attribute,
        ::core::ptr::null::<GVariantType>(),
    );
    if value.is_null() {
        return FALSE;
    }
    if g_variant_check_format_string(value, format_string, TRUE) == 0 {
        g_variant_unref(value);
        return FALSE;
    }
    ap = args.clone();
    g_variant_get_va(
        value,
        format_string,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
    g_variant_unref(value);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_iterate_item_links(
    mut model: *mut GMenuModel,
    mut item_index: gint,
) -> *mut GMenuLinkIter {
    return (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .iterate_item_links
        .expect("non-null function pointer")(model, item_index);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_get_item_link(
    mut model: *mut GMenuModel,
    mut item_index: gint,
    mut link: *const gchar,
) -> *mut GMenuModel {
    return (*((*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass))
        .get_item_link
        .expect("non-null function pointer")(model, item_index, link);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_model_items_changed(
    mut model: *mut GMenuModel,
    mut position: gint,
    mut removed: gint,
    mut added: gint,
) {
    g_signal_emit(
        model as gpointer,
        safe_c2rust_g_menu_model_items_changed_signal,
        0 as GQuark,
        position,
        removed,
        added,
    );
}
static mut safe_c2rust_GMenuAttributeIter_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_menu_attribute_iter_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GMenuAttributeIter\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMenuAttributeIterClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_attribute_iter_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMenuAttributeIter>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenuAttributeIter) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_attribute_iter_init
                    as unsafe extern "C" fn(*mut GMenuAttributeIter) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GMenuAttributeIter_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GMenuAttributeIterPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_get_instance_private(
    mut self_0: *mut GMenuAttributeIter,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GMenuAttributeIter_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_menu_attribute_iter_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMenuAttributeIter_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMenuAttributeIter_private_offset,
        );
    }
    safe_c2rust_g_menu_attribute_iter_class_init(klass as *mut GMenuAttributeIterClass);
}
static mut safe_c2rust_g_menu_attribute_iter_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_get_next(
    mut iter: *mut GMenuAttributeIter,
    mut out_name: *mut *const gchar,
    mut value: *mut *mut GVariant,
) -> gboolean {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    if !(*(*iter).priv_0).value.is_null() {
        g_variant_unref((*(*iter).priv_0).value);
        (*(*iter).priv_0).value = ::core::ptr::null_mut::<GVariant>();
    }
    (*(*iter).priv_0).valid = (*((*(iter as *mut GTypeInstance)).g_class
        as *mut GMenuAttributeIterClass))
        .get_next
        .expect("non-null function pointer")(
        iter, &raw mut name, &raw mut (*(*iter).priv_0).value
    );
    if (*(*iter).priv_0).valid != 0 {
        (*(*iter).priv_0).name = g_quark_from_string(name);
        if !out_name.is_null() {
            *out_name = g_quark_to_string((*(*iter).priv_0).name);
        }
        if !value.is_null() {
            *value = g_variant_ref((*(*iter).priv_0).value);
        }
    }
    return (*(*iter).priv_0).valid;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_next(
    mut iter: *mut GMenuAttributeIter,
) -> gboolean {
    return safe_c2rust_g_menu_attribute_iter_get_next(
        iter,
        ::core::ptr::null_mut::<*const gchar>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_get_name(
    mut iter: *mut GMenuAttributeIter,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*(*iter).priv_0).valid != 0 {
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
            b"iter->priv->valid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return g_quark_to_string((*(*iter).priv_0).name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_get_value(
    mut iter: *mut GMenuAttributeIter,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*(*iter).priv_0).valid != 0 {
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
            b"iter->priv->valid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_ref((*(*iter).priv_0).value);
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_finalize(mut object: *mut GObject) {
    let mut iter: *mut GMenuAttributeIter =
        object as *mut ::core::ffi::c_void as *mut GMenuAttributeIter;
    if !(*(*iter).priv_0).value.is_null() {
        g_variant_unref((*(*iter).priv_0).value);
    }
    (*(safe_c2rust_g_menu_attribute_iter_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_init(mut iter: *mut GMenuAttributeIter) {
    (*iter).priv_0 = safe_c2rust_g_menu_attribute_iter_get_instance_private(iter)
        as *mut GMenuAttributeIterPrivate;
}
unsafe extern "C" fn safe_c2rust_g_menu_attribute_iter_class_init(
    mut class: *mut GMenuAttributeIterClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize = Some(
        safe_c2rust_g_menu_attribute_iter_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_menu_link_iter_get_instance_private(
    mut self_0: *mut GMenuLinkIter,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GMenuLinkIter_private_offset as glong as isize) as gpointer;
}
static mut safe_c2rust_GMenuLinkIter_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_menu_link_iter_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GMenuLinkIter\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMenuLinkIterClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_link_iter_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMenuLinkIter>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenuLinkIter) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_link_iter_init as unsafe extern "C" fn(*mut GMenuLinkIter) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GMenuLinkIter_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GMenuLinkIterPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_link_iter_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_menu_link_iter_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_menu_link_iter_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_menu_link_iter_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMenuLinkIter_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMenuLinkIter_private_offset,
        );
    }
    safe_c2rust_g_menu_link_iter_class_init(klass as *mut GMenuLinkIterClass);
}
static mut safe_c2rust_g_menu_link_iter_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_link_iter_get_next(
    mut iter: *mut GMenuLinkIter,
    mut out_link: *mut *const gchar,
    mut value: *mut *mut GMenuModel,
) -> gboolean {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    if !(*(*iter).priv_0).value.is_null() {
        g_object_unref((*(*iter).priv_0).value as gpointer);
        (*(*iter).priv_0).value = ::core::ptr::null_mut::<GMenuModel>();
    }
    (*(*iter).priv_0).valid = (*((*(iter as *mut GTypeInstance)).g_class
        as *mut GMenuLinkIterClass))
        .get_next
        .expect("non-null function pointer")(
        iter, &raw mut name, &raw mut (*(*iter).priv_0).value
    );
    if (*(*iter).priv_0).valid != 0 {
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if !name.is_null() {
                _g_boolean_var_12 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_12 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_12
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenumodel.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                898 as ::core::ffi::c_int,
                G_STRFUNC,
                b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*iter).priv_0).name = g_quark_from_string(name);
        if !out_link.is_null() {
            *out_link = g_quark_to_string((*(*iter).priv_0).name);
        }
        if !value.is_null() {
            *value = g_object_ref((*(*iter).priv_0).value as gpointer) as *mut GMenuModel
                as *mut GMenuModel;
        }
    }
    return (*(*iter).priv_0).valid;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_link_iter_next(
    mut iter: *mut GMenuLinkIter,
) -> gboolean {
    return safe_c2rust_g_menu_link_iter_get_next(
        iter,
        ::core::ptr::null_mut::<*const gchar>(),
        ::core::ptr::null_mut::<*mut GMenuModel>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_link_iter_get_name(
    mut iter: *mut GMenuLinkIter,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*(*iter).priv_0).valid != 0 {
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
            b"iter->priv->valid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return g_quark_to_string((*(*iter).priv_0).name);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_link_iter_get_value(
    mut iter: *mut GMenuLinkIter,
) -> *mut GMenuModel {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*(*iter).priv_0).valid != 0 {
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
            b"iter->priv->valid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMenuModel>();
    }
    return g_object_ref((*(*iter).priv_0).value as gpointer) as *mut GMenuModel;
}
unsafe extern "C" fn safe_c2rust_g_menu_link_iter_finalize(mut object: *mut GObject) {
    let mut iter: *mut GMenuLinkIter = object as *mut ::core::ffi::c_void as *mut GMenuLinkIter;
    if !(*(*iter).priv_0).value.is_null() {
        g_object_unref((*(*iter).priv_0).value as gpointer);
    }
    (*(safe_c2rust_g_menu_link_iter_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_menu_link_iter_init(mut iter: *mut GMenuLinkIter) {
    (*iter).priv_0 =
        safe_c2rust_g_menu_link_iter_get_instance_private(iter) as *mut GMenuLinkIterPrivate;
}
unsafe extern "C" fn safe_c2rust_g_menu_link_iter_class_init(mut class: *mut GMenuLinkIterClass) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_menu_link_iter_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
