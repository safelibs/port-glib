extern "C" {
    pub type __spawn_action;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut safe_c2rust_environ: *mut *mut ::core::ffi::c_char;
    fn execve(
        __path: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn execv(
        __path: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn posix_spawn(
        __pid: *mut pid_t,
        __path: *const ::core::ffi::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn posix_spawnp(
        __pid: *mut pid_t,
        __file: *const ::core::ffi::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn posix_spawnattr_init(__attr: *mut posix_spawnattr_t) -> ::core::ffi::c_int;
    fn posix_spawnattr_destroy(__attr: *mut posix_spawnattr_t) -> ::core::ffi::c_int;
    fn posix_spawnattr_setsigdefault(
        __attr: *mut posix_spawnattr_t,
        __sigdefault: *const sigset_t,
    ) -> ::core::ffi::c_int;
    fn posix_spawnattr_setflags(
        _attr: *mut posix_spawnattr_t,
        __flags: ::core::ffi::c_short,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_addclose(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: ::core::ffi::c_int,
        __newfd: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_environ_getenv(envp: *mut *mut gchar, variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_poll(fds: *mut GPollFD, nfds: guint, timeout: gint) -> gint;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_find(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_shell_parse_argv(
        command_line: *const gchar,
        argcp: *mut gint,
        argvp: *mut *mut *mut gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn g_unix_open_pipe(fds: *mut gint, flags: gint, error: *mut *mut GError) -> gboolean;
    fn g_closefrom(lowfd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_fdwalk_set_cloexec(lowfd: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type __pid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type size_t = usize;
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: ::core::ffi::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: ::core::ffi::c_int,
    pub __cgroup: ::core::ffi::c_int,
    pub __pad: [::core::ffi::c_int; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: ::core::ffi::c_int,
    pub __used: ::core::ffi::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [::core::ffi::c_int; 16],
}
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type GPid = ::core::ffi::c_int;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_SPAWN_ERROR_FAILED: C2RustUnnamed = 19;
pub const G_SPAWN_ERROR_LIBBAD: C2RustUnnamed = 18;
pub const G_SPAWN_ERROR_ISDIR: C2RustUnnamed = 17;
pub const G_SPAWN_ERROR_INVAL: C2RustUnnamed = 16;
pub const G_SPAWN_ERROR_MFILE: C2RustUnnamed = 15;
pub const G_SPAWN_ERROR_NFILE: C2RustUnnamed = 14;
pub const G_SPAWN_ERROR_IO: C2RustUnnamed = 13;
pub const G_SPAWN_ERROR_TXTBUSY: C2RustUnnamed = 12;
pub const G_SPAWN_ERROR_LOOP: C2RustUnnamed = 11;
pub const G_SPAWN_ERROR_NOTDIR: C2RustUnnamed = 10;
pub const G_SPAWN_ERROR_NOMEM: C2RustUnnamed = 9;
pub const G_SPAWN_ERROR_NOENT: C2RustUnnamed = 8;
pub const G_SPAWN_ERROR_NAMETOOLONG: C2RustUnnamed = 7;
pub const G_SPAWN_ERROR_NOEXEC: C2RustUnnamed = 6;
pub const G_SPAWN_ERROR_2BIG: C2RustUnnamed = 5;
pub const G_SPAWN_ERROR_TOO_BIG: C2RustUnnamed = 5;
pub const G_SPAWN_ERROR_PERM: C2RustUnnamed = 4;
pub const G_SPAWN_ERROR_ACCES: C2RustUnnamed = 3;
pub const G_SPAWN_ERROR_CHDIR: C2RustUnnamed = 2;
pub const G_SPAWN_ERROR_READ: C2RustUnnamed = 1;
pub const G_SPAWN_ERROR_FORK: C2RustUnnamed = 0;
pub type GSpawnChildSetupFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GSpawnFlags = ::core::ffi::c_uint;
pub const G_SPAWN_STDIN_FROM_DEV_NULL: GSpawnFlags = 2048;
pub const G_SPAWN_CHILD_INHERITS_STDERR: GSpawnFlags = 1024;
pub const G_SPAWN_CHILD_INHERITS_STDOUT: GSpawnFlags = 512;
pub const G_SPAWN_CLOEXEC_PIPES: GSpawnFlags = 256;
pub const G_SPAWN_SEARCH_PATH_FROM_ENVP: GSpawnFlags = 128;
pub const G_SPAWN_FILE_AND_ARGV_ZERO: GSpawnFlags = 64;
pub const G_SPAWN_CHILD_INHERITS_STDIN: GSpawnFlags = 32;
pub const G_SPAWN_STDERR_TO_DEV_NULL: GSpawnFlags = 16;
pub const G_SPAWN_STDOUT_TO_DEV_NULL: GSpawnFlags = 8;
pub const G_SPAWN_SEARCH_PATH: GSpawnFlags = 4;
pub const G_SPAWN_DO_NOT_REAP_CHILD: GSpawnFlags = 2;
pub const G_SPAWN_LEAVE_DESCRIPTORS_OPEN: GSpawnFlags = 1;
pub const G_SPAWN_DEFAULT: GSpawnFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUnixPipe {
    pub fds: [::core::ffi::c_int; 2],
}
pub type GUnixPipeEnd = ::core::ffi::c_uint;
pub const G_UNIX_PIPE_END_WRITE: GUnixPipeEnd = 1;
pub const G_UNIX_PIPE_END_READ: GUnixPipeEnd = 0;
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
pub const CHILD_CLOSE_FAILED: C2RustUnnamed_1 = 5;
pub const CHILD_FORK_FAILED: C2RustUnnamed_1 = 4;
pub const CHILD_DUPFD_FAILED: C2RustUnnamed_1 = 3;
pub const CHILD_OPEN_FAILED: C2RustUnnamed_1 = 2;
pub const CHILD_EXEC_FAILED: C2RustUnnamed_1 = 1;
pub const CHILD_CHDIR_FAILED: C2RustUnnamed_1 = 0;
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub const READ_EOF: ReadResult = 2;
pub const READ_FAILED: ReadResult = 0;
pub type ReadResult = ::core::ffi::c_uint;
pub const READ_OK: ReadResult = 1;
pub type GPollFD = _GPollFD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub const G_IO_ERR: C2RustUnnamed_0 = 8;
pub const G_IO_HUP: C2RustUnnamed_0 = 16;
pub const G_IO_IN: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_IO_NVAL: C2RustUnnamed_0 = 32;
pub const G_IO_PRI: C2RustUnnamed_0 = 2;
pub const G_IO_OUT: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36;
pub const ELOOP: ::core::ffi::c_int = 40;
pub const ELIBBAD: ::core::ffi::c_int = 80;
pub const ETIMEDOUT: ::core::ffi::c_int = 110;
pub const ESTALE: ::core::ffi::c_int = 116;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EPERM: ::core::ffi::c_int = 1;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5;
pub const E2BIG: ::core::ffi::c_int = 7;
pub const ENOEXEC: ::core::ffi::c_int = 8;
pub const ECHILD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12;
pub const EACCES: ::core::ffi::c_int = 13;
pub const EBUSY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ENODEV: ::core::ffi::c_int = 19;
pub const ENOTDIR: ::core::ffi::c_int = 20;
pub const EISDIR: ::core::ffi::c_int = 21;
pub const EINVAL: ::core::ffi::c_int = 22;
pub const ENFILE: ::core::ffi::c_int = 23;
pub const EMFILE: ::core::ffi::c_int = 24;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_DUPFD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_DUPFD_CLOEXEC: ::core::ffi::c_int = 1030 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"fork_exec\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const POSIX_SPAWN_SETSIGDEF: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
#[inline]
unsafe extern "C" fn safe_c2rust__g_spawn_exec_err_to_g_error(mut en: gint) -> gint {
    match en {
        EACCES => return G_SPAWN_ERROR_ACCES as ::core::ffi::c_int as gint,
        EPERM => return G_SPAWN_ERROR_PERM as ::core::ffi::c_int as gint,
        E2BIG => return G_SPAWN_ERROR_TOO_BIG as ::core::ffi::c_int as gint,
        ENOEXEC => return G_SPAWN_ERROR_NOEXEC as ::core::ffi::c_int as gint,
        ENAMETOOLONG => return G_SPAWN_ERROR_NAMETOOLONG as ::core::ffi::c_int as gint,
        ENOENT => return G_SPAWN_ERROR_NOENT as ::core::ffi::c_int as gint,
        ENOMEM => return G_SPAWN_ERROR_NOMEM as ::core::ffi::c_int as gint,
        ENOTDIR => return G_SPAWN_ERROR_NOTDIR as ::core::ffi::c_int as gint,
        ELOOP => return G_SPAWN_ERROR_LOOP as ::core::ffi::c_int as gint,
        EIO => return G_SPAWN_ERROR_IO as ::core::ffi::c_int as gint,
        ENFILE => return G_SPAWN_ERROR_NFILE as ::core::ffi::c_int as gint,
        EMFILE => return G_SPAWN_ERROR_MFILE as ::core::ffi::c_int as gint,
        EINVAL => return G_SPAWN_ERROR_INVAL as ::core::ffi::c_int as gint,
        EISDIR => return G_SPAWN_ERROR_ISDIR as ::core::ffi::c_int as gint,
        ELIBBAD => return G_SPAWN_ERROR_LIBBAD as ::core::ffi::c_int as gint,
        _ => return G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_spawn_invalid_source_fd(
    mut fd: gint,
    mut source_fds: *const gint,
    mut n_fds: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut i: gsize = 0;
    i = 0 as gsize;
    while i < n_fds {
        if fd == *source_fds.offset(i as isize) {
            g_set_error(
                error,
                safe_c2rust_g_spawn_error_quark(),
                G_SPAWN_ERROR_INVAL as ::core::ffi::c_int as gint,
                glib_gettext(b"Invalid source FDs argument\0" as *const u8 as *const gchar),
            );
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_fd(
    mut fd_ptr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = *fd_ptr;
    *fd_ptr = -(1 as ::core::ffi::c_int);
    return fd;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_clear_fd(
    mut fd_ptr: *mut ::core::ffi::c_int,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut fd: ::core::ffi::c_int = *fd_ptr;
    *fd_ptr = -(1 as ::core::ffi::c_int);
    if fd < 0 as ::core::ffi::c_int {
        return TRUE;
    }
    return g_close(fd as gint, error);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_pipe_open(
    mut self_0: *mut GUnixPipe,
    mut flags: ::core::ffi::c_int,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_unix_open_pipe(&raw mut (*self_0).fds as *mut gint, flags as gint, error);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_pipe_get(
    mut self_0: *mut GUnixPipe,
    mut end: GUnixPipeEnd,
) -> ::core::ffi::c_int {
    return (*self_0).fds[end as usize];
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_pipe_steal(
    mut self_0: *mut GUnixPipe,
    mut end: GUnixPipeEnd,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_steal_fd(
        (&raw mut (*self_0).fds as *mut ::core::ffi::c_int).offset(end as isize)
            as *mut ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_pipe_close(
    mut self_0: *mut GUnixPipe,
    mut end: GUnixPipeEnd,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_clear_fd(
        (&raw mut (*self_0).fds as *mut ::core::ffi::c_int).offset(end as isize)
            as *mut ::core::ffi::c_int,
        error,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_unix_pipe_clear(mut self_0: *mut GUnixPipe) {
    let mut errsv: ::core::ffi::c_int = *__errno_location();
    safe_c2rust_g_unix_pipe_close(
        self_0,
        G_UNIX_PIPE_END_READ,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0;
    safe_c2rust_g_unix_pipe_close(
        self_0,
        G_UNIX_PIPE_END_WRITE,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0;
    *__errno_location() = errsv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-exec-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_exit_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-spawn-exit-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_async(
    mut working_directory: *const gchar,
    mut argv: *mut *mut gchar,
    mut envp: *mut *mut gchar,
    mut flags: GSpawnFlags,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
    mut child_pid: *mut GPid,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_spawn_async_with_pipes(
        working_directory,
        argv,
        envp,
        flags,
        child_setup,
        user_data,
        child_pid,
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        error,
    );
}
unsafe extern "C" fn safe_c2rust_read_data(
    mut str: *mut GString,
    mut fd: gint,
    mut error: *mut *mut GError,
) -> ReadResult {
    let mut bytes: gssize = 0;
    let mut buf: [gchar; 4096] = [0; 4096];
    loop {
        bytes = read(
            fd as ::core::ffi::c_int,
            &raw mut buf as *mut gchar as *mut ::core::ffi::c_void,
            4096 as size_t,
        ) as gssize;
        if bytes == 0 as gssize {
            return READ_EOF;
        } else if bytes > 0 as gssize {
            safe_c2rust_g_string_append_len_inline(str, &raw mut buf as *mut gchar, bytes);
            return READ_OK;
        } else {
            if *__errno_location() == EINTR {
                continue;
            }
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            g_set_error(
                error,
                safe_c2rust_g_spawn_error_quark(),
                G_SPAWN_ERROR_READ as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Failed to read data from child process (%s)\0" as *const u8 as *const gchar,
                ),
                g_strerror(errsv as gint),
            );
            return READ_FAILED;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_sync(
    mut working_directory: *const gchar,
    mut argv: *mut *mut gchar,
    mut envp: *mut *mut gchar,
    mut flags: GSpawnFlags,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
    mut standard_output: *mut *mut gchar,
    mut standard_error: *mut *mut gchar,
    mut wait_status: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut outpipe: gint = -(1 as gint);
    let mut errpipe: gint = -(1 as gint);
    let mut pid: GPid = 0;
    let mut ret: gint = 0;
    let mut outstr: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut errstr: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut failed: gboolean = 0;
    let mut status: gint = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !argv.is_null() {
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
            b"argv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null() {
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
            b"argv[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & G_SPAWN_DO_NOT_REAP_CHILD as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
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
            b"!(flags & G_SPAWN_DO_NOT_REAP_CHILD)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if standard_output.is_null()
            || flags as ::core::ffi::c_uint
                & G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
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
            b"standard_output == NULL || !(flags & G_SPAWN_STDOUT_TO_DEV_NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if standard_error.is_null()
            || flags as ::core::ffi::c_uint
                & G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
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
            b"standard_error == NULL || !(flags & G_SPAWN_STDERR_TO_DEV_NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !standard_output.is_null() {
        *standard_output = ::core::ptr::null_mut::<gchar>();
    }
    if !standard_error.is_null() {
        *standard_error = ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_fork_exec(
        FALSE,
        working_directory,
        argv as *const *const gchar,
        envp as *const *const gchar,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_LEAVE_DESCRIPTORS_OPEN as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_SEARCH_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_SEARCH_PATH_FROM_ENVP as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_FILE_AND_ARGV_ZERO as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_CLOEXEC_PIPES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        child_setup,
        user_data,
        &raw mut pid,
        ::core::ptr::null_mut::<gint>(),
        if !standard_output.is_null() {
            &raw mut outpipe
        } else {
            ::core::ptr::null_mut::<gint>()
        },
        if !standard_error.is_null() {
            &raw mut errpipe
        } else {
            ::core::ptr::null_mut::<gint>()
        },
        -(1 as gint),
        -(1 as gint),
        -(1 as gint),
        ::core::ptr::null::<gint>(),
        ::core::ptr::null::<gint>(),
        0 as gsize,
        error,
    ) == 0
    {
        return FALSE;
    }
    failed = FALSE as gboolean;
    if outpipe >= 0 as ::core::ffi::c_int {
        outstr = g_string_new(::core::ptr::null::<gchar>());
    }
    if errpipe >= 0 as ::core::ffi::c_int {
        errstr = g_string_new(::core::ptr::null::<gchar>());
    }
    while failed == 0 && (outpipe >= 0 as ::core::ffi::c_int || errpipe >= 0 as ::core::ffi::c_int)
    {
        let mut fds: [GPollFD; 2] = [
            _GPollFD {
                fd: outpipe,
                events: (G_IO_IN as ::core::ffi::c_int
                    | G_IO_HUP as ::core::ffi::c_int
                    | G_IO_ERR as ::core::ffi::c_int) as gushort,
                revents: 0 as gushort,
            },
            _GPollFD {
                fd: errpipe,
                events: (G_IO_IN as ::core::ffi::c_int
                    | G_IO_HUP as ::core::ffi::c_int
                    | G_IO_ERR as ::core::ffi::c_int) as gushort,
                revents: 0 as gushort,
            },
        ];
        ret = g_poll(
            &raw mut fds as *mut GPollFD,
            (::core::mem::size_of::<[GPollFD; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<GPollFD>() as usize) as guint,
            -(1 as gint),
        );
        if ret < 0 as ::core::ffi::c_int {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            if *__errno_location() == EINTR {
                continue;
            }
            failed = TRUE as gboolean;
            g_set_error(
                error,
                safe_c2rust_g_spawn_error_quark(),
                G_SPAWN_ERROR_READ as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unexpected error in reading data from a child process (%s)\0" as *const u8
                        as *const gchar,
                ),
                g_strerror(errsv as gint),
            );
            break;
        } else {
            if outpipe >= 0 as ::core::ffi::c_int
                && fds[0 as ::core::ffi::c_int as usize].revents as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                match safe_c2rust_read_data(outstr, outpipe, error) as ::core::ffi::c_uint {
                    0 => {
                        failed = TRUE as gboolean;
                    }
                    2 => {
                        safe_c2rust_g_clear_fd(
                            &raw mut outpipe,
                            ::core::ptr::null_mut::<*mut GError>(),
                        );
                    }
                    _ => {}
                }
                if failed != 0 {
                    break;
                }
            }
            if !(errpipe >= 0 as ::core::ffi::c_int
                && fds[1 as ::core::ffi::c_int as usize].revents as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
            {
                continue;
            }
            match safe_c2rust_read_data(errstr, errpipe, error) as ::core::ffi::c_uint {
                0 => {
                    failed = TRUE as gboolean;
                }
                2 => {
                    safe_c2rust_g_clear_fd(
                        &raw mut errpipe,
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                }
                _ => {}
            }
            if failed != 0 {
                break;
            }
        }
    }
    safe_c2rust_g_clear_fd(&raw mut outpipe, ::core::ptr::null_mut::<*mut GError>());
    safe_c2rust_g_clear_fd(&raw mut errpipe, ::core::ptr::null_mut::<*mut GError>());
    loop {
        ret = waitpid(pid as __pid_t, &raw mut status, 0 as ::core::ffi::c_int) as gint;
        if !(ret < 0 as ::core::ffi::c_int) {
            break;
        }
        if *__errno_location() == EINTR {
            continue;
        }
        if *__errno_location() == ECHILD {
            if !wait_status.is_null() {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"In call to g_spawn_sync(), wait status of a child process was requested but ECHILD was received by waitpid(). See the documentation of g_child_watch_source_new() for possible causes.\0"
                        as *const u8 as *const gchar,
                );
            }
        } else if failed == 0 {
            let mut errsv_0: ::core::ffi::c_int = *__errno_location();
            failed = TRUE as gboolean;
            g_set_error(
                error,
                safe_c2rust_g_spawn_error_quark(),
                G_SPAWN_ERROR_READ as ::core::ffi::c_int as gint,
                glib_gettext(b"Unexpected error in waitpid() (%s)\0" as *const u8 as *const gchar),
                g_strerror(errsv_0 as gint),
            );
        }
        break;
    }
    if failed != 0 {
        if !outstr.is_null() {
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(outstr, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                } else {
                    g_string_free_and_steal(outstr);
                };
            } else {
                g_string_free(outstr, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            };
        }
        if !errstr.is_null() {
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(errstr, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                } else {
                    g_string_free_and_steal(errstr);
                };
            } else {
                g_string_free(errstr, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            };
        }
        return FALSE;
    } else {
        if !wait_status.is_null() {
            *wait_status = status;
        }
        if !standard_output.is_null() {
            *standard_output = if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(outstr, 0 as gboolean)
                } else {
                    g_string_free_and_steal(outstr)
                }
            } else {
                g_string_free(outstr, 0 as gboolean)
            };
        }
        if !standard_error.is_null() {
            *standard_error = if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(errstr, 0 as gboolean)
                } else {
                    g_string_free_and_steal(errstr)
                }
            } else {
                g_string_free(errstr, 0 as gboolean)
            };
        }
        return TRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_async_with_pipes(
    mut working_directory: *const gchar,
    mut argv: *mut *mut gchar,
    mut envp: *mut *mut gchar,
    mut flags: GSpawnFlags,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
    mut child_pid: *mut GPid,
    mut standard_input: *mut gint,
    mut standard_output: *mut gint,
    mut standard_error: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_spawn_async_with_pipes_and_fds(
        working_directory,
        argv as *const *const gchar,
        envp as *const *const gchar,
        flags,
        child_setup,
        user_data,
        -(1 as gint),
        -(1 as gint),
        -(1 as gint),
        ::core::ptr::null::<gint>(),
        ::core::ptr::null::<gint>(),
        0 as gsize,
        child_pid,
        standard_input,
        standard_output,
        standard_error,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_async_with_pipes_and_fds(
    mut working_directory: *const gchar,
    mut argv: *const *const gchar,
    mut envp: *const *const gchar,
    mut flags: GSpawnFlags,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut source_fds: *const gint,
    mut target_fds: *const gint,
    mut n_fds: gsize,
    mut child_pid_out: *mut GPid,
    mut stdin_pipe_out: *mut gint,
    mut stdout_pipe_out: *mut gint,
    mut stderr_pipe_out: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !argv.is_null() {
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
            b"argv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null() {
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
            b"argv[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & (G_SPAWN_STDIN_FROM_DEV_NULL as ::core::ffi::c_int
                | G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_SPAWN_STDIN_FROM_DEV_NULL as ::core::ffi::c_int
                | G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
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
            b"(flags & INHERITS_OR_NULL_STDIN) != INHERITS_OR_NULL_STDIN\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & (G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int
                | G_SPAWN_CHILD_INHERITS_STDOUT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int
                | G_SPAWN_CHILD_INHERITS_STDOUT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & INHERITS_OR_NULL_STDOUT) != INHERITS_OR_NULL_STDOUT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & (G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int
                | G_SPAWN_CHILD_INHERITS_STDERR as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != (G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int
                | G_SPAWN_CHILD_INHERITS_STDERR as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & INHERITS_OR_NULL_STDERR) != INHERITS_OR_NULL_STDERR\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if stdin_pipe_out.is_null() || stdin_fd < 0 as ::core::ffi::c_int {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stdin_pipe_out == NULL || stdin_fd < 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if stdout_pipe_out.is_null() || stdout_fd < 0 as ::core::ffi::c_int {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stdout_pipe_out == NULL || stdout_fd < 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if stderr_pipe_out.is_null() || stderr_fd < 0 as ::core::ffi::c_int {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stderr_pipe_out == NULL || stderr_fd < 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if flags as ::core::ffi::c_uint
        & (G_SPAWN_STDIN_FROM_DEV_NULL as ::core::ffi::c_int
            | G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        stdin_pipe_out = ::core::ptr::null_mut::<gint>();
    }
    if flags as ::core::ffi::c_uint
        & (G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int
            | G_SPAWN_CHILD_INHERITS_STDOUT as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        stdout_pipe_out = ::core::ptr::null_mut::<gint>();
    }
    if flags as ::core::ffi::c_uint
        & (G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int
            | G_SPAWN_CHILD_INHERITS_STDERR as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        stderr_pipe_out = ::core::ptr::null_mut::<gint>();
    }
    return safe_c2rust_fork_exec(
        (flags as ::core::ffi::c_uint
            & G_SPAWN_DO_NOT_REAP_CHILD as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0) as ::core::ffi::c_int,
        working_directory,
        argv,
        envp,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_LEAVE_DESCRIPTORS_OPEN as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_SEARCH_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_SEARCH_PATH_FROM_ENVP as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_FILE_AND_ARGV_ZERO as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_CLOEXEC_PIPES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        child_setup,
        user_data,
        child_pid_out,
        stdin_pipe_out,
        stdout_pipe_out,
        stderr_pipe_out,
        stdin_fd,
        stdout_fd,
        stderr_fd,
        source_fds,
        target_fds,
        n_fds,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_async_with_fds(
    mut working_directory: *const gchar,
    mut argv: *mut *mut gchar,
    mut envp: *mut *mut gchar,
    mut flags: GSpawnFlags,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
    mut child_pid: *mut GPid,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !argv.is_null() {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"argv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"argv[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if stdout_fd < 0 as ::core::ffi::c_int
            || flags as ::core::ffi::c_uint
                & G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stdout_fd < 0 || !(flags & G_SPAWN_STDOUT_TO_DEV_NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if stderr_fd < 0 as ::core::ffi::c_int
            || flags as ::core::ffi::c_uint
                & G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stderr_fd < 0 || !(flags & G_SPAWN_STDERR_TO_DEV_NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if stdin_fd < 0 as ::core::ffi::c_int
            || flags as ::core::ffi::c_uint
                & G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"stdin_fd < 0 || !(flags & G_SPAWN_CHILD_INHERITS_STDIN)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_fork_exec(
        (flags as ::core::ffi::c_uint
            & G_SPAWN_DO_NOT_REAP_CHILD as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0) as ::core::ffi::c_int,
        working_directory,
        argv as *const *const gchar,
        envp as *const *const gchar,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_LEAVE_DESCRIPTORS_OPEN as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_SEARCH_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_SEARCH_PATH_FROM_ENVP as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_STDOUT_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_STDERR_TO_DEV_NULL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_CHILD_INHERITS_STDIN as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_FILE_AND_ARGV_ZERO as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (flags as ::core::ffi::c_uint
            & G_SPAWN_CLOEXEC_PIPES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        child_setup,
        user_data,
        child_pid,
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<gint>(),
        stdin_fd,
        stdout_fd,
        stderr_fd,
        ::core::ptr::null::<gint>(),
        ::core::ptr::null::<gint>(),
        0 as gsize,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_command_line_sync(
    mut command_line: *const gchar,
    mut standard_output: *mut *mut gchar,
    mut standard_error: *mut *mut gchar,
    mut wait_status: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut retval: gboolean = 0;
    let mut argv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !command_line.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"command_line != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_shell_parse_argv(
        command_line,
        ::core::ptr::null_mut::<gint>(),
        &raw mut argv,
        error,
    ) == 0
    {
        return FALSE;
    }
    retval = safe_c2rust_g_spawn_sync(
        ::core::ptr::null::<gchar>(),
        argv,
        ::core::ptr::null_mut::<*mut gchar>(),
        G_SPAWN_SEARCH_PATH,
        None,
        NULL,
        standard_output,
        standard_error,
        wait_status,
        error,
    );
    g_strfreev(argv);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_command_line_async(
    mut command_line: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut retval: gboolean = 0;
    let mut argv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !command_line.is_null() {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"command_line != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_shell_parse_argv(
        command_line,
        ::core::ptr::null_mut::<gint>(),
        &raw mut argv,
        error,
    ) == 0
    {
        return FALSE;
    }
    retval = safe_c2rust_g_spawn_async(
        ::core::ptr::null::<gchar>(),
        argv,
        ::core::ptr::null_mut::<*mut gchar>(),
        G_SPAWN_SEARCH_PATH,
        None,
        NULL,
        ::core::ptr::null_mut::<GPid>(),
        error,
    );
    g_strfreev(argv);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_check_wait_status(
    mut wait_status: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    if wait_status as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (wait_status as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int)
            >> 8 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            g_set_error(
                error,
                safe_c2rust_g_spawn_exit_error_quark(),
                (wait_status & 0xff00 as gint) >> 8 as ::core::ffi::c_int,
                glib_gettext(b"Child process exited with code %ld\0" as *const u8 as *const gchar),
                ((wait_status as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        } else {
            ret = TRUE as gboolean;
        }
    } else if ((wait_status as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as ::core::ffi::c_schar as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        g_set_error(
            error,
            safe_c2rust_g_spawn_error_quark(),
            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Child process killed by signal %ld\0" as *const u8 as *const gchar),
            (wait_status as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_long,
        );
    } else if wait_status as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        == 0x7f as ::core::ffi::c_int
    {
        g_set_error(
            error,
            safe_c2rust_g_spawn_error_quark(),
            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Child process stopped by signal %ld\0" as *const u8 as *const gchar),
            ((wait_status as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int)
                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_long,
        );
    } else {
        g_set_error(
            error,
            safe_c2rust_g_spawn_error_quark(),
            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Child process exited abnormally\0" as *const u8 as *const gchar),
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_check_exit_status(
    mut wait_status: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_spawn_check_wait_status(wait_status, error);
}
unsafe extern "C" fn safe_c2rust_write_all(
    mut fd: gint,
    mut vbuf: gconstpointer,
    mut to_write: gsize,
) -> gssize {
    let mut buf: *mut gchar = vbuf as *mut gchar;
    while to_write > 0 as gsize {
        let mut count: gssize = write(
            fd as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_void,
            to_write as size_t,
        ) as gssize;
        if count < 0 as gssize {
            if *__errno_location() != EINTR {
                return FALSE as gssize;
            }
        } else {
            to_write = to_write.wrapping_sub(count as gsize);
            buf = buf.offset(count as isize);
        }
    }
    return TRUE as gssize;
}
unsafe extern "C" fn safe_c2rust_write_err_and_exit(mut fd: gint, mut msg: gint) -> ! {
    let mut en: gint = *__errno_location();
    safe_c2rust_write_all(
        fd,
        &raw mut msg as gconstpointer,
        ::core::mem::size_of::<gint>() as gsize,
    );
    safe_c2rust_write_all(
        fd,
        &raw mut en as gconstpointer,
        ::core::mem::size_of::<gint>() as gsize,
    );
    _exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn safe_c2rust_set_cloexec(mut fd: ::core::ffi::c_int) {
    fcntl(fd, F_SETFD, FD_CLOEXEC);
}
unsafe extern "C" fn safe_c2rust_unset_cloexec(mut fd: ::core::ffi::c_int) {
    let mut flags: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = 0;
    flags = fcntl(fd, F_GETFD, 0 as ::core::ffi::c_int);
    if flags != -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = 0;
        flags &= !FD_CLOEXEC;
        loop {
            result = fcntl(fd, F_SETFD, flags);
            errsv = *__errno_location();
            if !(result == -(1 as ::core::ffi::c_int) && errsv == EINTR) {
                break;
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_dupfd_cloexec(
    mut old_fd: ::core::ffi::c_int,
    mut new_fd_min: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    loop {
        fd = fcntl(old_fd, F_DUPFD_CLOEXEC, new_fd_min);
        errsv = *__errno_location();
        if !(fd == -(1 as ::core::ffi::c_int) && errsv == EINTR) {
            break;
        }
    }
    return fd;
}
unsafe extern "C" fn safe_c2rust_safe_dup2(mut fd1: gint, mut fd2: gint) -> gint {
    let mut ret: gint = 0;
    loop {
        ret = dup2(fd1 as ::core::ffi::c_int, fd2 as ::core::ffi::c_int) as gint;
        if !(ret < 0 as ::core::ffi::c_int
            && (*__errno_location() == EINTR || *__errno_location() == EBUSY))
        {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_relocate_fd_out_of_standard_range(mut fd: *mut gint) -> gboolean {
    let mut ret: gint = -(1 as gint);
    let min_fileno: ::core::ffi::c_int = STDERR_FILENO + 1 as ::core::ffi::c_int;
    loop {
        ret = fcntl(*fd, F_DUPFD, min_fileno) as gint;
        if !(ret < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
            break;
        }
    }
    if ret >= min_fileno {
        *fd = ret;
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_safe_open(
    mut path: *const ::core::ffi::c_char,
    mut mode: gint,
) -> gint {
    let mut ret: gint = 0;
    loop {
        ret = open(path, mode as ::core::ffi::c_int) as gint;
        if !(ret < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_do_exec(
    mut child_err_report_fd: gint,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut source_fds: *mut gint,
    mut target_fds: *const gint,
    mut n_fds: gsize,
    mut working_directory: *const gchar,
    mut argv: *const *const gchar,
    mut argv_buffer: *mut *mut gchar,
    mut argv_buffer_len: gsize,
    mut envp: *const *const gchar,
    mut close_descriptors: gboolean,
    mut search_path: *const gchar,
    mut search_path_buffer: *mut gchar,
    mut search_path_buffer_len: gsize,
    mut stdout_to_null: gboolean,
    mut stderr_to_null: gboolean,
    mut child_inherits_stdin: gboolean,
    mut file_and_argv_zero: gboolean,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
) {
    let mut i: gsize = 0;
    let mut max_target_fd: gint = 0 as gint;
    if !working_directory.is_null()
        && chdir(working_directory as *const ::core::ffi::c_char) < 0 as ::core::ffi::c_int
    {
        safe_c2rust_write_err_and_exit(
            child_err_report_fd,
            CHILD_CHDIR_FAILED as ::core::ffi::c_int as gint,
        );
    }
    if stdin_fd >= STDIN_FILENO && stdin_fd <= STDERR_FILENO && stdin_fd != STDIN_FILENO {
        let mut old_fd: ::core::ffi::c_int = stdin_fd as ::core::ffi::c_int;
        if safe_c2rust_relocate_fd_out_of_standard_range(&raw mut stdin_fd) == 0 {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        if stdout_fd == old_fd {
            stdout_fd = stdin_fd;
        }
        if stderr_fd == old_fd {
            stderr_fd = stdin_fd;
        }
    }
    if stdin_fd >= 0 as ::core::ffi::c_int && stdin_fd != STDIN_FILENO {
        if safe_c2rust_safe_dup2(stdin_fd, 0 as gint) < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        safe_c2rust_set_cloexec(stdin_fd as ::core::ffi::c_int);
    } else if child_inherits_stdin == 0 {
        let mut read_null: gint = safe_c2rust_safe_open(
            b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
            O_RDONLY,
        );
        if read_null < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_OPEN_FAILED as ::core::ffi::c_int as gint,
            );
        }
        if safe_c2rust_safe_dup2(read_null, 0 as gint) < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        safe_c2rust_g_clear_fd(&raw mut read_null, ::core::ptr::null_mut::<*mut GError>());
    }
    if stdout_fd >= STDIN_FILENO && stdout_fd <= STDERR_FILENO && stdout_fd != STDOUT_FILENO {
        let mut old_fd_0: ::core::ffi::c_int = stdout_fd as ::core::ffi::c_int;
        if safe_c2rust_relocate_fd_out_of_standard_range(&raw mut stdout_fd) == 0 {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        if stderr_fd == old_fd_0 {
            stderr_fd = stdout_fd;
        }
    }
    if stdout_fd >= 0 as ::core::ffi::c_int && stdout_fd != STDOUT_FILENO {
        if safe_c2rust_safe_dup2(stdout_fd, 1 as gint) < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        safe_c2rust_set_cloexec(stdout_fd as ::core::ffi::c_int);
    } else if stdout_to_null != 0 {
        let mut write_null: gint = safe_c2rust_safe_open(
            b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
            O_WRONLY,
        );
        if write_null < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_OPEN_FAILED as ::core::ffi::c_int as gint,
            );
        }
        if safe_c2rust_safe_dup2(write_null, 1 as gint) < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        safe_c2rust_g_clear_fd(&raw mut write_null, ::core::ptr::null_mut::<*mut GError>());
    }
    if stderr_fd >= STDIN_FILENO && stderr_fd <= STDERR_FILENO && stderr_fd != STDERR_FILENO {
        if safe_c2rust_relocate_fd_out_of_standard_range(&raw mut stderr_fd) == 0 {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
    }
    if stderr_fd >= 0 as ::core::ffi::c_int && stderr_fd != STDERR_FILENO {
        if safe_c2rust_safe_dup2(stderr_fd, 2 as gint) < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        safe_c2rust_set_cloexec(stderr_fd as ::core::ffi::c_int);
    } else if stderr_to_null != 0 {
        let mut write_null_0: gint = safe_c2rust_safe_open(
            b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
            O_WRONLY,
        );
        if write_null_0 < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_OPEN_FAILED as ::core::ffi::c_int as gint,
            );
        }
        if safe_c2rust_safe_dup2(write_null_0, 2 as gint) < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        safe_c2rust_g_clear_fd(
            &raw mut write_null_0,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if close_descriptors != 0 {
        if child_setup.is_none() && n_fds == 0 as gsize {
            if safe_c2rust_safe_dup2(child_err_report_fd, 3 as gint) < 0 as ::core::ffi::c_int {
                safe_c2rust_write_err_and_exit(
                    child_err_report_fd,
                    CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
                );
            }
            safe_c2rust_set_cloexec(3 as ::core::ffi::c_int);
            if g_closefrom(4 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                safe_c2rust_write_err_and_exit(
                    child_err_report_fd,
                    CHILD_CLOSE_FAILED as ::core::ffi::c_int as gint,
                );
            }
            child_err_report_fd = 3 as ::core::ffi::c_int as gint;
        } else if g_fdwalk_set_cloexec(3 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_CLOSE_FAILED as ::core::ffi::c_int as gint,
            );
        }
    } else {
        safe_c2rust_set_cloexec(child_err_report_fd as ::core::ffi::c_int);
    }
    if n_fds > 0 as gsize {
        i = 0 as gsize;
        while i < n_fds {
            max_target_fd = if max_target_fd > *target_fds.offset(i as isize) {
                max_target_fd
            } else {
                *target_fds.offset(i as isize)
            };
            i = i.wrapping_add(1);
        }
        if max_target_fd == G_MAXINT {
            *__errno_location() = EINVAL;
            safe_c2rust_write_err_and_exit(
                child_err_report_fd,
                CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
            );
        }
        i = 0 as gsize;
        while i < n_fds {
            if *source_fds.offset(i as isize) != *target_fds.offset(i as isize) {
                *source_fds.offset(i as isize) = safe_c2rust_dupfd_cloexec(
                    *source_fds.offset(i as isize) as ::core::ffi::c_int,
                    max_target_fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                ) as gint;
                if *source_fds.offset(i as isize) < 0 as ::core::ffi::c_int {
                    safe_c2rust_write_err_and_exit(
                        child_err_report_fd,
                        CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
        i = 0 as gsize;
        while i < n_fds {
            if *source_fds.offset(i as isize) == *target_fds.offset(i as isize) {
                safe_c2rust_unset_cloexec(*source_fds.offset(i as isize) as ::core::ffi::c_int);
            } else {
                if *target_fds.offset(i as isize) == child_err_report_fd {
                    child_err_report_fd = safe_c2rust_dupfd_cloexec(
                        child_err_report_fd as ::core::ffi::c_int,
                        max_target_fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                    ) as gint;
                    if child_err_report_fd < 0 as ::core::ffi::c_int {
                        safe_c2rust_write_err_and_exit(
                            child_err_report_fd,
                            CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
                        );
                    }
                }
                if safe_c2rust_safe_dup2(
                    *source_fds.offset(i as isize),
                    *target_fds.offset(i as isize),
                ) < 0 as ::core::ffi::c_int
                {
                    safe_c2rust_write_err_and_exit(
                        child_err_report_fd,
                        CHILD_DUPFD_FAILED as ::core::ffi::c_int as gint,
                    );
                }
                safe_c2rust_g_clear_fd(
                    source_fds.offset(i as isize) as *mut ::core::ffi::c_int,
                    ::core::ptr::null_mut::<*mut GError>(),
                );
            }
            i = i.wrapping_add(1);
        }
    }
    if child_setup.is_some() {
        Some(child_setup.expect("non-null function pointer")).expect("non-null function pointer")(
            user_data,
        );
    }
    safe_c2rust_g_execute(
        *argv.offset(0 as ::core::ffi::c_int as isize),
        (if file_and_argv_zero != 0 {
            argv.offset(1 as ::core::ffi::c_int as isize)
        } else {
            argv
        }) as *mut *mut gchar,
        argv_buffer,
        argv_buffer_len,
        envp as *mut *mut gchar,
        search_path,
        search_path_buffer,
        search_path_buffer_len,
    );
    safe_c2rust_write_err_and_exit(
        child_err_report_fd,
        CHILD_EXEC_FAILED as ::core::ffi::c_int as gint,
    );
}
unsafe extern "C" fn safe_c2rust_read_ints(
    mut fd: ::core::ffi::c_int,
    mut buf: *mut gint,
    mut n_ints_in_buf: gint,
    mut n_ints_read: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut bytes: gsize = 0 as gsize;
    while FALSE == 0 {
        let mut chunk: gssize = 0;
        if bytes as usize >= (::core::mem::size_of::<gint>() as usize).wrapping_mul(2 as usize) {
            break;
        }
        loop {
            chunk = read(
                fd,
                (buf as *mut gchar).offset(bytes as isize) as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<gint>() as size_t)
                    .wrapping_mul(n_ints_in_buf as size_t)
                    .wrapping_sub(bytes as size_t),
            ) as gssize;
            if !(chunk < 0 as gssize && *__errno_location() == EINTR) {
                break;
            }
        }
        if chunk < 0 as gssize {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            g_set_error(
                error,
                safe_c2rust_g_spawn_error_quark(),
                G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(b"Failed to read from child pipe (%s)\0" as *const u8 as *const gchar),
                g_strerror(errsv as gint),
            );
            return FALSE;
        } else {
            if chunk == 0 as gssize {
                break;
            }
            bytes = bytes.wrapping_add(chunk as gsize);
        }
    }
    *n_ints_read = (bytes as usize).wrapping_div(::core::mem::size_of::<gint>() as usize) as gint;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_do_posix_spawn(
    mut argv: *const *const gchar,
    mut envp: *const *const gchar,
    mut search_path: gboolean,
    mut stdout_to_null: gboolean,
    mut stderr_to_null: gboolean,
    mut child_inherits_stdin: gboolean,
    mut file_and_argv_zero: gboolean,
    mut child_pid: *mut GPid,
    mut child_close_fds: *mut gint,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut source_fds: *const gint,
    mut target_fds: *const gint,
    mut n_fds: gsize,
) -> gboolean {
    let mut current_block: u64;
    let mut pid: pid_t = 0;
    let mut duped_source_fds: *mut gint = ::core::ptr::null_mut::<gint>();
    let mut max_target_fd: gint = 0 as gint;
    let mut argv_pass: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut attr: posix_spawnattr_t = posix_spawnattr_t {
        __flags: 0,
        __pgrp: 0,
        __sd: __sigset_t { __val: [0; 16] },
        __ss: __sigset_t { __val: [0; 16] },
        __sp: sched_param { sched_priority: 0 },
        __policy: 0,
        __cgroup: 0,
        __pad: [0; 15],
    };
    let mut file_actions: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: ::core::ptr::null_mut::<__spawn_action>(),
        __pad: [0; 16],
    };
    let mut parent_close_fds: [gint; 3] = [0; 3];
    let mut num_parent_close_fds: gsize = 0 as gsize;
    let mut child_close: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut elem: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut mask: sigset_t = __sigset_t { __val: [0; 16] };
    let mut i: gsize = 0;
    let mut r: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !argv.is_null() && !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gspawn.c\0" as *const u8 as *const ::core::ffi::c_char,
            1670 as ::core::ffi::c_int,
            G_STRFUNC,
            b"argv != NULL && argv[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if **argv.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        return ENOENT;
    }
    r = posix_spawnattr_init(&raw mut attr);
    if r != 0 as ::core::ffi::c_int {
        return r as gboolean;
    }
    if !child_close_fds.is_null() {
        let mut i_0: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        loop {
            i_0 += 1;
            if !(*child_close_fds.offset(i_0 as isize) != -(1 as ::core::ffi::c_int)) {
                break;
            }
            child_close = g_slist_prepend(
                child_close,
                *child_close_fds.offset(i_0 as isize) as glong as gpointer,
            );
        }
    }
    r = posix_spawnattr_setflags(&raw mut attr, POSIX_SPAWN_SETSIGDEF as ::core::ffi::c_short);
    if !(r != 0 as ::core::ffi::c_int) {
        sigemptyset(&raw mut mask);
        sigaddset(&raw mut mask, SIGCHLD);
        sigaddset(&raw mut mask, SIGINT);
        sigaddset(&raw mut mask, SIGTERM);
        sigaddset(&raw mut mask, SIGHUP);
        r = posix_spawnattr_setsigdefault(&raw mut attr, &raw mut mask);
        if !(r != 0 as ::core::ffi::c_int) {
            r = posix_spawn_file_actions_init(&raw mut file_actions);
            if !(r != 0 as ::core::ffi::c_int) {
                if stdin_fd >= 0 as ::core::ffi::c_int {
                    r = posix_spawn_file_actions_adddup2(
                        &raw mut file_actions,
                        stdin_fd as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    if r != 0 as ::core::ffi::c_int {
                        current_block = 13348308169390903390;
                    } else {
                        if g_slist_find(child_close, stdin_fd as glong as gpointer as gconstpointer)
                            .is_null()
                        {
                            child_close =
                                g_slist_prepend(child_close, stdin_fd as glong as gpointer);
                        }
                        current_block = 4090602189656566074;
                    }
                } else if child_inherits_stdin == 0 {
                    let mut read_null: gint = safe_c2rust_safe_open(
                        b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
                        O_RDONLY | O_CLOEXEC,
                    );
                    if ({
                        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
                        if read_null != -(1 as ::core::ffi::c_int) {
                            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_31
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gspawn.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1724 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"read_null != -1\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    let fresh8 = num_parent_close_fds;
                    num_parent_close_fds = num_parent_close_fds.wrapping_add(1);
                    parent_close_fds[fresh8 as usize] = read_null;
                    r = posix_spawn_file_actions_adddup2(
                        &raw mut file_actions,
                        read_null as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    if r != 0 as ::core::ffi::c_int {
                        current_block = 13348308169390903390;
                    } else {
                        current_block = 4090602189656566074;
                    }
                } else {
                    current_block = 4090602189656566074;
                }
                match current_block {
                    4090602189656566074 => {
                        if stdout_fd >= 0 as ::core::ffi::c_int {
                            r = posix_spawn_file_actions_adddup2(
                                &raw mut file_actions,
                                stdout_fd as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                            if r != 0 as ::core::ffi::c_int {
                                current_block = 13348308169390903390;
                            } else {
                                if g_slist_find(
                                    child_close,
                                    stdout_fd as glong as gpointer as gconstpointer,
                                )
                                .is_null()
                                {
                                    child_close = g_slist_prepend(
                                        child_close,
                                        stdout_fd as glong as gpointer,
                                    );
                                }
                                current_block = 5141539773904409130;
                            }
                        } else if stdout_to_null != 0 {
                            let mut write_null: gint = safe_c2rust_safe_open(
                                b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
                                O_WRONLY | O_CLOEXEC,
                            );
                            if ({
                                let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
                                if write_null != -(1 as ::core::ffi::c_int) {
                                    _g_boolean_var_32 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_32 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_32
                            }) as ::core::ffi::c_long
                                != 0
                            {
                            } else {
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"../original/glib/gspawn.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    1748 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"write_null != -1\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            let fresh9 = num_parent_close_fds;
                            num_parent_close_fds = num_parent_close_fds.wrapping_add(1);
                            parent_close_fds[fresh9 as usize] = write_null;
                            r = posix_spawn_file_actions_adddup2(
                                &raw mut file_actions,
                                write_null as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                            if r != 0 as ::core::ffi::c_int {
                                current_block = 13348308169390903390;
                            } else {
                                current_block = 5141539773904409130;
                            }
                        } else {
                            current_block = 5141539773904409130;
                        }
                        match current_block {
                            13348308169390903390 => {}
                            _ => {
                                if stderr_fd >= 0 as ::core::ffi::c_int {
                                    r = posix_spawn_file_actions_adddup2(
                                        &raw mut file_actions,
                                        stderr_fd as ::core::ffi::c_int,
                                        2 as ::core::ffi::c_int,
                                    );
                                    if r != 0 as ::core::ffi::c_int {
                                        current_block = 13348308169390903390;
                                    } else {
                                        if g_slist_find(
                                            child_close,
                                            stderr_fd as glong as gpointer as gconstpointer,
                                        )
                                        .is_null()
                                        {
                                            child_close = g_slist_prepend(
                                                child_close,
                                                stderr_fd as glong as gpointer,
                                            );
                                        }
                                        current_block = 3580086814630675314;
                                    }
                                } else if stderr_to_null != 0 {
                                    let mut write_null_0: gint = safe_c2rust_safe_open(
                                        b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
                                        O_WRONLY | O_CLOEXEC,
                                    );
                                    if ({
                                        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
                                        if write_null_0 != -(1 as ::core::ffi::c_int) {
                                            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_33
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                    } else {
                                        g_assertion_message_expr(
                                            G_LOG_DOMAIN.as_ptr(),
                                            b"../original/glib/gspawn.c\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            1772 as ::core::ffi::c_int,
                                            G_STRFUNC,
                                            b"write_null != -1\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    let fresh10 = num_parent_close_fds;
                                    num_parent_close_fds = num_parent_close_fds.wrapping_add(1);
                                    parent_close_fds[fresh10 as usize] = write_null_0;
                                    r = posix_spawn_file_actions_adddup2(
                                        &raw mut file_actions,
                                        write_null_0 as ::core::ffi::c_int,
                                        2 as ::core::ffi::c_int,
                                    );
                                    if r != 0 as ::core::ffi::c_int {
                                        current_block = 13348308169390903390;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                } else {
                                    current_block = 3580086814630675314;
                                }
                                match current_block {
                                    13348308169390903390 => {}
                                    _ => {
                                        i = 0 as gsize;
                                        while i < n_fds {
                                            max_target_fd =
                                                if max_target_fd > *target_fds.offset(i as isize) {
                                                    max_target_fd
                                                } else {
                                                    *target_fds.offset(i as isize)
                                                };
                                            i = i.wrapping_add(1);
                                        }
                                        if !(max_target_fd == G_MAXINT) {
                                            duped_source_fds = ({
                                                let mut __n: gsize = n_fds;
                                                let mut __s: gsize =
                                                    ::core::mem::size_of::<gint>() as gsize;
                                                let mut __p: gpointer =
                                                    ::core::ptr::null_mut::<::core::ffi::c_void>();
                                                if __s == 1 as gsize {
                                                    __p = g_malloc(__n);
                                                } else if 0 != 0
                                                    && (__s == 0 as gsize
                                                        || __n <= G_MAXSIZE.wrapping_div(__s))
                                                {
                                                    __p = g_malloc(__n.wrapping_mul(__s));
                                                } else {
                                                    __p = g_malloc_n(__n, __s);
                                                }
                                                __p
                                            })
                                                as *mut gint;
                                            i = 0 as gsize;
                                            loop {
                                                if !(i < n_fds) {
                                                    current_block = 16029476503615101993;
                                                    break;
                                                }
                                                *duped_source_fds.offset(i as isize) =
                                                    safe_c2rust_dupfd_cloexec(
                                                        *source_fds.offset(i as isize)
                                                            as ::core::ffi::c_int,
                                                        max_target_fd as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int,
                                                    )
                                                        as gint;
                                                if *duped_source_fds.offset(i as isize)
                                                    < 0 as ::core::ffi::c_int
                                                {
                                                    current_block = 13348308169390903390;
                                                    break;
                                                }
                                                i = i.wrapping_add(1);
                                            }
                                            match current_block {
                                                13348308169390903390 => {}
                                                _ => {
                                                    i = 0 as gsize;
                                                    loop {
                                                        if !(i < n_fds) {
                                                            current_block = 11052029508375673978;
                                                            break;
                                                        }
                                                        r = posix_spawn_file_actions_adddup2(
                                                            &raw mut file_actions,
                                                            *duped_source_fds.offset(i as isize)
                                                                as ::core::ffi::c_int,
                                                            *target_fds.offset(i as isize)
                                                                as ::core::ffi::c_int,
                                                        );
                                                        if r != 0 as ::core::ffi::c_int {
                                                            current_block = 13348308169390903390;
                                                            break;
                                                        }
                                                        i = i.wrapping_add(1);
                                                    }
                                                    match current_block {
                                                        13348308169390903390 => {}
                                                        _ => {
                                                            elem = child_close;
                                                            loop {
                                                                if elem.is_null() {
                                                                    current_block =
                                                                        8464383504555462953;
                                                                    break;
                                                                }
                                                                r = posix_spawn_file_actions_addclose(
                                                                    &raw mut file_actions,
                                                                    (*elem).data as glong as ::core::ffi::c_int,
                                                                );
                                                                if r != 0 as ::core::ffi::c_int {
                                                                    current_block =
                                                                        13348308169390903390;
                                                                    break;
                                                                }
                                                                elem = (*elem).next;
                                                            }
                                                            match current_block {
                                                                13348308169390903390 => {}
                                                                _ => {
                                                                    argv_pass =
                                                                        if file_and_argv_zero != 0 {
                                                                            argv.offset(1 as ::core::ffi::c_int as isize)
                                                                        } else {
                                                                            argv
                                                                        };
                                                                    if envp.is_null() {
                                                                        envp = safe_c2rust_environ
                                                                            as *const *const gchar;
                                                                    }
                                                                    if search_path == 0
                                                                        || !strchr(
                                                                                *argv.offset(0 as ::core::ffi::c_int as isize)
                                                                                    as *const ::core::ffi::c_char,
                                                                                '/' as i32,
                                                                            )
                                                                            .is_null()
                                                                    {
                                                                        r = posix_spawn(
                                                                            &raw mut pid,
                                                                            *argv.offset(0 as ::core::ffi::c_int as isize)
                                                                                as *const ::core::ffi::c_char,
                                                                            &raw mut file_actions,
                                                                            &raw mut attr,
                                                                            argv_pass as *const *mut ::core::ffi::c_char,
                                                                            envp as *const *mut ::core::ffi::c_char,
                                                                        );
                                                                    } else {
                                                                        r = posix_spawnp(
                                                                            &raw mut pid,
                                                                            *argv.offset(0 as ::core::ffi::c_int as isize)
                                                                                as *const ::core::ffi::c_char,
                                                                            &raw mut file_actions,
                                                                            &raw mut attr,
                                                                            argv_pass as *const *mut ::core::ffi::c_char,
                                                                            envp as *const *mut ::core::ffi::c_char,
                                                                        );
                                                                    }
                                                                    if r == 0 as ::core::ffi::c_int
                                                                        && !child_pid.is_null()
                                                                    {
                                                                        *child_pid = pid as GPid;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                i = 0 as gsize;
                while i < num_parent_close_fds {
                    safe_c2rust_g_clear_fd(
                        (&raw mut parent_close_fds as *mut gint).offset(i as isize)
                            as *mut ::core::ffi::c_int,
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                    i = i.wrapping_add(1);
                }
                if !duped_source_fds.is_null() {
                    i = 0 as gsize;
                    while i < n_fds {
                        safe_c2rust_g_clear_fd(
                            duped_source_fds.offset(i as isize) as *mut ::core::ffi::c_int,
                            ::core::ptr::null_mut::<*mut GError>(),
                        );
                        i = i.wrapping_add(1);
                    }
                    g_free(duped_source_fds as gpointer);
                }
                posix_spawn_file_actions_destroy(&raw mut file_actions);
            }
        }
    }
    posix_spawnattr_destroy(&raw mut attr);
    g_slist_free(child_close);
    return r as gboolean;
}
unsafe extern "C" fn safe_c2rust_source_fds_collide_with_pipe(
    mut pipefd: *const GUnixPipe,
    mut source_fds: *const ::core::ffi::c_int,
    mut n_fds: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    return (safe_c2rust__g_spawn_invalid_source_fd(
        (*pipefd).fds[G_UNIX_PIPE_END_READ as ::core::ffi::c_int as usize],
        source_fds as *const gint,
        n_fds,
        error,
    ) != 0
        || safe_c2rust__g_spawn_invalid_source_fd(
            (*pipefd).fds[G_UNIX_PIPE_END_WRITE as ::core::ffi::c_int as usize],
            source_fds as *const gint,
            n_fds,
            error,
        ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_fork_exec(
    mut intermediate_child: gboolean,
    mut working_directory: *const gchar,
    mut argv: *const *const gchar,
    mut envp: *const *const gchar,
    mut close_descriptors: gboolean,
    mut search_path: gboolean,
    mut search_path_from_envp: gboolean,
    mut stdout_to_null: gboolean,
    mut stderr_to_null: gboolean,
    mut child_inherits_stdin: gboolean,
    mut file_and_argv_zero: gboolean,
    mut cloexec_pipes: gboolean,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
    mut child_pid: *mut GPid,
    mut stdin_pipe_out: *mut gint,
    mut stdout_pipe_out: *mut gint,
    mut stderr_pipe_out: *mut gint,
    mut stdin_fd: gint,
    mut stdout_fd: gint,
    mut stderr_fd: gint,
    mut source_fds: *const gint,
    mut target_fds: *const gint,
    mut n_fds: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut current_block: u64;
    let mut pid: GPid = -(1 as GPid);
    let mut child_err_report_pipe: GUnixPipe = GUnixPipe {
        fds: [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)],
    };
    let mut child_pid_report_pipe: GUnixPipe = GUnixPipe {
        fds: [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)],
    };
    let mut pipe_flags: guint = (if cloexec_pipes != 0 {
        O_CLOEXEC
    } else {
        0 as ::core::ffi::c_int
    }) as guint;
    let mut status: gint = 0;
    let mut chosen_search_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut search_path_buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut search_path_buffer_heap: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut search_path_buffer_len: gsize = 0 as gsize;
    let mut argv_buffer: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut argv_buffer_heap: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut argv_buffer_len: gsize = 0 as gsize;
    let mut stdin_pipe: GUnixPipe = GUnixPipe {
        fds: [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)],
    };
    let mut stdout_pipe: GUnixPipe = GUnixPipe {
        fds: [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)],
    };
    let mut stderr_pipe: GUnixPipe = GUnixPipe {
        fds: [-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int)],
    };
    let mut child_close_fds: [gint; 4] = [
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
    ];
    let mut n_child_close_fds: gint = 0 as gint;
    let mut source_fds_copy: *mut gint = ::core::ptr::null_mut::<gint>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !argv.is_null() && !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gspawn.c\0" as *const u8 as *const ::core::ffi::c_char,
            1932 as ::core::ffi::c_int,
            G_STRFUNC,
            b"argv != NULL && argv[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if stdin_pipe_out.is_null() || stdin_fd < 0 as ::core::ffi::c_int {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gspawn.c\0" as *const u8 as *const ::core::ffi::c_char,
            1933 as ::core::ffi::c_int,
            G_STRFUNC,
            b"stdin_pipe_out == NULL || stdin_fd < 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if stdout_pipe_out.is_null() || stdout_fd < 0 as ::core::ffi::c_int {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gspawn.c\0" as *const u8 as *const ::core::ffi::c_char,
            1934 as ::core::ffi::c_int,
            G_STRFUNC,
            b"stdout_pipe_out == NULL || stdout_fd < 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if stderr_pipe_out.is_null() || stderr_fd < 0 as ::core::ffi::c_int {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gspawn.c\0" as *const u8 as *const ::core::ffi::c_char,
            1935 as ::core::ffi::c_int,
            G_STRFUNC,
            b"stderr_pipe_out == NULL || stderr_fd < 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !stdin_pipe_out.is_null() {
        if safe_c2rust_g_unix_pipe_open(
            &raw mut stdin_pipe,
            pipe_flags as ::core::ffi::c_int,
            error,
        ) == 0
        {
            current_block = 14169730049765255660;
        } else if safe_c2rust_source_fds_collide_with_pipe(
            &raw mut stdin_pipe,
            source_fds as *const ::core::ffi::c_int,
            n_fds,
            error,
        ) != 0
        {
            current_block = 14169730049765255660;
        } else {
            let fresh0 = n_child_close_fds;
            n_child_close_fds = n_child_close_fds + 1;
            child_close_fds[fresh0 as usize] =
                safe_c2rust_g_unix_pipe_get(&raw mut stdin_pipe, G_UNIX_PIPE_END_WRITE) as gint;
            stdin_fd =
                safe_c2rust_g_unix_pipe_get(&raw mut stdin_pipe, G_UNIX_PIPE_END_READ) as gint;
            current_block = 3437258052017859086;
        }
    } else {
        current_block = 3437258052017859086;
    }
    match current_block {
        3437258052017859086 => {
            if !stdout_pipe_out.is_null() {
                if safe_c2rust_g_unix_pipe_open(
                    &raw mut stdout_pipe,
                    pipe_flags as ::core::ffi::c_int,
                    error,
                ) == 0
                {
                    current_block = 14169730049765255660;
                } else if safe_c2rust_source_fds_collide_with_pipe(
                    &raw mut stdout_pipe,
                    source_fds as *const ::core::ffi::c_int,
                    n_fds,
                    error,
                ) != 0
                {
                    current_block = 14169730049765255660;
                } else {
                    let fresh1 = n_child_close_fds;
                    n_child_close_fds = n_child_close_fds + 1;
                    child_close_fds[fresh1 as usize] =
                        safe_c2rust_g_unix_pipe_get(&raw mut stdout_pipe, G_UNIX_PIPE_END_READ)
                            as gint;
                    stdout_fd =
                        safe_c2rust_g_unix_pipe_get(&raw mut stdout_pipe, G_UNIX_PIPE_END_WRITE)
                            as gint;
                    current_block = 11913429853522160501;
                }
            } else {
                current_block = 11913429853522160501;
            }
            match current_block {
                14169730049765255660 => {}
                _ => {
                    if !stderr_pipe_out.is_null() {
                        if safe_c2rust_g_unix_pipe_open(
                            &raw mut stderr_pipe,
                            pipe_flags as ::core::ffi::c_int,
                            error,
                        ) == 0
                        {
                            current_block = 14169730049765255660;
                        } else if safe_c2rust_source_fds_collide_with_pipe(
                            &raw mut stderr_pipe,
                            source_fds as *const ::core::ffi::c_int,
                            n_fds,
                            error,
                        ) != 0
                        {
                            current_block = 14169730049765255660;
                        } else {
                            let fresh2 = n_child_close_fds;
                            n_child_close_fds = n_child_close_fds + 1;
                            child_close_fds[fresh2 as usize] = safe_c2rust_g_unix_pipe_get(
                                &raw mut stderr_pipe,
                                G_UNIX_PIPE_END_READ,
                            )
                                as gint;
                            stderr_fd = safe_c2rust_g_unix_pipe_get(
                                &raw mut stderr_pipe,
                                G_UNIX_PIPE_END_WRITE,
                            ) as gint;
                            current_block = 15597372965620363352;
                        }
                    } else {
                        current_block = 15597372965620363352;
                    }
                    match current_block {
                        14169730049765255660 => {}
                        _ => {
                            let fresh3 = n_child_close_fds;
                            n_child_close_fds = n_child_close_fds + 1;
                            child_close_fds[fresh3 as usize] = -(1 as ::core::ffi::c_int) as gint;
                            if intermediate_child == 0
                                && working_directory.is_null()
                                && close_descriptors == 0
                                && search_path_from_envp == 0
                                && child_setup.is_none()
                            {
                                status = safe_c2rust_do_posix_spawn(
                                    argv,
                                    envp,
                                    search_path,
                                    stdout_to_null,
                                    stderr_to_null,
                                    child_inherits_stdin,
                                    file_and_argv_zero,
                                    child_pid,
                                    &raw mut child_close_fds as *mut gint,
                                    stdin_fd,
                                    stdout_fd,
                                    stderr_fd,
                                    source_fds,
                                    target_fds,
                                    n_fds,
                                ) as gint;
                                if status == 0 as ::core::ffi::c_int {
                                    current_block = 7437622663018780271;
                                } else if status != ENOEXEC {
                                    g_set_error(
                                        error,
                                        safe_c2rust_g_spawn_error_quark(),
                                        G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"Failed to spawn child process \xE2\x80\x9C%s\xE2\x80\x9D (%s)\0"
                                                as *const u8 as *const gchar,
                                        ),
                                        *argv.offset(0 as ::core::ffi::c_int as isize),
                                        g_strerror(status),
                                    );
                                    current_block = 14169730049765255660;
                                } else {
                                    g_log(
                                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                                        G_LOG_LEVEL_DEBUG,
                                        b"posix_spawn failed (ENOEXEC), fall back to regular gspawn\0"
                                            as *const u8 as *const gchar,
                                    );
                                    current_block = 12381812505308290051;
                                }
                            } else {
                                current_block = 12381812505308290051;
                            }
                            match current_block {
                                14169730049765255660 => {}
                                _ => {
                                    match current_block {
                                        12381812505308290051 => {
                                            chosen_search_path = ::core::ptr::null::<gchar>();
                                            if search_path_from_envp != 0 {
                                                chosen_search_path = g_environ_getenv(
                                                    envp as *mut *mut gchar,
                                                    b"PATH\0" as *const u8 as *const gchar,
                                                );
                                            }
                                            if search_path != 0 && chosen_search_path.is_null() {
                                                chosen_search_path = g_getenv(
                                                    b"PATH\0" as *const u8 as *const gchar,
                                                );
                                            }
                                            if (search_path != 0 || search_path_from_envp != 0)
                                                && chosen_search_path.is_null()
                                            {
                                                chosen_search_path = b"/bin:/usr/bin:.\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char
                                                    as *const gchar;
                                            }
                                            if search_path != 0 || search_path_from_envp != 0 {
                                                if ({
                                                    let mut _g_boolean_var_38: ::core::ffi::c_int =
                                                        0;
                                                    if !chosen_search_path.is_null() {
                                                        _g_boolean_var_38 = 1 as ::core::ffi::c_int;
                                                    } else {
                                                        _g_boolean_var_38 = 0 as ::core::ffi::c_int;
                                                    }
                                                    _g_boolean_var_38
                                                })
                                                    as ::core::ffi::c_long
                                                    != 0
                                                {
                                                } else {
                                                    g_assertion_message_expr(
                                                        G_LOG_DOMAIN.as_ptr(),
                                                        b"../original/glib/gspawn.c\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        2051 as ::core::ffi::c_int,
                                                        G_STRFUNC,
                                                        b"chosen_search_path != NULL\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                            } else if ({
                                                let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
                                                if chosen_search_path.is_null() {
                                                    _g_boolean_var_39 = 1 as ::core::ffi::c_int;
                                                } else {
                                                    _g_boolean_var_39 = 0 as ::core::ffi::c_int;
                                                }
                                                _g_boolean_var_39
                                            })
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                            } else {
                                                g_assertion_message_expr(
                                                    G_LOG_DOMAIN.as_ptr(),
                                                    b"../original/glib/gspawn.c\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    2053 as ::core::ffi::c_int,
                                                    G_STRFUNC,
                                                    b"chosen_search_path == NULL\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                            }
                                            if !chosen_search_path.is_null() {
                                                search_path_buffer_len = strlen(
                                                    chosen_search_path
                                                        as *const ::core::ffi::c_char,
                                                )
                                                .wrapping_add(strlen(
                                                    *argv.offset(0 as ::core::ffi::c_int as isize)
                                                        as *const ::core::ffi::c_char,
                                                ))
                                                .wrapping_add(2 as size_t)
                                                    as gsize;
                                                if search_path_buffer_len < 4000 as gsize {
                                                    alloca_allocations.push(::std::vec::from_elem(
                                                        0,
                                                        search_path_buffer_len as usize,
                                                    ));
                                                    search_path_buffer = alloca_allocations
                                                        .last_mut()
                                                        .unwrap()
                                                        .as_mut_ptr()
                                                        as *mut gchar;
                                                } else {
                                                    search_path_buffer_heap =
                                                        g_malloc(search_path_buffer_len)
                                                            as *mut gchar;
                                                    search_path_buffer = search_path_buffer_heap;
                                                }
                                            }
                                            if search_path != 0 || search_path_from_envp != 0 {
                                                if ({
                                                    let mut _g_boolean_var_40: ::core::ffi::c_int =
                                                        0;
                                                    if !search_path_buffer.is_null() {
                                                        _g_boolean_var_40 = 1 as ::core::ffi::c_int;
                                                    } else {
                                                        _g_boolean_var_40 = 0 as ::core::ffi::c_int;
                                                    }
                                                    _g_boolean_var_40
                                                })
                                                    as ::core::ffi::c_long
                                                    != 0
                                                {
                                                } else {
                                                    g_assertion_message_expr(
                                                        G_LOG_DOMAIN.as_ptr(),
                                                        b"../original/glib/gspawn.c\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        2078 as ::core::ffi::c_int,
                                                        G_STRFUNC,
                                                        b"search_path_buffer != NULL\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                            } else if ({
                                                let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
                                                if search_path_buffer.is_null() {
                                                    _g_boolean_var_41 = 1 as ::core::ffi::c_int;
                                                } else {
                                                    _g_boolean_var_41 = 0 as ::core::ffi::c_int;
                                                }
                                                _g_boolean_var_41
                                            })
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                            } else {
                                                g_assertion_message_expr(
                                                    G_LOG_DOMAIN.as_ptr(),
                                                    b"../original/glib/gspawn.c\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    2080 as ::core::ffi::c_int,
                                                    G_STRFUNC,
                                                    b"search_path_buffer == NULL\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                            }
                                            argv_buffer_len = g_strv_length(argv as *mut *mut gchar)
                                                .wrapping_add(2 as guint)
                                                as gsize;
                                            if (argv_buffer_len as usize)
                                                < (4000 as usize).wrapping_div(
                                                    ::core::mem::size_of::<*mut gchar>() as usize,
                                                )
                                            {
                                                alloca_allocations.push(::std::vec::from_elem(
                                                    0,
                                                    (::core::mem::size_of::<*mut gchar>() as usize)
                                                        .wrapping_mul(argv_buffer_len as usize)
                                                        as usize,
                                                ));
                                                argv_buffer = alloca_allocations
                                                    .last_mut()
                                                    .unwrap()
                                                    .as_mut_ptr()
                                                    as *mut *mut gchar;
                                            } else {
                                                argv_buffer_heap = ({
                                                    let mut __n: gsize = argv_buffer_len;
                                                    let mut __s: gsize =
                                                        ::core::mem::size_of::<*mut gchar>()
                                                            as gsize;
                                                    let mut __p: gpointer = ::core::ptr::null_mut::<
                                                        ::core::ffi::c_void,
                                                    >(
                                                    );
                                                    if __s == 1 as gsize {
                                                        __p = g_malloc(__n);
                                                    } else if 0 != 0
                                                        && (__s == 0 as gsize
                                                            || __n <= G_MAXSIZE.wrapping_div(__s))
                                                    {
                                                        __p = g_malloc(__n.wrapping_mul(__s));
                                                    } else {
                                                        __p = g_malloc_n(__n, __s);
                                                    }
                                                    __p
                                                })
                                                    as *mut *mut gchar;
                                                argv_buffer = argv_buffer_heap;
                                            }
                                            source_fds_copy = ({
                                                let mut __n: gsize = n_fds;
                                                let mut __s: gsize =
                                                    ::core::mem::size_of::<::core::ffi::c_int>()
                                                        as gsize;
                                                let mut __p: gpointer =
                                                    ::core::ptr::null_mut::<::core::ffi::c_void>();
                                                if __s == 1 as gsize {
                                                    __p = g_malloc(__n);
                                                } else if 0 != 0
                                                    && (__s == 0 as gsize
                                                        || __n <= G_MAXSIZE.wrapping_div(__s))
                                                {
                                                    __p = g_malloc(__n.wrapping_mul(__s));
                                                } else {
                                                    __p = g_malloc_n(__n, __s);
                                                }
                                                __p
                                            })
                                                as *mut ::core::ffi::c_int
                                                as *mut gint;
                                            if n_fds > 0 as gsize {
                                                memcpy(
                                                    source_fds_copy as *mut ::core::ffi::c_void,
                                                    source_fds as *const ::core::ffi::c_void,
                                                    (::core::mem::size_of::<gint>() as size_t)
                                                        .wrapping_mul(n_fds as size_t),
                                                );
                                            }
                                            if safe_c2rust_g_unix_pipe_open(
                                                &raw mut child_err_report_pipe,
                                                pipe_flags as ::core::ffi::c_int,
                                                error,
                                            ) == 0
                                            {
                                                current_block = 14169730049765255660;
                                            } else if safe_c2rust_source_fds_collide_with_pipe(
                                                &raw mut child_err_report_pipe,
                                                source_fds as *const ::core::ffi::c_int,
                                                n_fds,
                                                error,
                                            ) != 0
                                            {
                                                current_block = 14169730049765255660;
                                            } else {
                                                if intermediate_child != 0 {
                                                    if safe_c2rust_g_unix_pipe_open(
                                                        &raw mut child_pid_report_pipe,
                                                        pipe_flags as ::core::ffi::c_int,
                                                        error,
                                                    ) == 0
                                                    {
                                                        current_block = 14169730049765255660;
                                                    } else if safe_c2rust_source_fds_collide_with_pipe(
                                                        &raw mut child_pid_report_pipe,
                                                        source_fds as *const ::core::ffi::c_int,
                                                        n_fds,
                                                        error,
                                                    ) != 0
                                                    {
                                                        current_block = 14169730049765255660;
                                                    } else {
                                                        current_block = 6471821049853688503;
                                                    }
                                                } else {
                                                    current_block = 6471821049853688503;
                                                }
                                                match current_block {
                                                    14169730049765255660 => {}
                                                    _ => {
                                                        pid = fork() as GPid;
                                                        if pid < 0 as ::core::ffi::c_int {
                                                            let mut errsv: ::core::ffi::c_int =
                                                                *__errno_location();
                                                            g_set_error(
                                                                error,
                                                                safe_c2rust_g_spawn_error_quark(),
                                                                G_SPAWN_ERROR_FORK
                                                                    as ::core::ffi::c_int
                                                                    as gint,
                                                                glib_gettext(
                                                                    b"Failed to fork (%s)\0"
                                                                        as *const u8
                                                                        as *const gchar,
                                                                ),
                                                                g_strerror(errsv as gint),
                                                            );
                                                            current_block = 14169730049765255660;
                                                        } else if pid == 0 as ::core::ffi::c_int {
                                                            signal(SIGCHLD, SIG_DFL);
                                                            signal(SIGINT, SIG_DFL);
                                                            signal(SIGTERM, SIG_DFL);
                                                            signal(SIGHUP, SIG_DFL);
                                                            signal(SIGPIPE, SIG_DFL);
                                                            safe_c2rust_g_unix_pipe_close(
                                                                &raw mut child_err_report_pipe,
                                                                G_UNIX_PIPE_END_READ,
                                                                ::core::ptr::null_mut::<*mut GError>(
                                                                ),
                                                            );
                                                            safe_c2rust_g_unix_pipe_close(
                                                                &raw mut child_pid_report_pipe,
                                                                G_UNIX_PIPE_END_READ,
                                                                ::core::ptr::null_mut::<*mut GError>(
                                                                ),
                                                            );
                                                            if child_close_fds
                                                                [0 as ::core::ffi::c_int as usize]
                                                                != -(1 as ::core::ffi::c_int)
                                                            {
                                                                let mut i: ::core::ffi::c_int =
                                                                    -(1 as ::core::ffi::c_int);
                                                                loop {
                                                                    i += 1;
                                                                    if !(child_close_fds
                                                                        [i as usize]
                                                                        != -(1
                                                                            as ::core::ffi::c_int))
                                                                    {
                                                                        break;
                                                                    }
                                                                    safe_c2rust_g_clear_fd(
                                                                        (&raw mut child_close_fds as *mut gint).offset(i as isize)
                                                                            as *mut ::core::ffi::c_int,
                                                                        ::core::ptr::null_mut::<*mut GError>(),
                                                                    );
                                                                }
                                                            }
                                                            if intermediate_child != 0 {
                                                                let mut grandchild_pid: GPid = 0;
                                                                grandchild_pid = fork() as GPid;
                                                                if grandchild_pid
                                                                    < 0 as ::core::ffi::c_int
                                                                {
                                                                    safe_c2rust_write_all(
                                                                        safe_c2rust_g_unix_pipe_get(
                                                                            &raw mut child_pid_report_pipe,
                                                                            G_UNIX_PIPE_END_WRITE,
                                                                        ) as gint,
                                                                        &raw mut grandchild_pid as gconstpointer,
                                                                        ::core::mem::size_of::<GPid>() as gsize,
                                                                    );
                                                                    safe_c2rust_write_err_and_exit(
                                                                        safe_c2rust_g_unix_pipe_get(
                                                                            &raw mut child_err_report_pipe,
                                                                            G_UNIX_PIPE_END_WRITE,
                                                                        ) as gint,
                                                                        CHILD_FORK_FAILED as ::core::ffi::c_int as gint,
                                                                    );
                                                                } else if grandchild_pid
                                                                    == 0 as ::core::ffi::c_int
                                                                {
                                                                    safe_c2rust_g_unix_pipe_close(
                                                                        &raw mut child_pid_report_pipe,
                                                                        G_UNIX_PIPE_END_WRITE,
                                                                        ::core::ptr::null_mut::<*mut GError>(),
                                                                    );
                                                                    safe_c2rust_do_exec(
                                                                        safe_c2rust_g_unix_pipe_get(
                                                                            &raw mut child_err_report_pipe,
                                                                            G_UNIX_PIPE_END_WRITE,
                                                                        ) as gint,
                                                                        stdin_fd,
                                                                        stdout_fd,
                                                                        stderr_fd,
                                                                        source_fds_copy,
                                                                        target_fds,
                                                                        n_fds,
                                                                        working_directory,
                                                                        argv,
                                                                        argv_buffer,
                                                                        argv_buffer_len,
                                                                        envp,
                                                                        close_descriptors,
                                                                        chosen_search_path,
                                                                        search_path_buffer,
                                                                        search_path_buffer_len,
                                                                        stdout_to_null,
                                                                        stderr_to_null,
                                                                        child_inherits_stdin,
                                                                        file_and_argv_zero,
                                                                        child_setup,
                                                                        user_data,
                                                                    );
                                                                } else {
                                                                    safe_c2rust_write_all(
                                                                        safe_c2rust_g_unix_pipe_get(
                                                                            &raw mut child_pid_report_pipe,
                                                                            G_UNIX_PIPE_END_WRITE,
                                                                        ) as gint,
                                                                        &raw mut grandchild_pid as gconstpointer,
                                                                        ::core::mem::size_of::<GPid>() as gsize,
                                                                    );
                                                                    safe_c2rust_g_unix_pipe_close(
                                                                        &raw mut child_pid_report_pipe,
                                                                        G_UNIX_PIPE_END_WRITE,
                                                                        ::core::ptr::null_mut::<*mut GError>(),
                                                                    );
                                                                    _exit(0 as ::core::ffi::c_int);
                                                                }
                                                            } else {
                                                                safe_c2rust_do_exec(
                                                                    safe_c2rust_g_unix_pipe_get(
                                                                        &raw mut child_err_report_pipe,
                                                                        G_UNIX_PIPE_END_WRITE,
                                                                    ) as gint,
                                                                    stdin_fd,
                                                                    stdout_fd,
                                                                    stderr_fd,
                                                                    source_fds_copy,
                                                                    target_fds,
                                                                    n_fds,
                                                                    working_directory,
                                                                    argv,
                                                                    argv_buffer,
                                                                    argv_buffer_len,
                                                                    envp,
                                                                    close_descriptors,
                                                                    chosen_search_path,
                                                                    search_path_buffer,
                                                                    search_path_buffer_len,
                                                                    stdout_to_null,
                                                                    stderr_to_null,
                                                                    child_inherits_stdin,
                                                                    file_and_argv_zero,
                                                                    child_setup,
                                                                    user_data,
                                                                );
                                                            }
                                                            current_block = 7437622663018780271;
                                                        } else {
                                                            let mut buf: [gint; 2] = [0; 2];
                                                            let mut n_ints: gint = 0 as gint;
                                                            safe_c2rust_g_unix_pipe_close(
                                                                &raw mut child_err_report_pipe,
                                                                G_UNIX_PIPE_END_WRITE,
                                                                ::core::ptr::null_mut::<*mut GError>(
                                                                ),
                                                            );
                                                            safe_c2rust_g_unix_pipe_close(
                                                                &raw mut child_pid_report_pipe,
                                                                G_UNIX_PIPE_END_WRITE,
                                                                ::core::ptr::null_mut::<*mut GError>(
                                                                ),
                                                            );
                                                            if intermediate_child != 0 {
                                                                while waitpid(
                                                                    pid as __pid_t,
                                                                    &raw mut status,
                                                                    0 as ::core::ffi::c_int,
                                                                ) < 0 as ::core::ffi::c_int
                                                                {
                                                                    if *__errno_location() == EINTR
                                                                    {
                                                                        continue;
                                                                    }
                                                                    if !(*__errno_location()
                                                                        == ECHILD)
                                                                    {
                                                                        g_log(
                                                                            G_LOG_DOMAIN.as_ptr() as *const gchar,
                                                                            G_LOG_LEVEL_WARNING,
                                                                            b"waitpid() should not fail in 'fork_exec'\0" as *const u8
                                                                                as *const gchar,
                                                                        );
                                                                    }
                                                                    break;
                                                                }
                                                            }
                                                            if safe_c2rust_read_ints(
                                                                safe_c2rust_g_unix_pipe_get(
                                                                    &raw mut child_err_report_pipe,
                                                                    G_UNIX_PIPE_END_READ,
                                                                ),
                                                                &raw mut buf as *mut gint,
                                                                2 as gint,
                                                                &raw mut n_ints,
                                                                error,
                                                            ) == 0
                                                            {
                                                                current_block =
                                                                    14169730049765255660;
                                                            } else if n_ints
                                                                >= 2 as ::core::ffi::c_int
                                                            {
                                                                match buf[0 as ::core::ffi::c_int
                                                                    as usize]
                                                                {
                                                                    0 => {
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            G_SPAWN_ERROR_CHDIR as ::core::ffi::c_int as gint,
                                                                            glib_gettext(
                                                                                b"Failed to change to directory \xE2\x80\x9C%s\xE2\x80\x9D (%s)\0"
                                                                                    as *const u8 as *const gchar,
                                                                            ),
                                                                            working_directory,
                                                                            g_strerror(buf[1 as ::core::ffi::c_int as usize]),
                                                                        );
                                                                    }
                                                                    1 => {
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            safe_c2rust__g_spawn_exec_err_to_g_error(
                                                                                buf[1 as ::core::ffi::c_int as usize],
                                                                            ),
                                                                            glib_gettext(
                                                                                b"Failed to execute child process \xE2\x80\x9C%s\xE2\x80\x9D (%s)\0"
                                                                                    as *const u8 as *const gchar,
                                                                            ),
                                                                            *argv.offset(0 as ::core::ffi::c_int as isize),
                                                                            g_strerror(buf[1 as ::core::ffi::c_int as usize]),
                                                                        );
                                                                    }
                                                                    2 => {
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
                                                                            glib_gettext(
                                                                                b"Failed to open file to remap file descriptor (%s)\0"
                                                                                    as *const u8 as *const gchar,
                                                                            ),
                                                                            g_strerror(buf[1 as ::core::ffi::c_int as usize]),
                                                                        );
                                                                    }
                                                                    3 => {
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
                                                                            glib_gettext(
                                                                                b"Failed to duplicate file descriptor for child process (%s)\0"
                                                                                    as *const u8 as *const gchar,
                                                                            ),
                                                                            g_strerror(buf[1 as ::core::ffi::c_int as usize]),
                                                                        );
                                                                    }
                                                                    4 => {
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            G_SPAWN_ERROR_FORK as ::core::ffi::c_int as gint,
                                                                            glib_gettext(
                                                                                b"Failed to fork child process (%s)\0" as *const u8
                                                                                    as *const gchar,
                                                                            ),
                                                                            g_strerror(buf[1 as ::core::ffi::c_int as usize]),
                                                                        );
                                                                    }
                                                                    5 => {
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
                                                                            glib_gettext(
                                                                                b"Failed to close file descriptor for child process (%s)\0"
                                                                                    as *const u8 as *const gchar,
                                                                            ),
                                                                            g_strerror(buf[1 as ::core::ffi::c_int as usize]),
                                                                        );
                                                                    }
                                                                    _ => {
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
                                                                            glib_gettext(
                                                                                b"Unknown error executing child process \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                                                                    as *const u8 as *const gchar,
                                                                            ),
                                                                            *argv.offset(0 as ::core::ffi::c_int as isize),
                                                                        );
                                                                    }
                                                                }
                                                                current_block =
                                                                    14169730049765255660;
                                                            } else {
                                                                if intermediate_child != 0 {
                                                                    n_ints = 0 as ::core::ffi::c_int
                                                                        as gint;
                                                                    if safe_c2rust_read_ints(
                                                                        safe_c2rust_g_unix_pipe_get(
                                                                            &raw mut child_pid_report_pipe,
                                                                            G_UNIX_PIPE_END_READ,
                                                                        ),
                                                                        &raw mut buf as *mut gint,
                                                                        1 as gint,
                                                                        &raw mut n_ints,
                                                                        error,
                                                                    ) == 0
                                                                    {
                                                                        current_block = 14169730049765255660;
                                                                    } else if n_ints < 1 as ::core::ffi::c_int {
                                                                        let mut errsv_0: ::core::ffi::c_int = *__errno_location();
                                                                        g_set_error(
                                                                            error,
                                                                            safe_c2rust_g_spawn_error_quark(),
                                                                            G_SPAWN_ERROR_FAILED as ::core::ffi::c_int as gint,
                                                                            glib_gettext(
                                                                                b"Failed to read enough data from child pid pipe (%s)\0"
                                                                                    as *const u8 as *const gchar,
                                                                            ),
                                                                            g_strerror(errsv_0 as gint),
                                                                        );
                                                                        current_block = 14169730049765255660;
                                                                    } else {
                                                                        pid = buf[0 as ::core::ffi::c_int as usize] as GPid;
                                                                        current_block = 10494165753505607199;
                                                                    }
                                                                } else {
                                                                    current_block =
                                                                        10494165753505607199;
                                                                }
                                                                match current_block {
                                                                    14169730049765255660 => {}
                                                                    _ => {
                                                                        safe_c2rust_g_unix_pipe_close(
                                                                            &raw mut child_err_report_pipe,
                                                                            G_UNIX_PIPE_END_READ,
                                                                            ::core::ptr::null_mut::<*mut GError>(),
                                                                        );
                                                                        safe_c2rust_g_unix_pipe_close(
                                                                            &raw mut child_pid_report_pipe,
                                                                            G_UNIX_PIPE_END_READ,
                                                                            ::core::ptr::null_mut::<*mut GError>(),
                                                                        );
                                                                        g_free(
                                                                            search_path_buffer_heap
                                                                                as gpointer,
                                                                        );
                                                                        g_free(
                                                                            argv_buffer_heap
                                                                                as gpointer,
                                                                        );
                                                                        g_free(
                                                                            source_fds_copy
                                                                                as gpointer,
                                                                        );
                                                                        if !child_pid.is_null() {
                                                                            *child_pid = pid;
                                                                        }
                                                                        current_block =
                                                                            7437622663018780271;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        14169730049765255660 => {}
                                        _ => {
                                            safe_c2rust_g_unix_pipe_close(
                                                &raw mut stdin_pipe,
                                                G_UNIX_PIPE_END_READ,
                                                ::core::ptr::null_mut::<*mut GError>(),
                                            );
                                            safe_c2rust_g_unix_pipe_close(
                                                &raw mut stdout_pipe,
                                                G_UNIX_PIPE_END_WRITE,
                                                ::core::ptr::null_mut::<*mut GError>(),
                                            );
                                            safe_c2rust_g_unix_pipe_close(
                                                &raw mut stderr_pipe,
                                                G_UNIX_PIPE_END_WRITE,
                                                ::core::ptr::null_mut::<*mut GError>(),
                                            );
                                            if !stdin_pipe_out.is_null() {
                                                *stdin_pipe_out = safe_c2rust_g_unix_pipe_steal(
                                                    &raw mut stdin_pipe,
                                                    G_UNIX_PIPE_END_WRITE,
                                                )
                                                    as gint;
                                            }
                                            if !stdout_pipe_out.is_null() {
                                                *stdout_pipe_out = safe_c2rust_g_unix_pipe_steal(
                                                    &raw mut stdout_pipe,
                                                    G_UNIX_PIPE_END_READ,
                                                )
                                                    as gint;
                                            }
                                            if !stderr_pipe_out.is_null() {
                                                *stderr_pipe_out = safe_c2rust_g_unix_pipe_steal(
                                                    &raw mut stderr_pipe,
                                                    G_UNIX_PIPE_END_READ,
                                                )
                                                    as gint;
                                            }
                                            return TRUE;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if pid > 0 as ::core::ffi::c_int {
        while waitpid(
            pid as __pid_t,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            if *__errno_location() == EINTR {
                continue;
            }
            if !(*__errno_location() == ECHILD) {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"waitpid() should not fail in 'fork_exec'\0" as *const u8 as *const gchar,
                );
            }
            break;
        }
    }
    safe_c2rust_g_unix_pipe_clear(&raw mut stdin_pipe);
    safe_c2rust_g_unix_pipe_clear(&raw mut stdout_pipe);
    safe_c2rust_g_unix_pipe_clear(&raw mut stderr_pipe);
    safe_c2rust_g_unix_pipe_clear(&raw mut child_err_report_pipe);
    safe_c2rust_g_unix_pipe_clear(&raw mut child_pid_report_pipe);
    let mut _pp: *mut *mut gchar = &raw mut search_path_buffer_heap;
    let mut _ptr: *mut gchar = *_pp;
    *_pp = ::core::ptr::null_mut::<gchar>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut *mut gchar = &raw mut argv_buffer_heap;
    let mut _ptr_0: *mut *mut gchar = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<*mut gchar>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut gint = &raw mut source_fds_copy;
    let mut _ptr_1: *mut gint = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<gint>();
    if !_ptr_1.is_null() {
        g_free(_ptr_1 as gpointer);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_script_execute(
    mut file: *const gchar,
    mut argv: *mut *mut gchar,
    mut argv_buffer: *mut *mut gchar,
    mut argv_buffer_len: gsize,
    mut envp: *mut *mut gchar,
) -> gboolean {
    let mut argc: gsize = 0 as gsize;
    while !(*argv.offset(argc as isize)).is_null() {
        argc = argc.wrapping_add(1);
    }
    if argc.wrapping_add(2 as gsize) > argv_buffer_len {
        return FALSE;
    }
    let ref mut fresh5 = *argv_buffer.offset(0 as ::core::ffi::c_int as isize);
    *fresh5 = b"/bin/sh\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        as *mut gchar;
    let ref mut fresh6 = *argv_buffer.offset(1 as ::core::ffi::c_int as isize);
    *fresh6 = file as *mut ::core::ffi::c_char as *mut gchar;
    while argc > 0 as gsize {
        let ref mut fresh7 = *argv_buffer.offset(argc.wrapping_add(1 as gsize) as isize);
        *fresh7 = *argv.offset(argc as isize);
        argc = argc.wrapping_sub(1);
    }
    if !envp.is_null() {
        execve(
            *argv_buffer.offset(0 as ::core::ffi::c_int as isize),
            argv_buffer as *const *mut ::core::ffi::c_char,
            envp as *const *mut ::core::ffi::c_char,
        );
    } else {
        execv(
            *argv_buffer.offset(0 as ::core::ffi::c_int as isize),
            argv_buffer as *const *mut ::core::ffi::c_char,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_my_strchrnul(mut str: *const gchar, mut c: gchar) -> *mut gchar {
    let mut p: *mut gchar = str as *mut gchar;
    while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != c as ::core::ffi::c_int {
        p = p.offset(1);
    }
    return p;
}
unsafe extern "C" fn safe_c2rust_g_execute(
    mut file: *const gchar,
    mut argv: *mut *mut gchar,
    mut argv_buffer: *mut *mut gchar,
    mut argv_buffer_len: gsize,
    mut envp: *mut *mut gchar,
    mut search_path: *const gchar,
    mut search_path_buffer: *mut gchar,
    mut search_path_buffer_len: gsize,
) -> gint {
    if file.is_null() || *file as ::core::ffi::c_int == '\0' as i32 {
        *__errno_location() = ENOENT;
        return -(1 as gint);
    }
    if search_path.is_null() || !strchr(file as *const ::core::ffi::c_char, '/' as i32).is_null() {
        if !envp.is_null() {
            execve(
                file as *const ::core::ffi::c_char,
                argv as *const *mut ::core::ffi::c_char,
                envp as *const *mut ::core::ffi::c_char,
            );
        } else {
            execv(
                file as *const ::core::ffi::c_char,
                argv as *const *mut ::core::ffi::c_char,
            );
        }
        if *__errno_location() == ENOEXEC
            && safe_c2rust_script_execute(file, argv, argv_buffer, argv_buffer_len, envp) == 0
        {
            *__errno_location() = ENOMEM;
            return -(1 as gint);
        }
    } else {
        let mut got_eacces: gboolean = 0 as gboolean;
        let mut path: *const gchar = ::core::ptr::null::<gchar>();
        let mut p: *const gchar = ::core::ptr::null::<gchar>();
        let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut len: gsize = 0;
        let mut pathlen: gsize = 0;
        path = search_path;
        len = strlen(file as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as gsize;
        pathlen = strlen(path as *const ::core::ffi::c_char) as gsize;
        name = search_path_buffer;
        if search_path_buffer_len < pathlen.wrapping_add(len).wrapping_add(1 as gsize) {
            *__errno_location() = ENOMEM;
            return -(1 as gint);
        }
        memcpy(
            name.offset(pathlen as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            file as *const ::core::ffi::c_void,
            len as size_t,
        );
        name = name.offset(pathlen as isize);
        *name = '/' as i32 as gchar;
        p = path;
        loop {
            let mut startp: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            path = p;
            p = safe_c2rust_my_strchrnul(path, ':' as i32 as gchar);
            if p == path {
                startp = name.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
            } else {
                startp = memcpy(
                    name.offset(-(p.offset_from(path) as ::core::ffi::c_long as isize))
                        as *mut ::core::ffi::c_void,
                    path as *const ::core::ffi::c_void,
                    p.offset_from(path) as ::core::ffi::c_long as size_t,
                ) as *mut ::core::ffi::c_char;
            }
            if !envp.is_null() {
                execve(
                    startp,
                    argv as *const *mut ::core::ffi::c_char,
                    envp as *const *mut ::core::ffi::c_char,
                );
            } else {
                execv(startp, argv as *const *mut ::core::ffi::c_char);
            }
            if *__errno_location() == ENOEXEC
                && safe_c2rust_script_execute(startp, argv, argv_buffer, argv_buffer_len, envp) == 0
            {
                *__errno_location() = ENOMEM;
                return -(1 as gint);
            }
            let mut current_block_38: u64;
            match *__errno_location() {
                EACCES => {
                    got_eacces = TRUE as gboolean;
                    current_block_38 = 11600600136415632188;
                }
                ENOENT | ESTALE | ENOTDIR => {
                    current_block_38 = 11600600136415632188;
                }
                ENODEV | ETIMEDOUT => {
                    current_block_38 = 17500079516916021833;
                }
                _ => return -(1 as gint),
            }
            match current_block_38 {
                11600600136415632188 => {}
                _ => {}
            }
            let fresh4 = p;
            p = p.offset(1);
            if !(*fresh4 as ::core::ffi::c_int != '\0' as i32) {
                break;
            }
        }
        if got_eacces != 0 {
            *__errno_location() = EACCES;
        }
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spawn_close_pid(mut pid: GPid) {}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
