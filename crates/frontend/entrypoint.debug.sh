#!/bin/bash
while true; do dx serve --bin frontend --port 8080 --hot-reload true; done &
while true; do /root/.bun/bin/bunx @tailwindcss/cli -i ./input.css -o ./dist/tailwind.css --watch; done
