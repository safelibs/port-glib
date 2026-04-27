use std::ffi::c_void;
use std::sync::OnceLock;

use crate::abi::{GByteArray, GError};
use crate::ffi::{
    gboolean, gconstpointer, gchar, gint, gsize, gssize, gpointer, GDestroyNotify,
};

type GBytes = c_void;
type GKeyFile = c_void;
type GMarkupParseContext = c_void;
type GMarkupParser = c_void;
type GMarkupParseFlags = crate::ffi::guint;
#[cfg(unix)]
type GPid = crate::ffi::gint;
#[cfg(windows)]
type GPid = crate::ffi::gpointer;
type GSpawnFlags = gint;
type GSpawnChildSetupFunc = Option<unsafe extern "C" fn(gpointer)>;
type GVariant = c_void;
type GVariantType = c_void;

unsafe fn resolve_typed<T: Copy>(slot: &'static OnceLock<usize>, name: &'static [u8]) -> T {
    let address = *slot.get_or_init(|| unsafe { crate::forwarders::resolve_symbol(name) as usize });
    unsafe { core::mem::transmute_copy::<usize, T>(&address) }
}

macro_rules! oracle_fn {
    ($slot:ident, $name:literal, $ty:ty) => {{
        static $slot: OnceLock<usize> = OnceLock::new();
        unsafe { resolve_typed::<$ty>(& $slot, concat!($name, "\0").as_bytes()) }
    }};
}

pub(crate) unsafe fn byte_array_new_take(
    data: *mut crate::ffi::guint8,
    len: gsize,
) -> *mut GByteArray {
    let func: unsafe extern "C" fn(*mut crate::ffi::guint8, gsize) -> *mut GByteArray =
        oracle_fn!(G_BYTE_ARRAY_NEW_TAKE, "g_byte_array_new_take", unsafe extern "C" fn(*mut crate::ffi::guint8, gsize) -> *mut GByteArray);
    unsafe { func(data, len) }
}

pub(crate) unsafe fn get_charset(charset: *mut *const gchar) -> gboolean {
    let func: unsafe extern "C" fn(*mut *const gchar) -> gboolean =
        oracle_fn!(G_GET_CHARSET, "g_get_charset", unsafe extern "C" fn(*mut *const gchar) -> gboolean);
    unsafe { func(charset) }
}

pub(crate) unsafe fn get_filename_charsets(filename_charsets: *mut *const *const gchar) -> gboolean {
    let func: unsafe extern "C" fn(*mut *const *const gchar) -> gboolean = oracle_fn!(
        G_GET_FILENAME_CHARSETS,
        "g_get_filename_charsets",
        unsafe extern "C" fn(*mut *const *const gchar) -> gboolean
    );
    unsafe { func(filename_charsets) }
}

pub(crate) unsafe fn locale_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    let func: unsafe extern "C" fn(
        *const gchar,
        gssize,
        *mut gsize,
        *mut gsize,
        *mut *mut GError,
    ) -> *mut gchar = oracle_fn!(
        G_LOCALE_TO_UTF8,
        "g_locale_to_utf8",
        unsafe extern "C" fn(*const gchar, gssize, *mut gsize, *mut gsize, *mut *mut GError) -> *mut gchar
    );
    unsafe { func(opsysstring, len, bytes_read, bytes_written, error) }
}

pub(crate) unsafe fn locale_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    let func: unsafe extern "C" fn(
        *const gchar,
        gssize,
        *mut gsize,
        *mut gsize,
        *mut *mut GError,
    ) -> *mut gchar = oracle_fn!(
        G_LOCALE_FROM_UTF8,
        "g_locale_from_utf8",
        unsafe extern "C" fn(*const gchar, gssize, *mut gsize, *mut gsize, *mut *mut GError) -> *mut gchar
    );
    unsafe { func(utf8string, len, bytes_read, bytes_written, error) }
}

pub(crate) unsafe fn filename_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    let func: unsafe extern "C" fn(
        *const gchar,
        gssize,
        *mut gsize,
        *mut gsize,
        *mut *mut GError,
    ) -> *mut gchar = oracle_fn!(
        G_FILENAME_TO_UTF8,
        "g_filename_to_utf8",
        unsafe extern "C" fn(*const gchar, gssize, *mut gsize, *mut gsize, *mut *mut GError) -> *mut gchar
    );
    unsafe { func(opsysstring, len, bytes_read, bytes_written, error) }
}

