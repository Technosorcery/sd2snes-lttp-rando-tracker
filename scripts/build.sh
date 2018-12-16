#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

ls -l ./scripts

if [ "${TRAVIS_OS_NAME}" = "windows" ]; then
  bash "${SCRIPT_DIR}/build-windows.sh"
fi

if [ "${TRAVIS_OS_NAME}" = "linux" ]; then
  bash "${SCRIPT_DIR}/build-linux.sh"
fi

if [ "${TRAVIS_OS_NAME}" = "osx" ]; then
  bash "${SCRIPT_DIR}/build-macos.sh"
fi
