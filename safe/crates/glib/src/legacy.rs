use std::ffi::c_void;

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

pub(crate) unsafe fn byte_array_new_take(data: *mut crate::ffi::guint8, len: gsize) -> *mut GByteArray {
    crate::translated::garray::safe_c2rust_g_byte_array_new_take(data, len as _).cast()
}

pub(crate) unsafe fn get_charset(charset: *mut *const gchar) -> gboolean {
    crate::translated::gcharset::safe_c2rust_g_get_charset(charset)
}

pub(crate) unsafe fn get_filename_charsets(filename_charsets: *mut *const *const gchar) -> gboolean {
    crate::translated::gconvert::safe_c2rust_g_get_filename_charsets(filename_charsets.cast())
}

pub(crate) unsafe fn locale_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    crate::translated::gconvert::safe_c2rust_g_locale_to_utf8(
        opsysstring,
        len as _,
        bytes_read.cast(),
        bytes_written.cast(),
        error.cast(),
    )
}

pub(crate) unsafe fn locale_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    crate::translated::gconvert::safe_c2rust_g_locale_from_utf8(
        utf8string,
        len as _,
        bytes_read.cast(),
        bytes_written.cast(),
        error.cast(),
    )
}

pub(crate) unsafe fn filename_to_utf8(
    opsysstring: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    crate::translated::gconvert::safe_c2rust_g_filename_to_utf8(
        opsysstring,
        len as _,
        bytes_read.cast(),
        bytes_written.cast(),
        error.cast(),
    )
}

pub(crate) unsafe fn filename_from_utf8(
    utf8string: *const gchar,
    len: gssize,
    bytes_read: *mut gsize,
    bytes_written: *mut gsize,
    error: *mut *mut GError,
) -> *mut gchar {
    crate::translated::gconvert::safe_c2rust_g_filename_from_utf8(
        utf8string,
        len as _,
        bytes_read.cast(),
        bytes_written.cast(),
        error.cast(),
    )
}

pub(crate) unsafe fn filename_display_name(filename: *const gchar) -> *mut gchar {
    crate::translated::gconvert::safe_c2rust_g_filename_display_name(filename)
}

pub(crate) unsafe fn filename_display_basename(filename: *const gchar) -> *mut gchar {
    crate::translated::gconvert::safe_c2rust_g_filename_display_basename(filename)
}

pub(crate) unsafe fn canonicalize_filename(filename: *const gchar, relative_to: *const gchar) -> *mut gchar {
    crate::translated::gfileutils::safe_c2rust_g_canonicalize_filename(filename, relative_to)
}

pub(crate) unsafe fn key_file_load_from_data(
    key_file: *mut GKeyFile,
    data: *const gchar,
    length: gsize,
    flags: gint,
    error: *mut *mut GError,
) -> gboolean {
    crate::translated::gkeyfile::safe_c2rust_g_key_file_load_from_data(
        key_file.cast(),
        data,
        length as _,
        flags as _,
        error.cast(),
    )
}

pub(crate) unsafe fn markup_parse_context_new(
    parser: *const GMarkupParser,
    flags: GMarkupParseFlags,
    user_data: gpointer,
    user_data_dnotify: GDestroyNotify,
) -> *mut GMarkupParseContext {
    crate::translated::gmarkup::safe_c2rust_g_markup_parse_context_new(
        parser.cast(),
        flags,
        user_data,
        user_data_dnotify,
    )
    .cast()
}

pub(crate) unsafe fn markup_parse_context_ref(context: *mut GMarkupParseContext) -> *mut GMarkupParseContext {
    crate::translated::gmarkup::safe_c2rust_g_markup_parse_context_ref(context.cast()).cast()
}

pub(crate) unsafe fn markup_parse_context_unref(context: *mut GMarkupParseContext) {
    crate::translated::gmarkup::safe_c2rust_g_markup_parse_context_unref(context.cast())
}

pub(crate) unsafe fn markup_parse_context_free(context: *mut GMarkupParseContext) {
    crate::translated::gmarkup::safe_c2rust_g_markup_parse_context_free(context.cast())
}

pub(crate) unsafe fn markup_parse_context_parse(
    context: *mut GMarkupParseContext,
    text: *const gchar,
    text_len: gssize,
    error: *mut *mut GError,
) -> gboolean {
    crate::translated::gmarkup::safe_c2rust_g_markup_parse_context_parse(
        context.cast(),
        text,
        text_len as _,
        error.cast(),
    )
}

