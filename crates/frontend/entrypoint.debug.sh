#!/bin/bash
while true; do dx serve --bin frontend --port 8080 --hot-reload; done &
while true; do npx tailwindcss -i ./input.css -o ./dist/tailwind.css --watch; done &

# Required for running in docker-compose,
# else the previous command returns immediately.
sleep 9999999999999999999