pub(crate) unsafe fn filename_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    let func: unsafe extern "C" fn(
        *const gchar,
        gssize,
        *mut gsize,
        *mut gsize,
        *mut *mut GError,
    ) -> *mut gchar = oracle_fn!(
        G_FILENAME_FROM_UTF8,
        "g_filename_from_utf8",
        unsafe extern "C" fn(*const gchar, gssize, *mut gsize, *mut gsize, *mut *mut GError) -> *mut gchar
    );
    unsafe { func(utf8string, len, bytes_read, bytes_written, error) }
}

pub(crate) unsafe fn filename_display_name(filename: *const gchar) -> *mut gchar {
    let func: unsafe extern "C" fn(*const gchar) -> *mut gchar =
        oracle_fn!(G_FILENAME_DISPLAY_NAME, "g_filename_display_name", unsafe extern "C" fn(*const gchar) -> *mut gchar);
    unsafe { func(filename) }
}

pub(crate) unsafe fn filename_display_basename(filename: *const gchar) -> *mut gchar {
    let func: unsafe extern "C" fn(*const gchar) -> *mut gchar = oracle_fn!(
        G_FILENAME_DISPLAY_BASENAME,
        "g_filename_display_basename",
        unsafe extern "C" fn(*const gchar) -> *mut gchar
    );
    unsafe { func(filename) }
}

pub(crate) unsafe fn canonicalize_filename(
    filename: *const gchar,
    relative_to: *const gchar,
) -> *mut gchar {
    let func: unsafe extern "C" fn(*const gchar, *const gchar) -> *mut gchar = oracle_fn!(
        G_CANONICALIZE_FILENAME,
        "g_canonicalize_filename",
        unsafe extern "C" fn(*const gchar, *const gchar) -> *mut gchar
    );
    unsafe { func(filename, relative_to) }
}

pub(crate) unsafe fn key_file_load_from_data(
    key_file: *mut GKeyFile,
    data: *const gchar,
    length: gsize,
    flags: gint,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(*mut GKeyFile, *const gchar, gsize, gint, *mut *mut GError) -> gboolean =
        oracle_fn!(
            G_KEY_FILE_LOAD_FROM_DATA,
            "g_key_file_load_from_data",
            unsafe extern "C" fn(*mut GKeyFile, *const gchar, gsize, gint, *mut *mut GError) -> gboolean
        );
    unsafe { func(key_file, data, length, flags, error) }
}

pub(crate) unsafe fn markup_parse_context_new(
    parser: *const GMarkupParser,
    flags: GMarkupParseFlags,
    user_data: gpointer,
    user_data_dnotify: GDestroyNotify,
) -> *mut GMarkupParseContext {
    let func: unsafe extern "C" fn(
        *const GMarkupParser,
        GMarkupParseFlags,
        gpointer,
        GDestroyNotify,
    ) -> *mut GMarkupParseContext = oracle_fn!(
        G_MARKUP_PARSE_CONTEXT_NEW,
        "g_markup_parse_context_new",
        unsafe extern "C" fn(*const GMarkupParser, GMarkupParseFlags, gpointer, GDestroyNotify) -> *mut GMarkupParseContext
    );
    unsafe { func(parser, flags, user_data, user_data_dnotify) }
}

pub(crate) unsafe fn markup_parse_context_ref(
    context: *mut GMarkupParseContext,
) -> *mut GMarkupParseContext {
    let func: unsafe extern "C" fn(*mut GMarkupParseContext) -> *mut GMarkupParseContext = oracle_fn!(
        G_MARKUP_PARSE_CONTEXT_REF,
        "g_markup_parse_context_ref",
        unsafe extern "C" fn(*mut GMarkupParseContext) -> *mut GMarkupParseContext
    );
    unsafe { func(context) }
}

