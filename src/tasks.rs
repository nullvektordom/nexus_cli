use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Status of an adhoc task throughout its lifecycle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Planning phase - filling out planning documents
    Planning,
    /// Gate passed - planning validated, ready to start implementation
    GatePassed,
    /// In progress - actively working on implementation
    InProgress,
    /// Completed - task finished and validated
    Done,
}

/// Represents an adhoc task with metadata and status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdhocTask {
    /// Unique identifier for the task (typically project name)
    pub id: String,
    /// Task title/description
    pub title: String,
    /// Current status of the task
    pub status: TaskStatus,
    /// Timestamp when task was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when implementation started (gate passed)
    pub started_at: Option<DateTime<Utc>>,
    /// Timestamp when task was completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl AdhocTask {
    /// Create a new task in the Planning state
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            status: TaskStatus::Planning,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }

    /// Mark the task as started (gate passed)
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
        self.started_at = Some(Utc::now());
    }

    /// Mark the task as completed
    pub fn complete(&mut self) {
        self.status = TaskStatus::Done;
        self.completed_at = Some(Utc::now());
    }

    /// Check if the task has been started
    pub fn is_started(&self) -> bool {
        self.started_at.is_some()
    }

    /// Check if the task is completed
    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Done
    }
}

/// Context for rendering adhoc task templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdhocTaskContext {
    /// The task metadata
    pub task: AdhocTask,
    /// Content from Task-Capture.md (if available)
    pub capture_content: Option<String>,
    /// Content from Task-Approach.md (if available)
    pub approach_content: Option<String>,
}

impl AdhocTaskContext {
    /// Create a new context from a task
    pub fn new(task: AdhocTask) -> Self {
        Self {
            task,
            capture_content: None,
            approach_content: None,
        }
    }

    /// Add capture document content
    pub fn with_capture(mut self, content: String) -> Self {
        self.capture_content = Some(content);
        self
    }

    /// Add approach document content
    pub fn with_approach(mut self, content: String) -> Self {
        self.approach_content = Some(content);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task_defaults() {
        let task = AdhocTask::new("test-task".to_string(), "Test Task".to_string());

        assert_eq!(task.id, "test-task");
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, TaskStatus::Planning);
        assert!(!task.is_started());
        assert!(!task.is_completed());
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = AdhocTask::new("test".to_string(), "Test".to_string());

        // Start task
        task.start();
        assert_eq!(task.status, TaskStatus::InProgress);
        assert!(task.is_started());
        assert!(!task.is_completed());

        // Complete task
        task.complete();
        assert_eq!(task.status, TaskStatus::Done);
        assert!(task.is_completed());
    }

    #[test]
    fn test_task_context() {
        let task = AdhocTask::new("test".to_string(), "Test".to_string());
        let context = AdhocTaskContext::new(task)
            .with_capture("Capture content".to_string())
            .with_approach("Approach content".to_string());

        assert!(context.capture_content.is_some());
        assert!(context.approach_content.is_some());
    }
}
