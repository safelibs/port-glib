use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

fn emit_objects(object_dir: &Path) {
    let mut objects: Vec<_> = fs::read_dir(object_dir)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", object_dir.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension().and_then(|ext| ext.to_str()) == Some("o")).then_some(path)
        })
        .collect();
    objects.sort();
    if objects.is_empty() {
        panic!("no object files found in {}", object_dir.display());
    }

    for object in objects {
        println!("cargo:rerun-if-changed={}", object.display());
        emit_cdylib_arg(object.display().to_string());
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let object_dir = manifest_dir
        .join("../../vendor/build-check/gio/libgio-2.0.so.0.8000.0.p")
        .canonicalize()
        .expect("missing vendored gio object directory");
    let xdgmime = manifest_dir
        .join("../../vendor/build-check/gio/xdgmime/libxdgmime.a")
        .canonicalize()
        .expect("missing vendored xdgmime archive");
    let inotify = manifest_dir
        .join("../../vendor/build-check/gio/inotify/libinotify.a")
        .canonicalize()
        .expect("missing vendored inotify archive");
    println!("cargo:rerun-if-changed={}", object_dir.display());
    println!("cargo:rerun-if-changed={}", xdgmime.display());
    println!("cargo:rerun-if-changed={}", inotify.display());

    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");
    if let Ok(soname) = env::var("SAFE_LINK_SONAME") {
        emit_cdylib_arg(format!("-Wl,-soname,{soname}"));
    }
    if let Ok(version_script) = env::var("SAFE_LINK_VERSION_SCRIPT") {
        emit_cdylib_arg(format!("-Wl,--version-script={version_script}"));
    }
    emit_cdylib_arg("-Wl,--as-needed");
    emit_cdylib_arg("-Wl,--no-undefined");
    emit_cdylib_arg("-Wl,-z,nodelete");
    emit_cdylib_arg("-Wl,-Bsymbolic-functions");
    emit_objects(&object_dir);
    emit_cdylib_arg(xdgmime.display().to_string());
    emit_cdylib_arg(inotify.display().to_string());
    println!("cargo:rustc-link-lib=dylib=glib-2.0");
    println!("cargo:rustc-link-lib=dylib=gobject-2.0");
    println!("cargo:rustc-link-lib=dylib=gmodule-2.0");
    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-lib=dylib=mount");
    println!("cargo:rustc-link-lib=dylib=selinux");
}
