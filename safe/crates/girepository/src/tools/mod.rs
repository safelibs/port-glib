use crate::parser;

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

fn env_paths(name: &str) -> Vec<std::path::PathBuf> {
    std::env::var_os(name)
        .map(|value| std::env::split_paths(&value).collect())
        .unwrap_or_default()
}

fn exe_relative_dirs() -> Vec<std::path::PathBuf> {
    let mut dirs = Vec::new();
    let Ok(exe) = std::env::current_exe() else {
        return dirs;
    };
    let Some(exe_dir) = exe.parent() else {
        return dirs;
    };

    dirs.push(exe_dir.to_path_buf());
    match exe_dir.file_name().and_then(|name| name.to_str()) {
        Some("compiler" | "decompiler" | "inspector") => {
            if let Some(repository_dir) = exe_dir.parent() {
                dirs.push(repository_dir.join("introspection"));
                dirs.push(repository_dir.to_path_buf());
            }
        }
        Some(MULTIARCH_HELPER_SUBDIR) => {
            if let Some(multiarch_dir) = exe_dir.parent() {
                dirs.push(multiarch_dir.join(parser::TYPELIB_SUBDIR));
                dirs.push(multiarch_dir.join(parser::GIR_SUBDIR));
            }
            dirs.push(std::path::PathBuf::from("/usr/share/gir-1.0"));
        }
        _ => {}
    }
    dirs
}

fn typelib_search_dirs(extra: Option<&std::path::Path>) -> Vec<std::path::PathBuf> {
    let mut dirs = Vec::new();
    if let Some(extra) = extra {
        dirs.push(extra.to_path_buf());
    }
    dirs.extend(env_paths("GI_TYPELIB_PATH"));
    dirs.extend(exe_relative_dirs());
    dirs.push(std::path::PathBuf::from(
        "/usr/lib/x86_64-linux-gnu/girepository-1.0",
    ));
    dirs.push(std::path::PathBuf::from("/usr/lib/girepository-1.0"));
    dirs
}

fn gir_search_dirs(extra: Option<&std::path::Path>) -> Vec<std::path::PathBuf> {
    let mut dirs = Vec::new();
    if let Some(extra) = extra {
        dirs.push(extra.to_path_buf());
    }
    dirs.extend(env_paths("GI_GIR_PATH"));
    dirs.extend(exe_relative_dirs());
    dirs.push(std::path::PathBuf::from("/usr/share/gir-1.0"));
    dirs.push(std::path::PathBuf::from(
        "/usr/lib/x86_64-linux-gnu/gir-1.0",
    ));
    dirs
}

fn parse_output_arg(args: &[String], index: &mut usize) -> Option<std::path::PathBuf> {
    let arg = args.get(*index)?;
    if arg == "--output" || arg == "-o" {
        *index += 1;
        return args.get(*index).map(std::path::PathBuf::from);
    }
    arg.strip_prefix("--output=").map(std::path::PathBuf::from)
}

pub fn run_compile_repository() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut output = None;
    let mut input = None;
    let mut index = 0;
    while index < args.len() {
        if let Some(path) = parse_output_arg(&args, &mut index) {
            output = Some(path);
        } else if !args[index].starts_with('-') {
            input = Some(std::path::PathBuf::from(&args[index]));
        }
        index += 1;
    }

    let input = input.ok_or_else(|| "missing GIR input path".to_owned())?;
    let output = output.ok_or_else(|| "missing --output path".to_owned())?;
    parser::compile_gir_to_typelib(&input, &output)
}

pub fn run_decompile_typelib() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut output = None;
    let mut input = None;
    let mut index = 0;
    while index < args.len() {
        if let Some(path) = parse_output_arg(&args, &mut index) {
            output = Some(path);
        } else if !args[index].starts_with('-') {
            input = Some(std::path::PathBuf::from(&args[index]));
        }
        index += 1;
    }

    let input = input.ok_or_else(|| "missing typelib input path".to_owned())?;
    let gir = parser::decompile_typelib_to_gir(&input, &gir_search_dirs(input.parent()))?;
    if let Some(output) = output {
        std::fs::write(&output, gir).map_err(|error| format!("{}: {error}", output.display()))?;
    } else {
        print!("{gir}");
    }
    Ok(())
}

pub fn run_inspect_typelib() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut print_shlibs = false;
    let mut print_typelibs = false;
    let mut namespace = None;
    let mut version = None;
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {
            "--print-shlibs" => print_shlibs = true,
            "--print-typelibs" => print_typelibs = true,
            "--typelib-version" => {
                index += 1;
                version = args.get(index).map(String::as_str);
            }
            _ if arg.starts_with("--typelib-version=") => {
                version = arg.strip_prefix("--typelib-version=");
            }
            _ if !arg.starts_with('-') => {
                namespace = Some(arg.as_str());
            }
            _ => {}
        }
        index += 1;
    }

    let namespace = namespace.ok_or_else(|| "missing namespace".to_owned())?;
    let doc = parser::load_namespace(
        namespace,
        version,
        &typelib_search_dirs(None),
        &gir_search_dirs(None),
    )?;
    if print_shlibs {
        for shlib in &doc.shared_libraries {
            println!("shlib: {shlib}");
        }
    }
    if print_typelibs {
        for dependency in doc.dependency_names() {
            println!("typelib: {dependency}");
        }
    }
    if !print_shlibs && !print_typelibs {
        println!("namespace: {}-{}", doc.namespace, doc.version);
    }
    Ok(())
}
