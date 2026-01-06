# Planning Catalyst - Architecture & Implementation Plan

## Problem Statement

When starting a new project, filling out all 5 planning documents (01-05) manually is time-consuming and mentally draining. The user often has a clear vision in [`01-Problem-and-Vision.md`](templates/project/01-Problem-and-Vision.md) but struggles to systematically break it down into:
- Scope and boundaries
- Tech stack decisions
- Architecture design
- MVP breakdown into sprints

**Solution**: A "Planning Catalyst" that uses DeepSeek R1 (with sequential thinking) to intelligently generate planning documents from the initial vision, while maintaining the user's agency and allowing them to refine/override AI suggestions.

---

## Current State Analysis

### Existing LLM Integration
- OpenRouter integration via [`src/llm.rs`](src/llm.rs:60)
- Default model: `deepseek/deepseek-r1` (configured in `nexus.toml`)
- Context injection from:
  - Architecture snippets (Qdrant vector search)
  - Active sprint tasks (Obsidian files)
  - Conversation history (last 5 turns)
- Exposed via REPL natural language queries

### Available MCP Servers
- `sequentialthinking` - provides structured reasoning via [`mcp_sequentialthinking_sequentialthinking`](src/commands/shell.rs:909)
- `context7` - documentation lookup
- `filesystem` - file operations

### Planning Documents Structure
All documents follow a consistent template structure with:
- Required sections (headers)
- Minimum word counts
- Checkboxes for completion tracking
- Anti-patterns (illegal strings like "TODO", "TBD")

### Gate Validation
- [`src/planning.rs`](src/planning.rs:84) validates documents against heuristics
- [`src/heuristics.rs`](src/heuristics.rs) defines validation rules
- Blocks if documents are incomplete

---

## Architecture Design

### High-Level Flow

```
User writes 01-Problem-and-Vision.md manually
         ↓
User runs: catalyst generate
         ↓
CatalystEngine orchestrates generation of 02-05:
  1. Load 01-Problem-and-Vision.md
  2. Generate 02-Scope-and-Boundaries.md (uses thinking MCP)
  3. Generate 03-Tech-Stack.md (builds on scope)
  4. Generate 04-Architecture.md (builds on stack)
  5. Generate 05-MVP-Breakdown.md (builds on architecture)
  6. Validate each document against gate heuristics
  7. Save to Obsidian vault
         ↓
User reviews/refines generated docs in Obsidian
         ↓
User runs: nexus gate
         ↓
(Standard workflow continues: unlock, sprint, etc.)
```

### Component Architecture

```
src/catalyst/
├── mod.rs                  # Public API
├── engine.rs               # CatalystEngine - orchestration
├── generator.rs            # DocumentGenerator - per-doc logic
├── prompts.rs              # Prompt templates for each document
├── sequential_thinking.rs  # MCP integration wrapper
└── validation.rs           # Pre/post validation
```

### Module Responsibilities

#### 1. CatalystEngine ([`src/catalyst/engine.rs`](src/catalyst/engine.rs))
**Purpose**: Orchestrates the sequential generation of planning documents.

**Key Methods**:
```rust
pub struct CatalystEngine {
    project_id: String,
    obsidian_path: PathBuf,
    llm_client: LlmClient,
    thinking_enabled: bool, // Use sequential thinking MCP
}

impl CatalystEngine {
    /// Generate all planning documents (02-05) from 01
    pub async fn generate_all(&self) -> Result<GenerationReport>;
    
    /// Generate a specific document only
    pub async fn generate_document(&self, doc_type: DocumentType) -> Result<()>;
    
    /// Regenerate a document with user feedback
    pub async fn refine_document(&self, doc_type: DocumentType, feedback: &str) -> Result<()>;
}
```

