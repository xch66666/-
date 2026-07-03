use std::sync::Arc;
use tauri::State;

use crate::db::Database;
use crate::db::models::{Memory, UpdateMemory};
use crate::memory::MemoryManager;
use crate::ai::LlmConfig;
use super::AppError;

/// 获取记忆列表
#[tauri::command]
pub fn memory_list(
    db: State<'_, Arc<Database>>,
    category: Option<String>,
) -> Result<Vec<Memory>, AppError> {
    let memories = db.list_memories(category.as_deref())?;
    Ok(memories)
}

/// 搜索记忆
#[tauri::command]
pub fn memory_search(
    db: State<'_, Arc<Database>>,
    query: String,
    limit: Option<i32>,
) -> Result<Vec<Memory>, AppError> {
    let memories = db.search_memories(&query, limit.unwrap_or(20))?;
    Ok(memories)
}

/// 更新记忆
#[tauri::command]
pub fn memory_update(
    db: State<'_, Arc<Database>>,
    id: String,
    content: Option<String>,
    category: Option<String>,
    importance: Option<i32>,
    is_active: Option<bool>,
    tags: Option<String>,
) -> Result<(), AppError> {
    db.update_memory(&id, &UpdateMemory {
        content,
        category,
        importance,
        is_active,
        tags,
    })?;
    Ok(())
}

/// 删除记忆
#[tauri::command]
pub fn memory_delete(
    db: State<'_, Arc<Database>>,
    id: String,
) -> Result<(), AppError> {
    db.delete_memory(&id)?;
    Ok(())
}

/// 记忆统计
#[tauri::command]
pub fn memory_stats(
    db: State<'_, Arc<Database>>,
) -> Result<serde_json::Value, AppError> {
    let (total, categories) = db.memory_stats()?;
    Ok(serde_json::json!({
        "total": total,
        "category_count": categories,
    }))
}

/// 从会话中提取记忆
#[tauri::command]
pub async fn memory_extract(
    db: State<'_, Arc<Database>>,
    session_id: String,
) -> Result<serde_json::Value, AppError> {
    // 加载 LLM 配置
    let llm_config = {
        let config_entry = db.get_config("llm").map_err(AppError::from)?;
        match config_entry {
            Some(entry) => serde_json::from_str::<LlmConfig>(&entry.value).unwrap_or_default(),
            None => LlmConfig::default(),
        }
    };

    let manager = MemoryManager::new(&db);
    let count = manager
        .extract_from_session(&session_id, &llm_config)
        .await
        .map_err(|e| AppError {
            code: "MEMORY_EXTRACT_ERROR".to_string(),
            message: e,
        })?;

    Ok(serde_json::json!({
        "extracted": count,
    }))
}
