use std::ffi::{c_void, CStr};

use crate::bridge;
use crate::ffi::{gboolean, gconstpointer, gchar, gsize, GDestroyNotify};

type GVariant = c_void;
type GVariantType = c_void;
type GBytes = c_void;

unsafe extern "C" {
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_get_size(value: *mut GVariant) -> gsize;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
}

const FALSE: gboolean = 0;
const G_VARIANT_MAX_RECURSION_DEPTH: usize = 128;

#[derive(Clone, Copy)]
struct TypeInfo {
    min_size: usize,
    variable_sized: bool,
}

enum FrameKind {
    Array,
    Maybe,
    Tuple(u8),
}

struct Frame {
    kind: FrameKind,
    children: Vec<TypeInfo>,
}

fn leaf_type_info(token: u8) -> Option<TypeInfo> {
    match token {
        b'b' | b'y' => Some(TypeInfo {
            min_size: 1,
            variable_sized: false,
        }),
        b'n' | b'q' => Some(TypeInfo {
            min_size: 2,
            variable_sized: false,
        }),
        b'i' | b'u' | b'h' => Some(TypeInfo {
            min_size: 4,
            variable_sized: false,
        }),
        b'x' | b't' | b'd' => Some(TypeInfo {
            min_size: 8,
            variable_sized: false,
        }),
        b's' | b'o' | b'g' => Some(TypeInfo {
            min_size: 1,
            variable_sized: true,
        }),
        b'v' => Some(TypeInfo {
            min_size: 3,
            variable_sized: true,
        }),
        _ => None,
    }
}

fn complete_type(mut value: TypeInfo, stack: &mut Vec<Frame>, root: &mut Option<TypeInfo>) -> bool {
    loop {
        match stack.last_mut() {
            Some(Frame {
                kind: FrameKind::Array,
                children,
            }) => {
                if !children.is_empty() {
                    return false;
                }
                children.push(value);
                stack.pop();
                value = TypeInfo {
                    min_size: 0,
                    variable_sized: true,
                };
            }
            Some(Frame {
                kind: FrameKind::Maybe,
                children,
            }) => {
                if !children.is_empty() {
                    return false;
                }
                children.push(value);
                stack.pop();
                value = TypeInfo {
                    min_size: 0,
                    variable_sized: true,
                };
            }
            Some(frame) => {
                frame.children.push(value);
                return true;
            }
            None => {
                if root.is_some() {
                    return false;
                }
                *root = Some(value);
                return true;
            }
        }
    }
}

fn finish_tuple(frame: Frame) -> Option<TypeInfo> {
    let expected_close = match frame.kind {
        FrameKind::Tuple(close) => close,
        _ => return None,
    };
    let min_size = frame
        .children
        .iter()
        .map(|child| child.min_size)
        .sum::<usize>()
        + frame.children[..frame.children.len().saturating_sub(1)]
            .iter()
            .filter(|child| child.variable_sized)
            .count();
    let variable_sized = frame.children.iter().any(|child| child.variable_sized);
    if expected_close == b'}' && frame.children.len() != 2 {
        return None;
    }
    Some(TypeInfo {
        min_size,
        variable_sized,
    })
}

