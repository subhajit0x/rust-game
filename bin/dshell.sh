#!/usr/bin/env bash
set -e

source "$(dirname "$(realpath "$0")")/helpers.sh"
cd "${ROOT_DIR}"

docker-compose run --rm --entrypoint bash app