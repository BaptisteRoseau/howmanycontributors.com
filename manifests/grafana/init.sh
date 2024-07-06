#!/bin/bash

#TODO: Use CLI to install plugins, create users, dashboards and alerts
# https://grafana.com/docs/grafana/latest/cli/

PLUGINS=(
    "redis-app"
    "redis-datasource"
    "redis-explorer-app"
    "blackcowmoo-googleanalytics-datasource"
)

for plugin in "${PLUGINS[@]}"
do
    grafana cli plugins install "$plugin"
done
