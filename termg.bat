@echo off

title Loading

FOR /F "tokens=3" %%a IN ('reg query "HKCU\Control Panel\Desktop" /v PreferredUILanguages ^| find "PreferredUILanguages"') DO set UILanguage=%%a

set app_name="app.exe"
set first_title=tasklist /fi "imagename eq cmd.exe" /fo list /v | find "itel"

if exist %app_name% (
    %app_name%
    cls

    if %UILanguage% == de-DE (
        echo Language is : DE
        title %first_title%
    ) else (
        title %first_title%
    )

    color 07
) else (
    if exist "terminal.exe" (
         terminal
    ) else (
        title Command Prompt
        echo [-] Terminal failed
        echo [+] Some files are missing in %cd%
    )
)
