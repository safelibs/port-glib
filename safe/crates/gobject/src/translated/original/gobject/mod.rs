#![allow(
    path_statements,
    unpredictable_function_pointer_comparisons,
    unused,
    unused_parens
)]

// The files below remain a mechanically translated copy of upstream GObject.
// Keep generated-style lints contained here so hand-written runtime modules can
// move toward stricter ABI and unsafe-audit checks independently.
pub mod gatomicarray;
pub mod gbinding;
pub mod gbindinggroup;
pub mod gboxed;
pub mod gclosure;
pub mod genums;
pub mod gmarshal;
pub mod gobject;
pub mod gparam;
pub mod gparamspecs;
pub mod gsignal;
pub mod gsignalgroup;
pub mod gsourceclosure;
pub mod gtype;
pub mod gtypemodule;
pub mod gtypeplugin;
pub mod gvalue;
pub mod gvaluearray;
pub mod gvaluetransform;
pub mod gvaluetypes;
