#!/bin/bash

set -e
set -x

cd artifacts
for build in debug release; do
  (
    cd "${build}"
    zip ../sd2snes-lttp-rando-tracker-"${TRAVIS_OS_NAME}"-"${TRAVIS_TAG}"-"${build}".zip *
  )
done
