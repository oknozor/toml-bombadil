#!/usr/bin/env bash
set -euo pipefail

export BOMBADIL_HOME="$(cd "$(dirname "$0")/.." && pwd)"

echoerr() {
   echo "$@" 1>&2
}

release() {

   TAR_DIR="${BOMBADIL_HOME}/target/tar"
   COMPLETIONS_DIR = "${TAR_DIR}/completions"

   target="${1:-}"
   if [[ $target == *"osx"* ]]; then
      echoerr "OSX cross-compile is impossible. Fallbacking to cargo..."
      target=""
   fi

   cd "$BOMBADIL_HOME"

   rm -rf "${BOMBADIL_HOME}/target" 2> /dev/null || true

   if [ -n "$target" ]; then
      cargo install cross 2> /dev/null || true
      cross build --release --target "$target" --bin bombadil
      bin_folder="${target}/release"
   else
      cargo build --release
      bin_folder="release"
   fi

   bin_path="${BOMBADIL_HOME}/target/${bin_folder}/bombadil"
   chmod +x "$bin_path"
   mkdir -p "$COMPLETIONS_DIR" 2> /dev/null || true

   cp "$bin_path" "$TAR_DIR"

   # Copy completion files
   cp "${BOMBADIL_HOME}/target/${bin_folder}/build/toml-bombadil/"-*/out/_bombadil "$COMPLETIONS_DIR"
   cp "${BOMBADIL_HOME}/target/${bin_folder}/build/toml-bombadil/"-*/out/bombadil.bash "$COMPLETIONS_DIR"
   cp "${BOMBADIL_HOME}/target/${bin_folder}/build/toml-bombadil/"-*/out/bombadil.elv "$COMPLETIONS_DIR"
   cp "${BOMBADIL_HOME}/target/${bin_folder}/build/toml-bombadil/"-*/out/bombadil.fish "$COMPLETIONS_DIR"

   cd "$TAR_DIR"
   tar -czf bombadil.tar.gz *

}

cmd="$1"
shift

release "$@"