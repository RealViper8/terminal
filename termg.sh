#!/bin/bash

app_name="app"
default_name="terminal"
echo -e '\033k'Loading'\033\\'
clear

if [ -f "$app_name" ]; then
    chmod +x app
    ./app
else
    if [ -f "$default_name" ]; then
        chmod +x terminal
        ./terminal
    fi
fi