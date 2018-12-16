#!/bin/bash

set -e
set -x

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

for build in debug release; do
  ls -l "target/${build}"
  mkdir -p "artifacts/${build}"
  if [ -f "target/${build}/sd2snes-lttp-rando-tracker" ]; then
    mv -v "target/${build}/sd2snes-lttp-rando-tracker" "artifacts/${build}/."
  fi
  if [ -f "target/${build}/sd2snes-lttp-rando-tracker.exe" ]; then
    mv -v "target/${build}/sd2snes-lttp-rando-tracker.exe" "artifacts/${build}/."
  fi
done

find artifacts -print
