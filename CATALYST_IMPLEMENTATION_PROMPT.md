# Planning Catalyst - Implementation Prompt for Claude Code

## Context

This is a Rust CLI tool called **nexus_cli** that enforces planning discipline for software projects using Obsidian-based templates. The tool currently has:

- ✅ Planning template scaffolding ([`nexus init`](src/commands/init.rs))
- ✅ Validation system ([`nexus gate`](src/commands/gate.rs))
- ✅ Document generation ([`nexus unlock`](src/commands/unlock.rs))
- ✅ Sprint orchestration ([`nexus sprint`](src/commands/sprint.rs))
- ✅ Interactive REPL ([`nexus shell`](src/commands/shell.rs))
- ✅ LLM integration via OpenRouter ([`src/llm.rs`](src/llm.rs))
- ✅ Qdrant vector search ([`src/brain.rs`](src/brain.rs))
- ✅ Conversation history ([`src/history.rs`](src/history.rs))

## Problem to Solve

Users manually fill out 5 planning documents ([`01-Problem-and-Vision.md`](templates/project/01-Problem-and-Vision.md) through [`05-MVP-Breakdown.md`](templates/project/05-MVP-Breakdown.md)). This is time-consuming. 

**Goal**: Build a "Planning Catalyst" that uses DeepSeek R1 with sequential thinking to intelligently generate planning documents 02-05 from the user's manually written vision document (01).

## Architecture Overview

Full design: [`PLANNING_CATALYST_DESIGN.md`](PLANNING_CATALYST_DESIGN.md)

**High-level flow**:
1. User writes [`01-Problem-and-Vision.md`](templates/project/01-Problem-and-Vision.md) manually
2. User runs `catalyst generate` in REPL
3. System sequentially generates 02-05, each building on previous context
4. Each document is validated against gate heuristics before saving
5. User reviews/refines in Obsidian

**Key components**:
```
src/catalyst/
├── mod.rs              # Module exports
├── engine.rs           # CatalystEngine - orchestration
├── generator.rs        # DocumentGenerator - per-doc logic  
├── prompts.rs          # Prompt templates
└── validation.rs       # Document validation
```

## Implementation Phases

### Phase 1: Core Infrastructure (Start Here)

**Objective**: Build basic catalyst structure with scope document generation only.

#### Tasks

1. **Create module structure**
   - Create `src/catalyst/mod.rs` with public exports
   - Create `src/catalyst/engine.rs` with `CatalystEngine` struct
   - Create `src/catalyst/generator.rs` with `DocumentGenerator` struct
   - Create `src/catalyst/prompts.rs` with `PromptTemplate` struct
   - Create `src/catalyst/validation.rs` with validation functions
   - Update `src/lib.rs` to expose catalyst module

2. **Implement basic types in `src/catalyst/generator.rs`**
   ```rust
   pub enum DocumentType {
       Scope,           // 02-Scope-and-Boundaries.md
       TechStack,       // 03-Tech-Stack.md
       Architecture,    // 04-Architecture.md
       MvpBreakdown,    // 05-MVP-Breakdown.md
   }
   
   #[derive(Debug, Clone)]
   pub struct VisionData {
       pub problem: String,
       pub solution: String,
       pub success_criteria: String,
       pub anti_vision: String,
   }
   
   #[derive(Debug, Clone)]
   pub struct GenerationContext {
       pub vision: VisionData,
       // More fields added in later phases
   }
   ```

3. **Implement `PromptTemplate` in `src/catalyst/prompts.rs`**
   - Create template struct with system/user prompts
   - Implement `for_scope()` method (see design doc for prompt text)
   - Implement `render()` to interpolate context data
   - Template should guide DeepSeek R1 to output valid markdown

