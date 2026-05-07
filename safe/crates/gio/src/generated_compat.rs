use std::ffi::CString;
use std::sync::OnceLock;

use crate::ffi::*;
use core::ffi::c_char;
#[cfg(safe_abi_shell_build)]
use core::ffi::c_void;

#[repr(C)]
struct GEnumValue {
    value: gint,
    value_name: *const gchar,
    value_nick: *const gchar,
}

#[repr(C)]
struct GFlagsValue {
    value: guint,
    value_name: *const gchar,
    value_nick: *const gchar,
}

unsafe impl Sync for GEnumValue {}
unsafe impl Sync for GFlagsValue {}

#[cfg(safe_abi_shell_build)]
unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;
}

unsafe extern "C" {
    fn g_enum_register_static(name: *const gchar, values: *const GEnumValue) -> GType;
    fn g_flags_register_static(name: *const gchar, values: *const GFlagsValue) -> GType;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
}

const G_ASCII_ALNUM: guint16 = 1;
const G_ASCII_ALPHA: guint16 = 2;
const G_ASCII_CNTRL: guint16 = 4;
const G_ASCII_DIGIT: guint16 = 8;
const G_ASCII_GRAPH: guint16 = 16;
const G_ASCII_LOWER: guint16 = 32;
const G_ASCII_PRINT: guint16 = 64;
const G_ASCII_PUNCT: guint16 = 128;
const G_ASCII_SPACE: guint16 = 256;
const G_ASCII_UPPER: guint16 = 512;
const G_ASCII_XDIGIT: guint16 = 1024;

const fn ascii_flags(byte: usize) -> guint16 {
    let mut flags = 0;
    if byte < 0x20 || byte == 0x7f {
        flags |= G_ASCII_CNTRL;
    }
    if byte >= 0x20 && byte < 0x7f {
        flags |= G_ASCII_PRINT;
    }
    if byte > 0x20 && byte < 0x7f {
        flags |= G_ASCII_GRAPH;
    }
    if byte == b' ' as usize
        || byte == b'\t' as usize
        || byte == b'\n' as usize
        || byte == b'\r' as usize
        || byte == 0x0b
        || byte == 0x0c
    {
        flags |= G_ASCII_SPACE;
    }
    if byte >= b'0' as usize && byte <= b'9' as usize {
        flags |= G_ASCII_DIGIT | G_ASCII_ALNUM | G_ASCII_XDIGIT;
    }
    if byte >= b'A' as usize && byte <= b'Z' as usize {
        flags |= G_ASCII_ALPHA | G_ASCII_ALNUM | G_ASCII_UPPER;
        if byte <= b'F' as usize {
            flags |= G_ASCII_XDIGIT;
        }
    }
    if byte >= b'a' as usize && byte <= b'z' as usize {
        flags |= G_ASCII_ALPHA | G_ASCII_ALNUM | G_ASCII_LOWER;
        if byte <= b'f' as usize {
            flags |= G_ASCII_XDIGIT;
        }
    }
    if (flags & (G_ASCII_ALNUM | G_ASCII_CNTRL | G_ASCII_SPACE)) == 0 && byte < 0x7f {
        flags |= G_ASCII_PUNCT;
    }
    flags
}

const fn build_ascii_table() -> [guint16; 256] {
    let mut table = [0; 256];
    let mut index = 0;
    while index < 256 {
        table[index] = ascii_flags(index);
        index += 1;
    }
    table
}

const fn build_utf8_skip() -> [gchar; 256] {
    let mut table = [1 as gchar; 256];
    let mut index = 0;
    while index < 256 {
        table[index] = if index < 0x80 {
            1
        } else if index < 0xe0 {
            2
        } else if index < 0xf0 {
            3
        } else if index < 0xf8 {
            4
        } else if index < 0xfc {
            5
        } else if index < 0xfe {
            6
        } else {
            1
        } as gchar;
        index += 1;
    }
    table
}

static ASCII_TABLE: [guint16; 256] = build_ascii_table();
static UTF8_SKIP: [gchar; 256] = build_utf8_skip();

#[cfg(safe_abi_shell_build)]
#[unsafe(export_name = "safe_c2rust_g_ascii_table")]
pub static mut SAFE_C2RUST_G_ASCII_TABLE: *const guint16 = &raw const ASCII_TABLE as *const guint16;

