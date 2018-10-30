#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'


echo "hello from preinstall script";

angularInstalled=$(npm list -g | grep "@angular/cli");



echo $angularInstalled
