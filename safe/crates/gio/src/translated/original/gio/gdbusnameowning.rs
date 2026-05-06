use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GDBusConnection;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_error_free(error: *mut GError);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
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
    fn g_cclosure_marshal_generic(
        closure: *mut GClosure,
        return_gvalue: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_unique_name(string: *const gchar) -> gboolean;
    fn g_dbus_connection_get_type() -> GType;
    fn g_bus_get(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_bus_get_finish(res: *mut GAsyncResult, error: *mut *mut GError) -> *mut GDBusConnection;
    fn g_dbus_connection_is_closed(connection: *mut GDBusConnection) -> gboolean;
    fn g_dbus_connection_call(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_finish(
        connection: *mut GDBusConnection,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_call_sync(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_signal_subscribe(
        connection: *mut GDBusConnection,
        sender: *const gchar,
        interface_name: *const gchar,
        member: *const gchar,
        object_path: *const gchar,
        arg0: *const gchar,
        flags: GDBusSignalFlags,
        callback: GDBusSignalCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_dbus_connection_signal_unsubscribe(
        connection: *mut GDBusConnection,
        subscription_id: guint,
    );
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
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
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
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
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GBusNameOwnerFlags = ::core::ffi::c_uint;
pub const G_BUS_NAME_OWNER_FLAGS_DO_NOT_QUEUE: GBusNameOwnerFlags = 4;
pub const G_BUS_NAME_OWNER_FLAGS_REPLACE: GBusNameOwnerFlags = 2;
pub const G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT: GBusNameOwnerFlags = 1;
pub const G_BUS_NAME_OWNER_FLAGS_NONE: GBusNameOwnerFlags = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusConnection = _GDBusConnection;
pub type GBusAcquiredCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> ()>;
pub type GBusNameAcquiredCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> ()>;
pub type GBusNameLostCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Client {
    pub ref_count: gint,
    pub id: guint,
    pub flags: GBusNameOwnerFlags,
    pub name: *mut gchar,
    pub bus_acquired_handler: GBusAcquiredCallback,
    pub name_acquired_handler: GBusNameAcquiredCallback,
    pub name_lost_handler: GBusNameLostCallback,
    pub user_data: gpointer,
    pub user_data_free_func: GDestroyNotify,
    pub main_context: *mut GMainContext,
    pub previous_call: PreviousCall,
    pub connection: *mut GDBusConnection,
    pub disconnected_signal_handler_id: gulong,
    pub name_acquired_subscription_id: guint,
    pub name_lost_subscription_id: guint,
    pub cancelled: gboolean,
    pub needs_release: gboolean,
}
pub type PreviousCall = ::core::ffi::c_uint;
pub const PREVIOUS_CALL_LOST: PreviousCall = 2;
pub const PREVIOUS_CALL_ACQUIRED: PreviousCall = 1;
pub const PREVIOUS_CALL_NONE: PreviousCall = 0;
pub type CallType = ::core::ffi::c_uint;
pub const CALL_TYPE_NAME_LOST: CallType = 1;
pub const CALL_TYPE_NAME_ACQUIRED: CallType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallHandlerData {
    pub client: *mut Client,
    pub connection: *mut GDBusConnection,
    pub call_type: CallType,
}
pub type GDBusSignalCallback = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        gpointer,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OwnNameData {
    pub bus_acquired_closure: *mut GClosure,
    pub name_acquired_closure: *mut GClosure,
    pub name_lost_closure: *mut GClosure,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_PRIORITY_HIGH: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_g__lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_next_global_id: guint = 1 as guint;
static mut safe_c2rust_map_id_to_client: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
unsafe extern "C" fn safe_c2rust_client_ref(mut client: *mut Client) -> *mut Client {
    if 0 as ::core::ffi::c_int != 0 {
        (*client).ref_count;
        (*client).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*client).ref_count, 1 as ::core::ffi::c_int);
    return client;
}
unsafe extern "C" fn safe_c2rust_client_unref(mut client: *mut Client) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*client).ref_count;
            (*client).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*client).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        if !(*client).connection.is_null() {
            if (*client).disconnected_signal_handler_id > 0 as gulong {
                g_signal_handler_disconnect(
                    (*client).connection as gpointer,
                    (*client).disconnected_signal_handler_id,
                );
            }
            if (*client).name_acquired_subscription_id > 0 as guint {
                g_dbus_connection_signal_unsubscribe(
                    (*client).connection,
                    (*client).name_acquired_subscription_id,
                );
            }
            if (*client).name_lost_subscription_id > 0 as guint {
                g_dbus_connection_signal_unsubscribe(
                    (*client).connection,
                    (*client).name_lost_subscription_id,
                );
            }
            g_object_unref((*client).connection as gpointer);
        }
        g_main_context_unref((*client).main_context);
        g_free((*client).name as gpointer);
        if (*client).user_data_free_func.is_some() {
            (*client)
                .user_data_free_func
                .expect("non-null function pointer")((*client).user_data);
        }
        g_free(client as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_call_handler_data_free(mut data: *mut CallHandlerData) {
    if !(*data).connection.is_null() {
        g_object_unref((*data).connection as gpointer);
    }
    safe_c2rust_client_unref((*data).client);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_actually_do_call(
    mut client: *mut Client,
    mut connection: *mut GDBusConnection,
    mut call_type: CallType,
) {
    match call_type as ::core::ffi::c_uint {
        0 => {
            if (*client).name_acquired_handler.is_some() {
                (*client)
                    .name_acquired_handler
                    .expect("non-null function pointer")(
                    connection,
                    (*client).name,
                    (*client).user_data,
                );
            }
        }
        1 => {
            if (*client).name_lost_handler.is_some() {
                (*client)
                    .name_lost_handler
                    .expect("non-null function pointer")(
                    connection,
                    (*client).name,
                    (*client).user_data,
                );
            }
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnameowning.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_call_in_idle_cb(mut _data: gpointer) -> gboolean {
    let mut data: *mut CallHandlerData = _data as *mut CallHandlerData;
    safe_c2rust_actually_do_call((*data).client, (*data).connection, (*data).call_type);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_schedule_call_in_idle(
    mut client: *mut Client,
    mut call_type: CallType,
) {
    let mut data: *mut CallHandlerData = ::core::ptr::null_mut::<CallHandlerData>();
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<CallHandlerData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut CallHandlerData;
    (*data).client = safe_c2rust_client_ref(client);
    (*data).connection = (if !(*client).connection.is_null() {
        g_object_ref((*client).connection as gpointer) as *mut GDBusConnection
    } else {
        ::core::ptr::null_mut::<GDBusConnection>()
    }) as *mut GDBusConnection;
    (*data).call_type = call_type;
    idle_source = g_idle_source_new();
    g_source_set_priority(idle_source, G_PRIORITY_HIGH);
    g_source_set_callback(
        idle_source,
        Some(safe_c2rust_call_in_idle_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut CallHandlerData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_call_handler_data_free as unsafe extern "C" fn(*mut CallHandlerData) -> (),
        )),
    );
    g_source_set_static_name(
        idle_source,
        b"[gio, gdbusnameowning.c] call_in_idle_cb\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(idle_source, (*client).main_context);
    g_source_unref(idle_source);
}
unsafe extern "C" fn safe_c2rust_do_call(mut client: *mut Client, mut call_type: CallType) {
    let mut current_context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    current_context = g_main_context_ref_thread_default();
    if current_context != (*client).main_context {
        safe_c2rust_schedule_call_in_idle(client, call_type);
    } else {
        safe_c2rust_actually_do_call(client, (*client).connection, call_type);
    }
    g_main_context_unref(current_context);
}
unsafe extern "C" fn safe_c2rust_call_acquired_handler(mut client: *mut Client) {
    let mut current_block: u64;
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    if (*client).previous_call as ::core::ffi::c_uint
        != PREVIOUS_CALL_ACQUIRED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*client).previous_call = PREVIOUS_CALL_ACQUIRED;
        if (*client).cancelled == 0 {
            g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
            safe_c2rust_do_call(client, CALL_TYPE_NAME_ACQUIRED);
            current_block = 2473556513754201174;
        } else {
            current_block = 820271813250567934;
        }
    } else {
        current_block = 820271813250567934;
    }
    match current_block {
        820271813250567934 => {
            g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
        }
        _ => {}
    };
}
unsafe extern "C" fn safe_c2rust_call_lost_handler(mut client: *mut Client) {
    let mut current_block: u64;
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    if (*client).previous_call as ::core::ffi::c_uint
        != PREVIOUS_CALL_LOST as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*client).previous_call = PREVIOUS_CALL_LOST;
        if (*client).cancelled == 0 {
            g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
            safe_c2rust_do_call(client, CALL_TYPE_NAME_LOST);
            current_block = 2473556513754201174;
        } else {
            current_block = 820271813250567934;
        }
    } else {
        current_block = 820271813250567934;
    }
    match current_block {
        820271813250567934 => {
            g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
        }
        _ => {}
    };
}
unsafe extern "C" fn safe_c2rust_on_name_lost_or_acquired(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut client: *mut Client = user_data as *mut Client;
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    if !(g_strcmp0(
        object_path as *const ::core::ffi::c_char,
        b"/org/freedesktop/DBus\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        || g_strcmp0(
            interface_name as *const ::core::ffi::c_char,
            b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || g_strcmp0(
            sender_name as *const ::core::ffi::c_char,
            b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int)
    {
        if g_variant_is_of_type(
            parameters,
            g_variant_type_checked_(b"(s)\0" as *const u8 as *const gchar),
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s signal had unexpected signature %s\0" as *const u8 as *const gchar,
                signal_name,
                g_variant_get_type_string(parameters),
            );
        } else if g_strcmp0(
            signal_name as *const ::core::ffi::c_char,
            b"NameLost\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            g_variant_get(
                parameters,
                b"(&s)\0" as *const u8 as *const gchar,
                &raw mut name,
            );
            if g_strcmp0(name as *const ::core::ffi::c_char, (*client).name)
                == 0 as ::core::ffi::c_int
            {
                safe_c2rust_call_lost_handler(client);
            }
        } else if g_strcmp0(
            signal_name as *const ::core::ffi::c_char,
            b"NameAcquired\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            g_variant_get(
                parameters,
                b"(&s)\0" as *const u8 as *const gchar,
                &raw mut name,
            );
            if g_strcmp0(name as *const ::core::ffi::c_char, (*client).name)
                == 0 as ::core::ffi::c_int
            {
                safe_c2rust_call_acquired_handler(client);
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_request_name_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut client: *mut Client = user_data as *mut Client;
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut request_name_reply: guint32 = 0;
    let mut unsubscribe: gboolean = 0;
    request_name_reply = 0 as guint32;
    result = ::core::ptr::null_mut::<GVariant>();
    result = g_dbus_connection_call_finish(
        source_object as *mut ::core::ffi::c_void as *mut GDBusConnection,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !result.is_null() {
        g_variant_get(
            result,
            b"(u)\0" as *const u8 as *const gchar,
            &raw mut request_name_reply,
        );
        g_variant_unref(result);
    }
    unsubscribe = FALSE as gboolean;
    match request_name_reply {
        1 => {
            safe_c2rust_call_acquired_handler(client);
        }
        2 => {
            safe_c2rust_call_lost_handler(client);
        }
        3 | 4 | _ => {
            safe_c2rust_call_lost_handler(client);
            unsubscribe = TRUE as gboolean;
            (*client).needs_release = FALSE as gboolean;
        }
    }
    if unsubscribe != 0 {
        let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
        g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
        if (*client).cancelled == 0 {
            connection = g_object_ref((*client).connection as gpointer) as *mut GDBusConnection
                as *mut GDBusConnection;
        }
        g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
        if !connection.is_null() {
            if (*client).name_acquired_subscription_id > 0 as guint {
                g_dbus_connection_signal_unsubscribe(
                    (*client).connection,
                    (*client).name_acquired_subscription_id,
                );
            }
            if (*client).name_lost_subscription_id > 0 as guint {
                g_dbus_connection_signal_unsubscribe(
                    (*client).connection,
                    (*client).name_lost_subscription_id,
                );
            }
            (*client).name_acquired_subscription_id = 0 as guint;
            (*client).name_lost_subscription_id = 0 as guint;
            g_object_unref(connection as gpointer);
        }
    }
    safe_c2rust_client_unref(client);
}
unsafe extern "C" fn safe_c2rust_on_connection_disconnected(
    mut connection: *mut GDBusConnection,
    mut remote_peer_vanished: gboolean,
    mut error: *mut GError,
    mut user_data: gpointer,
) {
    let mut client: *mut Client = user_data as *mut Client;
    if (*client).disconnected_signal_handler_id > 0 as gulong {
        g_signal_handler_disconnect(
            (*client).connection as gpointer,
            (*client).disconnected_signal_handler_id,
        );
    }
    if (*client).name_acquired_subscription_id > 0 as guint {
        g_dbus_connection_signal_unsubscribe(
            (*client).connection,
            (*client).name_acquired_subscription_id,
        );
    }
    if (*client).name_lost_subscription_id > 0 as guint {
        g_dbus_connection_signal_unsubscribe(
            (*client).connection,
            (*client).name_lost_subscription_id,
        );
    }
    g_object_unref((*client).connection as gpointer);
    (*client).disconnected_signal_handler_id = 0 as gulong;
    (*client).name_acquired_subscription_id = 0 as guint;
    (*client).name_lost_subscription_id = 0 as guint;
    (*client).connection = ::core::ptr::null_mut::<GDBusConnection>();
    safe_c2rust_call_lost_handler(client);
}
unsafe extern "C" fn safe_c2rust_has_connection(mut client: *mut Client) {
    (*client).disconnected_signal_handler_id = g_signal_connect_data(
        (*client).connection as gpointer,
        b"closed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GDBusConnection, gboolean, *mut GError, gpointer) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_on_connection_disconnected
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    gboolean,
                    *mut GError,
                    gpointer,
                ) -> (),
        )),
        client as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*client).name_lost_subscription_id = g_dbus_connection_signal_subscribe(
        (*client).connection,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"NameLost\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        (*client).name,
        G_DBUS_SIGNAL_FLAGS_NONE,
        Some(
            safe_c2rust_on_name_lost_or_acquired
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    gpointer,
                ) -> (),
        ),
        safe_c2rust_client_ref(client) as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut Client) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_client_unref as unsafe extern "C" fn(*mut Client) -> ()),
        ),
    );
    (*client).name_acquired_subscription_id = g_dbus_connection_signal_subscribe(
        (*client).connection,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"NameAcquired\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        (*client).name,
        G_DBUS_SIGNAL_FLAGS_NONE,
        Some(
            safe_c2rust_on_name_lost_or_acquired
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    gpointer,
                ) -> (),
        ),
        safe_c2rust_client_ref(client) as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut Client) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_client_unref as unsafe extern "C" fn(*mut Client) -> ()),
        ),
    );
    (*client).needs_release = TRUE as gboolean;
    g_dbus_connection_call(
        (*client).connection,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"RequestName\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(su)\0" as *const u8 as *const gchar,
            (*client).name,
            (*client).flags as ::core::ffi::c_uint,
        ),
        g_variant_type_checked_(b"(u)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_request_name_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        )),
        safe_c2rust_client_ref(client) as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_connection_get_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut client: *mut Client = user_data as *mut Client;
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    if (*client).cancelled != 0 {
        g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
    } else {
        g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
        (*client).connection = g_bus_get_finish(res, ::core::ptr::null_mut::<*mut GError>());
        if (*client).connection.is_null() {
            safe_c2rust_call_lost_handler(client);
        } else {
            if (*client).bus_acquired_handler.is_some() {
                (*client)
                    .bus_acquired_handler
                    .expect("non-null function pointer")(
                    (*client).connection,
                    (*client).name,
                    (*client).user_data,
                );
            }
            safe_c2rust_has_connection(client);
        }
    }
    safe_c2rust_client_unref(client);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_own_name_on_connection(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut flags: GBusNameOwnerFlags,
    mut name_acquired_handler: GBusNameAcquiredCallback,
    mut name_lost_handler: GBusNameLostCallback,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
) -> guint {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 && g_dbus_is_unique_name(name) == 0 {
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
            b"g_dbus_is_name (name) && !g_dbus_is_unique_name (name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    client = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<Client>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut Client;
    (*client).ref_count = 1 as ::core::ffi::c_int as gint;
    let fresh1 = safe_c2rust_next_global_id;
    safe_c2rust_next_global_id = safe_c2rust_next_global_id.wrapping_add(1);
    (*client).id = fresh1;
    (*client).name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*client).flags = flags;
    (*client).name_acquired_handler = name_acquired_handler;
    (*client).name_lost_handler = name_lost_handler;
    (*client).user_data = user_data;
    (*client).user_data_free_func = user_data_free_func;
    (*client).main_context = g_main_context_ref_thread_default();
    (*client).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    if safe_c2rust_map_id_to_client.is_null() {
        safe_c2rust_map_id_to_client = g_hash_table_new(
            Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
    }
    g_hash_table_insert(
        safe_c2rust_map_id_to_client,
        (*client).id as gulong as gpointer,
        client as gpointer,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
    safe_c2rust_has_connection(client);
    return (*client).id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_own_name(
    mut bus_type: GBusType,
    mut name: *const gchar,
    mut flags: GBusNameOwnerFlags,
    mut bus_acquired_handler: GBusAcquiredCallback,
    mut name_acquired_handler: GBusNameAcquiredCallback,
    mut name_lost_handler: GBusNameLostCallback,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
) -> guint {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 && g_dbus_is_unique_name(name) == 0 {
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
            b"g_dbus_is_name (name) && !g_dbus_is_unique_name (name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    client = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<Client>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut Client;
    (*client).ref_count = 1 as ::core::ffi::c_int as gint;
    let fresh0 = safe_c2rust_next_global_id;
    safe_c2rust_next_global_id = safe_c2rust_next_global_id.wrapping_add(1);
    (*client).id = fresh0;
    (*client).name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*client).flags = flags;
    (*client).bus_acquired_handler = bus_acquired_handler;
    (*client).name_acquired_handler = name_acquired_handler;
    (*client).name_lost_handler = name_lost_handler;
    (*client).user_data = user_data;
    (*client).user_data_free_func = user_data_free_func;
    (*client).main_context = g_main_context_ref_thread_default();
    if safe_c2rust_map_id_to_client.is_null() {
        safe_c2rust_map_id_to_client = g_hash_table_new(
            Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
    }
    g_hash_table_insert(
        safe_c2rust_map_id_to_client,
        (*client).id as gulong as gpointer,
        client as gpointer,
    );
    g_bus_get(
        bus_type,
        ::core::ptr::null_mut::<GCancellable>(),
        Some(
            safe_c2rust_connection_get_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        safe_c2rust_client_ref(client) as gpointer,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
    return (*client).id;
}
unsafe extern "C" fn safe_c2rust_own_name_data_new(
    mut bus_acquired_closure: *mut GClosure,
    mut name_acquired_closure: *mut GClosure,
    mut name_lost_closure: *mut GClosure,
) -> *mut OwnNameData {
    let mut data: *mut OwnNameData = ::core::ptr::null_mut::<OwnNameData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<OwnNameData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut OwnNameData;
    if !bus_acquired_closure.is_null() {
        (*data).bus_acquired_closure = g_closure_ref(bus_acquired_closure);
        g_closure_sink(bus_acquired_closure);
        if (*bus_acquired_closure).marshal.is_none() {
            g_closure_set_marshal(
                bus_acquired_closure,
                Some(
                    g_cclosure_marshal_generic
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
    }
    if !name_acquired_closure.is_null() {
        (*data).name_acquired_closure = g_closure_ref(name_acquired_closure);
        g_closure_sink(name_acquired_closure);
        if (*name_acquired_closure).marshal.is_none() {
            g_closure_set_marshal(
                name_acquired_closure,
                Some(
                    g_cclosure_marshal_generic
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
    }
    if !name_lost_closure.is_null() {
        (*data).name_lost_closure = g_closure_ref(name_lost_closure);
        g_closure_sink(name_lost_closure);
        if (*name_lost_closure).marshal.is_none() {
            g_closure_set_marshal(
                name_lost_closure,
                Some(
                    g_cclosure_marshal_generic
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
    }
    return data;
}
unsafe extern "C" fn safe_c2rust_own_with_closures_on_bus_acquired(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut data: *mut OwnNameData = user_data as *mut OwnNameData;
    let mut params: [GValue; 2] = [
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
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        g_dbus_connection_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        connection as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        name,
    );
    g_closure_invoke(
        (*data).bus_acquired_closure,
        ::core::ptr::null_mut::<GValue>(),
        2 as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn safe_c2rust_own_with_closures_on_name_acquired(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut data: *mut OwnNameData = user_data as *mut OwnNameData;
    let mut params: [GValue; 2] = [
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
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        g_dbus_connection_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        connection as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        name,
    );
    g_closure_invoke(
        (*data).name_acquired_closure,
        ::core::ptr::null_mut::<GValue>(),
        2 as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn safe_c2rust_own_with_closures_on_name_lost(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut data: *mut OwnNameData = user_data as *mut OwnNameData;
    let mut params: [GValue; 2] = [
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
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        g_dbus_connection_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        connection as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        name,
    );
    g_closure_invoke(
        (*data).name_lost_closure,
        ::core::ptr::null_mut::<GValue>(),
        2 as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn safe_c2rust_bus_own_name_free_func(mut user_data: gpointer) {
    let mut data: *mut OwnNameData = user_data as *mut OwnNameData;
    if !(*data).bus_acquired_closure.is_null() {
        g_closure_unref((*data).bus_acquired_closure);
    }
    if !(*data).name_acquired_closure.is_null() {
        g_closure_unref((*data).name_acquired_closure);
    }
    if !(*data).name_lost_closure.is_null() {
        g_closure_unref((*data).name_lost_closure);
    }
    g_free(data as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_own_name_with_closures(
    mut bus_type: GBusType,
    mut name: *const gchar,
    mut flags: GBusNameOwnerFlags,
    mut bus_acquired_closure: *mut GClosure,
    mut name_acquired_closure: *mut GClosure,
    mut name_lost_closure: *mut GClosure,
) -> guint {
    return safe_c2rust_g_bus_own_name(
        bus_type,
        name,
        flags,
        if !bus_acquired_closure.is_null() {
            Some(
                safe_c2rust_own_with_closures_on_bus_acquired
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            )
        } else {
            None
        },
        if !name_acquired_closure.is_null() {
            Some(
                safe_c2rust_own_with_closures_on_name_acquired
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            )
        } else {
            None
        },
        if !name_lost_closure.is_null() {
            Some(
                safe_c2rust_own_with_closures_on_name_lost
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            )
        } else {
            None
        },
        safe_c2rust_own_name_data_new(
            bus_acquired_closure,
            name_acquired_closure,
            name_lost_closure,
        ) as gpointer,
        Some(safe_c2rust_bus_own_name_free_func as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_own_name_on_connection_with_closures(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut flags: GBusNameOwnerFlags,
    mut name_acquired_closure: *mut GClosure,
    mut name_lost_closure: *mut GClosure,
) -> guint {
    return safe_c2rust_g_bus_own_name_on_connection(
        connection,
        name,
        flags,
        if !name_acquired_closure.is_null() {
            Some(
                safe_c2rust_own_with_closures_on_name_acquired
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            )
        } else {
            None
        },
        if !name_lost_closure.is_null() {
            Some(
                safe_c2rust_own_with_closures_on_name_lost
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            )
        } else {
            None
        },
        safe_c2rust_own_name_data_new(
            ::core::ptr::null_mut::<GClosure>(),
            name_acquired_closure,
            name_lost_closure,
        ) as gpointer,
        Some(safe_c2rust_bus_own_name_free_func as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_unown_name(mut owner_id: guint) {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if owner_id > 0 as guint {
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
            b"owner_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    client = ::core::ptr::null_mut::<Client>();
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    if owner_id == 0 as guint || safe_c2rust_map_id_to_client.is_null() || {
        client = g_hash_table_lookup(
            safe_c2rust_map_id_to_client,
            owner_id as gulong as gpointer as gconstpointer,
        ) as *mut Client;
        client.is_null()
    } {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Invalid id %d passed to g_bus_unown_name()\0" as *const u8 as *const gchar,
            owner_id,
        );
    } else {
        (*client).cancelled = TRUE as gboolean;
        if !(({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                safe_c2rust_map_id_to_client,
                owner_id as gulong as gpointer as gconstpointer,
            ) != 0
            {
                _g_boolean_var_14 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_14 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_14
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnameowning.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                914 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (map_id_to_client, GUINT_TO_POINTER (owner_id))\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
    if !client.is_null() {
        if (*client).needs_release != 0
            && !(*client).connection.is_null()
            && g_dbus_connection_is_closed((*client).connection) == 0
        {
            let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
            let mut release_name_reply: guint32 = 0;
            error = ::core::ptr::null_mut::<GError>();
            result = g_dbus_connection_call_sync(
                (*client).connection,
                b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
                b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
                b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
                b"ReleaseName\0" as *const u8 as *const gchar,
                g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*client).name),
                g_variant_type_checked_(b"(u)\0" as *const u8 as *const gchar),
                G_DBUS_CALL_FLAGS_NONE,
                -(1 as gint),
                ::core::ptr::null_mut::<GCancellable>(),
                &raw mut error,
            );
            if result.is_null() {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Error releasing name %s: %s\0" as *const u8 as *const gchar,
                    (*client).name,
                    (*error).message,
                );
                g_error_free(error);
            } else {
                g_variant_get(
                    result,
                    b"(u)\0" as *const u8 as *const gchar,
                    &raw mut release_name_reply,
                );
                if release_name_reply != 1 as guint32 {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"Unexpected reply %d when releasing name %s\0" as *const u8
                            as *const gchar,
                        release_name_reply,
                        (*client).name,
                    );
                } else {
                    (*client).needs_release = FALSE as gboolean;
                }
                g_variant_unref(result);
            }
        }
        if (*client).disconnected_signal_handler_id > 0 as gulong {
            g_signal_handler_disconnect(
                (*client).connection as gpointer,
                (*client).disconnected_signal_handler_id,
            );
        }
        if (*client).name_acquired_subscription_id > 0 as guint {
            g_dbus_connection_signal_unsubscribe(
                (*client).connection,
                (*client).name_acquired_subscription_id,
            );
        }
        if (*client).name_lost_subscription_id > 0 as guint {
            g_dbus_connection_signal_unsubscribe(
                (*client).connection,
                (*client).name_lost_subscription_id,
            );
        }
        (*client).disconnected_signal_handler_id = 0 as gulong;
        (*client).name_acquired_subscription_id = 0 as guint;
        (*client).name_lost_subscription_id = 0 as guint;
        if !(*client).connection.is_null() {
            g_object_unref((*client).connection as gpointer);
            (*client).connection = ::core::ptr::null_mut::<GDBusConnection>();
        }
        safe_c2rust_client_unref(client);
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
