#!/usr/bin/env bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
VERSION=${VERSION:='1.0.2'}
TARGET=${TARGET:=x86_64-unknown-linux-gnu}
ENGINE=podman

"$ENGINE" build \
    --file "$SCRIPT_DIR/Dockerfile.release" \
    --build-arg target="$TARGET" \
    --format docker \
    -t frontend:"$VERSION" \
    -t frontend:latest \
    "$(git rev-parse --show-toplevel)"

"$ENGINE" build \
    --file "$SCRIPT_DIR/Dockerfile.debug" \
    --build-arg target="$TARGET" \
    --format docker \
    -t frontend:"$VERSION-debug" \
    -t frontend:latest-debug \
    "$(git rev-parse --show-toplevel)"
