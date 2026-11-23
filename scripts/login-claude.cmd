@echo off
setlocal
echo ===================================================
echo   Google Login for Claude Code (Port 8317)
echo ===================================================
echo A browser window will open. Sign in with your Google account.
echo.
"%~dp0bin\cli-proxy-api.exe" -config "%~dp0configs\config-claude.yaml" -antigravity-login
echo.
echo Login completed. Tokens saved safely to auth/claude.
pause