4. **Implement `CatalystEngine` in `src/catalyst/engine.rs`**
   ```rust
   pub struct CatalystEngine {
       project_id: String,
       obsidian_path: PathBuf,
       llm_client: LlmClient,
   }
   
   impl CatalystEngine {
       pub fn new(
           project_id: String,
           obsidian_path: PathBuf,
           llm_config: &LlmConfig,
       ) -> Result<Self>;
       
       pub async fn generate_scope(&self) -> Result<()>;
   }
   ```
   
   Key logic:
   - Load [`01-Problem-and-Vision.md`](templates/project/01-Problem-and-Vision.md)
   - Parse vision data using markdown parser
   - Build generation context
   - Generate scope document via LLM
   - Validate against heuristics
   - Save to `02-Scope-and-Boundaries.md`

5. **Add REPL command in `src/commands/shell.rs`**
   - Add `"catalyst" => execute_catalyst(state, args)` to command match
   - Implement `execute_catalyst()` function
   - Support `catalyst scope` command initially
   - Add to help text

6. **Implement validation in `src/catalyst/validation.rs`**
   - Re-use [`validate_planning_document_with_headers`](src/planning.rs:197) from planning module
   - Define required headers for scope document
   - Validate minimum word counts
   - Check for illegal strings (TODO, TBD, etc.)

#### Acceptance Criteria for Phase 1
- [ ] Can run `nexus shell` → `use <project>` → `catalyst scope`
- [ ] System loads vision from 01-Problem-and-Vision.md
- [ ] System calls DeepSeek R1 API via OpenRouter
- [ ] Generated 02-Scope-and-Boundaries.md is saved to Obsidian
- [ ] Document passes `nexus gate` validation
- [ ] Clear error messages if LLM unavailable or vision missing

---

### Phase 2: All Document Types

**Objective**: Extend to generate all planning documents (02-05) with progressive context building.

#### Tasks

1. **Extend `GenerationContext` in `src/catalyst/generator.rs`**
   ```rust
   #[derive(Debug, Clone)]
   pub struct ScopeData {
       pub mvp_features: Vec<String>,
       pub version2_features: Vec<String>,
       pub never_features: Vec<String>,
       pub constraints: String,
   }
   
   #[derive(Debug, Clone)]
   pub struct TechStackData {
       pub language: String,
       pub framework: String,
       pub database: Option<String>,
       pub justification: String,
   }
   
   #[derive(Debug, Clone)]
   pub struct ArchitectureData {
       pub folder_structure: String,
       pub data_model: String,
       pub user_flow: String,
   }
   
   pub struct GenerationContext {
       pub vision: VisionData,
       pub scope: Option<ScopeData>,
       pub tech_stack: Option<TechStackData>,
       pub architecture: Option<ArchitectureData>,
   }
   ```

2. **Add prompt templates in `src/catalyst/prompts.rs`**
   - Implement `for_tech_stack()` - uses vision + scope context
   - Implement `for_architecture()` - uses vision + scope + stack context
   - Implement `for_mvp_breakdown()` - uses all previous context
   - See [`PLANNING_CATALYST_DESIGN.md`](PLANNING_CATALYST_DESIGN.md) for prompt examples

3. **Implement document parsers**
   - Add parser functions to extract structured data from generated markdown
   - Parse scope → `ScopeData`
   - Parse tech stack → `TechStackData`
   - Parse architecture → `ArchitectureData`

4. **Extend `CatalystEngine` with sequential generation**
   ```rust
   pub async fn generate_all(&self) -> Result<GenerationReport>;
   ```
   
   Logic:
   - Load vision (01)
   - Generate scope (02), parse to ScopeData, add to context
   - Generate tech stack (03), parse to TechStackData, add to context
   - Generate architecture (04), parse to ArchitectureData, add to context
   - Generate MVP breakdown (05)
   - Return report with success/failure for each document

5. **Add progress indicators**
   - Print status updates during generation
   - Show validation results for each document
   - Report final summary

6. **Add REPL commands**
   - `catalyst generate` - generate all documents
   - `catalyst stack` - generate tech stack only
   - `catalyst arch` - generate architecture only
   - `catalyst mvp` - generate MVP breakdown only