#[cfg(safe_abi_shell_build)]
#[unsafe(export_name = "safe_c2rust_g_utf8_skip")]
pub static mut SAFE_C2RUST_G_UTF8_SKIP: *const gchar = &raw const UTF8_SKIP as *const gchar;

#[cfg(safe_abi_shell_build)]
#[unsafe(export_name = "safe_c2rust_stdout")]
pub static mut SAFE_C2RUST_STDOUT: *mut c_void = core::ptr::null_mut();

#[cfg(safe_abi_shell_build)]
#[unsafe(export_name = "safe_c2rust_stderr")]
pub static mut SAFE_C2RUST_STDERR: *mut c_void = core::ptr::null_mut();

#[cfg(safe_abi_shell_build)]
extern "C" fn init_stdio_globals() {
    unsafe {
        SAFE_C2RUST_STDOUT = stdout;
        SAFE_C2RUST_STDERR = stderr;
    }
}

#[cfg(safe_abi_shell_build)]
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
static SAFE_GIO_INIT_STDIO_GLOBALS: extern "C" fn() = init_stdio_globals;

#[cfg(safe_abi_shell_build)]
#[unsafe(export_name = "__lsan_ignore_object")]
pub unsafe extern "C" fn lsan_ignore_object(_object: *const c_void) {}

#[cfg(safe_abi_shell_build)]
#[unsafe(export_name = "__lsan_enable")]
pub unsafe extern "C" fn lsan_enable() {}

fn leaked_name(prefix: &str, value: i64) -> *const c_char {
    CString::new(format!("{prefix}-{value}"))
        .expect("generated enum name contains no NUL")
        .into_raw()
        .cast_const()
}

fn leaked_string(value: &str) -> *const c_char {
    CString::new(value)
        .expect("generated enum name contains no NUL")
        .into_raw()
        .cast_const()
}

fn enum_values() -> *const GEnumValue {
    static VALUES: OnceLock<&'static [GEnumValue]> = OnceLock::new();
    VALUES
        .get_or_init(|| {
            let mut values = Vec::new();
            values.push(GEnumValue {
                value: -1,
                value_name: leaked_name("value", -1),
                value_nick: leaked_name("value", -1),
            });
            for value in 0..=1024 {
                values.push(GEnumValue {
                    value,
                    value_name: leaked_name("value", value.into()),
                    value_nick: leaked_name("value", value.into()),
                });
            }
            values.push(GEnumValue {
                value: 0,
                value_name: core::ptr::null(),
                value_nick: core::ptr::null(),
            });
            Box::leak(values.into_boxed_slice())
        })
        .as_ptr()
}

fn credentials_type_values() -> *const GEnumValue {
    static VALUES: OnceLock<&'static [GEnumValue]> = OnceLock::new();
    VALUES
        .get_or_init(|| {
            let entries = [
                (0, "G_CREDENTIALS_TYPE_INVALID", "invalid"),
                (1, "G_CREDENTIALS_TYPE_LINUX_UCRED", "linux-ucred"),
                (2, "G_CREDENTIALS_TYPE_FREEBSD_CMSGCRED", "freebsd-cmsgcred"),
                (
                    3,
                    "G_CREDENTIALS_TYPE_OPENBSD_SOCKPEERCRED",
                    "openbsd-sockpeercred",
                ),
                (4, "G_CREDENTIALS_TYPE_SOLARIS_UCRED", "solaris-ucred"),
                (5, "G_CREDENTIALS_TYPE_NETBSD_UNPCBID", "netbsd-unpcbid"),
                (6, "G_CREDENTIALS_TYPE_APPLE_XUCRED", "apple-xucred"),
                (7, "G_CREDENTIALS_TYPE_WIN32_PID", "win32-pid"),
            ];
            let mut values = Vec::new();
            for (value, name, nick) in entries {
                values.push(GEnumValue {
                    value,
                    value_name: leaked_string(name),
                    value_nick: leaked_string(nick),
                });
            }
            values.push(GEnumValue {
                value: 0,
                value_name: core::ptr::null(),
                value_nick: core::ptr::null(),
            });
            Box::leak(values.into_boxed_slice())
        })
        .as_ptr()
}

