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
  -v "$repo_root:/src:ro" \
  "$image" \
  bash -s <<'EOF'
set -euo pipefail

SRC_ROOT=/src
MANIFEST="$SRC_ROOT/dependents.json"
WORK_ROOT=/tmp/port-glib
LOG_ROOT="$WORK_ROOT/logs"
GLIB_PREFIX=/opt/glib-original

mkdir -p "$WORK_ROOT" "$LOG_ROOT"

multiarch=""
glib_libdir=""

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
    cat "$log_file" >&2
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

assert_binary_uses_original_glib() {
  local binary=$1
  local resolved=$binary
  local ldd_output

  if [[ $resolved != /* ]]; then
    resolved="$(command -v "$resolved")"
  fi

  ldd_output="$(ldd "$resolved")"
  grep -F "$glib_libdir/libglib-2.0.so.0" <<<"$ldd_output" >/dev/null \
    || die "$resolved is not loading libglib-2.0 from $glib_libdir"
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
  run_logged apt-bootstrap apt-get install -y --no-install-recommends jq python3 dbus dbus-user-session
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

install_runtime_packages() {
  local runtime_packages=()

  mapfile -t runtime_packages < <(
    jq -r '.dependents[] | select(.glib_dependency_kind == "compile_time_and_runtime") | .binary_package' "$MANIFEST"
  )

  log "Installing runtime dependent packages"
  run_logged apt-runtime-install apt-get install -y --no-install-recommends \
    "${runtime_packages[@]}" \
    libvirt-clients
}

test_qemu() {
  assert_binary_uses_original_glib qemu-system-x86_64

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
  local output="$WORK_ROOT/network-manager.offline"

  assert_binary_uses_original_glib /usr/sbin/NetworkManager
  assert_binary_uses_original_glib nmcli

  /usr/sbin/NetworkManager --print-config >/dev/null
  nmcli --offline connection add \
    type ethernet \
    ifname '*' \
    con-name test-offline \
    save no \
    ipv4.method manual \
    ipv4.addresses 192.0.2.10/24 \
    ipv6.method ignore >"$output"
  grep -q '^\[connection\]' "$output"
}

test_bluez() {
  local log_file="$WORK_ROOT/bluetoothd.log"

  assert_binary_uses_original_glib /usr/sbin/bluetoothd

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
  local repo_file="$WORK_ROOT/local.flatpakrepo"
  local remotes_file="$WORK_ROOT/flatpak-remotes.txt"

  assert_binary_uses_original_glib flatpak

  rm -rf "$home_dir" "$runtime_dir"
  mkdir -p "$home_dir" "$runtime_dir"

  cat >"$repo_file" <<'REPO'
[Flatpak Repo]
Title=Local
Url=http://127.0.0.1/repo
NoGPGVerify=true
REPO

  HOME="$home_dir" XDG_RUNTIME_DIR="$runtime_dir" \
    flatpak remote-add --user --if-not-exists --from localhost "$repo_file" >/dev/null 2>"$WORK_ROOT/flatpak-remote-add.err"
  HOME="$home_dir" XDG_RUNTIME_DIR="$runtime_dir" \
    flatpak remotes --user >"$remotes_file"
  grep -q '^localhost[[:space:]]' "$remotes_file"
}

test_modemmanager() {
  local output="$WORK_ROOT/modemmanager.out"

  assert_binary_uses_original_glib /usr/sbin/ModemManager
  assert_binary_uses_original_glib mmcli

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

  assert_binary_uses_original_glib fwupdtool

  fwupdtool get-remotes >"$output" 2>"$WORK_ROOT/fwupd-remotes.err"
  grep -Eq 'Remote ID:[[:space:]]+lvfs' "$output"
}

test_gvfs_daemons() {
  assert_binary_uses_original_glib /usr/libexec/gvfsd

  dbus-run-session -- bash <<SH
set -euo pipefail
export XDG_RUNTIME_DIR="$WORK_ROOT/gvfs-runtime"
mkdir -p "\$XDG_RUNTIME_DIR"
/usr/libexec/gvfsd --replace >"$WORK_ROOT/gvfsd.log" 2>&1 &
gvfs_pid=\$!
cleanup() {
  kill "\$gvfs_pid" >/dev/null 2>&1 || true
  wait "\$gvfs_pid" || true
}
trap cleanup EXIT
gio mount -l >"$WORK_ROOT/gvfs-mounts.out" 2>"$WORK_ROOT/gvfs-mounts.err"
SH
}

test_gstreamer_tools() {
  assert_binary_uses_original_glib gst-launch-1.0
  gst-launch-1.0 -q fakesrc num-buffers=4 ! fakesink
}

test_libvirt_daemon() {
  local output="$WORK_ROOT/libvirt.out"

  assert_binary_uses_original_glib /usr/sbin/libvirtd
  assert_binary_uses_original_glib virsh

  /usr/sbin/libvirtd --version >/dev/null
  virsh -c test:///default list --all >"$output"
  grep -q '[[:space:]]test[[:space:]]' "$output"
}

test_udisks2() {
  local output="$WORK_ROOT/udisks.out"

  assert_binary_uses_original_glib /usr/libexec/udisks2/udisksd
  assert_binary_uses_original_glib udisksctl

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

  assert_binary_uses_original_glib tracker3
  assert_binary_uses_original_glib /usr/libexec/tracker-miner-fs-3

  dbus-run-session -- tracker3 daemon --list-miners-available >"$output"
  grep -q 'org.freedesktop.Tracker3.Miner.Files' "$output"
}

build_pocillo_icon_theme() {
  local source_root="$WORK_ROOT/deb-src"
  local package_dir

  set_glib_env

  [[ "$(readlink -f "$(command -v glib-compile-resources)")" == "$GLIB_PREFIX/bin/glib-compile-resources" ]] \
    || die "budgie-artwork build is not using glib-compile-resources from $GLIB_PREFIX"
  [[ "$(readlink -f "$(command -v glib-compile-schemas)")" == "$GLIB_PREFIX/bin/glib-compile-schemas" ]] \
    || die "budgie-artwork build is not using glib-compile-schemas from $GLIB_PREFIX"

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

main() {
  enable_source_repos
  prepare_apt
  verify_manifest
  build_original_glib
  set_glib_env
  install_runtime_packages

  while IFS= read -r package; do
    run_manifest_entry "$package"
  done < <(jq -r '.dependents[].binary_package' "$MANIFEST")

  log "All dependent package checks passed"
}

main "$@"
EOF