#### Acceptance Criteria for Phase 2
- [ ] `catalyst generate` produces all 4 documents (02-05)
- [ ] Each document builds on previous context
- [ ] All documents pass gate validation
- [ ] Progress is visible to user
- [ ] Can generate individual documents independently

---

### Phase 3: Sequential Thinking Integration

**Objective**: Integrate self-hosted MCP `sequentialthinking` server for improved reasoning quality.

#### Infrastructure Details

**Sequential Thinking MCP Server**:
- **Deployment**: Self-hosted on remote server (accessible via Tailscale network)
- **Protocol**: HTTP Server-Sent Events (SSE)
- **URL**: `http://100.105.8.97:8000/sse`
- **Configuration**: Environment variable `SEQUENTIAL_THINKING_URL` in `.env` file

**Connection Strategy**:
- Read URL from environment variable (fallback to hardcoded default)
- Use HTTP client with SSE support (not stdio like local MCP)
- Handle network timeouts and connection failures gracefully
- Validate server availability before attempting generation

#### Tasks

1. **Create MCP SSE client in `src/catalyst/thinking.rs`**
   ```rust
   use reqwest::Client;
   use std::env;
   
   pub struct ThinkingSession {
       thoughts: Vec<ThoughtStep>,
       final_answer: Option<String>,
   }
   
   pub struct ThoughtStep {
       thought_number: u32,
       content: String,
       is_revision: bool,
   }
   
   pub struct ThinkingClient {
       server_url: String,
       http_client: Client,
   }
   
   impl ThinkingClient {
       /// Create new MCP client, reading URL from environment
       pub fn new() -> Result<Self> {
           let server_url = env::var("SEQUENTIAL_THINKING_URL")
               .unwrap_or_else(|_| "http://100.105.8.97:8000/sse".to_string());
           
           Ok(Self {
               server_url,
               http_client: Client::new(),
           })
       }
       
       /// Check if the MCP server is reachable
       pub async fn health_check(&self) -> Result<bool> {
           // Implement health check endpoint
       }
       
       /// Generate document with sequential thinking
       pub async fn generate_with_thinking(
           &self,
           prompt: &str,
           llm_client: &LlmClient,
       ) -> Result<(String, ThinkingSession)> {
           // Connect to SSE endpoint
           // Stream thinking steps
           // Collect final answer
       }
   }
   ```

2. **Add environment variable support**
   - Create `.env.example` with:
     ```bash
     SEQUENTIAL_THINKING_URL=http://100.105.8.97:8000/sse
     ```
   - Load environment variables using `dotenv` crate
   - Document in README that `.env` file is optional (uses default if missing)

3. **Update prompts to request reasoning**
   - Modify templates to request "step-by-step thinking"
   - Structure: "Think through X, then provide final document"
   - Include instructions for the thinking MCP to structure responses

4. **Integrate with generator**
   - Update `DocumentGenerator` to use thinking when enabled
   - Add fallback: if MCP server unavailable, use direct LLM generation
   - Capture thinking steps for transparency
   - Extract final answer from thinking session

5. **Add configuration**
   - Add `[catalyst]` section to `nexus.toml`:
     ```toml
     [catalyst]
     enabled = true
     use_thinking = true          # Toggle sequential thinking integration
     thinking_timeout_secs = 120  # Max time to wait for thinking response
     show_thinking = false        # Debug mode: display reasoning steps
     ```

6. **Add `--show-thinking` flag**
   - Optional flag to display reasoning process in REPL
   - Shows each thinking step with timestamps
   - Helps debug generation quality issues and MCP connectivity

7. **Error handling for remote MCP**
   - Network timeouts: Fall back to direct generation after timeout
   - Connection refused: Warn user and continue without thinking
   - Invalid SSE format: Log error and retry with direct generation
   - Clear error messages: "Sequential thinking server unreachable, using direct generation"

