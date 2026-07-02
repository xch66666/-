use rusqlite::Result;
use uuid::Uuid;

use super::super::Database;
use super::super::models::{Memory, CreateMemory, UpdateMemory};

impl Database {
    /// 创建记忆
    pub fn create_memory(&self, data: &CreateMemory) -> Result<Memory> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();
        let id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO memories (id, content, category, importance, source_session, source_message, created_at, recall_count, is_active, user_edited, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, 1, 0, ?8)",
            rusqlite::params![
                id,
                data.content,
                data.category,
                data.importance.unwrap_or(3),
                data.source_session,
                data.source_message,
                now,
                data.tags,
            ],
        )?;

        Ok(Memory {
            id,
            content: data.content.clone(),
            category: data.category.clone(),
            importance: data.importance.unwrap_or(3),
            source_session: data.source_session.clone(),
            source_message: data.source_message.clone(),
            created_at: now,
            last_recalled_at: None,
            recall_count: 0,
            is_active: true,
            user_edited: false,
            tags: data.tags.clone(),
        })
    }

    /// 获取所有活跃记忆
    pub fn list_memories(&self, category: Option<&str>) -> Result<Vec<Memory>> {
        let conn = self.conn();

        let (sql, params): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = match category {
            Some(cat) => (
                "SELECT id, content, category, importance, source_session, source_message, created_at, last_recalled_at, recall_count, is_active, user_edited, tags FROM memories WHERE is_active = 1 AND category = ?1 ORDER BY importance DESC, created_at DESC".to_string(),
                vec![Box::new(cat.to_string())],
            ),
            None => (
                "SELECT id, content, category, importance, source_session, source_message, created_at, last_recalled_at, recall_count, is_active, user_edited, tags FROM memories WHERE is_active = 1 ORDER BY importance DESC, created_at DESC".to_string(),
                vec![],
            ),
        };

        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let memories = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(Memory {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    category: row.get(2)?,
                    importance: row.get(3)?,
                    source_session: row.get(4)?,
                    source_message: row.get(5)?,
                    created_at: row.get(6)?,
                    last_recalled_at: row.get(7)?,
                    recall_count: row.get(8)?,
                    is_active: row.get::<_, i32>(9)? != 0,
                    user_edited: row.get::<_, i32>(10)? != 0,
                    tags: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(memories)
    }

    /// 搜索记忆（全文搜索）
    pub fn search_memories(&self, query: &str, limit: i32) -> Result<Vec<Memory>> {
        let conn = self.conn();
        let search_pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, content, category, importance, source_session, source_message, created_at, last_recalled_at, recall_count, is_active, user_edited, tags FROM memories WHERE is_active = 1 AND (content LIKE ?1 OR tags LIKE ?1) ORDER BY importance DESC, recall_count DESC LIMIT ?2",
        )?;

        let memories = stmt
            .query_map(rusqlite::params![search_pattern, limit], |row| {
                Ok(Memory {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    category: row.get(2)?,
                    importance: row.get(3)?,
                    source_session: row.get(4)?,
                    source_message: row.get(5)?,
                    created_at: row.get(6)?,
                    last_recalled_at: row.get(7)?,
                    recall_count: row.get(8)?,
                    is_active: row.get::<_, i32>(9)? != 0,
                    user_edited: row.get::<_, i32>(10)? != 0,
                    tags: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(memories)
    }

    /// 更新记忆
    pub fn update_memory(&self, id: &str, data: &UpdateMemory) -> Result<()> {
        let conn = self.conn();
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref content) = data.content {
            updates.push(format!("content = ?{}", params.len() + 1));
            params.push(Box::new(content.clone()));
        }
        if let Some(ref category) = data.category {
            updates.push(format!("category = ?{}", params.len() + 1));
            params.push(Box::new(category.clone()));
        }
        if let Some(importance) = data.importance {
            updates.push(format!("importance = ?{}", params.len() + 1));
            params.push(Box::new(importance));
        }
        if let Some(is_active) = data.is_active {
            updates.push(format!("is_active = ?{}", params.len() + 1));
            params.push(Box::new(if is_active { 1 } else { 0 }));
        }
        if let Some(ref tags) = data.tags {
            updates.push(format!("tags = ?{}", params.len() + 1));
            params.push(Box::new(tags.clone()));
        }

        if data.content.is_some() {
            updates.push("user_edited = 1".to_string());
        }

        if updates.is_empty() {
            return Ok(());
        }

        let sql = format!(
            "UPDATE memories SET {} WHERE id = ?{}",
            updates.join(", "),
            params.len() + 1
        );
        params.push(Box::new(id.to_string()));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        conn.execute(&sql, param_refs.as_slice())?;

        Ok(())
    }

    /// 记录记忆被检索使用
    pub fn record_memory_recall(&self, id: &str) -> Result<()> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();
        conn.execute(
            "UPDATE memories SET last_recalled_at = ?1, recall_count = recall_count + 1 WHERE id = ?2",
            rusqlite::params![now, id],
        )?;
        Ok(())
    }

    /// 获取记忆统计
    pub fn memory_stats(&self) -> Result<(i32, i32)> {
        let conn = self.conn();
        let total: i32 = conn.query_row(
            "SELECT COUNT(*) FROM memories WHERE is_active = 1",
            [],
            |row| row.get(0),
        )?;
        let by_category: i32 = conn.query_row(
            "SELECT COUNT(DISTINCT category) FROM memories WHERE is_active = 1",
            [],
            |row| row.get(0),
        )?;
        Ok((total, by_category))
    }

    /// 删除记忆
    pub fn delete_memory(&self, id: &str) -> Result<()> {
        let conn = self.conn();
        conn.execute("DELETE FROM memories WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }

    /// 淘汰低分记忆（当记忆超过 500 条时）
    pub fn prune_memories(&self, max_count: i32) -> Result<i32> {
        let conn = self.conn();

        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM memories WHERE is_active = 1",
            [],
            |row| row.get(0),
        )?;

        if count <= max_count {
            return Ok(0);
        }

        // 计算淘汰数量（最低 10%）
        let prune_count = ((count - max_count) as f64 * 0.1).ceil() as i32;
        if prune_count <= 0 {
            return Ok(0);
        }

        let now = chrono::Utc::now().timestamp_millis();

        // 按打分公式淘汰：importance * 0.4 + recency * 0.3 + recall_freq * 0.2 + user_edited * 0.1
        conn.execute(
            &format!(
                "UPDATE memories SET is_active = 0 WHERE id IN (
                    SELECT id FROM memories WHERE is_active = 1
                    ORDER BY (importance * 0.4 + (created_at / 1000000000.0 / (?1 / 1000000000.0)) * 0.3 + (recall_count / 10.0) * 0.2 + user_edited * 0.1) ASC
                    LIMIT {}
                )",
                prune_count
            ),
            rusqlite::params![now],
        )?;

        Ok(prune_count)
    }
}
