use std::env;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

const CLAUDE_PORT: u16 = 8317;
const CODEX_PORT: u16 = 8318;
const API_KEY: &str = "sk-gemini-proxy-key";

fn find_base_dir() -> PathBuf {
    // 1. Check folder containing proxy.exe
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            if dir.join("cli-proxy-api.exe").exists() {
                return dir.to_path_buf();
            }
            if let Some(parent) = dir.parent() {
                if parent.join("cli-proxy-api.exe").exists() {
                    return parent.to_path_buf();
                }
            }
        }
    }
    // 2. Check current working directory
    if let Ok(cwd) = env::current_dir() {
        if cwd.join("cli-proxy-api.exe").exists() {
            return cwd;
        }
    }
    PathBuf::from(".")
}

fn resolve_path(base: &Path, rel: &str) -> PathBuf {
    let p = base.join(rel);
    if p.exists() {
        p
    } else {
        base.join("configs").join(rel)
    }
}

fn is_port_listening(port: u16) -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    TcpStream::connect_timeout(&addr, Duration::from_millis(300)).is_ok()
}

fn raw_http_ping(port: u16) -> Option<(u16, u128)> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let start = Instant::now();
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(1500)).ok()?;

    let req = format!(
        "GET /v1/models HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nAuthorization: Bearer {}\r\nConnection: close\r\n\r\n",
        port, API_KEY
    );

    stream.write_all(req.as_bytes()).ok()?;
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf).ok()?;
    let elapsed = start.elapsed().as_millis();

    let resp = String::from_utf8_lossy(&buf[..n]);
    if resp.starts_with("HTTP/1.") {
        let parts: Vec<&str> = resp.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(code) = parts[1].parse::<u16>() {
                return Some((code, elapsed));
            }
        }
    }
    None
}

fn status() {
    println!("\n  ================================================================");
    println!("     GEMINI AGENT PROXY STATUS (Claude Code & OpenAI Codex)       ");
    println!("  ================================================================");
    println!("  {:<10} {:<6} {:<15} {}", "AGENT", "PORT", "STATE", "TARGET SERVICE");
    println!("  ----------------------------------------------------------------");

    // 1. Antigravity CLI
    println!(
        "  {:<10} {:<6} {:<15} {}",
        "agy", "Direct", "[OK]", "Google Antigravity (Native)"
    );

    // 2. Claude Code
    let claude_state = if is_port_listening(CLAUDE_PORT) {
        if let Some((_, ms)) = raw_http_ping(CLAUDE_PORT) {
            format!("[ONLINE] ({}ms)", ms)
        } else {
            "[ONLINE]".to_string()
        }
    } else {
        "[OFFLINE]".to_string()
    };
    println!(
        "  {:<10} {:<6} {:<15} {}",
        "claude", CLAUDE_PORT, claude_state, "Gemini 3.8 Flash (High Reasoning)"
    );

    // 3. Codex CLI
    let codex_state = if is_port_listening(CODEX_PORT) {
        if let Some((_, ms)) = raw_http_ping(CODEX_PORT) {
            format!("[ONLINE] ({}ms)", ms)
        } else {
            "[ONLINE]".to_string()
        }
    } else {
        "[OFFLINE]".to_string()
    };
    println!(
        "  {:<10} {:<6} {:<15} {}",
        "codex", CODEX_PORT, codex_state, "Gemini 3.8 Flash (High Reasoning)"
    );

    println!("  ================================================================\n");
}

fn start() {
    let base = find_base_dir();
    let proxy_exe = base.join("cli-proxy-api.exe");
    if !proxy_exe.exists() {
        eprintln!("  [-] Error: Cannot find cli-proxy-api.exe in {}", base.display());
        return;
    }

    let claude_cfg = resolve_path(&base, "config-claude.yaml");
    let codex_cfg = resolve_path(&base, "config-codex.yaml");

    println!("\n  Starting Gemini Agent Proxies in background...");

    // Start Claude proxy (8317)
    if is_port_listening(CLAUDE_PORT) {
        println!("  [OK] Claude proxy (port 8317) is already running.");
    } else {
        match Command::new(&proxy_exe)
            .args(["-config", claude_cfg.to_str().unwrap_or("config-claude.yaml")])
            .current_dir(&base)
            .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
            .spawn()
        {
            Ok(_) => println!("  [+] Launched Claude proxy (127.0.0.1:8317) -> Gemini 3 Flash"),
            Err(e) => eprintln!("  [-] Failed to start Claude proxy: {}", e),
        }
    }

    // Start Codex proxy (8318)
    if is_port_listening(CODEX_PORT) {
        println!("  [OK] Codex proxy (port 8318) is already running.");
    } else {
        match Command::new(&proxy_exe)
            .args(["-config", codex_cfg.to_str().unwrap_or("config-codex.yaml")])
            .current_dir(&base)
            .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
            .spawn()
        {
            Ok(_) => println!("  [+] Launched Codex proxy (127.0.0.1:8318) -> Gemini 3 Flash"),
            Err(e) => eprintln!("  [-] Failed to start Codex proxy: {}", e),
        }
    }

    std::thread::sleep(Duration::from_millis(800));
    status();
}

