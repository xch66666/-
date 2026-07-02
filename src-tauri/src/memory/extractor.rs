use serde::{Deserialize, Serialize};

use crate::ai::LlmConfig;

/// 提取出的记忆
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedMemory {
    pub content: String,
    pub category: String,
    pub importance: i32,
    pub tags: Option<String>,
}

/// 从对话文本中提取记忆
pub async fn extract_memories(
    conversation: &str,
    config: &LlmConfig,
) -> Result<Vec<ExtractedMemory>, String> {
    let api_base = config.api_base.as_deref().unwrap_or("https://api.openai.com/v1");
    let url = format!("{}/chat/completions", api_base);
    let api_key = config.api_key.as_deref().unwrap_or("");

    let system_prompt = r#"你是一个记忆提取助手。从下面的对话中提取值得记住的信息。

提取规则：
1. 只提取用户的个人信息、偏好、习惯、关系等有价值的信息
2. 不要提取闲聊内容或临时性的话题
3. 每条记忆要简洁明确，一句话说清楚
4. importance 1-5（5最重要：生日/家人 > 偏好/习惯 > 一般事实）

分类(category)选项：preference, fact, habit, relationship, work, emotion, interest

请以 JSON 数组格式返回，每个元素包含 content, category, importance, tags(可选)。
如果没有值得记住的信息，返回空数组 []。

示例：
[{"content":"用户喜欢喝拿铁","category":"preference","importance":3,"tags":"饮食"},
 {"content":"用户在北京工作","category":"fact","importance":4}]

只返回 JSON，不要其他文字。"#;

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": conversation},
            ],
            "temperature": 0.3,
        }))
        .send()
        .await
        .map_err(|e| format!("API 请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API 错误 {}: {}", status, body));
    }

    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    let content = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("[]");

    // 清理可能的 markdown 包裹
    let clean_content = content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let memories: Vec<ExtractedMemory> = serde_json::from_str(clean_content)
        .unwrap_or_else(|e| {
            tracing::warn!("解析记忆 JSON 失败: {} - 原始内容: {}", e, clean_content);
            vec![]
        });

    Ok(memories)
}
