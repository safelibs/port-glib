extern "C" {
    pub type _GData;
    pub type _GSequence;
    pub type _GSequenceNode;
    pub type _GListModel;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_sequence_new(data_destroy: GDestroyNotify) -> *mut GSequence;
    fn g_sequence_free(seq: *mut GSequence);
    fn g_sequence_get_length(seq: *mut GSequence) -> gint;
    fn g_sequence_sort(seq: *mut GSequence, cmp_func: GCompareDataFunc, cmp_data: gpointer);
    fn g_sequence_get_begin_iter(seq: *mut GSequence) -> *mut GSequenceIter;
    fn g_sequence_get_end_iter(seq: *mut GSequence) -> *mut GSequenceIter;
    fn g_sequence_get_iter_at_pos(seq: *mut GSequence, pos: gint) -> *mut GSequenceIter;
    fn g_sequence_append(seq: *mut GSequence, data: gpointer) -> *mut GSequenceIter;
    fn g_sequence_insert_before(iter: *mut GSequenceIter, data: gpointer) -> *mut GSequenceIter;
    fn g_sequence_insert_sorted(
        seq: *mut GSequence,
        data: gpointer,
        cmp_func: GCompareDataFunc,
        cmp_data: gpointer,
    ) -> *mut GSequenceIter;
    fn g_sequence_remove(iter: *mut GSequenceIter);
    fn g_sequence_remove_range(begin: *mut GSequenceIter, end: *mut GSequenceIter);
    fn g_sequence_get(iter: *mut GSequenceIter) -> gpointer;
    fn g_sequence_iter_is_end(iter: *mut GSequenceIter) -> gboolean;
    fn g_sequence_iter_next(iter: *mut GSequenceIter) -> *mut GSequenceIter;
    fn g_sequence_iter_prev(iter: *mut GSequenceIter) -> *mut GSequenceIter;
    fn g_sequence_iter_get_position(iter: *mut GSequenceIter) -> gint;
    fn g_sequence_iter_move(iter: *mut GSequenceIter, delta: gint) -> *mut GSequenceIter;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
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
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify_by_pspec(object: *mut GObject, pspec: *mut GParamSpec);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_gtype(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        is_a_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_set_gtype(value: *mut GValue, v_gtype: GType);
    fn g_value_get_gtype(value: *const GValue) -> GType;
    fn g_list_model_get_type() -> GType;
    fn g_list_model_items_changed(
        list: *mut GListModel,
        position: guint,
        removed: guint,
        added: guint,
    );
}
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
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GEqualFuncFull =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GSequence = _GSequence;
pub type GSequenceIter = _GSequenceNode;
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
pub type GListStore = _GListStore;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GListStore {
    pub parent_instance: GObject,
    pub item_type: GType,
    pub items: *mut GSequence,
    pub last_position: guint,
    pub last_iter: *mut GSequenceIter,
    pub last_position_valid: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GListStoreClass {
    pub parent_class: GObjectClass,
}
pub const N_PROPERTIES: C2RustUnnamed_0 = 3;
pub const PROP_N_ITEMS: C2RustUnnamed_0 = 2;
pub const PROP_ITEM_TYPE: C2RustUnnamed_0 = 1;
pub type GListModelInterface = _GListModelInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GListModelInterface {
    pub g_iface: GTypeInterface,
    pub get_item_type: Option<unsafe extern "C" fn(*mut GListModel) -> GType>,
    pub get_n_items: Option<unsafe extern "C" fn(*mut GListModel) -> guint>,
    pub get_item: Option<unsafe extern "C" fn(*mut GListModel, guint) -> gpointer>,
}
pub type GListModel = _GListModel;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_OBJECT: GType = ((20 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[inline]
unsafe extern "C" fn safe_c2rust_G_LIST_STORE(mut ptr: gpointer) -> *mut GListStore {
    return ptr as *mut GListStore;
}
#[inline]
unsafe extern "C" fn safe_c2rust_G_IS_LIST_STORE(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = safe_c2rust_g_list_store_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    });
}
#[inline]
unsafe extern "C" fn safe_c2rust_G_LIST_MODEL(mut ptr: gpointer) -> *mut GListModel {
    return ptr as *mut GListModel;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_list_store_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_list_store_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GListStore\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GListStoreClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_list_store_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GListStore>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GListStore) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_list_store_init as unsafe extern "C" fn(*mut GListStore) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GListModelInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_list_store_iface_init
                as unsafe extern "C" fn(*mut GListModelInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_list_model_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_list_store_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_list_store_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GListStore_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GListStore_private_offset);
    }
    safe_c2rust_g_list_store_class_init(klass as *mut GListStoreClass);
}
static mut safe_c2rust_g_list_store_parent_class: gpointer = NULL;
static mut safe_c2rust_GListStore_private_offset: gint = 0;
static mut safe_c2rust_properties: [*mut GParamSpec; 3] = [
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
];
unsafe extern "C" fn safe_c2rust_g_list_store_items_changed(
    mut store: *mut GListStore,
    mut position: guint,
    mut removed: guint,
    mut added: guint,
) {
    if position <= (*store).last_position {
        (*store).last_iter = ::core::ptr::null_mut::<GSequenceIter>();
        (*store).last_position = 0 as guint;
        (*store).last_position_valid = FALSE as gboolean;
    }
    g_list_model_items_changed(
        safe_c2rust_G_LIST_MODEL(store as gpointer),
        position,
        removed,
        added,
    );
    if removed != added {
        g_object_notify_by_pspec(
            store as *mut ::core::ffi::c_void as *mut GObject,
            safe_c2rust_properties[PROP_N_ITEMS as ::core::ffi::c_int as usize],
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_list_store_dispose(mut object: *mut GObject) {
    let mut store: *mut GListStore = safe_c2rust_G_LIST_STORE(object as gpointer);
    let mut _pp: *mut *mut GSequence = &raw mut (*store).items;
    let mut _ptr: *mut GSequence = *_pp;
    *_pp = ::core::ptr::null_mut::<GSequence>();
    if !_ptr.is_null() {
        g_sequence_free(_ptr as *mut GSequence);
    }
    (*(safe_c2rust_g_list_store_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_list_store_get_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut store: *mut GListStore = safe_c2rust_G_LIST_STORE(object as gpointer);
    match property_id {
        1 => {
            g_value_set_gtype(value, (*store).item_type);
        }
        2 => {
            g_value_set_uint(value, g_sequence_get_length((*store).items) as guint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gliststore.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_list_store_set_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut store: *mut GListStore = safe_c2rust_G_LIST_STORE(object as gpointer);
    match property_id {
        1 => {
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if g_value_get_gtype(value)
                    == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                    || g_type_is_a(
                        g_value_get_gtype(value),
                        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
                    ) != 0
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
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gliststore.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    131 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_type_is_a (g_value_get_gtype (value), G_TYPE_OBJECT)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*store).item_type = g_value_get_gtype(value);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gliststore.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                136 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_list_store_class_init(mut klass: *mut GListStoreClass) {
    let mut object_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).dispose =
        Some(safe_c2rust_g_list_store_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_list_store_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_list_store_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    safe_c2rust_properties[PROP_ITEM_TYPE as ::core::ffi::c_int as usize] = g_param_spec_gtype(
        b"item-type\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        G_TYPE_OBJECT,
        (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
            | G_PARAM_READWRITE as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
    );
    safe_c2rust_properties[PROP_N_ITEMS as ::core::ffi::c_int as usize] = g_param_spec_uint(
        b"n-items\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        0 as guint,
        G_MAXUINT,
        0 as guint,
        (G_PARAM_READABLE as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
    );
    g_object_class_install_properties(
        object_class,
        N_PROPERTIES as ::core::ffi::c_int as guint,
        &raw mut safe_c2rust_properties as *mut *mut GParamSpec,
    );
}
unsafe extern "C" fn safe_c2rust_g_list_store_get_item_type(mut list: *mut GListModel) -> GType {
    let mut store: *mut GListStore = safe_c2rust_G_LIST_STORE(list as gpointer);
    return (*store).item_type;
}
unsafe extern "C" fn safe_c2rust_g_list_store_get_n_items(mut list: *mut GListModel) -> guint {
    let mut store: *mut GListStore = safe_c2rust_G_LIST_STORE(list as gpointer);
    return g_sequence_get_length((*store).items) as guint;
}
unsafe extern "C" fn safe_c2rust_g_list_store_get_item(
    mut list: *mut GListModel,
    mut position: guint,
) -> gpointer {
    let mut store: *mut GListStore = safe_c2rust_G_LIST_STORE(list as gpointer);
    let mut it: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if (*store).last_position_valid != 0 {
        if position < G_MAXUINT && (*store).last_position == position.wrapping_add(1 as guint) {
            it = g_sequence_iter_prev((*store).last_iter);
        } else if position > 0 as guint
            && (*store).last_position == position.wrapping_sub(1 as guint)
        {
            it = g_sequence_iter_next((*store).last_iter);
        } else if (*store).last_position == position {
            it = (*store).last_iter;
        }
    }
    if it.is_null() {
        it = g_sequence_get_iter_at_pos((*store).items, position as gint);
    }
    (*store).last_iter = it;
    (*store).last_position = position;
    (*store).last_position_valid = TRUE as gboolean;
    if g_sequence_iter_is_end(it) != 0 {
        return NULL;
    } else {
        return g_object_ref(g_sequence_get(it));
    };
}
unsafe extern "C" fn safe_c2rust_g_list_store_iface_init(mut iface: *mut GListModelInterface) {
    (*iface).get_item_type = Some(
        safe_c2rust_g_list_store_get_item_type as unsafe extern "C" fn(*mut GListModel) -> GType,
    ) as Option<unsafe extern "C" fn(*mut GListModel) -> GType>;
    (*iface).get_n_items = Some(
        safe_c2rust_g_list_store_get_n_items as unsafe extern "C" fn(*mut GListModel) -> guint,
    ) as Option<unsafe extern "C" fn(*mut GListModel) -> guint>;
    (*iface).get_item = Some(
        safe_c2rust_g_list_store_get_item
            as unsafe extern "C" fn(*mut GListModel, guint) -> gpointer,
    ) as Option<unsafe extern "C" fn(*mut GListModel, guint) -> gpointer>;
}
unsafe extern "C" fn safe_c2rust_g_list_store_init(mut store: *mut GListStore) {
    (*store).items = g_sequence_new(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    (*store).last_position = 0 as guint;
    (*store).last_position_valid = FALSE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_new(mut item_type: GType) -> *mut GListStore {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if item_type == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            || g_type_is_a(
                item_type,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"g_type_is_a (item_type, G_TYPE_OBJECT)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GListStore>();
    }
    return g_object_new(
        safe_c2rust_g_list_store_get_type(),
        b"item-type\0" as *const u8 as *const gchar,
        item_type,
        NULL,
    ) as *mut GListStore;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_insert(
    mut store: *mut GListStore,
    mut position: guint,
    mut item: gpointer,
) {
    let mut it: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*(*(item as *mut GTypeInstance)).g_class).g_type == (*store).item_type
            || g_type_is_a(
                (*(*(item as *mut GTypeInstance)).g_class).g_type,
                (*store).item_type,
            ) != 0
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
            b"g_type_is_a (G_OBJECT_TYPE (item), store->item_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if position <= g_sequence_get_length((*store).items) as guint {
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
            b"position <= (guint) g_sequence_get_length (store->items)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    it = g_sequence_get_iter_at_pos((*store).items, position as gint);
    g_sequence_insert_before(it, g_object_ref(item));
    safe_c2rust_g_list_store_items_changed(store, position, 0 as guint, 1 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_insert_sorted(
    mut store: *mut GListStore,
    mut item: gpointer,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) -> guint {
    let mut it: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut position: guint = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*(*(item as *mut GTypeInstance)).g_class).g_type == (*store).item_type
            || g_type_is_a(
                (*(*(item as *mut GTypeInstance)).g_class).g_type,
                (*store).item_type,
            ) != 0
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
            b"g_type_is_a (G_OBJECT_TYPE (item), store->item_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if compare_func.is_some() {
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
            b"compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    it = g_sequence_insert_sorted((*store).items, g_object_ref(item), compare_func, user_data);
    position = g_sequence_iter_get_position(it) as guint;
    safe_c2rust_g_list_store_items_changed(store, position, 0 as guint, 1 as guint);
    return position;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_sort(
    mut store: *mut GListStore,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    let mut n_items: gint = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if compare_func.is_some() {
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
            b"compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_sequence_sort((*store).items, compare_func, user_data);
    n_items = g_sequence_get_length((*store).items);
    safe_c2rust_g_list_store_items_changed(store, 0 as guint, n_items as guint, n_items as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_append(
    mut store: *mut GListStore,
    mut item: gpointer,
) {
    let mut n_items: guint = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if (*(*(item as *mut GTypeInstance)).g_class).g_type == (*store).item_type
            || g_type_is_a(
                (*(*(item as *mut GTypeInstance)).g_class).g_type,
                (*store).item_type,
            ) != 0
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
            b"g_type_is_a (G_OBJECT_TYPE (item), store->item_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    n_items = g_sequence_get_length((*store).items) as guint;
    g_sequence_append((*store).items, g_object_ref(item));
    safe_c2rust_g_list_store_items_changed(store, n_items, 0 as guint, 1 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_remove(
    mut store: *mut GListStore,
    mut position: guint,
) {
    let mut it: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    it = g_sequence_get_iter_at_pos((*store).items, position as gint);
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if g_sequence_iter_is_end(it) == 0 {
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
            b"!g_sequence_iter_is_end (it)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_sequence_remove(it);
    safe_c2rust_g_list_store_items_changed(store, position, 1 as guint, 0 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_remove_all(mut store: *mut GListStore) {
    let mut n_items: guint = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    n_items = g_sequence_get_length((*store).items) as guint;
    g_sequence_remove_range(
        g_sequence_get_begin_iter((*store).items),
        g_sequence_get_end_iter((*store).items),
    );
    safe_c2rust_g_list_store_items_changed(store, 0 as guint, n_items, 0 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_splice(
    mut store: *mut GListStore,
    mut position: guint,
    mut n_removals: guint,
    mut additions: *mut gpointer,
    mut n_additions: guint,
) {
    let mut it: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut n_items: guint = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if position.wrapping_add(n_removals) >= position {
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
            b"position + n_removals >= position\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    n_items = g_sequence_get_length((*store).items) as guint;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if position.wrapping_add(n_removals) <= n_items {
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
            b"position + n_removals <= n_items\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    it = g_sequence_get_iter_at_pos((*store).items, position as gint);
    if n_removals != 0 {
        let mut end: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
        end = g_sequence_iter_move(it, n_removals as gint);
        g_sequence_remove_range(it, end);
        it = end;
    }
    if n_additions != 0 {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < n_additions {
            if ({
                let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                if !((*(*(*additions.offset(i as isize) as *mut GTypeInstance)).g_class).g_type
                    == (*store).item_type
                    || g_type_is_a(
                        (*(*(*additions.offset(i as isize) as *mut GTypeInstance)).g_class).g_type,
                        (*store).item_type,
                    ) != 0)
                {
                    _g_boolean_var_28 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_28 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_28
            }) as ::core::ffi::c_long
                != 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"%s: item %d is a %s instead of a %s.  GListStore is now in an undefined state.\0"
                        as *const u8 as *const gchar,
                    b"g_list_store_splice\0" as *const u8 as *const ::core::ffi::c_char,
                    i,
                    g_type_name(
                        (*(*(*additions.offset(i as isize) as *mut GTypeInstance))
                            .g_class)
                            .g_type,
                    ),
                    g_type_name((*store).item_type),
                );
                return;
            }
            g_sequence_insert_before(it, g_object_ref(*additions.offset(i as isize)));
            i = i.wrapping_add(1);
        }
    }
    safe_c2rust_g_list_store_items_changed(store, position, n_removals, n_additions);
}
unsafe extern "C" fn safe_c2rust_simple_equal(
    mut a: gconstpointer,
    mut b: gconstpointer,
    mut equal_func: gpointer,
) -> gboolean {
    return ::core::mem::transmute::<gpointer, GEqualFunc>(equal_func)
        .expect("non-null function pointer")(a, b);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_find_with_equal_func(
    mut store: *mut GListStore,
    mut item: gpointer,
    mut equal_func: GEqualFunc,
    mut position: *mut guint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if equal_func.is_some() {
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
            b"equal_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_g_list_store_find_with_equal_func_full(
        store,
        item,
        Some(
            safe_c2rust_simple_equal
                as unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gboolean,
        ),
        ::core::mem::transmute::<GEqualFunc, gpointer>(equal_func),
        position,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_find_with_equal_func_full(
    mut store: *mut GListStore,
    mut item: gpointer,
    mut equal_func: GEqualFuncFull,
    mut user_data: gpointer,
    mut position: *mut guint,
) -> gboolean {
    let mut iter: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut begin: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut end: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if safe_c2rust_G_IS_LIST_STORE(store as gpointer) != 0 {
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
            b"G_IS_LIST_STORE (store)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if item.is_null()
            || ((*(*(item as *mut GTypeInstance)).g_class).g_type == (*store).item_type
                || g_type_is_a(
                    (*(*(item as *mut GTypeInstance)).g_class).g_type,
                    (*store).item_type,
                ) != 0)
        {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"item == NULL || g_type_is_a (G_OBJECT_TYPE (item), store->item_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if equal_func.is_some() {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"equal_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    begin = g_sequence_get_begin_iter((*store).items);
    end = g_sequence_get_end_iter((*store).items);
    iter = begin;
    while iter != end {
        let mut iter_item: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        iter_item = g_sequence_get(iter);
        if equal_func.expect("non-null function pointer")(
            iter_item as gconstpointer,
            item as gconstpointer,
            user_data,
        ) != 0
        {
            if !position.is_null() {
                *position = g_sequence_iter_get_position(iter) as guint;
            }
            return TRUE;
        }
        iter = g_sequence_iter_next(iter);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_store_find(
    mut store: *mut GListStore,
    mut item: gpointer,
    mut position: *mut guint,
) -> gboolean {
    return safe_c2rust_g_list_store_find_with_equal_func(
        store,
        item,
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        position,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
