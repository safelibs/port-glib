extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GIcon;
    pub type _GEmblem;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_insert_sorted(list: *mut GList, data: gpointer, func: GCompareFunc) -> *mut GList;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_open(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_icon_get_type() -> GType;
    fn g_icon_hash(icon: gconstpointer) -> guint;
    fn g_icon_equal(icon1: *mut GIcon, icon2: *mut GIcon) -> gboolean;
    fn g_icon_to_string(icon: *mut GIcon) -> *mut gchar;
    fn g_icon_new_for_string(str: *const gchar, error: *mut *mut GError) -> *mut GIcon;
    fn g_icon_serialize(icon: *mut GIcon) -> *mut GVariant;
    fn g_emblem_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_io_error_quark() -> GQuark;
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
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
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
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_2 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_2 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_2 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_2 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_2 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_2 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_2 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_2 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_2 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_2 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_2 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_2 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_2 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_2 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_2 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_2 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_2 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_2 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_2 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_2 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_2 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_2 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_2 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_2 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_2 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_2 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_2 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_2 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_2 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_2 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_2 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_2 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_2 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_2 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_2 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_2 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_2 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_2 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_2 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_2 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_2 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_2 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_2 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_2 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_2 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_2 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_2 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_2 = 0;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIconIface {
    pub g_iface: GTypeInterface,
    pub hash: Option<unsafe extern "C" fn(*mut GIcon) -> guint>,
    pub equal: Option<unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean>,
    pub to_tokens: Option<unsafe extern "C" fn(*mut GIcon, *mut GPtrArray, *mut gint) -> gboolean>,
    pub from_tokens:
        Option<unsafe extern "C" fn(*mut *mut gchar, gint, gint, *mut *mut GError) -> *mut GIcon>,
    pub serialize: Option<unsafe extern "C" fn(*mut GIcon) -> *mut GVariant>,
}
pub type GIconIface = _GIconIface;
pub type GEmblem = _GEmblem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEmblemedIcon {
    pub parent_instance: GObject,
    pub priv_0: *mut GEmblemedIconPrivate,
}
pub type GEmblemedIconPrivate = _GEmblemedIconPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEmblemedIconPrivate {
    pub icon: *mut GIcon,
    pub emblems: *mut GList,
}
pub type GEmblemedIcon = _GEmblemedIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEmblemedIconClass {
    pub parent_class: GObjectClass,
}
pub type GEmblemedIconClass = _GEmblemedIconClass;
pub const NUM_PROPERTIES: C2RustUnnamed_3 = 2;
pub const PROP_GICON: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut safe_c2rust_properties: [*mut GParamSpec; 2] = [
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
];
static mut safe_c2rust_g_emblemed_icon_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GEmblemedIcon\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GEmblemedIconClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_emblemed_icon_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GEmblemedIcon>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GEmblemedIcon) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_emblemed_icon_init as unsafe extern "C" fn(*mut GEmblemedIcon) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GEmblemedIcon_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GEmblemedIconPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GIconIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_emblemed_icon_icon_iface_init
                as unsafe extern "C" fn(*mut GIconIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_icon_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_emblemed_icon_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GEmblemedIcon_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GEmblemedIcon_private_offset,
        );
    }
    safe_c2rust_g_emblemed_icon_class_init(klass as *mut GEmblemedIconClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_emblemed_icon_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_emblemed_icon_get_type_once();
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
static mut safe_c2rust_GEmblemedIcon_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_get_instance_private(
    mut self_0: *mut GEmblemedIcon,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GEmblemedIcon_private_offset as glong as isize) as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_finalize(mut object: *mut GObject) {
    let mut emblemed: *mut GEmblemedIcon = ::core::ptr::null_mut::<GEmblemedIcon>();
    emblemed = object as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    let mut _pp: *mut *mut GIcon = &raw mut (*(*emblemed).priv_0).icon;
    let mut _ptr: *mut GIcon = *_pp;
    *_pp = ::core::ptr::null_mut::<GIcon>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_list_free_full(
        (*(*emblemed).priv_0).emblems,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    Some(
        (*(safe_c2rust_g_emblemed_icon_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_set_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GEmblemedIcon = object as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    match property_id {
        1 => {
            (*(*self_0).priv_0).icon = g_value_dup_object(value) as *mut GIcon;
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = property_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gemblemedicon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                93 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_get_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GEmblemedIcon = object as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    match property_id {
        1 => {
            g_value_set_object(value, (*(*self_0).priv_0).icon as gpointer);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = property_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gemblemedicon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_class_init(mut klass: *mut GEmblemedIconClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_emblemed_icon_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_emblemed_icon_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_emblemed_icon_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    safe_c2rust_properties[PROP_GICON as ::core::ffi::c_int as usize] = g_param_spec_object(
        b"gicon\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        g_icon_get_type(),
        (G_PARAM_READWRITE as ::core::ffi::c_int
            | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
    );
    g_object_class_install_properties(
        gobject_class,
        NUM_PROPERTIES as ::core::ffi::c_int as guint,
        &raw mut safe_c2rust_properties as *mut *mut GParamSpec,
    );
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_init(mut emblemed: *mut GEmblemedIcon) {
    (*emblemed).priv_0 =
        safe_c2rust_g_emblemed_icon_get_instance_private(emblemed) as *mut GEmblemedIconPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_emblemed_icon_new(
    mut icon: *mut GIcon,
    mut emblem: *mut GEmblem,
) -> *mut GIcon {
    let mut emblemed: *mut GEmblemedIcon = ::core::ptr::null_mut::<GEmblemedIcon>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = g_icon_get_type();
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
            b"G_IS_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = g_emblem_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) == 0
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
            b"!G_IS_EMBLEM (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    emblemed = g_object_new(
        safe_c2rust_g_emblemed_icon_get_type(),
        b"gicon\0" as *const u8 as *const gchar,
        icon,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GEmblemedIcon;
    if !emblem.is_null() {
        safe_c2rust_g_emblemed_icon_add_emblem(emblemed, emblem);
    }
    return emblemed as *mut ::core::ffi::c_void as *mut GIcon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_emblemed_icon_get_icon(
    mut emblemed: *mut GEmblemedIcon,
) -> *mut GIcon {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = emblemed as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_emblemed_icon_get_type();
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
            b"G_IS_EMBLEMED_ICON (emblemed)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    return (*(*emblemed).priv_0).icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_emblemed_icon_get_emblems(
    mut emblemed: *mut GEmblemedIcon,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = emblemed as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_emblemed_icon_get_type();
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
            b"G_IS_EMBLEMED_ICON (emblemed)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return (*(*emblemed).priv_0).emblems;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_emblemed_icon_clear_emblems(
    mut emblemed: *mut GEmblemedIcon,
) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = emblemed as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_emblemed_icon_get_type();
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
            b"G_IS_EMBLEMED_ICON (emblemed)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*emblemed).priv_0).emblems.is_null() {
        return;
    }
    g_list_free_full(
        (*(*emblemed).priv_0).emblems,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*(*emblemed).priv_0).emblems = ::core::ptr::null_mut::<GList>();
}
unsafe extern "C" fn safe_c2rust_g_emblem_comp(mut a: *mut GEmblem, mut b: *mut GEmblem) -> gint {
    let mut hash_a: guint =
        g_icon_hash(a as *mut ::core::ffi::c_void as *mut GIcon as gconstpointer);
    let mut hash_b: guint =
        g_icon_hash(b as *mut ::core::ffi::c_void as *mut GIcon as gconstpointer);
    if hash_a < hash_b {
        return -(1 as gint);
    }
    if hash_a == hash_b {
        return 0 as gint;
    }
    return 1 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_emblemed_icon_add_emblem(
    mut emblemed: *mut GEmblemedIcon,
    mut emblem: *mut GEmblem,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = emblemed as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_emblemed_icon_get_type();
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
            b"G_IS_EMBLEMED_ICON (emblemed)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = emblem as *mut GTypeInstance;
            let mut __t: GType = g_emblem_get_type();
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
            b"G_IS_EMBLEM (emblem)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(emblem as gpointer);
    (*(*emblemed).priv_0).emblems = g_list_insert_sorted(
        (*(*emblemed).priv_0).emblems,
        emblem as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GEmblem, *mut GEmblem) -> gint>,
            GCompareFunc,
        >(Some(
            safe_c2rust_g_emblem_comp as unsafe extern "C" fn(*mut GEmblem, *mut GEmblem) -> gint,
        )),
    );
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_hash(mut icon: *mut GIcon) -> guint {
    let mut emblemed: *mut GEmblemedIcon = icon as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut hash: guint = g_icon_hash((*(*emblemed).priv_0).icon as gconstpointer);
    list = (*(*emblemed).priv_0).emblems;
    while !list.is_null() {
        hash ^= g_icon_hash((*list).data as *mut GIcon as gconstpointer);
        list = (*list).next;
    }
    return hash;
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_equal(
    mut icon1: *mut GIcon,
    mut icon2: *mut GIcon,
) -> gboolean {
    let mut emblemed1: *mut GEmblemedIcon = icon1 as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    let mut emblemed2: *mut GEmblemedIcon = icon2 as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    let mut list1: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut list2: *mut GList = ::core::ptr::null_mut::<GList>();
    if g_icon_equal((*(*emblemed1).priv_0).icon, (*(*emblemed2).priv_0).icon) == 0 {
        return FALSE;
    }
    list1 = (*(*emblemed1).priv_0).emblems;
    list2 = (*(*emblemed2).priv_0).emblems;
    while !list1.is_null() && !list2.is_null() {
        if g_icon_equal((*list1).data as *mut GIcon, (*list2).data as *mut GIcon) == 0 {
            return FALSE;
        }
        list1 = (*list1).next;
        list2 = (*list2).next;
    }
    return (list1.is_null() && list2.is_null()) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_to_tokens(
    mut icon: *mut GIcon,
    mut tokens: *mut GPtrArray,
    mut out_version: *mut gint,
) -> gboolean {
    let mut emblemed_icon: *mut GEmblemedIcon =
        icon as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !out_version.is_null() {
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
            b"out_version != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    *out_version = 0 as ::core::ffi::c_int as gint;
    s = g_icon_to_string((*(*emblemed_icon).priv_0).icon) as *mut ::core::ffi::c_char;
    if s.is_null() {
        return FALSE;
    }
    g_ptr_array_add(tokens, s as gpointer);
    l = (*(*emblemed_icon).priv_0).emblems;
    while !l.is_null() {
        let mut emblem_icon: *mut GIcon = (*l).data as *mut GIcon;
        s = g_icon_to_string(emblem_icon) as *mut ::core::ffi::c_char;
        if s.is_null() {
            return FALSE;
        }
        g_ptr_array_add(tokens, s as gpointer);
        l = (*l).next;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_from_tokens(
    mut tokens: *mut *mut gchar,
    mut num_tokens: gint,
    mut version: gint,
    mut error: *mut *mut GError,
) -> *mut GIcon {
    let mut current_block: u64;
    let mut emblemed_icon: *mut GEmblemedIcon = ::core::ptr::null_mut::<GEmblemedIcon>();
    let mut n: ::core::ffi::c_int = 0;
    emblemed_icon = ::core::ptr::null_mut::<GEmblemedIcon>();
    if version != 0 as ::core::ffi::c_int {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t handle version %d of GEmblemedIcon encoding\0" as *const u8
                    as *const gchar,
            ),
            version,
        );
    } else if num_tokens < 1 as ::core::ffi::c_int {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Malformed number of tokens (%d) in GEmblemedIcon encoding\0" as *const u8
                    as *const gchar,
            ),
            num_tokens,
        );
    } else {
        emblemed_icon = g_object_new(
            safe_c2rust_g_emblemed_icon_get_type(),
            ::core::ptr::null::<gchar>(),
        ) as *mut GEmblemedIcon;
        (*(*emblemed_icon).priv_0).icon =
            g_icon_new_for_string(*tokens.offset(0 as ::core::ffi::c_int as isize), error);
        if !(*(*emblemed_icon).priv_0).icon.is_null() {
            n = 1 as ::core::ffi::c_int;
            loop {
                if !(n < num_tokens) {
                    current_block = 7976072742316086414;
                    break;
                }
                let mut emblem: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
                emblem = g_icon_new_for_string(*tokens.offset(n as isize), error);
                if emblem.is_null() {
                    current_block = 1059732163483880142;
                    break;
                }
                if ({
                    let mut __inst: *mut GTypeInstance = emblem as *mut GTypeInstance;
                    let mut __t: GType = g_emblem_get_type();
                    let mut __r: gboolean = 0;
                    if __inst.is_null() {
                        __r = FALSE as gboolean;
                    } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                        __r = TRUE as gboolean;
                    } else {
                        __r = g_type_check_instance_is_a(__inst, __t);
                    }
                    __r
                }) == 0
                {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Expected a GEmblem for GEmblemedIcon\0" as *const u8 as *const gchar,
                        ),
                    );
                    g_object_unref(emblem as gpointer);
                    current_block = 1059732163483880142;
                    break;
                } else {
                    (*(*emblemed_icon).priv_0).emblems =
                        g_list_append((*(*emblemed_icon).priv_0).emblems, emblem as gpointer);
                    n += 1;
                }
            }
            match current_block {
                1059732163483880142 => {}
                _ => return emblemed_icon as *mut ::core::ffi::c_void as *mut GIcon,
            }
        }
    }
    if !emblemed_icon.is_null() {
        g_object_unref(emblemed_icon as gpointer);
    }
    return ::core::ptr::null_mut::<GIcon>();
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_serialize(mut icon: *mut GIcon) -> *mut GVariant {
    let mut emblemed_icon: *mut GEmblemedIcon =
        icon as *mut ::core::ffi::c_void as *mut GEmblemedIcon;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut icon_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut node: *mut GList = ::core::ptr::null_mut::<GList>();
    icon_data = g_icon_serialize((*(*emblemed_icon).priv_0).icon);
    if icon_data.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"(va(va{sv}))\0" as *const u8 as *const gchar),
    );
    g_variant_builder_add(
        &raw mut builder,
        b"v\0" as *const u8 as *const gchar,
        icon_data,
    );
    g_variant_unref(icon_data);
    g_variant_builder_open(
        &raw mut builder,
        g_variant_type_checked_(b"a(va{sv})\0" as *const u8 as *const gchar),
    );
    node = (*(*emblemed_icon).priv_0).emblems;
    while !node.is_null() {
        icon_data = g_icon_serialize((*node).data as *mut GIcon);
        if !icon_data.is_null() {
            if g_variant_is_of_type(
                icon_data,
                g_variant_type_checked_(b"(sv)\0" as *const u8 as *const gchar),
            ) != 0
            {
                let mut name: *const gchar = ::core::ptr::null::<gchar>();
                let mut content: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                g_variant_get(
                    icon_data,
                    b"(&sv)\0" as *const u8 as *const gchar,
                    &raw mut name,
                    &raw mut content,
                );
                if strcmp(
                    name as *const ::core::ffi::c_char,
                    b"emblem\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    && g_variant_is_of_type(
                        content,
                        g_variant_type_checked_(b"(va{sv})\0" as *const u8 as *const gchar),
                    ) != 0
                {
                    g_variant_builder_add(
                        &raw mut builder,
                        b"@(va{sv})\0" as *const u8 as *const gchar,
                        content,
                    );
                }
                g_variant_unref(content);
            }
            g_variant_unref(icon_data);
        }
        node = (*node).next;
    }
    g_variant_builder_close(&raw mut builder);
    return g_variant_new(
        b"(sv)\0" as *const u8 as *const gchar,
        b"emblemed\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_builder_end(&raw mut builder),
    );
}
unsafe extern "C" fn safe_c2rust_g_emblemed_icon_icon_iface_init(mut iface: *mut GIconIface) {
    (*iface).hash =
        Some(safe_c2rust_g_emblemed_icon_hash as unsafe extern "C" fn(*mut GIcon) -> guint)
            as Option<unsafe extern "C" fn(*mut GIcon) -> guint>;
    (*iface).equal = Some(
        safe_c2rust_g_emblemed_icon_equal
            as unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean>;
    (*iface).to_tokens = Some(
        safe_c2rust_g_emblemed_icon_to_tokens
            as unsafe extern "C" fn(*mut GIcon, *mut GPtrArray, *mut gint) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GIcon, *mut GPtrArray, *mut gint) -> gboolean>;
    (*iface).from_tokens = Some(
        safe_c2rust_g_emblemed_icon_from_tokens
            as unsafe extern "C" fn(*mut *mut gchar, gint, gint, *mut *mut GError) -> *mut GIcon,
    )
        as Option<
            unsafe extern "C" fn(*mut *mut gchar, gint, gint, *mut *mut GError) -> *mut GIcon,
        >;
    (*iface).serialize = Some(
        safe_c2rust_g_emblemed_icon_serialize as unsafe extern "C" fn(*mut GIcon) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GIcon) -> *mut GVariant>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
