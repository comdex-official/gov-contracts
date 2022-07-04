#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

cargo fmt
(cd packages/bindings && cargo check && cargo clippy --all-targets -- -D warnings)
(cd packages/cw3 && cargo check && cargo clippy --all-targets -- -D warnings)
(cd packages/cw4 && cargo check && cargo clippy --all-targets -- -D warnings)
(cd packages/storage-plus && cargo check && cargo clippy --all-targets -- -D warnings)
(cd packages/utils && cargo check && cargo clippy --all-targets -- -D warnings)
