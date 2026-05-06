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
    fn g_main_context_get_thread_default() -> *mut GMainContext;
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
    fn g_source_set_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
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
    fn g_dbus_connection_get_type() -> GType;
    fn g_bus_get(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_bus_get_finish(res: *mut GAsyncResult, error: *mut *mut GError) -> *mut GDBusConnection;
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
pub type GBusNameWatcherFlags = ::core::ffi::c_uint;
pub const G_BUS_NAME_WATCHER_FLAGS_AUTO_START: GBusNameWatcherFlags = 1;
pub const G_BUS_NAME_WATCHER_FLAGS_NONE: GBusNameWatcherFlags = 0;
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
pub type GBusNameAppearedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, *const gchar, gpointer) -> ()>;
pub type GBusNameVanishedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Client {
    pub ref_count: gint,
    pub id: guint,
    pub name: *mut gchar,
    pub flags: GBusNameWatcherFlags,
    pub name_owner: *mut gchar,
    pub name_appeared_handler: GBusNameAppearedCallback,
    pub name_vanished_handler: GBusNameVanishedCallback,
    pub user_data: gpointer,
    pub user_data_free_func: GDestroyNotify,
    pub main_context: *mut GMainContext,
    pub connection: *mut GDBusConnection,
    pub disconnected_signal_handler_id: gulong,
    pub name_owner_changed_subscription_id: guint,
    pub previous_call: PreviousCall,
    pub cancelled: gboolean,
    pub initialized: gboolean,
}
pub type PreviousCall = ::core::ffi::c_uint;
pub const PREVIOUS_CALL_VANISHED: PreviousCall = 2;
pub const PREVIOUS_CALL_APPEARED: PreviousCall = 1;
pub const PREVIOUS_CALL_NONE: PreviousCall = 0;
pub type CallType = ::core::ffi::c_uint;
pub const CALL_TYPE_NAME_VANISHED: CallType = 1;
pub const CALL_TYPE_NAME_APPEARED: CallType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallHandlerData {
    pub client: *mut Client,
    pub connection: *mut GDBusConnection,
    pub name_owner: *mut gchar,
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
pub struct WatchNameData {
    pub name_appeared_closure: *mut GClosure,
    pub name_vanished_closure: *mut GClosure,
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
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
unsafe extern "C" fn safe_c2rust_free_user_data_cb(mut user_data: gpointer) -> gboolean {
    return G_SOURCE_REMOVE;
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
            if (*client).name_owner_changed_subscription_id > 0 as guint {
                g_dbus_connection_signal_unsubscribe(
                    (*client).connection,
                    (*client).name_owner_changed_subscription_id,
                );
            }
            if (*client).disconnected_signal_handler_id > 0 as gulong {
                g_signal_handler_disconnect(
                    (*client).connection as gpointer,
                    (*client).disconnected_signal_handler_id,
                );
            }
            g_object_unref((*client).connection as gpointer);
        }
        g_free((*client).name as gpointer);
        g_free((*client).name_owner as gpointer);
        if (*client).user_data_free_func.is_some() {
            if (*client).main_context != g_main_context_get_thread_default() {
                let mut idle_source: *mut GSource = g_idle_source_new();
                g_source_set_callback(
                    idle_source,
                    Some(
                        safe_c2rust_free_user_data_cb as unsafe extern "C" fn(gpointer) -> gboolean,
                    ),
                    (*client).user_data,
                    (*client).user_data_free_func,
                );
                g_source_set_name(
                    idle_source,
                    b"[gio, gdbusnamewatching.c] free_user_data_cb\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                g_source_attach(idle_source, (*client).main_context);
                g_source_unref(idle_source);
            } else {
                (*client)
                    .user_data_free_func
                    .expect("non-null function pointer")((*client).user_data);
            }
        }
        g_main_context_unref((*client).main_context);
        g_free(client as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_call_handler_data_free(mut data: *mut CallHandlerData) {
    if !(*data).connection.is_null() {
        g_object_unref((*data).connection as gpointer);
    }
    g_free((*data).name_owner as gpointer);
    safe_c2rust_client_unref((*data).client);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_actually_do_call(
    mut client: *mut Client,
    mut connection: *mut GDBusConnection,
    mut name_owner: *const gchar,
    mut call_type: CallType,
) {
    if (*client).cancelled != 0 {
        return;
    }
    match call_type as ::core::ffi::c_uint {
        0 => {
            if (*client).name_appeared_handler.is_some() {
                (*client)
                    .name_appeared_handler
                    .expect("non-null function pointer")(
                    connection,
                    (*client).name,
                    name_owner,
                    (*client).user_data,
                );
            }
        }
        1 => {
            if (*client).name_vanished_handler.is_some() {
                (*client)
                    .name_vanished_handler
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnamewatching.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                192 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_call_in_idle_cb(mut _data: gpointer) -> gboolean {
    let mut data: *mut CallHandlerData = _data as *mut CallHandlerData;
    safe_c2rust_actually_do_call(
        (*data).client,
        (*data).connection,
        (*data).name_owner,
        (*data).call_type,
    );
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
    (*data).name_owner = safe_c2rust_g_strdup_inline((*client).name_owner) as *mut gchar;
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
        b"[gio, gdbusnamewatching.c] call_in_idle_cb\0" as *const u8 as *const ::core::ffi::c_char,
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
        safe_c2rust_actually_do_call(
            client,
            (*client).connection,
            (*client).name_owner,
            call_type,
        );
    }
    g_main_context_unref(current_context);
}
unsafe extern "C" fn safe_c2rust_call_appeared_handler(mut client: *mut Client) {
    if (*client).previous_call as ::core::ffi::c_uint
        != PREVIOUS_CALL_APPEARED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*client).previous_call = PREVIOUS_CALL_APPEARED;
        if (*client).cancelled == 0 && (*client).name_appeared_handler.is_some() {
            safe_c2rust_do_call(client, CALL_TYPE_NAME_APPEARED);
        }
    }
}
unsafe extern "C" fn safe_c2rust_call_vanished_handler(mut client: *mut Client) {
    if (*client).previous_call as ::core::ffi::c_uint
        != PREVIOUS_CALL_VANISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*client).previous_call = PREVIOUS_CALL_VANISHED;
        if (*client).cancelled == 0 && (*client).name_vanished_handler.is_some() {
            safe_c2rust_do_call(client, CALL_TYPE_NAME_VANISHED);
        }
    }
}
unsafe extern "C" fn safe_c2rust_dup_client(mut watcher_id: guint) -> *mut Client {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if watcher_id != 0 as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnamewatching.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            279 as ::core::ffi::c_int,
            G_STRFUNC,
            b"watcher_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !safe_c2rust_map_id_to_client.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnamewatching.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            280 as ::core::ffi::c_int,
            G_STRFUNC,
            b"map_id_to_client != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    client = g_hash_table_lookup(
        safe_c2rust_map_id_to_client,
        watcher_id as gulong as gpointer as gconstpointer,
    ) as *mut Client;
    if !client.is_null() {
        safe_c2rust_client_ref(client);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
    return client;
}
unsafe extern "C" fn safe_c2rust_on_connection_disconnected(
    mut connection: *mut GDBusConnection,
    mut remote_peer_vanished: gboolean,
    mut error: *mut GError,
    mut user_data: gpointer,
) {
    let mut watcher_id: guint = user_data as gulong as guint;
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    client = safe_c2rust_dup_client(watcher_id);
    if client.is_null() {
        return;
    }
    if (*client).name_owner_changed_subscription_id > 0 as guint {
        g_dbus_connection_signal_unsubscribe(
            (*client).connection,
            (*client).name_owner_changed_subscription_id,
        );
    }
    if (*client).disconnected_signal_handler_id > 0 as gulong {
        g_signal_handler_disconnect(
            (*client).connection as gpointer,
            (*client).disconnected_signal_handler_id,
        );
    }
    g_object_unref((*client).connection as gpointer);
    (*client).disconnected_signal_handler_id = 0 as gulong;
    (*client).name_owner_changed_subscription_id = 0 as guint;
    (*client).connection = ::core::ptr::null_mut::<GDBusConnection>();
    safe_c2rust_call_vanished_handler(client);
    safe_c2rust_client_unref(client);
}
unsafe extern "C" fn safe_c2rust_on_name_owner_changed(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut watcher_id: guint = user_data as gulong as guint;
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut old_owner: *const gchar = ::core::ptr::null::<gchar>();
    let mut new_owner: *const gchar = ::core::ptr::null::<gchar>();
    client = safe_c2rust_dup_client(watcher_id);
    if client.is_null() {
        return;
    }
    if !((*client).initialized == 0) {
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
            g_variant_get(
                parameters,
                b"(&s&s&s)\0" as *const u8 as *const gchar,
                &raw mut name,
                &raw mut old_owner,
                &raw mut new_owner,
            );
            if !(g_strcmp0(name as *const ::core::ffi::c_char, (*client).name)
                != 0 as ::core::ffi::c_int)
            {
                if !old_owner.is_null()
                    && strlen(old_owner as *const ::core::ffi::c_char) > 0 as size_t
                    && !(*client).name_owner.is_null()
                {
                    g_free((*client).name_owner as gpointer);
                    (*client).name_owner = ::core::ptr::null_mut::<gchar>();
                    safe_c2rust_call_vanished_handler(client);
                }
                if !new_owner.is_null()
                    && strlen(new_owner as *const ::core::ffi::c_char) > 0 as size_t
                {
                    if !(({
                        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                        if (*client).name_owner.is_null() {
                            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_12
                    }) as ::core::ffi::c_long
                        != 0)
                    {
                        g_warn_message(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnamewatching.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            371 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"client->name_owner == NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    g_free((*client).name_owner as gpointer);
                    (*client).name_owner =
                        safe_c2rust_g_strdup_inline(new_owner as *const ::core::ffi::c_char)
                            as *mut gchar;
                    safe_c2rust_call_appeared_handler(client);
                }
            }
        }
    }
    safe_c2rust_client_unref(client);
}
unsafe extern "C" fn safe_c2rust_get_name_owner_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut client: *mut Client = user_data as *mut Client;
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut name_owner: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    name_owner = ::core::ptr::null::<::core::ffi::c_char>();
    result = ::core::ptr::null_mut::<GVariant>();
    result = g_dbus_connection_call_finish(
        (*client).connection,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !result.is_null() {
        g_variant_get(
            result,
            b"(&s)\0" as *const u8 as *const gchar,
            &raw mut name_owner,
        );
    }
    if !name_owner.is_null() {
        if !(({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if (*client).name_owner.is_null() {
                _g_boolean_var_13 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_13 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_13
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnamewatching.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                405 as ::core::ffi::c_int,
                G_STRFUNC,
                b"client->name_owner == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*client).name_owner = safe_c2rust_g_strdup_inline(name_owner) as *mut gchar;
        safe_c2rust_call_appeared_handler(client);
    } else {
        safe_c2rust_call_vanished_handler(client);
    }
    (*client).initialized = TRUE as gboolean;
    if !result.is_null() {
        g_variant_unref(result);
    }
    safe_c2rust_client_unref(client);
}
unsafe extern "C" fn safe_c2rust_invoke_get_name_owner(mut client: *mut Client) {
    g_dbus_connection_call(
        (*client).connection,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"GetNameOwner\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*client).name),
        g_variant_type_checked_(b"(s)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_get_name_owner_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        )),
        safe_c2rust_client_ref(client) as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_start_service_by_name_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut client: *mut Client = user_data as *mut Client;
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    result = ::core::ptr::null_mut::<GVariant>();
    result = g_dbus_connection_call_finish(
        (*client).connection,
        res,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !result.is_null() {
        let mut start_service_result: guint32 = 0;
        g_variant_get(
            result,
            b"(u)\0" as *const u8 as *const gchar,
            &raw mut start_service_result,
        );
        if start_service_result == 1 as guint32 {
            safe_c2rust_invoke_get_name_owner(client);
        } else if start_service_result == 2 as guint32 {
            safe_c2rust_invoke_get_name_owner(client);
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Unexpected reply %d from StartServiceByName() method\0" as *const u8
                    as *const gchar,
                start_service_result,
            );
            safe_c2rust_call_vanished_handler(client);
            (*client).initialized = TRUE as gboolean;
        }
    } else {
        safe_c2rust_invoke_get_name_owner(client);
    }
    if !result.is_null() {
        g_variant_unref(result);
    }
    safe_c2rust_client_unref(client);
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
        (*client).id as gulong as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*client).name_owner_changed_subscription_id = g_dbus_connection_signal_subscribe(
        (*client).connection,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"NameOwnerChanged\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        (*client).name,
        G_DBUS_SIGNAL_FLAGS_NONE,
        Some(
            safe_c2rust_on_name_owner_changed
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
        (*client).id as gulong as gpointer,
        None,
    );
    if (*client).flags as ::core::ffi::c_uint
        & G_BUS_NAME_WATCHER_FLAGS_AUTO_START as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        g_dbus_connection_call(
            (*client).connection,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"StartServiceByName\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(su)\0" as *const u8 as *const gchar,
                (*client).name,
                0 as ::core::ffi::c_int,
            ),
            g_variant_type_checked_(b"(u)\0" as *const u8 as *const gchar),
            G_DBUS_CALL_FLAGS_NONE,
            -(1 as gint),
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>,
                GAsyncReadyCallback,
            >(Some(
                safe_c2rust_start_service_by_name_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            )),
            safe_c2rust_client_ref(client) as gpointer,
        );
    } else {
        safe_c2rust_invoke_get_name_owner(client);
    };
}
unsafe extern "C" fn safe_c2rust_connection_get_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut client: *mut Client = user_data as *mut Client;
    (*client).connection = g_bus_get_finish(res, ::core::ptr::null_mut::<*mut GError>());
    if (*client).connection.is_null() {
        safe_c2rust_call_vanished_handler(client);
    } else {
        safe_c2rust_has_connection(client);
    }
    safe_c2rust_client_unref(client);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_watch_name(
    mut bus_type: GBusType,
    mut name: *const gchar,
    mut flags: GBusNameWatcherFlags,
    mut name_appeared_handler: GBusNameAppearedCallback,
    mut name_vanished_handler: GBusNameVanishedCallback,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
) -> guint {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 {
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
            b"g_dbus_is_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
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
    (*client).id = ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_next_global_id;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut safe_c2rust_next_global_id,
            1 as ::core::ffi::c_int as guint,
        ) as gint
    }) as guint;
    (*client).name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*client).flags = flags;
    (*client).name_appeared_handler = name_appeared_handler;
    (*client).name_vanished_handler = name_vanished_handler;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_watch_name_on_connection(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut flags: GBusNameWatcherFlags,
    mut name_appeared_handler: GBusNameAppearedCallback,
    mut name_vanished_handler: GBusNameVanishedCallback,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
) -> guint {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 {
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
            b"g_dbus_is_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
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
    (*client).id = ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_next_global_id;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut safe_c2rust_next_global_id,
            1 as ::core::ffi::c_int as guint,
        ) as gint
    }) as guint;
    (*client).name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*client).flags = flags;
    (*client).name_appeared_handler = name_appeared_handler;
    (*client).name_vanished_handler = name_vanished_handler;
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
    (*client).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
    safe_c2rust_has_connection(client);
    return (*client).id;
}
unsafe extern "C" fn safe_c2rust_watch_name_data_new(
    mut name_appeared_closure: *mut GClosure,
    mut name_vanished_closure: *mut GClosure,
) -> *mut WatchNameData {
    let mut data: *mut WatchNameData = ::core::ptr::null_mut::<WatchNameData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<WatchNameData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut WatchNameData;
    if !name_appeared_closure.is_null() {
        (*data).name_appeared_closure = g_closure_ref(name_appeared_closure);
        g_closure_sink(name_appeared_closure);
        if (*name_appeared_closure).marshal.is_none() {
            g_closure_set_marshal(
                name_appeared_closure,
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
    if !name_vanished_closure.is_null() {
        (*data).name_vanished_closure = g_closure_ref(name_vanished_closure);
        g_closure_sink(name_vanished_closure);
        if (*name_vanished_closure).marshal.is_none() {
            g_closure_set_marshal(
                name_vanished_closure,
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
unsafe extern "C" fn safe_c2rust_watch_with_closures_on_name_appeared(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut name_owner: *const gchar,
    mut user_data: gpointer,
) {
    let mut data: *mut WatchNameData = user_data as *mut WatchNameData;
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
    g_value_init(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        name_owner,
    );
    g_closure_invoke(
        (*data).name_appeared_closure,
        ::core::ptr::null_mut::<GValue>(),
        3 as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn safe_c2rust_watch_with_closures_on_name_vanished(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut data: *mut WatchNameData = user_data as *mut WatchNameData;
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
        (*data).name_vanished_closure,
        ::core::ptr::null_mut::<GValue>(),
        2 as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn safe_c2rust_bus_watch_name_free_func(mut user_data: gpointer) {
    let mut data: *mut WatchNameData = user_data as *mut WatchNameData;
    if !(*data).name_appeared_closure.is_null() {
        g_closure_unref((*data).name_appeared_closure);
    }
    if !(*data).name_vanished_closure.is_null() {
        g_closure_unref((*data).name_vanished_closure);
    }
    g_free(data as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_watch_name_with_closures(
    mut bus_type: GBusType,
    mut name: *const gchar,
    mut flags: GBusNameWatcherFlags,
    mut name_appeared_closure: *mut GClosure,
    mut name_vanished_closure: *mut GClosure,
) -> guint {
    return safe_c2rust_g_bus_watch_name(
        bus_type,
        name,
        flags,
        if !name_appeared_closure.is_null() {
            Some(
                safe_c2rust_watch_with_closures_on_name_appeared
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        gpointer,
                    ) -> (),
            )
        } else {
            None
        },
        if !name_vanished_closure.is_null() {
            Some(
                safe_c2rust_watch_with_closures_on_name_vanished
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            )
        } else {
            None
        },
        safe_c2rust_watch_name_data_new(name_appeared_closure, name_vanished_closure) as gpointer,
        Some(safe_c2rust_bus_watch_name_free_func as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_watch_name_on_connection_with_closures(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut flags: GBusNameWatcherFlags,
    mut name_appeared_closure: *mut GClosure,
    mut name_vanished_closure: *mut GClosure,
) -> guint {
    return safe_c2rust_g_bus_watch_name_on_connection(
        connection,
        name,
        flags,
        if !name_appeared_closure.is_null() {
            Some(
                safe_c2rust_watch_with_closures_on_name_appeared
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        gpointer,
                    ) -> (),
            )
        } else {
            None
        },
        if !name_vanished_closure.is_null() {
            Some(
                safe_c2rust_watch_with_closures_on_name_vanished
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            )
        } else {
            None
        },
        safe_c2rust_watch_name_data_new(name_appeared_closure, name_vanished_closure) as gpointer,
        Some(safe_c2rust_bus_watch_name_free_func as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_unwatch_name(mut watcher_id: guint) {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if watcher_id > 0 as guint {
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
            b"watcher_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    client = ::core::ptr::null_mut::<Client>();
    g_mutex_lock(&raw mut safe_c2rust_g__lock_lock);
    if watcher_id == 0 as guint || safe_c2rust_map_id_to_client.is_null() || {
        client = g_hash_table_lookup(
            safe_c2rust_map_id_to_client,
            watcher_id as gulong as gpointer as gconstpointer,
        ) as *mut Client;
        client.is_null()
    } {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Invalid id %d passed to g_bus_unwatch_name()\0" as *const u8 as *const gchar,
            watcher_id,
        );
    } else {
        (*client).cancelled = TRUE as gboolean;
        if !(({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                safe_c2rust_map_id_to_client,
                watcher_id as gulong as gpointer as gconstpointer,
            ) != 0
            {
                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_18
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusnamewatching.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                907 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (map_id_to_client, GUINT_TO_POINTER (watcher_id))\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__lock_lock);
    if !client.is_null() {
        safe_c2rust_client_unref(client);
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
