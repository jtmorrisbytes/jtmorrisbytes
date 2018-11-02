#!/bin/bash
printf "\nbuilding for production\n\n"
ng build --prod --extract-css --optimization=true --source-map --delete-output-path --output-path="dist"
cp dist/index.html dist/404.html