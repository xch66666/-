/// LLM 提供商抽象（统一用 OpenAI 兼容接口）
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Provider {
    OpenAI,
    DeepSeek,
    Qwen,
    Claude,
    Custom,
}

impl Provider {
    pub fn default_base_url(&self) -> &str {
        match self {
            Provider::OpenAI => "https://api.openai.com/v1",
            Provider::DeepSeek => "https://api.deepseek.com/v1",
            Provider::Qwen => "https://dashscope.aliyuncs.com/compatible-mode/v1",
            Provider::Claude => "https://api.anthropic.com/v1",
            Provider::Custom => "",
        }
    }
}