pub(crate) unsafe fn markup_parse_context_end_parse(
    context: *mut GMarkupParseContext,
    error: *mut *mut GError,
) -> gboolean {
    crate::translated::gmarkup::safe_c2rust_g_markup_parse_context_end_parse(context.cast(), error.cast())
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
    crate::translated::gspawn::safe_c2rust_g_spawn_async(
        working_directory,
        argv,
        envp,
        flags as _,
        child_setup,
        user_data,
        child_pid,
        error.cast(),
    )
}

#[allow(clippy::too_many_arguments)]
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
    crate::translated::gspawn::safe_c2rust_g_spawn_async_with_pipes(
        working_directory,
        argv,
        envp,
        flags as _,
        child_setup,
        user_data,
        child_pid,
        standard_input,
        standard_output,
        standard_error,
        error.cast(),
    )
}

#[allow(clippy::too_many_arguments)]
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
    crate::translated::gspawn::safe_c2rust_g_spawn_async_with_pipes_and_fds(
        working_directory,
        argv,
        envp,
        flags as _,
        child_setup,
        user_data,
        stdin_fd,
        stdout_fd,
        stderr_fd,
        source_fds,
        target_fds,
        n_fds as _,
        child_pid_out,
        stdin_pipe_out,
        stdout_pipe_out,
        stderr_pipe_out,
        error.cast(),
    )
}

#[allow(clippy::too_many_arguments)]
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
    crate::translated::gspawn::safe_c2rust_g_spawn_async_with_fds(
        working_directory,
        argv,
        envp,
        flags as _,
        child_setup,
        user_data,
        child_pid,
        stdin_fd,
        stdout_fd,
        stderr_fd,
        error.cast(),
    )
}

#[allow(clippy::too_many_arguments)]
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
    crate::translated::gspawn::safe_c2rust_g_spawn_sync(
        working_directory,
        argv,
        envp,
        flags as _,
        child_setup,
        user_data,
        standard_output,
        standard_error,
        wait_status,
        error.cast(),
    )
}

pub(crate) unsafe fn spawn_command_line_sync(
    command_line: *const gchar,
    standard_output: *mut *mut gchar,
    standard_error: *mut *mut gchar,
    wait_status: *mut gint,
    error: *mut *mut GError,
) -> gboolean {
    crate::translated::gspawn::safe_c2rust_g_spawn_command_line_sync(
        command_line,
        standard_output,
        standard_error,
        wait_status,
        error.cast(),
    )
}

pub(crate) unsafe fn spawn_command_line_async(command_line: *const gchar, error: *mut *mut GError) -> gboolean {
    crate::translated::gspawn::safe_c2rust_g_spawn_command_line_async(command_line, error.cast())
}

pub(crate) unsafe fn variant_new_from_bytes(
    type_: *const GVariantType,
    bytes: *mut GBytes,
    trusted: gboolean,
) -> *mut GVariant {
    crate::translated::gvariant_core::safe_c2rust_g_variant_new_from_bytes(
        type_ as *const crate::translated::gvariant_core::GVariantType,
        bytes as *mut crate::translated::gvariant_core::GBytes,
        trusted,
    )
    .cast()
}

pub(crate) unsafe fn variant_new_from_data(
    type_: *const GVariantType,
    data: gconstpointer,
    size: gsize,
    trusted: gboolean,
    notify: GDestroyNotify,
    user_data: *mut c_void,
) -> *mut GVariant {
    crate::translated::gvariant::safe_c2rust_g_variant_new_from_data(
        type_ as *const crate::translated::gvariant::GVariantType,
        data,
        size as _,
        trusted,
        notify,
        user_data,
    )
    .cast()
}

pub(crate) unsafe fn variant_is_normal_form(value: *mut GVariant) -> gboolean {
    crate::translated::gvariant_core::safe_c2rust_g_variant_is_normal_form(
        value as *mut crate::translated::gvariant_core::GVariant,
    )
}

pub(crate) unsafe fn variant_get_normal_form(value: *mut GVariant) -> *mut GVariant {
    crate::translated::gvariant::safe_c2rust_g_variant_get_normal_form(
        value as *mut crate::translated::gvariant::GVariant,
    )
    .cast()
}

pub(crate) unsafe fn variant_byteswap(value: *mut GVariant) -> *mut GVariant {
    crate::translated::gvariant::safe_c2rust_g_variant_byteswap(
        value as *mut crate::translated::gvariant::GVariant,
    )
    .cast()
}
