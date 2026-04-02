#!/usr/bin/env sh
set -eu

exec env -u NO_COLOR trunk "$@"
