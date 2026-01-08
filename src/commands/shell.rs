//! Shell Command - Interactive REPL for Nexus
//!
//! Provides a persistent REPL that allows running Nexus commands interactively.
//! Maintains session state for cross-computer handover.
//! Supports multi-project workflows with the `use <project>` command.

use crate::brain::NexusBrain;
use crate::config::NexusConfig;
use crate::state::NexusState;
use crate::watcher::SentinelWatcher;
use anyhow::{Context, Result};
use colored::Colorize;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::sync::{Arc, Mutex};

/// Global state file path (not project-specific)
const GLOBAL_STATE_FILE: &str = "/home/nullvektor/.config/nexus/session.json";

/// Execute the shell command - starts an interactive REPL
pub fn execute() -> Result<()> {
    // Load or create global session state
    let state_file = std::path::PathBuf::from(GLOBAL_STATE_FILE);
    let mut state = NexusState::load(&state_file)?;

    // Save initial state
    state.save(&state_file)?;

    // Initialize embeddings (local ONNX model for semantic search)
    let (model_path, tokenizer_path) = find_model_paths();
    if let Err(e) = crate::embeddings::initialize_embeddings(&model_path, &tokenizer_path) {
        eprintln!("{}", "Warning: Failed to initialize embeddings:".yellow());
        eprintln!("  {e}");
        eprintln!("{}", "  Semantic search will be degraded. Planning Catalyst features will use zero vectors.".yellow());
        eprintln!();
    } else {
        let msg = format!("✓ Embeddings initialized ({})", model_path);
        eprintln!("{}", msg.green());
        eprintln!();
    }

    // Print welcome banner
    print_banner(&state)?;

    // Initialize watcher (disabled by default, enabled with 'watch' command)
    let watcher: Arc<Mutex<Option<SentinelWatcher>>> = Arc::new(Mutex::new(None));
    let watcher_enabled = Arc::new(Mutex::new(false));

    // Track last gate error for "why" command
    let last_gate_error: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    // Context injection toggle (enabled by default)
    let context_enabled = Arc::new(Mutex::new(true));

    // Initialize REPL
    let mut rl = DefaultEditor::new().context("Failed to initialize readline editor")?;

    // REPL loop
    loop {
        // Dynamic prompt based on active project, watcher, and context status
        let prompt = if let Some(ref project) = state.active_project_id {
            let watch_indicator = if *watcher_enabled.lock().unwrap() {
                "👁"
            } else {
                ""
            };
            let context_indicator = if *context_enabled.lock().unwrap() {
                "🧠"
            } else {
                ""
            };
            
            // Check if adhoc mode
            let mode_indicator = if is_active_project_adhoc(&state) {
                "📌"
            } else {
                "🏃"
            };

            format!(
                "{} ",
                format!("nexus:{project}{mode_indicator}{watch_indicator}{context_indicator}❯")
                    .cyan()
                    .bold()
            )
        } else {
            format!("{} ", "nexus❯".cyan().bold())
        };

        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let line = line.trim();

                // Skip empty lines
                if line.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(line);

                // Update state timestamp
                state.touch();
                state.save(&state_file)?;

                // Parse and execute command
                if let Err(e) = execute_command(
                    line,
                    &mut state,
                    &watcher,
                    &watcher_enabled,
                    &last_gate_error,
                    &context_enabled,
                ) {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                }

                // Save state after command execution
                state.save(&state_file)?;
            }
            Err(ReadlineError::Interrupted) => {
                println!(
                    "{}",
                    "^C - Use 'exit' or 'quit' to leave the shell".yellow()
                );
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".green());
                break;
            }
            Err(err) => {
                eprintln!("{} {}", "Error:".red().bold(), err);
                break;
            }
        }
    }

    // Cleanup watcher on exit
    if let Some(w) = watcher.lock().unwrap().take() {
        let _ = w.shutdown();
    }

    Ok(())
}

/// Print the welcome banner with session information
#[allow(clippy::unnecessary_wraps)] // Consistent API, may add validation later
fn print_banner(state: &NexusState) -> Result<()> {
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║        🧠 Nexus Shell - Interactive REPL           ║".cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════╝".cyan()
    );
    println!();
    println!("  {} {}", "Session ID:".bold(), state.session_id.dimmed());
    println!(
        "  {} {}",
        "Obsidian Root:".bold(),
        state.obsidian_vault_root.display().to_string().dimmed()
    );
    println!(
        "  {} {}",
        "Repos Root:".bold(),
        state.repos_root.display().to_string().dimmed()
    );

    if let Some(ref project) = state.active_project_id {
        println!("  {} {}", "Active Project:".bold(), project.green());
    } else {
        println!(
            "  {} {}",
            "Active Project:".bold(),
            "None (use 'use <project>' to select)".yellow()
        );
    }

    println!("  {} {}", "Started:".bold(), state.created_at.dimmed());
    println!();
    println!(
        "{}",
        "Available commands: use, gate, unlock, sprint, plan, catalyst, status, context, help, exit".dimmed()
    );
    println!("{}", "Type 'help' for more information.".dimmed());
    println!(
        "{}",
        "💡 Natural language queries with context injection enabled by default!".dimmed()
    );
    println!();

    Ok(())
}

