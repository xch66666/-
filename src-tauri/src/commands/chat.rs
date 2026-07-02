use tauri::{AppHandle, State};

use crate::db::Database;
use crate::db::models::{Session, Message, CreateSession};
use crate::ai;
use super::AppError;

/// 创建新对话
#[tauri::command]
pub fn chat_create_session(
    db: State<'_, Database>,
    title: Option<String>,
) -> Result<Session, AppError> {
    let data = CreateSession { title };
    let session = db.create_session(&data)?;
    Ok(session)
}

/// 获取会话列表
#[tauri::command]
pub fn chat_list_sessions(
    db: State<'_, Database>,
    include_archived: Option<bool>,
) -> Result<Vec<Session>, AppError> {
    let sessions = db.list_sessions(include_archived.unwrap_or(false))?;
    Ok(sessions)
}

/// 获取会话消息
#[tauri::command]
pub fn chat_get_messages(
    db: State<'_, Database>,
    session_id: String,
    limit: Option<i32>,
) -> Result<Vec<Message>, AppError> {
    let messages = db.get_messages(&session_id, limit)?;
    Ok(messages)
}

/// 发送消息（调用 AI 流式回复）
#[tauri::command]
pub async fn chat_send(
    app: AppHandle,
    db: State<'_, Database>,
    session_id: String,
    content: String,
) -> Result<Message, AppError> {
    // 1. 加载 LLM 配置
    let llm_config = {
        let config_entry = db.get_config("llm").map_err(AppError::from)?;
        match config_entry {
            Some(entry) => {
                let parsed: ai::LlmConfig = serde_json::from_str(&entry.value)
                    .unwrap_or_default();
                parsed
            }
            None => ai::LlmConfig::default(),
        }
    };

    // 2. 加载人设配置
    let persona_json = {
        let config_entry = db.get_config("persona").map_err(AppError::from)?;
        config_entry.map(|e| e.value).unwrap_or_else(|| {
            r#"{"name":"小伴","personality":"温柔体贴","style":"口语化","nickname":"你"}"#.to_string()
        })
    };

    // 3. 加载相关记忆
    let memories = db.search_memories(&content, 10).unwrap_or_default();
    let memories_json = serde_json::to_string(&memories).unwrap_or_else(|_| "[]".to_string());

    // 4. 构建系统提示
    let system_prompt = ai::context::build_persona_prompt(&persona_json, &memories_json);

    // 5. 调用 AI 流式回复
    let ai_msg = ai::send_and_stream(
        &app,
        &db,
        &session_id,
        &content,
        &system_prompt,
        &llm_config,
    )
    .await
    .map_err(|e| AppError {
        code: "LLM_API_ERROR".to_string(),
        message: e,
    })?;

    Ok(ai_msg)
}

/// 删除会话
#[tauri::command]
pub fn chat_delete_session(
    db: State<'_, Database>,
    session_id: String,
) -> Result<(), AppError> {
    db.delete_session(&session_id)?;
    Ok(())
}

/// 归档会话
#[tauri::command]
pub fn chat_archive_session(
    db: State<'_, Database>,
    session_id: String,
) -> Result<(), AppError> {
    db.archive_session(&session_id)?;
    Ok(())
}
