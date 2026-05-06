extern "C" {
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_enum_register_static(name: *const gchar, const_static_values: *const GEnumValue) -> GType;
    fn g_flags_register_static(
        name: *const gchar,
        const_static_values: *const GFlagsValue,
    ) -> GType;
}
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GType = gsize;
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub const G_APP_INFO_CREATE_SUPPORTS_STARTUP_NOTIFICATION: C2RustUnnamed = 4;
pub const G_APP_INFO_CREATE_SUPPORTS_URIS: C2RustUnnamed = 2;
pub const G_APP_INFO_CREATE_NEEDS_TERMINAL: C2RustUnnamed = 1;
pub const G_APP_INFO_CREATE_NONE: C2RustUnnamed = 0;
pub const G_CONVERTER_FLUSH: C2RustUnnamed_0 = 2;
pub const G_CONVERTER_INPUT_AT_END: C2RustUnnamed_0 = 1;
pub const G_CONVERTER_NO_FLAGS: C2RustUnnamed_0 = 0;
pub const G_CONVERTER_FLUSHED: C2RustUnnamed_1 = 3;
pub const G_CONVERTER_FINISHED: C2RustUnnamed_1 = 2;
pub const G_CONVERTER_CONVERTED: C2RustUnnamed_1 = 1;
pub const G_CONVERTER_ERROR: C2RustUnnamed_1 = 0;
pub const G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN: C2RustUnnamed_2 = 2;
pub const G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN: C2RustUnnamed_2 = 1;
pub const G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN: C2RustUnnamed_2 = 0;
pub const G_DATA_STREAM_NEWLINE_TYPE_ANY: C2RustUnnamed_3 = 3;
pub const G_DATA_STREAM_NEWLINE_TYPE_CR_LF: C2RustUnnamed_3 = 2;
pub const G_DATA_STREAM_NEWLINE_TYPE_CR: C2RustUnnamed_3 = 1;
pub const G_DATA_STREAM_NEWLINE_TYPE_LF: C2RustUnnamed_3 = 0;
pub const G_FILE_ATTRIBUTE_TYPE_STRINGV: C2RustUnnamed_4 = 9;
pub const G_FILE_ATTRIBUTE_TYPE_OBJECT: C2RustUnnamed_4 = 8;
pub const G_FILE_ATTRIBUTE_TYPE_INT64: C2RustUnnamed_4 = 7;
pub const G_FILE_ATTRIBUTE_TYPE_UINT64: C2RustUnnamed_4 = 6;
pub const G_FILE_ATTRIBUTE_TYPE_INT32: C2RustUnnamed_4 = 5;
pub const G_FILE_ATTRIBUTE_TYPE_UINT32: C2RustUnnamed_4 = 4;
pub const G_FILE_ATTRIBUTE_TYPE_BOOLEAN: C2RustUnnamed_4 = 3;
pub const G_FILE_ATTRIBUTE_TYPE_BYTE_STRING: C2RustUnnamed_4 = 2;
pub const G_FILE_ATTRIBUTE_TYPE_STRING: C2RustUnnamed_4 = 1;
pub const G_FILE_ATTRIBUTE_TYPE_INVALID: C2RustUnnamed_4 = 0;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED: C2RustUnnamed_5 = 2;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE: C2RustUnnamed_5 = 1;
pub const G_FILE_ATTRIBUTE_INFO_NONE: C2RustUnnamed_5 = 0;
pub const G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING: C2RustUnnamed_6 = 2;
pub const G_FILE_ATTRIBUTE_STATUS_SET: C2RustUnnamed_6 = 1;
pub const G_FILE_ATTRIBUTE_STATUS_UNSET: C2RustUnnamed_6 = 0;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: C2RustUnnamed_7 = 1;
pub const G_FILE_QUERY_INFO_NONE: C2RustUnnamed_7 = 0;
pub const G_FILE_CREATE_REPLACE_DESTINATION: C2RustUnnamed_8 = 2;
pub const G_FILE_CREATE_PRIVATE: C2RustUnnamed_8 = 1;
pub const G_FILE_CREATE_NONE: C2RustUnnamed_8 = 0;
pub const G_FILE_MEASURE_NO_XDEV: C2RustUnnamed_9 = 8;
pub const G_FILE_MEASURE_APPARENT_SIZE: C2RustUnnamed_9 = 4;
pub const G_FILE_MEASURE_REPORT_ANY_ERROR: C2RustUnnamed_9 = 2;
pub const G_FILE_MEASURE_NONE: C2RustUnnamed_9 = 0;
pub const G_MOUNT_MOUNT_NONE: C2RustUnnamed_10 = 0;
pub const G_MOUNT_UNMOUNT_FORCE: C2RustUnnamed_11 = 1;
pub const G_MOUNT_UNMOUNT_NONE: C2RustUnnamed_11 = 0;
pub const G_DRIVE_START_NONE: C2RustUnnamed_12 = 0;
pub const G_DRIVE_START_STOP_TYPE_PASSWORD: C2RustUnnamed_13 = 4;
pub const G_DRIVE_START_STOP_TYPE_MULTIDISK: C2RustUnnamed_13 = 3;
pub const G_DRIVE_START_STOP_TYPE_NETWORK: C2RustUnnamed_13 = 2;
pub const G_DRIVE_START_STOP_TYPE_SHUTDOWN: C2RustUnnamed_13 = 1;
pub const G_DRIVE_START_STOP_TYPE_UNKNOWN: C2RustUnnamed_13 = 0;
pub const G_FILE_COPY_TARGET_DEFAULT_MODIFIED_TIME: C2RustUnnamed_14 = 64;
pub const G_FILE_COPY_TARGET_DEFAULT_PERMS: C2RustUnnamed_14 = 32;
pub const G_FILE_COPY_NO_FALLBACK_FOR_MOVE: C2RustUnnamed_14 = 16;
pub const G_FILE_COPY_ALL_METADATA: C2RustUnnamed_14 = 8;
pub const G_FILE_COPY_NOFOLLOW_SYMLINKS: C2RustUnnamed_14 = 4;
pub const G_FILE_COPY_BACKUP: C2RustUnnamed_14 = 2;
pub const G_FILE_COPY_OVERWRITE: C2RustUnnamed_14 = 1;
pub const G_FILE_COPY_NONE: C2RustUnnamed_14 = 0;
pub const G_FILE_MONITOR_WATCH_MOVES: C2RustUnnamed_15 = 8;
pub const G_FILE_MONITOR_WATCH_HARD_LINKS: C2RustUnnamed_15 = 4;
pub const G_FILE_MONITOR_SEND_MOVED: C2RustUnnamed_15 = 2;
pub const G_FILE_MONITOR_WATCH_MOUNTS: C2RustUnnamed_15 = 1;
pub const G_FILE_MONITOR_NONE: C2RustUnnamed_15 = 0;
pub const G_FILE_TYPE_MOUNTABLE: C2RustUnnamed_16 = 6;
pub const G_FILE_TYPE_SHORTCUT: C2RustUnnamed_16 = 5;
pub const G_FILE_TYPE_SPECIAL: C2RustUnnamed_16 = 4;
pub const G_FILE_TYPE_SYMBOLIC_LINK: C2RustUnnamed_16 = 3;
pub const G_FILE_TYPE_DIRECTORY: C2RustUnnamed_16 = 2;
pub const G_FILE_TYPE_REGULAR: C2RustUnnamed_16 = 1;
pub const G_FILE_TYPE_UNKNOWN: C2RustUnnamed_16 = 0;
pub const G_FILESYSTEM_PREVIEW_TYPE_NEVER: C2RustUnnamed_17 = 2;
pub const G_FILESYSTEM_PREVIEW_TYPE_IF_LOCAL: C2RustUnnamed_17 = 1;
pub const G_FILESYSTEM_PREVIEW_TYPE_IF_ALWAYS: C2RustUnnamed_17 = 0;
pub const G_FILE_MONITOR_EVENT_MOVED_OUT: C2RustUnnamed_18 = 10;
pub const G_FILE_MONITOR_EVENT_MOVED_IN: C2RustUnnamed_18 = 9;
pub const G_FILE_MONITOR_EVENT_RENAMED: C2RustUnnamed_18 = 8;
pub const G_FILE_MONITOR_EVENT_MOVED: C2RustUnnamed_18 = 7;
pub const G_FILE_MONITOR_EVENT_UNMOUNTED: C2RustUnnamed_18 = 6;
pub const G_FILE_MONITOR_EVENT_PRE_UNMOUNT: C2RustUnnamed_18 = 5;
pub const G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED: C2RustUnnamed_18 = 4;
pub const G_FILE_MONITOR_EVENT_CREATED: C2RustUnnamed_18 = 3;
pub const G_FILE_MONITOR_EVENT_DELETED: C2RustUnnamed_18 = 2;
pub const G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT: C2RustUnnamed_18 = 1;
pub const G_FILE_MONITOR_EVENT_CHANGED: C2RustUnnamed_18 = 0;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_19 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_19 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_19 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_19 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_19 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_19 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_19 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_19 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_19 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_19 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_19 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_19 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_19 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_19 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_19 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_19 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_19 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_19 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_19 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_19 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_19 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_19 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_19 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_19 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_19 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_19 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_19 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_19 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_19 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_19 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_19 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_19 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_19 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_19 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_19 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_19 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_19 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_19 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_19 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_19 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_19 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_19 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_19 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_19 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_19 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_19 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_19 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_19 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_19 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_19 = 0;
pub const G_ASK_PASSWORD_TCRYPT: C2RustUnnamed_20 = 32;
pub const G_ASK_PASSWORD_ANONYMOUS_SUPPORTED: C2RustUnnamed_20 = 16;
pub const G_ASK_PASSWORD_SAVING_SUPPORTED: C2RustUnnamed_20 = 8;
pub const G_ASK_PASSWORD_NEED_DOMAIN: C2RustUnnamed_20 = 4;
pub const G_ASK_PASSWORD_NEED_USERNAME: C2RustUnnamed_20 = 2;
pub const G_ASK_PASSWORD_NEED_PASSWORD: C2RustUnnamed_20 = 1;
pub const G_PASSWORD_SAVE_PERMANENTLY: C2RustUnnamed_21 = 2;
pub const G_PASSWORD_SAVE_FOR_SESSION: C2RustUnnamed_21 = 1;
pub const G_PASSWORD_SAVE_NEVER: C2RustUnnamed_21 = 0;
pub const G_MOUNT_OPERATION_UNHANDLED: C2RustUnnamed_22 = 2;
pub const G_MOUNT_OPERATION_ABORTED: C2RustUnnamed_22 = 1;
pub const G_MOUNT_OPERATION_HANDLED: C2RustUnnamed_22 = 0;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET: C2RustUnnamed_23 = 2;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE: C2RustUnnamed_23 = 1;
pub const G_OUTPUT_STREAM_SPLICE_NONE: C2RustUnnamed_23 = 0;
pub const G_IO_STREAM_SPLICE_WAIT_FOR_BOTH: C2RustUnnamed_24 = 4;
pub const G_IO_STREAM_SPLICE_CLOSE_STREAM2: C2RustUnnamed_24 = 2;
pub const G_IO_STREAM_SPLICE_CLOSE_STREAM1: C2RustUnnamed_24 = 1;
pub const G_IO_STREAM_SPLICE_NONE: C2RustUnnamed_24 = 0;
pub const G_EMBLEM_ORIGIN_TAG: C2RustUnnamed_25 = 3;
pub const G_EMBLEM_ORIGIN_LIVEMETADATA: C2RustUnnamed_25 = 2;
pub const G_EMBLEM_ORIGIN_DEVICE: C2RustUnnamed_25 = 1;
pub const G_EMBLEM_ORIGIN_UNKNOWN: C2RustUnnamed_25 = 0;
pub const G_RESOLVER_ERROR_INTERNAL: C2RustUnnamed_26 = 2;
pub const G_RESOLVER_ERROR_TEMPORARY_FAILURE: C2RustUnnamed_26 = 1;
pub const G_RESOLVER_ERROR_NOT_FOUND: C2RustUnnamed_26 = 0;
pub const G_RESOLVER_RECORD_NS: C2RustUnnamed_27 = 5;
pub const G_RESOLVER_RECORD_SOA: C2RustUnnamed_27 = 4;
pub const G_RESOLVER_RECORD_TXT: C2RustUnnamed_27 = 3;
pub const G_RESOLVER_RECORD_MX: C2RustUnnamed_27 = 2;
pub const G_RESOLVER_RECORD_SRV: C2RustUnnamed_27 = 1;
pub const G_RESOURCE_ERROR_INTERNAL: C2RustUnnamed_28 = 1;
pub const G_RESOURCE_ERROR_NOT_FOUND: C2RustUnnamed_28 = 0;
pub const G_RESOURCE_FLAGS_COMPRESSED: C2RustUnnamed_29 = 1;
pub const G_RESOURCE_FLAGS_NONE: C2RustUnnamed_29 = 0;
pub const G_RESOURCE_LOOKUP_FLAGS_NONE: C2RustUnnamed_30 = 0;
pub const G_SOCKET_FAMILY_IPV6: C2RustUnnamed_31 = 10;
pub const G_SOCKET_FAMILY_IPV4: C2RustUnnamed_31 = 2;
pub const G_SOCKET_FAMILY_UNIX: C2RustUnnamed_31 = 1;
pub const G_SOCKET_FAMILY_INVALID: C2RustUnnamed_31 = 0;
pub const G_SOCKET_TYPE_SEQPACKET: C2RustUnnamed_32 = 3;
pub const G_SOCKET_TYPE_DATAGRAM: C2RustUnnamed_32 = 2;
pub const G_SOCKET_TYPE_STREAM: C2RustUnnamed_32 = 1;
pub const G_SOCKET_TYPE_INVALID: C2RustUnnamed_32 = 0;
pub const G_SOCKET_MSG_DONTROUTE: C2RustUnnamed_33 = 4;
pub const G_SOCKET_MSG_PEEK: C2RustUnnamed_33 = 2;
pub const G_SOCKET_MSG_OOB: C2RustUnnamed_33 = 1;
pub const G_SOCKET_MSG_NONE: C2RustUnnamed_33 = 0;
pub const G_SOCKET_PROTOCOL_SCTP: C2RustUnnamed_34 = 132;
pub const G_SOCKET_PROTOCOL_UDP: C2RustUnnamed_34 = 17;
pub const G_SOCKET_PROTOCOL_TCP: C2RustUnnamed_34 = 6;
pub const G_SOCKET_PROTOCOL_DEFAULT: C2RustUnnamed_34 = 0;
pub const G_SOCKET_PROTOCOL_UNKNOWN: C2RustUnnamed_34 = -1;
pub const G_ZLIB_COMPRESSOR_FORMAT_RAW: C2RustUnnamed_35 = 2;
pub const G_ZLIB_COMPRESSOR_FORMAT_GZIP: C2RustUnnamed_35 = 1;
pub const G_ZLIB_COMPRESSOR_FORMAT_ZLIB: C2RustUnnamed_35 = 0;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED: C2RustUnnamed_36 = 4;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT: C2RustUnnamed_36 = 3;
pub const G_UNIX_SOCKET_ADDRESS_PATH: C2RustUnnamed_36 = 2;
pub const G_UNIX_SOCKET_ADDRESS_ANONYMOUS: C2RustUnnamed_36 = 1;
pub const G_UNIX_SOCKET_ADDRESS_INVALID: C2RustUnnamed_36 = 0;
pub const G_BUS_TYPE_SESSION: C2RustUnnamed_37 = 2;
pub const G_BUS_TYPE_SYSTEM: C2RustUnnamed_37 = 1;
pub const G_BUS_TYPE_NONE: C2RustUnnamed_37 = 0;
pub const G_BUS_TYPE_STARTER: C2RustUnnamed_37 = -1;
pub const G_BUS_NAME_OWNER_FLAGS_DO_NOT_QUEUE: C2RustUnnamed_38 = 4;
pub const G_BUS_NAME_OWNER_FLAGS_REPLACE: C2RustUnnamed_38 = 2;
pub const G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT: C2RustUnnamed_38 = 1;
pub const G_BUS_NAME_OWNER_FLAGS_NONE: C2RustUnnamed_38 = 0;
pub const G_BUS_NAME_WATCHER_FLAGS_AUTO_START: C2RustUnnamed_39 = 1;
pub const G_BUS_NAME_WATCHER_FLAGS_NONE: C2RustUnnamed_39 = 0;
pub const G_DBUS_PROXY_FLAGS_NO_MATCH_RULE: C2RustUnnamed_40 = 32;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION: C2RustUnnamed_40 = 16;
pub const G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES: C2RustUnnamed_40 = 8;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START: C2RustUnnamed_40 = 4;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS: C2RustUnnamed_40 = 2;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES: C2RustUnnamed_40 = 1;
pub const G_DBUS_PROXY_FLAGS_NONE: C2RustUnnamed_40 = 0;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_41 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_41 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_41 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_41 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_41 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_41 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_41 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_41 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_41 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_41 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_41 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_41 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_41 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_41 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_41 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_41 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_41 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_41 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_41 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_41 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_41 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_41 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_41 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_41 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_41 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_41 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_41 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_41 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_41 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_41 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_41 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_41 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_41 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_41 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_41 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_41 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_41 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_41 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_41 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_41 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_41 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_41 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_41 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_41 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_41 = 0;
pub const G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE: C2RustUnnamed_42 = 64;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: C2RustUnnamed_42 = 32;
pub const G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING: C2RustUnnamed_42 = 16;
pub const G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION: C2RustUnnamed_42 = 8;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: C2RustUnnamed_42 = 4;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER: C2RustUnnamed_42 = 2;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT: C2RustUnnamed_42 = 1;
pub const G_DBUS_CONNECTION_FLAGS_NONE: C2RustUnnamed_42 = 0;
pub const G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING: C2RustUnnamed_43 = 1;
pub const G_DBUS_CAPABILITY_FLAGS_NONE: C2RustUnnamed_43 = 0;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: C2RustUnnamed_44 = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: C2RustUnnamed_44 = 1;
pub const G_DBUS_CALL_FLAGS_NONE: C2RustUnnamed_44 = 0;
pub const G_DBUS_MESSAGE_TYPE_SIGNAL: C2RustUnnamed_45 = 4;
pub const G_DBUS_MESSAGE_TYPE_ERROR: C2RustUnnamed_45 = 3;
pub const G_DBUS_MESSAGE_TYPE_METHOD_RETURN: C2RustUnnamed_45 = 2;
pub const G_DBUS_MESSAGE_TYPE_METHOD_CALL: C2RustUnnamed_45 = 1;
pub const G_DBUS_MESSAGE_TYPE_INVALID: C2RustUnnamed_45 = 0;
pub const G_DBUS_MESSAGE_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: C2RustUnnamed_46 = 4;
pub const G_DBUS_MESSAGE_FLAGS_NO_AUTO_START: C2RustUnnamed_46 = 2;
pub const G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED: C2RustUnnamed_46 = 1;
pub const G_DBUS_MESSAGE_FLAGS_NONE: C2RustUnnamed_46 = 0;
pub const G_DBUS_MESSAGE_HEADER_FIELD_NUM_UNIX_FDS: C2RustUnnamed_47 = 9;
pub const G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE: C2RustUnnamed_47 = 8;
pub const G_DBUS_MESSAGE_HEADER_FIELD_SENDER: C2RustUnnamed_47 = 7;
pub const G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION: C2RustUnnamed_47 = 6;
pub const G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL: C2RustUnnamed_47 = 5;
pub const G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME: C2RustUnnamed_47 = 4;
pub const G_DBUS_MESSAGE_HEADER_FIELD_MEMBER: C2RustUnnamed_47 = 3;
pub const G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE: C2RustUnnamed_47 = 2;
pub const G_DBUS_MESSAGE_HEADER_FIELD_PATH: C2RustUnnamed_47 = 1;
pub const G_DBUS_MESSAGE_HEADER_FIELD_INVALID: C2RustUnnamed_47 = 0;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: C2RustUnnamed_48 = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: C2RustUnnamed_48 = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: C2RustUnnamed_48 = 0;
pub const G_DBUS_SUBTREE_FLAGS_DISPATCH_TO_UNENUMERATED_NODES: C2RustUnnamed_49 = 1;
pub const G_DBUS_SUBTREE_FLAGS_NONE: C2RustUnnamed_49 = 0;
pub const G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: C2RustUnnamed_50 = 4;
pub const G_DBUS_SERVER_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: C2RustUnnamed_50 = 2;
pub const G_DBUS_SERVER_FLAGS_RUN_IN_THREAD: C2RustUnnamed_50 = 1;
pub const G_DBUS_SERVER_FLAGS_NONE: C2RustUnnamed_50 = 0;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: C2RustUnnamed_51 = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: C2RustUnnamed_51 = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: C2RustUnnamed_51 = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: C2RustUnnamed_51 = 0;
pub const G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL: C2RustUnnamed_52 = 1;
pub const G_DBUS_SEND_MESSAGE_FLAGS_NONE: C2RustUnnamed_52 = 0;
pub const G_CREDENTIALS_TYPE_WIN32_PID: C2RustUnnamed_53 = 7;
pub const G_CREDENTIALS_TYPE_APPLE_XUCRED: C2RustUnnamed_53 = 6;
pub const G_CREDENTIALS_TYPE_NETBSD_UNPCBID: C2RustUnnamed_53 = 5;
pub const G_CREDENTIALS_TYPE_SOLARIS_UCRED: C2RustUnnamed_53 = 4;
pub const G_CREDENTIALS_TYPE_OPENBSD_SOCKPEERCRED: C2RustUnnamed_53 = 3;
pub const G_CREDENTIALS_TYPE_FREEBSD_CMSGCRED: C2RustUnnamed_53 = 2;
pub const G_CREDENTIALS_TYPE_LINUX_UCRED: C2RustUnnamed_53 = 1;
pub const G_CREDENTIALS_TYPE_INVALID: C2RustUnnamed_53 = 0;
pub const G_DBUS_MESSAGE_BYTE_ORDER_LITTLE_ENDIAN: C2RustUnnamed_54 = 108;
pub const G_DBUS_MESSAGE_BYTE_ORDER_BIG_ENDIAN: C2RustUnnamed_54 = 66;
pub const G_APPLICATION_REPLACE: C2RustUnnamed_55 = 256;
pub const G_APPLICATION_ALLOW_REPLACEMENT: C2RustUnnamed_55 = 128;
pub const G_APPLICATION_CAN_OVERRIDE_APP_ID: C2RustUnnamed_55 = 64;
pub const G_APPLICATION_NON_UNIQUE: C2RustUnnamed_55 = 32;
pub const G_APPLICATION_SEND_ENVIRONMENT: C2RustUnnamed_55 = 16;
pub const G_APPLICATION_HANDLES_COMMAND_LINE: C2RustUnnamed_55 = 8;
pub const G_APPLICATION_HANDLES_OPEN: C2RustUnnamed_55 = 4;
pub const G_APPLICATION_IS_LAUNCHER: C2RustUnnamed_55 = 2;
pub const G_APPLICATION_IS_SERVICE: C2RustUnnamed_55 = 1;
pub const G_APPLICATION_DEFAULT_FLAGS: C2RustUnnamed_55 = 0;
pub const G_APPLICATION_FLAGS_NONE: C2RustUnnamed_55 = 0;
pub const G_TLS_ERROR_BAD_CERTIFICATE_PASSWORD: C2RustUnnamed_56 = 8;
pub const G_TLS_ERROR_INAPPROPRIATE_FALLBACK: C2RustUnnamed_56 = 7;
pub const G_TLS_ERROR_EOF: C2RustUnnamed_56 = 6;
pub const G_TLS_ERROR_CERTIFICATE_REQUIRED: C2RustUnnamed_56 = 5;
pub const G_TLS_ERROR_HANDSHAKE: C2RustUnnamed_56 = 4;
pub const G_TLS_ERROR_NOT_TLS: C2RustUnnamed_56 = 3;
pub const G_TLS_ERROR_BAD_CERTIFICATE: C2RustUnnamed_56 = 2;
pub const G_TLS_ERROR_MISC: C2RustUnnamed_56 = 1;
pub const G_TLS_ERROR_UNAVAILABLE: C2RustUnnamed_56 = 0;
pub const G_TLS_CERTIFICATE_VALIDATE_ALL: C2RustUnnamed_57 = 127;
pub const G_TLS_CERTIFICATE_GENERIC_ERROR: C2RustUnnamed_57 = 64;
pub const G_TLS_CERTIFICATE_INSECURE: C2RustUnnamed_57 = 32;
pub const G_TLS_CERTIFICATE_REVOKED: C2RustUnnamed_57 = 16;
pub const G_TLS_CERTIFICATE_EXPIRED: C2RustUnnamed_57 = 8;
pub const G_TLS_CERTIFICATE_NOT_ACTIVATED: C2RustUnnamed_57 = 4;
pub const G_TLS_CERTIFICATE_BAD_IDENTITY: C2RustUnnamed_57 = 2;
pub const G_TLS_CERTIFICATE_UNKNOWN_CA: C2RustUnnamed_57 = 1;
pub const G_TLS_CERTIFICATE_NO_FLAGS: C2RustUnnamed_57 = 0;
pub const G_TLS_AUTHENTICATION_REQUIRED: C2RustUnnamed_58 = 2;
pub const G_TLS_AUTHENTICATION_REQUESTED: C2RustUnnamed_58 = 1;
pub const G_TLS_AUTHENTICATION_NONE: C2RustUnnamed_58 = 0;
pub const G_TLS_CHANNEL_BINDING_TLS_EXPORTER: C2RustUnnamed_59 = 2;
pub const G_TLS_CHANNEL_BINDING_TLS_SERVER_END_POINT: C2RustUnnamed_59 = 1;
pub const G_TLS_CHANNEL_BINDING_TLS_UNIQUE: C2RustUnnamed_59 = 0;
pub const G_TLS_CHANNEL_BINDING_ERROR_GENERAL_ERROR: C2RustUnnamed_60 = 4;
pub const G_TLS_CHANNEL_BINDING_ERROR_NOT_SUPPORTED: C2RustUnnamed_60 = 3;
pub const G_TLS_CHANNEL_BINDING_ERROR_NOT_AVAILABLE: C2RustUnnamed_60 = 2;
pub const G_TLS_CHANNEL_BINDING_ERROR_INVALID_STATE: C2RustUnnamed_60 = 1;
pub const G_TLS_CHANNEL_BINDING_ERROR_NOT_IMPLEMENTED: C2RustUnnamed_60 = 0;
pub const G_TLS_REHANDSHAKE_UNSAFELY: C2RustUnnamed_61 = 2;
pub const G_TLS_REHANDSHAKE_SAFELY: C2RustUnnamed_61 = 1;
pub const G_TLS_REHANDSHAKE_NEVER: C2RustUnnamed_61 = 0;
pub const G_TLS_PASSWORD_PKCS11_CONTEXT_SPECIFIC: _GTlsPasswordFlags = 64;
pub const G_TLS_PASSWORD_PKCS11_SECURITY_OFFICER: _GTlsPasswordFlags = 32;
pub const G_TLS_PASSWORD_PKCS11_USER: _GTlsPasswordFlags = 16;
pub const G_TLS_PASSWORD_FINAL_TRY: _GTlsPasswordFlags = 8;
pub const G_TLS_PASSWORD_MANY_TRIES: _GTlsPasswordFlags = 4;
pub const G_TLS_PASSWORD_RETRY: _GTlsPasswordFlags = 2;
pub const G_TLS_PASSWORD_NONE: _GTlsPasswordFlags = 0;
pub const G_TLS_INTERACTION_FAILED: C2RustUnnamed_62 = 2;
pub const G_TLS_INTERACTION_HANDLED: C2RustUnnamed_62 = 1;
pub const G_TLS_INTERACTION_UNHANDLED: C2RustUnnamed_62 = 0;
pub const G_DBUS_INTERFACE_SKELETON_FLAGS_HANDLE_METHOD_INVOCATIONS_IN_THREAD: C2RustUnnamed_63 = 1;
pub const G_DBUS_INTERFACE_SKELETON_FLAGS_NONE: C2RustUnnamed_63 = 0;
pub const G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_DO_NOT_AUTO_START: C2RustUnnamed_64 = 1;
pub const G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE: C2RustUnnamed_64 = 0;
pub const G_TLS_DATABASE_VERIFY_NONE: C2RustUnnamed_65 = 0;
pub const G_TLS_DATABASE_LOOKUP_KEYPAIR: C2RustUnnamed_66 = 1;
pub const G_TLS_DATABASE_LOOKUP_NONE: C2RustUnnamed_66 = 0;
pub const G_TLS_CERTIFICATE_REQUEST_NONE: C2RustUnnamed_67 = 0;
pub const G_TLS_PROTOCOL_VERSION_DTLS_1_2: C2RustUnnamed_68 = 202;
pub const G_TLS_PROTOCOL_VERSION_DTLS_1_0: C2RustUnnamed_68 = 201;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_3: C2RustUnnamed_68 = 5;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_2: C2RustUnnamed_68 = 4;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_1: C2RustUnnamed_68 = 3;
pub const G_TLS_PROTOCOL_VERSION_TLS_1_0: C2RustUnnamed_68 = 2;
pub const G_TLS_PROTOCOL_VERSION_SSL_3_0: C2RustUnnamed_68 = 1;
pub const G_TLS_PROTOCOL_VERSION_UNKNOWN: C2RustUnnamed_68 = 0;
pub const G_IO_MODULE_SCOPE_BLOCK_DUPLICATES: C2RustUnnamed_69 = 1;
pub const G_IO_MODULE_SCOPE_NONE: C2RustUnnamed_69 = 0;
pub const G_SOCKET_CLIENT_COMPLETE: C2RustUnnamed_70 = 8;
pub const G_SOCKET_CLIENT_TLS_HANDSHAKED: C2RustUnnamed_70 = 7;
pub const G_SOCKET_CLIENT_TLS_HANDSHAKING: C2RustUnnamed_70 = 6;
pub const G_SOCKET_CLIENT_PROXY_NEGOTIATED: C2RustUnnamed_70 = 5;
pub const G_SOCKET_CLIENT_PROXY_NEGOTIATING: C2RustUnnamed_70 = 4;
pub const G_SOCKET_CLIENT_CONNECTED: C2RustUnnamed_70 = 3;
pub const G_SOCKET_CLIENT_CONNECTING: C2RustUnnamed_70 = 2;
pub const G_SOCKET_CLIENT_RESOLVED: C2RustUnnamed_70 = 1;
pub const G_SOCKET_CLIENT_RESOLVING: C2RustUnnamed_70 = 0;
pub const G_SOCKET_LISTENER_LISTENED: C2RustUnnamed_71 = 3;
pub const G_SOCKET_LISTENER_LISTENING: C2RustUnnamed_71 = 2;
pub const G_SOCKET_LISTENER_BOUND: C2RustUnnamed_71 = 1;
pub const G_SOCKET_LISTENER_BINDING: C2RustUnnamed_71 = 0;
pub const G_TEST_DBUS_NONE: C2RustUnnamed_72 = 0;
pub const G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP: C2RustUnnamed_73 = 256;
pub const G_SUBPROCESS_FLAGS_INHERIT_FDS: C2RustUnnamed_73 = 128;
pub const G_SUBPROCESS_FLAGS_STDERR_MERGE: C2RustUnnamed_73 = 64;
pub const G_SUBPROCESS_FLAGS_STDERR_SILENCE: C2RustUnnamed_73 = 32;
pub const G_SUBPROCESS_FLAGS_STDERR_PIPE: C2RustUnnamed_73 = 16;
pub const G_SUBPROCESS_FLAGS_STDOUT_SILENCE: C2RustUnnamed_73 = 8;
pub const G_SUBPROCESS_FLAGS_STDOUT_PIPE: C2RustUnnamed_73 = 4;
pub const G_SUBPROCESS_FLAGS_STDIN_INHERIT: C2RustUnnamed_73 = 2;
pub const G_SUBPROCESS_FLAGS_STDIN_PIPE: C2RustUnnamed_73 = 1;
pub const G_SUBPROCESS_FLAGS_NONE: C2RustUnnamed_73 = 0;
pub const G_NOTIFICATION_PRIORITY_URGENT: C2RustUnnamed_74 = 3;
pub const G_NOTIFICATION_PRIORITY_HIGH: C2RustUnnamed_74 = 2;
pub const G_NOTIFICATION_PRIORITY_LOW: C2RustUnnamed_74 = 1;
pub const G_NOTIFICATION_PRIORITY_NORMAL: C2RustUnnamed_74 = 0;
pub const G_NETWORK_CONNECTIVITY_FULL: C2RustUnnamed_75 = 4;
pub const G_NETWORK_CONNECTIVITY_PORTAL: C2RustUnnamed_75 = 3;
pub const G_NETWORK_CONNECTIVITY_LIMITED: C2RustUnnamed_75 = 2;
pub const G_NETWORK_CONNECTIVITY_LOCAL: C2RustUnnamed_75 = 1;
pub const G_POLLABLE_RETURN_WOULD_BLOCK: C2RustUnnamed_76 = -27;
pub const G_POLLABLE_RETURN_OK: C2RustUnnamed_76 = 1;
pub const G_POLLABLE_RETURN_FAILED: C2RustUnnamed_76 = 0;
pub const G_MEMORY_MONITOR_WARNING_LEVEL_CRITICAL: C2RustUnnamed_77 = 255;
pub const G_MEMORY_MONITOR_WARNING_LEVEL_MEDIUM: C2RustUnnamed_77 = 100;
pub const G_MEMORY_MONITOR_WARNING_LEVEL_LOW: C2RustUnnamed_77 = 50;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY: C2RustUnnamed_78 = 2;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY: C2RustUnnamed_78 = 1;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT: C2RustUnnamed_78 = 0;
pub const G_SETTINGS_BIND_INVERT_BOOLEAN: C2RustUnnamed_79 = 16;
pub const G_SETTINGS_BIND_GET_NO_CHANGES: C2RustUnnamed_79 = 8;
pub const G_SETTINGS_BIND_NO_SENSITIVITY: C2RustUnnamed_79 = 4;
pub const G_SETTINGS_BIND_SET: C2RustUnnamed_79 = 2;
pub const G_SETTINGS_BIND_GET: C2RustUnnamed_79 = 1;
pub const G_SETTINGS_BIND_DEFAULT: C2RustUnnamed_79 = 0;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub type C2RustUnnamed_26 = ::core::ffi::c_uint;
pub type C2RustUnnamed_27 = ::core::ffi::c_uint;
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub type C2RustUnnamed_29 = ::core::ffi::c_uint;
pub type C2RustUnnamed_30 = ::core::ffi::c_uint;
pub type C2RustUnnamed_31 = ::core::ffi::c_uint;
pub type C2RustUnnamed_32 = ::core::ffi::c_uint;
pub type C2RustUnnamed_33 = ::core::ffi::c_uint;
pub type C2RustUnnamed_34 = ::core::ffi::c_int;
pub type C2RustUnnamed_35 = ::core::ffi::c_uint;
pub type C2RustUnnamed_36 = ::core::ffi::c_uint;
pub type C2RustUnnamed_37 = ::core::ffi::c_int;
pub type C2RustUnnamed_38 = ::core::ffi::c_uint;
pub type C2RustUnnamed_39 = ::core::ffi::c_uint;
pub type C2RustUnnamed_40 = ::core::ffi::c_uint;
pub type C2RustUnnamed_41 = ::core::ffi::c_uint;
pub type C2RustUnnamed_42 = ::core::ffi::c_uint;
pub type C2RustUnnamed_43 = ::core::ffi::c_uint;
pub type C2RustUnnamed_44 = ::core::ffi::c_uint;
pub type C2RustUnnamed_45 = ::core::ffi::c_uint;
pub type C2RustUnnamed_46 = ::core::ffi::c_uint;
pub type C2RustUnnamed_47 = ::core::ffi::c_uint;
pub type C2RustUnnamed_48 = ::core::ffi::c_uint;
pub type C2RustUnnamed_49 = ::core::ffi::c_uint;
pub type C2RustUnnamed_50 = ::core::ffi::c_uint;
pub type C2RustUnnamed_51 = ::core::ffi::c_uint;
pub type C2RustUnnamed_52 = ::core::ffi::c_uint;
pub type C2RustUnnamed_53 = ::core::ffi::c_uint;
pub type C2RustUnnamed_54 = ::core::ffi::c_uint;
pub type C2RustUnnamed_55 = ::core::ffi::c_uint;
pub type C2RustUnnamed_56 = ::core::ffi::c_uint;
pub type C2RustUnnamed_57 = ::core::ffi::c_uint;
pub type C2RustUnnamed_58 = ::core::ffi::c_uint;
pub type C2RustUnnamed_59 = ::core::ffi::c_uint;
pub type C2RustUnnamed_60 = ::core::ffi::c_uint;
pub type C2RustUnnamed_61 = ::core::ffi::c_uint;
pub type _GTlsPasswordFlags = ::core::ffi::c_uint;
pub type C2RustUnnamed_62 = ::core::ffi::c_uint;
pub type C2RustUnnamed_63 = ::core::ffi::c_uint;
pub type C2RustUnnamed_64 = ::core::ffi::c_uint;
pub type C2RustUnnamed_65 = ::core::ffi::c_uint;
pub type C2RustUnnamed_66 = ::core::ffi::c_uint;
pub type C2RustUnnamed_67 = ::core::ffi::c_uint;
pub type C2RustUnnamed_68 = ::core::ffi::c_uint;
pub type C2RustUnnamed_69 = ::core::ffi::c_uint;
pub type C2RustUnnamed_70 = ::core::ffi::c_uint;
pub type C2RustUnnamed_71 = ::core::ffi::c_uint;
pub type C2RustUnnamed_72 = ::core::ffi::c_uint;
pub type C2RustUnnamed_73 = ::core::ffi::c_uint;
pub type C2RustUnnamed_74 = ::core::ffi::c_uint;
pub type C2RustUnnamed_75 = ::core::ffi::c_uint;
pub type C2RustUnnamed_76 = ::core::ffi::c_int;
pub type C2RustUnnamed_77 = ::core::ffi::c_uint;
pub type C2RustUnnamed_78 = ::core::ffi::c_uint;
pub type C2RustUnnamed_79 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_create_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_APP_INFO_CREATE_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_APP_INFO_CREATE_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APP_INFO_CREATE_NEEDS_TERMINAL as ::core::ffi::c_int as guint,
                value_name: b"G_APP_INFO_CREATE_NEEDS_TERMINAL\0" as *const u8 as *const gchar,
                value_nick: b"needs-terminal\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APP_INFO_CREATE_SUPPORTS_URIS as ::core::ffi::c_int as guint,
                value_name: b"G_APP_INFO_CREATE_SUPPORTS_URIS\0" as *const u8 as *const gchar,
                value_nick: b"supports-uris\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APP_INFO_CREATE_SUPPORTS_STARTUP_NOTIFICATION as ::core::ffi::c_int
                    as guint,
                value_name: b"G_APP_INFO_CREATE_SUPPORTS_STARTUP_NOTIFICATION\0" as *const u8
                    as *const gchar,
                value_nick: b"supports-startup-notification\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GAppInfoCreateFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 4] = [
            _GFlagsValue {
                value: G_CONVERTER_NO_FLAGS as ::core::ffi::c_int as guint,
                value_name: b"G_CONVERTER_NO_FLAGS\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_CONVERTER_INPUT_AT_END as ::core::ffi::c_int as guint,
                value_name: b"G_CONVERTER_INPUT_AT_END\0" as *const u8 as *const gchar,
                value_nick: b"input-at-end\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_CONVERTER_FLUSH as ::core::ffi::c_int as guint,
                value_name: b"G_CONVERTER_FLUSH\0" as *const u8 as *const gchar,
                value_nick: b"flush\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GConverterFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_converter_result_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_CONVERTER_ERROR as ::core::ffi::c_int as gint,
                value_name: b"G_CONVERTER_ERROR\0" as *const u8 as *const gchar,
                value_nick: b"error\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CONVERTER_CONVERTED as ::core::ffi::c_int as gint,
                value_name: b"G_CONVERTER_CONVERTED\0" as *const u8 as *const gchar,
                value_nick: b"converted\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CONVERTER_FINISHED as ::core::ffi::c_int as gint,
                value_name: b"G_CONVERTER_FINISHED\0" as *const u8 as *const gchar,
                value_nick: b"finished\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CONVERTER_FLUSHED as ::core::ffi::c_int as gint,
                value_name: b"G_CONVERTER_FLUSHED\0" as *const u8 as *const gchar,
                value_nick: b"flushed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GConverterResult\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_stream_byte_order_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN\0" as *const u8 as *const gchar,
                value_nick: b"big-endian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN\0" as *const u8
                    as *const gchar,
                value_nick: b"little-endian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN\0" as *const u8 as *const gchar,
                value_nick: b"host-endian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GDataStreamByteOrder\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_data_stream_newline_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_DATA_STREAM_NEWLINE_TYPE_LF as ::core::ffi::c_int as gint,
                value_name: b"G_DATA_STREAM_NEWLINE_TYPE_LF\0" as *const u8 as *const gchar,
                value_nick: b"lf\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DATA_STREAM_NEWLINE_TYPE_CR as ::core::ffi::c_int as gint,
                value_name: b"G_DATA_STREAM_NEWLINE_TYPE_CR\0" as *const u8 as *const gchar,
                value_nick: b"cr\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DATA_STREAM_NEWLINE_TYPE_CR_LF as ::core::ffi::c_int as gint,
                value_name: b"G_DATA_STREAM_NEWLINE_TYPE_CR_LF\0" as *const u8 as *const gchar,
                value_nick: b"cr-lf\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DATA_STREAM_NEWLINE_TYPE_ANY as ::core::ffi::c_int as gint,
                value_name: b"G_DATA_STREAM_NEWLINE_TYPE_ANY\0" as *const u8 as *const gchar,
                value_nick: b"any\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GDataStreamNewlineType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 11] = [
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_STRING\0" as *const u8 as *const gchar,
                value_nick: b"string\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_BYTE_STRING as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_BYTE_STRING\0" as *const u8 as *const gchar,
                value_nick: b"byte-string\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_BOOLEAN as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_BOOLEAN\0" as *const u8 as *const gchar,
                value_nick: b"boolean\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_UINT32 as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_UINT32\0" as *const u8 as *const gchar,
                value_nick: b"uint32\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_INT32 as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_INT32\0" as *const u8 as *const gchar,
                value_nick: b"int32\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_UINT64 as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_UINT64\0" as *const u8 as *const gchar,
                value_nick: b"uint64\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_INT64 as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_INT64\0" as *const u8 as *const gchar,
                value_nick: b"int64\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_OBJECT as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_OBJECT\0" as *const u8 as *const gchar,
                value_nick: b"object\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_TYPE_STRINGV as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_TYPE_STRINGV\0" as *const u8 as *const gchar,
                value_nick: b"stringv\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GFileAttributeType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_info_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 4] = [
            _GFlagsValue {
                value: G_FILE_ATTRIBUTE_INFO_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_ATTRIBUTE_INFO_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE\0" as *const u8 as *const gchar,
                value_nick: b"copy-with-file\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED\0" as *const u8 as *const gchar,
                value_nick: b"copy-when-moved\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GFileAttributeInfoFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_status_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_STATUS_UNSET as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_STATUS_UNSET\0" as *const u8 as *const gchar,
                value_nick: b"unset\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_STATUS_SET as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_STATUS_SET\0" as *const u8 as *const gchar,
                value_nick: b"set\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING\0" as *const u8 as *const gchar,
                value_nick: b"error-setting\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GFileAttributeStatus\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_query_info_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_FILE_QUERY_INFO_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_QUERY_INFO_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS\0" as *const u8 as *const gchar,
                value_nick: b"nofollow-symlinks\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GFileQueryInfoFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_create_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 4] = [
            _GFlagsValue {
                value: G_FILE_CREATE_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_CREATE_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_CREATE_PRIVATE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_CREATE_PRIVATE\0" as *const u8 as *const gchar,
                value_nick: b"private\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_CREATE_REPLACE_DESTINATION as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_CREATE_REPLACE_DESTINATION\0" as *const u8 as *const gchar,
                value_nick: b"replace-destination\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GFileCreateFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_measure_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_FILE_MEASURE_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MEASURE_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_MEASURE_REPORT_ANY_ERROR as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MEASURE_REPORT_ANY_ERROR\0" as *const u8 as *const gchar,
                value_nick: b"report-any-error\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_MEASURE_APPARENT_SIZE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MEASURE_APPARENT_SIZE\0" as *const u8 as *const gchar,
                value_nick: b"apparent-size\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_MEASURE_NO_XDEV as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MEASURE_NO_XDEV\0" as *const u8 as *const gchar,
                value_nick: b"no-xdev\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GFileMeasureFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_mount_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 2] = [
            _GFlagsValue {
                value: G_MOUNT_MOUNT_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_MOUNT_MOUNT_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GMountMountFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_unmount_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_MOUNT_UNMOUNT_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_MOUNT_UNMOUNT_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_MOUNT_UNMOUNT_FORCE as ::core::ffi::c_int as guint,
                value_name: b"G_MOUNT_UNMOUNT_FORCE\0" as *const u8 as *const gchar,
                value_nick: b"force\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GMountUnmountFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_start_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 2] = [
            _GFlagsValue {
                value: G_DRIVE_START_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DRIVE_START_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDriveStartFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_drive_start_stop_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 6] = [
            _GEnumValue {
                value: G_DRIVE_START_STOP_TYPE_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_DRIVE_START_STOP_TYPE_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DRIVE_START_STOP_TYPE_SHUTDOWN as ::core::ffi::c_int as gint,
                value_name: b"G_DRIVE_START_STOP_TYPE_SHUTDOWN\0" as *const u8 as *const gchar,
                value_nick: b"shutdown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DRIVE_START_STOP_TYPE_NETWORK as ::core::ffi::c_int as gint,
                value_name: b"G_DRIVE_START_STOP_TYPE_NETWORK\0" as *const u8 as *const gchar,
                value_nick: b"network\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DRIVE_START_STOP_TYPE_MULTIDISK as ::core::ffi::c_int as gint,
                value_name: b"G_DRIVE_START_STOP_TYPE_MULTIDISK\0" as *const u8 as *const gchar,
                value_nick: b"multidisk\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DRIVE_START_STOP_TYPE_PASSWORD as ::core::ffi::c_int as gint,
                value_name: b"G_DRIVE_START_STOP_TYPE_PASSWORD\0" as *const u8 as *const gchar,
                value_nick: b"password\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GDriveStartStopType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_copy_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 9] = [
            _GFlagsValue {
                value: G_FILE_COPY_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_COPY_OVERWRITE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_OVERWRITE\0" as *const u8 as *const gchar,
                value_nick: b"overwrite\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_COPY_BACKUP as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_BACKUP\0" as *const u8 as *const gchar,
                value_nick: b"backup\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_COPY_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_NOFOLLOW_SYMLINKS\0" as *const u8 as *const gchar,
                value_nick: b"nofollow-symlinks\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_COPY_ALL_METADATA as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_ALL_METADATA\0" as *const u8 as *const gchar,
                value_nick: b"all-metadata\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_COPY_NO_FALLBACK_FOR_MOVE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_NO_FALLBACK_FOR_MOVE\0" as *const u8 as *const gchar,
                value_nick: b"no-fallback-for-move\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_COPY_TARGET_DEFAULT_PERMS as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_TARGET_DEFAULT_PERMS\0" as *const u8 as *const gchar,
                value_nick: b"target-default-perms\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_COPY_TARGET_DEFAULT_MODIFIED_TIME as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_COPY_TARGET_DEFAULT_MODIFIED_TIME\0" as *const u8
                    as *const gchar,
                value_nick: b"target-default-modified-time\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GFileCopyFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_monitor_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 6] = [
            _GFlagsValue {
                value: G_FILE_MONITOR_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MONITOR_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_MONITOR_WATCH_MOUNTS as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MONITOR_WATCH_MOUNTS\0" as *const u8 as *const gchar,
                value_nick: b"watch-mounts\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_MONITOR_SEND_MOVED as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MONITOR_SEND_MOVED\0" as *const u8 as *const gchar,
                value_nick: b"send-moved\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_MONITOR_WATCH_HARD_LINKS as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MONITOR_WATCH_HARD_LINKS\0" as *const u8 as *const gchar,
                value_nick: b"watch-hard-links\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_FILE_MONITOR_WATCH_MOVES as ::core::ffi::c_int as guint,
                value_name: b"G_FILE_MONITOR_WATCH_MOVES\0" as *const u8 as *const gchar,
                value_nick: b"watch-moves\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GFileMonitorFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 8] = [
            _GEnumValue {
                value: G_FILE_TYPE_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_TYPE_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_TYPE_REGULAR as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_TYPE_REGULAR\0" as *const u8 as *const gchar,
                value_nick: b"regular\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_TYPE_DIRECTORY as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_TYPE_DIRECTORY\0" as *const u8 as *const gchar,
                value_nick: b"directory\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_TYPE_SYMBOLIC_LINK as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_TYPE_SYMBOLIC_LINK\0" as *const u8 as *const gchar,
                value_nick: b"symbolic-link\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_TYPE_SPECIAL as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_TYPE_SPECIAL\0" as *const u8 as *const gchar,
                value_nick: b"special\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_TYPE_SHORTCUT as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_TYPE_SHORTCUT\0" as *const u8 as *const gchar,
                value_nick: b"shortcut\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_TYPE_MOUNTABLE as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_TYPE_MOUNTABLE\0" as *const u8 as *const gchar,
                value_nick: b"mountable\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GFileType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_filesystem_preview_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_FILESYSTEM_PREVIEW_TYPE_IF_ALWAYS as ::core::ffi::c_int as gint,
                value_name: b"G_FILESYSTEM_PREVIEW_TYPE_IF_ALWAYS\0" as *const u8 as *const gchar,
                value_nick: b"if-always\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILESYSTEM_PREVIEW_TYPE_IF_LOCAL as ::core::ffi::c_int as gint,
                value_name: b"G_FILESYSTEM_PREVIEW_TYPE_IF_LOCAL\0" as *const u8 as *const gchar,
                value_nick: b"if-local\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILESYSTEM_PREVIEW_TYPE_NEVER as ::core::ffi::c_int as gint,
                value_name: b"G_FILESYSTEM_PREVIEW_TYPE_NEVER\0" as *const u8 as *const gchar,
                value_nick: b"never\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GFilesystemPreviewType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_monitor_event_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 12] = [
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_CHANGED as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_CHANGED\0" as *const u8 as *const gchar,
                value_nick: b"changed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT\0" as *const u8
                    as *const gchar,
                value_nick: b"changes-done-hint\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_DELETED as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_DELETED\0" as *const u8 as *const gchar,
                value_nick: b"deleted\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_CREATED as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_CREATED\0" as *const u8 as *const gchar,
                value_nick: b"created\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED\0" as *const u8
                    as *const gchar,
                value_nick: b"attribute-changed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_PRE_UNMOUNT as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_PRE_UNMOUNT\0" as *const u8 as *const gchar,
                value_nick: b"pre-unmount\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_UNMOUNTED as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_UNMOUNTED\0" as *const u8 as *const gchar,
                value_nick: b"unmounted\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_MOVED as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_MOVED\0" as *const u8 as *const gchar,
                value_nick: b"moved\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_RENAMED as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_RENAMED\0" as *const u8 as *const gchar,
                value_nick: b"renamed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_MOVED_IN as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_MOVED_IN\0" as *const u8 as *const gchar,
                value_nick: b"moved-in\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_FILE_MONITOR_EVENT_MOVED_OUT as ::core::ffi::c_int as gint,
                value_name: b"G_FILE_MONITOR_EVENT_MOVED_OUT\0" as *const u8 as *const gchar,
                value_nick: b"moved-out\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GFileMonitorEvent\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_error_enum_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 51] = [
            _GEnumValue {
                value: G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_FOUND\0" as *const u8 as *const gchar,
                value_nick: b"not-found\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_EXISTS as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_EXISTS\0" as *const u8 as *const gchar,
                value_nick: b"exists\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_IS_DIRECTORY as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_IS_DIRECTORY\0" as *const u8 as *const gchar,
                value_nick: b"is-directory\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_DIRECTORY as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_DIRECTORY\0" as *const u8 as *const gchar,
                value_nick: b"not-directory\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_EMPTY as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_EMPTY\0" as *const u8 as *const gchar,
                value_nick: b"not-empty\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_REGULAR_FILE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_REGULAR_FILE\0" as *const u8 as *const gchar,
                value_nick: b"not-regular-file\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_SYMBOLIC_LINK as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_SYMBOLIC_LINK\0" as *const u8 as *const gchar,
                value_nick: b"not-symbolic-link\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_MOUNTABLE_FILE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_MOUNTABLE_FILE\0" as *const u8 as *const gchar,
                value_nick: b"not-mountable-file\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_FILENAME_TOO_LONG as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_FILENAME_TOO_LONG\0" as *const u8 as *const gchar,
                value_nick: b"filename-too-long\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_INVALID_FILENAME as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_INVALID_FILENAME\0" as *const u8 as *const gchar,
                value_nick: b"invalid-filename\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_TOO_MANY_LINKS as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_TOO_MANY_LINKS\0" as *const u8 as *const gchar,
                value_nick: b"too-many-links\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NO_SPACE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NO_SPACE\0" as *const u8 as *const gchar,
                value_nick: b"no-space\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_INVALID_ARGUMENT\0" as *const u8 as *const gchar,
                value_nick: b"invalid-argument\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_PERMISSION_DENIED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_PERMISSION_DENIED\0" as *const u8 as *const gchar,
                value_nick: b"permission-denied\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_SUPPORTED\0" as *const u8 as *const gchar,
                value_nick: b"not-supported\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_MOUNTED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_MOUNTED\0" as *const u8 as *const gchar,
                value_nick: b"not-mounted\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_ALREADY_MOUNTED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_ALREADY_MOUNTED\0" as *const u8 as *const gchar,
                value_nick: b"already-mounted\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_CLOSED\0" as *const u8 as *const gchar,
                value_nick: b"closed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_CANCELLED\0" as *const u8 as *const gchar,
                value_nick: b"cancelled\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_PENDING as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_PENDING\0" as *const u8 as *const gchar,
                value_nick: b"pending\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_READ_ONLY as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_READ_ONLY\0" as *const u8 as *const gchar,
                value_nick: b"read-only\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_CANT_CREATE_BACKUP\0" as *const u8 as *const gchar,
                value_nick: b"cant-create-backup\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_WRONG_ETAG as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_WRONG_ETAG\0" as *const u8 as *const gchar,
                value_nick: b"wrong-etag\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_TIMED_OUT\0" as *const u8 as *const gchar,
                value_nick: b"timed-out\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_WOULD_RECURSE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_WOULD_RECURSE\0" as *const u8 as *const gchar,
                value_nick: b"would-recurse\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_BUSY as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_BUSY\0" as *const u8 as *const gchar,
                value_nick: b"busy\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_WOULD_BLOCK as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_WOULD_BLOCK\0" as *const u8 as *const gchar,
                value_nick: b"would-block\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_HOST_NOT_FOUND as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_HOST_NOT_FOUND\0" as *const u8 as *const gchar,
                value_nick: b"host-not-found\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_WOULD_MERGE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_WOULD_MERGE\0" as *const u8 as *const gchar,
                value_nick: b"would-merge\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_FAILED_HANDLED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_FAILED_HANDLED\0" as *const u8 as *const gchar,
                value_nick: b"failed-handled\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_TOO_MANY_OPEN_FILES as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_TOO_MANY_OPEN_FILES\0" as *const u8 as *const gchar,
                value_nick: b"too-many-open-files\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_INITIALIZED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_INITIALIZED\0" as *const u8 as *const gchar,
                value_nick: b"not-initialized\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_ADDRESS_IN_USE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_ADDRESS_IN_USE\0" as *const u8 as *const gchar,
                value_nick: b"address-in-use\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_PARTIAL_INPUT\0" as *const u8 as *const gchar,
                value_nick: b"partial-input\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_INVALID_DATA as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_INVALID_DATA\0" as *const u8 as *const gchar,
                value_nick: b"invalid-data\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_DBUS_ERROR as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_DBUS_ERROR\0" as *const u8 as *const gchar,
                value_nick: b"dbus-error\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_HOST_UNREACHABLE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_HOST_UNREACHABLE\0" as *const u8 as *const gchar,
                value_nick: b"host-unreachable\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NETWORK_UNREACHABLE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NETWORK_UNREACHABLE\0" as *const u8 as *const gchar,
                value_nick: b"network-unreachable\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_CONNECTION_REFUSED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_CONNECTION_REFUSED\0" as *const u8 as *const gchar,
                value_nick: b"connection-refused\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_PROXY_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_PROXY_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"proxy-failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_PROXY_AUTH_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_PROXY_AUTH_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"proxy-auth-failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_PROXY_NEED_AUTH as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_PROXY_NEED_AUTH\0" as *const u8 as *const gchar,
                value_nick: b"proxy-need-auth\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_PROXY_NOT_ALLOWED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_PROXY_NOT_ALLOWED\0" as *const u8 as *const gchar,
                value_nick: b"proxy-not-allowed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_BROKEN_PIPE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_BROKEN_PIPE\0" as *const u8 as *const gchar,
                value_nick: b"broken-pipe\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_CONNECTION_CLOSED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_CONNECTION_CLOSED\0" as *const u8 as *const gchar,
                value_nick: b"connection-closed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NOT_CONNECTED as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NOT_CONNECTED\0" as *const u8 as *const gchar,
                value_nick: b"not-connected\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_MESSAGE_TOO_LARGE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_MESSAGE_TOO_LARGE\0" as *const u8 as *const gchar,
                value_nick: b"message-too-large\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_NO_SUCH_DEVICE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_NO_SUCH_DEVICE\0" as *const u8 as *const gchar,
                value_nick: b"no-such-device\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_ERROR_DESTINATION_UNSET as ::core::ffi::c_int as gint,
                value_name: b"G_IO_ERROR_DESTINATION_UNSET\0" as *const u8 as *const gchar,
                value_nick: b"destination-unset\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GIOErrorEnum\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ask_password_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 7] = [
            _GFlagsValue {
                value: G_ASK_PASSWORD_NEED_PASSWORD as ::core::ffi::c_int as guint,
                value_name: b"G_ASK_PASSWORD_NEED_PASSWORD\0" as *const u8 as *const gchar,
                value_nick: b"need-password\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_ASK_PASSWORD_NEED_USERNAME as ::core::ffi::c_int as guint,
                value_name: b"G_ASK_PASSWORD_NEED_USERNAME\0" as *const u8 as *const gchar,
                value_nick: b"need-username\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_ASK_PASSWORD_NEED_DOMAIN as ::core::ffi::c_int as guint,
                value_name: b"G_ASK_PASSWORD_NEED_DOMAIN\0" as *const u8 as *const gchar,
                value_nick: b"need-domain\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_ASK_PASSWORD_SAVING_SUPPORTED as ::core::ffi::c_int as guint,
                value_name: b"G_ASK_PASSWORD_SAVING_SUPPORTED\0" as *const u8 as *const gchar,
                value_nick: b"saving-supported\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_ASK_PASSWORD_ANONYMOUS_SUPPORTED as ::core::ffi::c_int as guint,
                value_name: b"G_ASK_PASSWORD_ANONYMOUS_SUPPORTED\0" as *const u8 as *const gchar,
                value_nick: b"anonymous-supported\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_ASK_PASSWORD_TCRYPT as ::core::ffi::c_int as guint,
                value_name: b"G_ASK_PASSWORD_TCRYPT\0" as *const u8 as *const gchar,
                value_nick: b"tcrypt\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GAskPasswordFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_password_save_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_PASSWORD_SAVE_NEVER as ::core::ffi::c_int as gint,
                value_name: b"G_PASSWORD_SAVE_NEVER\0" as *const u8 as *const gchar,
                value_nick: b"never\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_PASSWORD_SAVE_FOR_SESSION as ::core::ffi::c_int as gint,
                value_name: b"G_PASSWORD_SAVE_FOR_SESSION\0" as *const u8 as *const gchar,
                value_nick: b"for-session\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_PASSWORD_SAVE_PERMANENTLY as ::core::ffi::c_int as gint,
                value_name: b"G_PASSWORD_SAVE_PERMANENTLY\0" as *const u8 as *const gchar,
                value_nick: b"permanently\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GPasswordSave\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mount_operation_result_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_MOUNT_OPERATION_HANDLED as ::core::ffi::c_int as gint,
                value_name: b"G_MOUNT_OPERATION_HANDLED\0" as *const u8 as *const gchar,
                value_nick: b"handled\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_MOUNT_OPERATION_ABORTED as ::core::ffi::c_int as gint,
                value_name: b"G_MOUNT_OPERATION_ABORTED\0" as *const u8 as *const gchar,
                value_nick: b"aborted\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_MOUNT_OPERATION_UNHANDLED as ::core::ffi::c_int as gint,
                value_name: b"G_MOUNT_OPERATION_UNHANDLED\0" as *const u8 as *const gchar,
                value_nick: b"unhandled\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GMountOperationResult\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_output_stream_splice_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 4] = [
            _GFlagsValue {
                value: G_OUTPUT_STREAM_SPLICE_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_OUTPUT_STREAM_SPLICE_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE as ::core::ffi::c_int as guint,
                value_name: b"G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE\0" as *const u8 as *const gchar,
                value_nick: b"close-source\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET as ::core::ffi::c_int as guint,
                value_name: b"G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET\0" as *const u8 as *const gchar,
                value_nick: b"close-target\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GOutputStreamSpliceFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_stream_splice_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_IO_STREAM_SPLICE_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_IO_STREAM_SPLICE_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_STREAM_SPLICE_CLOSE_STREAM1 as ::core::ffi::c_int as guint,
                value_name: b"G_IO_STREAM_SPLICE_CLOSE_STREAM1\0" as *const u8 as *const gchar,
                value_nick: b"close-stream1\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_STREAM_SPLICE_CLOSE_STREAM2 as ::core::ffi::c_int as guint,
                value_name: b"G_IO_STREAM_SPLICE_CLOSE_STREAM2\0" as *const u8 as *const gchar,
                value_nick: b"close-stream2\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_IO_STREAM_SPLICE_WAIT_FOR_BOTH as ::core::ffi::c_int as guint,
                value_name: b"G_IO_STREAM_SPLICE_WAIT_FOR_BOTH\0" as *const u8 as *const gchar,
                value_nick: b"wait-for-both\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GIOStreamSpliceFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_emblem_origin_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_EMBLEM_ORIGIN_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_EMBLEM_ORIGIN_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_EMBLEM_ORIGIN_DEVICE as ::core::ffi::c_int as gint,
                value_name: b"G_EMBLEM_ORIGIN_DEVICE\0" as *const u8 as *const gchar,
                value_nick: b"device\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_EMBLEM_ORIGIN_LIVEMETADATA as ::core::ffi::c_int as gint,
                value_name: b"G_EMBLEM_ORIGIN_LIVEMETADATA\0" as *const u8 as *const gchar,
                value_nick: b"livemetadata\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_EMBLEM_ORIGIN_TAG as ::core::ffi::c_int as gint,
                value_name: b"G_EMBLEM_ORIGIN_TAG\0" as *const u8 as *const gchar,
                value_nick: b"tag\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GEmblemOrigin\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_error_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_RESOLVER_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_ERROR_NOT_FOUND\0" as *const u8 as *const gchar,
                value_nick: b"not-found\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_RESOLVER_ERROR_TEMPORARY_FAILURE as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_ERROR_TEMPORARY_FAILURE\0" as *const u8 as *const gchar,
                value_nick: b"temporary-failure\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_ERROR_INTERNAL\0" as *const u8 as *const gchar,
                value_nick: b"internal\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GResolverError\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_record_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 6] = [
            _GEnumValue {
                value: G_RESOLVER_RECORD_SRV as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_RECORD_SRV\0" as *const u8 as *const gchar,
                value_nick: b"srv\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_RESOLVER_RECORD_MX as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_RECORD_MX\0" as *const u8 as *const gchar,
                value_nick: b"mx\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_RESOLVER_RECORD_TXT as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_RECORD_TXT\0" as *const u8 as *const gchar,
                value_nick: b"txt\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_RESOLVER_RECORD_SOA as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_RECORD_SOA\0" as *const u8 as *const gchar,
                value_nick: b"soa\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_RESOLVER_RECORD_NS as ::core::ffi::c_int as gint,
                value_name: b"G_RESOLVER_RECORD_NS\0" as *const u8 as *const gchar,
                value_nick: b"ns\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GResolverRecordType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_error_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 3] = [
            _GEnumValue {
                value: G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                value_name: b"G_RESOURCE_ERROR_NOT_FOUND\0" as *const u8 as *const gchar,
                value_nick: b"not-found\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_RESOURCE_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                value_name: b"G_RESOURCE_ERROR_INTERNAL\0" as *const u8 as *const gchar,
                value_nick: b"internal\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GResourceError\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_RESOURCE_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_RESOURCE_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_RESOURCE_FLAGS_COMPRESSED as ::core::ffi::c_int as guint,
                value_name: b"G_RESOURCE_FLAGS_COMPRESSED\0" as *const u8 as *const gchar,
                value_nick: b"compressed\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GResourceFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_lookup_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 2] = [
            _GFlagsValue {
                value: G_RESOURCE_LOOKUP_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_RESOURCE_LOOKUP_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GResourceLookupFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_family_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_SOCKET_FAMILY_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_FAMILY_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_FAMILY_UNIX as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_FAMILY_UNIX\0" as *const u8 as *const gchar,
                value_nick: b"unix\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_FAMILY_IPV4 as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_FAMILY_IPV4\0" as *const u8 as *const gchar,
                value_nick: b"ipv4\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_FAMILY_IPV6 as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_FAMILY_IPV6\0" as *const u8 as *const gchar,
                value_nick: b"ipv6\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GSocketFamily\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_SOCKET_TYPE_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_TYPE_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_TYPE_STREAM as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_TYPE_STREAM\0" as *const u8 as *const gchar,
                value_nick: b"stream\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_TYPE_DATAGRAM as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_TYPE_DATAGRAM\0" as *const u8 as *const gchar,
                value_nick: b"datagram\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_TYPE_SEQPACKET as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_TYPE_SEQPACKET\0" as *const u8 as *const gchar,
                value_nick: b"seqpacket\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GSocketType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_msg_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_SOCKET_MSG_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_SOCKET_MSG_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SOCKET_MSG_OOB as ::core::ffi::c_int as guint,
                value_name: b"G_SOCKET_MSG_OOB\0" as *const u8 as *const gchar,
                value_nick: b"oob\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SOCKET_MSG_PEEK as ::core::ffi::c_int as guint,
                value_name: b"G_SOCKET_MSG_PEEK\0" as *const u8 as *const gchar,
                value_nick: b"peek\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SOCKET_MSG_DONTROUTE as ::core::ffi::c_int as guint,
                value_name: b"G_SOCKET_MSG_DONTROUTE\0" as *const u8 as *const gchar,
                value_nick: b"dontroute\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GSocketMsgFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_protocol_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 6] = [
            _GEnumValue {
                value: G_SOCKET_PROTOCOL_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_PROTOCOL_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_PROTOCOL_DEFAULT as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_PROTOCOL_DEFAULT\0" as *const u8 as *const gchar,
                value_nick: b"default\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_PROTOCOL_TCP as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_PROTOCOL_TCP\0" as *const u8 as *const gchar,
                value_nick: b"tcp\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_PROTOCOL_UDP as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_PROTOCOL_UDP\0" as *const u8 as *const gchar,
                value_nick: b"udp\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_PROTOCOL_SCTP as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_PROTOCOL_SCTP\0" as *const u8 as *const gchar,
                value_nick: b"sctp\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GSocketProtocol\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_zlib_compressor_format_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_ZLIB_COMPRESSOR_FORMAT_ZLIB as ::core::ffi::c_int as gint,
                value_name: b"G_ZLIB_COMPRESSOR_FORMAT_ZLIB\0" as *const u8 as *const gchar,
                value_nick: b"zlib\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_ZLIB_COMPRESSOR_FORMAT_GZIP as ::core::ffi::c_int as gint,
                value_name: b"G_ZLIB_COMPRESSOR_FORMAT_GZIP\0" as *const u8 as *const gchar,
                value_nick: b"gzip\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_ZLIB_COMPRESSOR_FORMAT_RAW as ::core::ffi::c_int as gint,
                value_name: b"G_ZLIB_COMPRESSOR_FORMAT_RAW\0" as *const u8 as *const gchar,
                value_nick: b"raw\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GZlibCompressorFormat\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unix_socket_address_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 6] = [
            _GEnumValue {
                value: G_UNIX_SOCKET_ADDRESS_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_UNIX_SOCKET_ADDRESS_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNIX_SOCKET_ADDRESS_ANONYMOUS as ::core::ffi::c_int as gint,
                value_name: b"G_UNIX_SOCKET_ADDRESS_ANONYMOUS\0" as *const u8 as *const gchar,
                value_nick: b"anonymous\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNIX_SOCKET_ADDRESS_PATH as ::core::ffi::c_int as gint,
                value_name: b"G_UNIX_SOCKET_ADDRESS_PATH\0" as *const u8 as *const gchar,
                value_nick: b"path\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNIX_SOCKET_ADDRESS_ABSTRACT as ::core::ffi::c_int as gint,
                value_name: b"G_UNIX_SOCKET_ADDRESS_ABSTRACT\0" as *const u8 as *const gchar,
                value_nick: b"abstract\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED as ::core::ffi::c_int as gint,
                value_name: b"G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED\0" as *const u8 as *const gchar,
                value_nick: b"abstract-padded\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GUnixSocketAddressType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_BUS_TYPE_STARTER as ::core::ffi::c_int as gint,
                value_name: b"G_BUS_TYPE_STARTER\0" as *const u8 as *const gchar,
                value_nick: b"starter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_BUS_TYPE_NONE as ::core::ffi::c_int as gint,
                value_name: b"G_BUS_TYPE_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_BUS_TYPE_SYSTEM as ::core::ffi::c_int as gint,
                value_name: b"G_BUS_TYPE_SYSTEM\0" as *const u8 as *const gchar,
                value_nick: b"system\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_BUS_TYPE_SESSION as ::core::ffi::c_int as gint,
                value_name: b"G_BUS_TYPE_SESSION\0" as *const u8 as *const gchar,
                value_nick: b"session\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GBusType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_name_owner_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_BUS_NAME_OWNER_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_BUS_NAME_OWNER_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT as ::core::ffi::c_int as guint,
                value_name: b"G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT\0" as *const u8
                    as *const gchar,
                value_nick: b"allow-replacement\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_BUS_NAME_OWNER_FLAGS_REPLACE as ::core::ffi::c_int as guint,
                value_name: b"G_BUS_NAME_OWNER_FLAGS_REPLACE\0" as *const u8 as *const gchar,
                value_nick: b"replace\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_BUS_NAME_OWNER_FLAGS_DO_NOT_QUEUE as ::core::ffi::c_int as guint,
                value_name: b"G_BUS_NAME_OWNER_FLAGS_DO_NOT_QUEUE\0" as *const u8 as *const gchar,
                value_nick: b"do-not-queue\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GBusNameOwnerFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_name_watcher_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_BUS_NAME_WATCHER_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_BUS_NAME_WATCHER_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_BUS_NAME_WATCHER_FLAGS_AUTO_START as ::core::ffi::c_int as guint,
                value_name: b"G_BUS_NAME_WATCHER_FLAGS_AUTO_START\0" as *const u8 as *const gchar,
                value_nick: b"auto-start\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GBusNameWatcherFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 8] = [
            _GFlagsValue {
                value: G_DBUS_PROXY_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROXY_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES\0" as *const u8
                    as *const gchar,
                value_nick: b"do-not-load-properties\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS\0" as *const u8
                    as *const gchar,
                value_nick: b"do-not-connect-signals\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START\0" as *const u8 as *const gchar,
                value_nick: b"do-not-auto-start\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES\0" as *const u8
                    as *const gchar,
                value_nick: b"get-invalidated-properties\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION\0" as *const u8
                    as *const gchar,
                value_nick: b"do-not-auto-start-at-construction\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROXY_FLAGS_NO_MATCH_RULE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROXY_FLAGS_NO_MATCH_RULE\0" as *const u8 as *const gchar,
                value_nick: b"no-match-rule\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusProxyFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 46] = [
            _GEnumValue {
                value: G_DBUS_ERROR_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_NO_MEMORY as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_NO_MEMORY\0" as *const u8 as *const gchar,
                value_nick: b"no-memory\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SERVICE_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SERVICE_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"service-unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_NAME_HAS_NO_OWNER as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_NAME_HAS_NO_OWNER\0" as *const u8 as *const gchar,
                value_nick: b"name-has-no-owner\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_NO_REPLY as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_NO_REPLY\0" as *const u8 as *const gchar,
                value_nick: b"no-reply\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_IO_ERROR as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_IO_ERROR\0" as *const u8 as *const gchar,
                value_nick: b"io-error\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_BAD_ADDRESS as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_BAD_ADDRESS\0" as *const u8 as *const gchar,
                value_nick: b"bad-address\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_NOT_SUPPORTED\0" as *const u8 as *const gchar,
                value_nick: b"not-supported\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_LIMITS_EXCEEDED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_LIMITS_EXCEEDED\0" as *const u8 as *const gchar,
                value_nick: b"limits-exceeded\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_ACCESS_DENIED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_ACCESS_DENIED\0" as *const u8 as *const gchar,
                value_nick: b"access-denied\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_AUTH_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_AUTH_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"auth-failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_NO_SERVER as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_NO_SERVER\0" as *const u8 as *const gchar,
                value_nick: b"no-server\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_TIMEOUT as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_TIMEOUT\0" as *const u8 as *const gchar,
                value_nick: b"timeout\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_NO_NETWORK as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_NO_NETWORK\0" as *const u8 as *const gchar,
                value_nick: b"no-network\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_ADDRESS_IN_USE as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_ADDRESS_IN_USE\0" as *const u8 as *const gchar,
                value_nick: b"address-in-use\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_DISCONNECTED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_DISCONNECTED\0" as *const u8 as *const gchar,
                value_nick: b"disconnected\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_INVALID_ARGS\0" as *const u8 as *const gchar,
                value_nick: b"invalid-args\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_FILE_NOT_FOUND as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_FILE_NOT_FOUND\0" as *const u8 as *const gchar,
                value_nick: b"file-not-found\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_FILE_EXISTS as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_FILE_EXISTS\0" as *const u8 as *const gchar,
                value_nick: b"file-exists\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_UNKNOWN_METHOD\0" as *const u8 as *const gchar,
                value_nick: b"unknown-method\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_TIMED_OUT\0" as *const u8 as *const gchar,
                value_nick: b"timed-out\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_MATCH_RULE_NOT_FOUND as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_MATCH_RULE_NOT_FOUND\0" as *const u8 as *const gchar,
                value_nick: b"match-rule-not-found\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_MATCH_RULE_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_MATCH_RULE_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"match-rule-invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_EXEC_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_EXEC_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"spawn-exec-failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_FORK_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_FORK_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"spawn-fork-failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_CHILD_EXITED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_CHILD_EXITED\0" as *const u8 as *const gchar,
                value_nick: b"spawn-child-exited\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_CHILD_SIGNALED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_CHILD_SIGNALED\0" as *const u8 as *const gchar,
                value_nick: b"spawn-child-signaled\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"spawn-failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_SETUP_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_SETUP_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"spawn-setup-failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_CONFIG_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_CONFIG_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"spawn-config-invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_SERVICE_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_SERVICE_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"spawn-service-invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND\0" as *const u8 as *const gchar,
                value_nick: b"spawn-service-not-found\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID\0" as *const u8
                    as *const gchar,
                value_nick: b"spawn-permissions-invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_FILE_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_FILE_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"spawn-file-invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SPAWN_NO_MEMORY as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SPAWN_NO_MEMORY\0" as *const u8 as *const gchar,
                value_nick: b"spawn-no-memory\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unix-process-id-unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_INVALID_SIGNATURE as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_INVALID_SIGNATURE\0" as *const u8 as *const gchar,
                value_nick: b"invalid-signature\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_INVALID_FILE_CONTENT as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_INVALID_FILE_CONTENT\0" as *const u8 as *const gchar,
                value_nick: b"invalid-file-content\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN\0" as *const u8
                    as *const gchar,
                value_nick: b"selinux-security-context-unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"adt-audit-data-unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_OBJECT_PATH_IN_USE as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_OBJECT_PATH_IN_USE\0" as *const u8 as *const gchar,
                value_nick: b"object-path-in-use\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_UNKNOWN_OBJECT as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_UNKNOWN_OBJECT\0" as *const u8 as *const gchar,
                value_nick: b"unknown-object\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_UNKNOWN_INTERFACE as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_UNKNOWN_INTERFACE\0" as *const u8 as *const gchar,
                value_nick: b"unknown-interface\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_UNKNOWN_PROPERTY as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_UNKNOWN_PROPERTY\0" as *const u8 as *const gchar,
                value_nick: b"unknown-property\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_ERROR_PROPERTY_READ_ONLY as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_ERROR_PROPERTY_READ_ONLY\0" as *const u8 as *const gchar,
                value_nick: b"property-read-only\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GDBusError\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 9] = [
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT\0" as *const u8
                    as *const gchar,
                value_nick: b"authentication-client\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER\0" as *const u8
                    as *const gchar,
                value_nick: b"authentication-server\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS\0" as *const u8
                    as *const gchar,
                value_nick: b"authentication-allow-anonymous\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION\0" as *const u8
                    as *const gchar,
                value_nick: b"message-bus-connection\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING\0" as *const u8
                    as *const gchar,
                value_nick: b"delay-message-processing\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER
                    as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER\0"
                    as *const u8 as *const gchar,
                value_nick: b"authentication-require-same-user\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE\0" as *const u8
                    as *const gchar,
                value_nick: b"cross-namespace\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusConnectionFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_capability_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_DBUS_CAPABILITY_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CAPABILITY_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING\0" as *const u8
                    as *const gchar,
                value_nick: b"unix-fd-passing\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusCapabilityFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_call_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 4] = [
            _GFlagsValue {
                value: G_DBUS_CALL_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CALL_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CALL_FLAGS_NO_AUTO_START as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_CALL_FLAGS_NO_AUTO_START\0" as *const u8 as *const gchar,
                value_nick: b"no-auto-start\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION\0" as *const u8
                    as *const gchar,
                value_nick: b"allow-interactive-authorization\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusCallFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 6] = [
            _GEnumValue {
                value: G_DBUS_MESSAGE_TYPE_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_TYPE_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_TYPE_METHOD_CALL as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_TYPE_METHOD_CALL\0" as *const u8 as *const gchar,
                value_nick: b"method-call\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_TYPE_METHOD_RETURN as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_TYPE_METHOD_RETURN\0" as *const u8 as *const gchar,
                value_nick: b"method-return\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_TYPE_ERROR as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_TYPE_ERROR\0" as *const u8 as *const gchar,
                value_nick: b"error\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_TYPE_SIGNAL as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_TYPE_SIGNAL\0" as *const u8 as *const gchar,
                value_nick: b"signal\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GDBusMessageType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_DBUS_MESSAGE_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_MESSAGE_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED\0" as *const u8
                    as *const gchar,
                value_nick: b"no-reply-expected\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_MESSAGE_FLAGS_NO_AUTO_START as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_MESSAGE_FLAGS_NO_AUTO_START\0" as *const u8 as *const gchar,
                value_nick: b"no-auto-start\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_MESSAGE_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_MESSAGE_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION\0" as *const u8
                    as *const gchar,
                value_nick: b"allow-interactive-authorization\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusMessageFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_header_field_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 11] = [
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_PATH as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_PATH\0" as *const u8 as *const gchar,
                value_nick: b"path\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE\0" as *const u8 as *const gchar,
                value_nick: b"interface\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_MEMBER as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_MEMBER\0" as *const u8 as *const gchar,
                value_nick: b"member\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME\0" as *const u8
                    as *const gchar,
                value_nick: b"error-name\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL\0" as *const u8
                    as *const gchar,
                value_nick: b"reply-serial\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION\0" as *const u8
                    as *const gchar,
                value_nick: b"destination\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_SENDER as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_SENDER\0" as *const u8 as *const gchar,
                value_nick: b"sender\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE\0" as *const u8 as *const gchar,
                value_nick: b"signature\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_HEADER_FIELD_NUM_UNIX_FDS as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_HEADER_FIELD_NUM_UNIX_FDS\0" as *const u8
                    as *const gchar,
                value_nick: b"num-unix-fds\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GDBusMessageHeaderField\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_property_info_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 4] = [
            _GFlagsValue {
                value: G_DBUS_PROPERTY_INFO_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROPERTY_INFO_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROPERTY_INFO_FLAGS_READABLE\0" as *const u8 as *const gchar,
                value_nick: b"readable\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE\0" as *const u8 as *const gchar,
                value_nick: b"writable\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusPropertyInfoFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_subtree_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_DBUS_SUBTREE_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SUBTREE_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SUBTREE_FLAGS_DISPATCH_TO_UNENUMERATED_NODES as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_SUBTREE_FLAGS_DISPATCH_TO_UNENUMERATED_NODES\0" as *const u8
                    as *const gchar,
                value_nick: b"dispatch-to-unenumerated-nodes\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusSubtreeFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_server_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_DBUS_SERVER_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SERVER_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SERVER_FLAGS_RUN_IN_THREAD as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SERVER_FLAGS_RUN_IN_THREAD\0" as *const u8 as *const gchar,
                value_nick: b"run-in-thread\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SERVER_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_SERVER_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS\0" as *const u8
                    as *const gchar,
                value_nick: b"authentication-allow-anonymous\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER\0" as *const u8
                    as *const gchar,
                value_nick: b"authentication-require-same-user\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusServerFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_signal_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 5] = [
            _GFlagsValue {
                value: G_DBUS_SIGNAL_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SIGNAL_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE\0" as *const u8 as *const gchar,
                value_nick: b"no-match-rule\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE\0" as *const u8
                    as *const gchar,
                value_nick: b"match-arg0-namespace\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH\0" as *const u8 as *const gchar,
                value_nick: b"match-arg0-path\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusSignalFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_send_message_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_DBUS_SEND_MESSAGE_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SEND_MESSAGE_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL\0" as *const u8
                    as *const gchar,
                value_nick: b"preserve-serial\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusSendMessageFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_credentials_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 9] = [
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_INVALID as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_INVALID\0" as *const u8 as *const gchar,
                value_nick: b"invalid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_LINUX_UCRED as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_LINUX_UCRED\0" as *const u8 as *const gchar,
                value_nick: b"linux-ucred\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_FREEBSD_CMSGCRED as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_FREEBSD_CMSGCRED\0" as *const u8 as *const gchar,
                value_nick: b"freebsd-cmsgcred\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_OPENBSD_SOCKPEERCRED as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_OPENBSD_SOCKPEERCRED\0" as *const u8
                    as *const gchar,
                value_nick: b"openbsd-sockpeercred\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_SOLARIS_UCRED as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_SOLARIS_UCRED\0" as *const u8 as *const gchar,
                value_nick: b"solaris-ucred\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_NETBSD_UNPCBID as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_NETBSD_UNPCBID\0" as *const u8 as *const gchar,
                value_nick: b"netbsd-unpcbid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_APPLE_XUCRED as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_APPLE_XUCRED\0" as *const u8 as *const gchar,
                value_nick: b"apple-xucred\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_CREDENTIALS_TYPE_WIN32_PID as ::core::ffi::c_int as gint,
                value_name: b"G_CREDENTIALS_TYPE_WIN32_PID\0" as *const u8 as *const gchar,
                value_nick: b"win32-pid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GCredentialsType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_byte_order_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 3] = [
            _GEnumValue {
                value: G_DBUS_MESSAGE_BYTE_ORDER_BIG_ENDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_BYTE_ORDER_BIG_ENDIAN\0" as *const u8 as *const gchar,
                value_nick: b"big-endian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_DBUS_MESSAGE_BYTE_ORDER_LITTLE_ENDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_DBUS_MESSAGE_BYTE_ORDER_LITTLE_ENDIAN\0" as *const u8
                    as *const gchar,
                value_nick: b"little-endian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GDBusMessageByteOrder\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 12] = [
            _GFlagsValue {
                value: G_APPLICATION_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"flags-none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_DEFAULT_FLAGS as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_DEFAULT_FLAGS\0" as *const u8 as *const gchar,
                value_nick: b"default-flags\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_IS_SERVICE as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_IS_SERVICE\0" as *const u8 as *const gchar,
                value_nick: b"is-service\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_IS_LAUNCHER as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_IS_LAUNCHER\0" as *const u8 as *const gchar,
                value_nick: b"is-launcher\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_HANDLES_OPEN as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_HANDLES_OPEN\0" as *const u8 as *const gchar,
                value_nick: b"handles-open\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_HANDLES_COMMAND_LINE as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_HANDLES_COMMAND_LINE\0" as *const u8 as *const gchar,
                value_nick: b"handles-command-line\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_SEND_ENVIRONMENT as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_SEND_ENVIRONMENT\0" as *const u8 as *const gchar,
                value_nick: b"send-environment\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_NON_UNIQUE as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_NON_UNIQUE\0" as *const u8 as *const gchar,
                value_nick: b"non-unique\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_CAN_OVERRIDE_APP_ID as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_CAN_OVERRIDE_APP_ID\0" as *const u8 as *const gchar,
                value_nick: b"can-override-app-id\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_ALLOW_REPLACEMENT as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_ALLOW_REPLACEMENT\0" as *const u8 as *const gchar,
                value_nick: b"allow-replacement\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_APPLICATION_REPLACE as ::core::ffi::c_int as guint,
                value_name: b"G_APPLICATION_REPLACE\0" as *const u8 as *const gchar,
                value_nick: b"replace\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GApplicationFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_error_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 10] = [
            _GEnumValue {
                value: G_TLS_ERROR_UNAVAILABLE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_UNAVAILABLE\0" as *const u8 as *const gchar,
                value_nick: b"unavailable\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_MISC as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_MISC\0" as *const u8 as *const gchar,
                value_nick: b"misc\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_BAD_CERTIFICATE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_BAD_CERTIFICATE\0" as *const u8 as *const gchar,
                value_nick: b"bad-certificate\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_NOT_TLS as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_NOT_TLS\0" as *const u8 as *const gchar,
                value_nick: b"not-tls\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_HANDSHAKE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_HANDSHAKE\0" as *const u8 as *const gchar,
                value_nick: b"handshake\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_CERTIFICATE_REQUIRED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_CERTIFICATE_REQUIRED\0" as *const u8 as *const gchar,
                value_nick: b"certificate-required\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_EOF as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_EOF\0" as *const u8 as *const gchar,
                value_nick: b"eof\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_INAPPROPRIATE_FALLBACK as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_INAPPROPRIATE_FALLBACK\0" as *const u8 as *const gchar,
                value_nick: b"inappropriate-fallback\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_ERROR_BAD_CERTIFICATE_PASSWORD as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_ERROR_BAD_CERTIFICATE_PASSWORD\0" as *const u8 as *const gchar,
                value_nick: b"bad-certificate-password\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsError\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 10] = [
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_NO_FLAGS as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_NO_FLAGS\0" as *const u8 as *const gchar,
                value_nick: b"no-flags\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_UNKNOWN_CA as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_UNKNOWN_CA\0" as *const u8 as *const gchar,
                value_nick: b"unknown-ca\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_BAD_IDENTITY as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_BAD_IDENTITY\0" as *const u8 as *const gchar,
                value_nick: b"bad-identity\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_NOT_ACTIVATED as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_NOT_ACTIVATED\0" as *const u8 as *const gchar,
                value_nick: b"not-activated\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_EXPIRED as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_EXPIRED\0" as *const u8 as *const gchar,
                value_nick: b"expired\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_REVOKED as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_REVOKED\0" as *const u8 as *const gchar,
                value_nick: b"revoked\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_INSECURE as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_INSECURE\0" as *const u8 as *const gchar,
                value_nick: b"insecure\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_GENERIC_ERROR as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_GENERIC_ERROR\0" as *const u8 as *const gchar,
                value_nick: b"generic-error\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_CERTIFICATE_VALIDATE_ALL as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_CERTIFICATE_VALIDATE_ALL\0" as *const u8 as *const gchar,
                value_nick: b"validate-all\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GTlsCertificateFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_authentication_mode_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_TLS_AUTHENTICATION_NONE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_AUTHENTICATION_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_AUTHENTICATION_REQUESTED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_AUTHENTICATION_REQUESTED\0" as *const u8 as *const gchar,
                value_nick: b"requested\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_AUTHENTICATION_REQUIRED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_AUTHENTICATION_REQUIRED\0" as *const u8 as *const gchar,
                value_nick: b"required\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsAuthenticationMode\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_channel_binding_type_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_TLS_UNIQUE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_TLS_UNIQUE\0" as *const u8 as *const gchar,
                value_nick: b"unique\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_TLS_SERVER_END_POINT as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_TLS_SERVER_END_POINT\0" as *const u8
                    as *const gchar,
                value_nick: b"server-end-point\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_TLS_EXPORTER as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_TLS_EXPORTER\0" as *const u8 as *const gchar,
                value_nick: b"exporter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsChannelBindingType\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_channel_binding_error_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 6] = [
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_ERROR_NOT_IMPLEMENTED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_ERROR_NOT_IMPLEMENTED\0" as *const u8
                    as *const gchar,
                value_nick: b"not-implemented\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_ERROR_INVALID_STATE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_ERROR_INVALID_STATE\0" as *const u8
                    as *const gchar,
                value_nick: b"invalid-state\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_ERROR_NOT_AVAILABLE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_ERROR_NOT_AVAILABLE\0" as *const u8
                    as *const gchar,
                value_nick: b"not-available\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_ERROR_NOT_SUPPORTED\0" as *const u8
                    as *const gchar,
                value_nick: b"not-supported\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_CHANNEL_BINDING_ERROR_GENERAL_ERROR as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CHANNEL_BINDING_ERROR_GENERAL_ERROR\0" as *const u8
                    as *const gchar,
                value_nick: b"general-error\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsChannelBindingError\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_rehandshake_mode_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_TLS_REHANDSHAKE_NEVER as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_REHANDSHAKE_NEVER\0" as *const u8 as *const gchar,
                value_nick: b"never\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_REHANDSHAKE_SAFELY as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_REHANDSHAKE_SAFELY\0" as *const u8 as *const gchar,
                value_nick: b"safely\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_REHANDSHAKE_UNSAFELY as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_REHANDSHAKE_UNSAFELY\0" as *const u8 as *const gchar,
                value_nick: b"unsafely\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsRehandshakeMode\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_password_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 8] = [
            _GFlagsValue {
                value: G_TLS_PASSWORD_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_PASSWORD_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_PASSWORD_RETRY as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_PASSWORD_RETRY\0" as *const u8 as *const gchar,
                value_nick: b"retry\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_PASSWORD_MANY_TRIES as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_PASSWORD_MANY_TRIES\0" as *const u8 as *const gchar,
                value_nick: b"many-tries\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_PASSWORD_FINAL_TRY as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_PASSWORD_FINAL_TRY\0" as *const u8 as *const gchar,
                value_nick: b"final-try\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_PASSWORD_PKCS11_USER as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_PASSWORD_PKCS11_USER\0" as *const u8 as *const gchar,
                value_nick: b"pkcs11-user\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_PASSWORD_PKCS11_SECURITY_OFFICER as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_PASSWORD_PKCS11_SECURITY_OFFICER\0" as *const u8
                    as *const gchar,
                value_nick: b"pkcs11-security-officer\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_TLS_PASSWORD_PKCS11_CONTEXT_SPECIFIC as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_PASSWORD_PKCS11_CONTEXT_SPECIFIC\0" as *const u8
                    as *const gchar,
                value_nick: b"pkcs11-context-specific\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GTlsPasswordFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_interaction_result_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_TLS_INTERACTION_UNHANDLED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_INTERACTION_UNHANDLED\0" as *const u8 as *const gchar,
                value_nick: b"unhandled\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_INTERACTION_HANDLED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_INTERACTION_HANDLED\0" as *const u8 as *const gchar,
                value_nick: b"handled\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_INTERACTION_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_INTERACTION_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsInteractionResult\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_DBUS_INTERFACE_SKELETON_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_INTERFACE_SKELETON_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_INTERFACE_SKELETON_FLAGS_HANDLE_METHOD_INVOCATIONS_IN_THREAD
                    as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_INTERFACE_SKELETON_FLAGS_HANDLE_METHOD_INVOCATIONS_IN_THREAD\0"
                    as *const u8 as *const gchar,
                value_nick: b"handle-method-invocations-in-thread\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusInterfaceSkeletonFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_client_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 3] = [
            _GFlagsValue {
                value: G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_NONE\0" as *const u8
                    as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_DO_NOT_AUTO_START as ::core::ffi::c_int
                    as guint,
                value_name: b"G_DBUS_OBJECT_MANAGER_CLIENT_FLAGS_DO_NOT_AUTO_START\0" as *const u8
                    as *const gchar,
                value_nick: b"do-not-auto-start\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GDBusObjectManagerClientFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_verify_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 2] = [
            _GFlagsValue {
                value: G_TLS_DATABASE_VERIFY_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_TLS_DATABASE_VERIFY_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GTlsDatabaseVerifyFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_database_lookup_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 3] = [
            _GEnumValue {
                value: G_TLS_DATABASE_LOOKUP_NONE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_DATABASE_LOOKUP_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_DATABASE_LOOKUP_KEYPAIR as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_DATABASE_LOOKUP_KEYPAIR\0" as *const u8 as *const gchar,
                value_nick: b"keypair\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsDatabaseLookupFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_certificate_request_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 2] = [
            _GEnumValue {
                value: G_TLS_CERTIFICATE_REQUEST_NONE as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_CERTIFICATE_REQUEST_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsCertificateRequestFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tls_protocol_version_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 9] = [
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_SSL_3_0 as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_SSL_3_0\0" as *const u8 as *const gchar,
                value_nick: b"ssl-3-0\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_TLS_1_0 as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_TLS_1_0\0" as *const u8 as *const gchar,
                value_nick: b"tls-1-0\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_TLS_1_1 as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_TLS_1_1\0" as *const u8 as *const gchar,
                value_nick: b"tls-1-1\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_TLS_1_2 as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_TLS_1_2\0" as *const u8 as *const gchar,
                value_nick: b"tls-1-2\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_TLS_1_3 as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_TLS_1_3\0" as *const u8 as *const gchar,
                value_nick: b"tls-1-3\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_DTLS_1_0 as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_DTLS_1_0\0" as *const u8 as *const gchar,
                value_nick: b"dtls-1-0\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_TLS_PROTOCOL_VERSION_DTLS_1_2 as ::core::ffi::c_int as gint,
                value_name: b"G_TLS_PROTOCOL_VERSION_DTLS_1_2\0" as *const u8 as *const gchar,
                value_nick: b"dtls-1-2\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GTlsProtocolVersion\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_module_scope_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 3] = [
            _GEnumValue {
                value: G_IO_MODULE_SCOPE_NONE as ::core::ffi::c_int as gint,
                value_name: b"G_IO_MODULE_SCOPE_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_IO_MODULE_SCOPE_BLOCK_DUPLICATES as ::core::ffi::c_int as gint,
                value_name: b"G_IO_MODULE_SCOPE_BLOCK_DUPLICATES\0" as *const u8 as *const gchar,
                value_nick: b"block-duplicates\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GIOModuleScopeFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_client_event_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 10] = [
            _GEnumValue {
                value: G_SOCKET_CLIENT_RESOLVING as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_RESOLVING\0" as *const u8 as *const gchar,
                value_nick: b"resolving\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_RESOLVED as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_RESOLVED\0" as *const u8 as *const gchar,
                value_nick: b"resolved\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_CONNECTING as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_CONNECTING\0" as *const u8 as *const gchar,
                value_nick: b"connecting\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_CONNECTED as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_CONNECTED\0" as *const u8 as *const gchar,
                value_nick: b"connected\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_PROXY_NEGOTIATING as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_PROXY_NEGOTIATING\0" as *const u8 as *const gchar,
                value_nick: b"proxy-negotiating\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_PROXY_NEGOTIATED as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_PROXY_NEGOTIATED\0" as *const u8 as *const gchar,
                value_nick: b"proxy-negotiated\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_TLS_HANDSHAKING as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_TLS_HANDSHAKING\0" as *const u8 as *const gchar,
                value_nick: b"tls-handshaking\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_TLS_HANDSHAKED as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_TLS_HANDSHAKED\0" as *const u8 as *const gchar,
                value_nick: b"tls-handshaked\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_CLIENT_COMPLETE as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_CLIENT_COMPLETE\0" as *const u8 as *const gchar,
                value_nick: b"complete\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GSocketClientEvent\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_socket_listener_event_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_SOCKET_LISTENER_BINDING as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_LISTENER_BINDING\0" as *const u8 as *const gchar,
                value_nick: b"binding\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_LISTENER_BOUND as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_LISTENER_BOUND\0" as *const u8 as *const gchar,
                value_nick: b"bound\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_LISTENER_LISTENING as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_LISTENER_LISTENING\0" as *const u8 as *const gchar,
                value_nick: b"listening\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_SOCKET_LISTENER_LISTENED as ::core::ffi::c_int as gint,
                value_name: b"G_SOCKET_LISTENER_LISTENED\0" as *const u8 as *const gchar,
                value_nick: b"listened\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GSocketListenerEvent\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_test_dbus_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 2] = [
            _GFlagsValue {
                value: G_TEST_DBUS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_TEST_DBUS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GTestDBusFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 11] = [
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_NONE as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_NONE\0" as *const u8 as *const gchar,
                value_nick: b"none\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_STDIN_PIPE\0" as *const u8 as *const gchar,
                value_nick: b"stdin-pipe\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_STDIN_INHERIT as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_STDIN_INHERIT\0" as *const u8 as *const gchar,
                value_nick: b"stdin-inherit\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_STDOUT_PIPE as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_STDOUT_PIPE\0" as *const u8 as *const gchar,
                value_nick: b"stdout-pipe\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_STDOUT_SILENCE as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_STDOUT_SILENCE\0" as *const u8 as *const gchar,
                value_nick: b"stdout-silence\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_STDERR_PIPE as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_STDERR_PIPE\0" as *const u8 as *const gchar,
                value_nick: b"stderr-pipe\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_STDERR_SILENCE as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_STDERR_SILENCE\0" as *const u8 as *const gchar,
                value_nick: b"stderr-silence\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_STDERR_MERGE as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_STDERR_MERGE\0" as *const u8 as *const gchar,
                value_nick: b"stderr-merge\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_INHERIT_FDS as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_INHERIT_FDS\0" as *const u8 as *const gchar,
                value_nick: b"inherit-fds\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP as ::core::ffi::c_int as guint,
                value_name: b"G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP\0" as *const u8
                    as *const gchar,
                value_nick: b"search-path-from-envp\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GSubprocessFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_notification_priority_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_NOTIFICATION_PRIORITY_NORMAL as ::core::ffi::c_int as gint,
                value_name: b"G_NOTIFICATION_PRIORITY_NORMAL\0" as *const u8 as *const gchar,
                value_nick: b"normal\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NOTIFICATION_PRIORITY_LOW as ::core::ffi::c_int as gint,
                value_name: b"G_NOTIFICATION_PRIORITY_LOW\0" as *const u8 as *const gchar,
                value_nick: b"low\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NOTIFICATION_PRIORITY_HIGH as ::core::ffi::c_int as gint,
                value_name: b"G_NOTIFICATION_PRIORITY_HIGH\0" as *const u8 as *const gchar,
                value_nick: b"high\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NOTIFICATION_PRIORITY_URGENT as ::core::ffi::c_int as gint,
                value_name: b"G_NOTIFICATION_PRIORITY_URGENT\0" as *const u8 as *const gchar,
                value_nick: b"urgent\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GNotificationPriority\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_connectivity_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 5] = [
            _GEnumValue {
                value: G_NETWORK_CONNECTIVITY_LOCAL as ::core::ffi::c_int as gint,
                value_name: b"G_NETWORK_CONNECTIVITY_LOCAL\0" as *const u8 as *const gchar,
                value_nick: b"local\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NETWORK_CONNECTIVITY_LIMITED as ::core::ffi::c_int as gint,
                value_name: b"G_NETWORK_CONNECTIVITY_LIMITED\0" as *const u8 as *const gchar,
                value_nick: b"limited\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NETWORK_CONNECTIVITY_PORTAL as ::core::ffi::c_int as gint,
                value_name: b"G_NETWORK_CONNECTIVITY_PORTAL\0" as *const u8 as *const gchar,
                value_nick: b"portal\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NETWORK_CONNECTIVITY_FULL as ::core::ffi::c_int as gint,
                value_name: b"G_NETWORK_CONNECTIVITY_FULL\0" as *const u8 as *const gchar,
                value_nick: b"full\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GNetworkConnectivity\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pollable_return_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_POLLABLE_RETURN_FAILED as ::core::ffi::c_int as gint,
                value_name: b"G_POLLABLE_RETURN_FAILED\0" as *const u8 as *const gchar,
                value_nick: b"failed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_POLLABLE_RETURN_OK as ::core::ffi::c_int as gint,
                value_name: b"G_POLLABLE_RETURN_OK\0" as *const u8 as *const gchar,
                value_nick: b"ok\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_POLLABLE_RETURN_WOULD_BLOCK as ::core::ffi::c_int as gint,
                value_name: b"G_POLLABLE_RETURN_WOULD_BLOCK\0" as *const u8 as *const gchar,
                value_nick: b"would-block\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GPollableReturn\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_monitor_warning_level_get_type() -> GType {
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
        static mut safe_c2rust_values: [GEnumValue; 4] = [
            _GEnumValue {
                value: G_MEMORY_MONITOR_WARNING_LEVEL_LOW as ::core::ffi::c_int as gint,
                value_name: b"G_MEMORY_MONITOR_WARNING_LEVEL_LOW\0" as *const u8 as *const gchar,
                value_nick: b"low\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_MEMORY_MONITOR_WARNING_LEVEL_MEDIUM as ::core::ffi::c_int as gint,
                value_name: b"G_MEMORY_MONITOR_WARNING_LEVEL_MEDIUM\0" as *const u8 as *const gchar,
                value_nick: b"medium\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_MEMORY_MONITOR_WARNING_LEVEL_CRITICAL as ::core::ffi::c_int as gint,
                value_name: b"G_MEMORY_MONITOR_WARNING_LEVEL_CRITICAL\0" as *const u8
                    as *const gchar,
                value_nick: b"critical\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GMemoryMonitorWarningLevel\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GEnumValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_name_lookup_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 4] = [
            _GFlagsValue {
                value: G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT as ::core::ffi::c_int as guint,
                value_name: b"G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT\0" as *const u8 as *const gchar,
                value_nick: b"default\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY as ::core::ffi::c_int as guint,
                value_name: b"G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY\0" as *const u8
                    as *const gchar,
                value_nick: b"ipv4-only\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY as ::core::ffi::c_int as guint,
                value_name: b"G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY\0" as *const u8
                    as *const gchar,
                value_nick: b"ipv6-only\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GResolverNameLookupFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_bind_flags_get_type() -> GType {
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
        static mut safe_c2rust_values: [GFlagsValue; 7] = [
            _GFlagsValue {
                value: G_SETTINGS_BIND_DEFAULT as ::core::ffi::c_int as guint,
                value_name: b"G_SETTINGS_BIND_DEFAULT\0" as *const u8 as *const gchar,
                value_nick: b"default\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SETTINGS_BIND_GET as ::core::ffi::c_int as guint,
                value_name: b"G_SETTINGS_BIND_GET\0" as *const u8 as *const gchar,
                value_nick: b"get\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SETTINGS_BIND_SET as ::core::ffi::c_int as guint,
                value_name: b"G_SETTINGS_BIND_SET\0" as *const u8 as *const gchar,
                value_nick: b"set\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SETTINGS_BIND_NO_SENSITIVITY as ::core::ffi::c_int as guint,
                value_name: b"G_SETTINGS_BIND_NO_SENSITIVITY\0" as *const u8 as *const gchar,
                value_nick: b"no-sensitivity\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SETTINGS_BIND_GET_NO_CHANGES as ::core::ffi::c_int as guint,
                value_name: b"G_SETTINGS_BIND_GET_NO_CHANGES\0" as *const u8 as *const gchar,
                value_nick: b"get-no-changes\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: G_SETTINGS_BIND_INVERT_BOOLEAN as ::core::ffi::c_int as guint,
                value_name: b"G_SETTINGS_BIND_INVERT_BOOLEAN\0" as *const u8 as *const gchar,
                value_nick: b"invert-boolean\0" as *const u8 as *const gchar,
            },
            _GFlagsValue {
                value: 0 as guint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_flags_register_static(
            g_intern_static_string(b"GSettingsBindFlags\0" as *const u8 as *const gchar),
            &raw const safe_c2rust_values as *const GFlagsValue,
        );
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
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
