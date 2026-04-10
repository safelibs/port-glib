#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
image="${TEST_ORIGINAL_IMAGE:-ubuntu:24.04}"
under_test="${GLIB_UNDER_TEST:-original}"
scope="${GLIB_TEST_SCOPE:-runtime}"
safe_deb_dir="${SAFE_DEB_DIR:-}"
safe_deb_dir_created=0

host_log() {
  printf '\n==> %s\n' "$1"
}

if ! command -v docker >/dev/null 2>&1; then
  printf 'docker is required to run %s\n' "$0" >&2
  exit 1
fi

case "$under_test" in
  original|safe)
    ;;
  *)
    printf 'GLIB_UNDER_TEST must be one of: original, safe\n' >&2
    exit 2
    ;;
esac

case "$scope" in
  package-smoke|compile-only|dev-package|runtime|all)
    ;;
  *)
    printf 'GLIB_TEST_SCOPE must be one of: package-smoke, compile-only, dev-package, runtime, all\n' >&2
    exit 2
    ;;
esac

cleanup_safe_deb_dir() {
  if (( safe_deb_dir_created == 1 )) && [[ -n $safe_deb_dir && -d $safe_deb_dir ]]; then
    rm -rf "$safe_deb_dir"
  fi
}

trap cleanup_safe_deb_dir EXIT

ensure_safe_deb_dir() {
  if [[ -n $safe_deb_dir ]]; then
    mkdir -p "$safe_deb_dir"
    return
  fi

  safe_deb_dir="$(mktemp -d "${TMPDIR:-/tmp}/port-glib-safe-debs.XXXXXX")"
  safe_deb_dir_created=1
}

