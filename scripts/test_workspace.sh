#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

cargo fmt
(cd packages/bindings && cargo test)
(cd packages/cw3 && cargo test )
(cd packages/storage-plus && cargo test )
(cd packages/cw4 && cargo test)
(cd packages/utils && cargo test )