#!/usr/bin/env bash
GIT_ROOT=$(git rev-parse --show-toplevel)
ENGINE=docker

VERSION=$(grep 'version' crates/frontend/Cargo.toml | head -n 1 | cut -d ' ' -f 3 | sed -e 's/^.//' -e 's/.$//')
"$ENGINE" build \
    --file "crates/frontend/Dockerfile.release" \
    --build-arg target="$TARGET" \
    -t hmc-frontend:"$VERSION" \
    -t hmc-frontend:latest \
    "$GIT_ROOT"
docker save hmc-frontend:"$VERSION" | gzip > "$GIT_ROOT/hmc-frontend-$VERSION.tar.gz"
echo "Built hmc-frontend-$VERSION.tar.gz"

VERSION=$(grep 'version' crates/backend/Cargo.toml | head -n 1 | cut -d ' ' -f 3 | sed -e 's/^.//' -e 's/.$//')
"$ENGINE" build \
    --file "crates/backend/Dockerfile.release" \
    --build-arg target="$TARGET" \
    -t hmc-backend:"$VERSION" \
    -t hmc-backend:latest \
    "$GIT_ROOT"
docker save hmc-backend:"$VERSION" | gzip > "$GIT_ROOT/hmc-backend-$VERSION.tar.gz"
echo "Built hmc-backend-$VERSION.tar.gz"
