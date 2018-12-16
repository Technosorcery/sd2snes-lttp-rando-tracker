#!/bin/bash

set -e
set -x

(
  cd ui && \
  yarn install && \
  yarn run unit && \
  yarn run build
)

cargo build --verbose --all
cargo test --verbose --all
