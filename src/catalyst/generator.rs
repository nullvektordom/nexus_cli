/// Document generation types and context management

use serde::{Deserialize, Serialize};

/// Types of planning documents that can be generated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentType {
    /// 02-Scope-and-Boundaries.md
    Scope,
    /// 03-Tech-Stack.md
    TechStack,
    /// 04-Architecture.md
    Architecture,
    /// 05-MVP-Breakdown.md
    MvpBreakdown,
}

impl DocumentType {
    /// Get the filename for this document type
    pub fn filename(&self) -> &'static str {
        match self {
            DocumentType::Scope => "02-Scope-and-Boundaries.md",
            DocumentType::TechStack => "03-Tech-Stack.md",
            DocumentType::Architecture => "04-Architecture.md",
            DocumentType::MvpBreakdown => "05-MVP-Breakdown.md",
        }
    }

    /// Get a human-readable name for this document type
    pub fn display_name(&self) -> &'static str {
        match self {
            DocumentType::Scope => "Scope and Boundaries",
            DocumentType::TechStack => "Tech Stack",
            DocumentType::Architecture => "Architecture",
            DocumentType::MvpBreakdown => "MVP Breakdown",
        }
    }
}

/// Structured data extracted from the vision document (01-Problem-and-Vision.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionData {
    /// The personal problem being solved
    pub problem: String,
    /// One-sentence solution description
    pub solution: String,
    /// Success criteria (3 months)
    pub success_criteria: String,
    /// What this project is NOT (anti-vision)
    pub anti_vision: String,
}

impl VisionData {
    /// Create an empty VisionData
    pub fn empty() -> Self {
        Self {
            problem: String::new(),
            solution: String::new(),
            success_criteria: String::new(),
            anti_vision: String::new(),
        }
    }

    /// Check if the vision data is complete
    pub fn is_complete(&self) -> bool {
        !self.problem.is_empty()
            && !self.solution.is_empty()
            && !self.success_criteria.is_empty()
            && !self.anti_vision.is_empty()
    }
}

/// Structured data extracted from the scope document (02-Scope-and-Boundaries.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeData {
    /// Features included in MVP
    pub mvp_features: Vec<String>,
    /// Features for version 2 (not now)
    pub version2_features: Vec<String>,
    /// Features that will never be built
    pub never_features: Vec<String>,
    /// Technical constraints
    pub constraints: String,
}

/// Structured data extracted from the tech stack document (03-Tech-Stack.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStackData {
    /// Primary programming language
    pub language: String,
    /// Framework or library
    pub framework: String,
    /// Database (if applicable)
    pub database: Option<String>,
    /// Justification for choices
    pub justification: String,
}

/// Structured data extracted from the architecture document (04-Architecture.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureData {
    /// Folder structure
    pub folder_structure: String,
    /// Data model
    pub data_model: String,
    /// User flow
    pub user_flow: String,
}

/// Context for document generation - accumulates data from previous documents
#[derive(Debug, Clone)]
pub struct GenerationContext {
    /// Vision data (always present)
    pub vision: VisionData,
    /// Scope data (present when generating tech stack or later)
    pub scope: Option<ScopeData>,
    /// Tech stack data (present when generating architecture or later)
    pub tech_stack: Option<TechStackData>,
    /// Architecture data (present when generating MVP breakdown)
    pub architecture: Option<ArchitectureData>,
}

impl GenerationContext {
    /// Create a new context with just vision data
    pub fn new(vision: VisionData) -> Self {
        Self {
            vision,
            scope: None,
            tech_stack: None,
            architecture: None,
        }
    }

    /// Add scope data to the context
    pub fn with_scope(mut self, scope: ScopeData) -> Self {
        self.scope = Some(scope);
        self
    }

    /// Add tech stack data to the context
    pub fn with_tech_stack(mut self, tech_stack: TechStackData) -> Self {
        self.tech_stack = Some(tech_stack);
        self
    }

    /// Add architecture data to the context
    pub fn with_architecture(mut self, architecture: ArchitectureData) -> Self {
        self.architecture = Some(architecture);
        self
    }
}
