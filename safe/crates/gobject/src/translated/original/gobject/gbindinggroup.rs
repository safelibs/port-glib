// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GBinding;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_steal(array: *mut GPtrArray, len: *mut gsize) -> *mut gpointer;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_steal_index_fast(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc0(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_closure_ref(closure: *mut GClosure) -> *mut GClosure;
    fn g_closure_sink(closure: *mut GClosure);
    fn g_closure_unref(closure: *mut GClosure);
    fn g_object_class_find_property(
        oclass: *mut GObjectClass,
        property_name: *const gchar,
    ) -> *mut GParamSpec;
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify_by_pspec(object: *mut GObject, pspec: *mut GParamSpec);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_weak_ref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_object_weak_unref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_value_take_object(value: *mut GValue, v_object: gpointer);
    fn g_binding_unbind(binding: *mut GBinding);
    fn g_object_bind_property_full(
        source: gpointer,
        source_property: *const gchar,
        target: gpointer,
        target_property: *const gchar,
        flags: GBindingFlags,
        transform_to: GBindingTransformFunc,
        transform_from: GBindingTransformFunc,
        user_data: gpointer,
        notify: GDestroyNotify,
    ) -> *mut GBinding;
    fn g_object_bind_property_with_closures(
        source: gpointer,
        source_property: *const gchar,
        target: gpointer,
        target_property: *const gchar,
        flags: GBindingFlags,
        transform_to: *mut GClosure,
        transform_from: *mut GClosure,
    ) -> *mut GBinding;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
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
pub type GWeakNotify = Option<unsafe extern "C" fn(gpointer, *mut GObject) -> ()>;
pub type GBinding = _GBinding;
pub type GBindingTransformFunc =
    Option<unsafe extern "C" fn(*mut GBinding, *const GValue, *mut GValue, gpointer) -> gboolean>;
pub type GBindingFlags = ::core::ffi::c_uint;
pub const G_BINDING_INVERT_BOOLEAN: GBindingFlags = 4;
pub const G_BINDING_SYNC_CREATE: GBindingFlags = 2;
pub const G_BINDING_BIDIRECTIONAL: GBindingFlags = 1;
pub const G_BINDING_DEFAULT: GBindingFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBindingGroup {
    pub parent_instance: GObject,
    pub mutex: GMutex,
    pub source: *mut GObject,
    pub lazy_bindings: *mut GPtrArray,
}
pub type GBindingGroup = _GBindingGroup;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct LazyBinding {
    pub group: *mut GBindingGroup,
    pub source_property: *const ::core::ffi::c_char,
    pub target_property: *const ::core::ffi::c_char,
    pub target: *mut GObject,
    pub binding: *mut GBinding,
    pub user_data: gpointer,
    pub user_data_destroy: GDestroyNotify,
    pub transform_to: gpointer,
    pub transform_from: gpointer,
    pub binding_flags: GBindingFlags,
    #[bitfield(name = "using_closures", ty = "guint", bits = "0..=0")]
    pub using_closures: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GBindingGroupClass = _GBindingGroupClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBindingGroupClass {
    pub parent_class: GObjectClass,
}
pub const N_PROPS: GBindingGroupProperty = 2;
pub const PROP_SOURCE: GBindingGroupProperty = 1;
pub type GBindingGroupProperty = ::core::ffi::c_uint;
static mut GBindingGroup_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn g_binding_group_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_binding_group_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
unsafe extern "C" fn g_binding_group_class_intern_init(mut klass: gpointer) {
    g_binding_group_parent_class = g_type_class_peek_parent(klass);
    if GBindingGroup_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut GBindingGroup_private_offset);
    }
    g_binding_group_class_init(klass as *mut GBindingGroupClass);
}
#[inline(never)]
unsafe extern "C" fn g_binding_group_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GBindingGroup\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GBindingGroupClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_binding_group_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GBindingGroup>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GBindingGroup) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_binding_group_init as unsafe extern "C" fn(*mut GBindingGroup) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut g_binding_group_parent_class: gpointer =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
static mut properties: [*mut GParamSpec; 2] =
    [::core::ptr::null::<GParamSpec>() as *mut GParamSpec; 2];