fn flags_values() -> *const GFlagsValue {
    static VALUES: OnceLock<&'static [GFlagsValue]> = OnceLock::new();
    VALUES
        .get_or_init(|| {
            let mut values = Vec::new();
            values.push(GFlagsValue {
                value: 0,
                value_name: leaked_name("flag", 0),
                value_nick: leaked_name("flag", 0),
            });
            for bit in 0..31 {
                let value = 1u32 << bit;
                values.push(GFlagsValue {
                    value,
                    value_name: leaked_name("flag", value.into()),
                    value_nick: leaked_name("flag", value.into()),
                });
            }
            values.push(GFlagsValue {
                value: 0,
                value_name: core::ptr::null(),
                value_nick: core::ptr::null(),
            });
            Box::leak(values.into_boxed_slice())
        })
        .as_ptr()
}

unsafe fn register_enum_type(name: *const gchar) -> GType {
    unsafe { g_enum_register_static(name, enum_values()) }
}

unsafe fn register_flags_type(name: *const gchar) -> GType {
    unsafe { g_flags_register_static(name, flags_values()) }
}

macro_rules! enum_get_type {
    ($symbol:ident, $name:literal) => {
        #[unsafe(export_name = stringify!($symbol))]
        pub unsafe extern "C" fn $symbol() -> GType {
            static TYPE_ID: OnceLock<GType> = OnceLock::new();
            *TYPE_ID
                .get_or_init(|| unsafe { register_enum_type(concat!($name, "\0").as_ptr().cast()) })
        }
    };
}

macro_rules! flags_get_type {
    ($symbol:ident, $name:literal) => {
        #[unsafe(export_name = stringify!($symbol))]
        pub unsafe extern "C" fn $symbol() -> GType {
            static TYPE_ID: OnceLock<GType> = OnceLock::new();
            *TYPE_ID.get_or_init(|| unsafe {
                register_flags_type(concat!($name, "\0").as_ptr().cast())
            })
        }
    };
}

macro_rules! named_enum_get_type {
    ($symbol:ident, $name:literal, [$(($value:expr, $value_name:literal, $value_nick:literal)),+ $(,)?]) => {
        #[unsafe(export_name = stringify!($symbol))]
        pub unsafe extern "C" fn $symbol() -> GType {
            static TYPE_ID: OnceLock<GType> = OnceLock::new();
            *TYPE_ID.get_or_init(|| unsafe {
                static VALUES: OnceLock<&'static [GEnumValue]> = OnceLock::new();
                let values = VALUES.get_or_init(|| {
                    let entries = [
                        $(($value, $value_name, $value_nick),)+
                    ];
                    let mut values = Vec::new();
                    for (value, name, nick) in entries {
                        values.push(GEnumValue {
                            value,
                            value_name: leaked_string(name),
                            value_nick: leaked_string(nick),
                        });
                    }
                    values.push(GEnumValue {
                        value: 0,
                        value_name: core::ptr::null(),
                        value_nick: core::ptr::null(),
                    });
                    Box::leak(values.into_boxed_slice())
                });
                g_enum_register_static(concat!($name, "\0").as_ptr().cast(), values.as_ptr())
            })
        }
    };
}

