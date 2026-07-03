use std::sync::Arc;
use tauri::State;

use crate::db::Database;
use super::AppError;

/// 获取当前活动信息
#[tauri::command]
pub fn activity_get_current(
    db: State<'_, Arc<Database>>,
) -> Result<serde_json::Value, AppError> {
    let logs = db.get_recent_activities(1)?;
    match logs.into_iter().next() {
        Some(log) => Ok(serde_json::json!({
            "activity": log.activity,
            "window_title": log.window_title,
            "process_name": log.process_name,
            "started_at": log.started_at,
        })),
        None => Ok(serde_json::Value::Null),
    }
}

/// 获取活动统计（各活动类型总时长）
#[tauri::command]
pub fn activity_get_stats(
    db: State<'_, Arc<Database>>,
    since: Option<i64>,
) -> Result<Vec<serde_json::Value>, AppError> {
    // 默认统计最近 7 天
    let since_ms = since.unwrap_or_else(|| {
        chrono::Utc::now().timestamp_millis() - 7 * 24 * 60 * 60 * 1000
    });

    let stats = db.get_activity_stats(since_ms)?;
    let result: Vec<serde_json::Value> = stats
        .into_iter()
        .map(|(activity, total_seconds)| {
            serde_json::json!({
                "activity": activity,
                "total_seconds": total_seconds,
            })
        })
        .collect();

    Ok(result)
}