/// Execute a command within the REPL
fn execute_command(
    input: &str,
    state: &mut NexusState,
    watcher: &Arc<Mutex<Option<SentinelWatcher>>>,
    watcher_enabled: &Arc<Mutex<bool>>,
    last_gate_error: &Arc<Mutex<Option<String>>>,
    context_enabled: &Arc<Mutex<bool>>,
) -> Result<()> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Ok(());
    }

    let command = parts[0].to_lowercase();
    let args = &parts[1..];

    match command.as_str() {
        "help" | "h" | "?" => {
            print_help();
            Ok(())
        }
        "exit" | "quit" | "q" => {
            println!("{}", "Goodbye!".green());
            std::process::exit(0);
        }
        "use" => execute_use(state, args),
        "gate" => execute_gate(state, last_gate_error),
        "unlock" => execute_unlock(state),
        "sprint" => execute_sprint(state, args),
        "task" => execute_task_command(state, args),
        "start" => execute_task_command(state, &["start"]),
        "done" => execute_task_command(state, &["done"]),
        "status" => execute_status(state),
        "catalyst" => execute_catalyst(state, args),
        "plan" => execute_plan(state, args),
        "watch" => execute_watch(state, watcher, watcher_enabled),
        "unwatch" => execute_unwatch(watcher, watcher_enabled),
        "why" => execute_why(state, last_gate_error),
        "context" => execute_context(args, context_enabled),
        "decision" => execute_decision(args, state),
        "clear" | "cls" => {
            print!("\x1B[2J\x1B[1;1H");
            Ok(())
        }
        "pwd" => {
            if let Some(ref project) = state.active_project_id {
                println!("Active project: {}", project.green());
                if let Some(repo_path) = state.get_active_repo_path() {
                    println!("  Repository: {}", repo_path.display());
                }
                if let Some(obsidian_path) = state.get_active_obsidian_path() {
                    println!("  Obsidian: {}", obsidian_path.display());
                }
            } else {
                println!(
                    "{}",
                    "No active project. Use 'use <project>' to select one.".yellow()
                );
            }
            Ok(())
        }
        "init" => execute_init_command(args),
        _ => {
            // Check if LLM is enabled and context is enabled for natural language processing
            let is_context_enabled = *context_enabled.lock().unwrap();

            if is_context_enabled {
                // Treat as natural language query with context injection
                execute_llm_query(input, state)
            } else {
                // Fall back to semantic query (vector search only)
                execute_semantic_query(input, state)
            }
        }
    }
}

/// Print help information
fn print_help() {
    println!("{}", "Nexus Shell Commands:".bold().underline());
    println!();
    println!(
        "  {}  <project>   Select and activate a project",
        "use".cyan()
    );
    println!(
        "  {}         Check if planning documents are complete",
        "gate".cyan()
    );
    println!(
        "  {}       Generate CLAUDE.md from planning documents",
        "unlock".cyan()
    );
    println!(
        "  {}  <N>   Create/switch to sprint N", "sprint".cyan());
    println!(
        "  {}  <start|done> Manage ad-hoc tasks",
        "task".cyan()
    );
    println!(
        "  {}       Check Brain health and memory usage",
        "status".cyan()
    );
    println!(
        "  {}          Show current active project and paths",
        "pwd".cyan()
    );
    println!(
        "  {} <name> [--project] Initialize a new project",
        "init".cyan()
    );
    println!();
    println!(
        "  {}        Start watching for file changes (Sentinel)",
        "watch".cyan()
    );
    println!("  {}      Stop watching for file changes", "unwatch".cyan());
    println!(
        "  {}          Explain last gate failure using Architecture",
        "why".cyan()
    );
    println!();
    println!(
        "  {} <on|off>  Toggle context injection for LLM queries (default: on)",
        "context".cyan()
    );
    println!();
    println!("  {}         Show this help message", "help".cyan());
    println!("  {}        Clear the screen", "clear".cyan());
    println!("  {}  | {}  Exit the shell", "exit".cyan(), "quit".cyan());
    println!();
    println!("{}", "Planning & Genesis:".bold().underline());
    println!();
    println!("  {} <init>      PROJECT GENESIS - Generate foundational docs (02-05)", "plan".cyan());
    println!();
    println!("  {} <cmd>  AI-powered task-scoped document generation", "catalyst".cyan());
    println!("    {} scope      Generate scope document", "catalyst".dimmed());
    println!("    {} stack      Generate tech stack document", "catalyst".dimmed());
    println!("    {} arch       Generate architecture document", "catalyst".dimmed());
    println!("    {} mvp        Generate MVP breakdown document", "catalyst".dimmed());
    println!("    {} generate   Generate all documents (02-05)", "catalyst".dimmed());
    println!();
    println!("{}", "Semantic Search Flags:".bold().underline());
    println!();
    println!("  {} <query>    Search across all projects (bypass project filter)", "--global".cyan());
    println!("  {}    <query>    Search only Architecture and GlobalStandard layers", "--arch".cyan());
    println!();
    println!(
        "{}",
        "  💡 Tip: Any unrecognized input is treated as a semantic search!".dimmed()
    );
    println!(
        "{}",
        "  Example: 'safety rules --arch' or 'error handling --global'".dimmed()
    );
    println!();
    println!("{}", "Planning Catalyst:".bold().underline());
    println!();
    println!(
        "{}",
        "  🧠 Natural language queries use Gemini 3 Pro (gemini-3-pro) by default".dimmed()
    );
    println!(
        "{}",
        "  🔍 Architecture snippets filtered by relevance (score ≥ 0.75)".dimmed()
    );
    println!(
        "{}",
        "  💾 Last 5 conversation turns saved to .nexus_history.json".dimmed()
    );
    println!(
        "{}",
        "  🎯 Local embeddings via ONNX (all-MiniLM-L6-v2) for semantic search".dimmed()
    );
    println!();
}

