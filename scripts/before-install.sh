#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

if [ "${TRAVIS_OS_NAME}" = "windows" ]; then
  bash "${SCRIPT_DIR}/before-install-windows.sh"
fi

if [ "${TRAVIS_OS_NAME}" = "osx"     ]; then
  bash "${SCRIPT_DIR}/before-install-macos.sh"
fi
