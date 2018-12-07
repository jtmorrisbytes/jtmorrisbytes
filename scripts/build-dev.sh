#!/bin/bash

printf "building for development\n\n"

ng build --verbose --delete-output-path --output-path="dist"
cp dist/index.html dist/404.html