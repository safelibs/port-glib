#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
image="${TEST_ORIGINAL_IMAGE:-ubuntu:24.04}"

if ! command -v docker >/dev/null 2>&1; then
  printf 'docker is required to run %s\n' "$0" >&2
  exit 1
fi

docker run --rm --pull=missing -i \
  --workdir /tmp/port-glib \
  -e DEBIAN_FRONTEND=noninteractive \
  -e GLIB_UNDER_TEST="${GLIB_UNDER_TEST:-safe}" \
  -e GLIB_TEST_SCOPE="${GLIB_TEST_SCOPE:-all}" \
  -e GLIB_PACKAGE_BUILD_JOBS="${GLIB_PACKAGE_BUILD_JOBS:-1}" \
  -e SAFELIBS_RUST_TOOLCHAIN="${SAFELIBS_RUST_TOOLCHAIN:-}" \
  -v "$repo_root:/src:ro" \
  "$image" \
  bash -s <<'EOF'
set -euo pipefail

SRC_ROOT=/src
MANIFEST="$SRC_ROOT/dependents.json"
WORK_ROOT=/tmp/port-glib
LOG_ROOT="$WORK_ROOT/logs"
GLIB_PREFIX=/opt/glib-original
SAFE_SOURCE="$WORK_ROOT/safe"
SAFE_PACKAGE_ROOT="$WORK_ROOT/safe-packages"
SAFE_EXTRACT_ROOT="$WORK_ROOT/safe-extract"
GLIB_UNDER_TEST="${GLIB_UNDER_TEST:-safe}"
GLIB_TEST_SCOPE="${GLIB_TEST_SCOPE:-all}"
GLIB_PACKAGE_BUILD_JOBS="${GLIB_PACKAGE_BUILD_JOBS:-1}"

mkdir -p "$WORK_ROOT" "$LOG_ROOT"

multiarch=""
glib_libdir=""
safe_package_profile="__unbuilt__"
safe_libglib_sha256=""

log() {
  printf '\n==> %s\n' "$*"
}

die() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

run_logged() {
  local name=$1
  shift
  local log_file="$LOG_ROOT/$name.log"

  if ! "$@" >"$log_file" 2>&1; then
    printf 'Command failed (%s): %s\n' "$name" "$*" >&2
    tail -n 240 "$log_file" >&2
    exit 1
  fi
}

run_logged_in() {
  local name=$1
  local cwd=$2
  shift 2
  local log_file="$LOG_ROOT/$name.log"

  if ! (cd "$cwd" && "$@") >"$log_file" 2>&1; then
    printf 'Command failed (%s in %s): %s\n' "$name" "$cwd" "$*" >&2
    tail -n 240 "$log_file" >&2
    exit 1
  fi
}

set_glib_env() {
  export PATH="$GLIB_PREFIX/bin:$PATH"
  export LD_LIBRARY_PATH="$glib_libdir:$GLIB_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  export PKG_CONFIG_PATH="$glib_libdir/pkgconfig:$GLIB_PREFIX/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
  export ACLOCAL_PATH="$GLIB_PREFIX/share/aclocal${ACLOCAL_PATH:+:$ACLOCAL_PATH}"
  export GIO_MODULE_DIR="/usr/lib/$multiarch/gio/modules"
}