fn minimum_serialized_size(type_string: &[u8]) -> Option<usize> {
    let mut stack = Vec::<Frame>::new();
    let mut root = None;

    for &token in type_string {
        match token {
            b'a' => {
                if stack.len() + 1 > G_VARIANT_MAX_RECURSION_DEPTH {
                    return None;
                }
                stack.push(Frame {
                    kind: FrameKind::Array,
                    children: Vec::new(),
                });
            }
            b'm' => {
                if stack.len() + 1 > G_VARIANT_MAX_RECURSION_DEPTH {
                    return None;
                }
                stack.push(Frame {
                    kind: FrameKind::Maybe,
                    children: Vec::new(),
                });
            }
            b'(' => {
                if stack.len() + 1 > G_VARIANT_MAX_RECURSION_DEPTH {
                    return None;
                }
                stack.push(Frame {
                    kind: FrameKind::Tuple(b')'),
                    children: Vec::new(),
                });
            }
            b'{' => {
                if stack.len() + 1 > G_VARIANT_MAX_RECURSION_DEPTH {
                    return None;
                }
                stack.push(Frame {
                    kind: FrameKind::Tuple(b'}'),
                    children: Vec::new(),
                });
            }
            b')' | b'}' => {
                let frame = stack.pop()?;
                let expected = match frame.kind {
                    FrameKind::Tuple(expected) => expected,
                    _ => return None,
                };
                if expected != token {
                    return None;
                }
                let value = finish_tuple(frame)?;
                if !complete_type(value, &mut stack, &mut root) {
                    return None;
                }
            }
            _ => {
                let value = leaf_type_info(token)?;
                if !complete_type(value, &mut stack, &mut root) {
                    return None;
                }
            }
        }
    }

    if !stack.is_empty() {
        return None;
    }
    root.map(|value| value.min_size)
}

unsafe fn is_serialized_layout_plausible(type_ptr: *const GVariantType, size: gsize) -> bool {
    let Some(type_ptr) = (!type_ptr.is_null()).then_some(type_ptr.cast::<i8>()) else {
        return false;
    };
    let type_string = CStr::from_ptr(type_ptr);
    let min_size = minimum_serialized_size(type_string.to_bytes()).unwrap_or(usize::MAX);
    size >= min_size
}

unsafe fn variant_layout_is_plausible(value: *mut GVariant) -> bool {
    if value.is_null() {
        return false;
    }
    let type_string = g_variant_get_type_string(value);
    if type_string.is_null() {
        return false;
    }
    let min_size = minimum_serialized_size(CStr::from_ptr(type_string.cast()).to_bytes())
        .unwrap_or(usize::MAX);
    g_variant_get_size(value) >= min_size
}

#[unsafe(export_name = "g_variant_new_from_bytes")]
pub unsafe extern "C" fn variant_new_from_bytes(
    type_: *const GVariantType,
    bytes: *mut GBytes,
    trusted: gboolean,
) -> *mut GVariant {
    let mut size = 0usize;
    let data = g_bytes_get_data(bytes, core::ptr::addr_of_mut!(size));
    let trusted = if trusted != 0 && data.is_null() {
        FALSE
    } else if trusted != 0 && !is_serialized_layout_plausible(type_, size) {
        FALSE
    } else {
        trusted
    };
    bridge::variant_new_from_bytes(type_, bytes, trusted)
}

#[unsafe(export_name = "g_variant_new_from_data")]
pub unsafe extern "C" fn variant_new_from_data(
    type_: *const GVariantType,
    data: gconstpointer,
    size: gsize,
    trusted: gboolean,
    notify: GDestroyNotify,
    user_data: *mut c_void,
) -> *mut GVariant {
    let trusted = if trusted != 0 && (data.is_null() || !is_serialized_layout_plausible(type_, size))
    {
        FALSE
    } else {
        trusted
    };
    bridge::variant_new_from_data(type_, data, size, trusted, notify, user_data)
}

#[unsafe(export_name = "g_variant_is_normal_form")]
pub unsafe extern "C" fn variant_is_normal_form(value: *mut GVariant) -> gboolean {
    if !variant_layout_is_plausible(value) {
        return FALSE;
    }
    bridge::variant_is_normal_form(value)
}

#[unsafe(export_name = "g_variant_get_normal_form")]
pub unsafe extern "C" fn variant_get_normal_form(value: *mut GVariant) -> *mut GVariant {
    let _ = variant_layout_is_plausible(value);
    bridge::variant_get_normal_form(value)
}

#[unsafe(export_name = "g_variant_byteswap")]
pub unsafe extern "C" fn variant_byteswap(value: *mut GVariant) -> *mut GVariant {
    let _ = variant_layout_is_plausible(value);
    bridge::variant_byteswap(value)
}
