#!/bin/bash
while true; do dx serve --bin frontend --port 8080 --hot-reload; done &
while true; do npx @tailwindcss/cli -i ./input.css -o ./dist/tailwind.css --watch; done
