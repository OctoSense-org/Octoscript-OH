#!/usr/bin/env bash
# Build this app and install it on a connected phone.
#
# The app shell -- the ArkTS entry point, the DevEco project, the Rust bridge --
# lives in a Octoscript-OH checkout, not in here. This project is the frontend and
# your own native code; the shell is what turns them into a HAP.
#
# Point OCTOSCRIPT_OH at that checkout, or keep one beside this directory.
set -euo pipefail
cd "$(dirname "$0")"
APP_DIR="$PWD"

OCTOSCRIPT_OH="${OCTOSCRIPT_OH:-$APP_DIR/../Octoscript-OH}"
if [ ! -x "$OCTOSCRIPT_OH/build.sh" ]; then
  cat >&2 <<MSG
This needs a Octoscript-OH checkout to build against, and there is none at:
  $OCTOSCRIPT_OH

Either clone one beside this project:
  git clone https://github.com/OctoSense-org/Octoscript-OH.git "$APP_DIR/../Octoscript-OH"

or point at an existing one:
  OCTOSCRIPT_OH=/path/to/Octoscript-OH ./build.sh
MSG
  exit 1
fi

if [ ! -d dist ]; then
  echo "no dist/ yet — run: npm install && npm run build" >&2
  exit 1
fi

# OCTOSCRIPT_DEV_SERVER passes straight through, so a live-reload build is
# `OCTOSCRIPT_DEV_SERVER=http://127.0.0.1:5173 ./build.sh` from here too.
exec env OCTOSCRIPT_FRONTEND_DIR="$APP_DIR/dist" "$OCTOSCRIPT_OH/build.sh" "$@"