/// Execute the use command - select and activate a project
fn execute_use(state: &mut NexusState, args: &[&str]) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("Usage: use <project_name>");
    }

    let project_id = args[0];

    println!(
        "{}",
        format!("Selecting project '{project_id}'...").dimmed()
    );

    // Validate that the project exists
    state.validate_project(project_id)?;

    // Set as active project
    state.set_active_project(project_id.to_string());

    println!(
        "{} Project '{}' activated",
        "✓".green().bold(),
        project_id.green()
    );

    if let Some(repo_path) = state.get_active_repo_path() {
        println!(
            "  {} {}",
            "Repository:".bold(),
            repo_path.display().to_string().dimmed()
        );
    }
    if let Some(obsidian_path) = state.get_active_obsidian_path() {
        println!(
            "  {} {}",
            "Obsidian:".bold(),
            obsidian_path.display().to_string().dimmed()
        );
    }

    Ok(())
}

/// Execute the gate command
fn execute_gate(state: &NexusState, last_gate_error: &Arc<Mutex<Option<String>>>) -> Result<()> {
    let project_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    println!("{}", "Running gate...".dimmed());

    match crate::commands::gate::execute(&project_path) {
        Ok(()) => {
            // Clear error on success
            *last_gate_error.lock().unwrap() = None;
            Ok(())
        }
        Err(e) => {
            // Capture error for "why" command
            *last_gate_error.lock().unwrap() = Some(e.to_string());
            Err(e)
        }
    }
}

/// Execute the unlock command
fn execute_unlock(state: &NexusState) -> Result<()> {
    let project_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    println!("{}", "Running unlock...".dimmed());
    crate::commands::unlock::execute(&project_path)
}

/// Execute the sprint command
fn execute_sprint(state: &NexusState, args: &[&str]) -> Result<()> {
    let project_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    if args.is_empty() {
        anyhow::bail!("Usage: sprint <number>");
    }

    let sprint_number: u32 = args[0]
        .parse()
        .context("Sprint number must be a positive integer")?;

    println!(
        "{}",
        format!("Activating sprint {sprint_number}...").dimmed()
    );
    crate::commands::sprint::execute(&project_path, sprint_number)
}

/// Execute the catalyst command - AI-powered planning document generation
fn execute_catalyst(state: &NexusState, args: &[&str]) -> Result<()> {
    use crate::catalyst::CatalystEngine;
    use crate::llm::{LlmClient, LlmProvider};

    // Parse flags
    let mut show_reasoning = false;
    let mut command_args = Vec::new();
    
    for arg in args {
        if *arg == "--show-reasoning" || *arg == "-r" {
            show_reasoning = true;
        } else {
            command_args.push(*arg);
        }
    }

    // Set environment variable for reasoning display
    if show_reasoning {
        unsafe {
            std::env::set_var("CATALYST_SHOW_REASONING", "1");
        }
    }

    let project_id = state
        .active_project_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    let obsidian_path = state
        .get_active_obsidian_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get Obsidian path"))?;

    let repo_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get repo path"))?;

    // Load config
    let config_path = repo_path.join("nexus.toml");
    if !config_path.exists() {
        anyhow::bail!(
            "No nexus.toml found for this project.\nExpected at: {}",
            config_path.display()
        );
    }

    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    // Verify LLM is configured
    let llm_config = config.llm.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "LLM not configured. Add [llm] section to nexus.toml:\n\n\
            [llm]\n\
            provider = \"gemini\"\n\
            model = \"gemini-3-pro\"\n\
            enabled = true\n\n\
            Set your API key:\n\
            export GOGGLE_AI_STUDIO_API_KEY=\"your-key\""
        )
    })?;

    if !llm_config.enabled {
        anyhow::bail!("LLM is disabled. Set 'enabled = true' in [llm] section");
    }

    // Get API key from environment
    let api_key = match llm_config.provider.as_str() {
        "openrouter" => std::env::var("OPENROUTER_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("OPENROUTER_API_KEY not set"))
            })?,
        "claude" | "anthropic" => std::env::var("ANTHROPIC_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))
            })?,
        "gemini" | "google" => std::env::var("GOGGLE_AI_STUDIO_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("GOGGLE_AI_STUDIO_API_KEY not set"))
            })?,
        _ => anyhow::bail!("Unknown LLM provider: {}", llm_config.provider),
    };

    // Create runtime for async operations
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        // Create LLM client
        let provider = LlmProvider::from_str(&llm_config.provider)
            .ok_or_else(|| anyhow::anyhow!("Invalid provider: {}", llm_config.provider))?;

        let llm_client = LlmClient::new(provider, api_key, llm_config.model.clone());

        // Create catalyst engine
        let engine = CatalystEngine::new(
            project_id.clone(),
            obsidian_path.clone(),
            llm_client,
        )?;

        // Execute subcommand
        let subcommand = command_args.first().map(|s| s.to_lowercase());
        let result = match subcommand.as_deref() {
            Some("scope") => engine.generate_scope().await,
            Some("stack") => engine.generate_tech_stack().await,
            Some("arch" | "architecture") => engine.generate_architecture().await,
            Some("mvp" | "breakdown") => engine.generate_mvp_breakdown().await,
            Some("generate" | "all") => {
                let report = engine.generate_all().await?;
                report.print_summary();
                Ok(())
            }
            Some("refine") => {
                if command_args.len() < 3 {
                    anyhow::bail!("Usage: catalyst refine <doc> <feedback>\nExample: catalyst refine scope \"Add mobile app to MVP\"");
                }
                let doc_name = command_args[1];
                let feedback = command_args[2..].join(" ");
                let doc_type = parse_doc_type(doc_name)?;
                engine.refine_document(doc_type, &feedback).await
            }
            Some("status") => {
                let status = engine.status();
                status.print_summary();
                Ok(())
            }
            Some("help") | None => {
                print_catalyst_help();
                Ok(())
            }
            Some(cmd) => {
                anyhow::bail!("Unknown catalyst command: '{cmd}'. Use 'catalyst help' for usage.")
            }
        };

        // Clean up environment variable
        if show_reasoning {
            unsafe {
                std::env::remove_var("CATALYST_SHOW_REASONING");
            }
        }

        result
    })
}

