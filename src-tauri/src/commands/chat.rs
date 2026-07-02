use tauri::{AppHandle, Emitter, State};

use crate::db::Database;
use crate::db::models::{Session, Message, CreateSession, CreateMessage};
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

/// 发送消息（先保存到数据库，后续集成 AI 回复）
#[tauri::command]
pub async fn chat_send(
    app: AppHandle,
    db: State<'_, Database>,
    session_id: String,
    content: String,
) -> Result<Message, AppError> {
    // 保存用户消息
    let user_msg = db.create_message(&CreateMessage {
        session_id: session_id.clone(),
        role: "user".to_string(),
        content,
        model: None,
        tokens_used: None,
        metadata: None,
    })?;

    // TODO: 调用 AI API 获取回复并流式推送
    // 这里先返回一个占位回复
    let placeholder = format!("[AI 回复功能待集成] 你说的是: {}", user_msg.content);

    let _ = app.emit("chat:stream_chunk", serde_json::json!({
        "session_id": session_id,
        "chunk": placeholder,
    }));

    let ai_msg = db.create_message(&CreateMessage {
        session_id,
        role: "assistant".to_string(),
        content: placeholder,
        model: Some("placeholder".to_string()),
        tokens_used: None,
        metadata: None,
    })?;

    let _ = app.emit("chat:stream_done", serde_json::json!({
        "session_id": ai_msg.session_id,
        "message_id": ai_msg.id,
    }));

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
