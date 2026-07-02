use tauri::{AppHandle, Emitter, State};

use crate::db::Database;
use crate::db::models::ConfigEntry;
use super::AppError;

/// 获取配置项
#[tauri::command]
pub fn config_get(
    db: State<'_, Database>,
    key: String,
) -> Result<Option<ConfigEntry>, AppError> {
    let entry = db.get_config(&key)?;
    Ok(entry)
}

/// 获取所有配置
#[tauri::command]
pub fn config_get_all(
    db: State<'_, Database>,
) -> Result<Vec<ConfigEntry>, AppError> {
    let configs = db.get_all_configs()?;
    Ok(configs)
}

/// 设置配置项
#[tauri::command]
pub async fn config_set(
    app: AppHandle,
    db: State<'_, Database>,
    key: String,
    value: String,
) -> Result<(), AppError> {
    // 验证 JSON 格式
    let _: serde_json::Value = serde_json::from_str(&value)?;

    db.set_config(&key, &value)?;

    // 推送配置变更事件
    let _ = app.emit("config:changed", serde_json::json!({
        "key": key,
        "value": value,
    }));

    Ok(())
}
