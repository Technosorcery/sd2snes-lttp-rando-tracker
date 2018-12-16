#!/bin/bash

set -x

export PATH=$PATH:'/c/Program Files (x86)/Yarn/bin':'/c/Program Files/nodejs'

(
  cd ui && \
  powershell 'yarn install' && \
  powershell 'yarn run unit' && \
  powershell 'yarn run build'
)

ls -l '/c/Program Files (x86)/Yarn/bin/'
export YARN_PATH='/c/Program Files (x86)/Yarn/bin/yarn.cmd'
cargo build --verbose --all
cargo test --verbose --all
