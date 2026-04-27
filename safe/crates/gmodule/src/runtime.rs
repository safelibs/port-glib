use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::{Mutex, OnceLock};

use crate::ffi::{gboolean, gchar, gint, gpointer, GQuark};

const G_MODULE_BIND_LAZY: gint = 1 << 0;
const G_MODULE_BIND_LOCAL: gint = 1 << 1;
const G_MODULE_ERROR_FAILED: gint = 0;
const G_MODULE_ERROR_CHECK_FAILED: gint = 1;
const RTLD_LAZY: c_int = 1;
const RTLD_NOW: c_int = 2;
const RTLD_LOCAL: c_int = 0;
const RTLD_GLOBAL: c_int = 0x100;
const TRUE: gboolean = 1;
const FALSE: gboolean = 0;

pub(crate) type GModule = Module;
type GModuleUnload = Option<unsafe extern "C" fn(*mut GModule)>;

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

#[derive(Default)]
struct Registry {
    main_module: usize,
    modules_by_name: HashMap<String, usize>,
}

pub(crate) struct Module {
    pub(crate) file_name: Option<CString>,
    pub(crate) handle: *mut c_void,
    pub(crate) ref_count: usize,
    pub(crate) is_resident: bool,
    pub(crate) unload: GModuleUnload,
    pub(crate) is_main: bool,
}

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;

    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error_literal(err: *mut *mut crate::glib_abi::GError, domain: GQuark, code: gint, message: *const gchar);
    fn g_strdup(string: *const gchar) -> *mut gchar;
}

fn registry() -> &'static Mutex<Registry> {
    static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::default()))
}

fn to_cstring_lossy(text: &str) -> CString {
    CString::new(text).unwrap_or_else(|_| CString::new("invalid module error").unwrap())
}

pub(crate) fn set_module_error(message: Option<String>) {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = message.map(|message| to_cstring_lossy(&message));
    });
}

pub(crate) fn module_error_message() -> Option<String> {
    LAST_ERROR.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|message| message.to_string_lossy().into_owned())
    })
}

pub(crate) fn module_error_ptr() -> *const gchar {
    LAST_ERROR.with(|slot| {
        slot.borrow()
            .as_ref()
            .map_or(ptr::null(), |message| message.as_ptr().cast())
    })
}

pub(crate) unsafe fn set_open_error(error: *mut *mut crate::glib_abi::GError, code: gint, message: &str) {
    set_module_error(Some(message.to_owned()));
    if error.is_null() {
        return;
    }
    let message = to_cstring_lossy(message);
    g_set_error_literal(error, module_error_quark(), code, message.as_ptr().cast());
}

pub(crate) unsafe fn module_error_quark() -> GQuark {
    static QUARK: OnceLock<GQuark> = OnceLock::new();
    *QUARK.get_or_init(|| unsafe { g_quark_from_static_string(c"g-module-error-quark".as_ptr().cast()) })
}

pub(crate) fn open_flags(flags: gint) -> c_int {
    let binding = if (flags & G_MODULE_BIND_LAZY) != 0 {
        RTLD_LAZY
    } else {
        RTLD_NOW
    };
    let visibility = if (flags & G_MODULE_BIND_LOCAL) != 0 {
        RTLD_LOCAL
    } else {
        RTLD_GLOBAL
    };
    binding | visibility
}

pub(crate) unsafe fn dlerror_message() -> String {
    let message = dlerror();
    if message.is_null() {
        "unknown dynamic loader error".to_owned()
    } else {
        CStr::from_ptr(message).to_string_lossy().into_owned()
    }
}

pub(crate) unsafe fn module_symbol(handle: *mut c_void, symbol_name: *const gchar) -> Result<gpointer, String> {
    dlerror();
    let symbol = dlsym(handle, symbol_name.cast());
    let error = dlerror();
    if error.is_null() {
        Ok(symbol.cast())
    } else {
        Err(CStr::from_ptr(error).to_string_lossy().into_owned())
    }
}

pub(crate) unsafe fn duplicate_string(text: &str) -> *mut gchar {
    let text = to_cstring_lossy(text);
    g_strdup(text.as_ptr().cast())
}

pub(crate) unsafe fn open_main_module(flags: gint, error: *mut *mut crate::glib_abi::GError) -> *mut GModule {
    {
        let registry = registry().lock().unwrap();
        if registry.main_module != 0 {
            let module = registry.main_module as *mut GModule;
            (*module).ref_count += 1;
            set_module_error(None);
            return module;
        }
    }

    let handle = dlopen(ptr::null(), open_flags(flags));
    if handle.is_null() {
        let message = dlerror_message();
        set_open_error(error, G_MODULE_ERROR_FAILED, &message);
        return ptr::null_mut();
    }

    let module = Box::into_raw(Box::new(Module {
        file_name: None,
        handle,
        ref_count: 1,
        is_resident: false,
        unload: None,
        is_main: true,
    }));

    let mut registry = registry().lock().unwrap();
    registry.main_module = module as usize;
    set_module_error(None);
    module
}

