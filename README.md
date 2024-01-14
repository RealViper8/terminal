# terminal
A Terminal for windows, macOS, linux written in Rust to reset the configs just delete the config.ini and then start the terminal again

Latest Version:

![Latest](https://raw.githubusercontent.com/RealViper8/terminal/master/img/latest.png)

Configurable terminal

<img width="155" alt="config" src="https://github.com/RealViper8/terminal/assets/101727162/31023325-1883-4137-bfd6-4412c2a37f2f">

# Configurations
if you want to configure it manually then go in config.ini
, if you dont have this file then just run the terminal one time

[terminal]   => is the category

path=/bin   => is the option

color=1     => set the default color when you open the terminal

ls=list2
OR
ls=list

[app]

debug=off => set on if you have a error and want a detailed look on the issue

# Setup
If you dont want to compile it yourself then you can get the release
for linux, windows and macOS if you have installed the release follow this
steps to open 

For linux and macOS:
  1. chmod +x app
  2. ./app
     
  OR
  sh termgstart.sh

and for windows just do ./app or open the termgstart.bat

[Releases](https://github.com/RealViper8/terminal/releases)
