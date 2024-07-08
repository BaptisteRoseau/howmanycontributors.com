#!/usr/bin/env bash
set -e

# TODO:
# - CLI commands and arguments "$0 import/exort" "--filter FILTER --verbose --quiet"


function log() {
    echo "[$(date +"%Y-%m-%dT%H:%M:%S%z")]" "$@"
}

function exit_on_missing_commands() {
    if ! type date mkdir mktemp podman rm tar >/dev/null; then
        log "Aborting" >&2
        exit 1
    fi
}

function export_to() {
    to="$1"
    filter="${2:-*}"
    log "Exporting volumes to $to"

    tempdir=$(mktemp -d)
    mkdir -p "$tempdir/volumes"
    for volume in $(podman volume ls -q); do
        if [[ "$volume" =~ $filter ]]; then
            log "Exporting volume $volume..."
            podman volume export "$volume" -o "$tempdir/volumes/${volume}.tar"
            tar --remove-files \
                -czf "$tempdir/volumes/${volume}.tar.gz" \
                -C "$tempdir/volumes" \
                "${volume}.tar"
        fi
    done
    log "Building final archive..."
    tar -czf "$to" -C "$tempdir" volumes
    rm -r "$tempdir"
    log "Export done to $to"
}

function import_from() {
    from="$1"
    filter="${2:-*}"
    log "Importing volumes from $from"

    tempdir=$(mktemp -d)
    tar xzf "$from" -C "$tempdir"
    for volume_file in "$tempdir"/volumes/*; do
        volume=${volume_file//.tar.gz/}
        volume=${volume//$tempdir\/volumes\//}
        if [[ "$volume" =~ $filter ]]; then
            log "Importing volume $volume..."
            tar xzf "$volume_file" -C "$tempdir"
            podman volume import "$volume" "$tempdir/$volume.tar"
            rm "$tempdir/$volume.tar"
        fi
    done
    rm -r "$tempdir"
    log "Import done from $from"
}

exit_on_missing_commands
