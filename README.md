# Nexus CLI

A Rust CLI tool for managing Obsidian-based project planning and execution with AI assistance.

## Overview

Nexus CLI enforces planning discipline through a gated workflow:
1. **Init** - Create a new project with planning templates
2. **Gate** - Validate that planning documents are complete
3. **Unlock** - Generate CLAUDE.md from validated planning docs

## Features

- **Structured Planning**: Template-based planning workflow with enforced completion
- **Gate Validation**: Heuristic-based validation ensures planning quality
- **AI Integration**: Generate CLAUDE.md for AI-assisted development
- **Git Integration**: Automatic repository initialization and initial commit
- **Path Flexibility**: Configurable paths for Obsidian vaults and planning directories

## Installation

```bash
cargo build --release
./target/release/nexus_cli --help
```

## Usage

### 1. Initialize a New Project

Create a new project with planning templates:

```bash
nexus init my-project
cd my-project
```

This creates:
- Project directory structure
- Planning document templates (01-PLANNING/)
- Management directory (00-MANAGEMENT/)
- `nexus.toml` configuration file

### 2. Fill Out Planning Documents

Open the project in Obsidian and complete the planning templates:

1. `01-Problem-and-Vision.md` - Define problem, vision, and success criteria
2. `02-Scope-and-Boundaries.md` - Define MVP scope and boundaries
3. `03-Tech-Stack.md` - Choose and justify technology stack
4. `04-Architecture.md` - Design system architecture
5. `05-MVP-Breakdown.md` - Break MVP into sprints

Complete all tasks in `00-MANAGEMENT/00-START-HERE.md`.

### 3. Validate Planning (Gate Check)

Verify planning is complete before unlocking:

```bash
nexus gate .
```

The gate checks:
- All checkboxes in dashboard are checked
- Planning documents meet minimum quality standards
- No incomplete placeholders (TODO, TBD, etc.)
- All required sections are present

Example output when validation passes:
```
🚪 INITIATING GATE SEQUENCE...

📋 SCANNING DASHBOARD...
  ✓ Dashboard clean - all tasks completed

📝 SCANNING PLANNING DOCUMENTS...
  ✓ 01-Problem-and-Vision.md
  ✓ 02-Scope-and-Boundaries.md
  ✓ 03-Tech-Stack.md
  ✓ 04-Architecture.md
  ✓ 05-MVP-Breakdown.md

────────────────────────────────────────────────────────

✅ MISSION READY
   Gate is open. All validation checks passed.
```

### 4. Unlock Project (Generate CLAUDE.md)

Once planning passes the gate, unlock the project:

```bash
nexus unlock .
```

This command:
1. **Runs gate check** - Aborts if planning is incomplete
2. **Parses planning documents** - Extracts structured content
3. **Generates CLAUDE.md** - Creates permanent AI context file
4. **Initializes git** - Creates repository if needed
5. **Creates initial commit** - Commits CLAUDE.md and planning docs

Example output:
```
🔓 INITIATING UNLOCK SEQUENCE...

📋 Loading configuration...
  ✓ Config loaded: my-project

🚪 Running gate check...
  ✓ Gate check passed - planning complete

🔧 Parsing planning documents...
  ✓ Planning documents parsed

📝 Generating CLAUDE.md...
  ✓ CLAUDE.md generated

🔧 Initializing git repository...
  ✓ Files staged for commit
  ✓ Initial commit created

────────────────────────────────────────────────────────
✅ PROJECT UNLOCKED
────────────────────────────────────────────────────────

📋 Summary:
  • CLAUDE.md generated at: /path/to/project/CLAUDE.md
  • Git repository initialized
  • Initial commit created with planning docs

🚀 Next Steps:
  1. Review CLAUDE.md in your repository
  2. Share CLAUDE.md with your AI assistant (Claude, etc.)
  3. Start development with clear context and constraints
```

### Safety Gate

The unlock command **requires** the gate check to pass. If planning is incomplete:

```bash
nexus unlock .
```

Output:
```
🚪 Running gate check...

🚫 UNLOCK ABORTED
  Gate check failed. Fix planning issues before unlocking.
```

This prevents generating CLAUDE.md from incomplete planning.

### Idempotency

Running `unlock` multiple times is safe:
- Git repository initialization is skipped if .git exists
- Initial commit is skipped if repository already has commits
- CLAUDE.md is regenerated (updates with latest planning content)

## Configuration

The `nexus.toml` file configures project paths and settings:

```toml
[project]
name = "my-project"
version = "0.1.0"
obsidian_path = "/path/to/project"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[obsidian]
planning_path = "/path/to/project"  # Defaults to obsidian_path if not set

[state]
is_unlocked = false

[templates]
claude_template = "templates/CLAUDE.md.example"
```

### Path Configuration

- `obsidian_path` - Root directory where nexus.toml lives
- `planning_path` - Directory containing 01-PLANNING/ (defaults to obsidian_path)

All commands resolve paths using these configuration values, not the current working directory.

## Testing

Run tests:

```bash
cargo test
```

Run specific test suite:

```bash
cargo test --test gate_integration
cargo test --test unlock_integration
```

## Architecture

- `src/commands/` - Command implementations (init, gate, unlock)
- `src/config.rs` - Configuration structure and loading
- `src/heuristics.rs` - Gate validation rules
- `src/planning.rs` - Planning document parsing and validation
- `src/templating.rs` - CLAUDE.md template rendering
- `templates/` - Project templates and Tera templates

## License

MIT
