#!/usr/bin/env bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
VERSION=${VERSION:='0.0.0'}
TARGET=${TARGET:=x86_64-unknown-linux-gnu}

podman build \
    --file "$SCRIPT_DIR/Dockerfile.release" \
    --build-arg target="$TARGET" \
    -t backend:"$VERSION" \
    -t backend:latest \
    "$(git rev-parse --show-toplevel)"

podman build \
    --file "$SCRIPT_DIR/Dockerfile.debug" \
    --build-arg target="$TARGET" \
    -t backend:"$VERSION-debug" \
    -t backend:latest-debug \
    "$(git rev-parse --show-toplevel)"
