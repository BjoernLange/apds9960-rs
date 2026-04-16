#!/bin/bash

set -euo pipefail

cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=apds9960\">" > target/doc/index.html
cp -r target/doc ./docs