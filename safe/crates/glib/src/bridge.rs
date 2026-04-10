use std::ffi::c_void;

use crate::abi::{GByteArray, GError};
use crate::ffi::{gboolean, gconstpointer, gchar, gint, gsize, gssize, gpointer, GDestroyNotify};

type GBytes = c_void;
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

unsafe extern "C" {
    #[link_name = "safe_glib_legacy_g_byte_array_new_take"]
    fn raw_g_byte_array_new_take(data: *mut crate::ffi::guint8, len: gsize) -> *mut GByteArray;

    #[link_name = "safe_glib_legacy_g_get_charset"]
    fn raw_g_get_charset(charset: *mut *const gchar) -> gboolean;
    #[link_name = "safe_glib_legacy_g_get_filename_charsets"]
    fn raw_g_get_filename_charsets(filename_charsets: *mut *const *const gchar) -> gboolean;
    #[link_name = "safe_glib_legacy_g_locale_to_utf8"]
    fn raw_g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    #[link_name = "safe_glib_legacy_g_locale_from_utf8"]
    fn raw_g_locale_from_utf8(
        utf8string: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    #[link_name = "safe_glib_legacy_g_filename_to_utf8"]
    fn raw_g_filename_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    #[link_name = "safe_glib_legacy_g_filename_from_utf8"]
    fn raw_g_filename_from_utf8(
        utf8string: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    #[link_name = "safe_glib_legacy_g_filename_display_name"]
    fn raw_g_filename_display_name(filename: *const gchar) -> *mut gchar;
    #[link_name = "safe_glib_legacy_g_filename_display_basename"]
    fn raw_g_filename_display_basename(filename: *const gchar) -> *mut gchar;

    #[link_name = "safe_glib_legacy_g_markup_parse_context_new"]
    fn raw_g_markup_parse_context_new(
        parser: *const GMarkupParser,
        flags: GMarkupParseFlags,
        user_data: gpointer,
        user_data_dnotify: GDestroyNotify,
    ) -> *mut GMarkupParseContext;
    #[link_name = "safe_glib_legacy_g_markup_parse_context_ref"]
    fn raw_g_markup_parse_context_ref(context: *mut GMarkupParseContext) -> *mut GMarkupParseContext;
    #[link_name = "safe_glib_legacy_g_markup_parse_context_unref"]
    fn raw_g_markup_parse_context_unref(context: *mut GMarkupParseContext);
    #[link_name = "safe_glib_legacy_g_markup_parse_context_free"]
    fn raw_g_markup_parse_context_free(context: *mut GMarkupParseContext);
    #[link_name = "safe_glib_legacy_g_markup_parse_context_parse"]
    fn raw_g_markup_parse_context_parse(
        context: *mut GMarkupParseContext,
        text: *const gchar,
        text_len: gssize,
        error: *mut *mut GError,
    ) -> gboolean;
    #[link_name = "safe_glib_legacy_g_markup_parse_context_end_parse"]
    fn raw_g_markup_parse_context_end_parse(
        context: *mut GMarkupParseContext,
        error: *mut *mut GError,
    ) -> gboolean;

    #[link_name = "safe_glib_legacy_g_spawn_async"]
    fn raw_g_spawn_async(
        working_directory: *const gchar,
        argv: *mut *mut gchar,
        envp: *mut *mut gchar,
        flags: GSpawnFlags,
        child_setup: GSpawnChildSetupFunc,
        user_data: gpointer,
        child_pid: *mut GPid,
        error: *mut *mut GError,
    ) -> gboolean;
    #[link_name = "safe_glib_legacy_g_spawn_async_with_pipes"]
    fn raw_g_spawn_async_with_pipes(
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
    ) -> gboolean;
    #[link_name = "safe_glib_legacy_g_spawn_async_with_pipes_and_fds"]
    fn raw_g_spawn_async_with_pipes_and_fds(
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
    ) -> gboolean;
    #[link_name = "safe_glib_legacy_g_spawn_async_with_fds"]
    fn raw_g_spawn_async_with_fds(
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
    ) -> gboolean;
    #[link_name = "safe_glib_legacy_g_spawn_sync"]
    fn raw_g_spawn_sync(
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
    ) -> gboolean;
    #[link_name = "safe_glib_legacy_g_spawn_command_line_sync"]
    fn raw_g_spawn_command_line_sync(
        command_line: *const gchar,
        standard_output: *mut *mut gchar,
        standard_error: *mut *mut gchar,
        wait_status: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
    #[link_name = "safe_glib_legacy_g_spawn_command_line_async"]
    fn raw_g_spawn_command_line_async(command_line: *const gchar, error: *mut *mut GError) -> gboolean;

    #[link_name = "safe_glib_legacy_g_variant_new_from_bytes"]
    fn raw_g_variant_new_from_bytes(
        type_: *const GVariantType,
        bytes: *mut GBytes,
        trusted: gboolean,
    ) -> *mut GVariant;
    #[link_name = "safe_glib_legacy_g_variant_new_from_data"]
    fn raw_g_variant_new_from_data(
        type_: *const GVariantType,
        data: gconstpointer,
        size: gsize,
        trusted: gboolean,
        notify: GDestroyNotify,
        user_data: *mut c_void,
    ) -> *mut GVariant;
    #[link_name = "safe_glib_legacy_g_variant_is_normal_form"]
    fn raw_g_variant_is_normal_form(value: *mut GVariant) -> gboolean;
    #[link_name = "safe_glib_legacy_g_variant_get_normal_form"]
    fn raw_g_variant_get_normal_form(value: *mut GVariant) -> *mut GVariant;
    #[link_name = "safe_glib_legacy_g_variant_byteswap"]
    fn raw_g_variant_byteswap(value: *mut GVariant) -> *mut GVariant;
}

pub(crate) unsafe fn byte_array_new_take(data: *mut crate::ffi::guint8, len: gsize) -> *mut GByteArray {
    raw_g_byte_array_new_take(data, len)
}

pub(crate) unsafe fn get_charset(charset: *mut *const gchar) -> gboolean {
    raw_g_get_charset(charset)
}

pub(crate) unsafe fn get_filename_charsets(filename_charsets: *mut *const *const gchar) -> gboolean {
    raw_g_get_filename_charsets(filename_charsets)
}

pub(crate) unsafe fn locale_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    raw_g_locale_to_utf8(opsysstring, len, bytes_read, bytes_written, error)
}

pub(crate) unsafe fn locale_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    raw_g_locale_from_utf8(utf8string, len, bytes_read, bytes_written, error)
}

pub(crate) unsafe fn filename_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    raw_g_filename_to_utf8(opsysstring, len, bytes_read, bytes_written, error)
}

