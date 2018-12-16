#!/bin/bash

set -x
set -e

(
  cd ui && \
  yarn install && \
  yarn run unit && \
  yarn run build
)

cargo build --verbose --all
cargo test --verbose --all
