// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
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
    fn g_atomic_rc_box_alloc0(block_size: gsize) -> gpointer;
    fn g_atomic_rc_box_acquire(mem_block: gpointer) -> gpointer;
    fn g_atomic_rc_box_release_full(mem_block: gpointer, clear_func: GDestroyNotify);
    fn g_slice_alloc0(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_unset(value: *mut GValue);
    fn g_value_type_compatible(src_type: GType, dest_type: GType) -> gboolean;
    fn g_value_type_transformable(src_type: GType, dest_type: GType) -> gboolean;
    fn g_value_transform(src_value: *const GValue, dest_value: *mut GValue) -> gboolean;
    fn g_param_value_validate(pspec: *mut GParamSpec, value: *mut GValue) -> gboolean;
    fn g_param_spec_is_valid_name(name: *const gchar) -> gboolean;
    fn g_cclosure_new(
        callback_func: GCallback,
        user_data: gpointer,
        destroy_data: GClosureNotify,
    ) -> *mut GClosure;
    fn g_closure_ref(closure: *mut GClosure) -> *mut GClosure;
    fn g_closure_sink(closure: *mut GClosure);
    fn g_closure_unref(closure: *mut GClosure);
    fn g_closure_set_marshal(closure: *mut GClosure, marshal: GClosureMarshal);
    fn g_closure_invoke(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
    );
    fn g_cclosure_marshal_BOOLEAN__BOXED_BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_signal_lookup(name: *const gchar, itype: GType) -> guint;
    fn g_signal_connect_closure_by_id(
        instance: gpointer,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        after: gboolean,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
    fn g_value_get_type() -> GType;
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
    fn g_object_set_property(
        object: *mut GObject,
        property_name: *const gchar,
        value: *const GValue,
    );
    fn g_object_get_property(object: *mut GObject, property_name: *const gchar, value: *mut GValue);
    fn g_object_unref(object: gpointer);
    fn g_object_weak_ref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_object_weak_unref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_value_take_object(value: *mut GValue, v_object: gpointer);
    fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_weak_ref_set(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_flags_register_static(
        name: *const gchar,
        const_static_values: *const GFlagsValue,
    ) -> GType;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_interned_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
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
pub type GQuark = guint32;
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
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p: gpointer,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GBinding {
    pub parent_instance: GObject,
    pub context: *mut BindingContext,
    pub unbind_lock: GMutex,
    pub transform_func: *mut TransformFunc,
    pub source_property: *const gchar,
    pub target_property: *const gchar,
    pub source_pspec: *mut GParamSpec,
    pub target_pspec: *mut GParamSpec,
    pub flags: GBindingFlags,
    pub source_notify: guint,
    pub target_notify: guint,
    pub target_weak_notify_installed: gboolean,
    #[bitfield(name = "is_frozen", ty = "guint", bits = "0..=0")]
    pub is_frozen: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GBindingFlags = ::core::ffi::c_uint;
pub const G_BINDING_INVERT_BOOLEAN: GBindingFlags = 4;
pub const G_BINDING_SYNC_CREATE: GBindingFlags = 2;
pub const G_BINDING_BIDIRECTIONAL: GBindingFlags = 1;
pub const G_BINDING_DEFAULT: GBindingFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransformFunc {
    pub transform_s2t: GBindingTransformFunc,
    pub transform_t2s: GBindingTransformFunc,
    pub transform_data: gpointer,
    pub destroy_notify: GDestroyNotify,
}
pub type GBindingTransformFunc =
    Option<unsafe extern "C" fn(*mut GBinding, *const GValue, *mut GValue, gpointer) -> gboolean>;
pub type GBinding = _GBinding;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindingContext {
    pub binding: GWeakRef,
    pub source: GWeakRef,
    pub target: GWeakRef,
    pub binding_removed: gboolean,
}
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GBindingClass = _GBindingClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBindingClass {
    pub parent_class: GObjectClass,
}
pub const PROP_FLAGS: C2RustUnnamed_1 = 5;
pub const PROP_TARGET_PROPERTY: C2RustUnnamed_1 = 4;
pub const PROP_SOURCE_PROPERTY: C2RustUnnamed_1 = 3;
pub const PROP_TARGET: C2RustUnnamed_1 = 2;
pub const PROP_SOURCE: C2RustUnnamed_1 = 1;
pub type TransformData = _TransformData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TransformData {
    pub transform_to_closure: *mut GClosure,
    pub transform_from_closure: *mut GClosure,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    return ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_flags_get_type() -> GType {
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
        static mut values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_BINDING_DEFAULT as ::core::ffi::c_int as guint,
                value_name: b"G_BINDING_DEFAULT\0" as *const u8 as *const gchar,
                value_nick: b"default\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_BINDING_BIDIRECTIONAL as ::core::ffi::c_int as guint,
                value_name: b"G_BINDING_BIDIRECTIONAL\0" as *const u8 as *const gchar,
                value_nick: b"bidirectional\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_BINDING_SYNC_CREATE as ::core::ffi::c_int as guint,
                value_name: b"G_BINDING_SYNC_CREATE\0" as *const u8 as *const gchar,
                value_nick: b"sync-create\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_BINDING_INVERT_BOOLEAN as ::core::ffi::c_int as guint,
                value_name: b"G_BINDING_INVERT_BOOLEAN\0" as *const u8 as *const gchar,
                value_nick: b"invert-boolean\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GBindingFlags\0" as *const u8 as *const gchar),
            &raw const values as *const GFlagsValue,
        );
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
unsafe extern "C" fn binding_context_ref(mut context: *mut BindingContext) -> *mut BindingContext {
    return g_atomic_rc_box_acquire(context as gpointer) as *mut BindingContext;
}
unsafe extern "C" fn binding_context_clear(mut context: *mut BindingContext) {
    g_weak_ref_clear(&raw mut (*context).binding);
    g_weak_ref_clear(&raw mut (*context).source);
    g_weak_ref_clear(&raw mut (*context).target);
}
unsafe extern "C" fn binding_context_unref(mut context: *mut BindingContext) {
    g_atomic_rc_box_release_full(
        context as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut BindingContext) -> ()>,
            GDestroyNotify,
        >(Some(
            binding_context_clear as unsafe extern "C" fn(*mut BindingContext) -> (),
        )),
    );
}
unsafe extern "C" fn transform_func_new(
    mut transform_s2t: GBindingTransformFunc,
    mut transform_t2s: GBindingTransformFunc,
    mut transform_data: gpointer,
    mut destroy_notify: GDestroyNotify,
) -> *mut TransformFunc {
    let mut func: *mut TransformFunc =
        g_atomic_rc_box_alloc0(::core::mem::size_of::<TransformFunc>() as gsize)
            as *mut TransformFunc;
    (*func).transform_s2t = transform_s2t;
    (*func).transform_t2s = transform_t2s;
    (*func).transform_data = transform_data;
    (*func).destroy_notify = destroy_notify;
    return func;
}
unsafe extern "C" fn transform_func_ref(mut func: *mut TransformFunc) -> *mut TransformFunc {
    return g_atomic_rc_box_acquire(func as gpointer) as *mut TransformFunc;
}
unsafe extern "C" fn transform_func_clear(mut func: *mut TransformFunc) {
    if (*func).destroy_notify.is_some() {
        (*func).destroy_notify.expect("non-null function pointer")((*func).transform_data);
    }
}
unsafe extern "C" fn transform_func_unref(mut func: *mut TransformFunc) {
    g_atomic_rc_box_release_full(
        func as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut TransformFunc) -> ()>,
            GDestroyNotify,
        >(Some(
            transform_func_clear as unsafe extern "C" fn(*mut TransformFunc) -> (),
        )),
    );
}
static mut gobject_notify_signal_id: guint = 0;
static mut GBinding_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn g_binding_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_binding_get_type_once();
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
unsafe extern "C" fn g_binding_class_intern_init(mut klass: gpointer) {
    g_binding_parent_class = g_type_class_peek_parent(klass);
    if GBinding_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut GBinding_private_offset);
    }
    g_binding_class_init(klass as *mut GBindingClass);
}
static mut g_binding_parent_class: gpointer =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[inline(never)]
unsafe extern "C" fn g_binding_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GBinding\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GBindingClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_binding_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GBinding>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GBinding) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_binding_init as unsafe extern "C" fn(*mut GBinding) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn unbind_internal_locked(
    mut context: *mut BindingContext,
    mut binding: *mut GBinding,
    mut source: *mut GObject,
    mut target: *mut GObject,
) -> gboolean {
    let mut binding_was_removed: gboolean = 0 as gboolean;
    if !context.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            298 as ::core::ffi::c_int,
            b"unbind_internal_locked\0" as *const u8 as *const ::core::ffi::c_char,
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !binding.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            299 as ::core::ffi::c_int,
            b"unbind_internal_locked\0" as *const u8 as *const ::core::ffi::c_char,
            b"binding != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !source.is_null() {
        if (*binding).source_notify != 0 as guint {
            g_signal_handler_disconnect(source as gpointer, (*binding).source_notify as gulong);
            g_object_weak_unref(
                source,
                Some(weak_unbind as unsafe extern "C" fn(gpointer, *mut GObject) -> ()),
                context as gpointer,
            );
            binding_context_unref(context);
            (*binding).source_notify = 0 as guint;
        }
        g_weak_ref_set(
            &raw mut (*context).source,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    }
    if !target.is_null() {
        if (*binding).target_notify != 0 as guint {
            g_signal_handler_disconnect(target as gpointer, (*binding).target_notify as gulong);
            (*binding).target_notify = 0 as guint;
        }
        g_weak_ref_set(
            &raw mut (*context).target,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if (*binding).target_weak_notify_installed != 0 {
            g_object_weak_unref(
                target,
                Some(weak_unbind as unsafe extern "C" fn(gpointer, *mut GObject) -> ()),
                context as gpointer,
            );
            binding_context_unref(context);
            (*binding).target_weak_notify_installed = 0 as ::core::ffi::c_int as gboolean;
        }
    }
    if (*context).binding_removed == 0 {
        (*context).binding_removed =
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        binding_was_removed = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    }
    return binding_was_removed;
}
unsafe extern "C" fn weak_unbind(mut user_data: gpointer, mut where_the_object_was: *mut GObject) {
    let mut context: *mut BindingContext = user_data as *mut BindingContext;
    let mut binding: *mut GBinding = ::core::ptr::null_mut::<GBinding>();
    let mut source: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut binding_was_removed: gboolean = 0 as gboolean;
    let mut transform_func: *mut TransformFunc = ::core::ptr::null_mut::<TransformFunc>();
    binding = g_weak_ref_get(&raw mut (*context).binding) as *mut GBinding;
    if binding.is_null() {
        binding_context_unref(context);
        return;
    }
    g_mutex_lock(&raw mut (*binding).unbind_lock);
    transform_func = g_steal_pointer(&raw mut (*binding).transform_func as gpointer)
        as *mut TransformFunc as *mut TransformFunc;
    source = g_weak_ref_get(&raw mut (*context).source) as *mut GObject;
    target = g_weak_ref_get(&raw mut (*context).target) as *mut GObject;
    if source == where_the_object_was {
        g_weak_ref_set(
            &raw mut (*context).source,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        let mut _pp: *mut *mut GObject = &raw mut source;
        let mut _ptr: *mut GObject = *_pp;
        *_pp = ::core::ptr::null_mut::<GObject>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    }
    if target == where_the_object_was {
        g_weak_ref_set(
            &raw mut (*context).target,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        let mut _pp_0: *mut *mut GObject = &raw mut target;
        let mut _ptr_0: *mut GObject = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<GObject>();
        if !_ptr_0.is_null() {
            g_object_unref(_ptr_0 as gpointer);
        }
    }
    binding_was_removed = unbind_internal_locked(context, binding, source, target);
    g_mutex_unlock(&raw mut (*binding).unbind_lock);
    let mut _pp_1: *mut *mut GObject = &raw mut target;
    let mut _ptr_1: *mut GObject = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GObject>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    let mut _pp_2: *mut *mut GObject = &raw mut source;
    let mut _ptr_2: *mut GObject = *_pp_2;
    *_pp_2 = ::core::ptr::null_mut::<GObject>();
    if !_ptr_2.is_null() {
        g_object_unref(_ptr_2 as gpointer);
    }
    let mut _pp_3: *mut *mut TransformFunc = &raw mut transform_func;
    let mut _ptr_3: *mut TransformFunc = *_pp_3;
    *_pp_3 = ::core::ptr::null_mut::<TransformFunc>();
    if !_ptr_3.is_null() {
        transform_func_unref(_ptr_3 as *mut TransformFunc);
    }
    g_object_unref(binding as gpointer);
    if binding_was_removed != 0 {
        g_object_unref(binding as gpointer);
    }
    binding_context_unref(context);
}
unsafe extern "C" fn default_transform(
    mut binding: *mut GBinding,
    mut value_a: *const GValue,
    mut value_b: *mut GValue,
    mut user_data: gpointer,
) -> gboolean {
    if !((*(value_a as *mut GValue)).g_type == (*value_b).g_type
        || g_type_is_a((*(value_a as *mut GValue)).g_type, (*value_b).g_type) != 0)
    {
        if g_value_type_compatible((*(value_a as *mut GValue)).g_type, (*value_b).g_type) != 0 {
            g_value_copy(value_a, value_b);
            return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        }
        if g_value_type_transformable((*(value_a as *mut GValue)).g_type, (*value_b).g_type) != 0 {
            if g_value_transform(value_a, value_b) != 0 {
                return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
            }
        }
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: Unable to convert a value of type %s to a value of type %s\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:466\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(value_a as *mut GValue)).g_type),
            g_type_name((*value_b).g_type),
        );
        return 0 as gboolean;
    }
    g_value_copy(value_a, value_b);
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn default_invert_boolean_transform(
    mut binding: *mut GBinding,
    mut value_a: *const GValue,
    mut value_b: *mut GValue,
    mut user_data: gpointer,
) -> gboolean {
    let mut value: gboolean = 0;
    if ({
        let mut __val: *const GValue = value_a;
        let mut __t: GType = ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            485 as ::core::ffi::c_int,
            b"default_invert_boolean_transform\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOOLEAN (value_a)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut __val: *const GValue = value_b as *const GValue;
        let mut __t: GType = ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            486 as ::core::ffi::c_int,
            b"default_invert_boolean_transform\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOOLEAN (value_b)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = g_value_get_boolean(value_a);
    value = (value == 0) as ::core::ffi::c_int as gboolean;
    g_value_set_boolean(value_b, value);
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn on_source_notify(
    mut source: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut context: *mut BindingContext,
) {
    let mut binding: *mut GBinding = ::core::ptr::null_mut::<GBinding>();
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut transform_func: *mut TransformFunc = ::core::ptr::null_mut::<TransformFunc>();
    let mut from_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut to_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut res: gboolean = 0;
    binding = g_weak_ref_get(&raw mut (*context).binding) as *mut GBinding;
    if binding.is_null() {
        return;
    }
    if (*binding).is_frozen() != 0 {
        g_object_unref(binding as gpointer);
        return;
    }
    target = g_weak_ref_get(&raw mut (*context).target) as *mut GObject;
    if target.is_null() {
        g_object_unref(binding as gpointer);
        return;
    }
    g_mutex_lock(&raw mut (*binding).unbind_lock);
    if (*binding).transform_func.is_null() {
        g_mutex_unlock(&raw mut (*binding).unbind_lock);
        return;
    }
    transform_func = transform_func_ref((*binding).transform_func);
    g_mutex_unlock(&raw mut (*binding).unbind_lock);
    g_value_init(
        &raw mut from_value,
        (*((*binding).source_pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
    );
    g_value_init(
        &raw mut to_value,
        (*((*binding).target_pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
    );
    g_object_get_property(source, (*(*binding).source_pspec).name, &raw mut from_value);
    res = (*transform_func)
        .transform_s2t
        .expect("non-null function pointer")(
        binding,
        &raw mut from_value,
        &raw mut to_value,
        (*transform_func).transform_data,
    );
    transform_func_unref(transform_func);
    if res != 0 {
        (*binding)
            .set_is_frozen((0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint);
        g_param_value_validate((*binding).target_pspec, &raw mut to_value);
        g_object_set_property(target, (*(*binding).target_pspec).name, &raw mut to_value);
        (*binding).set_is_frozen(0 as guint as guint);
    }
    g_value_unset(&raw mut from_value);
    g_value_unset(&raw mut to_value);
    g_object_unref(target as gpointer);
    g_object_unref(binding as gpointer);
}
unsafe extern "C" fn on_target_notify(
    mut target: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut context: *mut BindingContext,
) {
    let mut binding: *mut GBinding = ::core::ptr::null_mut::<GBinding>();
    let mut source: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut transform_func: *mut TransformFunc = ::core::ptr::null_mut::<TransformFunc>();
    let mut from_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut to_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut res: gboolean = 0;
    binding = g_weak_ref_get(&raw mut (*context).binding) as *mut GBinding;
    if binding.is_null() {
        return;
    }
    if (*binding).is_frozen() != 0 {
        g_object_unref(binding as gpointer);
        return;
    }
    source = g_weak_ref_get(&raw mut (*context).source) as *mut GObject;
    if source.is_null() {
        g_object_unref(binding as gpointer);
        return;
    }
    g_mutex_lock(&raw mut (*binding).unbind_lock);
    if (*binding).transform_func.is_null() {
        g_mutex_unlock(&raw mut (*binding).unbind_lock);
        return;
    }
    transform_func = transform_func_ref((*binding).transform_func);
    g_mutex_unlock(&raw mut (*binding).unbind_lock);
    g_value_init(
        &raw mut from_value,
        (*((*binding).target_pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
    );
    g_value_init(
        &raw mut to_value,
        (*((*binding).source_pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
    );
    g_object_get_property(target, (*(*binding).target_pspec).name, &raw mut from_value);
    res = (*transform_func)
        .transform_t2s
        .expect("non-null function pointer")(
        binding,
        &raw mut from_value,
        &raw mut to_value,
        (*transform_func).transform_data,
    );
    transform_func_unref(transform_func);
    if res != 0 {
        (*binding)
            .set_is_frozen((0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint);
        g_param_value_validate((*binding).source_pspec, &raw mut to_value);
        g_object_set_property(source, (*(*binding).source_pspec).name, &raw mut to_value);
        (*binding).set_is_frozen(0 as guint as guint);
    }
    g_value_unset(&raw mut from_value);
    g_value_unset(&raw mut to_value);
    g_object_unref(source as gpointer);
    g_object_unref(binding as gpointer);
}
#[inline]
unsafe extern "C" fn g_binding_unbind_internal(
    mut binding: *mut GBinding,
    mut unref_binding: gboolean,
) {
    let mut context: *mut BindingContext = (*binding).context;
    let mut source: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut binding_was_removed: gboolean = 0 as gboolean;
    let mut transform_func: *mut TransformFunc = ::core::ptr::null_mut::<TransformFunc>();
    g_mutex_lock(&raw mut (*binding).unbind_lock);
    transform_func = g_steal_pointer(&raw mut (*binding).transform_func as gpointer)
        as *mut TransformFunc as *mut TransformFunc;
    source = g_weak_ref_get(&raw mut (*context).source) as *mut GObject;
    target = g_weak_ref_get(&raw mut (*context).target) as *mut GObject;
    binding_was_removed = unbind_internal_locked(context, binding, source, target);
    g_mutex_unlock(&raw mut (*binding).unbind_lock);
    let mut _pp: *mut *mut GObject = &raw mut target;
    let mut _ptr: *mut GObject = *_pp;
    *_pp = ::core::ptr::null_mut::<GObject>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GObject = &raw mut source;
    let mut _ptr_0: *mut GObject = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GObject>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut TransformFunc = &raw mut transform_func;
    let mut _ptr_1: *mut TransformFunc = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<TransformFunc>();
    if !_ptr_1.is_null() {
        transform_func_unref(_ptr_1 as *mut TransformFunc);
    }
    if binding_was_removed != 0 && unref_binding != 0 {
        g_object_unref(binding as gpointer);
    }
}
unsafe extern "C" fn g_binding_finalize(mut gobject: *mut GObject) {
    let mut binding: *mut GBinding = gobject as *mut ::core::ffi::c_void as *mut GBinding;
    g_binding_unbind_internal(binding, 0 as gboolean);
    binding_context_unref((*binding).context);
    g_mutex_clear(&raw mut (*binding).unbind_lock);
    (*(g_binding_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(gobject);
}
unsafe extern "C" fn canonicalize_key(mut key: *mut gchar) {
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    p = key;
    while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut c: gchar = *p;
        if c as ::core::ffi::c_int == '_' as i32 {
            *p = '-' as i32 as gchar;
        }
        p = p.offset(1);
    }
}
unsafe extern "C" fn is_canonical(mut key: *const gchar) -> gboolean {
    return (strchr(key as *const ::core::ffi::c_char, '_' as i32)
        == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn g_binding_set_property(
    mut gobject: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut binding: *mut GBinding = gobject as *mut ::core::ffi::c_void as *mut GBinding;
    match prop_id {
        1 => {
            g_weak_ref_set(
                &raw mut (*(*binding).context).source,
                g_value_get_object(value),
            );
        }
        2 => {
            g_weak_ref_set(
                &raw mut (*(*binding).context).target,
                g_value_get_object(value),
            );
        }
        3 | 4 => {
            let mut name_copy: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut name: *const gchar = g_value_get_string(value);
            let mut dest: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
            if is_canonical(name) == 0 {
                name_copy = g_value_dup_string(value);
                canonicalize_key(name_copy);
                name = name_copy;
            }
            if prop_id == PROP_SOURCE_PROPERTY as ::core::ffi::c_int as guint {
                dest = &raw mut (*binding).source_property;
            } else {
                dest = &raw mut (*binding).target_property;
            }
            *dest = g_intern_string(name);
            g_free(name_copy as gpointer);
        }
        5 => {
            (*binding).flags = g_value_get_flags(value) as GBindingFlags;
        }
        _ => {
            let mut _glib__object: *mut GObject = gobject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                750 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn g_binding_get_property(
    mut gobject: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut binding: *mut GBinding = gobject as *mut ::core::ffi::c_void as *mut GBinding;
    match prop_id {
        1 => {
            g_value_take_object(value, g_weak_ref_get(&raw mut (*(*binding).context).source));
        }
        3 => {
            g_value_set_interned_string(value, (*binding).source_property);
        }
        2 => {
            g_value_take_object(value, g_weak_ref_get(&raw mut (*(*binding).context).target));
        }
        4 => {
            g_value_set_interned_string(value, (*binding).target_property);
        }
        5 => {
            g_value_set_flags(value, (*binding).flags as guint);
        }
        _ => {
            let mut _glib__object: *mut GObject = gobject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                788 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn g_binding_constructed(mut gobject: *mut GObject) {
    let mut binding: *mut GBinding = gobject as *mut ::core::ffi::c_void as *mut GBinding;
    let mut transform_func: GBindingTransformFunc = Some(
        default_transform
            as unsafe extern "C" fn(
                *mut GBinding,
                *const GValue,
                *mut GValue,
                gpointer,
            ) -> gboolean,
    );
    let mut source: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_property_detail: GQuark = 0;
    let mut source_notify_closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    source = g_weak_ref_get(&raw mut (*(*binding).context).source) as *mut GObject;
    target = g_weak_ref_get(&raw mut (*(*binding).context).target) as *mut GObject;
    if !source.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            805 as ::core::ffi::c_int,
            b"g_binding_constructed\0" as *const u8 as *const ::core::ffi::c_char,
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !target.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            806 as ::core::ffi::c_int,
            b"g_binding_constructed\0" as *const u8 as *const ::core::ffi::c_char,
            b"target != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*binding).source_property.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            807 as ::core::ffi::c_int,
            b"g_binding_constructed\0" as *const u8 as *const ::core::ffi::c_char,
            b"binding->source_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*binding).target_property.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            808 as ::core::ffi::c_int,
            b"g_binding_constructed\0" as *const u8 as *const ::core::ffi::c_char,
            b"binding->target_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*binding).source_pspec = g_object_class_find_property(
        (*(source as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*binding).source_property,
    );
    (*binding).target_pspec = g_object_class_find_property(
        (*(target as *mut GTypeInstance)).g_class as *mut GObjectClass,
        (*binding).target_property,
    );
    if !(*binding).source_pspec.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            816 as ::core::ffi::c_int,
            b"g_binding_constructed\0" as *const u8 as *const ::core::ffi::c_char,
            b"binding->source_pspec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*binding).target_pspec.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            817 as ::core::ffi::c_int,
            b"g_binding_constructed\0" as *const u8 as *const ::core::ffi::c_char,
            b"binding->target_pspec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*binding).flags as ::core::ffi::c_uint
        & G_BINDING_INVERT_BOOLEAN as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        transform_func = Some(
            default_invert_boolean_transform
                as unsafe extern "C" fn(
                    *mut GBinding,
                    *const GValue,
                    *mut GValue,
                    gpointer,
                ) -> gboolean,
        ) as GBindingTransformFunc;
    }
    (*binding).transform_func = transform_func_new(
        transform_func,
        transform_func,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
    );
    source_property_detail = g_quark_from_string((*binding).source_property);
    source_notify_closure = g_cclosure_new(
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec, *mut BindingContext) -> ()>,
            GCallback,
        >(Some(
            on_source_notify
                as unsafe extern "C" fn(*mut GObject, *mut GParamSpec, *mut BindingContext) -> (),
        )),
        binding_context_ref((*binding).context) as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut BindingContext) -> ()>,
            GClosureNotify,
        >(Some(
            binding_context_unref as unsafe extern "C" fn(*mut BindingContext) -> (),
        )),
    );
    (*binding).source_notify = g_signal_connect_closure_by_id(
        source as gpointer,
        gobject_notify_signal_id,
        source_property_detail,
        source_notify_closure,
        0 as gboolean,
    ) as guint;
    g_object_weak_ref(
        source,
        Some(weak_unbind as unsafe extern "C" fn(gpointer, *mut GObject) -> ()),
        binding_context_ref((*binding).context) as gpointer,
    );
    if (*binding).flags as ::core::ffi::c_uint
        & G_BINDING_BIDIRECTIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut target_property_detail: GQuark = 0;
        let mut target_notify_closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
        target_property_detail = g_quark_from_string((*binding).target_property);
        target_notify_closure = g_cclosure_new(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GObject, *mut GParamSpec, *mut BindingContext) -> (),
                >,
                GCallback,
            >(Some(
                on_target_notify
                    as unsafe extern "C" fn(
                        *mut GObject,
                        *mut GParamSpec,
                        *mut BindingContext,
                    ) -> (),
            )),
            binding_context_ref((*binding).context) as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut BindingContext) -> ()>,
                GClosureNotify,
            >(Some(
                binding_context_unref as unsafe extern "C" fn(*mut BindingContext) -> (),
            )),
        );
        (*binding).target_notify = g_signal_connect_closure_by_id(
            target as gpointer,
            gobject_notify_signal_id,
            target_property_detail,
            target_notify_closure,
            0 as gboolean,
        ) as guint;
    }
    if target != source {
        g_object_weak_ref(
            target,
            Some(weak_unbind as unsafe extern "C" fn(gpointer, *mut GObject) -> ()),
            binding_context_ref((*binding).context) as gpointer,
        );
        (*binding).target_weak_notify_installed =
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    }
    g_object_unref(source as gpointer);
    g_object_unref(target as gpointer);
}
unsafe extern "C" fn g_binding_class_init(mut klass: *mut GBindingClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    gobject_notify_signal_id = g_signal_lookup(
        b"notify\0" as *const u8 as *const gchar,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    if gobject_notify_signal_id != 0 as guint {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            874 as ::core::ffi::c_int,
            b"g_binding_class_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"gobject_notify_signal_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*gobject_class).constructed =
        Some(g_binding_constructed as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        g_binding_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        g_binding_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize = Some(g_binding_finalize as unsafe extern "C" fn(*mut GObject) -> ())
        as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_SOURCE as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"source\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TARGET as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"target\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_SOURCE_PROPERTY as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"source-property\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_TARGET_PROPERTY as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"target-property\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_binding_flags_get_type(),
            G_BINDING_DEFAULT as ::core::ffi::c_int as guint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn g_binding_init(mut binding: *mut GBinding) {
    g_mutex_init(&raw mut (*binding).unbind_lock);
    (*binding).context = g_atomic_rc_box_alloc0(::core::mem::size_of::<BindingContext>() as gsize)
        as *mut BindingContext;
    g_weak_ref_init(&raw mut (*(*binding).context).binding, binding as gpointer);
    g_weak_ref_init(
        &raw mut (*(*binding).context).source,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    g_weak_ref_init(
        &raw mut (*(*binding).context).target,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_get_flags(mut binding: *mut GBinding) -> GBindingFlags {
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_get_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_BINDING_DEFAULT;
    }
    return (*binding).flags;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_get_source(mut binding: *mut GBinding) -> *mut GObject {
    let mut source: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_get_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    source = g_weak_ref_get(&raw mut (*(*binding).context).source) as *mut GObject;
    if !source.is_null() {
        g_object_unref(source as gpointer);
    }
    return source;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_dup_source(mut binding: *mut GBinding) -> *mut GObject {
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_dup_source\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    return g_weak_ref_get(&raw mut (*(*binding).context).source) as *mut GObject;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_get_target(mut binding: *mut GBinding) -> *mut GObject {
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_get_target\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    target = g_weak_ref_get(&raw mut (*(*binding).context).target) as *mut GObject;
    if !target.is_null() {
        g_object_unref(target as gpointer);
    }
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_dup_target(mut binding: *mut GBinding) -> *mut GObject {
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_dup_target\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    return g_weak_ref_get(&raw mut (*(*binding).context).target) as *mut GObject;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_get_source_property(mut binding: *mut GBinding) -> *const gchar {
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_get_source_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*binding).source_property;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_get_target_property(mut binding: *mut GBinding) -> *const gchar {
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_get_target_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*binding).target_property;
}
#[no_mangle]
pub unsafe extern "C" fn g_binding_unbind(mut binding: *mut GBinding) {
    if ({
        let mut __inst: *mut GTypeInstance = binding as *mut GTypeInstance;
        let mut __t: GType = g_binding_get_type();
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
            b"g_binding_unbind\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_BINDING (binding)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_binding_unbind_internal(
        binding,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_bind_property_full(
    mut source: gpointer,
    mut source_property: *const gchar,
    mut target: gpointer,
    mut target_property: *const gchar,
    mut flags: GBindingFlags,
    mut transform_to: GBindingTransformFunc,
    mut transform_from: GBindingTransformFunc,
    mut user_data: gpointer,
    mut notify: GDestroyNotify,
) -> *mut GBinding {
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut binding: *mut GBinding = ::core::ptr::null_mut::<GBinding>();
    if g_type_check_instance_is_fundamentally_a(
        source as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_bind_property_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (source)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if !source_property.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_bind_property_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"source_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if g_param_spec_is_valid_name(source_property) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_bind_property_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_is_valid_name (source_property)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if g_type_check_instance_is_fundamentally_a(
        target as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_bind_property_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (target)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if !target_property.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_bind_property_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"target_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if g_param_spec_is_valid_name(target_property) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_bind_property_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_is_valid_name (target_property)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if source == target
        && g_strcmp0(
            source_property as *const ::core::ffi::c_char,
            target_property as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Unable to bind the same property on the same instance\0" as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if flags as ::core::ffi::c_uint
        & G_BINDING_INVERT_BOOLEAN as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (transform_to.is_some() || transform_from.is_some())
    {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GBindingFlags>(
            flags as ::core::ffi::c_uint
                & !(G_BINDING_INVERT_BOOLEAN as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    }
    pspec = g_object_class_find_property(
        (*(source as *mut GTypeInstance)).g_class as *mut GObjectClass,
        source_property,
    );
    if pspec.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The source object of type %s has no property called '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1260\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(source as *mut GTypeInstance)).g_class).g_type),
            source_property,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_READABLE as ::core::ffi::c_int == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The source object of type %s has no readable property called '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1269\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(source as *mut GTypeInstance)).g_class).g_type),
            source_property,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if flags as ::core::ffi::c_uint
        & G_BINDING_BIDIRECTIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && ((*pspec).flags as ::core::ffi::c_int & G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
            != 0
            || (*pspec).flags as ::core::ffi::c_int & G_PARAM_WRITABLE as ::core::ffi::c_int == 0)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The source object of type %s has no writable property called '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1279\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(source as *mut GTypeInstance)).g_class).g_type),
            source_property,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if flags as ::core::ffi::c_uint
        & G_BINDING_INVERT_BOOLEAN as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !((*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type
            == ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The G_BINDING_INVERT_BOOLEAN flag can only be used when binding boolean properties; the source property '%s' is of type '%s'\0"
                as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1291\0"
                as *const u8 as *const ::core::ffi::c_char,
            source_property,
            g_type_name(
                (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
            ),
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    pspec = g_object_class_find_property(
        (*(target as *mut GTypeInstance)).g_class as *mut GObjectClass,
        target_property,
    );
    if pspec.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The target object of type %s has no property called '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1301\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(target as *mut GTypeInstance)).g_class).g_type),
            target_property,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int != 0
        || (*pspec).flags as ::core::ffi::c_int & G_PARAM_WRITABLE as ::core::ffi::c_int == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The target object of type %s has no writable property called '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1310\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(target as *mut GTypeInstance)).g_class).g_type),
            target_property,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if flags as ::core::ffi::c_uint
        & G_BINDING_BIDIRECTIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*pspec).flags as ::core::ffi::c_int & G_PARAM_READABLE as ::core::ffi::c_int == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The target object of type %s has no readable property called '%s'\0" as *const u8
                as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1320\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(target as *mut GTypeInstance)).g_class).g_type),
            target_property,
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    if flags as ::core::ffi::c_uint
        & G_BINDING_INVERT_BOOLEAN as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !((*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type
            == ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: The G_BINDING_INVERT_BOOLEAN flag can only be used when binding boolean properties; the target property '%s' is of type '%s'\0"
                as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c:1332\0"
                as *const u8 as *const ::core::ffi::c_char,
            target_property,
            g_type_name(
                (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
            ),
        );
        return ::core::ptr::null_mut::<GBinding>();
    }
    binding = g_object_new(
        g_binding_get_type(),
        b"source\0" as *const u8 as *const gchar,
        source,
        b"source-property\0" as *const u8 as *const ::core::ffi::c_char,
        source_property,
        b"target\0" as *const u8 as *const ::core::ffi::c_char,
        target,
        b"target-property\0" as *const u8 as *const ::core::ffi::c_char,
        target_property,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GBinding;
    if !(*binding).transform_func.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1346 as ::core::ffi::c_int,
            b"g_object_bind_property_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"binding->transform_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if transform_to.is_none() {
        transform_to = (*(*binding).transform_func).transform_s2t;
    }
    if transform_from.is_none() {
        transform_from = (*(*binding).transform_func).transform_t2s;
    }
    let mut _pp: *mut *mut TransformFunc = &raw mut (*binding).transform_func;
    let mut _ptr: *mut TransformFunc = *_pp;
    *_pp = ::core::ptr::null_mut::<TransformFunc>();
    if !_ptr.is_null() {
        transform_func_unref(_ptr as *mut TransformFunc);
    }
    (*binding).transform_func = transform_func_new(transform_to, transform_from, user_data, notify);
    if flags as ::core::ffi::c_uint
        & G_BINDING_SYNC_CREATE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        on_source_notify(
            source as *mut GObject,
            (*binding).source_pspec,
            (*binding).context,
        );
    }
    return binding;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_bind_property(
    mut source: gpointer,
    mut source_property: *const gchar,
    mut target: gpointer,
    mut target_property: *const gchar,
    mut flags: GBindingFlags,
) -> *mut GBinding {
    return g_object_bind_property_full(
        source,
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
unsafe extern "C" fn bind_with_closures_transform_to(
    mut binding: *mut GBinding,
    mut source: *const GValue,
    mut target: *mut GValue,
    mut data: gpointer,
) -> gboolean {
    let mut t_data: *mut TransformData = data as *mut TransformData;
    let mut params: [GValue; 3] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
    ];
    let mut retval: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut res: gboolean = 0;
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        g_binding_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        binding as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        g_value_get_type(),
    );
    g_value_set_boxed(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        source as gconstpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        g_value_get_type(),
    );
    g_value_set_boxed(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        target as gconstpointer,
    );
    g_value_init(
        &raw mut retval,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_set_boolean(&raw mut retval, 0 as gboolean);
    g_closure_invoke(
        (*t_data).transform_to_closure,
        &raw mut retval,
        3 as guint,
        &raw mut params as *mut GValue,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    res = g_value_get_boolean(&raw mut retval);
    if res != 0 {
        let mut out_value: *const GValue = g_value_get_boxed(
            (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize)
                as *mut GValue,
        ) as *const GValue;
        if !out_value.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1468 as ::core::ffi::c_int,
                b"bind_with_closures_transform_to\0" as *const u8 as *const ::core::ffi::c_char,
                b"out_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_value_copy(out_value, target);
    }
    g_value_unset(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(&raw mut retval);
    return res;
}
unsafe extern "C" fn bind_with_closures_transform_from(
    mut binding: *mut GBinding,
    mut source: *const GValue,
    mut target: *mut GValue,
    mut data: gpointer,
) -> gboolean {
    let mut t_data: *mut TransformData = data as *mut TransformData;
    let mut params: [GValue; 3] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        },
    ];
    let mut retval: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut res: gboolean = 0;
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        g_binding_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        binding as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        g_value_get_type(),
    );
    g_value_set_boxed(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        source as gconstpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        g_value_get_type(),
    );
    g_value_set_boxed(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        target as gconstpointer,
    );
    g_value_init(
        &raw mut retval,
        ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    g_value_set_boolean(&raw mut retval, 0 as gboolean);
    g_closure_invoke(
        (*t_data).transform_from_closure,
        &raw mut retval,
        3 as guint,
        &raw mut params as *mut GValue,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    res = g_value_get_boolean(&raw mut retval);
    if res != 0 {
        let mut out_value: *const GValue = g_value_get_boxed(
            (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize)
                as *mut GValue,
        ) as *const GValue;
        if !out_value.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gbinding.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1511 as ::core::ffi::c_int,
                b"bind_with_closures_transform_from\0" as *const u8 as *const ::core::ffi::c_char,
                b"out_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_value_copy(out_value, target);
    }
    g_value_unset(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
    );
    g_value_unset(&raw mut retval);
    return res;
}
unsafe extern "C" fn bind_with_closures_free_func(mut data: gpointer) {
    let mut t_data: *mut TransformData = data as *mut TransformData;
    if !(*t_data).transform_to_closure.is_null() {
        g_closure_unref((*t_data).transform_to_closure);
    }
    if !(*t_data).transform_from_closure.is_null() {
        g_closure_unref((*t_data).transform_from_closure);
    }
    g_slice_free1(
        ::core::mem::size_of::<TransformData>() as gsize,
        t_data as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_bind_property_with_closures(
    mut source: gpointer,
    mut source_property: *const gchar,
    mut target: gpointer,
    mut target_property: *const gchar,
    mut flags: GBindingFlags,
    mut transform_to: *mut GClosure,
    mut transform_from: *mut GClosure,
) -> *mut GBinding {
    let mut data: *mut TransformData = ::core::ptr::null_mut::<TransformData>();
    data = g_slice_alloc0(::core::mem::size_of::<TransformData>() as gsize) as *mut TransformData;
    if !transform_to.is_null() {
        if (*transform_to).marshal.is_none() {
            g_closure_set_marshal(
                transform_to,
                Some(
                    g_cclosure_marshal_BOOLEAN__BOXED_BOXED
                        as unsafe extern "C" fn(
                            *mut GClosure,
                            *mut GValue,
                            guint,
                            *const GValue,
                            gpointer,
                            gpointer,
                        ) -> (),
                ),
            );
        }
        (*data).transform_to_closure = g_closure_ref(transform_to);
        g_closure_sink((*data).transform_to_closure);
    }
    if !transform_from.is_null() {
        if (*transform_from).marshal.is_none() {
            g_closure_set_marshal(
                transform_from,
                Some(
                    g_cclosure_marshal_BOOLEAN__BOXED_BOXED
                        as unsafe extern "C" fn(
                            *mut GClosure,
                            *mut GValue,
                            guint,
                            *const GValue,
                            gpointer,
                            gpointer,
                        ) -> (),
                ),
            );
        }
        (*data).transform_from_closure = g_closure_ref(transform_from);
        g_closure_sink((*data).transform_from_closure);
    }
    return g_object_bind_property_full(
        source,
        source_property,
        target,
        target_property,
        flags,
        if !transform_to.is_null() {
            Some(
                bind_with_closures_transform_to
                    as unsafe extern "C" fn(
                        *mut GBinding,
                        *const GValue,
                        *mut GValue,
                        gpointer,
                    ) -> gboolean,
            )
        } else {
            None
        },
        if !transform_from.is_null() {
            Some(
                bind_with_closures_transform_from
                    as unsafe extern "C" fn(
                        *mut GBinding,
                        *const GValue,
                        *mut GValue,
                        gpointer,
                    ) -> gboolean,
            )
        } else {
            None
        },
        data as gpointer,
        Some(bind_with_closures_free_func as unsafe extern "C" fn(gpointer) -> ()),
    );
}
