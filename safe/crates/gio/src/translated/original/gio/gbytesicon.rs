extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GIcon;
    pub type _GInputStreamPrivate;
    pub type _GLoadableIcon;
    pub type _GTask;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_bytes_hash(bytes: gconstpointer) -> guint;
    fn g_bytes_equal(bytes1: gconstpointer, bytes2: gconstpointer) -> gboolean;
    fn g_variant_new_from_bytes(
        type_0: *const GVariantType,
        bytes: *mut GBytes,
        trusted: gboolean,
    ) -> *mut GVariant;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_bytes_get_type() -> GType;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_dup_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_icon_get_type() -> GType;
    fn g_loadable_icon_get_type() -> GType;
    fn g_memory_input_stream_new_from_bytes(bytes: *mut GBytes) -> *mut GInputStream;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBytes = _GBytes;
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
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GInputStream = _GInputStream;
pub type GLoadableIcon = _GLoadableIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBytesIcon {
    pub parent_instance: GObject,
    pub bytes: *mut GBytes,
}
pub type GBytesIcon = _GBytesIcon;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GBytesIconClass = GObjectClass;
pub const PROP_BYTES: C2RustUnnamed_0 = 1;
pub type GLoadableIconIface = _GLoadableIconIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLoadableIconIface {
    pub g_iface: GTypeInterface,
    pub load: Option<
        unsafe extern "C" fn(
            *mut GLoadableIcon,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GInputStream,
    >,
    pub load_async: Option<
        unsafe extern "C" fn(
            *mut GLoadableIcon,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub load_finish: Option<
        unsafe extern "C" fn(
            *mut GLoadableIcon,
            *mut GAsyncResult,
            *mut *mut ::core::ffi::c_char,
            *mut *mut GError,
        ) -> *mut GInputStream,
    >,
}
pub type GIconIface = _GIconIface;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_VARIANT_TYPE_BYTESTRING: *const GVariantType =
    b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_bytes_icon_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GBytesIcon\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GBytesIconClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_bytes_icon_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GBytesIcon>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GBytesIcon) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_bytes_icon_init as unsafe extern "C" fn(*mut GBytesIcon) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GIconIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_bytes_icon_icon_iface_init as unsafe extern "C" fn(*mut GIconIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_icon_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GLoadableIconIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_bytes_icon_loadable_icon_iface_init
                as unsafe extern "C" fn(*mut GLoadableIconIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_loadable_icon_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_icon_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_bytes_icon_get_type_once();
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
static mut safe_c2rust_g_bytes_icon_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_bytes_icon_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_bytes_icon_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GBytesIcon_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GBytesIcon_private_offset);
    }
    safe_c2rust_g_bytes_icon_class_init(klass as *mut GBytesIconClass);
}
static mut safe_c2rust_GBytesIcon_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_bytes_icon_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut icon: *mut GBytesIcon = object as *mut ::core::ffi::c_void as *mut GBytesIcon;
    match prop_id {
        1 => {
            g_value_set_boxed(value, (*icon).bytes as gconstpointer);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbytesicon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                80 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut icon: *mut GBytesIcon = object as *mut ::core::ffi::c_void as *mut GBytesIcon;
    match prop_id {
        1 => {
            (*icon).bytes = g_value_dup_boxed(value) as *mut GBytes;
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gbytesicon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                99 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_finalize(mut object: *mut GObject) {
    let mut icon: *mut GBytesIcon = ::core::ptr::null_mut::<GBytesIcon>();
    icon = object as *mut ::core::ffi::c_void as *mut GBytesIcon;
    g_bytes_unref((*icon).bytes);
    (*(safe_c2rust_g_bytes_icon_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_class_init(mut klass: *mut GBytesIconClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_bytes_icon_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_bytes_icon_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_bytes_icon_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_BYTES as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"bytes\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_bytes_get_type(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_init(mut bytes: *mut GBytesIcon) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_icon_new(mut bytes: *mut GBytes) -> *mut GIcon {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    return g_object_new(
        safe_c2rust_g_bytes_icon_get_type(),
        b"bytes\0" as *const u8 as *const gchar,
        bytes,
        NULL,
    ) as *mut GIcon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bytes_icon_get_bytes(
    mut icon: *mut GBytesIcon,
) -> *mut GBytes {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_bytes_icon_get_type();
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
            b"G_IS_BYTES_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    return (*icon).bytes;
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_hash(mut icon: *mut GIcon) -> guint {
    let mut bytes_icon: *mut GBytesIcon = icon as *mut ::core::ffi::c_void as *mut GBytesIcon;
    return g_bytes_hash((*bytes_icon).bytes as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_equal(
    mut icon1: *mut GIcon,
    mut icon2: *mut GIcon,
) -> gboolean {
    let mut bytes1: *mut GBytesIcon = icon1 as *mut ::core::ffi::c_void as *mut GBytesIcon;
    let mut bytes2: *mut GBytesIcon = icon2 as *mut ::core::ffi::c_void as *mut GBytesIcon;
    return g_bytes_equal(
        (*bytes1).bytes as gconstpointer,
        (*bytes2).bytes as gconstpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_serialize(mut icon: *mut GIcon) -> *mut GVariant {
    let mut bytes_icon: *mut GBytesIcon = icon as *mut ::core::ffi::c_void as *mut GBytesIcon;
    return g_variant_new(
        b"(sv)\0" as *const u8 as *const gchar,
        b"bytes\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new_from_bytes(G_VARIANT_TYPE_BYTESTRING, (*bytes_icon).bytes, TRUE),
    );
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_icon_iface_init(mut iface: *mut GIconIface) {
    (*iface).hash = Some(safe_c2rust_g_bytes_icon_hash as unsafe extern "C" fn(*mut GIcon) -> guint)
        as Option<unsafe extern "C" fn(*mut GIcon) -> guint>;
    (*iface).equal = Some(
        safe_c2rust_g_bytes_icon_equal as unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean>;
    (*iface).serialize = Some(
        safe_c2rust_g_bytes_icon_serialize as unsafe extern "C" fn(*mut GIcon) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GIcon) -> *mut GVariant>;
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_load(
    mut icon: *mut GLoadableIcon,
    mut size: ::core::ffi::c_int,
    mut type_0: *mut *mut ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GInputStream {
    let mut bytes_icon: *mut GBytesIcon = icon as *mut ::core::ffi::c_void as *mut GBytesIcon;
    if !type_0.is_null() {
        *type_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return g_memory_input_stream_new_from_bytes((*bytes_icon).bytes);
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_load_async(
    mut icon: *mut GLoadableIcon,
    mut size: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut bytes_icon: *mut GBytesIcon = icon as *mut ::core::ffi::c_void as *mut GBytesIcon;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(icon as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GLoadableIcon,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_bytes_icon_load_async
                as unsafe extern "C" fn(
                    *mut GLoadableIcon,
                    ::core::ffi::c_int,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_bytes_icon_load_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_return_pointer(
        task,
        g_memory_input_stream_new_from_bytes((*bytes_icon).bytes) as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_load_finish(
    mut icon: *mut GLoadableIcon,
    mut res: *mut GAsyncResult,
    mut type_0: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GInputStream {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, icon as gpointer) != 0 {
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
            b"g_task_is_valid (res, icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GInputStream>();
    }
    if !type_0.is_null() {
        *type_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GInputStream;
}
unsafe extern "C" fn safe_c2rust_g_bytes_icon_loadable_icon_iface_init(
    mut iface: *mut GLoadableIconIface,
) {
    (*iface).load = Some(
        safe_c2rust_g_bytes_icon_load
            as unsafe extern "C" fn(
                *mut GLoadableIcon,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GInputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GLoadableIcon,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GInputStream,
        >;
    (*iface).load_async = Some(
        safe_c2rust_g_bytes_icon_load_async
            as unsafe extern "C" fn(
                *mut GLoadableIcon,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GLoadableIcon,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).load_finish = Some(
        safe_c2rust_g_bytes_icon_load_finish
            as unsafe extern "C" fn(
                *mut GLoadableIcon,
                *mut GAsyncResult,
                *mut *mut ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GInputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GLoadableIcon,
                *mut GAsyncResult,
                *mut *mut ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GInputStream,
        >;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