**Orchestration Logic**:
1. Load [`01-Problem-and-Vision.md`](templates/project/01-Problem-and-Vision.md)
2. Extract structured data (problem, vision, anti-vision, etc.)
3. For each document type (Scope, Stack, Architecture, MVP):
   - Build context from previously generated docs
   - Generate using appropriate prompt template
   - Validate against gate heuristics
   - Save to Obsidian vault
   - Report progress to user

#### 2. DocumentGenerator ([`src/catalyst/generator.rs`](src/catalyst/generator.rs))
**Purpose**: Handles generation logic for individual document types.

**Key Methods**:
```rust
pub enum DocumentType {
    Scope,           // 02-Scope-and-Boundaries.md
    TechStack,       // 03-Tech-Stack.md
    Architecture,    // 04-Architecture.md
    MvpBreakdown,    // 05-MVP-Breakdown.md
}

pub struct DocumentGenerator {
    doc_type: DocumentType,
    llm_client: LlmClient,
}

impl DocumentGenerator {
    /// Generate document content using LLM and sequential thinking
    pub async fn generate(&self, context: &GenerationContext) -> Result<String>;
    
    /// Validate generated content meets template requirements
    pub fn validate(&self, content: &str) -> Result<ValidationResult>;
}
```

**Generation Context**:
```rust
pub struct GenerationContext {
    /// Always present - the foundation
    pub vision: VisionData,
    
    /// Optional - builds as we progress
    pub scope: Option<ScopeData>,
    pub tech_stack: Option<TechStackData>,
    pub architecture: Option<ArchitectureData>,
    
    /// User constraints from nexus.toml
    pub project_constraints: ProjectConstraints,
}
```

#### 3. Prompt Templates ([`src/catalyst/prompts.rs`](src/catalyst/prompts.rs))
**Purpose**: Specialized prompts for each document type that guide DeepSeek R1's reasoning.

**Structure**:
```rust
pub struct PromptTemplate {
    system_prompt: String,
    user_prompt_template: String,
    output_format: String,
    thinking_enabled: bool,
}

impl PromptTemplate {
    pub fn for_scope() -> Self { /* ... */ }
    pub fn for_tech_stack() -> Self { /* ... */ }
    pub fn for_architecture() -> Self { /* ... */ }
    pub fn for_mvp_breakdown() -> Self { /* ... */ }
    
    /// Render the prompt with context data
    pub fn render(&self, context: &GenerationContext) -> String;
}
```

**Example Prompt for Scope Document**:
```markdown
# Task: Generate Scope and Boundaries Document

## Context
You are helping plan a software project. The user has described their problem and vision:

**Problem**: {vision.problem}
**Solution (1 sentence)**: {vision.solution}
**Success Criteria**: {vision.success_criteria}
**Anti-Vision**: {vision.anti_vision}

## Your Task
Generate a "02-Scope-and-Boundaries.md" document that defines:
1. **MVP** - Minimum viable features (max 5)
2. **Version 2** - Future features (document but NOT now)
3. **Never** - What will NOT be built
4. **Tech Constraints** - Budget, timeline, platform limitations

## Reasoning Process
Use step-by-step thinking to:
1. Identify core features from the problem statement
2. Distinguish between "must have" (MVP) and "nice to have" (V2)
3. Identify scope creep risks from anti-vision
4. Determine realistic constraints

## Output Format
Your response MUST match this markdown structure exactly:

```markdown
# What am I building? (Scope)

## MVP (Minimum Viable Product):
[List max 5 features that solve the core problem]

## Version 2 (NOT NOW - just document):
[List future features]

## Never (things I will NOT build):
[List anti-features based on anti-vision]

## Tech constraints:
- Budget: [amount or "0 SEK"]
- Deadline: [timeframe]
- Platform: [target platform]
- Integration: [must work with...]
```

Begin your reasoning, then provide the final document.
```

#### 4. Sequential Thinking Integration ([`src/catalyst/sequential_thinking.rs`](src/catalyst/sequential_thinking.rs))
**Purpose**: Wrapper for the `sequentialthinking` MCP server integration.

