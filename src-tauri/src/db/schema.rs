/// 所有建表 SQL
pub const CREATE_ALL_TABLES: &str = r#"
    CREATE TABLE IF NOT EXISTS sessions (
        id            TEXT PRIMARY KEY,
        title         TEXT DEFAULT '新对话',
        created_at    INTEGER NOT NULL,
        updated_at    INTEGER NOT NULL,
        message_count INTEGER DEFAULT 0,
        summary       TEXT,
        is_archived   INTEGER DEFAULT 0
    );

    CREATE INDEX IF NOT EXISTS idx_sessions_updated ON sessions(updated_at DESC);

    CREATE TABLE IF NOT EXISTS messages (
        id          TEXT PRIMARY KEY,
        session_id  TEXT NOT NULL,
        role        TEXT NOT NULL,
        content     TEXT NOT NULL,
        created_at  INTEGER NOT NULL,
        model       TEXT,
        tokens_used INTEGER,
        metadata    TEXT,
        FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id, created_at);

    CREATE TABLE IF NOT EXISTS memories (
        id               TEXT PRIMARY KEY,
        content          TEXT NOT NULL,
        category         TEXT NOT NULL,
        importance       INTEGER DEFAULT 3,
        source_session   TEXT,
        source_message   TEXT,
        created_at       INTEGER NOT NULL,
        last_recalled_at INTEGER,
        recall_count     INTEGER DEFAULT 0,
        is_active        INTEGER DEFAULT 1,
        user_edited      INTEGER DEFAULT 0,
        tags             TEXT,
        FOREIGN KEY (source_session) REFERENCES sessions(id)
    );

    CREATE INDEX IF NOT EXISTS idx_memories_category ON memories(category, is_active);
    CREATE INDEX IF NOT EXISTS idx_memories_importance ON memories(importance DESC);

    CREATE TABLE IF NOT EXISTS config (
        key        TEXT PRIMARY KEY,
        value      TEXT NOT NULL,
        updated_at INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS activity_log (
        id               INTEGER PRIMARY KEY AUTOINCREMENT,
        activity         TEXT NOT NULL,
        window_title     TEXT,
        process_name     TEXT,
        started_at       INTEGER NOT NULL,
        ended_at         INTEGER,
        duration_seconds INTEGER
    );

    CREATE INDEX IF NOT EXISTS idx_activity_time ON activity_log(started_at DESC);

    CREATE TABLE IF NOT EXISTS task_log (
        id               TEXT PRIMARY KEY,
        session_id       TEXT,
        message_id       TEXT,
        task_type        TEXT NOT NULL,
        permission_level INTEGER NOT NULL,
        user_approved    INTEGER,
        input            TEXT NOT NULL,
        output           TEXT,
        status           TEXT NOT NULL,
        error_message    TEXT,
        created_at       INTEGER NOT NULL,
        completed_at     INTEGER,
        FOREIGN KEY (session_id) REFERENCES sessions(id)
    );

    CREATE TABLE IF NOT EXISTS reminders (
        id              TEXT PRIMARY KEY,
        content         TEXT NOT NULL,
        remind_at       INTEGER NOT NULL,
        is_recurring    INTEGER DEFAULT 0,
        recurring_rule  TEXT,
        is_done         INTEGER DEFAULT 0,
        source_session  TEXT,
        created_at      INTEGER NOT NULL,
        FOREIGN KEY (source_session) REFERENCES sessions(id)
    );

    CREATE INDEX IF NOT EXISTS idx_reminders_time ON reminders(remind_at, is_done);
"#;
