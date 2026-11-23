#  Portable Zero-Key Frontier Models Proxy🦀

Route **Claude Code CLI** and **OpenAI Codex CLI** directly through your **Google Gemini** account (Gemini Pro / Free quota) with **zero paid API keys, zero rate-limit 429 cooling downs, and sub-millisecond local proxy latency**.

Includes `proxy.exe` — a standalone, sub-millisecond management tool written in pure Rust with zero external dependencies and 0 MB idle RAM overhead.

---

## ✨ Features

- 🆓 **Zero Paid API Keys**: No Anthropic or OpenAI credit cards or API subscriptions required.
- ⚡ **Google Gemini 3 Flash / 3.8**: Maps all Claude (`claude-3-5-sonnet`, `claude-3-7-sonnet`, `claude-opus`) and OpenAI (`gpt-4o`, `o3-mini`, `o1`, `gpt-5`) requests to high-speed Gemini endpoints.
- 🛡️ **Zero Token Leaks**: Strictly isolates credentials locally. Personal OAuth tokens are saved in `.gitignore`'d local directories and are never committed.
- 🦀 **Ultra-Lightweight Rust Manager**: `proxy.exe` is under 400 KB, compiles with zero external crates, and provides sub-millisecond health pings and background process management.

---

## 🚀 Quick Start (For You and Your Friends)

### 1. Download or Clone
```bash
git clone https://github.com/Chamod88/gemini-cli-proxy.git
cd gemini-cli-proxy
```

### 2. Connect Your Google Account(s)
Double-click or run:
```powershell
.\scripts\login-claude.cmd   # Opens browser to log into Google for Claude Code (Port 8317)
.\scripts\login-codex.cmd    # Opens browser to log into Google for OpenAI Codex (Port 8318)
```
*(You can use the same Google account for both, or two different Google accounts for isolated quotas!)*

### 3. One-Click CLI Setup
Double-click or run:
```powershell
.\scripts\setup-agents.cmd
```
This automatically sets your Windows environment variables (`ANTHROPIC_BASE_URL`, `OPENAI_BASE_URL`) and configures `~/.codex/config.toml`.

### 4. Start the Proxies & Code!
```powershell
.\start.cmd
```
or run `.\bin\proxy.exe start`.

Now open any terminal and use your agents:
```powershell
claude -p "Explain quantum computing in 3 bullets"
codex exec "Write a fast binary search algorithm in Rust"
```

---

## 🛠️ Proxy CLI Commands

Add `.\bin` to your `PATH` (or use `.\bin\proxy.exe`):

```powershell
proxy status          # Show live proxy states, ports, and latency
proxy start           # Start Claude (8317) and Codex (8318) in background
proxy stop            # Stop all proxy processes
proxy restart         # Restart both proxies cleanly
proxy test            # Fast sub-millisecond HTTP ping test
proxy login [target]  # Log into Google (claude or codex)
proxy open            # Open Web Management Center in browser
```

---

## 🏗️ Architecture & Model Mapping

```
                        ┌───────────────────────────────┐
                        │   Claude Code CLI (claude)    │
                        └──────────────┬────────────────┘
                                       │ (Port 8317)
                                       ▼
    ┌────────────────────────────────────────────────────────────────────────┐
    │                       Local CLI Proxy (Port 8317)                      │
    │  - Translates Anthropic Messages API -> Google Gemini Protocol         │
    │  - claude-3-5-sonnet-20241022  ──►  gemini-3-flash                     │
    │  - claude-3-7-sonnet-20250219  ──►  gemini-3-flash                     │
    └──────────────────────────────────┬─────────────────────────────────────┘
                                       │
                                       ▼
                           Google Gemini Cloud API
                                       ▲
                                       │
    ┌──────────────────────────────────┴─────────────────────────────────────┐
    │                       Local CLI Proxy (Port 8318)                      │
    │  - Translates OpenAI Responses / Chat API -> Google Gemini Protocol    │
    │  - gpt-4o / o3-mini / o1 / gpt-5 ──►  gemini-3-flash                   │
    └──────────────────────────────────┬─────────────────────────────────────┘
                                       │ (Port 8318)
                        ┌──────────────┴────────────────┐
                        │    OpenAI Codex CLI (codex)   │
                        └───────────────────────────────┘
```

---

## 🔒 Security Notice

- Your Google refresh and access tokens are stored strictly on your local disk in `auth/` (which is excluded by `.gitignore`).
- **Never share or push your `auth/` directory or `*.json` token files to Git.**

---

## 📄 License
MIT License. Created by [Chamod88](https://github.com/Chamod88).