#### Acceptance Criteria for Phase 3
- [ ] Reads `SEQUENTIAL_THINKING_URL` from environment (or uses default)
- [ ] Successfully connects to remote MCP server via SSE
- [ ] Generation uses sequential thinking when server is available
- [ ] Gracefully falls back to direct generation if server unavailable
- [ ] Document quality improves (measurable via validation pass rate)
- [ ] Can toggle thinking on/off via config
- [ ] `--show-thinking` displays reasoning steps with timestamps
- [ ] Network errors are handled gracefully with clear user feedback

---

### Phase 3.1: Filesystem MCP Integration

**Objective**: Provide LLM with controlled filesystem access via local MCP server for reading existing planning documents and project files.

#### Rationale

The LLM needs filesystem access to:
1. **Read existing planning documents** - Build context from previously generated docs
2. **Inspect project structure** - Understand folder layouts for architecture generation
3. **Reference template files** - Ensure generated documents match template structure
4. **Read user notes** - Incorporate insights from Obsidian vault

#### Infrastructure Details

**Filesystem MCP Server**:
- **Deployment**: Local child process spawned by nexus
- **Package**: `@modelcontextprotocol/server-filesystem` (stdio-based MCP)
- **Protocol**: JSON-RPC over stdio (stdin/stdout)
- **Configuration**: Environment variable `FILESYSTEM_PATHS` in `.env` file
- **Allowed Paths**: Colon-separated list (e.g., `/home/nullvektor/oblivion:/home/nullvektor/repos`)

**Security Model**:
- MCP server enforces path restrictions (cannot access files outside configured paths)
- Read-only access by default (no writes without explicit tool use)
- Process lifecycle managed by catalyst engine (spawn on start, cleanup on exit)

#### Tasks

1. **Create MCP process manager in `src/catalyst/filesystem_mcp.rs`**
   ```rust
   use std::process::{Child, Command, Stdio};
   use std::io::{BufRead, BufReader, Write};
   use serde_json::Value;
   
   pub struct FilesystemMcpServer {
       process: Child,
       allowed_paths: Vec<String>,
   }
   
   impl FilesystemMcpServer {
       /// Spawn the filesystem MCP server as a child process
       pub fn spawn() -> Result<Self> {
           // Read paths from environment
           let paths_str = std::env::var("FILESYSTEM_PATHS")
               .unwrap_or_else(|_| "/home/nullvektor/oblivion:/home/nullvektor/repos".to_string());
           
           let allowed_paths: Vec<String> = paths_str
               .split(':')
               .map(|s| s.to_string())
               .collect();
           
           // Build command arguments
           let mut args = vec![
               "-y".to_string(),
               "@modelcontextprotocol/server-filesystem".to_string(),
           ];
           args.extend(allowed_paths.clone());
           
           // Spawn process
           let mut process = Command::new("npx")
               .args(&args)
               .stdin(Stdio::piped())
               .stdout(Stdio::piped())
               .stderr(Stdio::piped())
               .spawn()
               .context("Failed to spawn filesystem MCP server")?;
           
           Ok(Self {
               process,
               allowed_paths,
           })
       }
       
       /// Send JSON-RPC request to MCP server
       pub fn call_tool(&mut self, tool_name: &str, params: Value) -> Result<Value> {
           let request = serde_json::json!({
               "jsonrpc": "2.0",
               "id": 1,
               "method": tool_name,
               "params": params,
           });
           
           // Write to stdin
           let stdin = self.process.stdin.as_mut()
               .ok_or_else(|| anyhow::anyhow!("Failed to get stdin"))?;
           
           writeln!(stdin, "{}", request.to_string())?;
           stdin.flush()?;
           
           // Read from stdout
           let stdout = self.process.stdout.as_mut()
               .ok_or_else(|| anyhow::anyhow!("Failed to get stdout"))?;
           
           let reader = BufReader::new(stdout);
           let mut line = String::new();
           reader.read_line(&mut line)?;
           
           let response: Value = serde_json::from_str(&line)?;
           Ok(response)
       }
       
       /// Read file via MCP server
       pub fn read_file(&mut self, path: &str) -> Result<String> {
           let params = serde_json::json!({ "path": path });
           let response = self.call_tool("read_file", params)?;
           
           // Extract content from response
           let content = response["result"]["content"]
               .as_str()
               .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
               .to_string();
           
           Ok(content)
       }
       
       /// List directory via MCP server
       pub fn list_directory(&mut self, path: &str) -> Result<Vec<String>> {
           let params = serde_json::json!({ "path": path });
           let response = self.call_tool("list_directory", params)?;
           
           // Extract file list from response
           let files: Vec<String> = response["result"]["files"]
               .as_array()
               .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
               .iter()
               .filter_map(|v| v.as_str().map(String::from))
               .collect();
           
           Ok(files)
       }
   }
   
   impl Drop for FilesystemMcpServer {
       fn drop(&mut self) {
           // Gracefully shutdown MCP server on drop
           let _ = self.process.kill();
           let _ = self.process.wait();
       }
   }
   ```

