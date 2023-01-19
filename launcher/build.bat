@echo off

cd /D "%~dp0"

cd ../automated_kingdom
cargo build -r

cd ../launcher
node zip.js
cargo build -r

echo Finished building
