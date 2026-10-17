#!/usr/bin/env bash
# bupha entry point — Rust-only crate (no `.hnl` surface).
set -euo pipefail
export CARGO_NET_GIT_FETCH_WITH_CLI="${CARGO_NET_GIT_FETCH_WITH_CLI:-true}"
if [ -z "${GIT_CONFIG_COUNT:-}" ]; then
  export GIT_CONFIG_COUNT=1
  export GIT_CONFIG_KEY_0="url.git@github.com:handl-lang/.insteadOf"
  export GIT_CONFIG_VALUE_0="https://github.com/handl-lang/"
fi
exec cargo test --all-targets