2. **Add environment variable support**
   - Update `.env.example`:
     ```bash
     SEQUENTIAL_THINKING_URL=http://100.105.8.97:8000/sse
     FILESYSTEM_PATHS=/home/nullvektor/oblivion:/home/nullvektor/repos
     ```
   - Document that paths should be colon-separated
   - Default paths if env var not set

3. **Integrate with `CatalystEngine`**
   ```rust
   pub struct CatalystEngine {
       project_id: String,
       obsidian_path: PathBuf,
       llm_client: LlmClient,
       filesystem_mcp: Option<FilesystemMcpServer>,
   }
   
   impl CatalystEngine {
       pub fn new(
           project_id: String,
           obsidian_path: PathBuf,
           llm_config: &LlmConfig,
       ) -> Result<Self> {
           // Spawn filesystem MCP if enabled
           let filesystem_mcp = if config.catalyst.use_filesystem_mcp {
               Some(FilesystemMcpServer::spawn()?)
           } else {
               None
           };
           
           Ok(Self {
               project_id,
               obsidian_path,
               llm_client: create_llm_client(llm_config)?,
               filesystem_mcp,
           })
       }
   }
   ```

4. **Use MCP for reading planning documents**
   - Replace direct `fs::read_to_string()` calls with MCP reads
   - Add fallback: if MCP unavailable, use direct filesystem access
   - Log when MCP is used vs direct access for debugging
   
   ```rust
   fn load_vision_document(&mut self) -> Result<VisionData> {
       let vision_path = self.obsidian_path
           .join("01-PLANNING/01-Problem-and-Vision.md");
       
       let content = if let Some(ref mut mcp) = self.filesystem_mcp {
           // Use MCP to read file
           mcp.read_file(&vision_path.to_string_lossy())?
       } else {
           // Fallback to direct filesystem access
           std::fs::read_to_string(&vision_path)?
       };
       
       parse_vision_content(&content)
   }
   ```

5. **Add configuration in `nexus.toml`**
   ```toml
   [catalyst]
   enabled = true
   use_thinking = true
   use_filesystem_mcp = true  # Enable filesystem MCP integration
   filesystem_timeout_secs = 10
   show_thinking = false
   ```

6. **Error handling**
   - MCP spawn failure: Fall back to direct filesystem access
   - Tool call timeout: Retry once, then fallback
   - Invalid path (outside allowed): Clear error message
   - Process crash: Detect and restart MCP if needed

7. **Add logging for transparency**
   - Log when MCP server is spawned
   - Log each tool call (file read, directory list)
   - Log fallback to direct access
   - Helps debug permission issues

#### Acceptance Criteria for Phase 3.1
- [ ] Successfully spawns filesystem MCP server with configured paths
- [ ] Can read planning documents via MCP `read_file` tool
- [ ] Can list directories via MCP `list_directory` tool
- [ ] Gracefully falls back to direct filesystem access if MCP unavailable
- [ ] Process cleanup on engine drop (no zombie processes)
- [ ] Respects allowed path restrictions
- [ ] Clear error messages for permission violations
- [ ] Logging shows MCP usage vs direct access

