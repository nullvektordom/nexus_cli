pub struct DocumentSchema {
    pub filename: &'static str,
    pub display_name: &'static str,
    pub required_headers: &'static [&'static str],
    #[allow(dead_code)]
    pub optional_headers: &'static [&'static str],
    pub min_word_count: usize,
    pub template_guidance: &'static str,
}

pub const PLANNING_DOCUMENTS: &[DocumentSchema] = &[
    DocumentSchema {
        filename: "01-Problem-and-Vision.md",
        display_name: "Problem and Vision",
        required_headers: &["Problem", "Vision"],
        optional_headers: &[],
        min_word_count: 100,
        template_guidance: "Define the problem and your vision for solving it",
    },
    DocumentSchema {
        filename: "02-Scope-and-Boundaries.md",
        display_name: "Scope and Boundaries",
        required_headers: &["Scope", "Boundaries"],
        optional_headers: &[],
        min_word_count: 100,
        template_guidance: "Define what is in scope and what is out of scope (boundaries)",
    },
    DocumentSchema {
        filename: "03-Tech-Stack.md",
        display_name: "Tech Stack",
        required_headers: &["Tech Stack"],
        optional_headers: &[],
        min_word_count: 100,
        template_guidance: "Define the technology stack and justify your choices",
    },
    DocumentSchema {
        filename: "04-Architecture.md",
        display_name: "Architecture",
        required_headers: &["Architecture"],
        optional_headers: &[],
        min_word_count: 100,
        template_guidance: "Describe the system architecture, components, and data flow",
    },
    DocumentSchema {
        filename: "05-MVP-Roadmap.md",
        display_name: "MVP Roadmap",
        required_headers: &["Phase 1", "Phase 2", "Phase 3"],
        optional_headers: &[],
        min_word_count: 100,
        template_guidance: "Define the 3-phase strategic path with Definitions of Done",
    },
];

#[allow(dead_code)]
pub fn get_document_schema(filename: &str) -> Option<&'static DocumentSchema> {
    PLANNING_DOCUMENTS.iter().find(|doc| doc.filename == filename)
}
