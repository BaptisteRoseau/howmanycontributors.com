#!/bin/bash
dx serve --bin frontend --port 8080 --hot-reload &
npx tailwindcss -i ./input.css -o ./dist/tailwind.css --watch

# Required for running in docker-compose,
# else the previous command returns immediately.
sleep 9999999999999999999
