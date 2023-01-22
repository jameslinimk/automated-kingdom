@echo off

cd /D "%~dp0"

cd ..\automated-kingdom\
cargo build -r

cd ..\launcher\
node zip.js
ping -n 1 127.0.0.1
cargo build -r

echo Finished building