pub(crate) unsafe fn filename_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    raw_g_filename_from_utf8(utf8string, len, bytes_read, bytes_written, error)
}

pub(crate) unsafe fn filename_display_name(filename: *const gchar) -> *mut gchar {
    raw_g_filename_display_name(filename)
}

pub(crate) unsafe fn filename_display_basename(filename: *const gchar) -> *mut gchar {
    raw_g_filename_display_basename(filename)
}

pub(crate) unsafe fn markup_parse_context_new(
    parser: *const GMarkupParser,
    flags: GMarkupParseFlags,
    user_data: gpointer,
    user_data_dnotify: GDestroyNotify,
) -> *mut GMarkupParseContext {
    raw_g_markup_parse_context_new(parser, flags, user_data, user_data_dnotify)
}

pub(crate) unsafe fn markup_parse_context_ref(context: *mut GMarkupParseContext) -> *mut GMarkupParseContext {
    raw_g_markup_parse_context_ref(context)
}

pub(crate) unsafe fn markup_parse_context_unref(context: *mut GMarkupParseContext) {
    raw_g_markup_parse_context_unref(context)
}

pub(crate) unsafe fn markup_parse_context_free(context: *mut GMarkupParseContext) {
    raw_g_markup_parse_context_free(context)
}

pub(crate) unsafe fn markup_parse_context_parse(
    context: *mut GMarkupParseContext,
    text: *const gchar,
    text_len: gssize,
    error: *mut *mut GError,
) -> gboolean {
    raw_g_markup_parse_context_parse(context, text, text_len, error)
}

pub(crate) unsafe fn markup_parse_context_end_parse(
    context: *mut GMarkupParseContext,
    error: *mut *mut GError,
) -> gboolean {
    raw_g_markup_parse_context_end_parse(context, error)
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
    raw_g_spawn_async(working_directory, argv, envp, flags, child_setup, user_data, child_pid, error)
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
    raw_g_spawn_async_with_pipes(
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
    raw_g_spawn_async_with_pipes_and_fds(
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
    raw_g_spawn_async_with_fds(
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
    raw_g_spawn_sync(
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

pub(crate) unsafe fn spawn_command_line_sync(
    command_line: *const gchar,
    standard_output: *mut *mut gchar,
    standard_error: *mut *mut gchar,
    wait_status: *mut gint,
    error: *mut *mut GError,
) -> gboolean {
    raw_g_spawn_command_line_sync(command_line, standard_output, standard_error, wait_status, error)
}

pub(crate) unsafe fn spawn_command_line_async(command_line: *const gchar, error: *mut *mut GError) -> gboolean {
    raw_g_spawn_command_line_async(command_line, error)
}

pub(crate) unsafe fn variant_new_from_bytes(
    type_: *const GVariantType,
    bytes: *mut GBytes,
    trusted: gboolean,
) -> *mut GVariant {
    raw_g_variant_new_from_bytes(type_, bytes, trusted)
}

pub(crate) unsafe fn variant_new_from_data(
    type_: *const GVariantType,
    data: gconstpointer,
    size: gsize,
    trusted: gboolean,
    notify: GDestroyNotify,
    user_data: *mut c_void,
) -> *mut GVariant {
    raw_g_variant_new_from_data(type_, data, size, trusted, notify, user_data)
}

pub(crate) unsafe fn variant_is_normal_form(value: *mut GVariant) -> gboolean {
    raw_g_variant_is_normal_form(value)
}

pub(crate) unsafe fn variant_get_normal_form(value: *mut GVariant) -> *mut GVariant {
    raw_g_variant_get_normal_form(value)
}

pub(crate) unsafe fn variant_byteswap(value: *mut GVariant) -> *mut GVariant {
    raw_g_variant_byteswap(value)
}
