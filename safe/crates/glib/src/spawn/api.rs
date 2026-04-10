use crate::abi::GError;
use crate::bridge;
use crate::ffi::{gboolean, gchar, gint, gsize, gpointer};
#[cfg(windows)]
use crate::ffi::GQuark;

#[cfg(unix)]
type GPid = crate::ffi::gint;
#[cfg(windows)]
type GPid = gpointer;

type GSpawnFlags = gint;
type GSpawnChildSetupFunc = Option<unsafe extern "C" fn(gpointer)>;

unsafe extern "C" {
    #[cfg(windows)]
    fn g_spawn_error_quark() -> GQuark;
    #[cfg(windows)]
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
}

#[cfg(windows)]
const G_SPAWN_ERROR_TOO_BIG: gint = 5;
#[cfg(windows)]
const MAX_WINDOWS_COMMANDLINE_UNITS: usize = 32_767;

#[cfg(windows)]
fn utf16_units(ptr: *const gchar) -> usize {
    if ptr.is_null() {
        return 0;
    }

    unsafe {
        std::ffi::CStr::from_ptr(ptr.cast())
            .to_string_lossy()
            .encode_utf16()
            .count()
    }
}

#[cfg(windows)]
fn escaped_arg_units(ptr: *const gchar) -> usize {
    if ptr.is_null() {
        return 0;
    }

    let bytes = unsafe { std::ffi::CStr::from_ptr(ptr.cast()).to_bytes() };
    let mut units = 0usize;
    let mut need_quotes = false;
    let mut trailing_backslashes = 0usize;

    for &byte in bytes {
        if byte == b' ' || byte == b'\t' {
            need_quotes = true;
        }

        if byte == b'"' || byte == b'\\' {
            units += 2;
        } else {
            units += 1;
        }

        if byte == b'\\' {
            trailing_backslashes += 1;
        } else {
            trailing_backslashes = 0;
        }
    }

    if need_quotes {
        units += 2 + trailing_backslashes;
    }

    units
}

#[cfg(windows)]
fn argv_commandline_units(argv: *const *const gchar) -> usize {
    if argv.is_null() {
        return 0;
    }

    let mut total = 0usize;
    let mut index = 0usize;
    loop {
        let arg = unsafe { *argv.add(index) };
        if arg.is_null() {
            break;
        }
        if index > 0 {
            total += 1;
        }
        total += escaped_arg_units(arg);
        index += 1;
    }
    total
}

#[cfg(windows)]
fn set_spawn_too_big(error: *mut *mut GError) {
    unsafe {
        if !error.is_null() && (*error).is_null() {
            *error = g_error_new_literal(
                g_spawn_error_quark(),
                G_SPAWN_ERROR_TOO_BIG,
                c"Command line exceeds the supported Windows CreateProcessW limit".as_ptr(),
            );
        }
    }
}

#[cfg(windows)]
fn check_windows_argv(argv: *const *const gchar, error: *mut *mut GError) -> bool {
    if argv_commandline_units(argv) >= MAX_WINDOWS_COMMANDLINE_UNITS {
        set_spawn_too_big(error);
        return false;
    }
    true
}

#[cfg(windows)]
fn check_windows_command_line(command_line: *const gchar, error: *mut *mut GError) -> bool {
    if utf16_units(command_line) >= MAX_WINDOWS_COMMANDLINE_UNITS {
        set_spawn_too_big(error);
        return false;
    }
    true
}

#[cfg(not(windows))]
fn check_windows_argv(_argv: *const *const gchar, _error: *mut *mut GError) -> bool {
    true
}

#[cfg(not(windows))]
fn check_windows_command_line(_command_line: *const gchar, _error: *mut *mut GError) -> bool {
    true
}

#[unsafe(export_name = "g_spawn_async")]
pub unsafe extern "C" fn spawn_async(
    working_directory: *const gchar,
    argv: *mut *mut gchar,
    envp: *mut *mut gchar,
    flags: GSpawnFlags,
    child_setup: GSpawnChildSetupFunc,
    user_data: gpointer,
    child_pid: *mut GPid,
    error: *mut *mut GError,
) -> gboolean {
    if !check_windows_argv(argv.cast(), error) {
        return 0;
    }
    bridge::spawn_async(working_directory, argv, envp, flags, child_setup, user_data, child_pid, error)
}

#[unsafe(export_name = "g_spawn_async_with_pipes")]
pub unsafe extern "C" fn spawn_async_with_pipes(
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
    if !check_windows_argv(argv.cast(), error) {
        return 0;
    }
    bridge::spawn_async_with_pipes(
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

#[unsafe(export_name = "g_spawn_async_with_pipes_and_fds")]
pub unsafe extern "C" fn spawn_async_with_pipes_and_fds(
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
    if !check_windows_argv(argv, error) {
        return 0;
    }
    bridge::spawn_async_with_pipes_and_fds(
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

#[unsafe(export_name = "g_spawn_async_with_fds")]
pub unsafe extern "C" fn spawn_async_with_fds(
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
    if !check_windows_argv(argv.cast(), error) {
        return 0;
    }
    bridge::spawn_async_with_fds(
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

#[unsafe(export_name = "g_spawn_sync")]
pub unsafe extern "C" fn spawn_sync(
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
    if !check_windows_argv(argv.cast(), error) {
        return 0;
    }
    bridge::spawn_sync(
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

#[unsafe(export_name = "g_spawn_command_line_sync")]
pub unsafe extern "C" fn spawn_command_line_sync(
    command_line: *const gchar,
    standard_output: *mut *mut gchar,
    standard_error: *mut *mut gchar,
    wait_status: *mut gint,
    error: *mut *mut GError,
) -> gboolean {
    if !check_windows_command_line(command_line, error) {
        return 0;
    }
    bridge::spawn_command_line_sync(
        command_line,
        standard_output,
        standard_error,
        wait_status,
        error,
    )
}

#[unsafe(export_name = "g_spawn_command_line_async")]
pub unsafe extern "C" fn spawn_command_line_async(
    command_line: *const gchar,
    error: *mut *mut GError,
) -> gboolean {
    if !check_windows_command_line(command_line, error) {
        return 0;
    }
    bridge::spawn_command_line_async(command_line, error)
}

#[cfg(all(test, windows))]
mod tests {
    use super::{argv_commandline_units, escaped_arg_units, utf16_units, MAX_WINDOWS_COMMANDLINE_UNITS};

    #[test]
    fn quoted_argv_estimator_grows_with_escapes() {
        assert!(escaped_arg_units(c"plain".as_ptr()) < escaped_arg_units(c"needs quotes".as_ptr()));
        assert!(escaped_arg_units(c"backslash\\trail".as_ptr()) >= utf16_units(c"backslash\\trail".as_ptr()));
    }

    #[test]
    fn oversized_command_line_is_detected() {
        let long = "x".repeat(MAX_WINDOWS_COMMANDLINE_UNITS + 8);
        let utf8 = std::ffi::CString::new(long).unwrap();
        assert!(utf16_units(utf8.as_ptr().cast()) > MAX_WINDOWS_COMMANDLINE_UNITS);
    }

    #[test]
    fn argv_estimator_counts_separators() {
        let argv = [c"a".as_ptr(), c"b".as_ptr(), core::ptr::null()];
        assert!(argv_commandline_units(argv.as_ptr()) >= 3);
    }
}
