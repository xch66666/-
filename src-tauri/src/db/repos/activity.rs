use rusqlite::Result;

use super::super::Database;
use super::super::models::{ActivityLog, CreateActivityLog};

impl Database {
    /// 记录活动日志
    pub fn create_activity_log(&self, data: &CreateActivityLog) -> Result<ActivityLog> {
        let conn = self.conn();

        conn.execute(
            "INSERT INTO activity_log (activity, window_title, process_name, started_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![data.activity, data.window_title, data.process_name, data.started_at],
        )?;

        let id = conn.last_insert_rowid();

        Ok(ActivityLog {
            id,
            activity: data.activity.clone(),
            window_title: data.window_title.clone(),
            process_name: data.process_name.clone(),
            started_at: data.started_at,
            ended_at: None,
            duration_seconds: None,
        })
    }

    /// 结束当前活动日志
    pub fn end_activity_log(&self, id: i64, ended_at: i64) -> Result<()> {
        let conn = self.conn();

        // 先获取开始时间计算时长
        let started_at: i64 = conn.query_row(
            "SELECT started_at FROM activity_log WHERE id = ?1",
            rusqlite::params![id],
            |row| row.get(0),
        )?;

        let duration = (ended_at - started_at) / 1000; // 毫秒转秒

        conn.execute(
            "UPDATE activity_log SET ended_at = ?1, duration_seconds = ?2 WHERE id = ?3",
            rusqlite::params![ended_at, duration, id],
        )?;

        Ok(())
    }

    /// 获取最近的活动日志
    pub fn get_recent_activities(&self, limit: i32) -> Result<Vec<ActivityLog>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, activity, window_title, process_name, started_at, ended_at, duration_seconds FROM activity_log ORDER BY started_at DESC LIMIT ?1",
        )?;

        let logs = stmt
            .query_map(rusqlite::params![limit], |row| {
                Ok(ActivityLog {
                    id: row.get(0)?,
                    activity: row.get(1)?,
                    window_title: row.get(2)?,
                    process_name: row.get(3)?,
                    started_at: row.get(4)?,
                    ended_at: row.get(5)?,
                    duration_seconds: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(logs)
    }

    /// 获取活动统计（按类型汇总时长）
    pub fn get_activity_stats(&self, since: i64) -> Result<Vec<(String, i64)>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT activity, COALESCE(SUM(duration_seconds), 0) as total_seconds FROM activity_log WHERE started_at >= ?1 GROUP BY activity ORDER BY total_seconds DESC",
        )?;

        let stats = stmt
            .query_map(rusqlite::params![since], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(stats)
    }

    /// 清理过期的活动日志（保留 30 天）
    pub fn prune_activity_logs(&self) -> Result<i32> {
        let conn = self.conn();
        let cutoff = chrono::Utc::now().timestamp_millis() - 30 * 24 * 60 * 60 * 1000;
        let deleted = conn.execute(
            "DELETE FROM activity_log WHERE started_at < ?1",
            rusqlite::params![cutoff],
        )?;
        Ok(deleted as i32)
    }
}
