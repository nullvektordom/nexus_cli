/// Prompt templates for document generation

use crate::catalyst::generator::GenerationContext;

/// Template for generating planning documents
pub struct PromptTemplate {
    system_prompt: String,
    user_prompt: String,
}

impl PromptTemplate {
    /// Create a prompt template for generating the scope document
    pub fn for_scope(context: &GenerationContext) -> Self {
        let system_prompt = r#"You are an expert product strategist helping to define project scope and boundaries.

Your task is to generate a "Scope and Boundaries" document based on the user's vision.

CRITICAL REQUIREMENTS:
1. Output ONLY valid markdown - no explanations, no meta-commentary
2. Use EXACTLY these section headers (with ## prefix):
   - ## MVP (Minimum Viable Product):
   - ## Version 2 (NOT NOW - just document):
   - ## Never (things I will NOT build):
   - ## Tech constraints:
3. Each section must have substantial content (minimum 50 words per section)
4. Be specific and concrete - NO placeholders like TODO, TBD, [fill], [describe]
5. Focus on what makes the MVP truly minimal while still being viable
6. Be ruthless about what goes in "Never" - this prevents scope creep

Think step-by-step about:
- What is the absolute minimum needed to validate the core value proposition?
- What features would be nice but aren't essential for the first version?
- What features might seem related but would dilute focus?
- What technical constraints exist (time, budget, skills, platform)?

Output the complete markdown document with all sections filled out."#.to_string();

        let user_prompt = format!(
            r#"Based on this vision, generate a comprehensive Scope and Boundaries document:

# Vision

**Problem**: {}

**Solution**: {}

**Success Criteria (3 months)**: {}

**Anti-Vision (what this is NOT)**: {}

---

Generate the Scope and Boundaries document now. Remember:
- Use the exact section headers specified
- Be specific and concrete
- No placeholders or TODOs
- Minimum 50 words per section
- Focus on making the MVP truly minimal"#,
            context.vision.problem,
            context.vision.solution,
            context.vision.success_criteria,
            context.vision.anti_vision
        );

        Self {
            system_prompt,
            user_prompt,
        }
    }

    /// Create a prompt template for generating the tech stack document
    pub fn for_tech_stack(context: &GenerationContext) -> Self {
        let scope_context = if let Some(scope) = &context.scope {
            format!(
                r#"
# Scope Context

**MVP Features**:
{}

**Version 2 Features**:
{}

**Never Features**:
{}

**Constraints**:
{}"#,
                scope
                    .mvp_features
                    .iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n"),
                scope
                    .version2_features
                    .iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n"),
                scope
                    .never_features
                    .iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n"),
                scope.constraints
            )
        } else {
            String::new()
        };

        let system_prompt = r#"You are an expert software architect helping to select the optimal tech stack.

Your task is to generate a "Tech Stack" document based on the vision and scope.

CRITICAL REQUIREMENTS:
1. Output ONLY valid markdown - no explanations, no meta-commentary
2. Use EXACTLY these section headers (with ## prefix):
   - ## Language:
   - ## Framework/Library:
   - ## Database (if needed):
   - ## Justification:
3. Each section must have substantial content
4. Be specific - name actual technologies, not categories
5. NO placeholders like TODO, TBD, [fill], [describe]
6. Justify choices based on MVP requirements and constraints

Think step-by-step about:
- What language best fits the developer's skills and project needs?
- What framework minimizes boilerplate while staying flexible?
- Does the MVP actually need a database, or can it start simpler?
- How do these choices support rapid iteration?

Output the complete markdown document with all sections filled out."#.to_string();

        let user_prompt = format!(
            r#"Based on this vision and scope, generate a comprehensive Tech Stack document:

# Vision

**Problem**: {}

**Solution**: {}

**Success Criteria**: {}
{}

---

Generate the Tech Stack document now. Remember:
- Use the exact section headers specified
- Name specific technologies
- No placeholders or TODOs
- Justify based on MVP needs and constraints"#,
            context.vision.problem,
            context.vision.solution,
            context.vision.success_criteria,
            scope_context
        );

        Self {
            system_prompt,
            user_prompt,
        }
    }

    /// Get the system prompt
    pub fn system_prompt(&self) -> &str {
        &self.system_prompt
    }

    /// Create a prompt template for generating the architecture document
    pub fn for_architecture(context: &GenerationContext) -> Self {
        let scope_context = if let Some(scope) = &context.scope {
            format!(
                r#"
# Scope Context

**MVP Features**:
{}

**Constraints**:
{}"#,
                scope
                    .mvp_features
                    .iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n"),
                scope.constraints
            )
        } else {
            String::new()
        };

        let tech_context = if let Some(tech) = &context.tech_stack {
            format!(
                r#"
# Tech Stack Context

**Language**: {}
**Framework**: {}
**Database**: {}
**Justification**: {}"#,
                tech.language,
                tech.framework,
                tech.database.as_deref().unwrap_or("None"),
                tech.justification
            )
        } else {
            String::new()
        };

        let system_prompt = r#"You are an expert software architect helping to design system architecture.

Your task is to generate an "Architecture" document based on the vision, scope, and tech stack.

CRITICAL REQUIREMENTS:
1. Output ONLY valid markdown - no explanations, no meta-commentary
2. Use EXACTLY these section headers (with ## prefix):
   - ## Folder structure:
   - ## Data model (main entities):
   - ## Flow (user journey):
   - ## Critical technical decisions:
3. Each section must have substantial content
4. Be specific and concrete - NO placeholders like TODO, TBD, [fill], [describe]
5. Folder structure should be in a code block with actual directories
6. Data model should list actual entities with fields
7. User flow should be step-by-step

Think step-by-step about:
- What folder structure best fits the chosen tech stack?
- What are the core data entities needed for the MVP?
- What is the primary user journey through the system?
- What critical technical decisions need to be made upfront?

Output the complete markdown document with all sections filled out."#.to_string();

        let user_prompt = format!(
            r#"Based on this vision, scope, and tech stack, generate a comprehensive Architecture document:

# Vision

**Problem**: {}

**Solution**: {}

**Success Criteria**: {}
{}
{}

---

Generate the Architecture document now. Remember:
- Use the exact section headers specified
- Be specific and concrete
- No placeholders or TODOs
- Folder structure in code block
- List actual entities and fields
- Step-by-step user flow"#,
            context.vision.problem,
            context.vision.solution,
            context.vision.success_criteria,
            scope_context,
            tech_context
        );

        Self {
            system_prompt,
            user_prompt,
        }
    }

    /// Create a prompt template for generating the MVP breakdown document
    pub fn for_mvp_breakdown(context: &GenerationContext) -> Self {
        let scope_context = if let Some(scope) = &context.scope {
            format!(
                r#"
# Scope Context

**MVP Features**:
{}"#,
                scope
                    .mvp_features
                    .iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            String::new()
        };

        let tech_context = if let Some(tech) = &context.tech_stack {
            format!(
                r#"
# Tech Stack

**Language**: {}
**Framework**: {}"#,
                tech.language, tech.framework
            )
        } else {
            String::new()
        };

        let arch_context = if let Some(arch) = &context.architecture {
            format!(
                r#"
# Architecture

**Folder Structure**:
{}

**Data Model**:
{}"#,
                arch.folder_structure, arch.data_model
            )
        } else {
            String::new()
        };

        let system_prompt = r#"You are an expert project manager helping to break down an MVP into sprints.

Your task is to generate an "MVP Breakdown" document that divides the project into 3-5 sprints.

CRITICAL REQUIREMENTS:
1. Output ONLY valid markdown - no explanations, no meta-commentary
2. Create 3-5 sprint sections with headers like: ## Sprint 0: Setup (day 1)
3. Each sprint must have:
   - A descriptive name
   - 3-7 concrete tasks as checkboxes (- [ ] Task)
   - **Exit criteria:** with a measurable goal
4. Be specific and concrete - NO placeholders like TODO, TBD, [fill], [describe]
5. Sprint 0 should always be "Setup" (repo, dev environment, hello world)
6. Order sprints logically (foundation → features → polish)
7. Keep MVP scope tight - max 5 sprints

Think step-by-step about:
- What needs to be set up first? (Sprint 0)
- What is the core feature that proves the concept?
- What features build on each other?
- What can be deferred to Version 2?

Output the complete markdown document with all sprint sections."#.to_string();

        let user_prompt = format!(
            r#"Based on this vision, scope, tech stack, and architecture, generate a comprehensive MVP Breakdown:

# Vision

**Problem**: {}

**Solution**: {}
{}
{}
{}

---

Generate the MVP Breakdown document now. Remember:
- 3-5 sprints total
- Sprint 0 is always "Setup"
- Each sprint has 3-7 concrete tasks
- Each sprint has measurable exit criteria
- No placeholders or TODOs
- Order sprints logically"#,
            context.vision.problem,
            context.vision.solution,
            scope_context,
            tech_context,
            arch_context
        );

        Self {
            system_prompt,
            user_prompt,
        }
    }

    /// Get the user prompt
    pub fn user_prompt(&self) -> &str {
        &self.user_prompt
    }

    /// Render the complete prompt as a single string (for simple LLM clients)
    pub fn render(&self) -> String {
        format!("{}\n\n{}", self.system_prompt, self.user_prompt)
    }
}
