@echo off

cd /D "%~dp0"

cd ..\automated-kingdom\
cargo build -r

cd ..\launcher\
node zip.js
timeout 1
cargo build -r

echo Finished building