pub(crate) unsafe fn markup_parse_context_unref(context: *mut GMarkupParseContext) {
    let func: unsafe extern "C" fn(*mut GMarkupParseContext) =
        oracle_fn!(G_MARKUP_PARSE_CONTEXT_UNREF, "g_markup_parse_context_unref", unsafe extern "C" fn(*mut GMarkupParseContext));
    unsafe { func(context) }
}

pub(crate) unsafe fn markup_parse_context_free(context: *mut GMarkupParseContext) {
    let func: unsafe extern "C" fn(*mut GMarkupParseContext) =
        oracle_fn!(G_MARKUP_PARSE_CONTEXT_FREE, "g_markup_parse_context_free", unsafe extern "C" fn(*mut GMarkupParseContext));
    unsafe { func(context) }
}

pub(crate) unsafe fn markup_parse_context_parse(
    context: *mut GMarkupParseContext,
    text: *const gchar,
    text_len: gssize,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(*mut GMarkupParseContext, *const gchar, gssize, *mut *mut GError) -> gboolean =
        oracle_fn!(
            G_MARKUP_PARSE_CONTEXT_PARSE,
            "g_markup_parse_context_parse",
            unsafe extern "C" fn(*mut GMarkupParseContext, *const gchar, gssize, *mut *mut GError) -> gboolean
        );
    unsafe { func(context, text, text_len, error) }
}

pub(crate) unsafe fn markup_parse_context_end_parse(
    context: *mut GMarkupParseContext,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(*mut GMarkupParseContext, *mut *mut GError) -> gboolean =
        oracle_fn!(
            G_MARKUP_PARSE_CONTEXT_END_PARSE,
            "g_markup_parse_context_end_parse",
            unsafe extern "C" fn(*mut GMarkupParseContext, *mut *mut GError) -> gboolean
        );
    unsafe { func(context, error) }
}

pub(crate) unsafe fn spawn_async(
    working_directory: *const gchar,
    argv: *mut *mut gchar,
    envp: *mut *mut gchar,
    flags: GSpawnFlags,
    child_setup: GSpawnChildSetupFunc,
    user_data: gpointer,
    child_pid: *mut GPid,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(
        *const gchar,
        *mut *mut gchar,
        *mut *mut gchar,
        GSpawnFlags,
        GSpawnChildSetupFunc,
        gpointer,
        *mut GPid,
        *mut *mut GError,
    ) -> gboolean = oracle_fn!(
        G_SPAWN_ASYNC,
        "g_spawn_async",
        unsafe extern "C" fn(*const gchar, *mut *mut gchar, *mut *mut gchar, GSpawnFlags, GSpawnChildSetupFunc, gpointer, *mut GPid, *mut *mut GError) -> gboolean
    );
    unsafe { func(working_directory, argv, envp, flags, child_setup, user_data, child_pid, error) }
}

pub(crate) unsafe fn spawn_async_with_pipes(
    working_directory: *const gchar,
    argv: *mut *mut gchar,
    envp: *mut *mut gchar,
    flags: GSpawnFlags,
    child_setup: GSpawnChildSetupFunc,
    user_data: gpointer,
    child_pid: *mut GPid,
    standard_input: *mut gint,
    standard_output: *mut gint,
    standard_error: *mut gint,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(
        *const gchar,
        *mut *mut gchar,
        *mut *mut gchar,
        GSpawnFlags,
        GSpawnChildSetupFunc,
        gpointer,
        *mut GPid,
        *mut gint,
        *mut gint,
        *mut gint,
        *mut *mut GError,
    ) -> gboolean = oracle_fn!(
        G_SPAWN_ASYNC_WITH_PIPES,
        "g_spawn_async_with_pipes",
        unsafe extern "C" fn(*const gchar, *mut *mut gchar, *mut *mut gchar, GSpawnFlags, GSpawnChildSetupFunc, gpointer, *mut GPid, *mut gint, *mut gint, *mut gint, *mut *mut GError) -> gboolean
    );
    unsafe {
        func(
            working_directory,
            argv,
            envp,
            flags,
            child_setup,
            user_data,
            child_pid,
            standard_input,
            standard_output,
            standard_error,
            error,
        )
    }
}

