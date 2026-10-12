@echo off
setlocal
cd /d "%~dp0"

cargo run --release --quiet -- %*
exit /b %ERRORLEVEL%