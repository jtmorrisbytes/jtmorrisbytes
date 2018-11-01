#!usr/bin/env bash

printf "building for development\n\n"

ng build --verbose --output-path="$TRAVIS_BUILD_DIR/dev"