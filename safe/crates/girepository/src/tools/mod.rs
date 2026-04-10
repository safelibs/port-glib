#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InstalledTool {
    pub binary_name: &'static str,
    pub build_relpath: &'static str,
}

pub const MULTIARCH_HELPER_SUBDIR: &str = "glib-2.0";

pub const INSTALLED_TOOLS: &[InstalledTool] = &[
    InstalledTool {
        binary_name: "gi-compile-repository",
        build_relpath: "girepository/compiler/gi-compile-repository",
    },
    InstalledTool {
        binary_name: "gi-decompile-typelib",
        build_relpath: "girepository/decompiler/gi-decompile-typelib",
    },
    InstalledTool {
        binary_name: "gi-inspect-typelib",
        build_relpath: "girepository/inspector/gi-inspect-typelib",
    },
];
