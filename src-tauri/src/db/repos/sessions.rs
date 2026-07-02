use rusqlite::Result;
use uuid::Uuid;

use super::super::Database;
use super::super::models::{Session, CreateSession};

impl Database {
    /// 创建新会话
    pub fn create_session(&self, data: &CreateSession) -> Result<Session> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();
        let id = Uuid::new_v4().to_string();
        let title = data.title.as_deref().unwrap_or("新对话");

        conn.execute(
            "INSERT INTO sessions (id, title, created_at, updated_at, message_count, is_archived) VALUES (?1, ?2, ?3, ?4, 0, 0)",
            rusqlite::params![id, title, now, now],
        )?;

        Ok(Session {
            id,
            title: title.to_string(),
            created_at: now,
            updated_at: now,
            message_count: 0,
            summary: None,
            is_archived: false,
        })
    }

    /// 获取所有会话列表
    pub fn list_sessions(&self, include_archived: bool) -> Result<Vec<Session>> {
        let conn = self.conn();
        let sql = if include_archived {
            "SELECT id, title, created_at, updated_at, message_count, summary, is_archived FROM sessions ORDER BY updated_at DESC"
        } else {
            "SELECT id, title, created_at, updated_at, message_count, summary, is_archived FROM sessions WHERE is_archived = 0 ORDER BY updated_at DESC"
        };

        let mut stmt = conn.prepare(sql)?;
        let sessions = stmt
            .query_map([], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    message_count: row.get(4)?,
                    summary: row.get(5)?,
                    is_archived: row.get::<_, i32>(6)? != 0,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(sessions)
    }

    /// 获取单个会话
    pub fn get_session(&self, id: &str) -> Result<Option<Session>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, title, created_at, updated_at, message_count, summary, is_archived FROM sessions WHERE id = ?1",
        )?;

        let result = stmt
            .query_map(rusqlite::params![id], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    message_count: row.get(4)?,
                    summary: row.get(5)?,
                    is_archived: row.get::<_, i32>(6)? != 0,
                })
            })?
            .next();

        match result {
            Some(Ok(session)) => Ok(Some(session)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    /// 更新会话标题
    pub fn update_session_title(&self, id: &str, title: &str) -> Result<()> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();
        conn.execute(
            "UPDATE sessions SET title = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![title, now, id],
        )?;
        Ok(())
    }

    /// 更新会话摘要
    pub fn update_session_summary(&self, id: &str, summary: &str) -> Result<()> {
        let conn = self.conn();
        conn.execute(
            "UPDATE sessions SET summary = ?1 WHERE id = ?2",
            rusqlite::params![summary, id],
        )?;
        Ok(())
    }

    /// 增加消息计数并更新活跃时间
    pub fn increment_message_count(&self, session_id: &str) -> Result<()> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();
        conn.execute(
            "UPDATE sessions SET message_count = message_count + 1, updated_at = ?1 WHERE id = ?2",
            rusqlite::params![now, session_id],
        )?;
        Ok(())
    }

    /// 归档会话
    pub fn archive_session(&self, id: &str) -> Result<()> {
        let conn = self.conn();
        conn.execute(
            "UPDATE sessions SET is_archived = 1 WHERE id = ?1",
            rusqlite::params![id],
        )?;
        Ok(())
    }

    /// 删除会话（级联删除消息）
    pub fn delete_session(&self, id: &str) -> Result<()> {
        let conn = self.conn();
        conn.execute("DELETE FROM sessions WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }
}
