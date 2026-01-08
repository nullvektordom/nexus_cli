//! Diagnose Command - Check LLM configuration and connectivity

use crate::config::NexusConfig;
use crate::llm::{LlmClient, LlmProvider};
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

/// Execute the diagnose command - check LLM setup
pub fn execute(project_path: &Path) -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║   🔍 Nexus LLM Diagnostics                        ║".cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".cyan());
    println!();

    // Load config
    let config_path = project_path.join("nexus.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    println!("{}", "1. Checking LLM Configuration...".bold());
    println!();

    // Check 1: LLM config exists
    let llm_config = match config.llm.as_ref() {
        Some(cfg) => {
            println!("  {} LLM section found in nexus.toml", "✓".green().bold());
            println!("    Provider: {}", cfg.provider.cyan());
            println!("    Model: {}", cfg.model.cyan());
            println!("    Enabled: {}", if cfg.enabled { "true".green() } else { "false".red() });
            cfg
        }
        None => {
            println!("  {} LLM section NOT found in nexus.toml", "✗".red().bold());
            println!();
            println!("  Add this to your nexus.toml:");
            println!("  {}", "[llm]".yellow());
            println!("  {}", "provider = \"gemini\"".yellow());
            println!("  {}", "model = \"gemini-3-pro\"".yellow());
            println!("  {}", "enabled = true".yellow());
            return Ok(());
        }
    };
    println!();

    if !llm_config.enabled {
        println!("  {} LLM is disabled", "⚠".yellow().bold());
        println!("  Set 'enabled = true' in [llm] section");
        return Ok(());
    }

    // Check 2: API key
    println!("{}", "2. Checking API Key...".bold());
    println!();

    let (env_var, api_key) = match llm_config.provider.as_str() {
        "openrouter" => {
            let key = std::env::var("OPENROUTER_API_KEY")
                .or_else(|_| llm_config.api_key.clone().ok_or_else(|| anyhow::anyhow!("")));
            ("OPENROUTER_API_KEY", key)
        }
        "claude" | "anthropic" => {
            let key = std::env::var("ANTHROPIC_API_KEY")
                .or_else(|_| llm_config.api_key.clone().ok_or_else(|| anyhow::anyhow!("")));
            ("ANTHROPIC_API_KEY", key)
        }
        "gemini" | "google" => {
            let key = std::env::var("GOGGLE_AI_STUDIO_API_KEY")
                .or_else(|_| llm_config.api_key.clone().ok_or_else(|| anyhow::anyhow!("")));
            ("GOGGLE_AI_STUDIO_API_KEY", key)
        }
        _ => {
            println!("  {} Unknown provider: {}", "✗".red().bold(), llm_config.provider);
            return Ok(());
        }
    };

    match api_key {
        Ok(ref key) => {
            println!("  {} API key found", "✓".green().bold());
            let masked = if key.len() > 8 {
                format!("{}...{}", &key[..4], &key[key.len() - 4..])
            } else {
                "***".to_string()
            };
            println!("    Source: {}", if std::env::var(env_var).is_ok() {
                format!("Environment variable ({})", env_var).cyan()
            } else {
                "Config file".cyan()
            });
            println!("    Value: {}", masked.dimmed());
        }
        Err(_) => {
            println!("  {} API key NOT found", "✗".red().bold());
            println!();
            println!("  Set your API key:");
            println!("    export {}=\"your-key-here\"", env_var.yellow());
            println!("  Or add it to nexus.toml:");
            println!("    {}", "[llm]".yellow());
            println!("    {}", format!("api_key = \"your-key\"").yellow());
            return Ok(());
        }
    }
    println!();

    // Check 3: Network connectivity
    println!("{}", "3. Checking Network Connectivity...".bold());
    println!();

    let test_url = match llm_config.provider.as_str() {
        "gemini" | "google" => "https://generativelanguage.googleapis.com",
        "claude" | "anthropic" => "https://api.anthropic.com",
        "openrouter" => "https://openrouter.ai",
        _ => "",
    };

    println!("  Testing connection to: {}", test_url.cyan());

    let connectivity_result = std::process::Command::new("curl")
        .args(["-I", "-s", "-o", "/dev/null", "-w", "%{http_code}", test_url, "--max-time", "5"])
        .output();

    match connectivity_result {
        Ok(output) => {
            let status_code = String::from_utf8_lossy(&output.stdout);
            if status_code.starts_with('2') || status_code.starts_with('3') || status_code == "401" || status_code == "403" {
                println!("  {} Network connection successful (HTTP {})", "✓".green().bold(), status_code.trim());
            } else if status_code.is_empty() {
                println!("  {} Network connection failed (timeout or unreachable)", "✗".red().bold());
                println!("  Check your internet connection and firewall settings");
            } else {
                println!("  {} Unexpected HTTP status: {}", "⚠".yellow().bold(), status_code.trim());
            }
        }
        Err(_) => {
            println!("  {} curl command not available, skipping network test", "⚠".yellow().bold());
            println!("  Install curl to enable network diagnostics");
        }
    }
    println!();

    // Check 4: Test LLM call
    println!("{}", "4. Testing LLM API Call...".bold());
    println!();

    if let Ok(key) = api_key {
        let provider = LlmProvider::from_str(&llm_config.provider)
            .ok_or_else(|| anyhow::anyhow!("Invalid provider"))?;

        let client = LlmClient::new(provider, key, llm_config.model.clone());

        println!("  Sending test request (this may take 5-10 seconds)...");

        let runtime = tokio::runtime::Runtime::new()?;
        let result = runtime.block_on(async {
            client.complete_with_system(
                "You are a helpful assistant.",
                "Reply with exactly: 'Hello from Nexus!' (no extra text)"
            ).await
        });

        match result {
            Ok(response) => {
                println!("  {} LLM responded successfully", "✓".green().bold());
                println!("    Response: {}", response.trim().dimmed());
                println!();
                println!("{}", "═══════════════════════════════════════".green());
                println!("{}", "  ✅ All checks passed!".green().bold());
                println!("{}", "═══════════════════════════════════════".green());
                println!();
                println!("Your LLM is configured correctly and ready to use.");
            }
            Err(e) => {
                println!("  {} LLM call failed", "✗".red().bold());
                println!();
                println!("  Error details:");
                println!("    {}", format!("{:?}", e).red());
                println!();
                println!("  Common causes:");
                println!("    • Invalid API key");
                println!("    • Incorrect model name");
                println!("    • API rate limiting");
                println!("    • Insufficient API credits/quota");
                println!("    • Provider service outage");
            }
        }
    }
    println!();

    Ok(())
}