**Key Methods**:
```rust
pub struct ThinkingSession {
    current_thought: u32,
    total_thoughts: u32,
    thoughts: Vec<ThoughtStep>,
}

pub struct ThoughtStep {
    thought_number: u32,
    content: String,
    is_revision: bool,
    revises_thought: Option<u32>,
}

/// Execute a thinking session for document generation
pub async fn generate_with_thinking(
    prompt: &str,
    llm_client: &LlmClient,
) -> Result<(String, ThinkingSession)>;
```

**Integration Strategy**:
- Use MCP `mcp--sequentialthinking--sequentialthinking` tool
- Let DeepSeek R1 reason through document structure
- Capture thinking steps for debugging/transparency
- Extract final answer from thinking session

#### 5. Validation Layer ([`src/catalyst/validation.rs`](src/catalyst/validation.rs))
**Purpose**: Ensure generated documents meet gate heuristics before saving.

**Key Methods**:
```rust
/// Validate generated document against template requirements
pub fn validate_generated_document(
    content: &str,
    doc_type: DocumentType,
) -> Result<ValidationResult>;

/// Check if document has required sections
pub fn has_required_sections(content: &str, required: &[String]) -> bool;

/// Check if document meets minimum word count
pub fn meets_word_count(content: &str, min_words: usize) -> bool;

/// Check for illegal placeholder strings
pub fn has_illegal_strings(content: &str, illegal: &[String]) -> Vec<String>;
```