pub(crate) unsafe fn spawn_async_with_pipes_and_fds(
    working_directory: *const gchar,
    argv: *const *const gchar,
    envp: *const *const gchar,
    flags: GSpawnFlags,
    child_setup: GSpawnChildSetupFunc,
    user_data: gpointer,
    stdin_fd: gint,
    stdout_fd: gint,
    stderr_fd: gint,
    source_fds: *const gint,
    target_fds: *const gint,
    n_fds: gsize,
    child_pid_out: *mut GPid,
    stdin_pipe_out: *mut gint,
    stdout_pipe_out: *mut gint,
    stderr_pipe_out: *mut gint,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(
        *const gchar,
        *const *const gchar,
        *const *const gchar,
        GSpawnFlags,
        GSpawnChildSetupFunc,
        gpointer,
        gint,
        gint,
        gint,
        *const gint,
        *const gint,
        gsize,
        *mut GPid,
        *mut gint,
        *mut gint,
        *mut gint,
        *mut *mut GError,
    ) -> gboolean = oracle_fn!(
        G_SPAWN_ASYNC_WITH_PIPES_AND_FDS,
        "g_spawn_async_with_pipes_and_fds",
        unsafe extern "C" fn(*const gchar, *const *const gchar, *const *const gchar, GSpawnFlags, GSpawnChildSetupFunc, gpointer, gint, gint, gint, *const gint, *const gint, gsize, *mut GPid, *mut gint, *mut gint, *mut gint, *mut *mut GError) -> gboolean
    );
    unsafe {
        func(
            working_directory,
            argv,
            envp,
            flags,
            child_setup,
            user_data,
            stdin_fd,
            stdout_fd,
            stderr_fd,
            source_fds,
            target_fds,
            n_fds,
            child_pid_out,
            stdin_pipe_out,
            stdout_pipe_out,
            stderr_pipe_out,
            error,
        )
    }
}

pub(crate) unsafe fn spawn_async_with_fds(
    working_directory: *const gchar,
    argv: *mut *mut gchar,
    envp: *mut *mut gchar,
    flags: GSpawnFlags,
    child_setup: GSpawnChildSetupFunc,
    user_data: gpointer,
    child_pid: *mut GPid,
    stdin_fd: gint,
    stdout_fd: gint,
    stderr_fd: gint,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(
        *const gchar,
        *mut *mut gchar,
        *mut *mut gchar,
        GSpawnFlags,
        GSpawnChildSetupFunc,
        gpointer,
        *mut GPid,
        gint,
        gint,
        gint,
        *mut *mut GError,
    ) -> gboolean = oracle_fn!(
        G_SPAWN_ASYNC_WITH_FDS,
        "g_spawn_async_with_fds",
        unsafe extern "C" fn(*const gchar, *mut *mut gchar, *mut *mut gchar, GSpawnFlags, GSpawnChildSetupFunc, gpointer, *mut GPid, gint, gint, gint, *mut *mut GError) -> gboolean
    );
    unsafe {
        func(
            working_directory,
            argv,
            envp,
            flags,
            child_setup,
            user_data,
            child_pid,
            stdin_fd,
            stdout_fd,
            stderr_fd,
            error,
        )
    }
}

pub(crate) unsafe fn spawn_sync(
    working_directory: *const gchar,
    argv: *mut *mut gchar,
    envp: *mut *mut gchar,
    flags: GSpawnFlags,
    child_setup: GSpawnChildSetupFunc,
    user_data: gpointer,
    standard_output: *mut *mut gchar,
    standard_error: *mut *mut gchar,
    wait_status: *mut gint,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(
        *const gchar,
        *mut *mut gchar,
        *mut *mut gchar,
        GSpawnFlags,
        GSpawnChildSetupFunc,
        gpointer,
        *mut *mut gchar,
        *mut *mut gchar,
        *mut gint,
        *mut *mut GError,
    ) -> gboolean = oracle_fn!(
        G_SPAWN_SYNC,
        "g_spawn_sync",
        unsafe extern "C" fn(*const gchar, *mut *mut gchar, *mut *mut gchar, GSpawnFlags, GSpawnChildSetupFunc, gpointer, *mut *mut gchar, *mut *mut gchar, *mut gint, *mut *mut GError) -> gboolean
    );
    unsafe {
        func(
            working_directory,
            argv,
            envp,
            flags,
            child_setup,
            user_data,
            standard_output,
            standard_error,
            wait_status,
            error,
        )
    }
}

