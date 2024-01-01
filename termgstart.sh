#!/bin/bash
echo [*] Trying to start terminal
app_name=app
default_name=terminal
if [ -f "$app_name" ]; then
    exists=true
    name=app
else
    if [ -f "$default_name" ]; then
        exists=true
        name=terminal
    else
        exists=false
    fi
fi

if [ "$exists" = false ]; then
    echo [-] Terminal failed
    echo [+] Some files could be missing
else
    chmod +x termgstart.sh
    echo [*] Terminal successfully opened
    echo ""
    ./app
fi