unsafe extern "C" fn g_binding_group_connect(
    mut self_0: *mut GBindingGroup,
    mut lazy_binding: *mut LazyBinding,
) {
    let mut binding: *mut GBinding = ::core::ptr::null_mut::<GBinding>();
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            93 as ::core::ffi::c_int,
            b"g_binding_group_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*self_0).source.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            94 as ::core::ffi::c_int,
            b"g_binding_group_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"self->source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !lazy_binding.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            95 as ::core::ffi::c_int,
            b"g_binding_group_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"lazy_binding != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*lazy_binding).binding.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            96 as ::core::ffi::c_int,
            b"g_binding_group_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"lazy_binding->binding == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*lazy_binding).target.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            97 as ::core::ffi::c_int,
            b"g_binding_group_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"lazy_binding->target != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*lazy_binding).target_property.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            98 as ::core::ffi::c_int,
            b"g_binding_group_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"lazy_binding->target_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*lazy_binding).source_property.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            99 as ::core::ffi::c_int,
            b"g_binding_group_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"lazy_binding->source_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*lazy_binding).using_closures() == 0 {
        binding = g_object_bind_property_full(
            (*self_0).source as gpointer,
            (*lazy_binding).source_property as *const gchar,
            (*lazy_binding).target as gpointer,
            (*lazy_binding).target_property as *const gchar,
            (*lazy_binding).binding_flags,
            ::core::mem::transmute::<gpointer, GBindingTransformFunc>((*lazy_binding).transform_to),
            ::core::mem::transmute::<gpointer, GBindingTransformFunc>(
                (*lazy_binding).transform_from,
            ),
            (*lazy_binding).user_data,
            None,
        );
    } else {
        binding = g_object_bind_property_with_closures(
            (*self_0).source as gpointer,
            (*lazy_binding).source_property as *const gchar,
            (*lazy_binding).target as gpointer,
            (*lazy_binding).target_property as *const gchar,
            (*lazy_binding).binding_flags,
            (*lazy_binding).transform_to as *mut GClosure,
            (*lazy_binding).transform_from as *mut GClosure,
        );
    }
    (*lazy_binding).binding = binding;
}
unsafe extern "C" fn g_binding_group_disconnect(mut lazy_binding: *mut LazyBinding) {
    if !lazy_binding.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            147 as ::core::ffi::c_int,
            b"g_binding_group_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
            b"lazy_binding != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*lazy_binding).binding.is_null() {
        g_binding_unbind((*lazy_binding).binding);
        (*lazy_binding).binding = ::core::ptr::null_mut::<GBinding>();
    }
}
unsafe extern "C" fn g_binding_group__source_weak_notify(
    mut data: gpointer,
    mut where_object_was: *mut GObject,
) {
    let mut self_0: *mut GBindingGroup = data as *mut GBindingGroup;
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            163 as ::core::ffi::c_int,
            b"g_binding_group__source_weak_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*self_0).mutex);
    (*self_0).source = ::core::ptr::null_mut::<GObject>();
    i = 0 as guint;
    while i < (*(*self_0).lazy_bindings).len {
        let mut lazy_binding: *mut LazyBinding =
            *(*(*self_0).lazy_bindings).pdata.offset(i as isize) as *mut LazyBinding;
        (*lazy_binding).binding = ::core::ptr::null_mut::<GBinding>();
        i = i.wrapping_add(1);
    }
    g_mutex_unlock(&raw mut (*self_0).mutex);
}
unsafe extern "C" fn g_binding_group__target_weak_notify(
    mut data: gpointer,
    mut where_object_was: *mut GObject,
) {
    let mut self_0: *mut GBindingGroup = data as *mut GBindingGroup;
    let mut to_free: *mut LazyBinding = ::core::ptr::null_mut::<LazyBinding>();
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            187 as ::core::ffi::c_int,
            b"g_binding_group__target_weak_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*self_0).mutex);
    i = 0 as guint;
    while i < (*(*self_0).lazy_bindings).len {
        let mut lazy_binding: *mut LazyBinding =
            *(*(*self_0).lazy_bindings).pdata.offset(i as isize) as *mut LazyBinding;
        if (*lazy_binding).target == where_object_was {
            (*lazy_binding).target = ::core::ptr::null_mut::<GObject>();
            (*lazy_binding).binding = ::core::ptr::null_mut::<GBinding>();
            to_free = g_ptr_array_steal_index_fast((*self_0).lazy_bindings, i) as *mut LazyBinding;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    g_mutex_unlock(&raw mut (*self_0).mutex);
    if !to_free.is_null() {
        lazy_binding_free(to_free as gpointer);
    }
}
unsafe extern "C" fn lazy_binding_free(mut data: gpointer) {
    let mut lazy_binding: *mut LazyBinding = data as *mut LazyBinding;
    if !(*lazy_binding).target.is_null() {
        g_object_weak_unref(
            (*lazy_binding).target,
            Some(
                g_binding_group__target_weak_notify
                    as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
            ),
            (*lazy_binding).group as gpointer,
        );
        (*lazy_binding).target = ::core::ptr::null_mut::<GObject>();
    }
    g_binding_group_disconnect(lazy_binding);
    (*lazy_binding).group = ::core::ptr::null_mut::<GBindingGroup>();
    (*lazy_binding).source_property = ::core::ptr::null::<::core::ffi::c_char>();
    (*lazy_binding).target_property = ::core::ptr::null::<::core::ffi::c_char>();
    if (*lazy_binding).user_data_destroy.is_some() {
        (*lazy_binding)
            .user_data_destroy
            .expect("non-null function pointer")((*lazy_binding).user_data);
    }
    if (*lazy_binding).using_closures() != 0 {
        let mut _pp: *mut gpointer = &raw mut (*lazy_binding).transform_to;
        let mut _ptr: gpointer = *_pp;
        *_pp = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        if !_ptr.is_null() {
            g_closure_unref(_ptr as *mut GClosure);
        }
        let mut _pp_0: *mut gpointer = &raw mut (*lazy_binding).transform_from;
        let mut _ptr_0: gpointer = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        if !_ptr_0.is_null() {
            g_closure_unref(_ptr_0 as *mut GClosure);
        }
    }
    g_slice_free1(
        ::core::mem::size_of::<LazyBinding>() as gsize,
        lazy_binding as gpointer,
    );
}
unsafe extern "C" fn g_binding_group_dispose(mut object: *mut GObject) {
    let mut self_0: *mut GBindingGroup = object as *mut GBindingGroup;
    let mut lazy_bindings: *mut *mut LazyBinding = ::core::ptr::null_mut::<*mut LazyBinding>();
    let mut len: gsize = 0 as gsize;
    let mut i: gsize = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            250 as ::core::ffi::c_int,
            b"g_binding_group_dispose\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*self_0).mutex);
    if !(*self_0).source.is_null() {
        g_object_weak_unref(
            (*self_0).source,
            Some(
                g_binding_group__source_weak_notify
                    as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
            ),
            self_0 as gpointer,
        );
        (*self_0).source = ::core::ptr::null_mut::<GObject>();
    }
    if (*(*self_0).lazy_bindings).len > 0 as guint {
        lazy_bindings =
            g_ptr_array_steal((*self_0).lazy_bindings, &raw mut len) as *mut *mut LazyBinding;
    }
    g_mutex_unlock(&raw mut (*self_0).mutex);
    i = 0 as gsize;
    while i < len {
        lazy_binding_free(*lazy_bindings.offset(i as isize) as gpointer);
        i = i.wrapping_add(1);
    }
    g_free(lazy_bindings as gpointer);
    (*(g_binding_group_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn g_binding_group_finalize(mut object: *mut GObject) {
    let mut self_0: *mut GBindingGroup = object as *mut GBindingGroup;
    if !(*self_0).lazy_bindings.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            283 as ::core::ffi::c_int,
            b"g_binding_group_finalize\0" as *const u8 as *const ::core::ffi::c_char,
            b"self->lazy_bindings != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*(*self_0).lazy_bindings).len == 0 as guint {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            284 as ::core::ffi::c_int,
            b"g_binding_group_finalize\0" as *const u8 as *const ::core::ffi::c_char,
            b"self->lazy_bindings->len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _pp: *mut *mut GPtrArray = &raw mut (*self_0).lazy_bindings;
    let mut _ptr: *mut GPtrArray = *_pp;
    *_pp = ::core::ptr::null_mut::<GPtrArray>();
    if !_ptr.is_null() {
        g_ptr_array_unref(_ptr as *mut GPtrArray);
    }
    g_mutex_clear(&raw mut (*self_0).mutex);
    (*(g_binding_group_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn g_binding_group_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GBindingGroup = object as *mut ::core::ffi::c_void as *mut GBindingGroup;
    match prop_id as GBindingGroupProperty as ::core::ffi::c_uint {
        1 => {
            g_value_take_object(value, g_binding_group_dup_source(self_0));
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                307 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn g_binding_group_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GBindingGroup = object as *mut ::core::ffi::c_void as *mut GBindingGroup;
    match prop_id as GBindingGroupProperty as ::core::ffi::c_uint {
        1 => {
            g_binding_group_set_source(self_0, g_value_get_object(value));
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                326 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn g_binding_group_class_init(mut klass: *mut GBindingGroupClass) {
    let mut object_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).dispose =
        Some(g_binding_group_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).finalize =
        Some(g_binding_group_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).get_property = Some(
        g_binding_group_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        g_binding_group_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    properties[PROP_SOURCE as ::core::ffi::c_int as usize] = g_param_spec_object(
        b"source\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        (G_PARAM_READWRITE as ::core::ffi::c_int
            | G_PARAM_EXPLICIT_NOTIFY as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
    );
    g_object_class_install_properties(
        object_class,
        N_PROPS as ::core::ffi::c_int as guint,
        &raw mut properties as *mut *mut GParamSpec,
    );
}
unsafe extern "C" fn g_binding_group_init(mut self_0: *mut GBindingGroup) {
    g_mutex_init(&raw mut (*self_0).mutex);
    (*self_0).lazy_bindings = g_ptr_array_new_with_free_func(Some(
        lazy_binding_free as unsafe extern "C" fn(gpointer) -> (),
    ));
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_group_new() -> *mut GBindingGroup {
    return g_object_new(g_binding_group_get_type(), ::core::ptr::null::<gchar>())
        as *mut GBindingGroup;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_group_dup_source(mut self_0: *mut GBindingGroup) -> gpointer {
    let mut source: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
            b"g_binding_group_dup_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_mutex_lock(&raw mut (*self_0).mutex);
    source = (if !(*self_0).source.is_null() {
        g_object_ref((*self_0).source as gpointer) as *mut GObject
    } else {
        ::core::ptr::null_mut::<GObject>()
    }) as *mut GObject;
    g_mutex_unlock(&raw mut (*self_0).mutex);
    return source as gpointer;
}
unsafe extern "C" fn g_binding_group_check_source(
    mut self_0: *mut GBindingGroup,
    mut source: gpointer,
) -> gboolean {
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            407 as ::core::ffi::c_int,
            b"g_binding_group_check_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if source.is_null()
        || g_type_check_instance_is_fundamentally_a(
            source as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbindinggroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            408 as ::core::ffi::c_int,
            b"g_binding_group_check_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"!source || G_IS_OBJECT (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 0 as guint;
    while i < (*(*self_0).lazy_bindings).len {
        let mut lazy_binding: *mut LazyBinding =
            *(*(*self_0).lazy_bindings).pdata.offset(i as isize) as *mut LazyBinding;
        if !g_object_class_find_property(
            (*(source as *mut GTypeInstance)).g_class as *mut GObjectClass,
            (*lazy_binding).source_property as *const gchar,
        )
        .is_null()
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_binding_group_check_source\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"g_object_class_find_property (G_OBJECT_GET_CLASS (source), lazy_binding->source_property) != NULL\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        i = i.wrapping_add(1);
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_group_set_source(
    mut self_0: *mut GBindingGroup,
    mut source: gpointer,
) {
    let mut notify: gboolean = 0 as gboolean;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
            b"g_binding_group_set_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if source.is_null()
        || g_type_check_instance_is_fundamentally_a(
            source as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_set_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"!source || G_IS_OBJECT (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if source != self_0 as gpointer {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_set_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"source != (gpointer) self\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*self_0).mutex);
    if !(source == (*self_0).source as gpointer) {
        if !(*self_0).source.is_null() {
            let mut i: guint = 0;
            g_object_weak_unref(
                (*self_0).source,
                Some(
                    g_binding_group__source_weak_notify
                        as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
                ),
                self_0 as gpointer,
            );
            (*self_0).source = ::core::ptr::null_mut::<GObject>();
            i = 0 as guint;
            while i < (*(*self_0).lazy_bindings).len {
                let mut lazy_binding: *mut LazyBinding =
                    *(*(*self_0).lazy_bindings).pdata.offset(i as isize) as *mut LazyBinding;
                g_binding_group_disconnect(lazy_binding);
                i = i.wrapping_add(1);
            }
        }
        if !source.is_null() && g_binding_group_check_source(self_0, source) != 0 {
            let mut i_0: guint = 0;
            (*self_0).source = source as *mut GObject;
            g_object_weak_ref(
                (*self_0).source,
                Some(
                    g_binding_group__source_weak_notify
                        as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
                ),
                self_0 as gpointer,
            );
            i_0 = 0 as guint;
            while i_0 < (*(*self_0).lazy_bindings).len {
                let mut lazy_binding_0: *mut LazyBinding = ::core::ptr::null_mut::<LazyBinding>();
                lazy_binding_0 =
                    *(*(*self_0).lazy_bindings).pdata.offset(i_0 as isize) as *mut LazyBinding;
                g_binding_group_connect(self_0, lazy_binding_0);
                i_0 = i_0.wrapping_add(1);
            }
        }
        notify = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    }
    g_mutex_unlock(&raw mut (*self_0).mutex);
    if notify != 0 {
        g_object_notify_by_pspec(
            self_0 as *mut ::core::ffi::c_void as *mut GObject,
            properties[PROP_SOURCE as ::core::ffi::c_int as usize],
        );
    }
}
unsafe extern "C" fn g_binding_group_bind_helper(
    mut self_0: *mut GBindingGroup,
    mut source_property: *const gchar,
    mut target: gpointer,
    mut target_property: *const gchar,
    mut flags: GBindingFlags,
    mut transform_to: gpointer,
    mut transform_from: gpointer,
    mut user_data: gpointer,
    mut user_data_destroy: GDestroyNotify,
    mut using_closures: gboolean,
) {
    let mut lazy_binding: *mut LazyBinding = ::core::ptr::null_mut::<LazyBinding>();
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_binding_group_get_type();
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
            b"g_binding_group_bind_helper\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !source_property.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_bind_helper\0" as *const u8 as *const ::core::ffi::c_char,
            b"source_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*self_0).source.is_null()
        || !g_object_class_find_property(
            (*((*self_0).source as *mut GTypeInstance)).g_class as *mut GObjectClass,
            source_property,
        )
        .is_null()
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_bind_helper\0" as *const u8 as *const ::core::ffi::c_char,
            b"self->source == NULL || g_object_class_find_property (G_OBJECT_GET_CLASS (self->source), source_property) != NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_check_instance_is_fundamentally_a(
        target as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_bind_helper\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (target)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !target_property.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_bind_helper\0" as *const u8 as *const ::core::ffi::c_char,
            b"target_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !g_object_class_find_property(
        (*(target as *mut GTypeInstance)).g_class as *mut GObjectClass,
        target_property,
    )
    .is_null()
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_bind_helper\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_find_property (G_OBJECT_GET_CLASS (target), target_property) != NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if target != self_0 as gpointer
        || strcmp(
            source_property as *const ::core::ffi::c_char,
            target_property as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_binding_group_bind_helper\0" as *const u8 as *const ::core::ffi::c_char,
            b"target != (gpointer) self || strcmp (source_property, target_property) != 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*self_0).mutex);
    lazy_binding =
        g_slice_alloc0(::core::mem::size_of::<LazyBinding>() as gsize) as *mut LazyBinding;
    (*lazy_binding).group = self_0;
    (*lazy_binding).source_property =
        g_intern_string(source_property) as *const ::core::ffi::c_char;
    (*lazy_binding).target_property =
        g_intern_string(target_property) as *const ::core::ffi::c_char;
    (*lazy_binding).target = target as *mut GObject;
    (*lazy_binding).binding_flags = (flags as ::core::ffi::c_uint
        | G_BINDING_SYNC_CREATE as ::core::ffi::c_int as ::core::ffi::c_uint)
        as GBindingFlags;
    (*lazy_binding).user_data = user_data;
    (*lazy_binding).user_data_destroy = user_data_destroy;
    (*lazy_binding).transform_to = transform_to;
    (*lazy_binding).transform_from = transform_from;
    if using_closures != 0 {
        (*lazy_binding).set_using_closures(
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
        );
        if !transform_to.is_null() {
            g_closure_sink(g_closure_ref(transform_to as *mut GClosure));
        }
        if !transform_from.is_null() {
            g_closure_sink(g_closure_ref(transform_from as *mut GClosure));
        }
    }
    g_object_weak_ref(
        target as *mut GObject,
        Some(
            g_binding_group__target_weak_notify
                as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
        ),
        self_0 as gpointer,
    );
    g_ptr_array_add((*self_0).lazy_bindings, lazy_binding as gpointer);
    if !(*self_0).source.is_null() {
        g_binding_group_connect(self_0, lazy_binding);
    }
    g_mutex_unlock(&raw mut (*self_0).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_group_bind(
    mut self_0: *mut GBindingGroup,
    mut source_property: *const gchar,
    mut target: gpointer,
    mut target_property: *const gchar,
    mut flags: GBindingFlags,
) {
    g_binding_group_bind_full(
        self_0,
        source_property,
        target,
        target_property,
        flags,
        None,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_group_bind_full(
    mut self_0: *mut GBindingGroup,
    mut source_property: *const gchar,
    mut target: gpointer,
    mut target_property: *const gchar,
    mut flags: GBindingFlags,
    mut transform_to: GBindingTransformFunc,
    mut transform_from: GBindingTransformFunc,
    mut user_data: gpointer,
    mut user_data_destroy: GDestroyNotify,
) {
    g_binding_group_bind_helper(
        self_0,
        source_property,
        target,
        target_property,
        flags,
        ::core::mem::transmute::<GBindingTransformFunc, gpointer>(transform_to),
        ::core::mem::transmute::<GBindingTransformFunc, gpointer>(transform_from),
        user_data,
        user_data_destroy,
        0 as gboolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_group_bind_with_closures(
    mut self_0: *mut GBindingGroup,
    mut source_property: *const gchar,
    mut target: gpointer,
    mut target_property: *const gchar,
    mut flags: GBindingFlags,
    mut transform_to: *mut GClosure,
    mut transform_from: *mut GClosure,
) {
    g_binding_group_bind_helper(
        self_0,
        source_property,
        target,
        target_property,
        flags,
        transform_to as gpointer,
        transform_from as gpointer,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
