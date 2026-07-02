/// 数据库集成测试
use companion_app_lib::db::Database;
use companion_app_lib::db::models::*;

fn setup_test_db() -> Database {
    let path = format!("/tmp/companion_test_{}.db", uuid::Uuid::new_v4());
    Database::new(&path).expect("创建测试数据库失败")
}

#[test]
fn test_create_and_list_sessions() {
    let db = setup_test_db();

    // 创建会话
    let session = db.create_session(&CreateSession {
        title: Some("测试会话".to_string()),
    }).unwrap();

    assert_eq!(session.title, "测试会话");
    assert_eq!(session.message_count, 0);
    assert!(!session.is_archived);

    // 列表
    let sessions = db.list_sessions(false).unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].id, session.id);
}

#[test]
fn test_messages_crud() {
    let db = setup_test_db();

    let session = db.create_session(&CreateSession { title: None }).unwrap();

    // 创建消息
    let msg = db.create_message(&CreateMessage {
        session_id: session.id.clone(),
        role: "user".to_string(),
        content: "你好".to_string(),
        model: None,
        tokens_used: None,
        metadata: None,
    }).unwrap();

    assert_eq!(msg.content, "你好");
    assert_eq!(msg.role, "user");

    // 获取消息列表
    let messages = db.get_messages(&session.id, None).unwrap();
    assert_eq!(messages.len(), 1);

    // 验证 session 消息计数更新
    let updated = db.get_session(&session.id).unwrap().unwrap();
    assert_eq!(updated.message_count, 1);
}

#[test]
fn test_memories_crud() {
    let db = setup_test_db();

    // 创建记忆
    let mem = db.create_memory(&CreateMemory {
        content: "用户喜欢喝拿铁".to_string(),
        category: "preference".to_string(),
        importance: Some(4),
        source_session: None,
        source_message: None,
        tags: Some("饮食".to_string()),
    }).unwrap();

    assert_eq!(mem.content, "用户喜欢喝拿铁");
    assert_eq!(mem.importance, 4);

    // 列表
    let memories = db.list_memories(None).unwrap();
    assert_eq!(memories.len(), 1);

    // 搜索
    let found = db.search_memories("拿铁", 10).unwrap();
    assert_eq!(found.len(), 1);

    // 更新
    db.update_memory(&mem.id, &UpdateMemory {
        content: Some("用户喜欢喝冰拿铁".to_string()),
        category: None,
        importance: Some(5),
        is_active: None,
        tags: None,
    }).unwrap();

    // 验证更新
    let updated = db.list_memories(None).unwrap();
    assert_eq!(updated[0].content, "用户喜欢喝冰拿铁");
    assert_eq!(updated[0].importance, 5);
    assert!(updated[0].user_edited);

    // 统计
    let (total, _) = db.memory_stats().unwrap();
    assert_eq!(total, 1);

    // 删除
    db.delete_memory(&mem.id).unwrap();
    let remaining = db.list_memories(None).unwrap();
    assert_eq!(remaining.len(), 0);
}

#[test]
fn test_config_crud() {
    let db = setup_test_db();

    // 初始化默认配置
    db.init_default_configs().unwrap();

    // 获取配置
    let persona = db.get_config("persona").unwrap().unwrap();
    assert!(persona.value.contains("小伴"));

    // 获取所有
    let all = db.get_all_configs().unwrap();
    assert!(all.len() >= 6); // 至少 6 个默认配置

    // 设置配置
    db.set_config("persona", r#"{"name":"测试"}"#).unwrap();
    let updated = db.get_config("persona").unwrap().unwrap();
    assert!(updated.value.contains("测试"));
}

#[test]
fn test_activity_log() {
    let db = setup_test_db();

    let log = db.create_activity_log(&CreateActivityLog {
        activity: "coding".to_string(),
        window_title: Some("VS Code".to_string()),
        process_name: Some("Code.exe".to_string()),
        started_at: 1700000000000,
    }).unwrap();

    assert_eq!(log.activity, "coding");

    // 结束活动
    db.end_activity_log(log.id, 1700000060000).unwrap();

    // 获取最近活动
    let recent = db.get_recent_activities(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].duration_seconds, Some(60));
}

#[test]
fn test_reminders() {
    let db = setup_test_db();

    let reminder = db.create_reminder(&CreateReminder {
        content: "开会".to_string(),
        remind_at: chrono::Utc::now().timestamp_millis() - 1000, // 已过期
        is_recurring: Some(false),
        recurring_rule: None,
        source_session: None,
    }).unwrap();

    assert_eq!(reminder.content, "开会");
    assert!(!reminder.is_done);

    // 获取待处理提醒
    let pending = db.get_pending_reminders().unwrap();
    assert_eq!(pending.len(), 1);

    // 完成提醒
    db.complete_reminder(&reminder.id).unwrap();

    let pending = db.get_pending_reminders().unwrap();
    assert_eq!(pending.len(), 0);
}

#[test]
fn test_archive_and_delete_session() {
    let db = setup_test_db();

    let session = db.create_session(&CreateSession { title: None }).unwrap();

    // 归档
    db.archive_session(&session.id).unwrap();
    let archived = db.get_session(&session.id).unwrap().unwrap();
    assert!(archived.is_archived);

    // 列表不含已归档
    let active = db.list_sessions(false).unwrap();
    assert_eq!(active.len(), 0);

    // 列表包含已归档
    let all = db.list_sessions(true).unwrap();
    assert_eq!(all.len(), 1);

    // 删除
    db.delete_session(&session.id).unwrap();
    let deleted = db.get_session(&session.id).unwrap();
    assert!(deleted.is_none());
}

#[test]
fn test_recent_messages_for_context() {
    let db = setup_test_db();
    let session = db.create_session(&CreateSession { title: None }).unwrap();

    // 创建 5 条消息
    for i in 0..5 {
        db.create_message(&CreateMessage {
            session_id: session.id.clone(),
            role: if i % 2 == 0 { "user".to_string() } else { "assistant".to_string() },
            content: format!("消息 {}", i),
            model: None,
            tokens_used: None,
            metadata: None,
        }).unwrap();
    }

    // 获取最近 3 条
    let recent = db.get_recent_messages(&session.id, 3).unwrap();
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].content, "消息 2"); // 时间正序
    assert_eq!(recent[2].content, "消息 4");
}
