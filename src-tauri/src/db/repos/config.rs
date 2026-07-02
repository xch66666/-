use rusqlite::Result;

use super::super::Database;
use super::super::models::ConfigEntry;

/// 默认配置
pub const DEFAULT_CONFIGS: &[(&str, &str)] = &[
    ("persona", r#"{"name":"小伴","personality":"温柔体贴，偶尔俏皮","style":"口语化，像朋友聊天","nickname":"你"}"#),
    ("tts", r#"{"enabled":false,"voice":"zh-CN-XiaoxiaoNeural","rate":1.0,"volume":0.8}"#),
    ("proactive", r#"{"enabled":true,"quiet_start":"23:00","quiet_end":"08:00","min_interval_minutes":30}"#),
    ("activity", r#"{"enabled":true,"poll_interval_seconds":10}"#),
    ("appearance", r##"{"theme":"light","accent_color":"#6366f1"}"##),
    ("llm", r#"{"provider":"openai","model":"gpt-4o-mini","max_context_messages":20,"temperature":0.8}"#),
];

impl Database {
    /// 初始化默认配置（首次启动时）
    pub fn init_default_configs(&self) -> Result<()> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();

        for (key, value) in DEFAULT_CONFIGS {
            conn.execute(
                "INSERT OR IGNORE INTO config (key, value, updated_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![key, value, now],
            )?;
        }

        Ok(())
    }

    /// 获取单个配置
    pub fn get_config(&self, key: &str) -> Result<Option<ConfigEntry>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT key, value, updated_at FROM config WHERE key = ?1",
        )?;

        let result = stmt
            .query_map(rusqlite::params![key], |row| {
                Ok(ConfigEntry {
                    key: row.get(0)?,
                    value: row.get(1)?,
                    updated_at: row.get(2)?,
                })
            })?
            .next();

        match result {
            Some(Ok(entry)) => Ok(Some(entry)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    /// 获取所有配置
    pub fn get_all_configs(&self) -> Result<Vec<ConfigEntry>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT key, value, updated_at FROM config ORDER BY key",
        )?;

        let configs = stmt
            .query_map([], |row| {
                Ok(ConfigEntry {
                    key: row.get(0)?,
                    value: row.get(1)?,
                    updated_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(configs)
    }

    /// 设置配置（存在则更新，不存在则创建）
    pub fn set_config(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();

        conn.execute(
            "INSERT INTO config (key, value, updated_at) VALUES (?1, ?2, ?3) ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
            rusqlite::params![key, value, now],
        )?;

        Ok(())
    }
}
