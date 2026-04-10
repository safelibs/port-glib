use std::collections::HashMap;
use std::ffi::c_char;
use std::ffi::c_void;
use std::sync::{Mutex, OnceLock};

use crate::abi::GError;
use crate::bridge;
use crate::ffi::{gboolean, gchar, gint, gssize, gpointer, GDestroyNotify, GQuark};

type GMarkupParseContext = c_void;
type GMarkupParser = c_void;
type GMarkupParseFlags = guint;
type guint = crate::ffi::guint;

unsafe extern "C" {
    fn g_markup_error_quark() -> GQuark;
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    fn g_return_if_fail_warning(
        log_domain: *const c_char,
        pretty_function: *const c_char,
        expression: *const c_char,
    );
}

const FALSE: gboolean = 0;
const G_MARKUP_ERROR_PARSE: gint = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MarkupPhase {
    Ready,
    Feeding,
    Failed,
    Finished,
}

#[derive(Clone, Copy)]
struct MarkupState {
    phase: MarkupPhase,
    refs: usize,
}

fn states() -> &'static Mutex<HashMap<usize, MarkupState>> {
    static STATES: OnceLock<Mutex<HashMap<usize, MarkupState>>> = OnceLock::new();
    STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn ensure_failed_error(error: *mut *mut GError, message: &'static std::ffi::CStr) {
    if !error.is_null() && (*error).is_null() {
        *error = g_error_new_literal(
            g_markup_error_quark(),
            G_MARKUP_ERROR_PARSE,
            message.as_ptr(),
        );
    }
}

#[unsafe(export_name = "g_markup_parse_context_new")]
pub unsafe extern "C" fn markup_parse_context_new(
    parser: *const GMarkupParser,
    flags: GMarkupParseFlags,
    user_data: gpointer,
    user_data_dnotify: GDestroyNotify,
) -> *mut GMarkupParseContext {
    let context = bridge::markup_parse_context_new(parser, flags, user_data, user_data_dnotify);
    if !context.is_null() {
        states().lock().expect("markup state mutex poisoned").insert(
            context as usize,
            MarkupState {
                phase: MarkupPhase::Ready,
                refs: 1,
            },
        );
    }
    context
}

#[unsafe(export_name = "g_markup_parse_context_ref")]
pub unsafe extern "C" fn markup_parse_context_ref(
    context: *mut GMarkupParseContext,
) -> *mut GMarkupParseContext {
    if !context.is_null() {
        let mut states = states().lock().expect("markup state mutex poisoned");
        if let Some(state) = states.get_mut(&(context as usize)) {
            state.refs += 1;
        }
    }
    bridge::markup_parse_context_ref(context)
}

#[unsafe(export_name = "g_markup_parse_context_unref")]
pub unsafe extern "C" fn markup_parse_context_unref(context: *mut GMarkupParseContext) {
    bridge::markup_parse_context_unref(context);
    if context.is_null() {
        return;
    }

    let mut states = states().lock().expect("markup state mutex poisoned");
    if let Some(state) = states.get_mut(&(context as usize)) {
        state.refs = state.refs.saturating_sub(1);
        if state.refs == 0 {
            states.remove(&(context as usize));
        }
    }
}

#[unsafe(export_name = "g_markup_parse_context_free")]
pub unsafe extern "C" fn markup_parse_context_free(context: *mut GMarkupParseContext) {
    bridge::markup_parse_context_free(context);
    if !context.is_null() {
        states()
            .lock()
            .expect("markup state mutex poisoned")
            .remove(&(context as usize));
    }
}

#[unsafe(export_name = "g_markup_parse_context_parse")]
pub unsafe extern "C" fn markup_parse_context_parse(
    context: *mut GMarkupParseContext,
    text: *const gchar,
    text_len: gssize,
    error: *mut *mut GError,
) -> gboolean {
    if context.is_null() {
        return bridge::markup_parse_context_parse(context, text, text_len, error);
    }

    {
        let mut states = states().lock().expect("markup state mutex poisoned");
        if let Some(state) = states.get_mut(&(context as usize)) {
            match state.phase {
                MarkupPhase::Failed => {
                    ensure_failed_error(
                        error,
                        c"Cannot continue parsing after a markup parser error",
                    );
                    return FALSE;
                }
                MarkupPhase::Finished => {
                    ensure_failed_error(
                        error,
                        c"Cannot continue parsing after g_markup_parse_context_end_parse()",
                    );
                    return FALSE;
                }
                MarkupPhase::Ready | MarkupPhase::Feeding => {
                    state.phase = MarkupPhase::Feeding;
                }
            }
        }
    }

    let result = bridge::markup_parse_context_parse(context, text, text_len, error);
    if result == FALSE {
        if let Some(state) = states()
            .lock()
            .expect("markup state mutex poisoned")
            .get_mut(&(context as usize))
        {
            state.phase = MarkupPhase::Failed;
        }
    }
    result
}

#[unsafe(export_name = "g_markup_parse_context_end_parse")]
pub unsafe extern "C" fn markup_parse_context_end_parse(
    context: *mut GMarkupParseContext,
    error: *mut *mut GError,
) -> gboolean {
    if context.is_null() {
        return bridge::markup_parse_context_end_parse(context, error);
    }

    {
        let mut states = states().lock().expect("markup state mutex poisoned");
        if let Some(state) = states.get_mut(&(context as usize)) {
            match state.phase {
                MarkupPhase::Failed => {
                    g_return_if_fail_warning(
                        c"GLib".as_ptr(),
                        c"g_markup_parse_context_end_parse".as_ptr(),
                        c"context->state != STATE_ERROR".as_ptr(),
                    );
                    ensure_failed_error(
                        error,
                        c"Markup parser is already in an error state",
                    );
                    return FALSE;
                }
                MarkupPhase::Finished => {
                    return 1;
                }
                MarkupPhase::Ready | MarkupPhase::Feeding => {}
            }
        }
    }

    let result = bridge::markup_parse_context_end_parse(context, error);
    let mut states = states().lock().expect("markup state mutex poisoned");
    if let Some(state) = states.get_mut(&(context as usize)) {
        state.phase = if result == FALSE {
            MarkupPhase::Failed
        } else {
            MarkupPhase::Finished
        };
    }
    result
}
