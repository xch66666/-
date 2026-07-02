pub mod provider;
pub mod context;

use serde::{Deserialize, Serialize};
use futures::StreamExt;
use tauri::{AppHandle, Emitter};

use crate::db::Database;
use crate::db::models::{CreateMessage, Message};

/// LLM 提供商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub api_base: Option<String>,
    pub max_context_messages: i32,
    pub temperature: f32,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4o-mini".to_string(),
            api_key: None,
            api_base: None,
            max_context_messages: 20,
            temperature: 0.8,
        }
    }
}

/// API 请求消息
#[derive(Debug, Clone, Serialize)]
pub struct ApiMessage {
    pub role: String,
    pub content: String,
}

/// 流式响应块
#[derive(Debug, Deserialize)]
pub struct StreamChunk {
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    pub delta: StreamDelta,
}

#[derive(Debug, Deserialize)]
pub struct StreamDelta {
    pub content: Option<String>,
}

/// 发送消息并获取 AI 流式回复
pub async fn send_and_stream(
    app: &AppHandle,
    db: &Database,
    session_id: &str,
    user_content: &str,
    system_prompt: &str,
    config: &LlmConfig,
) -> Result<Message, String> {
    // 保存用户消息
    let _user_msg = db.create_message(&CreateMessage {
        session_id: session_id.to_string(),
        role: "user".to_string(),
        content: user_content.to_string(),
        model: None,
        tokens_used: None,
        metadata: None,
    }).map_err(|e| format!("保存用户消息失败: {}", e))?;

    // 构建上下文消息
    let context_messages = context::build_context(db, session_id, system_prompt, config.max_context_messages)
        .map_err(|e| format!("构建上下文失败: {}", e))?;

    // 获取 API endpoint
    let api_base = config.api_base.as_deref().unwrap_or("https://api.openai.com/v1");
    let url = format!("{}/chat/completions", api_base);

    let api_key = config.api_key.as_deref().unwrap_or("");

    // 构建请求
    let client = reqwest::Client::new();
    let request_body = serde_json::json!({
        "model": config.model,
        "messages": context_messages,
        "stream": true,
        "temperature": config.temperature,
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("API 请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API 返回错误 {}: {}", status, body));
    }

    // 流式读取
    let mut full_content = String::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("流读取失败: {}", e))?;
        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            let line = line.trim();
            if !line.starts_with("data: ") {
                continue;
            }

            let data = &line[6..];
            if data == "[DONE]" {
                break;
            }

            if let Ok(parsed) = serde_json::from_str::<StreamChunk>(data) {
                if let Some(choice) = parsed.choices.first() {
                    if let Some(ref content) = choice.delta.content {
                        full_content.push_str(content);

                        // 推送流式块到前端
                        let _ = app.emit("chat:stream_chunk", serde_json::json!({
                            "session_id": session_id,
                            "chunk": content,
                        }));
                    }
                }
            }
        }
    }

    // 保存 AI 回复
    let ai_msg = db.create_message(&CreateMessage {
        session_id: session_id.to_string(),
        role: "assistant".to_string(),
        content: full_content.clone(),
        model: Some(config.model.clone()),
        tokens_used: None,
        metadata: None,
    }).map_err(|e| format!("保存 AI 回复失败: {}", e))?;

    // 推送完成事件
    let _ = app.emit("chat:stream_done", serde_json::json!({
        "session_id": session_id,
        "message_id": ai_msg.id,
    }));

    Ok(ai_msg)
}