/// Parse document type from string
fn parse_doc_type(name: &str) -> Result<crate::catalyst::DocumentType> {
    use crate::catalyst::DocumentType;
    
    match name.to_lowercase().as_str() {
        "scope" => Ok(DocumentType::Scope),
        "stack" | "tech" | "techstack" => Ok(DocumentType::TechStack),
        "arch" | "architecture" => Ok(DocumentType::Architecture),
        "mvp" | "breakdown" => Ok(DocumentType::MvpBreakdown),
        _ => anyhow::bail!(
            "Unknown document type: '{name}'. Valid types: scope, stack, arch, mvp"
        ),
    }
}

/// Print catalyst-specific help
fn print_catalyst_help() {
    println!("{}", "Planning Catalyst - AI Document Generation".bold().underline());
    println!();
    println!("{}", "Document Generation:".bold());
    println!("  {} scope      Generate 02-Scope-and-Boundaries.md", "catalyst".cyan());
    println!("  {} stack      Generate 03-Tech-Stack.md", "catalyst".cyan());
    println!("  {} arch       Generate 04-Architecture.md", "catalyst".cyan());
    println!("  {} mvp        Generate 05-MVP-Breakdown.md", "catalyst".cyan());
    println!("  {} generate   Generate all documents (02-05) sequentially", "catalyst".cyan());
    println!();
    println!("{}", "Refinement & Status:".bold());
    println!("  {} refine <doc> <feedback>  Refine a document with feedback", "catalyst".cyan());
    println!("    Example: catalyst refine scope \"Add mobile app to MVP\"");
    println!("  {} status     Show generation status for all documents", "catalyst".cyan());
    println!();
    println!("{}", "Flags:".bold());
    println!("  {} --show-reasoning  Display model's reasoning process", "-r,".dimmed());
    println!();
    println!("{}", "Prerequisites:".bold());
    println!("  • Complete 01-Problem-and-Vision.md manually");
    println!("  • Configure LLM in nexus.toml ([llm] section)");
    println!("  • Set GOGGLE_AI_STUDIO_API_KEY environment variable");
    println!("  • Recommended: Use reasoning models like gemini-3-pro");
    println!();
    println!("{}", "Example workflows:".dimmed());
    println!();
    println!("  {} Sequential (recommended):", "Option 1:".bold());
    println!("    1. Fill out 01-Problem-and-Vision.md in Obsidian");
    println!("    2. Run 'catalyst generate' to create all documents");
    println!("    3. Review and refine in Obsidian");
    println!();
    println!("  {} Step-by-step:", "Option 2:".bold());
    println!("    1. Fill out 01-Problem-and-Vision.md");
    println!("    2. Run 'catalyst scope'");
    println!("    3. Review scope, then run 'catalyst stack'");
    println!("    4. Review stack, then run 'catalyst arch'");
    println!("    5. Review architecture, then run 'catalyst mvp'");
    println!();
    println!("  {} With reasoning display:", "Option 3:".bold());
    println!("    Run 'catalyst generate --show-reasoning' to see model's thinking");
    println!();
}

/// Execute the plan command - PROJECT GENESIS
fn execute_plan(state: &NexusState, args: &[&str]) -> Result<()> {
    let repo_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    if args.is_empty() {
        print_plan_help();
        return Ok(());
    }

    let subcommand = args[0].to_lowercase();
    match subcommand.as_str() {
        "init" | "--init" => {
            println!("{}", "Running project genesis...".dimmed());
            crate::commands::plan::execute_init(&repo_path)
        }
        "help" => {
            print_plan_help();
            Ok(())
        }
        _ => {
            anyhow::bail!("Unknown plan command: '{subcommand}'. Use 'plan help' for usage.")
        }
    }
}

/// Print plan-specific help
fn print_plan_help() {
    println!("{}", "Project Genesis - Foundation Generator".bold().underline());
    println!();
    println!("{}", "Commands:".bold());
    println!("  {} init      Generate all foundational documents (02-05)", "plan".cyan());
    println!("  {} help      Show this help message", "plan".cyan());
    println!();
    println!("{}", "What is Project Genesis?".bold());
    println!("  Genesis creates the architectural foundation for brand new projects.");
    println!("  It transforms your 01-Problem-and-Vision.md into a complete planning skeleton:");
    println!("    • 02-Scope-and-Boundaries.md");
    println!("    • 03-Architecture-Logic.md");
    println!("    • 04-Tech-Stack-Standard.md");
    println!("    • 05-MVP-Roadmap.md");
    println!();
    println!("{}", "Prerequisites:".bold());
    println!("  • Complete 01-Problem-and-Vision.md manually");
    println!("  • No active Task Capsule (Genesis vs Task separation)");
    println!("  • Configure LLM in nexus.toml ([llm] section)");
    println!("  • Set GOGGLE_AI_STUDIO_API_KEY environment variable");
    println!();
    println!("{}", "Genesis vs Catalyst:".dimmed());
    println!("  • {} Creates architectural foundation (NEW projects)", "Genesis:".bold());
    println!("  • {} Task-scoped generation (features/bugs)", "Catalyst:".bold());
    println!();
}