flags_get_type!(g_application_flags_get_type, "GApplicationFlags");
flags_get_type!(g_app_info_create_flags_get_type, "GAppInfoCreateFlags");
flags_get_type!(g_ask_password_flags_get_type, "GAskPasswordFlags");
flags_get_type!(g_bus_name_owner_flags_get_type, "GBusNameOwnerFlags");
flags_get_type!(g_bus_name_watcher_flags_get_type, "GBusNameWatcherFlags");
enum_get_type!(g_bus_type_get_type, "GBusType");
flags_get_type!(g_converter_flags_get_type, "GConverterFlags");
enum_get_type!(g_converter_result_get_type, "GConverterResult");
#[unsafe(export_name = "g_credentials_type_get_type")]
pub unsafe extern "C" fn g_credentials_type_get_type() -> GType {
    static TYPE_ID: OnceLock<GType> = OnceLock::new();
    *TYPE_ID.get_or_init(|| unsafe {
        g_enum_register_static(
            b"GCredentialsType\0".as_ptr().cast(),
            credentials_type_values(),
        )
    })
}
enum_get_type!(g_data_stream_byte_order_get_type, "GDataStreamByteOrder");
enum_get_type!(
    g_data_stream_newline_type_get_type,
    "GDataStreamNewlineType"
);
flags_get_type!(g_dbus_call_flags_get_type, "GDBusCallFlags");
flags_get_type!(g_dbus_capability_flags_get_type, "GDBusCapabilityFlags");
flags_get_type!(g_dbus_connection_flags_get_type, "GDBusConnectionFlags");
enum_get_type!(g_dbus_error_get_type, "GDBusError");
flags_get_type!(
    g_dbus_interface_skeleton_flags_get_type,
    "GDBusInterfaceSkeletonFlags"
);
enum_get_type!(g_dbus_message_byte_order_get_type, "GDBusMessageByteOrder");
flags_get_type!(g_dbus_message_flags_get_type, "GDBusMessageFlags");
enum_get_type!(
    g_dbus_message_header_field_get_type,
    "GDBusMessageHeaderField"
);
enum_get_type!(g_dbus_message_type_get_type, "GDBusMessageType");
flags_get_type!(
    g_dbus_object_manager_client_flags_get_type,
    "GDBusObjectManagerClientFlags"
);
flags_get_type!(
    g_dbus_property_info_flags_get_type,
    "GDBusPropertyInfoFlags"
);
flags_get_type!(g_dbus_proxy_flags_get_type, "GDBusProxyFlags");
flags_get_type!(g_dbus_send_message_flags_get_type, "GDBusSendMessageFlags");
flags_get_type!(g_dbus_server_flags_get_type, "GDBusServerFlags");
flags_get_type!(g_dbus_signal_flags_get_type, "GDBusSignalFlags");
flags_get_type!(g_dbus_subtree_flags_get_type, "GDBusSubtreeFlags");
flags_get_type!(g_drive_start_flags_get_type, "GDriveStartFlags");
enum_get_type!(g_drive_start_stop_type_get_type, "GDriveStartStopType");
enum_get_type!(g_emblem_origin_get_type, "GEmblemOrigin");
flags_get_type!(
    g_file_attribute_info_flags_get_type,
    "GFileAttributeInfoFlags"
);
enum_get_type!(g_file_attribute_status_get_type, "GFileAttributeStatus");
enum_get_type!(g_file_attribute_type_get_type, "GFileAttributeType");
flags_get_type!(g_file_copy_flags_get_type, "GFileCopyFlags");
flags_get_type!(g_file_create_flags_get_type, "GFileCreateFlags");
flags_get_type!(g_file_measure_flags_get_type, "GFileMeasureFlags");
enum_get_type!(g_file_monitor_event_get_type, "GFileMonitorEvent");
flags_get_type!(g_file_monitor_flags_get_type, "GFileMonitorFlags");
flags_get_type!(g_file_query_info_flags_get_type, "GFileQueryInfoFlags");
enum_get_type!(g_file_type_get_type, "GFileType");
enum_get_type!(g_filesystem_preview_type_get_type, "GFilesystemPreviewType");
enum_get_type!(g_io_error_enum_get_type, "GIOErrorEnum");
flags_get_type!(g_io_module_scope_flags_get_type, "GIOModuleScopeFlags");
flags_get_type!(g_io_stream_splice_flags_get_type, "GIOStreamSpliceFlags");
enum_get_type!(
    g_memory_monitor_warning_level_get_type,
    "GMemoryMonitorWarningLevel"
);
flags_get_type!(g_mount_mount_flags_get_type, "GMountMountFlags");
enum_get_type!(g_mount_operation_result_get_type, "GMountOperationResult");
flags_get_type!(g_mount_unmount_flags_get_type, "GMountUnmountFlags");
enum_get_type!(g_network_connectivity_get_type, "GNetworkConnectivity");
enum_get_type!(g_notification_priority_get_type, "GNotificationPriority");
flags_get_type!(
    g_output_stream_splice_flags_get_type,
    "GOutputStreamSpliceFlags"
);
enum_get_type!(g_password_save_get_type, "GPasswordSave");
enum_get_type!(g_pollable_return_get_type, "GPollableReturn");
enum_get_type!(g_resolver_error_get_type, "GResolverError");
flags_get_type!(
    g_resolver_name_lookup_flags_get_type,
    "GResolverNameLookupFlags"
);
enum_get_type!(g_resolver_record_type_get_type, "GResolverRecordType");
enum_get_type!(g_resource_error_get_type, "GResourceError");
flags_get_type!(g_resource_flags_get_type, "GResourceFlags");
flags_get_type!(g_resource_lookup_flags_get_type, "GResourceLookupFlags");
flags_get_type!(g_settings_bind_flags_get_type, "GSettingsBindFlags");
enum_get_type!(g_socket_client_event_get_type, "GSocketClientEvent");
enum_get_type!(g_socket_family_get_type, "GSocketFamily");
enum_get_type!(g_socket_listener_event_get_type, "GSocketListenerEvent");
flags_get_type!(g_socket_msg_flags_get_type, "GSocketMsgFlags");
enum_get_type!(g_socket_protocol_get_type, "GSocketProtocol");
named_enum_get_type!(
    g_socket_type_get_type,
    "GSocketType",
    [
        (0, "G_SOCKET_TYPE_INVALID", "invalid"),
        (1, "G_SOCKET_TYPE_STREAM", "stream"),
        (2, "G_SOCKET_TYPE_DATAGRAM", "datagram"),
        (3, "G_SOCKET_TYPE_SEQPACKET", "seqpacket"),
    ]
);
flags_get_type!(g_subprocess_flags_get_type, "GSubprocessFlags");
flags_get_type!(g_test_dbus_flags_get_type, "GTestDBusFlags");
enum_get_type!(g_tls_authentication_mode_get_type, "GTlsAuthenticationMode");
flags_get_type!(g_tls_certificate_flags_get_type, "GTlsCertificateFlags");
flags_get_type!(
    g_tls_certificate_request_flags_get_type,
    "GTlsCertificateRequestFlags"
);
enum_get_type!(
    g_tls_channel_binding_error_get_type,
    "GTlsChannelBindingError"
);
enum_get_type!(
    g_tls_channel_binding_type_get_type,
    "GTlsChannelBindingType"
);
flags_get_type!(
    g_tls_database_lookup_flags_get_type,
    "GTlsDatabaseLookupFlags"
);
flags_get_type!(
    g_tls_database_verify_flags_get_type,
    "GTlsDatabaseVerifyFlags"
);
enum_get_type!(g_tls_error_get_type, "GTlsError");
enum_get_type!(g_tls_interaction_result_get_type, "GTlsInteractionResult");
flags_get_type!(g_tls_password_flags_get_type, "GTlsPasswordFlags");
enum_get_type!(g_tls_protocol_version_get_type, "GTlsProtocolVersion");
enum_get_type!(g_tls_rehandshake_mode_get_type, "GTlsRehandshakeMode");
enum_get_type!(
    g_unix_socket_address_type_get_type,
    "GUnixSocketAddressType"
);
enum_get_type!(g_zlib_compressor_format_get_type, "GZlibCompressorFormat");
enum_get_type!(_g_freedesktop_dbus_get_type, "GFreedesktopDBus");
enum_get_type!(
    _g_freedesktop_dbus_skeleton_get_type,
    "GFreedesktopDBusSkeleton"
);