require_safe_debs() {
  local artifact_root=$1
  local patterns=(
    "$artifact_root"/libglib2.0-0t64_*.deb
    "$artifact_root"/libglib2.0-bin_*.deb
    "$artifact_root"/libglib2.0-dev_*.deb
    "$artifact_root"/libglib2.0-dev-bin_*.deb
    "$artifact_root"/libglib2.0-data_*.deb
    "$artifact_root"/libgirepository-2.0-0_*.deb
    "$artifact_root"/libgirepository-2.0-dev_*.deb
  )
  local pattern matches

  shopt -s nullglob
  for pattern in "${patterns[@]}"; do
    matches=( $pattern )
    if (( ${#matches[@]} == 0 )); then
      printf 'Missing required safe package artifact for pattern: %s\n' "$pattern" >&2
      exit 1
    fi
  done
  shopt -u nullglob
}

clear_safe_artifacts() {
  local artifact_root=$1

  find "$artifact_root" -maxdepth 1 -type f \
    \( -name 'libglib2.0-*.deb' -o -name 'libgirepository-2.0-*.deb' -o -name '*.changes' -o -name '*.buildinfo' -o -name '*.ddeb' \) \
    -delete
}

copy_safe_artifacts() {
  local source_root=$1
  local artifact_root=$2
  local patterns=(
    "$source_root"/libglib2.0-0t64_*.deb
    "$source_root"/libglib2.0-bin_*.deb
    "$source_root"/libglib2.0-dev_*.deb
    "$source_root"/libglib2.0-dev-bin_*.deb
    "$source_root"/libglib2.0-data_*.deb
    "$source_root"/libgirepository-2.0-0_*.deb
    "$source_root"/libgirepository-2.0-dev_*.deb
    "$source_root"/glib2.0_*.changes
    "$source_root"/glib2.0_*.buildinfo
    "$source_root"/libglib2.0-*.ddeb
    "$source_root"/libgirepository-2.0-*.ddeb
  )
  local pattern matches

  clear_safe_artifacts "$artifact_root"
  shopt -s nullglob
  for pattern in "${patterns[@]}"; do
    matches=( $pattern )
    if (( ${#matches[@]} == 0 )); then
      continue
    fi
    cp -f "${matches[@]}" "$artifact_root"/
  done
  shopt -u nullglob
}

build_safe_debs_host() {
  host_log "Building local safe packages on host"
  (
    cd "$repo_root/safe"
    export DEB_BUILD_PROFILES='nogir noinsttest nodoc noudeb'
    dpkg-buildpackage -b -uc -us
  )
}

prepare_safe_debs() {
  ensure_safe_deb_dir
  if ! compgen -G "$repo_root/libglib2.0-0t64_*.deb" >/dev/null; then
    build_safe_debs_host
  fi
  copy_safe_artifacts "$repo_root" "$safe_deb_dir"
  require_safe_debs "$safe_deb_dir"
}

run_container() {
  local internal_stage=$1
  local docker_args=(
    docker run --rm --pull=missing -i
    --workdir /tmp/port-glib
    -e DEBIAN_FRONTEND=noninteractive
    -e GLIB_UNDER_TEST="$under_test"
    -e GLIB_TEST_SCOPE="$scope"
    -e GLIB_INTERNAL_STAGE="$internal_stage"
    -v "$repo_root:/src:ro"
  )

  if [[ $under_test == safe ]]; then
    docker_args+=(
      -e SAFE_DEB_ROOT=/artifacts
      -v "$safe_deb_dir:/artifacts"
    )
  fi

  "${docker_args[@]}" "$image" bash -s <<'EOF'
set -euo pipefail

SRC_ROOT=/src
SAFE_ROOT=/src/safe
MANIFEST="$SRC_ROOT/dependents.json"
WORK_ROOT=/tmp/port-glib
LOG_ROOT="$WORK_ROOT/logs"
GLIB_PREFIX=/opt/glib-original
UNDER_TEST="${GLIB_UNDER_TEST:?}"
SCOPE="${GLIB_TEST_SCOPE:?}"
INTERNAL_STAGE="${GLIB_INTERNAL_STAGE:-main}"
SAFE_DEB_ROOT="${SAFE_DEB_ROOT:-/src}"
AUTOPKGTEST_TMP="$WORK_ROOT/autopkgtest-tmp"
WRAPPER_TESTS=(
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
)
SAFE_BASE_PACKAGES=(
  libglib2.0-0t64
  libglib2.0-bin
  libglib2.0-dev
  libglib2.0-dev-bin
  libglib2.0-data
  libgirepository-2.0-0
  libgirepository-2.0-dev
)

mkdir -p "$WORK_ROOT" "$LOG_ROOT" "$AUTOPKGTEST_TMP"

multiarch=""
glib_libdir=""
glib_pkgconfig_dir=""

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

run_shell_logged() {
  local name=$1
  local script=$2

  run_logged "$name" bash -lc "$script"
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
  if grep -q '^Types: deb deb-src$' /etc/apt/sources.list.d/ubuntu.sources; then
    return
  fi
  sed -i 's/^Types: deb$/Types: deb deb-src/' /etc/apt/sources.list.d/ubuntu.sources
}

prepare_apt_base() {
  log "Updating apt metadata"
  run_logged apt-update apt-get update

  log "Installing container bootstrap tools"
  run_logged apt-bootstrap apt-get install -y --no-install-recommends jq python3 pkg-config dbus dbus-user-session ca-certificates dpkg-dev
  multiarch="$(dpkg-architecture -qDEB_HOST_MULTIARCH)"
}

set_original_env() {
  export PATH="$GLIB_PREFIX/bin:/usr/sbin:/usr/bin:/sbin:/bin"
  export LD_LIBRARY_PATH="$glib_libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  export PKG_CONFIG_PATH="$glib_pkgconfig_dir${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
  export ACLOCAL_PATH="$GLIB_PREFIX/share/aclocal${ACLOCAL_PATH:+:$ACLOCAL_PATH}"
  export GIO_MODULE_DIR="/usr/lib/$multiarch/gio/modules"
}

set_safe_env() {
  export PATH="/usr/sbin:/usr/bin:/sbin:/bin"
  unset LD_LIBRARY_PATH
  unset PKG_CONFIG_PATH
  unset ACLOCAL_PATH
  export GIO_MODULE_DIR="/usr/lib/$multiarch/gio/modules"
}

assert_not_src_path() {
  local value=$1
  local context=$2

  if [[ $value == *"/src/"* ]]; then
    die "$context unexpectedly resolved into the source tree: $value"
  fi
}

assert_command_target() {
  local command_name=$1
  local expected_command=$2
  local expected_resolved=${3:-$expected_command}
  local actual_command
  local actual_resolved

  actual_command="$(command -v "$command_name")"
  [[ $actual_command == "$expected_command" ]] || die "$command_name resolved to $actual_command, expected $expected_command"
  actual_resolved="$(readlink -f "$actual_command")"
  [[ $actual_resolved == "$expected_resolved" ]] || die "$command_name ultimately resolved to $actual_resolved, expected $expected_resolved"
  assert_not_src_path "$actual_command" "command -v $command_name"
  assert_not_src_path "$actual_resolved" "readlink -f $(command -v $command_name)"
}

assert_pkg_config_dir() {
  local package_name=$1
  local expected_dir=$2
  local actual_dir

  actual_dir="$(pkg-config --variable=pcfiledir "$package_name")"
  [[ $actual_dir == "$expected_dir" ]] || die "pkg-config pcfiledir for $package_name is $actual_dir, expected $expected_dir"
  assert_not_src_path "$actual_dir" "pkg-config pcfiledir for $package_name"
}

assert_library_target() {
  local target=$1
  local expected_real=$2
  local actual_real
  local ldd_output

  actual_real="$(readlink -f "$target")"
  [[ $actual_real == "$expected_real" ]] || die "$target ultimately resolved to $actual_real, expected $expected_real"
  ldd_output="$(ldd "$actual_real")"
  ! grep -F 'not found' <<<"$ldd_output" >/dev/null || die "$actual_real has missing runtime dependencies"
  assert_not_src_path "$ldd_output" "ldd output for $target"
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

  glib_libdir="$GLIB_PREFIX/lib/$multiarch"
  [[ -d $glib_libdir ]] || glib_libdir="$GLIB_PREFIX/lib"
  glib_pkgconfig_dir="$glib_libdir/pkgconfig"
  [[ -d $glib_pkgconfig_dir ]] || glib_pkgconfig_dir="$GLIB_PREFIX/lib/pkgconfig"
}

collect_safe_debs() {
  local patterns=(
    "$SAFE_DEB_ROOT"/libglib2.0-0t64_*.deb
    "$SAFE_DEB_ROOT"/libglib2.0-bin_*.deb
    "$SAFE_DEB_ROOT"/libglib2.0-dev_*.deb
    "$SAFE_DEB_ROOT"/libglib2.0-dev-bin_*.deb
    "$SAFE_DEB_ROOT"/libglib2.0-data_*.deb
    "$SAFE_DEB_ROOT"/libgirepository-2.0-0_*.deb
    "$SAFE_DEB_ROOT"/libgirepository-2.0-dev_*.deb
  )
  local pattern
  local matches

  SAFE_DEBS=()
  shopt -s nullglob
  for pattern in "${patterns[@]}"; do
    matches=( $pattern )
    (( ${#matches[@]} > 0 )) || die "Missing required local safe package for pattern $pattern"
    SAFE_DEBS+=( "${matches[@]}" )
  done
  shopt -u nullglob
}

install_safe_packages() {
  log "Installing locally built safe packages"
  collect_safe_debs
  run_logged apt-install-safe apt-get install -y --no-install-recommends "${SAFE_DEBS[@]}"
  run_logged apt-hold-safe apt-mark hold "${SAFE_BASE_PACKAGES[@]}"
}

install_safe_or_archive_tests_package() {
  local local_tests=( "$SAFE_DEB_ROOT"/libglib2.0-tests_*.deb )

  shopt -s nullglob
  if (( ${#local_tests[@]} > 0 )); then
    run_logged apt-install-tests-local apt-get install -y --no-install-recommends "${local_tests[@]}"
    if [[ $UNDER_TEST == safe ]]; then
      run_logged apt-hold-tests apt-mark hold libglib2.0-tests
    fi
  else
    run_logged apt-install-tests-archive apt-get install -y --no-install-recommends libglib2.0-tests
  fi
  shopt -u nullglob
}

install_compile_prereqs() {
  log "Installing compile prerequisites"
  run_logged apt-compile-prereqs apt-get install -y --no-install-recommends build-essential pkg-config
}

install_original_dev_support() {
  log "Installing archive development helpers for original-mode package checks"
  run_logged apt-original-dev apt-get install -y --no-install-recommends libglib2.0-dev-bin libgirepository-2.0-dev
}

install_runtime_packages() {
  local runtime_packages=()

  mapfile -t runtime_packages < <(
    jq -r '.dependents[] | select(.glib_dependency_kind == "compile_time_and_runtime") | .binary_package' "$MANIFEST"
  )

  log "Installing runtime dependent packages"
  run_logged apt-runtime-install apt-get install -y --no-install-recommends \
    "${runtime_packages[@]}" \
    libvirt-clients \
    ostree
}

install_all_final_dependencies() {
  log "Installing installed-tests and futureproofing dependencies"
  run_logged apt-all-final apt-get install -y --no-install-recommends \
    dbus-daemon \
    dbus-x11 \
    dconf-gsettings-backend \
    dpkg-repack \
    gnome-desktop-testing \
    gsettings-desktop-schemas \
    locales \
    xauth \
    xvfb
  install_safe_or_archive_tests_package
}

run_package_smoke_original() {
  build_original_glib
  set_original_env

  assert_command_target gio "$GLIB_PREFIX/bin/gio"
  assert_command_target gdbus "$GLIB_PREFIX/bin/gdbus"
  assert_command_target glib-compile-resources "$GLIB_PREFIX/bin/glib-compile-resources"
  assert_command_target gdbus-codegen "$GLIB_PREFIX/bin/gdbus-codegen"
  assert_pkg_config_dir glib-2.0 "$glib_pkgconfig_dir"
  if pkg-config --exists girepository-2.0; then
    assert_pkg_config_dir girepository-2.0 "$glib_pkgconfig_dir"
  fi
  assert_library_target "$glib_libdir/libgio-2.0.so.0" "$glib_libdir/libgio-2.0.so.0.8000.0"
}

run_package_smoke_safe() {
  install_safe_packages
  set_safe_env

  [[ -z ${LD_LIBRARY_PATH-} ]] || die "safe mode must not set LD_LIBRARY_PATH"
  [[ -z ${PKG_CONFIG_PATH-} ]] || die "safe mode must not set PKG_CONFIG_PATH"

  assert_command_target gio "/usr/bin/gio"
  assert_command_target gdbus "/usr/bin/gdbus"
  assert_command_target gapplication "/usr/bin/gapplication"
  assert_command_target gio-querymodules "/usr/bin/gio-querymodules" "/usr/lib/$multiarch/glib-2.0/gio-querymodules"
  assert_command_target glib-compile-schemas "/usr/bin/glib-compile-schemas" "/usr/lib/$multiarch/glib-2.0/glib-compile-schemas"
  assert_command_target glib-compile-resources "/usr/bin/glib-compile-resources"
  assert_command_target gdbus-codegen "/usr/bin/gdbus-codegen"
  assert_command_target gi-compile-repository "/usr/bin/gi-compile-repository" "/usr/lib/$multiarch/glib-2.0/gi-compile-repository"
  assert_command_target gi-decompile-typelib "/usr/bin/gi-decompile-typelib" "/usr/lib/$multiarch/glib-2.0/gi-decompile-typelib"
  assert_command_target gi-inspect-typelib "/usr/bin/gi-inspect-typelib" "/usr/lib/$multiarch/glib-2.0/gi-inspect-typelib"
  assert_pkg_config_dir glib-2.0 "/usr/lib/$multiarch/pkgconfig"
  assert_pkg_config_dir girepository-2.0 "/usr/lib/$multiarch/pkgconfig"
  assert_library_target "/usr/lib/$multiarch/libgio-2.0.so.0" "/usr/lib/$multiarch/libgio-2.0.so.0.8000.0"
  assert_library_target "/usr/lib/$multiarch/libgirepository-2.0.so.0" "/usr/lib/$multiarch/libgirepository-2.0.so.0.8000.0"

  run_logged postinst-state python3 /src/safe/tools/check-postinst-state.py \
    --manifest /src/safe/abi/postinst-state/runtime.json \
    --root /
}

run_package_smoke() {
  case "$UNDER_TEST" in
    original)
      run_package_smoke_original
      ;;
    safe)
      run_package_smoke_safe
      ;;
  esac
}

assert_binary_uses_target_glib() {
  local binary=$1
  local resolved=$binary
  local ldd_output
  local actual_lib
  local expected_lib

  if [[ $resolved != /* ]]; then
    resolved="$(command -v "$resolved")"
  fi

  ldd_output="$(ldd "$resolved")"
  actual_lib="$(awk '/libglib-2\.0\.so\.0/ { print $3; exit }' <<<"$ldd_output")"
  [[ -n $actual_lib && $actual_lib != "not" ]] || die "unable to resolve libglib-2.0 for $resolved"
  actual_lib="$(readlink -f "$actual_lib")"

  case "$UNDER_TEST" in
    original)
      expected_lib="$(readlink -f "$glib_libdir/libglib-2.0.so.0")"
      [[ $actual_lib == "$expected_lib" ]] \
        || die "$resolved is loading libglib-2.0 from $actual_lib, expected $expected_lib"
      ;;
    safe)
      expected_lib="$(readlink -f "/usr/lib/$multiarch/libglib-2.0.so.0")"
      [[ $actual_lib == "$expected_lib" ]] \
        || die "$resolved is loading libglib-2.0 from $actual_lib, expected $expected_lib"
      assert_not_src_path "$ldd_output" "ldd output for $resolved"
      ;;
  esac
}

test_qemu() {
  assert_binary_uses_target_glib qemu-system-x86_64

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

  assert_binary_uses_target_glib /usr/sbin/NetworkManager
  assert_binary_uses_target_glib nmcli

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

  assert_binary_uses_target_glib /usr/sbin/bluetoothd

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

  assert_binary_uses_target_glib flatpak

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

  assert_binary_uses_target_glib /usr/sbin/ModemManager
  assert_binary_uses_target_glib mmcli

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

  assert_binary_uses_target_glib /usr/libexec/fwupd/fwupd
  assert_binary_uses_target_glib fwupdmgr

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

  assert_binary_uses_target_glib /usr/libexec/gvfsd
  assert_binary_uses_target_glib /usr/libexec/gvfs-udisks2-volume-monitor

  if ! dbus-run-session -- bash >"$WORK_ROOT/gvfs-session.out" 2>"$WORK_ROOT/gvfs-session.err" <<SH
set -euo pipefail
export XDG_RUNTIME_DIR="$WORK_ROOT/gvfs-runtime"
mkdir -p "\$XDG_RUNTIME_DIR"
gdbus call --session \
  --dest org.gtk.vfs.Daemon \
  --object-path /org/gtk/vfs/Daemon \
  --method org.gtk.vfs.Daemon.ListMonitorImplementations >"$output" 2>"$WORK_ROOT/gvfs-call.err"
grep -q 'org.gtk.vfs.UDisks2VolumeMonitor' "$output"
SH
  then
    cat "$WORK_ROOT/gvfs-session.err" >&2 || true
    cat "$WORK_ROOT/gvfs-call.err" >&2 || true
    cat "$output" >&2 || true
    return 1
  fi
}

test_gstreamer_tools() {
  assert_binary_uses_target_glib gst-launch-1.0
  gst-launch-1.0 -q fakesrc num-buffers=4 ! fakesink
}

test_libvirt_daemon() {
  local output="$WORK_ROOT/libvirt.out"

  assert_binary_uses_target_glib /usr/sbin/libvirtd
  assert_binary_uses_target_glib virsh

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

  assert_binary_uses_target_glib /usr/libexec/udisks2/udisksd
  assert_binary_uses_target_glib udisksctl

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

  assert_binary_uses_target_glib tracker3
  assert_binary_uses_target_glib /usr/libexec/tracker-miner-fs-3

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
grep -q 'org.freedesktop.Tracker3.Miner.Files' "$output"
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

  case "$UNDER_TEST" in
    original)
      set_original_env
      assert_command_target glib-compile-resources "$GLIB_PREFIX/bin/glib-compile-resources"
      assert_command_target glib-compile-schemas "$GLIB_PREFIX/bin/glib-compile-schemas"
      ;;
    safe)
      set_safe_env
      assert_command_target glib-compile-resources "/usr/bin/glib-compile-resources"
      assert_command_target glib-compile-schemas "/usr/bin/glib-compile-schemas" "/usr/lib/$multiarch/glib-2.0/glib-compile-schemas"
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

run_compile_only() {
  run_package_smoke
  install_compile_prereqs
  if [[ $UNDER_TEST == original ]]; then
    install_original_dev_support
    set_original_env
  fi

  run_shell_logged debian-build "cd '$SAFE_ROOT' && debian/tests/build"
  run_shell_logged debian-build-static "cd '$SAFE_ROOT' && debian/tests/build-static"
  build_pocillo_icon_theme
  run_shell_logged girepository-compile-only "cd '$SAFE_ROOT' && bash tests/package/girepository-compile-only.sh"
}

run_dev_package() {
  run_compile_only
  run_shell_logged girepository-installed "cd '$SAFE_ROOT' && bash tests/package/girepository-installed.sh"
}

run_runtime() {
  run_package_smoke
  install_runtime_packages
  if [[ $UNDER_TEST == original ]]; then
    set_original_env
  fi

  while IFS= read -r package; do
    run_manifest_entry "$package"
  done < <(jq -r '.dependents[].binary_package' "$MANIFEST")
}

run_all_final() {
  run_package_smoke
  install_all_final_dependencies
  if [[ $UNDER_TEST == original ]]; then
    set_original_env
  fi

  run_shell_logged installed-tests "cd '$SAFE_ROOT' && AUTOPKGTEST_TMP='$AUTOPKGTEST_TMP' debian/tests/installed-tests"
  local wrapper
  for wrapper in "${WRAPPER_TESTS[@]}"; do
    run_shell_logged "$wrapper" "cd '$SAFE_ROOT' && AUTOPKGTEST_TMP='$AUTOPKGTEST_TMP' debian/tests/$wrapper"
  done
  run_shell_logged 1065022-futureproofing "cd '$SAFE_ROOT' && AUTOPKGTEST_TMP='$AUTOPKGTEST_TMP' debian/tests/1065022-futureproofing"
}

main() {
  enable_source_repos
  prepare_apt_base
  verify_manifest

  case "$INTERNAL_STAGE" in
    main)
      case "$SCOPE" in
        package-smoke)
          run_package_smoke
          ;;
        compile-only)
          run_compile_only
          ;;
        dev-package)
          run_dev_package
          ;;
        runtime)
          run_runtime
          ;;
        *)
          die "internal stage 'main' does not support scope $SCOPE"
          ;;
      esac
      ;;
    package-smoke)
      run_package_smoke
      ;;
    dev-package)
      run_dev_package
      ;;
    runtime)
      run_runtime
      ;;
    all-final)
      run_all_final
      ;;
    *)
      die "unknown internal stage: $INTERNAL_STAGE"
      ;;
  esac

  log "Completed $UNDER_TEST / ${INTERNAL_STAGE:-$SCOPE}"
}

main "$@"
EOF
}

if [[ $under_test == safe ]]; then
  prepare_safe_debs
fi

case "$scope" in
  all)
    run_container package-smoke
    run_container dev-package
    run_container runtime
    run_container all-final
    ;;
  *)
    run_container main
    ;;
esac