/// Execute the status command - check Brain health
fn execute_status(state: &NexusState) -> Result<()> {
    let project_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    // Load project configuration
    let config_path = project_path.join("nexus.toml");

    if !config_path.exists() {
        anyhow::bail!(
            "No nexus.toml found for this project.\nExpected at: {}",
            config_path.display()
        );
    }

    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    // Check if brain is configured
    let brain_config = config.brain.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "Brain is not configured. Add [brain] section to nexus.toml:\n\n\
            [brain]\n\
            qdrant_url = \"http://100.64.0.1:6334\"\n\
            enabled = true"
        )
    })?;

    if !brain_config.enabled {
        anyhow::bail!("Brain is disabled. Set 'enabled = true' in [brain] section of nexus.toml");
    }

    println!("{}", "Checking Brain status...".dimmed());
    println!(
        "  {} {}",
        "Qdrant URL:".bold(),
        brain_config.qdrant_url.dimmed()
    );
    println!();

    // Create a tokio runtime for async operations
    let runtime = tokio::runtime::Runtime::new().context("Failed to create tokio runtime")?;

    // Run async health check
    let health = runtime.block_on(async {
        let brain = NexusBrain::connect(&brain_config.qdrant_url).await?;
        brain.ensure_collection().await?;
        brain.health_check().await
    })?;

    // Print health status
    println!(
        "{}",
        "╔══════════════════════════════════════════════════╗".green()
    );
    println!(
        "{}",
        "║          🧠 Brain Status: ONLINE ✓               ║".green()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════╝".green()
    );
    println!();

    let (ram, disk) = health.format_memory();

    println!(
        "  {} {}",
        "Collection:".bold(),
        health.collection_name.green()
    );
    println!(
        "  {} {}",
        "Total Points:".bold(),
        health.points_count.to_string().cyan()
    );
    println!(
        "  {} {}",
        "Total Vectors:".bold(),
        health.vectors_count.to_string().cyan()
    );
    println!(
        "  {} {}",
        "Indexed Vectors:".bold(),
        health.indexed_vectors_count.to_string().cyan()
    );
    println!(
        "  {} {}",
        "Segments:".bold(),
        health.segments_count.to_string().cyan()
    );
    println!();
    println!("  {} {}", "RAM Usage:".bold(), ram.yellow());
    println!("  {} {}", "Disk Usage:".bold(), disk.blue());
    println!();

    Ok(())
}

/// Execute the watch command - start Sentinel file watcher
fn execute_watch(
    state: &NexusState,
    watcher: &Arc<Mutex<Option<SentinelWatcher>>>,
    watcher_enabled: &Arc<Mutex<bool>>,
) -> Result<()> {
    let project_id = state
        .active_project_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    let repo_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get repo path"))?;

    let obsidian_path = state
        .get_active_obsidian_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get Obsidian path"))?;

    // Load project config to get Brain URL
    let config_path = repo_path.join("nexus.toml");

    if !config_path.exists() {
        anyhow::bail!(
            "No nexus.toml found for this project.\nExpected at: {}",
            config_path.display()
        );
    }

    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    let brain_config = config.brain.as_ref().ok_or_else(|| {
        anyhow::anyhow!("Brain is not configured. Add [brain] section to nexus.toml")
    })?;

    if !brain_config.enabled {
        anyhow::bail!("Brain is disabled. Set 'enabled = true' in [brain] section");
    }

    println!("{}", "🔍 Starting Sentinel file watcher...".cyan());

    // Stop existing watcher if any
    {
        let mut w = watcher.lock().unwrap();
        if w.is_some() {
            println!("  Stopping existing watcher...");
        }
        *w = None;
    }

    // Create new watcher
    let new_watcher = SentinelWatcher::new(brain_config.qdrant_url.clone())?;

    // Start watching
    new_watcher.watch_project(project_id.clone(), repo_path.clone(), obsidian_path.clone())?;

    // Store watcher
    *watcher.lock().unwrap() = Some(new_watcher);
    *watcher_enabled.lock().unwrap() = true;

    println!(
        "{} Sentinel is now watching for changes",
        "✓".green().bold()
    );
    println!("  Look for the 👁 indicator in your prompt");

    Ok(())
}

/// Execute the unwatch command - stop Sentinel file watcher
fn execute_unwatch(
    watcher: &Arc<Mutex<Option<SentinelWatcher>>>,
    watcher_enabled: &Arc<Mutex<bool>>,
) -> Result<()> {
    let mut w = watcher.lock().unwrap();

    if w.is_none() {
        println!("{}", "No watcher is currently running".yellow());
        return Ok(());
    }

    println!("{}", "🔍 Stopping Sentinel file watcher...".cyan());

    if let Some(sentinel) = w.take() {
        sentinel.stop_watching()?;
    }

    *watcher_enabled.lock().unwrap() = false;

    println!("{} Sentinel stopped", "✓".green().bold());

    Ok(())
}

