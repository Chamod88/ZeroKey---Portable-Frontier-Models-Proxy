@echo off
setlocal
echo ===================================================
echo   Configuring Claude Code & OpenAI Codex CLI
echo   Default Model: Gemini 3.8 Flash (High Reasoning)
echo ===================================================

echo [1/2] Setting User Environment Variables for Claude Code...
setx ANTHROPIC_BASE_URL "http://127.0.0.1:8317" >nul
setx ANTHROPIC_AUTH_TOKEN "sk-gemini-proxy-key" >nul
setx ANTHROPIC_API_KEY "sk-gemini-proxy-key" >nul

echo [2/2] Setting User Environment Variables for OpenAI Codex...
setx OPENAI_BASE_URL "http://127.0.0.1:8318/v1" >nul
setx OPENAI_API_KEY "sk-gemini-proxy-key" >nul

if not exist "%USERPROFILE%\.codex" mkdir "%USERPROFILE%\.codex"
(
echo model = "gemini-3.8-flash-high"
echo model_provider = "gemini_proxy"
echo openai_base_url = "http://127.0.0.1:8318/v1"
echo model_reasoning_effort = "high"
echo.
echo [model_providers.gemini_proxy]
echo name = "gemini_proxy"
echo base_url = "http://127.0.0.1:8318/v1"
echo wire_api = "responses"
echo supports_websockets = false
echo requires_openai_auth = true
) > "%USERPROFILE%\.codex\config.toml"

echo.
echo ===================================================
echo   Configuration Complete!
echo   Default: Gemini 3.8 Flash (High Reasoning)
echo   Restart your open terminal windows to apply.
echo ===================================================
pause