ldd_resolved_library() {
  local binary=$1
  local library=$2
  local resolved=$binary
  local line
  local path

  if [[ $resolved != /* ]]; then
    resolved="$(command -v "$resolved")"
  fi

  line="$(ldd "$resolved" | awk -v lib="$library" '$1 == lib {print; exit}')"
  [[ -n $line ]] || die "$resolved is not linked to $library"
  path="$(awk '{print $3}' <<<"$line")"
  [[ -n $path && $path != "not" ]] || die "$resolved did not resolve $library: $line"
  readlink -f "$path"
}

assert_binary_uses_test_glib() {
  local binary=$1
  local lib_path
  local lib_sha

  lib_path="$(ldd_resolved_library "$binary" "libglib-2.0.so.0")"

  case "$GLIB_UNDER_TEST" in
    original)
      grep -F "$glib_libdir/libglib-2.0.so.0" <<<"$lib_path" >/dev/null \
        || die "$binary is not loading libglib-2.0 from $glib_libdir"
      ;;
    safe)
      [[ $lib_path == /usr/lib/"$multiarch"/libglib-2.0.so.0.8000.0 ]] \
        || die "$binary resolved libglib-2.0 outside the safe package path: $lib_path"
      dpkg-query -S "$lib_path" | grep -q '^libglib2.0-0t64:' \
        || die "$lib_path is not owned by libglib2.0-0t64"
      lib_sha="$(sha256sum "$lib_path" | awk '{print $1}')"
      [[ -n $safe_libglib_sha256 && $lib_sha == "$safe_libglib_sha256" ]] \
        || die "$lib_path does not match the libglib built from safe/"
      ;;
    *)
      die "unsupported GLIB_UNDER_TEST=$GLIB_UNDER_TEST"
      ;;
  esac
}

assert_installed_safe_libglib() {
  local lib_path="/usr/lib/$multiarch/libglib-2.0.so.0.8000.0"
  local lib_sha

  [[ -f $lib_path ]] || die "installed safe libglib is missing: $lib_path"
  dpkg-query -S "$lib_path" | grep -q '^libglib2.0-0t64:' \
    || die "$lib_path is not owned by libglib2.0-0t64"
  lib_sha="$(sha256sum "$lib_path" | awk '{print $1}')"
  [[ -n $safe_libglib_sha256 && $lib_sha == "$safe_libglib_sha256" ]] \
    || die "$lib_path does not match the libglib built from safe/"
}

verify_manifest() {
  local expected_packages
  local manifest_packages

  expected_packages="$WORK_ROOT/expected-packages.txt"
  manifest_packages="$WORK_ROOT/manifest-packages.txt"

  cat >"$expected_packages" <<'LIST'
bluez
flatpak
fwupd
gstreamer1.0-tools
gvfs-daemons
libvirt-daemon
modemmanager
network-manager
pocillo-icon-theme
qemu-system-x86
tracker-miner-fs
udisks2
LIST

  jq -r '.dependents[].binary_package' "$MANIFEST" | sort -u >"$manifest_packages"

  if ! diff -u "$expected_packages" "$manifest_packages" >/dev/null; then
    diff -u "$expected_packages" "$manifest_packages" >&2 || true
    die "dependents.json does not match the supported dependent set"
  fi

  jq -e '.dependents[] | select(.binary_package == "pocillo-icon-theme" and .source_package == "budgie-artwork" and .glib_dependency_kind == "compile_time_only")' "$MANIFEST" >/dev/null \
    || die "dependents.json no longer describes pocillo-icon-theme as budgie-artwork compile-time-only coverage"
}

enable_source_repos() {
  sed -i 's/^Types: deb$/Types: deb deb-src/' /etc/apt/sources.list.d/ubuntu.sources
}

prepare_apt() {
  log "Updating apt metadata"
  run_logged apt-update apt-get update

  log "Installing container bootstrap tools"
  run_logged apt-bootstrap apt-get install -y --no-install-recommends \
    jq \
    python3 \
    dbus \
    dbus-user-session \
    dpkg-dev \
    devscripts \
    equivs \
    fakeroot \
    build-essential \
    ca-certificates \
    curl \
    xz-utils
}

build_original_glib() {
  local source_dir="$WORK_ROOT/original"
  local build_dir="$WORK_ROOT/original-build"

  log "Installing GLib build dependencies"
  run_logged apt-build-deps-glib apt-get build-dep -y glib2.0

  log "Copying original GLib source tree"
  rm -rf "$source_dir" "$build_dir"
  cp -a "$SRC_ROOT/original" "$source_dir"

  log "Building and installing original GLib"
  run_logged build-original-glib bash -lc "
    set -euo pipefail
    meson setup '$build_dir' '$source_dir' \
      --prefix='$GLIB_PREFIX' \
      -Dtests=false \
      -Dinstalled_tests=false \
      -Ddocumentation=false \
      -Dman-pages=disabled \
      -Dintrospection=disabled \
      -Ddtrace=false \
      -Dsystemtap=false \
      -Dsysprof=disabled
    meson compile -C '$build_dir'
    meson install -C '$build_dir'
  "

  multiarch="$(dpkg-architecture -qDEB_HOST_MULTIARCH)"
  glib_libdir="$GLIB_PREFIX/lib/$multiarch"
  [[ -d $glib_libdir ]] || glib_libdir="$GLIB_PREFIX/lib"
}

copy_safe_source() {
  log "Copying safe source tree without ignored build artifacts"
  rm -rf "$SAFE_SOURCE"
  run_logged copy-safe-source bash -lc "
    set -euo pipefail
    tar -C '$SRC_ROOT' \
      --exclude='safe/build-*' \
      --exclude='safe/package-baseline-*' \
      --exclude='safe/target' \
      --exclude='safe/debian/.debhelper' \
      --exclude='safe/debian/build' \
      --exclude='safe/debian/tmp' \
      --exclude='safe/debian/cross-tools' \
      --exclude='safe/debian/gir1.2-girepository-3.0' \
      --exclude='safe/debian/gir1.2-girepository-3.0-dev' \
      --exclude='safe/debian/gir1.2-glib-2.0' \
      --exclude='safe/debian/gir1.2-glib-2.0-dev' \
      --exclude='safe/debian/libgirepository-2.0-0' \
      --exclude='safe/debian/libgirepository-2.0-dev' \
      --exclude='safe/debian/libglib2.0-0t64' \
      --exclude='safe/debian/libglib2.0-bin' \
      --exclude='safe/debian/libglib2.0-data' \
      --exclude='safe/debian/libglib2.0-dev' \
      --exclude='safe/debian/libglib2.0-dev-bin' \
      --exclude='safe/debian/libglib2.0-tests' \
      --exclude='safe/debian/*.debhelper.log' \
      --exclude='safe/debian/*.substvars' \
      --exclude='safe/debian/*.postinst.debhelper' \
      --exclude='safe/debian/*.prerm.debhelper' \
      --exclude='safe/debian/debhelper-build-stamp' \
      --exclude='safe/debian/files' \
      -cf - safe \
      | tar -C '$WORK_ROOT' -xf -
  "
  [[ -f $SAFE_SOURCE/debian/control ]] || die "safe source copy did not include debian packaging"
  [[ -d $SAFE_SOURCE/vendor/original ]] || die "safe source copy did not include vendored original GLib assets"
  [[ -d $SAFE_SOURCE/vendor/build-check ]] || die "safe source copy did not include prepared ABI build-check assets"
  [[ ! -e $SAFE_SOURCE/target ]] || die "safe source copy unexpectedly included Cargo target artifacts"
  [[ ! -e $SAFE_SOURCE/debian/build ]] || die "safe source copy unexpectedly included Debian build artifacts"
}

install_safe_build_dependencies() {
  local profiles=$1

  log "Installing safe package build dependencies"
  run_logged apt-build-deps-safe bash -lc "
    set -euo pipefail
    cd '$SAFE_SOURCE'
    env DEB_BUILD_PROFILES='$profiles' \
      mk-build-deps -i -r -t 'apt-get -y --no-install-recommends' debian/control
    env DEB_BUILD_PROFILES='$profiles' dpkg-checkbuilddeps debian/control
  "
}

safe_rust_toolchain() {
  local toolchain

  toolchain="${SAFELIBS_RUST_TOOLCHAIN:-}"
  if [[ -z $toolchain && -f "$SAFE_SOURCE/rust-toolchain.toml" ]]; then
    toolchain="$(grep -oP '^channel\s*=\s*"\K[^"]+' "$SAFE_SOURCE/rust-toolchain.toml" || true)"
  fi
  printf '%s\n' "${toolchain:-stable}"
}

install_safe_rust_toolchain() {
  local toolchain

  toolchain="$(safe_rust_toolchain)"
  log "Installing Rust toolchain $toolchain"
  run_logged rustup-install bash -lc "
    set -euo pipefail
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain '$toolchain' --no-modify-path
    . \"\$HOME/.cargo/env\"
    rustup default '$toolchain'
    rustc --version
    cargo --version
  "
  # shellcheck source=/dev/null
  . "$HOME/.cargo/env"
  export PATH="$HOME/.cargo/bin:$PATH"
}

build_safe_packages() {
  local profiles=$1

  if [[ $safe_package_profile == "$profiles" ]]; then
    return
  fi

  copy_safe_source
  install_safe_build_dependencies "$profiles"
  install_safe_rust_toolchain

  log "Building safe Debian packages"
  rm -rf "$SAFE_PACKAGE_ROOT" "$SAFE_EXTRACT_ROOT"
  mkdir -p "$SAFE_PACKAGE_ROOT" "$SAFE_EXTRACT_ROOT"
  run_logged_in build-safe-packages "$SAFE_SOURCE" env \
    CARGO_BUILD_JOBS="$GLIB_PACKAGE_BUILD_JOBS" \
    CARGO_INCREMENTAL=0 \
    DEB_BUILD_OPTIONS="nocheck parallel=$GLIB_PACKAGE_BUILD_JOBS" \
    DEB_BUILD_PROFILES="$profiles" \
    MAKEFLAGS="-j$GLIB_PACKAGE_BUILD_JOBS" \
    NINJAFLAGS="-j$GLIB_PACKAGE_BUILD_JOBS" \
    SAFE_FULL_PACKAGE_BUILD=1 \
    dpkg-buildpackage -us -uc -b
  find "$WORK_ROOT" -maxdepth 1 -type f \( -name '*.deb' -o -name '*.udeb' \) -exec mv -f {} "$SAFE_PACKAGE_ROOT/" \;
  find "$SAFE_PACKAGE_ROOT" -maxdepth 1 -type f -name '*.deb' | grep -q . \
    || die "safe package build did not produce any .deb files"
  safe_package_profile="$profiles"
}

install_safe_packages() {
  local profiles=$1
  local runtime_deb
  local extracted_lib

  build_safe_packages "$profiles"

  log "Installing safe Debian packages"
  run_logged apt-install-safe apt-get install -y --allow-downgrades --no-install-recommends "$SAFE_PACKAGE_ROOT"/*.deb

  multiarch="$(dpkg-architecture -qDEB_HOST_MULTIARCH)"
  runtime_deb="$(find "$SAFE_PACKAGE_ROOT" -maxdepth 1 -type f -name 'libglib2.0-0t64_*.deb' | head -n 1)"
  [[ -n $runtime_deb ]] || die "safe runtime package was not built"
  rm -rf "$SAFE_EXTRACT_ROOT/libglib2.0-0t64"
  mkdir -p "$SAFE_EXTRACT_ROOT/libglib2.0-0t64"
  dpkg-deb -x "$runtime_deb" "$SAFE_EXTRACT_ROOT/libglib2.0-0t64"
  extracted_lib="$SAFE_EXTRACT_ROOT/libglib2.0-0t64/usr/lib/$multiarch/libglib-2.0.so.0.8000.0"
  [[ -f $extracted_lib ]] || die "safe runtime package does not contain $extracted_lib"
  safe_libglib_sha256="$(sha256sum "$extracted_lib" | awk '{print $1}')"

  case "$GLIB_UNDER_TEST" in
    safe)
      assert_installed_safe_libglib
      ;;
    original)
      assert_binary_uses_test_glib /usr/bin/gio
      ;;
  esac
}

install_dependent_runtime_packages() {
  local runtime_packages=()

  mapfile -t runtime_packages < <(
    jq -r '.dependents[] | select(.glib_dependency_kind == "compile_time_and_runtime") | .binary_package' "$MANIFEST"
  )

  log "Installing runtime dependent packages"
  run_logged apt-runtime-install apt-get install -y --no-install-recommends \
    "${runtime_packages[@]}" \
    libvirt-clients \
    ostree

  case "$GLIB_UNDER_TEST" in
    safe)
      assert_installed_safe_libglib
      ;;
    original)
      ;;
    *)
      die "unsupported GLIB_UNDER_TEST=$GLIB_UNDER_TEST"
      ;;
  esac
}

test_qemu() {
  assert_binary_uses_test_glib qemu-system-x86_64

  python3 <<'PY'
import subprocess

p = subprocess.Popen(
    ["qemu-system-x86_64", "-display", "none", "-machine", "none", "-nodefaults", "-qmp", "stdio"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
)

try:
    banner = p.stdout.readline()
    if '"QMP"' not in banner:
        raise SystemExit(f"unexpected QMP banner: {banner!r}")
    p.stdin.write('{"execute":"qmp_capabilities"}\n')
    p.stdin.flush()
    if '"return"' not in p.stdout.readline():
        raise SystemExit("qmp_capabilities did not succeed")
    p.stdin.write('{"execute":"quit"}\n')
    p.stdin.flush()
    if '"SHUTDOWN"' not in p.stdout.readline():
        raise SystemExit("QEMU did not emit the expected shutdown event")
    rc = p.wait(timeout=10)
    if rc != 0:
        raise SystemExit(f"unexpected QEMU exit status {rc}")
finally:
    if p.poll() is None:
        p.kill()
PY
}

test_network_manager() {
  local output="$WORK_ROOT/network-manager.out"

  assert_binary_uses_test_glib /usr/sbin/NetworkManager
  assert_binary_uses_test_glib nmcli

  dbus-run-session -- bash <<SH
set -euo pipefail
export DBUS_SYSTEM_BUS_ADDRESS="\$DBUS_SESSION_BUS_ADDRESS"
mkdir -p /run/NetworkManager
/usr/sbin/NetworkManager --no-daemon --log-level=OFF >"$WORK_ROOT/network-manager-daemon.log" 2>&1 &
nm_pid=\$!
cleanup() {
  kill "\$nm_pid" >/dev/null 2>&1 || true
  wait "\$nm_pid" || true
}
trap cleanup EXIT
for _ in \$(seq 1 40); do
  if nmcli -t -f RUNNING general >"$output" 2>"$WORK_ROOT/network-manager.err"; then
    grep -qx 'running' "$output"
    exit 0
  fi
  sleep 0.5
done
cat "$WORK_ROOT/network-manager-daemon.log" >&2 || true
cat "$WORK_ROOT/network-manager.err" >&2 || true
exit 1
SH
}

test_bluez() {
  local log_file="$WORK_ROOT/bluetoothd.log"

  assert_binary_uses_test_glib /usr/sbin/bluetoothd

  dbus-run-session -- bash <<SH
set -euo pipefail
export DBUS_SYSTEM_BUS_ADDRESS="\$DBUS_SESSION_BUS_ADDRESS"
/usr/sbin/bluetoothd --nodetach --debug >"$log_file" 2>&1 || true
grep -q 'Bluetooth daemon' "$log_file"
grep -q 'Adapter handling initialization failed' "$log_file"
SH
}

test_flatpak() {
  local home_dir="$WORK_ROOT/flatpak-home"
  local runtime_dir="$WORK_ROOT/flatpak-runtime"
  local repo_dir="$WORK_ROOT/flatpak-repo"
  local remotes_file="$WORK_ROOT/flatpak-remote-ls.txt"

  assert_binary_uses_test_glib flatpak

  rm -rf "$home_dir" "$runtime_dir" "$repo_dir"
  mkdir -p "$home_dir" "$runtime_dir" "$repo_dir"

  ostree --repo="$repo_dir" init --mode=archive-z2 >/dev/null
  flatpak build-update-repo "$repo_dir" >"$WORK_ROOT/flatpak-build-update-repo.out" 2>"$WORK_ROOT/flatpak-build-update-repo.err"
  [[ -f "$repo_dir/summary" ]] || die "flatpak repo summary was not generated"

  HOME="$home_dir" XDG_RUNTIME_DIR="$runtime_dir" \
    flatpak remote-add --user --if-not-exists --no-gpg-verify local "file://$repo_dir" >/dev/null 2>"$WORK_ROOT/flatpak-remote-add.err"
  HOME="$home_dir" XDG_RUNTIME_DIR="$runtime_dir" \
    flatpak remote-ls --user local >"$remotes_file" 2>"$WORK_ROOT/flatpak-remote-ls.err"
}

test_modemmanager() {
  local output="$WORK_ROOT/modemmanager.out"

  assert_binary_uses_test_glib /usr/sbin/ModemManager
  assert_binary_uses_test_glib mmcli

  dbus-run-session -- bash <<SH
set -euo pipefail
/usr/sbin/ModemManager --test-session --test-no-udev --test-no-qrtr --no-auto-scan >"$WORK_ROOT/modemmanager-daemon.log" 2>&1 &
mm_pid=\$!
cleanup() {
  kill "\$mm_pid" >/dev/null 2>&1 || true
  wait "\$mm_pid" || true
}
trap cleanup EXIT
for _ in \$(seq 1 20); do
  if mmcli --test-session --list-modems >"$output" 2>"$WORK_ROOT/modemmanager.err"; then
    grep -q 'No modems were found' "$output"
    exit 0
  fi
  sleep 0.5
done
cat "$WORK_ROOT/modemmanager-daemon.log" >&2 || true
cat "$WORK_ROOT/modemmanager.err" >&2 || true
exit 1
SH
}

test_fwupd() {
  local output="$WORK_ROOT/fwupd-remotes.out"

  assert_binary_uses_test_glib /usr/libexec/fwupd/fwupd
  assert_binary_uses_test_glib fwupdmgr

  dbus-run-session -- bash <<SH
set -euo pipefail
export DBUS_SYSTEM_BUS_ADDRESS="\$DBUS_SESSION_BUS_ADDRESS"
/usr/libexec/fwupd/fwupd >"$WORK_ROOT/fwupd-daemon.log" 2>&1 &
fwupd_pid=\$!
cleanup() {
  kill "\$fwupd_pid" >/dev/null 2>&1 || true
  wait "\$fwupd_pid" || true
}
trap cleanup EXIT
for _ in \$(seq 1 40); do
  if fwupdmgr get-remotes >"$output" 2>"$WORK_ROOT/fwupd-remotes.err"; then
    grep -Eq 'Remote ID:[[:space:]]+lvfs' "$output"
    exit 0
  fi
  sleep 0.5
done
cat "$WORK_ROOT/fwupd-daemon.log" >&2 || true
cat "$WORK_ROOT/fwupd-remotes.err" >&2 || true
exit 1
SH
}

test_gvfs_daemons() {
  local output="$WORK_ROOT/gvfs-call.out"

  assert_binary_uses_test_glib /usr/libexec/gvfsd
  assert_binary_uses_test_glib /usr/libexec/gvfs-udisks2-volume-monitor

  if ! dbus-run-session -- bash >"$WORK_ROOT/gvfs-session.out" 2>"$WORK_ROOT/gvfs-session.err" <<SH
set -euo pipefail
export XDG_RUNTIME_DIR="$WORK_ROOT/gvfs-runtime"
mkdir -p "\$XDG_RUNTIME_DIR"
gdbus call --session \
  --dest org.gtk.vfs.Daemon \
  --object-path /org/gtk/vfs/Daemon \
  --method org.gtk.vfs.Daemon.ListMonitorImplementations >"$output" 2>"$WORK_ROOT/gvfs-call.err"
SH
  then
    cat "$WORK_ROOT/gvfs-session.err" >&2 || true
    cat "$WORK_ROOT/gvfs-call.err" >&2 || true
    cat "$output" >&2 || true
    return 1
  fi
}

test_gstreamer_tools() {
  assert_binary_uses_test_glib gst-launch-1.0
  gst-launch-1.0 -q fakesrc num-buffers=4 ! fakesink
}

test_libvirt_daemon() {
  local output="$WORK_ROOT/libvirt.out"

  assert_binary_uses_test_glib /usr/sbin/libvirtd
  assert_binary_uses_test_glib virsh

  getent group libvirt-qemu >/dev/null || groupadd --system libvirt-qemu
  getent passwd libvirt-qemu >/dev/null || useradd --system --gid libvirt-qemu --home-dir /var/lib/libvirt/qemu --shell /usr/sbin/nologin libvirt-qemu
  mkdir -p /run/libvirt /var/log/libvirt /var/lib/libvirt/qemu

  dbus-run-session -- bash <<SH
set -euo pipefail
export DBUS_SYSTEM_BUS_ADDRESS="\$DBUS_SESSION_BUS_ADDRESS"
/usr/sbin/libvirtd --timeout 20 --daemon --pid-file /run/libvirtd.pid
cleanup() {
  if [[ -f /run/libvirtd.pid ]]; then
    kill "\$(cat /run/libvirtd.pid)" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT
for _ in \$(seq 1 40); do
  if virsh -c qemu:///system list --all >"$output" 2>"$WORK_ROOT/libvirt.err"; then
    grep -q '^ Id' "$output"
    exit 0
  fi
  sleep 0.5
done
cat "$WORK_ROOT/libvirt.err" >&2 || true
cat /var/log/libvirt/libvirtd.log >&2 || true
exit 1
SH
}

test_udisks2() {
  local output="$WORK_ROOT/udisks.out"

  assert_binary_uses_test_glib /usr/libexec/udisks2/udisksd
  assert_binary_uses_test_glib udisksctl

  dbus-run-session -- bash <<SH
set -euo pipefail
export DBUS_SYSTEM_BUS_ADDRESS="\$DBUS_SESSION_BUS_ADDRESS"
/usr/libexec/udisks2/udisksd --replace --uninstalled >"$WORK_ROOT/udisksd.log" 2>&1 &
udisks_pid=\$!
cleanup() {
  kill "\$udisks_pid" >/dev/null 2>&1 || true
  wait "\$udisks_pid" || true
}
trap cleanup EXIT
for _ in \$(seq 1 20); do
  if udisksctl status >"$output" 2>"$WORK_ROOT/udisks.err"; then
    grep -q '^MODEL' "$output"
    exit 0
  fi
  sleep 0.5
done
cat "$WORK_ROOT/udisksd.log" >&2 || true
cat "$WORK_ROOT/udisks.err" >&2 || true
exit 1
SH
}

test_tracker_miner_fs() {
  local output="$WORK_ROOT/tracker.out"

  assert_binary_uses_test_glib tracker3
  assert_binary_uses_test_glib /usr/libexec/tracker-miner-fs-3

  if ! dbus-run-session -- bash >"$WORK_ROOT/tracker-session.out" 2>"$WORK_ROOT/tracker-session.err" <<SH
set -euo pipefail
export HOME="$WORK_ROOT/tracker-home"
export XDG_RUNTIME_DIR="$WORK_ROOT/tracker-runtime"
export XDG_CACHE_HOME="$WORK_ROOT/tracker-cache"
export XDG_DATA_HOME="$WORK_ROOT/tracker-data"
mkdir -p "\$HOME" "\$XDG_RUNTIME_DIR" "\$XDG_CACHE_HOME" "\$XDG_DATA_HOME"
gdbus call --session \
  --dest org.freedesktop.Tracker3.Miner.Files \
  --object-path /org/freedesktop/Tracker3/Miner/Files \
  --method org.freedesktop.DBus.Peer.Ping >"$WORK_ROOT/tracker-ping.out" 2>"$WORK_ROOT/tracker-ping.err"
tracker3 daemon --list-miners-running >"$output" 2>"$WORK_ROOT/tracker.err"
SH
  then
    cat "$WORK_ROOT/tracker-session.err" >&2 || true
    cat "$WORK_ROOT/tracker-ping.err" >&2 || true
    cat "$WORK_ROOT/tracker.err" >&2 || true
    cat "$output" >&2 || true
    return 1
  fi
}

build_pocillo_icon_theme() {
  local source_root="$WORK_ROOT/deb-src"
  local package_dir
  local tool
  local resolved
  local owner

  case "$GLIB_UNDER_TEST" in
    original)
      set_glib_env
      [[ "$(readlink -f "$(command -v glib-compile-resources)")" == "$GLIB_PREFIX/bin/glib-compile-resources" ]] \
        || die "budgie-artwork build is not using glib-compile-resources from $GLIB_PREFIX"
      [[ "$(readlink -f "$(command -v glib-compile-schemas)")" == "$GLIB_PREFIX/bin/glib-compile-schemas" ]] \
        || die "budgie-artwork build is not using glib-compile-schemas from $GLIB_PREFIX"
      ;;
    safe)
      for tool in glib-compile-resources glib-compile-schemas; do
        resolved="$(command -v "$tool")"
        [[ $resolved == /usr/bin/"$tool" ]] \
          || die "budgie-artwork build would resolve $tool from $resolved instead of /usr/bin"
        case "$(readlink -f "$resolved")" in
          /src/*|"$WORK_ROOT"/*)
            die "$tool resolved into the source/build tree: $(readlink -f "$resolved")"
            ;;
        esac
        owner="$(dpkg-query -S "$resolved" | cut -d: -f1 | head -n 1)"
        case "$tool:$owner" in
          glib-compile-resources:libglib2.0-dev-bin|glib-compile-schemas:libglib2.0-bin)
            ;;
          *)
            die "$tool is not owned by the expected safe GLib package: $owner"
            ;;
        esac
      done
      ;;
    *)
      die "unsupported GLIB_UNDER_TEST=$GLIB_UNDER_TEST"
      ;;
  esac

  log "Installing build dependencies for budgie-artwork"
  run_logged apt-build-deps-budgie-artwork apt-get build-dep -y budgie-artwork

  rm -rf "$source_root"
  mkdir -p "$source_root"

  log "Fetching budgie-artwork source package"
  run_logged source-budgie-artwork bash -lc "
    set -euo pipefail
    cd '$source_root'
    apt-get source budgie-artwork
  "

  package_dir="$(find "$source_root" -mindepth 1 -maxdepth 1 -type d -name 'budgie-artwork-*' | head -n 1)"
  [[ -n $package_dir ]] || die "failed to locate extracted budgie-artwork source tree"

  log "Building budgie-artwork to cover pocillo-icon-theme"
  run_logged build-budgie-artwork bash -lc "
    set -euo pipefail
    cd '$package_dir'
    dpkg-buildpackage -b -uc -us
  "

  ls "$source_root"/pocillo-icon-theme_*.deb >/dev/null 2>&1 \
    || die "budgie-artwork build did not produce a pocillo-icon-theme .deb"
}

run_manifest_entry() {
  local package=$1

  case "$package" in
    qemu-system-x86)
      log "Testing $package"
      test_qemu
      ;;
    network-manager)
      log "Testing $package"
      test_network_manager
      ;;
    bluez)
      log "Testing $package"
      test_bluez
      ;;
    flatpak)
      log "Testing $package"
      test_flatpak
      ;;
    modemmanager)
      log "Testing $package"
      test_modemmanager
      ;;
    fwupd)
      log "Testing $package"
      test_fwupd
      ;;
    gvfs-daemons)
      log "Testing $package"
      test_gvfs_daemons
      ;;
    gstreamer1.0-tools)
      log "Testing $package"
      test_gstreamer_tools
      ;;
    libvirt-daemon)
      log "Testing $package"
      test_libvirt_daemon
      ;;
    udisks2)
      log "Testing $package"
      test_udisks2
      ;;
    tracker-miner-fs)
      log "Testing $package"
      test_tracker_miner_fs
      ;;
    pocillo-icon-theme)
      log "Building coverage for $package"
      build_pocillo_icon_theme
      ;;
    *)
      die "unsupported dependent entry: $package"
      ;;
  esac
}

setup_original_dependents() {
  build_original_glib
  set_glib_env
  install_dependent_runtime_packages
}

setup_safe_packages() {
  local profiles=$1

  install_safe_packages "$profiles"
}

run_package_smoke_scope() {
  [[ $GLIB_UNDER_TEST == safe ]] \
    || die "package-smoke scope is only supported with GLIB_UNDER_TEST=safe"

  setup_safe_packages "nodoc noudeb"

  log "Running package smoke test: debian/tests/build"
  run_logged_in package-build "$SAFE_SOURCE" env AUTOPKGTEST_TMP="$WORK_ROOT/autopkgtest-build" \
    debian/tests/build

  log "Running package smoke test: debian/tests/build-static"
  run_logged_in package-build-static "$SAFE_SOURCE" env AUTOPKGTEST_TMP="$WORK_ROOT/autopkgtest-build-static" \
    debian/tests/build-static

  log "Running package smoke test: girepository compile-only"
  run_logged_in package-girepository-compile "$SAFE_SOURCE" \
    sh tests/package/girepository-compile-only.sh

  log "Running package smoke test: girepository installed"
  run_logged_in package-girepository-installed "$SAFE_SOURCE" \
    sh tests/package/girepository-installed.sh
}

install_debian_test_dependencies() {
  log "Installing Debian autopkgtest dependencies"
  run_logged apt-debian-test-deps apt-get install -y --no-install-recommends \
    dbus-daemon \
    dbus-x11 \
    gnome-desktop-testing \
    locales \
    xauth \
    xvfb \
    dconf-gsettings-backend \
    dpkg-repack \
    gsettings-desktop-schemas
}

run_debian_test_script() {
  local test_name=$1
  local tmp="$WORK_ROOT/autopkgtest-$test_name"

  rm -rf "$tmp"
  mkdir -p "$tmp"
  chmod 700 "$tmp"

  log "Running Debian autopkgtest: $test_name"
  run_logged_in "debian-test-$test_name" "$SAFE_SOURCE" env \
    AUTOPKGTEST_TMP="$tmp" \
    DEBIAN_FRONTEND=noninteractive \
    "debian/tests/$test_name"
}

run_debian_tests_scope() {
  local test_name
  local tests=(
    installed-tests
    closure-refcount
    debugcontroller
    gdbus-server-auth
    gdbus-threading
    gmenumodel
    mainloop
    memory-monitor-dbus
    socket
    testfilemonitor
    thread-pool-slow
    threadtests
    timeout
    timer
    1065022-futureproofing
  )

  [[ $GLIB_UNDER_TEST == safe ]] \
    || die "debian-tests scope is only supported with GLIB_UNDER_TEST=safe"

  setup_safe_packages "nodoc noudeb"
  install_debian_test_dependencies

  for test_name in "${tests[@]}"; do
    run_debian_test_script "$test_name"
  done
}

run_dependents_scope() {
  case "$GLIB_UNDER_TEST" in
    original)
      setup_original_dependents
      ;;
    safe)
      setup_safe_packages "nodoc noinsttest nogir noudeb"
      install_dependent_runtime_packages
      ;;
    *)
      die "unsupported GLIB_UNDER_TEST=$GLIB_UNDER_TEST"
      ;;
  esac

  while IFS= read -r package; do
    run_manifest_entry "$package"
  done < <(jq -r '.dependents[].binary_package' "$MANIFEST")
}

run_scope() {
  local scope=$1

  case "$scope" in
    package-smoke)
      run_package_smoke_scope
      ;;
    debian-tests)
      run_debian_tests_scope
      ;;
    dependents)
      run_dependents_scope
      ;;
    all)
      run_package_smoke_scope
      run_debian_tests_scope
      run_dependents_scope
      ;;
    *)
      die "unsupported GLIB_TEST_SCOPE=$scope"
      ;;
  esac
}

main() {
  enable_source_repos
  prepare_apt
  verify_manifest
  run_scope "$GLIB_TEST_SCOPE"

  log "GLib $GLIB_UNDER_TEST checks passed for scope $GLIB_TEST_SCOPE"
}

main "$@"
EOF
