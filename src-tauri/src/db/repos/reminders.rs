use rusqlite::Result;
use uuid::Uuid;

use super::super::Database;
use super::super::models::{Reminder, CreateReminder};

impl Database {
    /// 创建提醒
    pub fn create_reminder(&self, data: &CreateReminder) -> Result<Reminder> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();
        let id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO reminders (id, content, remind_at, is_recurring, recurring_rule, is_done, source_session, created_at) VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, ?7)",
            rusqlite::params![
                id,
                data.content,
                data.remind_at,
                data.is_recurring.unwrap_or(false) as i32,
                data.recurring_rule,
                data.source_session,
                now,
            ],
        )?;

        Ok(Reminder {
            id,
            content: data.content.clone(),
            remind_at: data.remind_at,
            is_recurring: data.is_recurring.unwrap_or(false),
            recurring_rule: data.recurring_rule.clone(),
            is_done: false,
            source_session: data.source_session.clone(),
            created_at: now,
        })
    }

    /// 获取未完成的提醒（按时间排序）
    pub fn get_pending_reminders(&self) -> Result<Vec<Reminder>> {
        let conn = self.conn();
        let now = chrono::Utc::now().timestamp_millis();

        let mut stmt = conn.prepare(
            "SELECT id, content, remind_at, is_recurring, recurring_rule, is_done, source_session, created_at FROM reminders WHERE is_done = 0 AND remind_at <= ?1 ORDER BY remind_at ASC",
        )?;

        let reminders = stmt
            .query_map(rusqlite::params![now], |row| {
                Ok(Reminder {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    remind_at: row.get(2)?,
                    is_recurring: row.get::<_, i32>(3)? != 0,
                    recurring_rule: row.get(4)?,
                    is_done: row.get::<_, i32>(5)? != 0,
                    source_session: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(reminders)
    }

    /// 获取所有提醒
    pub fn list_reminders(&self) -> Result<Vec<Reminder>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, content, remind_at, is_recurring, recurring_rule, is_done, source_session, created_at FROM reminders ORDER BY remind_at ASC",
        )?;

        let reminders = stmt
            .query_map([], |row| {
                Ok(Reminder {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    remind_at: row.get(2)?,
                    is_recurring: row.get::<_, i32>(3)? != 0,
                    recurring_rule: row.get(4)?,
                    is_done: row.get::<_, i32>(5)? != 0,
                    source_session: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(reminders)
    }

    /// 标记提醒为已完成
    pub fn complete_reminder(&self, id: &str) -> Result<()> {
        let conn = self.conn();
        conn.execute(
            "UPDATE reminders SET is_done = 1 WHERE id = ?1",
            rusqlite::params![id],
        )?;
        Ok(())
    }

    /// 删除提醒
    pub fn delete_reminder(&self, id: &str) -> Result<()> {
        let conn = self.conn();
        conn.execute("DELETE FROM reminders WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }

    /// 清理已完成且过期的提醒
    pub fn prune_reminders(&self) -> Result<i32> {
        let conn = self.conn();
        let cutoff = chrono::Utc::now().timestamp_millis() - 7 * 24 * 60 * 60 * 1000; // 7天前
        let deleted = conn.execute(
            "DELETE FROM reminders WHERE is_done = 1 AND remind_at < ?1",
            rusqlite::params![cutoff],
        )?;
        Ok(deleted as i32)
    }
}