#### Integration with Document Generation

The filesystem MCP enables the LLM to:

1. **Build Progressive Context**:
   ```rust
   // When generating Tech Stack, read previously generated Scope
   let scope_content = filesystem_mcp.read_file("02-Scope-and-Boundaries.md")?;
   let scope_data = parse_scope_document(&scope_content)?;
   context.scope = Some(scope_data);
   ```

2. **Reference Templates**:
   ```rust
   // Read template to ensure output format matches
   let template = filesystem_mcp.read_file("templates/project/03-Tech-Stack.md")?;
   prompt.add_context("Template structure", &template);
   ```

3. **Inspect Project Files**:
   ```rust
   // For architecture generation, read existing folder structure
   let files = filesystem_mcp.list_directory(&project_repo_path)?;
   prompt.add_context("Existing project structure", &files.join("\n"));
   ```

---

### Phase 4: Refinement & Iteration

**Objective**: Allow user to refine generated documents with feedback.

#### Tasks

1. **Implement refinement in `CatalystEngine`**
   ```rust
   pub async fn refine_document(
       &self,
       doc_type: DocumentType,
       feedback: &str,
   ) -> Result<()>;
   ```
   
   Logic:
   - Load existing generated document
   - Include in context: "Previous attempt: {content}"
   - Include user feedback: "User feedback: {feedback}"
   - Regenerate with refinements
   - Validate and save

2. **Add conversation history integration**
   - Use existing [`ConversationHistory`](src/history.rs) module
   - Track generation attempts for each document
   - Include previous attempts in context

3. **Add `catalyst refine` command**
   ```bash
   catalyst refine scope "Add mobile app to MVP"
   catalyst refine stack "Use PostgreSQL instead of SQLite"
   ```

4. **Implement retry logic with validation**
   - If validation fails, auto-retry with error feedback
   - Max 3 attempts per document
   - Save draft with `.draft` extension if still failing

5. **Add status command**
   ```bash
   catalyst status
   ```
   Shows:
   - Which documents have been generated
   - Which passed validation
   - Which need refinement

#### Acceptance Criteria for Phase 4
- [ ] Can refine individual documents with feedback
- [ ] Refinement preserves document structure
- [ ] Retry logic handles validation failures
- [ ] Status command shows generation state

---

### Phase 5: Polish & Testing

**Objective**: Production-ready quality and comprehensive testing.

#### Tasks

1. **Error handling**
   - Graceful LLM API failures
   - Clear error messages for missing config
   - Handle partial generation state
   - Allow resume after interruption

2. **Integration tests**
   - Test with real vision documents
   - Verify gate validation passes
   - Test refinement workflow
   - Test error scenarios

3. **Documentation**
   - Update [`README.md`](README.md) with catalyst usage
   - Add examples to help text
   - Document configuration options
   - Create troubleshooting guide

4. **Performance optimization**
   - Parallel document generation where possible (if no dependencies)
   - Cache parsed context to avoid re-parsing
   - Optimize prompt sizes

5. **Dogfooding**
   - Use catalyst to plan a real project
   - Document pain points
   - Iterate on UX based on real usage

#### Acceptance Criteria for Phase 5
- [ ] All integration tests pass
- [ ] Successfully used on at least 1 real project
- [ ] Documentation complete
- [ ] No known critical bugs

---

## Technical Implementation Details

### Vision Document Parsing

Use existing markdown parser from [`src/planning.rs`](src/planning.rs):

```rust
use pulldown_cmark::{Event, Options, Parser, Tag};

fn parse_vision_document(path: &Path) -> Result<VisionData> {
    let content = fs::read_to_string(path)?;
    let mut parser = Parser::new(&content);
    
    let mut current_section: Option<String> = None;
    let mut sections: HashMap<String, String> = HashMap::new();
    
    // Extract sections via markdown parser
    // ... (similar to existing planning.rs logic)
    
    Ok(VisionData {
        problem: sections.get("My problem (personal):").cloned().unwrap_or_default(),
        solution: sections.get("Solution in ONE SENTENCE:").cloned().unwrap_or_default(),
        success_criteria: sections.get("Success criteria (3 months):").cloned().unwrap_or_default(),
        anti_vision: sections.get("Anti-vision (what this project is NOT):").cloned().unwrap_or_default(),
    })
}
```

