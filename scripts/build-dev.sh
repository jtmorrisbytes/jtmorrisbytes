#!usr/bin/env bash

printf "building for development\n\n"

ng build --verbose --delete-output-path --output-path="dist"