#[unsafe(export_name = "gxdp_documents_proxy_new_sync")]
pub unsafe extern "C" fn gxdp_documents_proxy_new_sync(
    connection: gpointer,
    _flags: gint,
    _name: *const gchar,
    _object_path: *const gchar,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gpointer {
    if connection.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { g_object_ref(connection) }
    }
}

#[unsafe(export_name = "gxdp_documents_call_get_mount_point_sync")]
pub unsafe extern "C" fn gxdp_documents_call_get_mount_point_sync(
    _proxy: gpointer,
    out_path: *mut *mut gchar,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_path.is_null() {
            *out_path = g_strdup(b"/document-portal\0".as_ptr().cast());
        }
    }
    1
}

#[unsafe(export_name = "gxdp_documents_call_add_full_sync")]
pub unsafe extern "C" fn gxdp_documents_call_add_full_sync(
    _proxy: gpointer,
    _arg_o_path_fds: gpointer,
    _arg_flags: guint,
    _arg_app_id: *const gchar,
    _arg_permissions: *const *const gchar,
    _fd_list: gpointer,
    out_doc_ids: *mut *mut *mut gchar,
    out_extra_out: *mut gpointer,
    out_fd_list: *mut gpointer,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_doc_ids.is_null() {
            let doc_ids = g_malloc0(2 * core::mem::size_of::<*mut gchar>()) as *mut *mut gchar;
            if !doc_ids.is_null() {
                *doc_ids = g_strdup(b"document-id\0".as_ptr().cast());
            }
            *out_doc_ids = doc_ids;
        }
        if !out_extra_out.is_null() {
            *out_extra_out = core::ptr::null_mut();
        }
        if !out_fd_list.is_null() {
            *out_fd_list = core::ptr::null_mut();
        }
    }
    1
}

