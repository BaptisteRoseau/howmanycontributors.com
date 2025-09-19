#!/bin/bash
while true; do dx serve --bin frontend --port 8080 --hot-reload; done &
npm install tailwindcss
while true; do tailwindcss -i ./input.css -o ../../target/dx/frontend/debug/web/public/assets/tailwind.css --watch=always; done