pub(crate) unsafe fn open_named_module(
    file_name: &CStr,
    flags: gint,
    error: *mut *mut crate::glib_abi::GError,
) -> *mut GModule {
    let file_name_string = file_name.to_string_lossy().into_owned();
    {
        let registry = registry().lock().unwrap();
        if let Some(&module) = registry.modules_by_name.get(&file_name_string) {
            let module = module as *mut GModule;
            (*module).ref_count += 1;
            set_module_error(None);
            return module;
        }
    }

    let candidate_name = resolve_module_name(&file_name_string);
    if candidate_name.ends_with(".la") {
        set_open_error(
            error,
            G_MODULE_ERROR_FAILED,
            &format!("unsupported libtool archive '{}'", candidate_name),
        );
        return ptr::null_mut();
    }

    let candidate_name = to_cstring_lossy(&candidate_name);
    let handle = dlopen(candidate_name.as_ptr(), open_flags(flags));
    if handle.is_null() {
        let message = dlerror_message();
        set_open_error(error, G_MODULE_ERROR_FAILED, &message);
        return ptr::null_mut();
    }

    let module = Box::into_raw(Box::new(Module {
        file_name: Some(file_name.to_owned()),
        handle,
        ref_count: 1,
        is_resident: false,
        unload: None,
        is_main: false,
    }));

    let check_init = match module_symbol(handle, c"g_module_check_init".as_ptr().cast()) {
        Ok(symbol) if !symbol.is_null() => Some(std::mem::transmute::<gpointer, unsafe extern "C" fn(*mut GModule) -> *const gchar>(symbol)),
        _ => None,
    };

    if let Some(check_init) = check_init {
        let failure = check_init(module);
        if !failure.is_null() {
            let failure = CStr::from_ptr(failure).to_string_lossy().into_owned();
            let message = format!(
                "GModule ({}) initialization check failed: {}",
                file_name_string, failure
            );
            let _ = dlclose(handle);
            drop(Box::from_raw(module));
            set_open_error(error, G_MODULE_ERROR_CHECK_FAILED, &message);
            return ptr::null_mut();
        }
    }

    if let Ok(symbol) = module_symbol(handle, c"g_module_unload".as_ptr().cast()) {
        if !symbol.is_null() {
            (*module).unload = Some(std::mem::transmute::<gpointer, unsafe extern "C" fn(*mut GModule)>(symbol));
        }
    }

    let mut registry = registry().lock().unwrap();
    registry
        .modules_by_name
        .insert(file_name_string, module as usize);
    set_module_error(None);
    module
}

fn resolve_module_name(file_name: &str) -> String {
    let path = Path::new(file_name);
    if path.is_file() {
        return file_name.to_owned();
    }

    let basename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(file_name);
    let dirname = path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut candidates = Vec::new();
    if basename.starts_with("lib") {
        candidates.push(format!("{basename}.so"));
    } else {
        candidates.push(format!("lib{basename}.so"));
    }
    candidates.push(format!("{file_name}.la"));
    if Path::new(file_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_none()
    {
        candidates.push(format!("{file_name}.so"));
    }
    candidates.push(file_name.to_owned());

    for candidate in candidates {
        let resolved = if Path::new(&candidate).is_absolute() || candidate.contains('/') {
            PathBuf::from(&candidate)
        } else {
            dirname.join(&candidate)
        };
        if resolved.is_file() {
            return resolved.to_string_lossy().into_owned();
        }
    }

    if path.extension().and_then(|extension| extension.to_str()).is_none() {
        if basename.starts_with("lib") {
            return format!("{file_name}.so");
        }
        return dirname
            .join(format!("lib{basename}.so"))
            .to_string_lossy()
            .into_owned();
    }

    file_name.to_owned()
}

pub(crate) unsafe fn close_module(module: *mut GModule) -> gboolean {
    if module.is_null() {
        return FALSE;
    }

    set_module_error(None);
    (*module).ref_count = (*module).ref_count.saturating_sub(1);

    if (*module).ref_count != 0 || (*module).is_resident {
        return TRUE;
    }

    if let Some(unload) = (*module).unload.take() {
        unload(module);
    }

    {
        let mut registry = registry().lock().unwrap();
        if (*module).is_main {
            if registry.main_module == module as usize {
                registry.main_module = 0;
            }
        } else if let Some(file_name) = (*module).file_name.as_ref() {
            registry
                .modules_by_name
                .remove(&file_name.to_string_lossy().into_owned());
        }
    }

    if !(*module).is_main && dlclose((*module).handle) != 0 {
        let message = dlerror_message();
        set_module_error(Some(message));
    }

    drop(Box::from_raw(module));
    if module_error_message().is_none() { TRUE } else { FALSE }
}
