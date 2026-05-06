// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_remove_index(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_rec_mutex_init(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_clear(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_lock(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_unlock(rec_mutex: *mut GRecMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_peek(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_default_interface_ref(g_type: GType) -> gpointer;
    fn g_type_default_interface_peek(g_type: GType) -> gpointer;
    fn g_type_default_interface_unref(g_iface: gpointer);
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
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_cclosure_new(
        callback_func: GCallback,
        user_data: gpointer,
        destroy_data: GClosureNotify,
    ) -> *mut GClosure;
    fn g_cclosure_new_swap(
        callback_func: GCallback,
        user_data: gpointer,
        destroy_data: GClosureNotify,
    ) -> *mut GClosure;
    fn g_closure_ref(closure: *mut GClosure) -> *mut GClosure;
    fn g_closure_sink(closure: *mut GClosure);
    fn g_closure_unref(closure: *mut GClosure);
    fn g_closure_invalidate(closure: *mut GClosure);
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
    fn g_signal_parse_name(
        detailed_signal: *const gchar,
        itype: GType,
        signal_id_p: *mut guint,
        detail_p: *mut GQuark,
        force_detail_quark: gboolean,
    ) -> gboolean;
    fn g_signal_connect_closure_by_id(
        instance: gpointer,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        after: gboolean,
    ) -> gulong;
    fn g_signal_handler_block(instance: gpointer, handler_id: gulong);
    fn g_signal_handler_unblock(instance: gpointer, handler_id: gulong);
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify_by_pspec(object: *mut GObject, pspec: *mut GParamSpec);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_weak_ref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_object_weak_unref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_object_watch_closure(object: *mut GObject, closure: *mut GClosure);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_value_take_object(value: *mut GValue, v_object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_weak_ref_set(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_gtype(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        is_a_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_gtype(value: *mut GValue, v_gtype: GType);
    fn g_value_get_gtype(value: *const GValue) -> GType;
}
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRecMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRecMutex = _GRecMutex;
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
pub type GConnectFlags = ::core::ffi::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
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
pub struct _GSignalGroup {
    pub parent_instance: GObject,
    pub target_ref: GWeakRef,
    pub mutex: GRecMutex,
    pub handlers: *mut GPtrArray,
    pub target_type: GType,
    pub block_count: gssize,
    #[bitfield(name = "has_bound_at_least_once", ty = "guint", bits = "0..=0")]
    pub has_bound_at_least_once: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GSignalGroup = _GSignalGroup;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SignalHandler {
    pub group: *mut GSignalGroup,
    pub handler_id: gulong,
    pub closure: *mut GClosure,
    pub signal_id: guint,
    pub signal_detail: GQuark,
    #[bitfield(name = "connect_after", ty = "guint", bits = "0..=0")]
    pub connect_after: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GSignalGroupClass = _GSignalGroupClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalGroupClass {
    pub parent_class: GObjectClass,
    pub bind: Option<unsafe extern "C" fn(*mut GSignalGroup, *mut GObject) -> ()>,
}
pub const UNBIND: C2RustUnnamed_1 = 1;
pub const BIND: C2RustUnnamed_1 = 0;
pub const LAST_PROP: GSignalGroupProperty = 3;
pub const PROP_TARGET_TYPE: GSignalGroupProperty = 2;
pub const PROP_TARGET: GSignalGroupProperty = 1;
pub type GSignalGroupProperty = ::core::ffi::c_uint;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_1 = 2;
static mut g_signal_group_parent_class: gpointer =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_signal_group_get_type_once();
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
static mut GSignalGroup_private_offset: gint = 0;
unsafe extern "C" fn g_signal_group_class_intern_init(mut klass: gpointer) {
    g_signal_group_parent_class = g_type_class_peek_parent(klass);
    if GSignalGroup_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut GSignalGroup_private_offset);
    }
    g_signal_group_class_init(klass as *mut GSignalGroupClass);
}
#[inline(never)]
unsafe extern "C" fn g_signal_group_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSignalGroup\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSignalGroupClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_signal_group_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSignalGroup>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSignalGroup) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_signal_group_init as unsafe extern "C" fn(*mut GSignalGroup) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut properties: [*mut GParamSpec; 3] =
    [::core::ptr::null::<GParamSpec>() as *mut GParamSpec; 3];
static mut signals: [guint; 2] = [0; 2];
unsafe extern "C" fn g_signal_group_set_target_type(
    mut self_0: *mut GSignalGroup,
    mut target_type: GType,
) {
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            110 as ::core::ffi::c_int,
            b"g_signal_group_set_target_type\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if target_type == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        || g_type_is_a(
            target_type,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            111 as ::core::ffi::c_int,
            b"g_signal_group_set_target_type\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_is_a (target_type, G_TYPE_OBJECT)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*self_0).target_type = target_type;
    if g_type_fundamental(target_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        if g_type_default_interface_peek(target_type).is_null() {
            g_type_default_interface_unref(g_type_default_interface_ref(target_type));
        }
    } else if g_type_class_peek(target_type).is_null() {
        g_type_class_unref(g_type_class_ref(target_type));
    }
}
unsafe extern "C" fn g_signal_group_gc_handlers(mut self_0: *mut GSignalGroup) {
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            135 as ::core::ffi::c_int,
            b"g_signal_group_gc_handlers\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = (*(*self_0).handlers).len;
    while i > 0 as guint {
        let mut handler: *const SignalHandler = *(*(*self_0).handlers)
            .pdata
            .offset(i.wrapping_sub(1 as guint) as isize)
            as *const SignalHandler;
        if !handler.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int,
                b"g_signal_group_gc_handlers\0" as *const u8 as *const ::core::ffi::c_char,
                b"handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(*handler).closure.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_int,
                b"g_signal_group_gc_handlers\0" as *const u8 as *const ::core::ffi::c_char,
                b"handler->closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*(*handler).closure).is_invalid() != 0 {
            g_ptr_array_remove_index((*self_0).handlers, i.wrapping_sub(1 as guint));
        }
        i = i.wrapping_sub(1);
    }
}
unsafe extern "C" fn g_signal_group__target_weak_notify(
    mut data: gpointer,
    mut where_object_was: *mut GObject,
) {
    let mut self_0: *mut GSignalGroup = data as *mut GSignalGroup;
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            162 as ::core::ffi::c_int,
            b"g_signal_group__target_weak_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !where_object_was.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            163 as ::core::ffi::c_int,
            b"g_signal_group__target_weak_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"where_object_was != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    g_weak_ref_set(
        &raw mut (*self_0).target_ref,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    i = 0 as guint;
    while i < (*(*self_0).handlers).len {
        let mut handler: *mut SignalHandler =
            *(*(*self_0).handlers).pdata.offset(i as isize) as *mut SignalHandler;
        (*handler).handler_id = 0 as gulong;
        i = i.wrapping_add(1);
    }
    g_signal_emit(
        self_0 as gpointer,
        signals[UNBIND as ::core::ffi::c_int as usize],
        0 as GQuark,
    );
    g_object_notify_by_pspec(
        self_0 as *mut ::core::ffi::c_void as *mut GObject,
        properties[PROP_TARGET as ::core::ffi::c_int as usize],
    );
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
}
unsafe extern "C" fn g_signal_group_bind_handler(
    mut self_0: *mut GSignalGroup,
    mut handler: *mut SignalHandler,
    mut target: *mut GObject,
) {
    let mut i: gssize = 0;
    if !self_0.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            189 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"self != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if g_type_check_instance_is_fundamentally_a(
        target as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            190 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (target)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !handler.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            191 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*handler).signal_id != 0 as guint {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            192 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->signal_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*handler).closure.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            193 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*(*handler).closure).is_invalid() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            194 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->closure->is_invalid == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*handler).handler_id == 0 as gulong {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            195 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->handler_id == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*handler).handler_id = g_signal_connect_closure_by_id(
        target as gpointer,
        (*handler).signal_id,
        (*handler).signal_detail,
        (*handler).closure,
        (*handler).connect_after() as gboolean,
    );
    if (*handler).handler_id != 0 as gulong {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            203 as ::core::ffi::c_int,
            b"g_signal_group_bind_handler\0" as *const u8 as *const ::core::ffi::c_char,
            b"handler->handler_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 0 as gssize;
    while i < (*self_0).block_count {
        g_signal_handler_block(target as gpointer, (*handler).handler_id);
        i += 1;
    }
}
unsafe extern "C" fn g_signal_group_bind(mut self_0: *mut GSignalGroup, mut target: *mut GObject) {
    let mut hold: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            216 as ::core::ffi::c_int,
            b"g_signal_group_bind\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if target.is_null()
        || g_type_check_instance_is_fundamentally_a(
            target as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            217 as ::core::ffi::c_int,
            b"g_signal_group_bind\0" as *const u8 as *const ::core::ffi::c_char,
            b"!target || G_IS_OBJECT (target)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if target.is_null() {
        return;
    }
    (*self_0).set_has_bound_at_least_once(
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
    );
    hold = g_object_ref(target as gpointer) as *mut GObject as *mut GObject;
    g_weak_ref_set(&raw mut (*self_0).target_ref, hold as gpointer);
    g_object_weak_ref(
        hold,
        Some(
            g_signal_group__target_weak_notify
                as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
        ),
        self_0 as gpointer,
    );
    g_signal_group_gc_handlers(self_0);
    i = 0 as guint;
    while i < (*(*self_0).handlers).len {
        let mut handler: *mut SignalHandler =
            *(*(*self_0).handlers).pdata.offset(i as isize) as *mut SignalHandler;
        g_signal_group_bind_handler(self_0, handler, hold);
        i = i.wrapping_add(1);
    }
    g_signal_emit(
        self_0 as gpointer,
        signals[BIND as ::core::ffi::c_int as usize],
        0 as GQuark,
        hold,
    );
    g_object_unref(hold as gpointer);
}
unsafe extern "C" fn g_signal_group_unbind(mut self_0: *mut GSignalGroup) {
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"g_signal_group_unbind\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    target = g_weak_ref_get(&raw mut (*self_0).target_ref) as *mut GObject;
    if !target.is_null() {
        g_weak_ref_set(
            &raw mut (*self_0).target_ref,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        g_object_weak_unref(
            target,
            Some(
                g_signal_group__target_weak_notify
                    as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
            ),
            self_0 as gpointer,
        );
    }
    g_signal_group_gc_handlers(self_0);
    i = 0 as guint;
    while i < (*(*self_0).handlers).len {
        let mut handler: *mut SignalHandler = ::core::ptr::null_mut::<SignalHandler>();
        let mut handler_id: gulong = 0;
        handler = *(*(*self_0).handlers).pdata.offset(i as isize) as *mut SignalHandler;
        if !handler.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                281 as ::core::ffi::c_int,
                b"g_signal_group_unbind\0" as *const u8 as *const ::core::ffi::c_char,
                b"handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*handler).signal_id != 0 as guint {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int,
                b"g_signal_group_unbind\0" as *const u8 as *const ::core::ffi::c_char,
                b"handler->signal_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(*handler).closure.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int,
                b"g_signal_group_unbind\0" as *const u8 as *const ::core::ffi::c_char,
                b"handler->closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        handler_id = (*handler).handler_id;
        (*handler).handler_id = 0 as gulong;
        if !target.is_null() && handler_id != 0 as gulong {
            g_signal_handler_disconnect(target as gpointer, handler_id);
        }
        i = i.wrapping_add(1);
    }
    g_signal_emit(
        self_0 as gpointer,
        signals[UNBIND as ::core::ffi::c_int as usize],
        0 as GQuark,
    );
    let mut _pp: *mut *mut GObject = &raw mut target;
    let mut _ptr: *mut GObject = *_pp;
    *_pp = ::core::ptr::null_mut::<GObject>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
}
unsafe extern "C" fn g_signal_group_check_target_type(
    mut self_0: *mut GSignalGroup,
    mut target: gpointer,
) -> gboolean {
    if !target.is_null()
        && !((*(*(target as *mut GTypeInstance)).g_class).g_type == (*self_0).target_type
            || g_type_is_a(
                (*(*(target as *mut GTypeInstance)).g_class).g_type,
                (*self_0).target_type,
            ) != 0)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Failed to set GSignalGroup of target type %s using target %p of type %s\0"
                as *const u8 as *const gchar,
            g_type_name((*self_0).target_type),
            target,
            g_type_name((*(*(target as *mut GTypeInstance)).g_class).g_type),
        );
        return 0 as gboolean;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_block(mut self_0: *mut GSignalGroup) {
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"g_signal_group_block\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*self_0).block_count >= 0 as gssize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_block\0" as *const u8 as *const ::core::ffi::c_char,
            b"self->block_count >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    (*self_0).block_count += 1;
    target = g_weak_ref_get(&raw mut (*self_0).target_ref) as *mut GObject;
    if !target.is_null() {
        i = 0 as guint;
        while i < (*(*self_0).handlers).len {
            let mut handler: *const SignalHandler =
                *(*(*self_0).handlers).pdata.offset(i as isize) as *const SignalHandler;
            if !handler.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    354 as ::core::ffi::c_int,
                    b"g_signal_group_block\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*handler).signal_id != 0 as guint {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    355 as ::core::ffi::c_int,
                    b"g_signal_group_block\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler->signal_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !(*handler).closure.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    356 as ::core::ffi::c_int,
                    b"g_signal_group_block\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler->closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*handler).handler_id != 0 as gulong {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    357 as ::core::ffi::c_int,
                    b"g_signal_group_block\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler->handler_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            g_signal_handler_block(target as gpointer, (*handler).handler_id);
            i = i.wrapping_add(1);
        }
        g_object_unref(target as gpointer);
    }
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_unblock(mut self_0: *mut GSignalGroup) {
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut i: guint = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"g_signal_group_unblock\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*self_0).block_count > 0 as gssize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_unblock\0" as *const u8 as *const ::core::ffi::c_char,
            b"self->block_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    (*self_0).block_count -= 1;
    target = g_weak_ref_get(&raw mut (*self_0).target_ref) as *mut GObject;
    if !target.is_null() {
        i = 0 as guint;
        while i < (*(*self_0).handlers).len {
            let mut handler: *const SignalHandler =
                *(*(*self_0).handlers).pdata.offset(i as isize) as *const SignalHandler;
            if !handler.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    400 as ::core::ffi::c_int,
                    b"g_signal_group_unblock\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*handler).signal_id != 0 as guint {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    401 as ::core::ffi::c_int,
                    b"g_signal_group_unblock\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler->signal_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !(*handler).closure.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    402 as ::core::ffi::c_int,
                    b"g_signal_group_unblock\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler->closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*handler).handler_id != 0 as gulong {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    403 as ::core::ffi::c_int,
                    b"g_signal_group_unblock\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handler->handler_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            g_signal_handler_unblock(target as gpointer, (*handler).handler_id);
            i = i.wrapping_add(1);
        }
        g_object_unref(target as gpointer);
    }
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_dup_target(mut self_0: *mut GSignalGroup) -> gpointer {
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"g_signal_group_dup_target\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    target = g_weak_ref_get(&raw mut (*self_0).target_ref) as *mut GObject;
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
    return target as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_set_target(
    mut self_0: *mut GSignalGroup,
    mut target: gpointer,
) {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"g_signal_group_set_target\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    object = g_weak_ref_get(&raw mut (*self_0).target_ref) as *mut GObject;
    if !(object == target as *mut GObject) {
        if !(g_signal_group_check_target_type(self_0, target) == 0) {
            if (*self_0).has_bound_at_least_once() != 0 {
                g_signal_group_unbind(self_0);
            }
            g_signal_group_bind(self_0, target as *mut GObject);
            g_object_notify_by_pspec(
                self_0 as *mut ::core::ffi::c_void as *mut GObject,
                properties[PROP_TARGET as ::core::ffi::c_int as usize],
            );
        }
    }
    let mut _pp: *mut *mut GObject = &raw mut object;
    let mut _ptr: *mut GObject = *_pp;
    *_pp = ::core::ptr::null_mut::<GObject>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
}
unsafe extern "C" fn signal_handler_free(mut data: gpointer) {
    let mut handler: *mut SignalHandler = data as *mut SignalHandler;
    if !(*handler).closure.is_null() {
        g_closure_invalidate((*handler).closure);
    }
    (*handler).handler_id = 0 as gulong;
    (*handler).signal_id = 0 as guint;
    (*handler).signal_detail = 0 as GQuark;
    let mut _pp: *mut *mut GClosure = &raw mut (*handler).closure;
    let mut _ptr: *mut GClosure = *_pp;
    *_pp = ::core::ptr::null_mut::<GClosure>();
    if !_ptr.is_null() {
        g_closure_unref(_ptr as *mut GClosure);
    }
    g_slice_free1(
        ::core::mem::size_of::<SignalHandler>() as gsize,
        handler as gpointer,
    );
}
unsafe extern "C" fn g_signal_group_constructed(mut object: *mut GObject) {
    let mut self_0: *mut GSignalGroup = object as *mut GSignalGroup;
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    target = g_weak_ref_get(&raw mut (*self_0).target_ref) as *mut GObject;
    if g_signal_group_check_target_type(self_0, target as gpointer) == 0 {
        g_signal_group_set_target(self_0, ::core::ptr::null_mut::<::core::ffi::c_void>());
    }
    (*(g_signal_group_parent_class as *mut GObjectClass))
        .constructed
        .expect("non-null function pointer")(object);
    let mut _pp: *mut *mut GObject = &raw mut target;
    let mut _ptr: *mut GObject = *_pp;
    *_pp = ::core::ptr::null_mut::<GObject>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
}
unsafe extern "C" fn g_signal_group_dispose(mut object: *mut GObject) {
    let mut self_0: *mut GSignalGroup = object as *mut GSignalGroup;
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    g_signal_group_gc_handlers(self_0);
    if (*self_0).has_bound_at_least_once() != 0 {
        g_signal_group_unbind(self_0);
    }
    let mut _pp: *mut *mut GPtrArray = &raw mut (*self_0).handlers;
    let mut _ptr: *mut GPtrArray = *_pp;
    *_pp = ::core::ptr::null_mut::<GPtrArray>();
    if !_ptr.is_null() {
        g_ptr_array_unref(_ptr as *mut GPtrArray);
    }
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
    (*(g_signal_group_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn g_signal_group_finalize(mut object: *mut GObject) {
    let mut self_0: *mut GSignalGroup = object as *mut GSignalGroup;
    g_weak_ref_clear(&raw mut (*self_0).target_ref);
    g_rec_mutex_clear(&raw mut (*self_0).mutex);
    (*(g_signal_group_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn g_signal_group_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GSignalGroup = object as *mut ::core::ffi::c_void as *mut GSignalGroup;
    match prop_id as GSignalGroupProperty as ::core::ffi::c_uint {
        1 => {
            g_value_take_object(value, g_signal_group_dup_target(self_0));
        }
        2 => {
            g_value_set_gtype(value, (*self_0).target_type);
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
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                567 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn g_signal_group_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GSignalGroup = object as *mut ::core::ffi::c_void as *mut GSignalGroup;
    match prop_id as GSignalGroupProperty as ::core::ffi::c_uint {
        1 => {
            g_signal_group_set_target(self_0, g_value_get_object(value));
        }
        2 => {
            g_signal_group_set_target_type(self_0, g_value_get_gtype(value));
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
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gsignalgroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn g_signal_group_class_init(mut klass: *mut GSignalGroupClass) {
    let mut object_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).constructed =
        Some(g_signal_group_constructed as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).dispose =
        Some(g_signal_group_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).finalize =
        Some(g_signal_group_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).get_property = Some(
        g_signal_group_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        g_signal_group_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    properties[PROP_TARGET as ::core::ffi::c_int as usize] = g_param_spec_object(
        b"target\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        (G_PARAM_READWRITE as ::core::ffi::c_int
            | G_PARAM_EXPLICIT_NOTIFY as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
    );
    properties[PROP_TARGET_TYPE as ::core::ffi::c_int as usize] = g_param_spec_gtype(
        b"target-type\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        (G_PARAM_READWRITE as ::core::ffi::c_int
            | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
    );
    g_object_class_install_properties(
        object_class,
        LAST_PROP as ::core::ffi::c_int as guint,
        &raw mut properties as *mut *mut GParamSpec,
    );
    signals[BIND as ::core::ffi::c_int as usize] = g_signal_new(
        b"bind\0" as *const u8 as *const gchar,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        0 as guint,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
        ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        1 as guint,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    );
    signals[UNBIND as ::core::ffi::c_int as usize] = g_signal_new(
        b"unbind\0" as *const u8 as *const gchar,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        0 as guint,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
        ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        0 as guint,
    );
}
unsafe extern "C" fn g_signal_group_init(mut self_0: *mut GSignalGroup) {
    g_rec_mutex_init(&raw mut (*self_0).mutex);
    (*self_0).handlers = g_ptr_array_new_with_free_func(Some(
        signal_handler_free as unsafe extern "C" fn(gpointer) -> (),
    ));
    (*self_0).target_type = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_new(mut target_type: GType) -> *mut GSignalGroup {
    if target_type == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        || g_type_is_a(
            target_type,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_new\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_is_a (target_type, G_TYPE_OBJECT)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSignalGroup>();
    }
    return g_object_new(
        g_signal_group_get_type(),
        b"target-type\0" as *const u8 as *const gchar,
        target_type,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GSignalGroup;
}
unsafe extern "C" fn g_signal_group_connect_closure_(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut closure: *mut GClosure,
    mut after: gboolean,
) -> gboolean {
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut handler: *mut SignalHandler = ::core::ptr::null_mut::<SignalHandler>();
    let mut signal_id: guint = 0;
    let mut signal_detail: GQuark = 0;
    if ({
        let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
        let mut __t: GType = g_signal_group_get_type();
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
            b"g_signal_group_connect_closure_\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_SIGNAL_GROUP (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !detailed_signal.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_connect_closure_\0" as *const u8 as *const ::core::ffi::c_char,
            b"detailed_signal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_connect_closure_\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_signal_parse_name(
        detailed_signal,
        (*self_0).target_type,
        &raw mut signal_id,
        &raw mut signal_detail,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    ) == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Invalid signal name \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8 as *const gchar,
            detailed_signal,
        );
        return 0 as gboolean;
    }
    g_rec_mutex_lock(&raw mut (*self_0).mutex);
    if (*self_0).has_bound_at_least_once() != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Cannot add signals after setting target\0" as *const u8 as *const gchar,
        );
        g_rec_mutex_unlock(&raw mut (*self_0).mutex);
        return 0 as gboolean;
    }
    handler =
        g_slice_alloc0(::core::mem::size_of::<SignalHandler>() as gsize) as *mut SignalHandler;
    (*handler).group = self_0;
    (*handler).signal_id = signal_id;
    (*handler).signal_detail = signal_detail;
    (*handler).closure = g_closure_ref(closure);
    (*handler).set_connect_after(after as guint as guint);
    g_closure_sink(closure);
    g_ptr_array_add((*self_0).handlers, handler as gpointer);
    target = g_weak_ref_get(&raw mut (*self_0).target_ref) as *mut GObject;
    if !target.is_null() {
        g_signal_group_bind_handler(self_0, handler, target);
        g_object_unref(target as gpointer);
    }
    g_signal_group_gc_handlers(self_0);
    g_rec_mutex_unlock(&raw mut (*self_0).mutex);
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_connect_closure(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut closure: *mut GClosure,
    mut after: gboolean,
) {
    g_signal_group_connect_closure_(self_0, detailed_signal, closure, after);
}
unsafe extern "C" fn g_signal_group_connect_full(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut data: gpointer,
    mut notify: GClosureNotify,
    mut flags: GConnectFlags,
    mut is_object: gboolean,
) {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if c_handler.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_connect_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"c_handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if is_object == 0
        || g_type_check_instance_is_fundamentally_a(
            data as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_connect_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"!is_object || G_IS_OBJECT (data)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if flags as ::core::ffi::c_uint & G_CONNECT_SWAPPED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        closure = g_cclosure_new_swap(c_handler, data, notify);
    } else {
        closure = g_cclosure_new(c_handler, data, notify);
    }
    if is_object != 0 {
        g_object_watch_closure(data as *mut GObject, closure);
    }
    if g_signal_group_connect_closure_(
        self_0,
        detailed_signal,
        closure,
        (flags as ::core::ffi::c_uint
            & G_CONNECT_AFTER as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
    ) == 0
    {
        g_closure_unref(closure);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_connect_object(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut object: gpointer,
    mut flags: GConnectFlags,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_group_connect_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_signal_group_connect_full(
        self_0,
        detailed_signal,
        c_handler,
        object,
        None,
        flags,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_connect_data(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut data: gpointer,
    mut notify: GClosureNotify,
    mut flags: GConnectFlags,
) {
    g_signal_group_connect_full(
        self_0,
        detailed_signal,
        c_handler,
        data,
        notify,
        flags,
        0 as gboolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_connect(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut data: gpointer,
) {
    g_signal_group_connect_full(
        self_0,
        detailed_signal,
        c_handler,
        data,
        None,
        G_CONNECT_DEFAULT,
        0 as gboolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_connect_after(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut data: gpointer,
) {
    g_signal_group_connect_full(
        self_0,
        detailed_signal,
        c_handler,
        data,
        None,
        G_CONNECT_AFTER,
        0 as gboolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_group_connect_swapped(
    mut self_0: *mut GSignalGroup,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut data: gpointer,
) {
    g_signal_group_connect_full(
        self_0,
        detailed_signal,
        c_handler,
        data,
        None,
        G_CONNECT_SWAPPED,
        0 as gboolean,
    );
}
