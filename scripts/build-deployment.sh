#!usr/bin/env bash
printf "\nbuilding for production\n\n"
TRAVIS_BUILD_DIR="./dist/"
ng build --prod --extract-css --optimization=true --source-map --delete-output-path --output-path="$TRAVIS_BUILD_DIR/production"