/// Execute the why command - explain last gate failure using architecture
fn execute_why(state: &NexusState, last_gate_error: &Arc<Mutex<Option<String>>>) -> Result<()> {
    let error = last_gate_error.lock().unwrap().clone();

    if error.is_none() {
        println!("{}", "No recent gate failure to explain.".yellow());
        println!("Run 'gate' first to check your planning documents.");
        return Ok(());
    }

    let error_msg = error.unwrap();
    println!("{}", "🤔 Analyzing gate failure...".cyan());
    println!();
    println!("{} {}", "Last error:".bold(), error_msg.dimmed());
    println!();

    // Search architecture for relevant context
    println!("{}", "Searching Architecture for guidance...".dimmed());

    let project_id = state
        .active_project_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active project selected"))?;

    let repo_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get repo path"))?;

    // Load config to get Brain URL
    let config_path = repo_path.join("nexus.toml");
    if !config_path.exists() {
        anyhow::bail!("No nexus.toml found. Brain not configured.");
    }

    let config_content = std::fs::read_to_string(&config_path)?;
    let config: NexusConfig = toml::from_str(&config_content)?;

    let brain_config = config
        .brain
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Brain not configured in nexus.toml"))?;

    if !brain_config.enabled {
        anyhow::bail!("Brain is disabled");
    }

    // Search for relevant architecture content
    let results = tokio::runtime::Runtime::new()?.block_on(async {
        let brain = NexusBrain::connect(&brain_config.qdrant_url).await?;

        // Generate query embedding (in production, use actual embedding model)
        let query_vector = generate_query_embedding(&error_msg);

        // Search architecture specifically
        brain.search_architecture(query_vector, project_id, 3).await
    })?;

    if results.is_empty() {
        println!(
            "{}",
            "No relevant architecture documentation found.".yellow()
        );
        return Ok(());
    }

    println!();
    println!("{}", "📚 Relevant Architecture Context:".bold().underline());
    println!();

    for (idx, result) in results.iter().enumerate() {
        println!(
            "{} {} {}",
            format!("[{}]", idx + 1).cyan(),
            "From".dimmed(),
            result.file_name().green()
        );
        println!(
            "   {} {}",
            "Score:".dimmed(),
            format!("{:.2}", result.score).yellow()
        );
        println!("   {}", result.content.dimmed());
        println!();
    }

    println!(
        "{}",
        "💡 These architecture guidelines may help resolve the issue.".cyan()
    );

    Ok(())
}

/// Execute a semantic query against the Brain with optional flags
///
/// Supported flags:
/// - `--global`: Search across all projects (bypass project filter)
/// - `--arch`: Only search Architecture and `GlobalStandard` layers
fn execute_semantic_query(input: &str, state: &NexusState) -> Result<()> {
    use crate::brain::Layer;

    // Parse flags and extract actual query
    let mut is_global = false;
    let mut arch_only = false;
    let mut query_parts = Vec::new();

    for part in input.split_whitespace() {
        match part {
            "--global" => is_global = true,
            "--arch" => arch_only = true,
            _ => query_parts.push(part),
        }
    }

    let query = query_parts.join(" ");

    if query.is_empty() {
        anyhow::bail!("Query cannot be empty");
    }

    let project_id = state.active_project_id.as_ref().ok_or_else(|| {
        anyhow::anyhow!("No active project. Use 'use <project>' first for semantic search.")
    })?;

    let repo_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get repo path"))?;

    // Display search mode
    let search_mode = if is_global && arch_only {
        "🌍 Global Architecture Search".yellow()
    } else if is_global {
        "🌍 Global Search".yellow()
    } else if arch_only {
        "🏛️ Architecture Search".cyan()
    } else {
        "🔍 Project Search".cyan()
    };

    println!(
        "{} \"{}\"",
        search_mode,
        query.italic()
    );

    // Load config to get Brain URL
    let config_path = repo_path.join("nexus.toml");
    if !config_path.exists() {
        anyhow::bail!(
            "No nexus.toml found. Brain not configured.\nRun 'help' to see available commands."
        );
    }

    let config_content = std::fs::read_to_string(&config_path)?;
    let config: NexusConfig = toml::from_str(&config_content)?;

    let brain_config = config.brain.as_ref()
        .ok_or_else(|| anyhow::anyhow!(
            "Brain not configured. This looks like a command, but it's not recognized.\nRun 'help' to see available commands."
        ))?;

    if !brain_config.enabled {
        anyhow::bail!("Brain is disabled. Cannot perform semantic search.");
    }

    // Determine layer filter
    let layers = if arch_only {
        Some(vec![Layer::ProjectArchitecture, Layer::GlobalStandard])
    } else {
        None
    };

    // Search the Brain
    let results = tokio::runtime::Runtime::new()?.block_on(async {
        let brain = NexusBrain::connect(&brain_config.qdrant_url).await?;

        // Generate query embedding (in production, use actual embedding model)
        let query_vector = generate_query_embedding(&query);

        // Execute appropriate search based on flags
        if is_global {
            brain.global_search(query_vector, 5, layers).await
        } else {
            brain.search(query_vector, 5, project_id, layers).await
        }
    })?;

    if results.is_empty() {
        println!();
        println!("{}", "No results found.".yellow());
        println!("Try rephrasing your query or ensure files are indexed (use 'watch').");
        return Ok(());
    }

    println!();
    println!(
        "{} {} {}",
        "Found".green().bold(),
        results.len(),
        "results:".green().bold()
    );
    println!();

    for (idx, result) in results.iter().enumerate() {
        println!(
            "{} {} {}",
            format!("[{}]", idx + 1).cyan().bold(),
            "From".dimmed(),
            result.file_path.green()
        );
        println!(
            "   {} {}",
            "Relevance:".dimmed(),
            format!("{:.1}%", result.score * 100.0).yellow()
        );

        if let Some(ref file_type) = result.file_type {
            println!("   {} {}", "Type:".dimmed(), file_type.blue());
        }

        println!("   {}", result.content);
        println!();
    }

    Ok(())
}

/// Execute the context command - toggle context injection
fn execute_context(args: &[&str], context_enabled: &Arc<Mutex<bool>>) -> Result<()> {
    if args.is_empty() {
        // Show current status
        let is_enabled = *context_enabled.lock().unwrap();
        println!(
            "{} Context injection is currently {}",
            if is_enabled { "🧠" } else { "  " },
            if is_enabled {
                "enabled".green()
            } else {
                "disabled".red()
            }
        );
        println!();
        println!("  Use 'context on' to enable or 'context off' to disable");
        return Ok(());
    }

    match args[0].to_lowercase().as_str() {
        "on" | "enable" | "true" => {
            *context_enabled.lock().unwrap() = true;
            println!("{} Context injection enabled", "🧠".green().bold());
            println!("  Natural language queries will now include:");
            println!("    • Top 3 architecture snippets from Qdrant");
            println!("    • Active sprint context from Obsidian");
        }
        "off" | "disable" | "false" => {
            *context_enabled.lock().unwrap() = false;
            println!("{} Context injection disabled", "✓".yellow().bold());
            println!("  Queries will fall back to simple semantic search");
        }
        _ => {
            anyhow::bail!("Invalid argument. Use 'context on' or 'context off'");
        }
    }

    Ok(())
}