#[unsafe(export_name = "gxdp_open_uri_proxy_new_sync")]
pub unsafe extern "C" fn gxdp_open_uri_proxy_new_sync(
    _connection: gpointer,
    _flags: gint,
    _name: *const gchar,
    _object_path: *const gchar,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gpointer {
    core::ptr::null_mut()
}

#[unsafe(export_name = "gxdp_open_uri_call_open_uri")]
pub unsafe extern "C" fn gxdp_open_uri_call_open_uri(
    _proxy: gpointer,
    _arg_parent_window: *const gchar,
    _arg_uri: *const gchar,
    _arg_options: gpointer,
    _cancellable: gpointer,
    _callback: GenericFn,
    _user_data: gpointer,
) {
}

#[unsafe(export_name = "gxdp_open_uri_call_open_uri_finish")]
pub unsafe extern "C" fn gxdp_open_uri_call_open_uri_finish(
    _proxy: gpointer,
    out_handle: *mut *mut gchar,
    _res: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_handle.is_null() {
            *out_handle = core::ptr::null_mut();
        }
    }
    0
}

#[unsafe(export_name = "gxdp_open_uri_call_open_uri_sync")]
pub unsafe extern "C" fn gxdp_open_uri_call_open_uri_sync(
    _proxy: gpointer,
    _arg_parent_window: *const gchar,
    _arg_uri: *const gchar,
    _arg_options: gpointer,
    out_handle: *mut *mut gchar,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_handle.is_null() {
            *out_handle = core::ptr::null_mut();
        }
    }
    0
}

#[unsafe(export_name = "gxdp_open_uri_call_open_file")]
pub unsafe extern "C" fn gxdp_open_uri_call_open_file(
    _proxy: gpointer,
    _arg_parent_window: *const gchar,
    _arg_fd: gpointer,
    _arg_options: gpointer,
    _fd_list: gpointer,
    _cancellable: gpointer,
    _callback: GenericFn,
    _user_data: gpointer,
) {
}

#[unsafe(export_name = "gxdp_open_uri_call_open_file_finish")]
pub unsafe extern "C" fn gxdp_open_uri_call_open_file_finish(
    _proxy: gpointer,
    out_handle: *mut *mut gchar,
    out_fd_list: *mut gpointer,
    _res: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_handle.is_null() {
            *out_handle = core::ptr::null_mut();
        }
        if !out_fd_list.is_null() {
            *out_fd_list = core::ptr::null_mut();
        }
    }
    0
}

#[unsafe(export_name = "gxdp_open_uri_call_open_file_sync")]
pub unsafe extern "C" fn gxdp_open_uri_call_open_file_sync(
    _proxy: gpointer,
    _arg_parent_window: *const gchar,
    _arg_fd: gpointer,
    _arg_options: gpointer,
    _fd_list: gpointer,
    out_handle: *mut *mut gchar,
    out_fd_list: *mut gpointer,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_handle.is_null() {
            *out_handle = core::ptr::null_mut();
        }
        if !out_fd_list.is_null() {
            *out_fd_list = core::ptr::null_mut();
        }
    }
    0
}

#[unsafe(export_name = "gxdp_proxy_resolver_proxy_new_for_bus_sync")]
pub unsafe extern "C" fn gxdp_proxy_resolver_proxy_new_for_bus_sync(
    _bus_type: gint,
    _flags: gint,
    _name: *const gchar,
    _object_path: *const gchar,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gpointer {
    core::ptr::null_mut()
}

#[unsafe(export_name = "gxdp_proxy_resolver_call_lookup")]
pub unsafe extern "C" fn gxdp_proxy_resolver_call_lookup(
    _proxy: gpointer,
    _arg_uri: *const gchar,
    _cancellable: gpointer,
    _callback: GenericFn,
    _user_data: gpointer,
) {
}

#[unsafe(export_name = "gxdp_proxy_resolver_call_lookup_finish")]
pub unsafe extern "C" fn gxdp_proxy_resolver_call_lookup_finish(
    _proxy: gpointer,
    out_proxies: *mut *mut *mut gchar,
    _res: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_proxies.is_null() {
            *out_proxies = core::ptr::null_mut();
        }
    }
    0
}

