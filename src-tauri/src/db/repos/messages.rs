use rusqlite::Result;
use uuid::Uuid;

use super::super::Database;
use super::super::models::{Message, CreateMessage};

impl Database {
    /// 创建消息
    pub fn create_message(&self, data: &CreateMessage) -> Result<Message> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();
        let id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO messages (id, session_id, role, content, created_at, model, tokens_used, metadata) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                id,
                data.session_id,
                data.role,
                data.content,
                now,
                data.model,
                data.tokens_used,
                data.metadata,
            ],
        )?;

        // 同时更新 session 的消息计数和活跃时间
        conn.execute(
            "UPDATE sessions SET message_count = message_count + 1, updated_at = ?1 WHERE id = ?2",
            rusqlite::params![now, data.session_id],
        )?;

        Ok(Message {
            id,
            session_id: data.session_id.clone(),
            role: data.role.clone(),
            content: data.content.clone(),
            created_at: now,
            model: data.model.clone(),
            tokens_used: data.tokens_used,
            metadata: data.metadata.clone(),
        })
    }

    /// 获取会话的所有消息
    pub fn get_messages(&self, session_id: &str, limit: Option<i32>) -> Result<Vec<Message>> {
        let conn = self.conn();
        let sql = match limit {
            Some(n) => format!(
                "SELECT id, session_id, role, content, created_at, model, tokens_used, metadata FROM messages WHERE session_id = ?1 ORDER BY created_at ASC LIMIT {}",
                n
            ),
            None => "SELECT id, session_id, role, content, created_at, model, tokens_used, metadata FROM messages WHERE session_id = ?1 ORDER BY created_at ASC".to_string(),
        };

        let mut stmt = conn.prepare(&sql)?;
        let messages = stmt
            .query_map(rusqlite::params![session_id], |row| {
                Ok(Message {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    model: row.get(5)?,
                    tokens_used: row.get(6)?,
                    metadata: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(messages)
    }

    /// 获取最近的 N 条消息（用于 AI 上下文）
    pub fn get_recent_messages(&self, session_id: &str, limit: i32) -> Result<Vec<Message>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, role, content, created_at, model, tokens_used, metadata FROM messages WHERE session_id = ?1 ORDER BY created_at DESC LIMIT ?2",
        )?;

        let mut messages: Vec<Message> = stmt
            .query_map(rusqlite::params![session_id, limit], |row| {
                Ok(Message {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    model: row.get(5)?,
                    tokens_used: row.get(6)?,
                    metadata: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        // 反转为时间正序
        messages.reverse();
        Ok(messages)
    }

    /// 删除单条消息
    pub fn delete_message(&self, id: &str) -> Result<()> {
        let conn = self.conn();
        conn.execute("DELETE FROM messages WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }
}
