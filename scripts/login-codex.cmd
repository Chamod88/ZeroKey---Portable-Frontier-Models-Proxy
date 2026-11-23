@echo off
setlocal
echo ===================================================
echo   Google Login for OpenAI Codex (Port 8318)
echo ===================================================
echo A browser window will open. Sign in with your Google account.
echo.
"%~dp0bin\cli-proxy-api.exe" -config "%~dp0configs\config-codex.yaml" -antigravity-login
echo.
echo Login completed. Tokens saved safely to auth/codex.
pause