### LLM Client Usage

Re-use existing [`LlmClient`](src/llm.rs:29):

```rust
use crate::llm::{LlmClient, LlmProvider};

async fn generate_document(
    prompt: &str,
    llm_client: &LlmClient,
) -> Result<String> {
    llm_client.complete(prompt).await
}
```

### Validation Integration

Re-use existing validation from [`src/planning.rs`](src/planning.rs):

```rust
use crate::planning::validate_planning_document_with_headers;
use crate::heuristics::GateHeuristics;

fn validate_scope_document(content: &str) -> Result<bool> {
    let required_headers = vec![
        "MVP (Minimum Viable Product):".to_string(),
        "Version 2 (NOT NOW - just document):".to_string(),
        "Never (things I will NOT build):".to_string(),
        "Tech constraints:".to_string(),
    ];
    
    let validation = validate_planning_document_with_headers(
        Path::new("02-Scope-and-Boundaries.md"),
        &required_headers,
        50, // min word count
        &["TODO", "TBD", "[fill]", "[describe]"],
    )?;
    
    Ok(validation.passed)
}
```

### Configuration Loading

Use existing [`NexusConfig`](src/config.rs):

```rust
use crate::config::NexusConfig;

fn load_project_config(repo_path: &Path) -> Result<NexusConfig> {
    let config_path = repo_path.join("nexus.toml");
    let content = fs::read_to_string(&config_path)?;
    let config: NexusConfig = toml::from_str(&content)?;
    Ok(config)
}
```

---

## Configuration Schema

Add to `src/config.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalystConfig {
    pub enabled: bool,
    #[serde(default = "default_use_thinking")]
    pub use_thinking: bool,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default)]
    pub show_thinking: bool,
}

fn default_use_thinking() -> bool { true }
fn default_max_retries() -> u32 { 3 }

// Add to NexusConfig struct:
pub catalyst: Option<CatalystConfig>,
```

Example `nexus.toml`:

```toml
[catalyst]
enabled = true
use_thinking = true
max_retries = 3
show_thinking = false
```

---

## REPL Integration

Update [`src/commands/shell.rs`](src/commands/shell.rs):

```rust
// Add to execute_command match:
"catalyst" => execute_catalyst(state, args),

// Implement handler:
fn execute_catalyst(state: &NexusState, args: &[&str]) -> Result<()> {
    let project_id = state.active_project_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active project. Use 'use <project>' first."))?;
    
    let obsidian_path = state.get_active_obsidian_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get Obsidian path"))?;
    
    let repo_path = state.get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("Failed to get repo path"))?;
    
    // Load config
    let config = load_project_config(&repo_path)?;
    
    // Verify LLM is configured
    let llm_config = config.llm.as_ref()
        .ok_or_else(|| anyhow::anyhow!("LLM not configured in nexus.toml"))?;
    
    if !llm_config.enabled {
        anyhow::bail!("LLM is disabled. Set 'enabled = true' in [llm] section");
    }
    
    // Get API key from environment
    let api_key = std::env::var("OPENROUTER_API_KEY")
        .context("OPENROUTER_API_KEY not set")?;
    
    // Create runtime for async operations
    let runtime = tokio::runtime::Runtime::new()?;
    
    runtime.block_on(async {
        use crate::catalyst::CatalystEngine;
        use crate::llm::{LlmClient, LlmProvider};
        
        let provider = LlmProvider::from_str(&llm_config.provider)
            .ok_or_else(|| anyhow::anyhow!("Invalid provider"))?;
        
        let llm_client = LlmClient::new(provider, api_key, llm_config.model.clone());
        
        let engine = CatalystEngine::new(
            project_id.clone(),
            obsidian_path.clone(),
            llm_client,
        )?;
        
        match args.get(0).map(|s| s.to_lowercase().as_str()) {
            Some("generate") => engine.generate_all().await,
            Some("scope") => engine.generate_scope().await,
            Some("stack") => engine.generate_tech_stack().await,
            Some("arch") => engine.generate_architecture().await,
            Some("mvp") => engine.generate_mvp_breakdown().await,
            Some("refine") => {
                let doc = args.get(1)
                    .ok_or_else(|| anyhow::anyhow!("Usage: catalyst refine <doc> <feedback>"))?;
                let feedback = args[2..].join(" ");
                engine.refine_document(parse_doc_type(doc)?, &feedback).await
            }
            Some("status") => show_catalyst_status(&obsidian_path),
            _ => {
                print_catalyst_help();
                Ok(())
            }
        }
    })
}
```