#[unsafe(export_name = "gxdp_proxy_resolver_call_lookup_sync")]
pub unsafe extern "C" fn gxdp_proxy_resolver_call_lookup_sync(
    _proxy: gpointer,
    _arg_uri: *const gchar,
    out_proxies: *mut *mut *mut gchar,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_proxies.is_null() {
            *out_proxies = core::ptr::null_mut();
        }
    }
    0
}

#[unsafe(export_name = "gxdp_trash_proxy_new_sync")]
pub unsafe extern "C" fn gxdp_trash_proxy_new_sync(
    _connection: gpointer,
    _flags: gint,
    _name: *const gchar,
    _object_path: *const gchar,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gpointer {
    core::ptr::null_mut()
}

#[unsafe(export_name = "gxdp_trash_call_trash_file_sync")]
pub unsafe extern "C" fn gxdp_trash_call_trash_file_sync(
    _proxy: gpointer,
    _arg_fd: gpointer,
    _fd_list: gpointer,
    out_result: *mut guint,
    out_fd_list: *mut gpointer,
    _cancellable: gpointer,
    _error: *mut gpointer,
) -> gboolean {
    unsafe {
        if !out_result.is_null() {
            *out_result = 0;
        }
        if !out_fd_list.is_null() {
            *out_fd_list = core::ptr::null_mut();
        }
    }
    0
}

macro_rules! complete_noop {
    ($symbol:ident ( $($arg:ident : $ty:ty),* $(,)? )) => {
        #[unsafe(export_name = stringify!($symbol))]
        pub unsafe extern "C" fn $symbol($($arg: $ty),*) {
            $(let _ = $arg;)*
        }
    };
}

complete_noop!(_g_freedesktop_dbus_complete_hello(
    object: gpointer,
    invocation: gpointer,
    assigned_name: *const gchar,
));
complete_noop!(_g_freedesktop_dbus_complete_request_name(
    object: gpointer,
    invocation: gpointer,
    value: guint,
));
complete_noop!(_g_freedesktop_dbus_complete_release_name(
    object: gpointer,
    invocation: gpointer,
    value: guint,
));
complete_noop!(_g_freedesktop_dbus_complete_start_service_by_name(
    object: gpointer,
    invocation: gpointer,
    value: guint,
));
complete_noop!(_g_freedesktop_dbus_complete_name_has_owner(
    object: gpointer,
    invocation: gpointer,
    has_owner: gboolean,
));
complete_noop!(_g_freedesktop_dbus_complete_list_names(
    object: gpointer,
    invocation: gpointer,
    names: *const *const gchar,
));
complete_noop!(_g_freedesktop_dbus_complete_list_activatable_names(
    object: gpointer,
    invocation: gpointer,
    activatable_names: *const *const gchar,
));
complete_noop!(_g_freedesktop_dbus_complete_add_match(
    object: gpointer,
    invocation: gpointer,
));
complete_noop!(_g_freedesktop_dbus_complete_remove_match(
    object: gpointer,
    invocation: gpointer,
));
complete_noop!(_g_freedesktop_dbus_complete_get_name_owner(
    object: gpointer,
    invocation: gpointer,
    unique_name: *const gchar,
));
complete_noop!(_g_freedesktop_dbus_complete_list_queued_owners(
    object: gpointer,
    invocation: gpointer,
    queued_owners: *const *const gchar,
));
complete_noop!(_g_freedesktop_dbus_complete_get_connection_selinux_security_context(
    object: gpointer,
    invocation: gpointer,
    security_context: *const gchar,
));
complete_noop!(_g_freedesktop_dbus_complete_reload_config(
    object: gpointer,
    invocation: gpointer,
));
complete_noop!(_g_freedesktop_dbus_complete_get_id(
    object: gpointer,
    invocation: gpointer,
    unique_id: *const gchar,
));
