#!/bin/bash

set -e
set -x

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

if [ "${TRAVIS_OS_NAME}" = "linux" ]; then
  bash "${SCRIPT_DIR}/before-deploy-linux.sh"
fi

if [ "${TRAVIS_OS_NAME}" = "windows" ]; then
  bash "${SCRIPT_DIR}/before-deploy-windows.sh"
fi

if [ "${TRAVIS_OS_NAME}" = "osx"     ]; then
  bash "${SCRIPT_DIR}/before-deploy-macos.sh"
fi
