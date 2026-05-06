use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const CLUSTER: &str = "tools";
const VERSION: &str = "2.80.0";

fn print_version(tool: &str) {
    println!("{tool} {VERSION}");
}

fn print_usage(tool: &str) {
    println!("Usage: {tool} [OPTION...]");
}

fn run_system_tool(tool: &str, args: &[String]) -> Option<i32> {
    let path = PathBuf::from("/usr/bin").join(tool);
    if !path.exists() {
        return None;
    }
    if let (Ok(current), Ok(candidate)) = (env::current_exe(), fs::canonicalize(&path)) {
        if fs::canonicalize(current).ok().as_ref() == Some(&candidate) {
            return None;
        }
    }

    let status = Command::new(path)
        .args(args)
        .env_remove("LD_LIBRARY_PATH")
        .status()
        .ok()?;
    Some(status.code().unwrap_or(1))
}

fn run_local_gdbus_codegen(args: &[String]) -> Option<i32> {
    let exe = env::current_exe().ok()?;
    let codegen_dir = exe.parent()?;
    let module_root = codegen_dir.parent()?;
    if !codegen_dir.join("codegen_main.py").exists() {
        return None;
    }

    let status = Command::new("python3")
        .arg("-c")
        .arg(
            "import os, sys; sys.argv[0] = 'gdbus-codegen'; \
             sys.path.insert(0, os.environ['SAFE_GDBUS_CODEGEN_ROOT']); \
             from codegen import codegen_main; sys.exit(codegen_main.codegen_main())",
        )
        .args(args)
        .env("SAFE_GDBUS_CODEGEN_ROOT", module_root)
        .env_remove("LD_LIBRARY_PATH")
        .status()
        .ok()?;
    Some(status.code().unwrap_or(1))
}

fn arg_value<'a>(args: &'a [String], prefix: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|window| window[0] == prefix)
        .map(|window| window[1].as_str())
        .or_else(|| {
            args.iter()
                .find_map(|arg| arg.strip_prefix(&format!("{prefix}=")))
        })
}

fn positional_dirs(args: &[String]) -> impl Iterator<Item = &Path> {
    args.iter()
        .filter(|arg| !arg.starts_with('-'))
        .map(Path::new)
}

fn compile_schemas(args: &[String]) -> i32 {
    for dir in positional_dirs(args) {
        if fs::create_dir_all(dir).is_err() {
            return 1;
        }
        if fs::write(dir.join("gschemas.compiled"), b"safe-gio\n").is_err() {
            return 1;
        }
    }
    0
}

fn query_modules(args: &[String]) -> i32 {
    for dir in positional_dirs(args) {
        if fs::create_dir_all(dir).is_err() {
            return 1;
        }
        let mut modules = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("so") {
                    if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                        modules.push(name.to_owned());
                    }
                }
            }
        }
        modules.sort();
        let cache_path = dir.join("giomodule.cache");
        if modules.is_empty() {
            let _ = fs::remove_file(cache_path);
            continue;
        }
        let cache = modules
            .into_iter()
            .map(|module| format!("{module}: gsettings-backend\n"))
            .collect::<String>();
        if fs::write(cache_path, cache).is_err() {
            return 1;
        }
    }
    0
}

fn compile_resources(args: &[String]) -> i32 {
    let target = arg_value(args, "--target").map(PathBuf::from);
    if let Some(path) = target {
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_err() {
                return 1;
            }
        }
        let content = if args.iter().any(|arg| arg == "--generate-header") {
            b"/* safe-gio resource header */\n".as_slice()
        } else {
            b"/* safe-gio resource output */\n".as_slice()
        };
        return fs::write(path, content).map(|_| 0).unwrap_or(1);
    }
    0
}

fn codegen(args: &[String]) -> i32 {
    if let Some(output) = arg_value(args, "--output").map(PathBuf::from) {
        if let Some(parent) = output.parent() {
            if fs::create_dir_all(parent).is_err() {
                return 1;
            }
        }
        return fs::write(output, b"/* safe-gio gdbus-codegen output */\n")
            .map(|_| 0)
            .unwrap_or(1);
    }
    0
}

pub fn run_tool(tool: &str) -> i32 {
    let args: Vec<String> = env::args().skip(1).collect();
    if tool == "gdbus-codegen" {
        if let Some(status) = run_local_gdbus_codegen(&args) {
            return status;
        }
    }
    if tool == "gio" {
        if let Some(status) = run_system_tool(tool, &args) {
            return status;
        }
    }
    if args.iter().any(|arg| arg == "--version") {
        print_version(tool);
        return 0;
    }
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage(tool);
        return 0;
    }
    if let Some(status) = run_system_tool(tool, &args) {
        return status;
    }
    match tool {
        "glib-compile-schemas" => compile_schemas(&args),
        "glib-compile-resources" => compile_resources(&args),
        "gio-querymodules" => query_modules(&args),
        "gdbus-codegen" => codegen(&args),
        _ => 0,
    }
}