/// Execute the decision command - store an architectural decision in Qdrant
fn execute_decision(args: &[&str], _state: &NexusState) -> Result<()> {
    use crate::memory::NexusMemory;

    if args.is_empty() {
        anyhow::bail!("Usage: decision <text>");
    }

    let text = args.join(" ");
    println!("{}", "💾 Storing architectural decision...".cyan());

    tokio::runtime::Runtime::new()?.block_on(async {
        let memory = NexusMemory::connect().await?;
        memory.store_decision(&text).await
    })?;

    println!("{} Decision stored in nexus_ledger", "✓".green().bold());
    Ok(())
}

/// Execute an LLM query with context injection and conversation history
fn execute_llm_query(input: &str, state: &NexusState) -> Result<()> {
    use crate::context::{get_active_context, ContextTemplate, RELEVANCE_THRESHOLD};
    use crate::history::ConversationHistory;
    use crate::llm::{LlmClient, LlmProvider};
    use crate::session::NexusSession;
    use crate::memory::NexusMemory;

    let project_id = state
        .active_project_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    let repo_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get repo path"))?;

    let obsidian_path = state
        .get_active_obsidian_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get Obsidian path"))?;

    // Load conversation history
    let mut history = ConversationHistory::load(&obsidian_path, project_id)?;

    // Load session for Gemini thought rehydration
    let session_path = repo_path.join(".nexus_session.json");
    let mut session = NexusSession::load(&session_path)?;

    // Connect to architectural memory
    let memory = tokio::runtime::Runtime::new()?.block_on(async {
        NexusMemory::connect().await
    })?;

    // Load project config
    let config_path = repo_path.join("nexus.toml");
    if !config_path.exists() {
        anyhow::bail!(
            "No nexus.toml found. LLM not configured.\nRun 'help' to see available commands."
        );
    }

    let config_content = std::fs::read_to_string(&config_path)?;
    let config: NexusConfig = toml::from_str(&config_content)?;

    // Check if LLM is configured
    let llm_config = config.llm.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "LLM not configured. Add [llm] section to nexus.toml:\n\n\
            [llm]\n\
            provider = \"gemini\"  # Default (high reasoning), or \"claude\"/\"openrouter\"\n\
            model = \"gemini-3-pro\"  # Default model\n\
            enabled = true\n\n\
            Set your API key via environment variable:\n\
            export GOGGLE_AI_STUDIO_API_KEY=\"your-key\"  # for Gemini (default)\n\
            export ANTHROPIC_API_KEY=\"your-key\"   # for Claude\n\
            export OPENROUTER_API_KEY=\"your-key\"  # for OpenRouter"
        )
    })?;

    if !llm_config.enabled {
        anyhow::bail!("LLM is disabled. Set 'enabled = true' in [llm] section of nexus.toml");
    }

    // Get API key from environment or config
    let api_key = match llm_config.provider.as_str() {
        "openrouter" => std::env::var("OPENROUTER_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("OPENROUTER_API_KEY not set"))
            })?,
        "claude" | "anthropic" => std::env::var("ANTHROPIC_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))
            })?,
        "gemini" | "google" => std::env::var("GOGGLE_AI_STUDIO_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("GOGGLE_AI_STUDIO_API_KEY not set"))
            })?,
        _ => anyhow::bail!("Unknown LLM provider: {}", llm_config.provider),
    };

    println!("{}", "🧠 Retrieving context...".cyan());

    // Check if Brain is configured for architecture context
    let brain_config = config.brain.as_ref();
    let qdrant_url = if let Some(brain) = brain_config {
        if brain.enabled {
            Some(brain.qdrant_url.clone())
        } else {
            None
        }
    } else {
        None
    };

    // Run async context retrieval to build the prompt
    let runtime = tokio::runtime::Runtime::new()?;
    let prompt = runtime.block_on(async {
        // Retrieve architectural decisions
        let previous_decisions = memory.retrieve_context(input).await.unwrap_or_default();

        // Get active context (architecture + sprint)
        let context = if let Some(ref url) = qdrant_url {
            get_active_context(input, project_id, url, &obsidian_path, &config).await?
        } else {
            // No brain configured, create empty context
            println!("  {} Brain not configured, skipping architecture context", "⚠".yellow());
            crate::context::ActiveContext {
                architecture: crate::context::ArchitectureContext {
                    snippets: Vec::new(),
                },
                sprint: None,
            }
        };

        // Show context summary
        if context.architecture.snippets.is_empty() {
            println!(
                "  {} No architecture snippets above threshold ({})",
                "⚠".yellow(),
                RELEVANCE_THRESHOLD
            );
        } else {
            println!(
                "  {} Retrieved {} architecture snippets (score ≥ {})",
                "✓".green(),
                context.architecture.snippets.len(),
                RELEVANCE_THRESHOLD
            );
        }

        if let Some(ref sprint) = context.sprint {
            println!("  {} Retrieved sprint context: {}", "✓".green(), sprint.sprint_id);
        }

        // Build context template
        let template = ContextTemplate::new(context, input.to_string());
        let mut prompt = template.render();

        // Inject architectural decisions
        if !previous_decisions.is_empty() {
            let decisions_block = format!(
                "\n### PREVIOUS ARCHITECTURAL DECISIONS\n{}\n",
                previous_decisions.join("\n")
            );
            prompt = format!("{decisions_block}{prompt}");
            println!("  {} Included {} previous architectural decisions", "✓".green(), previous_decisions.len());
        }

        // Prepend conversation history if available
        if !history.is_empty() {
            let history_context = history.get_context_string();
            prompt = format!("{history_context}\n{prompt}");
            println!("  {} Included {} previous conversation turns", "✓".green(), history.len());
        }

        Ok::<String, anyhow::Error>(prompt)
    })?;

    // SAFETY GATE: Show prompt and ask for confirmation
    let confirmed = crate::llm::confirm_llm_prompt(&prompt, "Natural language query")?;
    if !confirmed {
        return Ok(()); // User cancelled
    }

    // Create LLM client
    let provider_enum = LlmProvider::from_str(&llm_config.provider)
        .ok_or_else(|| anyhow::anyhow!("Invalid LLM provider: {}", llm_config.provider))?;

    let client = LlmClient::new(provider_enum.clone(), api_key, llm_config.model.clone());

    println!();
    println!("{}", "🤖 Querying LLM...".cyan());
    println!();

    // Send to LLM in async block
    let result = runtime.block_on(async {

        // Send to LLM
        let (response, thought_signature) = if provider_enum == LlmProvider::Gemini {
            // Rehydrate session messages
            let session_messages: Vec<(String, String)> = session.get_last_5_messages()
                .into_iter()
                .map(|m| (m.role, m.content))
                .collect();
            
            client.complete_gemini_full(
                "", // System prompt is already in the main prompt string for now
                &session_messages,
                session.last_thought_signature.clone(),
                &prompt
            ).await?
        } else {
            (client.complete(&prompt).await?, None)
        };

        Ok::<(String, Option<String>), anyhow::Error>((response, thought_signature))
    })?;

    let (result, thought_signature) = result;

    // Print response
    println!("{result}");
    println!();

    // Save conversation turn to history
    history.add_turn(input.to_string(), Some(result.clone()));
    if let Err(e) = history.save(&obsidian_path) {
        eprintln!("Warning: Failed to save conversation history: {e}");
    }

    // Update and save session
    session.add_message("user".to_string(), input.to_string());
    session.add_message("model".to_string(), result.clone());
    session.last_thought_signature = thought_signature;
    if let Err(e) = session.save(&session_path) {
        eprintln!("Warning: Failed to save session: {e}");
    }

    Ok(())
}

