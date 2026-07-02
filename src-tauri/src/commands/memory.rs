use tauri::State;

use crate::db::Database;
use crate::db::models::{Memory, UpdateMemory};
use super::AppError;

/// 获取记忆列表
#[tauri::command]
pub fn memory_list(
    db: State<'_, Database>,
    category: Option<String>,
) -> Result<Vec<Memory>, AppError> {
    let memories = db.list_memories(category.as_deref())?;
    Ok(memories)
}

/// 搜索记忆
#[tauri::command]
pub fn memory_search(
    db: State<'_, Database>,
    query: String,
    limit: Option<i32>,
) -> Result<Vec<Memory>, AppError> {
    let memories = db.search_memories(&query, limit.unwrap_or(20))?;
    Ok(memories)
}

/// 更新记忆
#[tauri::command]
pub fn memory_update(
    db: State<'_, Database>,
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
    db: State<'_, Database>,
    id: String,
) -> Result<(), AppError> {
    db.delete_memory(&id)?;
    Ok(())
}

/// 记忆统计
#[tauri::command]
pub fn memory_stats(
    db: State<'_, Database>,
) -> Result<serde_json::Value, AppError> {
    let (total, categories) = db.memory_stats()?;
    Ok(serde_json::json!({
        "total": total,
        "category_count": categories,
    }))
}
