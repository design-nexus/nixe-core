#!/usr/bin/env bash
# Regenerate the EDS FFI crates under crates/eds from the installed GIR files.
# Needs gir (cargo install --git https://github.com/gtk-rs/gir --tag 0.22.3 gir)
# and evolution-data-server / libical / json-glib / libsoup3 development GIRs.
# The generated code is committed; builds never run this. libxml2-sys and camel-sys are hand-written
# opaque stubs: only pointers to their types appear, and the Camel GIR does not
# generate working code.
set -euo pipefail
cd "$(dirname "$0")/../crates/eds"
GIR_FILES_TAG=0.22.5
cache=.gir-files
rm -rf "$cache" && mkdir -p "$cache"
git -c advice.detachedHead=false clone -q --depth 1 --branch "$GIR_FILES_TAG" https://github.com/gtk-rs/gir-files "$cache/gtk-rs"
for f in "$cache"/gtk-rs/*.gir; do ln -s "gtk-rs/$(basename "$f")" "$cache/"; done
for g in EDataServer-1.2 ECal-2.0 ICalGLib-4.0 Camel-1.2 Json-1.0 Soup-3.0; do
  python3 -I ../../tools/fix-eds-gir.py "/usr/share/gir-1.0/$g.gir" "$cache/$g.gir"
done
for crate in json-glib-sys icalglib-sys edataserver-sys ecal-sys; do
  (cd "$crate" && gir -c Gir.toml -o .)
done
{
  echo "gir 0.22.3, gir-files $GIR_FILES_TAG"
  pacman -Q evolution-data-server libical json-glib libsoup3 2>/dev/null || true
} > GENERATED_FROM