/// Generate a query embedding (stub - replace with real embedding model)
fn generate_query_embedding(_query: &str) -> Vec<f32> {
    // In production, use an actual embedding model
    // For now, return a dummy vector (384 for all-MiniLM-L6-v2)
    vec![0.0; 384]
}

/// Find the paths to the ONNX model and tokenizer
fn find_model_paths() -> (String, String) {
    let model_name = "model.onnx";
    let tokenizer_name = "tokenizer.json";

    // List of candidate directories to check
    let candidates = [
        "models/models",
        "models",
        "/home/nullvektor/repos/nexus_cli/models/models",
    ];

    for dir in candidates {
        let model_path = std::path::Path::new(dir).join(model_name);
        let tokenizer_path = std::path::Path::new(dir).join(tokenizer_name);

        if model_path.exists() && tokenizer_path.exists() {
            return (
                model_path.to_string_lossy().to_string(),
                tokenizer_path.to_string_lossy().to_string(),
            );
        }
    }

    // Default to relative paths if not found (will fail with clear error)
    ("models/models/model.onnx".to_string(), "models/models/tokenizer.json".to_string())
}

/// Execute task-related commands (start, done)
///
/// # Arguments
/// * `state` - Current shell session state
/// * `args` - Command arguments (e.g., ["start"] or ["done"])
/// Execute the init command from REPL
fn execute_init_command(args: &[&str]) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("Usage: init <project-name> [--mode sprint|adhoc] [--project]");
    }

    let project_name = args[0];
    let mut mode = "sprint";
    let mut is_full_project = false;

    // Parse flags
    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "--mode" => {
                if i + 1 < args.len() {
                    mode = args[i + 1];
                    i += 2;
                } else {
                    anyhow::bail!("--mode requires a value (sprint or adhoc)");
                }
            }
            "--project" => {
                is_full_project = true;
                i += 1;
            }
            _ => {
                anyhow::bail!("Unknown flag: {}", args[i]);
            }
        }
    }

    // Call the init module's execute function
    crate::commands::init::execute(project_name, mode, is_full_project)
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    Ok(())
}

fn execute_task_command(state: &NexusState, args: &[&str]) -> Result<()> {
    let project_path = state
        .get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;

    if args.is_empty() {
        anyhow::bail!("Usage: task <start|done>");
    }

    match args[0].to_lowercase().as_str() {
        "start" => crate::commands::task::execute_start(&project_path),
        "done" => crate::commands::task::execute_done(&project_path),
        _ => anyhow::bail!("Unknown task command: '{}'. Use 'start' or 'done'.", args[0]),
    }
}

/// Check if the active project is in adhoc mode
///
/// # Arguments
/// * `state` - Current shell session state
///
/// # Returns
/// * `true` if the active project is in adhoc mode, `false` otherwise
fn is_active_project_adhoc(state: &NexusState) -> bool {
    let repo_path = match state.get_active_repo_path() {
        Some(path) => path,
        None => return false,
    };

    let config_path = repo_path.join("nexus.toml");
    if !config_path.exists() {
        return false;
    }

    let config_content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(_) => return false,
    };

    let config: NexusConfig = match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(_) => return false,
    };

    config.is_adhoc_mode()
}
