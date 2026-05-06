extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GMenuModelPrivate;
    pub type _GIcon;
    pub type _GMenuLinkIterPrivate;
    pub type _GMenuAttributeIterPrivate;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_insert_vals(
        array: *mut GArray,
        index_: guint,
        data: gconstpointer,
        len: guint,
    ) -> *mut GArray;
    fn g_array_set_size(array: *mut GArray, length: guint) -> *mut GArray;
    fn g_array_remove_index(array: *mut GArray, index_: guint) -> *mut GArray;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_ref(hash_table: *mut GHashTable) -> *mut GHashTable;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_va(
        format_string: *const gchar,
        endptr: *mut *const gchar,
        app: *mut ::core::ffi::VaList,
    ) -> *mut GVariant;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_menu_model_get_type() -> GType;
    fn g_menu_model_iterate_item_attributes(
        model: *mut GMenuModel,
        item_index: gint,
    ) -> *mut GMenuAttributeIter;
    fn g_menu_model_iterate_item_links(
        model: *mut GMenuModel,
        item_index: gint,
    ) -> *mut GMenuLinkIter;
    fn g_menu_model_items_changed(
        model: *mut GMenuModel,
        position: gint,
        removed: gint,
        added: gint,
    );
    fn g_menu_attribute_iter_get_next(
        iter: *mut GMenuAttributeIter,
        out_name: *mut *const gchar,
        value: *mut *mut GVariant,
    ) -> gboolean;
    fn g_menu_link_iter_get_next(
        iter: *mut GMenuLinkIter,
        out_link: *mut *const gchar,
        value: *mut *mut GMenuModel,
    ) -> gboolean;
    fn g_action_parse_detailed_name(
        detailed_name: *const gchar,
        action_name: *mut *mut gchar,
        target_value: *mut *mut GVariant,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_icon_get_type() -> GType;
    fn g_icon_serialize(icon: *mut GIcon) -> *mut GVariant;
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
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type va_list = __builtin_va_list;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
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
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type GIcon = _GIcon;
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
pub type GMenuAttributeIter = _GMenuAttributeIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuAttributeIter {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuAttributeIterPrivate,
}
pub type GMenuAttributeIterPrivate = _GMenuAttributeIterPrivate;
pub type GMenuModelClass = _GMenuModelClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuItem {
    pub parent_instance: GObject,
    pub attributes: *mut GHashTable,
    pub links: *mut GHashTable,
    pub cow: gboolean,
}
pub type GMenuItem = _GMenuItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenu {
    pub parent_instance: GMenuModel,
    pub items: *mut GArray,
    pub mutable: gboolean,
}
pub type GMenu = _GMenu;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub attributes: *mut GHashTable,
    pub links: *mut GHashTable,
}
pub type GMenuClass = GMenuModelClass;
pub type GMenuItemClass = GObjectClass;
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
pub const G_MENU_ATTRIBUTE_ACTION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"action\0") };
pub const G_MENU_ATTRIBUTE_TARGET: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"target\0") };
pub const G_MENU_ATTRIBUTE_LABEL: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"label\0") };
pub const G_MENU_ATTRIBUTE_ICON: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"icon\0") };
pub const G_MENU_LINK_SUBMENU: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"submenu\0") };
pub const G_MENU_LINK_SECTION: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"section\0") };
unsafe extern "C" fn safe_c2rust_g_menu_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_menu_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMenu_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GMenu_private_offset);
    }
    safe_c2rust_g_menu_class_init(klass as *mut GMenuClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_menu_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_menu_model_get_type(),
        g_intern_static_string(b"GMenu\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMenuClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMenu>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenu) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_init as unsafe extern "C" fn(*mut GMenu) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_menu_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_menu_get_type_once();
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
static mut safe_c2rust_GMenu_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_menu_item_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_menu_item_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMenuItem_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GMenuItem_private_offset);
    }
    safe_c2rust_g_menu_item_class_init(klass as *mut GMenuItemClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_menu_item_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GMenuItem\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMenuItemClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_item_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMenuItem>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenuItem) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_menu_item_init as unsafe extern "C" fn(*mut GMenuItem) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_menu_item_get_type_once();
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
static mut safe_c2rust_g_menu_item_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GMenuItem_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_menu_is_mutable(mut model: *mut GMenuModel) -> gboolean {
    let mut menu: *mut GMenu = model as *mut ::core::ffi::c_void as *mut GMenu;
    return (*menu).mutable;
}
unsafe extern "C" fn safe_c2rust_g_menu_get_n_items(mut model: *mut GMenuModel) -> gint {
    let mut menu: *mut GMenu = model as *mut ::core::ffi::c_void as *mut GMenu;
    return (*(*menu).items).len as gint;
}
unsafe extern "C" fn safe_c2rust_g_menu_get_item_attributes(
    mut model: *mut GMenuModel,
    mut position: gint,
    mut table: *mut *mut GHashTable,
) {
    let mut menu: *mut GMenu = model as *mut ::core::ffi::c_void as *mut GMenu;
    *table = g_hash_table_ref(
        (*((*(*menu).items).data as *mut ::core::ffi::c_void as *mut item)
            .offset(position as isize))
        .attributes,
    );
}
unsafe extern "C" fn safe_c2rust_g_menu_get_item_links(
    mut model: *mut GMenuModel,
    mut position: gint,
    mut table: *mut *mut GHashTable,
) {
    let mut menu: *mut GMenu = model as *mut ::core::ffi::c_void as *mut GMenu;
    *table = g_hash_table_ref(
        (*((*(*menu).items).data as *mut ::core::ffi::c_void as *mut item)
            .offset(position as isize))
        .links,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_insert_item(
    mut menu: *mut GMenu,
    mut position: gint,
    mut item: *mut GMenuItem,
) {
    let mut new_item: item = item {
        attributes: ::core::ptr::null_mut::<GHashTable>(),
        links: ::core::ptr::null_mut::<GHashTable>(),
    };
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_get_type();
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
            b"G_IS_MENU (menu)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = item as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_item_get_type();
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
            b"G_IS_MENU_ITEM (item)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if position < 0 as ::core::ffi::c_int || position as guint > (*(*menu).items).len {
        position = (*(*menu).items).len as gint;
    }
    new_item.attributes = g_hash_table_ref((*item).attributes);
    new_item.links = g_hash_table_ref((*item).links);
    (*item).cow = TRUE as gboolean;
    g_array_insert_vals(
        (*menu).items,
        position as guint,
        &raw mut new_item as gconstpointer,
        1 as guint,
    );
    g_menu_model_items_changed(
        menu as *mut ::core::ffi::c_void as *mut GMenuModel,
        position,
        0 as gint,
        1 as gint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_prepend_item(
    mut menu: *mut GMenu,
    mut item: *mut GMenuItem,
) {
    safe_c2rust_g_menu_insert_item(menu, 0 as gint, item);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_append_item(
    mut menu: *mut GMenu,
    mut item: *mut GMenuItem,
) {
    safe_c2rust_g_menu_insert_item(menu, -(1 as gint), item);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_freeze(mut menu: *mut GMenu) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_get_type();
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
            b"G_IS_MENU (menu)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*menu).mutable = FALSE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_new() -> *mut GMenu {
    return g_object_new(safe_c2rust_g_menu_get_type(), ::core::ptr::null::<gchar>()) as *mut GMenu;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_insert(
    mut menu: *mut GMenu,
    mut position: gint,
    mut label: *const gchar,
    mut detailed_action: *const gchar,
) {
    let mut menu_item: *mut GMenuItem = ::core::ptr::null_mut::<GMenuItem>();
    menu_item = safe_c2rust_g_menu_item_new(label, detailed_action);
    safe_c2rust_g_menu_insert_item(menu, position, menu_item);
    g_object_unref(menu_item as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_prepend(
    mut menu: *mut GMenu,
    mut label: *const gchar,
    mut detailed_action: *const gchar,
) {
    safe_c2rust_g_menu_insert(menu, 0 as gint, label, detailed_action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_append(
    mut menu: *mut GMenu,
    mut label: *const gchar,
    mut detailed_action: *const gchar,
) {
    safe_c2rust_g_menu_insert(menu, -(1 as gint), label, detailed_action);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_insert_section(
    mut menu: *mut GMenu,
    mut position: gint,
    mut label: *const gchar,
    mut section: *mut GMenuModel,
) {
    let mut menu_item: *mut GMenuItem = ::core::ptr::null_mut::<GMenuItem>();
    menu_item = safe_c2rust_g_menu_item_new_section(label, section);
    safe_c2rust_g_menu_insert_item(menu, position, menu_item);
    g_object_unref(menu_item as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_prepend_section(
    mut menu: *mut GMenu,
    mut label: *const gchar,
    mut section: *mut GMenuModel,
) {
    safe_c2rust_g_menu_insert_section(menu, 0 as gint, label, section);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_append_section(
    mut menu: *mut GMenu,
    mut label: *const gchar,
    mut section: *mut GMenuModel,
) {
    safe_c2rust_g_menu_insert_section(menu, -(1 as gint), label, section);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_insert_submenu(
    mut menu: *mut GMenu,
    mut position: gint,
    mut label: *const gchar,
    mut submenu: *mut GMenuModel,
) {
    let mut menu_item: *mut GMenuItem = ::core::ptr::null_mut::<GMenuItem>();
    menu_item = safe_c2rust_g_menu_item_new_submenu(label, submenu);
    safe_c2rust_g_menu_insert_item(menu, position, menu_item);
    g_object_unref(menu_item as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_prepend_submenu(
    mut menu: *mut GMenu,
    mut label: *const gchar,
    mut submenu: *mut GMenuModel,
) {
    safe_c2rust_g_menu_insert_submenu(menu, 0 as gint, label, submenu);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_append_submenu(
    mut menu: *mut GMenu,
    mut label: *const gchar,
    mut submenu: *mut GMenuModel,
) {
    safe_c2rust_g_menu_insert_submenu(menu, -(1 as gint), label, submenu);
}
unsafe extern "C" fn safe_c2rust_g_menu_clear_item(mut item: *mut item) {
    if !(*item).attributes.is_null() {
        g_hash_table_unref((*item).attributes);
    }
    if !(*item).links.is_null() {
        g_hash_table_unref((*item).links);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_remove(mut menu: *mut GMenu, mut position: gint) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_get_type();
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
            b"G_IS_MENU (menu)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if 0 as ::core::ffi::c_int <= position && (position as guint) < (*(*menu).items).len {
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
            b"0 <= position && (guint) position < menu->items->len\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_menu_clear_item(
        ((*(*menu).items).data as *mut ::core::ffi::c_void as *mut item).offset(position as isize)
            as *mut item,
    );
    g_array_remove_index((*menu).items, position as guint);
    g_menu_model_items_changed(
        menu as *mut ::core::ffi::c_void as *mut GMenuModel,
        position,
        1 as gint,
        0 as gint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_remove_all(mut menu: *mut GMenu) {
    let mut i: gint = 0;
    let mut n: gint = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_get_type();
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
            b"G_IS_MENU (menu)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    n = (*(*menu).items).len as gint;
    i = 0 as ::core::ffi::c_int as gint;
    while i < n {
        safe_c2rust_g_menu_clear_item(
            ((*(*menu).items).data as *mut ::core::ffi::c_void as *mut item).offset(i as isize)
                as *mut item,
        );
        i += 1;
    }
    g_array_set_size((*menu).items, 0 as guint);
    g_menu_model_items_changed(
        menu as *mut ::core::ffi::c_void as *mut GMenuModel,
        0 as gint,
        n,
        0 as gint,
    );
}
unsafe extern "C" fn safe_c2rust_g_menu_finalize(mut object: *mut GObject) {
    let mut menu: *mut GMenu = object as *mut ::core::ffi::c_void as *mut GMenu;
    let mut items: *mut item = ::core::ptr::null_mut::<item>();
    let mut n_items: gint = 0;
    let mut i: gint = 0;
    n_items = (*(*menu).items).len as gint;
    items = g_array_free((*menu).items, FALSE) as *mut item;
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_items {
        safe_c2rust_g_menu_clear_item(items.offset(i as isize) as *mut item);
        i += 1;
    }
    g_free(items as gpointer);
    (*(safe_c2rust_g_menu_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_menu_init(mut menu: *mut GMenu) {
    (*menu).items = g_array_new(FALSE, FALSE, ::core::mem::size_of::<item>() as guint);
    (*menu).mutable = TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_menu_class_init(mut class: *mut GMenuClass) {
    let mut model_class: *mut GMenuModelClass =
        class as *mut ::core::ffi::c_void as *mut GMenuModelClass;
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_menu_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*model_class).is_mutable =
        Some(safe_c2rust_g_menu_is_mutable as unsafe extern "C" fn(*mut GMenuModel) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GMenuModel) -> gboolean>;
    (*model_class).get_n_items =
        Some(safe_c2rust_g_menu_get_n_items as unsafe extern "C" fn(*mut GMenuModel) -> gint)
            as Option<unsafe extern "C" fn(*mut GMenuModel) -> gint>;
    (*model_class).get_item_attributes = Some(
        safe_c2rust_g_menu_get_item_attributes
            as unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>;
    (*model_class).get_item_links = Some(
        safe_c2rust_g_menu_get_item_links
            as unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_menu_item_clear_cow(mut menu_item: *mut GMenuItem) {
    if (*menu_item).cow != 0 {
        let mut iter: GHashTableIter = _GHashTableIter {
            dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy4: 0,
            dummy5: 0,
            dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        let mut new: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut val: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        new = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVariant) -> ()>,
                GDestroyNotify,
            >(Some(
                g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> (),
            )),
        );
        g_hash_table_iter_init(&raw mut iter, (*menu_item).attributes);
        while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut val) != 0 {
            g_hash_table_insert(
                new,
                safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
                g_variant_ref(val as *mut GVariant) as gpointer,
            );
        }
        g_hash_table_unref((*menu_item).attributes);
        (*menu_item).attributes = new;
        new = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
        g_hash_table_iter_init(&raw mut iter, (*menu_item).links);
        while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut val) != 0 {
            g_hash_table_insert(
                new,
                safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
                g_object_ref(val),
            );
        }
        g_hash_table_unref((*menu_item).links);
        (*menu_item).links = new;
        (*menu_item).cow = FALSE as gboolean;
    }
}
unsafe extern "C" fn safe_c2rust_g_menu_item_finalize(mut object: *mut GObject) {
    let mut menu_item: *mut GMenuItem = object as *mut ::core::ffi::c_void as *mut GMenuItem;
    g_hash_table_unref((*menu_item).attributes);
    g_hash_table_unref((*menu_item).links);
    (*(safe_c2rust_g_menu_item_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_menu_item_init(mut menu_item: *mut GMenuItem) {
    (*menu_item).attributes = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
    (*menu_item).links = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*menu_item).cow = FALSE as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_menu_item_class_init(mut class: *mut GMenuItemClass) {
    (*class).finalize =
        Some(safe_c2rust_g_menu_item_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_valid_attribute_name(mut name: *const gchar) -> gboolean {
    let mut i: gint = 0;
    if !(*safe_c2rust_g_ascii_table
        .offset(*name.offset(0 as ::core::ffi::c_int as isize) as guchar as isize)
        as ::core::ffi::c_int
        & G_ASCII_LOWER as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int)
    {
        return FALSE;
    }
    i = 1 as ::core::ffi::c_int as gint;
    while *name.offset(i as isize) != 0 {
        if *name.offset(i as isize) as ::core::ffi::c_int != '-' as i32
            && !(*safe_c2rust_g_ascii_table.offset(*name.offset(i as isize) as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_LOWER as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
            && !(*safe_c2rust_g_ascii_table.offset(*name.offset(i as isize) as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_DIGIT as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
        {
            return FALSE;
        }
        if *name.offset(i as isize) as ::core::ffi::c_int == '-' as i32
            && *name.offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == '-' as i32
        {
            return FALSE;
        }
        i += 1;
    }
    if *name.offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        == '-' as i32
    {
        return FALSE;
    }
    if i > 1024 as ::core::ffi::c_int {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_attribute_value(
    mut menu_item: *mut GMenuItem,
    mut attribute: *const gchar,
    mut value: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu_item as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_item_get_type();
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
            b"G_IS_MENU_ITEM (menu_item)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !attribute.is_null() {
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
            b"attribute != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_attribute_name(attribute) != 0 {
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
            b"valid_attribute_name (attribute)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_menu_item_clear_cow(menu_item);
    if !value.is_null() {
        g_hash_table_insert(
            (*menu_item).attributes,
            safe_c2rust_g_strdup_inline(attribute as *const ::core::ffi::c_char) as gpointer,
            g_variant_ref_sink(value) as gpointer,
        );
    } else {
        g_hash_table_remove((*menu_item).attributes, attribute as gconstpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_attribute(
    mut menu_item: *mut GMenuItem,
    mut attribute: *const gchar,
    mut format_string: *const gchar,
    mut args: ...
) {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !format_string.is_null() {
        let mut ap: ::core::ffi::VaList;
        ap = args.clone();
        value = g_variant_new_va(
            format_string,
            ::core::ptr::null_mut::<*const gchar>(),
            &raw mut ap,
        );
    } else {
        value = ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_menu_item_set_attribute_value(menu_item, attribute, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_link(
    mut menu_item: *mut GMenuItem,
    mut link: *const gchar,
    mut model: *mut GMenuModel,
) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu_item as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_item_get_type();
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
            b"G_IS_MENU_ITEM (menu_item)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !link.is_null() {
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
            b"link != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_attribute_name(link) != 0 {
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
            b"valid_attribute_name (link)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_menu_item_clear_cow(menu_item);
    if !model.is_null() {
        g_hash_table_insert(
            (*menu_item).links,
            safe_c2rust_g_strdup_inline(link as *const ::core::ffi::c_char) as gpointer,
            g_object_ref(model as gpointer) as *mut GMenuModel as gpointer,
        );
    } else {
        g_hash_table_remove((*menu_item).links, link as gconstpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_get_attribute_value(
    mut menu_item: *mut GMenuItem,
    mut attribute: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu_item as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_item_get_type();
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
            b"G_IS_MENU_ITEM (menu_item)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !attribute.is_null() {
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
            b"attribute != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    value =
        g_hash_table_lookup((*menu_item).attributes, attribute as gconstpointer) as *mut GVariant;
    if !value.is_null() {
        if expected_type.is_null() || g_variant_is_of_type(value, expected_type) != 0 {
            g_variant_ref(value);
        } else {
            value = ::core::ptr::null_mut::<GVariant>();
        }
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_get_attribute(
    mut menu_item: *mut GMenuItem,
    mut attribute: *const gchar,
    mut format_string: *const gchar,
    mut args: ...
) -> gboolean {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu_item as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_item_get_type();
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
            b"G_IS_MENU_ITEM (menu_item)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !attribute.is_null() {
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
            b"attribute != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !format_string.is_null() {
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
            b"format_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    value =
        g_hash_table_lookup((*menu_item).attributes, attribute as gconstpointer) as *mut GVariant;
    if value.is_null() {
        return FALSE;
    }
    if g_variant_check_format_string(value, format_string, FALSE) == 0 {
        return FALSE;
    }
    ap = args.clone();
    g_variant_get_va(
        value,
        format_string,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_get_link(
    mut menu_item: *mut GMenuItem,
    mut link: *const gchar,
) -> *mut GMenuModel {
    let mut model: *mut GMenuModel = ::core::ptr::null_mut::<GMenuModel>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu_item as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_item_get_type();
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
            b"G_IS_MENU_ITEM (menu_item)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMenuModel>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !link.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMenuModel>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_attribute_name(link) != 0 {
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
            b"valid_attribute_name (link)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMenuModel>();
    }
    model = g_hash_table_lookup((*menu_item).links, link as gconstpointer) as *mut GMenuModel;
    if !model.is_null() {
        g_object_ref(model as gpointer);
    }
    return model;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_label(
    mut menu_item: *mut GMenuItem,
    mut label: *const gchar,
) {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !label.is_null() {
        value = g_variant_new_string(label);
    } else {
        value = ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_menu_item_set_attribute_value(
        menu_item,
        G_MENU_ATTRIBUTE_LABEL.as_ptr() as *const gchar,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_submenu(
    mut menu_item: *mut GMenuItem,
    mut submenu: *mut GMenuModel,
) {
    safe_c2rust_g_menu_item_set_link(
        menu_item,
        G_MENU_LINK_SUBMENU.as_ptr() as *const gchar,
        submenu,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_section(
    mut menu_item: *mut GMenuItem,
    mut section: *mut GMenuModel,
) {
    safe_c2rust_g_menu_item_set_link(
        menu_item,
        G_MENU_LINK_SECTION.as_ptr() as *const gchar,
        section,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_action_and_target_value(
    mut menu_item: *mut GMenuItem,
    mut action: *const gchar,
    mut target_value: *mut GVariant,
) {
    let mut action_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !action.is_null() {
        action_value = g_variant_new_string(action);
    } else {
        action_value = ::core::ptr::null_mut::<GVariant>();
        target_value = ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_menu_item_set_attribute_value(
        menu_item,
        G_MENU_ATTRIBUTE_ACTION.as_ptr() as *const gchar,
        action_value,
    );
    safe_c2rust_g_menu_item_set_attribute_value(
        menu_item,
        G_MENU_ATTRIBUTE_TARGET.as_ptr() as *const gchar,
        target_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_action_and_target(
    mut menu_item: *mut GMenuItem,
    mut action: *const gchar,
    mut format_string: *const gchar,
    mut args: ...
) {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !format_string.is_null() {
        let mut ap: ::core::ffi::VaList;
        ap = args.clone();
        value = g_variant_new_va(
            format_string,
            ::core::ptr::null_mut::<*const gchar>(),
            &raw mut ap,
        );
    } else {
        value = ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_menu_item_set_action_and_target_value(menu_item, action, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_detailed_action(
    mut menu_item: *mut GMenuItem,
    mut detailed_action: *const gchar,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut target: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if g_action_parse_detailed_name(
        detailed_action,
        &raw mut name,
        &raw mut target,
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_menu_item_set_detailed_action: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        loop {}
    }
    safe_c2rust_g_menu_item_set_action_and_target_value(menu_item, name, target);
    if !target.is_null() {
        g_variant_unref(target);
    }
    g_free(name as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_new(
    mut label: *const gchar,
    mut detailed_action: *const gchar,
) -> *mut GMenuItem {
    let mut menu_item: *mut GMenuItem = ::core::ptr::null_mut::<GMenuItem>();
    menu_item = g_object_new(
        safe_c2rust_g_menu_item_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GMenuItem;
    if !label.is_null() {
        safe_c2rust_g_menu_item_set_label(menu_item, label);
    }
    if !detailed_action.is_null() {
        safe_c2rust_g_menu_item_set_detailed_action(menu_item, detailed_action);
    }
    return menu_item;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_new_submenu(
    mut label: *const gchar,
    mut submenu: *mut GMenuModel,
) -> *mut GMenuItem {
    let mut menu_item: *mut GMenuItem = ::core::ptr::null_mut::<GMenuItem>();
    menu_item = g_object_new(
        safe_c2rust_g_menu_item_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GMenuItem;
    if !label.is_null() {
        safe_c2rust_g_menu_item_set_label(menu_item, label);
    }
    safe_c2rust_g_menu_item_set_submenu(menu_item, submenu);
    return menu_item;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_new_section(
    mut label: *const gchar,
    mut section: *mut GMenuModel,
) -> *mut GMenuItem {
    let mut menu_item: *mut GMenuItem = ::core::ptr::null_mut::<GMenuItem>();
    menu_item = g_object_new(
        safe_c2rust_g_menu_item_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GMenuItem;
    if !label.is_null() {
        safe_c2rust_g_menu_item_set_label(menu_item, label);
    }
    safe_c2rust_g_menu_item_set_section(menu_item, section);
    return menu_item;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_new_from_model(
    mut model: *mut GMenuModel,
    mut item_index: gint,
) -> *mut GMenuItem {
    let mut class: *mut GMenuModelClass =
        (*(model as *mut GTypeInstance)).g_class as *mut GMenuModelClass;
    let mut menu_item: *mut GMenuItem = ::core::ptr::null_mut::<GMenuItem>();
    menu_item = g_object_new(
        safe_c2rust_g_menu_item_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GMenuItem;
    if (*class).get_item_attributes.is_some() {
        let mut attributes: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        (*class)
            .get_item_attributes
            .expect("non-null function pointer")(model, item_index, &raw mut attributes);
        if !attributes.is_null() {
            g_hash_table_unref((*menu_item).attributes);
            (*menu_item).attributes = attributes;
            (*menu_item).cow = TRUE as gboolean;
        }
    } else {
        let mut iter: *mut GMenuAttributeIter = ::core::ptr::null_mut::<GMenuAttributeIter>();
        let mut attribute: *const gchar = ::core::ptr::null::<gchar>();
        let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        iter = g_menu_model_iterate_item_attributes(model, item_index);
        while g_menu_attribute_iter_get_next(iter, &raw mut attribute, &raw mut value) != 0 {
            g_hash_table_insert(
                (*menu_item).attributes,
                safe_c2rust_g_strdup_inline(attribute as *const ::core::ffi::c_char) as gpointer,
                value as gpointer,
            );
        }
        g_object_unref(iter as gpointer);
    }
    if (*class).get_item_links.is_some() {
        let mut links: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        (*class).get_item_links.expect("non-null function pointer")(
            model,
            item_index,
            &raw mut links,
        );
        if !links.is_null() {
            g_hash_table_unref((*menu_item).links);
            (*menu_item).links = links;
            (*menu_item).cow = TRUE as gboolean;
        }
    } else {
        let mut iter_0: *mut GMenuLinkIter = ::core::ptr::null_mut::<GMenuLinkIter>();
        let mut link: *const gchar = ::core::ptr::null::<gchar>();
        let mut value_0: *mut GMenuModel = ::core::ptr::null_mut::<GMenuModel>();
        iter_0 = g_menu_model_iterate_item_links(model, item_index);
        while g_menu_link_iter_get_next(iter_0, &raw mut link, &raw mut value_0) != 0 {
            g_hash_table_insert(
                (*menu_item).links,
                safe_c2rust_g_strdup_inline(link as *const ::core::ffi::c_char) as gpointer,
                value_0 as gpointer,
            );
        }
        g_object_unref(iter_0 as gpointer);
    }
    return menu_item;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_menu_item_set_icon(
    mut menu_item: *mut GMenuItem,
    mut icon: *mut GIcon,
) {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = menu_item as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_menu_item_get_type();
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
            b"G_IS_MENU_ITEM (menu_item)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if icon.is_null()
            || ({
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
            b"icon == NULL || G_IS_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !icon.is_null() {
        value = g_icon_serialize(icon);
    } else {
        value = ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_menu_item_set_attribute_value(
        menu_item,
        G_MENU_ATTRIBUTE_ICON.as_ptr() as *const gchar,
        value,
    );
    if !value.is_null() {
        g_variant_unref(value);
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