---

## Testing Strategy

### Unit Tests
- Test vision document parsing
- Test prompt template rendering
- Test document validation
- Test context building logic

### Integration Tests
Create `tests/catalyst_integration.rs`:

```rust
#[tokio::test]
async fn test_generate_scope_document() {
    // Setup: Create test project with vision
    let temp_dir = tempfile::tempdir().unwrap();
    create_test_vision(&temp_dir);
    
    // Execute: Generate scope
    let engine = create_test_engine(&temp_dir);
    engine.generate_scope().await.unwrap();
    
    // Verify: Scope document exists and passes validation
    let scope_path = temp_dir.path().join("02-Scope-and-Boundaries.md");
    assert!(scope_path.exists());
    
    let content = fs::read_to_string(&scope_path).unwrap();
    assert!(content.contains("## MVP (Minimum Viable Product):"));
}
```

### Manual Testing Checklist
- [ ] Generate scope from real vision document
- [ ] Generate all documents via `catalyst generate`
- [ ] Refine a document with feedback
- [ ] Verify all documents pass gate validation
- [ ] Test retry logic with invalid documents
- [ ] Test with/without sequential thinking
- [ ] Test error handling (missing API key, network failure)

---

## Dependencies

Add to `Cargo.toml` (if not already present):

```toml
[dependencies]
# Existing dependencies already in project:
anyhow = "1.0"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
pulldown-cmark = { version = "0.11", features = ["simd"] }
reqwest = { version = "0.12", features = ["json"] }
colored = "2.1"

# No new dependencies required - reuse existing infrastructure!
```

---

## Success Criteria Summary

The Planning Catalyst is complete when:

1. **Functional**:
   - User can run `catalyst generate` to create all planning documents
   - Generated documents pass gate validation (>90% success rate)
   - User can refine documents with natural language feedback

2. **Quality**:
   - Documents match template structure exactly
   - Content is contextually relevant to vision
   - Sequential thinking produces coherent reasoning

3. **UX**:
   - Clear progress indicators during generation
   - Helpful error messages
   - Intuitive REPL commands

4. **Validated**:
   - Successfully used to plan at least 1 real project
   - All integration tests pass
   - Documentation complete

---

## Getting Started

**Recommended approach**:

1. Start with Phase 1 (Core Infrastructure)
2. Get scope generation working end-to-end
3. Test with a real vision document
4. Proceed to Phase 2 once scope generation is solid
5. Iterate based on actual usage

**First concrete step**:
Create `src/catalyst/mod.rs` and implement the basic module structure, then implement vision document parsing in `engine.rs`.

**Reference files**:
- Design: [`PLANNING_CATALYST_DESIGN.md`](PLANNING_CATALYST_DESIGN.md)
- Existing LLM: [`src/llm.rs`](src/llm.rs)
- Existing validation: [`src/planning.rs`](src/planning.rs)
- REPL integration: [`src/commands/shell.rs`](src/commands/shell.rs)
- Templates: [`templates/project/`](templates/project/)

Good luck! 🚀
