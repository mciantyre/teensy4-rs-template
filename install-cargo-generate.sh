#!/usr/bin/env bash

set -euf -o pipefail

PACKAGE=cargo-generate-v0.5.0-x86_64-unknown-linux-musl

curl -LJO https://github.com/ashleygwilliams/cargo-generate/releases/latest/download/${PACKAGE}.tar.gz

tar -xvf ${PACKAGE}.tar.gz

cp ${PACKAGE}/cargo-generate $(dirname $(which cargo))

cargo generate --help

