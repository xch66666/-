use tauri::Manager;

/// 应用状态
pub struct AppState {
    pub db_path: String,
}

///  greet 命令 - 测试用
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! Companion 已就绪 🐾", name)
}

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

            // 初始化应用状态
            app.manage(AppState {
                db_path: app_dir
                    .join("companion.db")
                    .to_string_lossy()
                    .to_string(),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("启动 Companion 失败");
}
