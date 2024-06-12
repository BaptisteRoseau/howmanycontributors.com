# Manifests

This directory contains manifests regarding production and infrastructure as code for the benchmark website.

It contains:

- Kubernetes manifest files
- Helm charts
- Docker image registry files
- Dockerfiles

## Containers

Container manifests are stored in the [dockerfiles](./dockerfiles/) directory.

## File pattern

The files names follow this pattern:

```text
Dockerfile.[debug|release].<service name>
```

For example `Dockerfile.release.backend` is the release image manifest of the backend service, supposed to run in production.

## Building

To build the images locally using Docker, use the [build.sh](./dockerfiles/build.sh).

You can also build the images by hand, from the git top level directory:

```bash
GIT_ROOT=$(git rev-parse --show-toplevel)
docker build --file "$GIT_ROOT/manifests/dockerfiles/Dockerfile.debug.backend" -t backend:0.0.0-debug "$GIT_ROOT"
```