pub(crate) unsafe fn spawn_command_line_sync(
    command_line: *const gchar,
    standard_output: *mut *mut gchar,
    standard_error: *mut *mut gchar,
    wait_status: *mut gint,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(
        *const gchar,
        *mut *mut gchar,
        *mut *mut gchar,
        *mut gint,
        *mut *mut GError,
    ) -> gboolean = oracle_fn!(
        G_SPAWN_COMMAND_LINE_SYNC,
        "g_spawn_command_line_sync",
        unsafe extern "C" fn(*const gchar, *mut *mut gchar, *mut *mut gchar, *mut gint, *mut *mut GError) -> gboolean
    );
    unsafe { func(command_line, standard_output, standard_error, wait_status, error) }
}

pub(crate) unsafe fn spawn_command_line_async(
    command_line: *const gchar,
    error: *mut *mut GError,
) -> gboolean {
    let func: unsafe extern "C" fn(*const gchar, *mut *mut GError) -> gboolean =
        oracle_fn!(G_SPAWN_COMMAND_LINE_ASYNC, "g_spawn_command_line_async", unsafe extern "C" fn(*const gchar, *mut *mut GError) -> gboolean);
    unsafe { func(command_line, error) }
}

pub(crate) unsafe fn variant_new_from_bytes(
    type_: *const GVariantType,
    bytes: *mut GBytes,
    trusted: gboolean,
) -> *mut GVariant {
    let func: unsafe extern "C" fn(*const GVariantType, *mut GBytes, gboolean) -> *mut GVariant =
        oracle_fn!(
            G_VARIANT_NEW_FROM_BYTES,
            "g_variant_new_from_bytes",
            unsafe extern "C" fn(*const GVariantType, *mut GBytes, gboolean) -> *mut GVariant
        );
    unsafe { func(type_, bytes, trusted) }
}

pub(crate) unsafe fn variant_new_from_data(
    type_: *const GVariantType,
    data: gconstpointer,
    size: gsize,
    trusted: gboolean,
    notify: GDestroyNotify,
    user_data: *mut c_void,
) -> *mut GVariant {
    let func: unsafe extern "C" fn(
        *const GVariantType,
        gconstpointer,
        gsize,
        gboolean,
        GDestroyNotify,
        *mut c_void,
    ) -> *mut GVariant = oracle_fn!(
        G_VARIANT_NEW_FROM_DATA,
        "g_variant_new_from_data",
        unsafe extern "C" fn(*const GVariantType, gconstpointer, gsize, gboolean, GDestroyNotify, *mut c_void) -> *mut GVariant
    );
    unsafe { func(type_, data, size, trusted, notify, user_data) }
}

pub(crate) unsafe fn variant_is_normal_form(value: *mut GVariant) -> gboolean {
    let func: unsafe extern "C" fn(*mut GVariant) -> gboolean =
        oracle_fn!(G_VARIANT_IS_NORMAL_FORM, "g_variant_is_normal_form", unsafe extern "C" fn(*mut GVariant) -> gboolean);
    unsafe { func(value) }
}

pub(crate) unsafe fn variant_get_normal_form(value: *mut GVariant) -> *mut GVariant {
    let func: unsafe extern "C" fn(*mut GVariant) -> *mut GVariant = oracle_fn!(
        G_VARIANT_GET_NORMAL_FORM,
        "g_variant_get_normal_form",
        unsafe extern "C" fn(*mut GVariant) -> *mut GVariant
    );
    unsafe { func(value) }
}

pub(crate) unsafe fn variant_byteswap(value: *mut GVariant) -> *mut GVariant {
    let func: unsafe extern "C" fn(*mut GVariant) -> *mut GVariant =
        oracle_fn!(G_VARIANT_BYTESWAP, "g_variant_byteswap", unsafe extern "C" fn(*mut GVariant) -> *mut GVariant);
    unsafe { func(value) }
}
