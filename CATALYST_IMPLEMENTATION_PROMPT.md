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

**Objective**: Integrate MCP `sequentialthinking` server for improved reasoning quality.

#### Tasks

1. **Create MCP wrapper in `src/catalyst/thinking.rs`**
   ```rust
   pub struct ThinkingSession {
       thoughts: Vec<ThoughtStep>,
       final_answer: Option<String>,
   }
   
   pub struct ThoughtStep {
       thought_number: u32,
       content: String,
       is_revision: bool,
   }
   
   pub async fn generate_with_thinking(
       prompt: &str,
       llm_client: &LlmClient,
   ) -> Result<(String, ThinkingSession)>;
   ```

2. **Update prompts to request reasoning**
   - Modify templates to request "step-by-step thinking"
   - Structure: "Think through X, then provide final document"
   - Guide model to use MCP thinking tool

3. **Integrate with generator**
   - Update `DocumentGenerator` to use thinking when enabled
   - Capture thinking steps for transparency
   - Extract final answer from thinking session

4. **Add configuration**
   - Add `[catalyst]` section to `nexus.toml`:
     ```toml
     [catalyst]
     enabled = true
     use_thinking = true
     show_thinking = false  # Debug mode
     ```

5. **Add `--show-thinking` flag**
   - Optional flag to display reasoning process
   - Helps debug generation quality issues

#### Acceptance Criteria for Phase 3
- [ ] Generation uses sequential thinking MCP
- [ ] Document quality improves (subjective but measurable)
- [ ] Can toggle thinking on/off via config
- [ ] `--show-thinking` displays reasoning steps

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
