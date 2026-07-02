use serde::{Deserialize, Serialize};

// ==================== Session ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub message_count: i32,
    pub summary: Option<String>,
    pub is_archived: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateSession {
    pub title: Option<String>,
}

// ==================== Message ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: i64,
    pub model: Option<String>,
    pub tokens_used: Option<i32>,
    pub metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessage {
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub model: Option<String>,
    pub tokens_used: Option<i32>,
    pub metadata: Option<String>,
}

// ==================== Memory ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub content: String,
    pub category: String,
    pub importance: i32,
    pub source_session: Option<String>,
    pub source_message: Option<String>,
    pub created_at: i64,
    pub last_recalled_at: Option<i64>,
    pub recall_count: i32,
    pub is_active: bool,
    pub user_edited: bool,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMemory {
    pub content: String,
    pub category: String,
    pub importance: Option<i32>,
    pub source_session: Option<String>,
    pub source_message: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemory {
    pub content: Option<String>,
    pub category: Option<String>,
    pub importance: Option<i32>,
    pub is_active: Option<bool>,
    pub tags: Option<String>,
}

// ==================== Config ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub updated_at: i64,
}

// ==================== Activity Log ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: i64,
    pub activity: String,
    pub window_title: Option<String>,
    pub process_name: Option<String>,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub duration_seconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateActivityLog {
    pub activity: String,
    pub window_title: Option<String>,
    pub process_name: Option<String>,
    pub started_at: i64,
}

// ==================== Task Log ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLog {
    pub id: String,
    pub session_id: Option<String>,
    pub message_id: Option<String>,
    pub task_type: String,
    pub permission_level: i32,
    pub user_approved: Option<bool>,
    pub input: String,
    pub output: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

// ==================== Reminder ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub content: String,
    pub remind_at: i64,
    pub is_recurring: bool,
    pub recurring_rule: Option<String>,
    pub is_done: bool,
    pub source_session: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateReminder {
    pub content: String,
    pub remind_at: i64,
    pub is_recurring: Option<bool>,
    pub recurring_rule: Option<String>,
    pub source_session: Option<String>,
}
