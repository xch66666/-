use crate::db::Database;

/// 构建 AI 上下文（系统提示 + 历史消息）
pub fn build_context(
    db: &Database,
    session_id: &str,
    system_prompt: &str,
    max_messages: i32,
) -> Result<Vec<serde_json::Value>, rusqlite::Error> {
    let mut messages = Vec::new();

    // 1. 添加系统提示
    messages.push(serde_json::json!({
        "role": "system",
        "content": system_prompt,
    }));

    // 2. 获取最近的历史消息
    let history = db.get_recent_messages(session_id, max_messages)?;

    for msg in history {
        messages.push(serde_json::json!({
            "role": msg.role,
            "content": msg.content,
        }));
    }

    Ok(messages)
}

/// 构建人设系统提示
pub fn build_persona_prompt(
    persona_json: &str,
    memories_json: &str,
) -> String {
    let persona: serde_json::Value = serde_json::from_str(persona_json)
        .unwrap_or(serde_json::json!({"name": "小伴", "personality": "温柔", "style": "口语化"}));

    let name = persona["name"].as_str().unwrap_or("小伴");
    let personality = persona["personality"].as_str().unwrap_or("温柔体贴");
    let style = persona["style"].as_str().unwrap_or("口语化，像朋友聊天");
    let nickname = persona["nickname"].as_str().unwrap_or("你");

    let mut prompt = format!(
        "你是{}，一个桌面 AI 陪伴体。\n\n\
        ## 你的人设\n\
        - 名字：{}\n\
        - 性格：{}\n\
        - 说话风格：{}\n\
        - 称呼用户为：{}\n\n\
        ## 行为准则\n\
        1. 像朋友一样自然对话，不要像客服\n\
        2. 简短回复为主，不要写长篇大论\n\
        3. 适当使用 emoji 增加亲和力\n\
        4. 如果用户在忙，简短关心就好\n\
        5. 记住用户告诉过你的事情\n\n",
        name, name, personality, style, nickname
    );

    // 添加记忆上下文
    if !memories_json.is_empty() && memories_json != "[]" {
        if let Ok(memories) = serde_json::from_str::<Vec<serde_json::Value>>(memories_json) {
            if !memories.is_empty() {
                prompt.push_str("## 你对用户的了解\n");
                for mem in &memories {
                    if let Some(content) = mem["content"].as_str() {
                        prompt.push_str(&format!("- {}\n", content));
                    }
                }
                prompt.push('\n');
            }
        }
    }

    prompt.push_str("请根据以上设定和用户对话。回复要简短自然。\n");

    prompt
}