fn stop() {
    println!("\n  Stopping Gemini Agent Proxies...");
    let output = Command::new("taskkill")
        .args(["/F", "/IM", "cli-proxy-api.exe"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("  [OK] Stopped all proxy processes.");
            } else {
                let err_str = String::from_utf8_lossy(&out.stderr);
                if err_str.contains("not found") {
                    println!("  [i] No proxy processes were running.");
                } else {
                    println!("  [-] {}", err_str.trim());
                }
            }
        }
        Err(e) => eprintln!("  [-] Failed to run taskkill: {}", e),
    }
    status();
}

fn test_connections() {
    println!("\n  Testing Proxy Latency and Health...");
    println!("  ----------------------------------------------------------------");

    // Ping Claude
    print!("  Claude Proxy (127.0.0.1:{}) ... ", CLAUDE_PORT);
    let _ = std::io::stdout().flush();
    if let Some((code, ms)) = raw_http_ping(CLAUDE_PORT) {
        println!("ONLINE (HTTP {}, {} ms)", code, ms);
    } else {
        println!("OFFLINE (Run 'proxy start' to start)");
    }

    // Ping Codex
    print!("  Codex Proxy  (127.0.0.1:{}) ... ", CODEX_PORT);
    let _ = std::io::stdout().flush();
    if let Some((code, ms)) = raw_http_ping(CODEX_PORT) {
        println!("ONLINE (HTTP {}, {} ms)", code, ms);
    } else {
        println!("OFFLINE (Run 'proxy start' to start)");
    }

    println!("  ----------------------------------------------------------------\n");
}

fn login(target: &str) {
    let base = find_base_dir();
    let proxy_exe = base.join("cli-proxy-api.exe");
    if !proxy_exe.exists() {
        eprintln!("  [-] Error: Cannot find cli-proxy-api.exe in {}", base.display());
        return;
    }

    let cfg = match target {
        "claude" => resolve_path(&base, "config-claude.yaml"),
        "codex" => resolve_path(&base, "config-codex.yaml"),
        _ => {
            eprintln!("  Usage: proxy login [claude|codex]");
            return;
        }
    };

    println!("\n  Opening browser to log into Google for {}...", target);
    let _ = Command::new(&proxy_exe)
        .args(["-config", cfg.to_str().unwrap_or(""), "-antigravity-login"])
        .current_dir(&base)
        .status();
}

fn open_panel() {
    println!("  Opening Web Management Center in browser...");
    let _ = Command::new("cmd")
        .args(["/c", "start", "http://localhost:8317/management.html"])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();
}

fn print_help() {
    println!("\n  GEMINI CLI PROXY MANAGER");
    println!("  ================================================================");
    println!("  COMMANDS:");
    println!("    proxy start           Start proxies (Claude: 8317, Codex: 8318)");
    println!("    proxy status          Show current agent status, ports, and latency");
    println!("    proxy stop            Stop all running proxies");
    println!("    proxy restart         Restart both proxies cleanly");
    println!("    proxy test            Fast sub-millisecond HTTP ping test");
    println!("    proxy login [target]  Log in with Google (target: claude or codex)");
    println!("    proxy open            Open Web Management Center in browser\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        status();
        print_help();
        return;
    }

    match args[1].to_lowercase().as_str() {
        "status" => status(),
        "start" => start(),
        "stop" => stop(),
        "restart" => {
            stop();
            std::thread::sleep(Duration::from_millis(500));
            start();
        }
        "test" | "ping" => test_connections(),
        "login" | "auth" => {
            let target = if args.len() > 2 { args[2].as_str() } else { "claude" };
            login(target);
        }
        "open" | "web" | "ui" => open_panel(),
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("  Unknown command: '{}'\n", other);
            print_help();
        }
    }
}
