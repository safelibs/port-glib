#!/usr/bin/env bash
# Build the safe port via dpkg-buildpackage rooted in safe/. Stamps the
# changelog with `+safelibs<commit-epoch>` so the produced .deb files
# carry a deterministic version that wins over Ubuntu's copy under the
# apt pin in safelibs/apt.
#
# port-glib's debian/rules invokes tools/build-abi-shell.py, which reads
# from safe/vendor/{original,build-check,workspace}. Those paths are
# intentionally untracked (they are derivable from sources elsewhere in
# the repo) so this script materializes them before the build runs:
#   - vendor/original  -> symlink to the tracked port-glib/original/
#   - vendor/workspace -> symlinks to the CVE / dependents JSON at the
#                          repo root.
#   - vendor/build-check -> fresh `meson setup` + `meson compile` against
#                          vendor/original, giving build-abi-shell.py the
#                          meson-info, meson-private, config.h, and
#                          per-subdir compiled outputs it copies into
#                          debian/build/deb.
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=/dev/null
. "$repo_root/scripts/lib/build-deb-common.sh"

debian_patches_backup_dir=""

restore_debian_patches() {
  if [[ -z "${debian_patches_backup_dir:-}" ]]; then
    return 0
  fi

  rm -rf -- "$repo_root/safe/debian/patches"
  if [[ -d "$debian_patches_backup_dir/patches" ]]; then
    mv "$debian_patches_backup_dir/patches" "$repo_root/safe/debian/patches"
  fi
  rm -rf -- "$debian_patches_backup_dir"
  debian_patches_backup_dir=""
}

hide_debian_patches_for_binary_build() {
  if [[ ! -d "$repo_root/safe/debian/patches" ]]; then
    return 0
  fi

  debian_patches_backup_dir="$(mktemp -d)"
  mv "$repo_root/safe/debian/patches" "$debian_patches_backup_dir/patches"
  trap restore_debian_patches EXIT
}

setup_vendor_dir() {
  local safe_dir="$1"
  cd "$safe_dir"
  mkdir -p vendor vendor/workspace

  if [[ ! -e vendor/original ]]; then
    ln -s ../../original vendor/original
  fi

  local f
  for f in all_cves.json dependents.json relevant_cves.json; do
    if [[ ! -e "vendor/workspace/$f" ]]; then
      ln -s "../../../$f" "vendor/workspace/$f"
    fi
  done
}

build_vendor_build_check() {
  local safe_dir="$1"
  cd "$safe_dir"

  local build_dir="vendor/build-check"
  local current_source
  current_source="$(cd vendor/original && pwd -P)"

  if [[ -f "$build_dir/build.ninja" ]] \
    && grep -Eq 'meson --internal regenerate .*/original \.' "$build_dir/build.ninja" \
    && ! grep -Fq "$current_source" "$build_dir/build.ninja" \
    && ! grep -Fq "$safe_dir/vendor/original" "$build_dir/build.ninja"; then
    printf 'build-debs.sh: removing stale Meson build-check rooted outside this checkout\n' >&2
    rm -rf -- "$build_dir"
  fi

  if [[ ! -f vendor/build-check/build.ninja ]]; then
    meson setup vendor/build-check vendor/original
  fi
  meson compile -C vendor/build-check
}

prepare_rust_env
prepare_dist_dir "$repo_root"

cd "$repo_root/safe"
stamp_safelibs_changelog "$repo_root"

# Install build deps (incl. meson + ninja + glib's libs) before
# materializing vendor/build-check, which needs meson on PATH.
sudo mk-build-deps -i -r -t "apt-get -y --no-install-recommends" debian/control

setup_vendor_dir "$repo_root/safe"
build_vendor_build_check "$repo_root/safe"

cd "$repo_root/safe"
# Drop the existing debian/patches set: the tree under safe/ is already
# in post-patch shape (see _synthesize_orig_tarball_if_needed for the
# 3.0 (quilt) variant of this rule), and -b skips the source-package
# build that would otherwise try to apply them. Restore the tracked
# directory after the build so local/CI worktrees stay clean.
hide_debian_patches_for_binary_build

# Binary-only build. We deliberately skip the source package: the tree
# legitimately contains ~90MB of freshly compiled upstream artifacts
# under vendor/build-check/ that dpkg-source can't represent in a
# 3.0 (quilt) debian.tar without bloating the source release. The .deb
# / .changes / .buildinfo artifacts CI publishes don't depend on the
# source package.
current_version="$(dpkg-parsechangelog -S Version)"
dpkg-buildpackage -us -uc -b
restore_debian_patches
trap - EXIT

shopt -s nullglob
artifacts=(
  ../*_"$current_version"_*.deb
  ../*_"$current_version"_*.ddeb
  ../*_"$current_version"_*.buildinfo
  ../*_"$current_version"_*.changes
)
shopt -u nullglob
if (( ${#artifacts[@]} == 0 )); then
  printf 'build-debs.sh: dpkg-buildpackage produced no artifacts\n' >&2
  exit 1
fi
cp -v "${artifacts[@]}" "$repo_root/dist"/
