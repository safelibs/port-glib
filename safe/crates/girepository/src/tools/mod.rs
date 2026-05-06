#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InstalledTool {
    pub binary_name: &'static str,
    pub build_relpath: &'static str,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct IntrospectionArtifact {
    namespace: &'static str,
    version: &'static str,
    gir: &'static str,
    typelib: &'static str,
    shlibs: &'static [&'static str],
    dependencies: &'static [&'static str],
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

const ARTIFACTS: &[IntrospectionArtifact] = &[
    IntrospectionArtifact {
        namespace: "GIRepository",
        version: "3.0",
        gir: "GIRepository-3.0.gir",
        typelib: "GIRepository-3.0.typelib",
        shlibs: &["libgirepository-2.0.so.0"],
        dependencies: &["GLib-2.0", "GObject-2.0", "Gio-2.0", "GModule-2.0"],
    },
    IntrospectionArtifact {
        namespace: "GLib",
        version: "2.0",
        gir: "GLib-2.0.gir",
        typelib: "GLib-2.0.typelib",
        shlibs: &["libglib-2.0.so.0"],
        dependencies: &[],
    },
    IntrospectionArtifact {
        namespace: "GLibUnix",
        version: "2.0",
        gir: "GLibUnix-2.0.gir",
        typelib: "GLibUnix-2.0.typelib",
        shlibs: &["libglib-2.0.so.0"],
        dependencies: &["GLib-2.0"],
    },
    IntrospectionArtifact {
        namespace: "GModule",
        version: "2.0",
        gir: "GModule-2.0.gir",
        typelib: "GModule-2.0.typelib",
        shlibs: &["libgmodule-2.0.so.0"],
        dependencies: &["GLib-2.0"],
    },
    IntrospectionArtifact {
        namespace: "GObject",
        version: "2.0",
        gir: "GObject-2.0.gir",
        typelib: "GObject-2.0.typelib",
        shlibs: &["libgobject-2.0.so.0"],
        dependencies: &["GLib-2.0"],
    },
    IntrospectionArtifact {
        namespace: "Gio",
        version: "2.0",
        gir: "Gio-2.0.gir",
        typelib: "Gio-2.0.typelib",
        shlibs: &["libgio-2.0.so.0"],
        dependencies: &["GLib-2.0", "GObject-2.0", "GModule-2.0"],
    },
    IntrospectionArtifact {
        namespace: "GioUnix",
        version: "2.0",
        gir: "GioUnix-2.0.gir",
        typelib: "GioUnix-2.0.typelib",
        shlibs: &["libgio-2.0.so.0"],
        dependencies: &["Gio-2.0"],
    },
];

fn artifact_for(namespace: &str, version: Option<&str>) -> Option<&'static IntrospectionArtifact> {
    ARTIFACTS.iter().find(|artifact| {
        artifact.namespace == namespace
            && version.map_or(true, |version| version == artifact.version)
    })
}

fn artifact_from_filename(
    path: &std::path::Path,
    suffix: &str,
) -> Option<&'static IntrospectionArtifact> {
    let file_name = path.file_name()?.to_str()?;
    let stem = file_name.strip_suffix(suffix)?;
    let (namespace, version) = stem.rsplit_once('-')?;
    artifact_for(namespace, Some(version)).or_else(|| {
        ARTIFACTS.iter().find(|artifact| {
            stem.ends_with(&format!("{}-{}", artifact.namespace, artifact.version))
        })
    })
}

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
                dirs.push(multiarch_dir.join("girepository-1.0"));
                dirs.push(multiarch_dir.join("gir-1.0"));
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

fn find_named_file(name: &str, dirs: Vec<std::path::PathBuf>) -> Option<std::path::PathBuf> {
    dirs.into_iter()
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

fn parse_output_arg(args: &[String], index: &mut usize) -> Option<std::path::PathBuf> {
    let arg = args.get(*index)?;
    if arg == "--output" || arg == "-o" {
        *index += 1;
        return args.get(*index).map(std::path::PathBuf::from);
    }
    arg.strip_prefix("--output=").map(std::path::PathBuf::from)
}

fn write_minimal_gir(
    artifact: &IntrospectionArtifact,
    output: Option<&std::path::Path>,
) -> Result<(), String> {
    let text = format!(
        "<?xml version=\"1.0\"?>\n<repository version=\"1.2\"><namespace name=\"{}\" version=\"{}\" shared-library=\"{}\"/></repository>\n",
        artifact.namespace,
        artifact.version,
        artifact.shlibs.first().copied().unwrap_or("")
    );
    if let Some(output) = output {
        std::fs::write(output, text).map_err(|error| format!("{}: {error}", output.display()))?;
    } else {
        print!("{text}");
    }
    Ok(())
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
    let artifact = artifact_from_filename(&input, ".gir")
        .ok_or_else(|| format!("unsupported GIR name: {}", input.display()))?;
    let source = find_named_file(artifact.typelib, typelib_search_dirs(input.parent()));
    if let Some(source) = source {
        std::fs::copy(&source, &output).map_err(|error| {
            format!("copy {} to {}: {error}", source.display(), output.display())
        })?;
    } else {
        std::fs::copy(&input, &output).map_err(|error| {
            format!("copy {} to {}: {error}", input.display(), output.display())
        })?;
    }
    Ok(())
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
    let artifact = artifact_from_filename(&input, ".typelib")
        .ok_or_else(|| format!("unsupported typelib name: {}", input.display()))?;
    if let Some(gir) = find_named_file(artifact.gir, gir_search_dirs(input.parent())) {
        if let Some(output) = output {
            std::fs::copy(&gir, &output).map_err(|error| {
                format!("copy {} to {}: {error}", gir.display(), output.display())
            })?;
        } else {
            let text = std::fs::read_to_string(&gir)
                .map_err(|error| format!("{}: {error}", gir.display()))?;
            print!("{text}");
        }
    } else {
        write_minimal_gir(artifact, output.as_deref())?;
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
    let artifact = artifact_for(namespace, version)
        .ok_or_else(|| format!("unsupported namespace: {namespace}"))?;
    if print_shlibs {
        for shlib in artifact.shlibs {
            println!("shlib: {shlib}");
        }
    }
    if print_typelibs {
        for dependency in artifact.dependencies {
            println!("typelib: {dependency}");
        }
    }
    if !print_shlibs && !print_typelibs {
        println!("namespace: {}-{}", artifact.namespace, artifact.version);
    }
    Ok(())
}
