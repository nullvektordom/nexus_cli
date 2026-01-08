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
        required_headers: &[
            "My problem (personal):",
            "Who else has this problem?",
            "Solution in ONE SENTENCE:",
            "Success criteria (3 months):",
            "Anti-vision (what this project is NOT):",
        ],
        optional_headers: &[],
        min_word_count: 50,
        template_guidance: "Define the personal problem, identify who shares it, provide a one-sentence solution, set 3-month success criteria, and clarify what this project is NOT",
    },
    DocumentSchema {
        filename: "02-Scope-and-Boundaries.md",
        display_name: "Scope and Boundaries",
        required_headers: &[
            "MVP (Minimum Viable Product):",
            "Version 2 (NOT NOW - just document):",
            "Never (things I will NOT build):",
            "Tech constraints:",
        ],
        optional_headers: &[],
        min_word_count: 50,
        template_guidance: "Define MVP features (3-5 max), document future Version 2 features, list what will NEVER be built to prevent scope creep, and specify technical constraints (budget, timeline, platform)",
    },
    DocumentSchema {
        filename: "03-Tech-Stack.md",
        display_name: "Tech Stack",
        required_headers: &[
            "Stack (force yourself to choose NOW):",
            "Why these choices?",
            "What I will NOT use:",
            "Dependencies (important ones):",
            "Development environment:",
        ],
        optional_headers: &[],
        min_word_count: 50,
        template_guidance: "Choose specific technologies NOW (frontend, backend, database, hosting), justify each choice in 2 sentences, list technologies to avoid, specify key dependencies (max 10), and document dev environment (IDE, OS, device)",
    },
    DocumentSchema {
        filename: "04-Architecture.md",
        display_name: "Architecture",
        required_headers: &[
            "Folder structure:",
            "Data model (main entities):",
            "Flow (user journey):",
            "Critical technical decisions:",
        ],
        optional_headers: &[],
        min_word_count: 50,
        template_guidance: "Define project folder structure with main directories, describe data entities (2 sentences each) with their fields, map out user journey step-by-step, and document critical decisions (state management, navigation, data persistence)",
    },
    DocumentSchema {
        filename: "05-MVP-Breakdown.md",
        display_name: "MVP Breakdown",
        required_headers: &[
            "Sprint 1",
            "Sprint 2",
            "Sprint 3",
            "Definition of Done (each sprint):",
        ],
        optional_headers: &[],
        min_word_count: 50,
        template_guidance: "Break MVP into 3-5 sprints (can include Sprint 0 for setup), each with specific tasks and concrete exit criteria. Include a universal Definition of Done checklist (builds without errors, tested, committed to git, session log updated)",
    },
];

#[allow(dead_code)]
pub fn get_document_schema(filename: &str) -> Option<&'static DocumentSchema> {
    PLANNING_DOCUMENTS.iter().find(|doc| doc.filename == filename)
}
