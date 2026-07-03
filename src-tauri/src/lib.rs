pub mod activity;
pub mod ai;
pub mod commands;
pub mod db;
pub mod memory;

use std::sync::Arc;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 创建应用数据目录
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("无法获取应用数据目录");
            std::fs::create_dir_all(&app_dir).expect("无法创建应用数据目录");

            tracing::info!("Companion 启动，数据目录: {:?}", app_dir);

            // 初始化数据库
            let db_path = app_dir
                .join("companion.db")
                .to_string_lossy()
                .to_string();

            let database = Database::new(&db_path)
                .expect("数据库初始化失败");

            // 初始化默认配置
            database.init_default_configs()
                .expect("默认配置初始化失败");

            tracing::info!("数据库初始化完成: {}", db_path);

            // 包裹 Arc 以便后台任务和命令共享
            let db = Arc::new(database);
            app.manage(db.clone());

            // 启动活动监控后台任务
            let app_handle = app.handle().clone();
            let db_for_monitor = db.clone();
            tauri::async_runtime::spawn(async move {
                activity::monitor::start_activity_monitor(app_handle, db_for_monitor).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 对话模块
            commands::chat::chat_create_session,
            commands::chat::chat_list_sessions,
            commands::chat::chat_get_messages,
            commands::chat::chat_send,
            commands::chat::chat_delete_session,
            commands::chat::chat_archive_session,
            // 记忆模块
            commands::memory::memory_list,
            commands::memory::memory_search,
            commands::memory::memory_update,
            commands::memory::memory_delete,
            commands::memory::memory_stats,
            commands::memory::memory_extract,
            // 配置模块
            commands::config::config_get,
            commands::config::config_get_all,
            commands::config::config_set,
            // 活动模块
            commands::activity::activity_get_current,
            commands::activity::activity_get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Companion 失败");
}

use db::Database;
