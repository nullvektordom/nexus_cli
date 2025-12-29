# Technical choices

## Stack (force yourself to choose NOW):
- **Language:** Rust
- **CLI Framework:** clap (v4 with derive)
- **Storage:** JSON file (serde_json)
- **Hosting:** Local installation only (cargo install)

## Why these choices?
Rust provides fast, safe binaries perfect for CLI tools. Clap makes CLI argument parsing trivial with derive macros. JSON storage is simple, human-readable, and requires no database setup. Local-only means zero deployment complexity.

## What I will NOT use:
- Not: "Database server (SQLite, Postgres)"
- Not: "Web framework or HTTP server"
- Not: "Docker containers"
- Not: "Configuration management systems"

## Dependencies (max 10 important ones):
1. clap (CLI parsing)
2. serde (JSON serialization)
3. serde_json (JSON handling)
4. chrono (timestamp handling)

## Development environment:
- IDE: VS Code / Neovim
- OS: Linux (Fedora)
- Device: Desktop

---
✅ Done when: Stack chosen + each choice justified
