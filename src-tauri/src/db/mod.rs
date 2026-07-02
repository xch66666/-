pub mod models;
pub mod repos;
pub mod schema;

use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::Mutex;

/// 数据库连接管理器
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// 打开或创建数据库
    pub fn new(path: &str) -> Result<Self> {
        // 确保目录存在
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(path)?;

        // 启用 WAL 模式（提升并发读写性能）
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;

        let db = Database {
            conn: Mutex::new(conn),
        };

        // 初始化表结构
        db.initialize_tables()?;

        Ok(db)
    }

    /// 创建所有表
    fn initialize_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(schema::CREATE_ALL_TABLES)?;
        Ok(())
    }

    /// 获取连接引用（内部使用）
    pub fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }
}