**Validation Strategy**:
1. Re-use existing [`validate_planning_document`](src/planning.rs:84) logic
2. Add catalyst-specific checks (e.g., ensure AI didn't hallucinate sections)
3. If validation fails: retry with feedback to LLM
4. Max 3 retry attempts before surfacing error to user

---

## REPL Command Interface

### New Commands in Shell

Add these commands to [`execute_command`](src/commands/shell.rs:193):

```rust
"catalyst" => execute_catalyst(state, args),
```

### Command Structure

```bash
# Generate all planning documents from vision
nexus❯ catalyst generate

# Generate specific document only
nexus❯ catalyst scope      # 02-Scope-and-Boundaries.md
nexus❯ catalyst stack      # 03-Tech-Stack.md
nexus❯ catalyst arch       # 04-Architecture.md
nexus❯ catalyst mvp        # 05-MVP-Breakdown.md

# Refine a document with feedback
nexus❯ catalyst refine scope "Add mobile app to MVP"

# Show generation status
nexus❯ catalyst status
```

### Command Execution Flow

```rust
fn execute_catalyst(state: &NexusState, args: &[&str]) -> Result<()> {
    let project_id = state.active_project_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active project"))?;
    
    let obsidian_path = state.get_active_obsidian_path()
        .ok_or_else(|| anyhow::anyhow!("No Obsidian path"))?;
    
    let repo_path = state.get_active_repo_path()
        .ok_or_else(|| anyhow::anyhow!("No repo path"))?;
    
    // Load config for LLM settings
    let config = load_config(&repo_path)?;
    
    // Verify LLM is configured
    let llm_config = config.llm.as_ref()
        .ok_or_else(|| anyhow::anyhow!("LLM not configured"))?;
    
    if !llm_config.enabled {
        anyhow::bail!("LLM is disabled. Enable it in nexus.toml");
    }
    
    // Create catalyst engine
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let engine = CatalystEngine::new(
            project_id.clone(),
            obsidian_path,
            llm_config,
        ).await?;
        
        match args.get(0).map(|s| s.to_lowercase().as_str()) {
            Some("generate") => engine.generate_all().await,
            Some("scope") => engine.generate_document(DocumentType::Scope).await,
            Some("stack") => engine.generate_document(DocumentType::TechStack).await,
            Some("arch") => engine.generate_document(DocumentType::Architecture).await,
            Some("mvp") => engine.generate_document(DocumentType::MvpBreakdown).await,
            Some("refine") => {
                let doc = args.get(1).ok_or_else(|| anyhow::anyhow!("Missing document type"))?;
                let feedback = args[2..].join(" ");
                let doc_type = parse_doc_type(doc)?;
                engine.refine_document(doc_type, &feedback).await
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

## Sequential Thinking Strategy

### Why DeepSeek R1 + Sequential Thinking?

1. **Complex Reasoning**: Breaking down a vision into structured planning requires multi-step reasoning
2. **Self-Correction**: Sequential thinking allows the model to revise previous thoughts
3. **Transparency**: We can show thinking steps to user for debugging
4. **Cost-Effective**: DeepSeek R1 via OpenRouter is cheaper than Claude while still powerful

### Integration with MCP

Use the existing `sequentialthinking` MCP server:

```rust
// Example thinking session for scope generation
let thinking_request = ThinkingRequest {
    thought: "Let me analyze the problem statement to identify core features",
    next_thought_needed: true,
    thought_number: 1,
    total_thoughts: 10, // Initial estimate
    is_revision: false,
    revises_thought: None,
    branch_from_thought: None,
    branch_id: None,
    needs_more_thoughts: false,
};

// Iteratively build thoughts until conclusion
while thinking_session.next_thought_needed {
    let response = mcp_client.call("sequentialthinking", thinking_request).await?;
    thinking_session.add_thought(response);
}

// Extract final answer
let generated_document = thinking_session.get_final_answer();
```

### Prompt Engineering for Thinking

Structure prompts to guide the thinking process:

```markdown
You will generate this document using step-by-step reasoning.

THINKING PROCESS:
1. Analyze the vision and identify key constraints
2. List all potential features mentioned or implied
3. Categorize features: Must-Have vs Nice-to-Have vs Never
4. Justify each categorization
5. Synthesize into final MVP list (max 5 items)
6. Verify against anti-vision to ensure no scope creep
7. Format final document

After your reasoning, output the final document in the required format.
```

---

## Implementation Plan

### Phase 1: Core Infrastructure (Sprint 1)
**Goal**: Basic catalyst engine without thinking MCP

Tasks:
- [ ] Create `src/catalyst/mod.rs` module structure
- [ ] Implement [`CatalystEngine`](src/catalyst/engine.rs) with sync document generation
- [ ] Implement [`PromptTemplate`](src/catalyst/prompts.rs) for Scope document only
- [ ] Add `catalyst scope` command to REPL
- [ ] Test: Generate [`02-Scope-and-Boundaries.md`](templates/project/02-Scope-and-Boundaries.md) from a sample vision

**Exit Criteria**: Can run `catalyst scope` and get a valid scope document that passes gate validation

### Phase 2: All Documents (Sprint 2)
**Goal**: Generate all planning documents sequentially

Tasks:
- [ ] Implement prompts for Tech Stack, Architecture, MVP Breakdown
- [ ] Add progressive context building (each doc builds on previous)
- [ ] Implement `catalyst generate` for full workflow
- [ ] Add validation retry logic (max 3 attempts)
- [ ] Add progress indicators in REPL

**Exit Criteria**: `catalyst generate` produces all 4 documents (02-05) that pass gate validation

### Phase 3: Sequential Thinking Integration (Sprint 3)
**Goal**: Integrate MCP sequential thinking for better reasoning

Tasks:
- [ ] Implement [`sequential_thinking.rs`](src/catalyst/sequential_thinking.rs) MCP wrapper
- [ ] Update prompts to request step-by-step reasoning
- [ ] Add thinking session capture and display
- [ ] Add `--show-thinking` flag to see reasoning process
- [ ] Compare quality: with/without thinking

**Exit Criteria**: Generated documents show evidence of multi-step reasoning and are higher quality than basic generation

### Phase 4: Refinement & Polish (Sprint 4)
**Goal**: User feedback integration and iterative refinement

Tasks:
- [ ] Implement `catalyst refine <doc> <feedback>` command
- [ ] Add conversation history to catalyst context
- [ ] Implement `catalyst status` to show which docs are generated
- [ ] Add dry-run mode (`--preview`) to preview without saving
- [ ] Write integration tests with real vision documents

**Exit Criteria**: User can iteratively refine generated documents via feedback

### Phase 5: Dogfooding (Sprint 5)
**Goal**: Use catalyst to plan a real project

Tasks:
- [ ] Write vision for a new test project
- [ ] Use catalyst to generate all planning docs
- [ ] Document pain points and UX issues
- [ ] Fix discovered bugs
- [ ] Update documentation with examples

**Exit Criteria**: Successfully used catalyst to plan at least one real project end-to-end

---

## Configuration

### nexus.toml Updates

Add catalyst-specific configuration:

```toml
[catalyst]
enabled = true
# Which LLM provider to use for generation (defaults to [llm] settings)
provider = "openrouter"
model = "deepseek/deepseek-r1"
# Enable sequential thinking MCP integration
use_thinking = true
# Maximum retry attempts for validation failures
max_retries = 3
# Show thinking process in output
show_thinking = false
```

---

## Prompt Templates Detail

### Template for 02-Scope-and-Boundaries.md

```markdown
# Generate Scope and Boundaries Document

## Input Context
**Vision**: {vision.solution}
**Problem**: {vision.problem}
**Success Criteria**: {vision.success_criteria}
**Anti-Vision**: {vision.anti_vision}

## Task
Create a scope document defining:
1. MVP features (max 5) - what MUST be built
2. Version 2 features - what COULD be built later
3. Never features - what will NOT be built
4. Technical constraints - budget, timeline, platform

## Reasoning Guidelines
- MVP should directly address the success criteria
- Version 2 should enhance but not be critical
- Never list should include anything in anti-vision
- Constraints should be realistic for solo dev

## Output Format
Follow this exact structure:

# What am I building? (Scope)

## MVP (Minimum Viable Product):
- [Feature 1]
- [Feature 2]
...

## Version 2 (NOT NOW - just document):
- [Future feature 1]
...

## Never (things I will NOT build):
- [Anti-feature 1]
...

## Tech constraints:
- Budget: [amount]
- Deadline: [timeframe]
- Platform: [platform]
- Integration: [requirements]

Think step-by-step about each section, then provide the final document.
```

### Template for 03-Tech-Stack.md

```markdown
# Generate Tech Stack Document

## Input Context
**Vision**: {vision.solution}
**MVP Features**: {scope.mvp}
**Constraints**: {scope.constraints}
**Anti-Scope**: {scope.never}

## Task
Choose a technology stack including:
1. Programming language
2. Key frameworks/libraries
3. Database (if needed)
4. Deployment platform

## Reasoning Guidelines
- Match language to MVP complexity and timeline
- Consider developer familiarity (user profile: {user.experience})
- Respect budget constraints (0 SEK = no paid services)
- Justify each choice with reasoning
- List what NOT to use and why

## Output Format
# Technical choices

## Stack (force yourself to choose NOW):
- **Language:** [language]
- **Framework:** [framework]
- **Database:** [database or "none"]
...

## Why these choices?
[Paragraph explaining each choice]

## What I will NOT use:
- Not: [rejected option] - [reason]
...

## Dependencies (max 10 important ones):
1. [dependency 1]
...

Think through technology trade-offs, then provide the final document.
```

### Template for 04-Architecture.md

```markdown
# Generate Architecture Document

## Input Context
**Vision**: {vision.solution}
**MVP**: {scope.mvp}
**Tech Stack**: {stack.language}, {stack.framework}, {stack.database}

## Task
Design system architecture including:
1. Folder structure
2. Data model (main entities)
3. User flow (how features work)
4. Critical technical decisions

## Reasoning Guidelines
- Folder structure should support {stack.language} conventions
- Data model should reflect MVP features
- User flow should map to success criteria
- Technical decisions should justify key trade-offs

## Output Format
# System design

## Folder structure:
```
project-name/
├── [folders based on tech stack]
```

## Data model (main entities):
1. **Entity1**: [description]
...

## Flow (user journey):
1. User [action]
2. System [response]
...

## Critical technical decisions:
**Decision 1**: [reasoning]
...

Think through system design, then provide the final document.
```

### Template for 05-MVP-Breakdown.md

```markdown
# Generate MVP Breakdown Document

## Input Context
**MVP Features**: {scope.mvp}
**Architecture**: {architecture.summary}
**Tech Stack**: {stack.summary}
**Deadline**: {constraints.deadline}

## Task
Break MVP into 3-5 sprints with:
1. Sprint name and duration
2. Task list per sprint
3. Exit criteria

## Reasoning Guidelines
- Sprint 0: Always setup/scaffolding
- Each sprint: 2-5 days of work
- Tasks should be concrete and testable
- Exit criteria should be verifiable
- Final sprint: Polish and dogfooding

## Output Format
# MVP broken into sprints

## Sprint 0: Setup (day 1)
- [ ] [Setup task 1]
- [ ] [Setup task 2]
**Exit criteria:** [Concrete verification]

## Sprint 1: [Feature Name] (days 2-3)
_Focus: [What this sprint accomplishes]_
- [ ] [Task 1]
- [ ] [Task 2]
**Exit criteria:** [Concrete verification]

...

Think through sprint sequencing and dependencies, then provide the final document.
```

---

## Error Handling & Edge Cases

### Validation Failure
If generated document fails gate validation:
1. Capture validation errors
2. Retry with feedback: "Previous attempt failed: {errors}. Please fix and regenerate."
3. Max 3 retries
4. If still failing: save draft with `.draft` extension and notify user

### LLM Unavailable
- Graceful degradation: prompt user to check API key and network
- Suggest manual completion as fallback

### Partial Generation
- Save progress after each document
- Allow resuming from last successful document
- Track generation state in `.catalyst_state.json`

### User Interruption
- Save partial results
- Allow `catalyst resume` to continue

---

## Success Metrics

### Quality Metrics
- Generated documents pass gate validation (>90% success rate)
- User refinement rate (<30% of docs need manual changes)
- Time savings (5 docs in <5 minutes vs. hours manually)

### UX Metrics
- Command discoverability (clear help text)
- Progress visibility (real-time feedback)
- Error clarity (actionable error messages)

---

## Future Enhancements (Post-MVP)

### Version 2 Features
- **Conversational Mode**: Chat-based refinement instead of single-shot generation
- **Template Customization**: User-defined document templates
- **Multi-Model Support**: A/B test different LLMs (Claude, GPT-4, etc.)
- **Knowledge Base**: Learn from previous projects to improve suggestions
- **Interactive Approval**: Review each document before moving to next

### Never Features
- Fully autonomous planning (user must write vision manually)
- Cloud-based generation (local/self-hosted only)
- Business model generation (focused on technical planning only)

---

## Summary

The Planning Catalyst transforms nexus from a planning *enforcer* into a planning *accelerator*. By combining:
- DeepSeek R1's reasoning capabilities
- Sequential thinking for structured problem-solving  
- Existing template validation infrastructure
- REPL integration for smooth UX

We enable users to go from vision to complete planning in minutes instead of hours, while maintaining the rigor and discipline that makes nexus valuable.

**Key Design Principles**:
1. **User Agency**: AI suggests, user decides
2. **Progressive Refinement**: Each document builds on previous
3. **Validation First**: Never save invalid documents
4. **Transparency**: Show thinking process when requested
5. **Graceful Degradation**: Work without thinking MCP if needed

**Implementation Priority**:
1. Basic scope generation (prove value)
2. All documents (complete workflow)
3. Sequential thinking (improve quality)
4. Refinement (polish UX)
5. Dogfooding (validate in practice)
