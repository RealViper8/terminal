@echo off

echo [*] Trying to start terminal

set app_name="app.exe"

if exist %app_name% (
    set exists="true"
    set name=app
) else (
    if exist "terminal.exe" (
        set exists="true"
        set name=terminal
    ) else (
        set exists="false"
    )
)

if %exists% == "false" (
    echo [-] Terminal failed
    echo [+] Some files could be missing in %cd%
) else (
    start %name%
    echo [*] Terminal successfully opened
)