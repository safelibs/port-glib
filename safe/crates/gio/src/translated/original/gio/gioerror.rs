extern "C" {
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_file_error_from_errno(err_no: gint) -> GFileError;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
}
pub type guint32 = ::core::ffi::c_uint;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type GQuark = guint32;
pub type GFileError = ::core::ffi::c_uint;
pub const G_FILE_ERROR_FAILED: GFileError = 24;
pub const G_FILE_ERROR_NOSYS: GFileError = 23;
pub const G_FILE_ERROR_PERM: GFileError = 22;
pub const G_FILE_ERROR_IO: GFileError = 21;
pub const G_FILE_ERROR_INTR: GFileError = 20;
pub const G_FILE_ERROR_AGAIN: GFileError = 19;
pub const G_FILE_ERROR_PIPE: GFileError = 18;
pub const G_FILE_ERROR_INVAL: GFileError = 17;
pub const G_FILE_ERROR_BADF: GFileError = 16;
pub const G_FILE_ERROR_NFILE: GFileError = 15;
pub const G_FILE_ERROR_MFILE: GFileError = 14;
pub const G_FILE_ERROR_NOMEM: GFileError = 13;
pub const G_FILE_ERROR_NOSPC: GFileError = 12;
pub const G_FILE_ERROR_LOOP: GFileError = 11;
pub const G_FILE_ERROR_FAULT: GFileError = 10;
pub const G_FILE_ERROR_TXTBSY: GFileError = 9;
pub const G_FILE_ERROR_ROFS: GFileError = 8;
pub const G_FILE_ERROR_NODEV: GFileError = 7;
pub const G_FILE_ERROR_NXIO: GFileError = 6;
pub const G_FILE_ERROR_NOTDIR: GFileError = 5;
pub const G_FILE_ERROR_NOENT: GFileError = 4;
pub const G_FILE_ERROR_NAMETOOLONG: GFileError = 3;
pub const G_FILE_ERROR_ACCES: GFileError = 2;
pub const G_FILE_ERROR_ISDIR: GFileError = 1;
pub const G_FILE_ERROR_EXIST: GFileError = 0;
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
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const EAGAIN: ::core::ffi::c_int = 11;
pub const EBUSY: ::core::ffi::c_int = 16;
pub const EMLINK: ::core::ffi::c_int = 31;
pub const ENOTEMPTY: ::core::ffi::c_int = 39;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const ENOMSG: ::core::ffi::c_int = 42;
pub const ENODATA: ::core::ffi::c_int = 61;
pub const EBADMSG: ::core::ffi::c_int = 74;
pub const ENOTSOCK: ::core::ffi::c_int = 88;
pub const EDESTADDRREQ: ::core::ffi::c_int = 89;
pub const EMSGSIZE: ::core::ffi::c_int = 90;
pub const EPROTONOSUPPORT: ::core::ffi::c_int = 93;
pub const ESOCKTNOSUPPORT: ::core::ffi::c_int = 94;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95;
pub const EPFNOSUPPORT: ::core::ffi::c_int = 96;
pub const EAFNOSUPPORT: ::core::ffi::c_int = 97;
pub const EADDRINUSE: ::core::ffi::c_int = 98;
pub const ENETDOWN: ::core::ffi::c_int = 100;
pub const ENETUNREACH: ::core::ffi::c_int = 101;
pub const ECONNRESET: ::core::ffi::c_int = 104;
pub const ENOTCONN: ::core::ffi::c_int = 107;
pub const ETIMEDOUT: ::core::ffi::c_int = 110;
pub const ECONNREFUSED: ::core::ffi::c_int = 111;
pub const EHOSTUNREACH: ::core::ffi::c_int = 113;
pub const ECANCELED: ::core::ffi::c_int = 125;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-io-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_error_from_errno(mut err_no: gint) -> GIOErrorEnum {
    let mut file_error: GFileError = G_FILE_ERROR_EXIST;
    let mut io_error: GIOErrorEnum = G_IO_ERROR_FAILED;
    file_error = g_file_error_from_errno(err_no);
    io_error = safe_c2rust_g_io_error_from_file_error(file_error);
    if io_error as ::core::ffi::c_uint
        != G_IO_ERROR_FAILED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return io_error;
    }
    match err_no {
        EMLINK => return G_IO_ERROR_TOO_MANY_LINKS,
        ENOMSG => return G_IO_ERROR_INVALID_DATA,
        ENODATA => return G_IO_ERROR_INVALID_DATA,
        EBADMSG => return G_IO_ERROR_INVALID_DATA,
        ECANCELED => return G_IO_ERROR_CANCELLED,
        ENOTEMPTY => return G_IO_ERROR_NOT_EMPTY,
        ENOTSUP => return G_IO_ERROR_NOT_SUPPORTED,
        EPROTONOSUPPORT => return G_IO_ERROR_NOT_SUPPORTED,
        ESOCKTNOSUPPORT => return G_IO_ERROR_NOT_SUPPORTED,
        EPFNOSUPPORT => return G_IO_ERROR_NOT_SUPPORTED,
        EAFNOSUPPORT => return G_IO_ERROR_NOT_SUPPORTED,
        ETIMEDOUT => return G_IO_ERROR_TIMED_OUT,
        EBUSY => return G_IO_ERROR_BUSY,
        EWOULDBLOCK => return G_IO_ERROR_WOULD_BLOCK,
        EADDRINUSE => return G_IO_ERROR_ADDRESS_IN_USE,
        EHOSTUNREACH => return G_IO_ERROR_HOST_UNREACHABLE,
        ENETUNREACH => return G_IO_ERROR_NETWORK_UNREACHABLE,
        ENETDOWN => return G_IO_ERROR_NETWORK_UNREACHABLE,
        ECONNREFUSED => return G_IO_ERROR_CONNECTION_REFUSED,
        ECONNRESET => return G_IO_ERROR_CONNECTION_CLOSED,
        ENOTCONN => return G_IO_ERROR_NOT_CONNECTED,
        EDESTADDRREQ => return G_IO_ERROR_DESTINATION_UNSET,
        EMSGSIZE => return G_IO_ERROR_MESSAGE_TOO_LARGE,
        ENOTSOCK => return G_IO_ERROR_INVALID_ARGUMENT,
        _ => return G_IO_ERROR_FAILED,
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_error_from_file_error(
    mut file_error: GFileError,
) -> GIOErrorEnum {
    match file_error as ::core::ffi::c_uint {
        0 => return G_IO_ERROR_EXISTS,
        1 => return G_IO_ERROR_IS_DIRECTORY,
        2 => return G_IO_ERROR_PERMISSION_DENIED,
        3 => return G_IO_ERROR_FILENAME_TOO_LONG,
        4 => return G_IO_ERROR_NOT_FOUND,
        5 => return G_IO_ERROR_NOT_DIRECTORY,
        6 => return G_IO_ERROR_NOT_REGULAR_FILE,
        7 => return G_IO_ERROR_NO_SUCH_DEVICE,
        8 => return G_IO_ERROR_READ_ONLY,
        9 => return G_IO_ERROR_BUSY,
        11 => return G_IO_ERROR_TOO_MANY_LINKS,
        12 | 13 => return G_IO_ERROR_NO_SPACE,
        14 | 15 => return G_IO_ERROR_TOO_MANY_OPEN_FILES,
        17 => return G_IO_ERROR_INVALID_ARGUMENT,
        18 => return G_IO_ERROR_BROKEN_PIPE,
        19 => return G_IO_ERROR_WOULD_BLOCK,
        22 => return G_IO_ERROR_PERMISSION_DENIED,
        23 => return G_IO_ERROR_NOT_SUPPORTED,
        16 | 24 | 10 | 20 | 21 => return G_IO_ERROR_FAILED,
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gioerror.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                307 as ::core::ffi::c_int,
                G_STRFUNC,
            );
            return G_IO_ERROR_FAILED;
        }
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
