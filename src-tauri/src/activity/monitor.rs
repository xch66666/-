use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::time::{sleep, Duration};

use crate::db::Database;

use super::sensor::{ActivitySensor, ActivitySnapshot};

/// 活动监控配置
struct ActivityConfig {
    enabled: bool,
    poll_interval_secs: u64,
}

/// 从数据库加载活动监控配置
fn load_config(db: &Database) -> ActivityConfig {
    match db.get_config("activity") {
        Ok(Some(entry)) => {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&entry.value) {
                return ActivityConfig {
                    enabled: val["enabled"].as_bool().unwrap_or(true),
                    poll_interval_secs: val["poll_interval_seconds"].as_u64().unwrap_or(10),
                };
            }
            ActivityConfig {
                enabled: true,
                poll_interval_secs: 10,
            }
        }
        _ => ActivityConfig {
            enabled: true,
            poll_interval_secs: 10,
        },
    }
}

/// 启动活动监控后台任务
///
/// 每 N 秒采集一次活动快照，检测变化后写入数据库并发射事件。
pub async fn start_activity_monitor(app: AppHandle, db: Arc<Database>) {
    tracing::info!("[activity] 活动监控任务启动");

    let sensor = ActivitySensor::new();
    let mut config = load_config(&db);
    let mut current_activity: Option<ActivitySnapshot> = None;
    let mut current_log_id: Option<i64> = None;
    let mut last_prune_date: Option<chrono::NaiveDate> = None;

    loop {
        // 每日清理旧数据
        let today = chrono::Local::now().date_naive();
        if last_prune_date.map_or(true, |d| d != today) {
            if let Err(e) = db.prune_activity_logs() {
                tracing::warn!("[activity] 清理旧数据失败: {e}");
            } else {
                tracing::info!("[activity] 已清理 30 天前的活动记录");
            }
            last_prune_date = Some(today);
        }

        // 如果禁用则跳过采集
        if !config.enabled {
            sleep(Duration::from_secs(config.poll_interval_secs)).await;
            // 重新读取配置
            config = load_config(&db);
            continue;
        }

        // 采集快照
        let snapshot = sensor.collect_snapshot();
        tracing::debug!(
            "[activity] 采集: type={}, proc={}, idle={}, fullscreen={}",
            snapshot.activity_type,
            snapshot.process_name,
            snapshot.is_idle,
            snapshot.is_fullscreen
        );

        // 全屏检测：发射事件
        if snapshot.is_fullscreen {
            let _ = app.emit(
                "activity:fullscreen",
                serde_json::json!({
                    "process_name": &snapshot.process_name,
                    "window_title": &snapshot.window_title,
                }),
            );
        }

        // 活动变化检测
        let activity_changed = match &current_activity {
            None => true,
            Some(prev) => {
                prev.activity_type != snapshot.activity_type
                    || prev.process_name != snapshot.process_name
            }
        };

        if activity_changed {
            // 结束旧活动记录
            if let Some(log_id) = current_log_id.take() {
                let now_ms = chrono::Utc::now().timestamp_millis();
                if let Err(e) = db.end_activity_log(log_id, now_ms) {
                    tracing::warn!("[activity] 结束旧活动记录失败: {e}");
                }
            }

            // 创建新活动记录
            let now_ms = chrono::Utc::now().timestamp_millis();
            let create_data = crate::db::models::CreateActivityLog {
                activity: snapshot.activity_type.clone(),
                window_title: Some(snapshot.window_title.clone()),
                process_name: Some(snapshot.process_name.clone()),
                started_at: now_ms,
            };

            match db.create_activity_log(&create_data) {
                Ok(log) => {
                    current_log_id = Some(log.id);
                    tracing::info!(
                        "[activity] 检测到活动变化: {} (进程: {})",
                        snapshot.activity_type,
                        snapshot.process_name
                    );
                }
                Err(e) => {
                    tracing::warn!("[activity] 创建活动记录失败: {e}");
                }
            }

            // 发射事件到前端
            let _ = app.emit(
                "activity:changed",
                serde_json::json!({
                    "activity": &snapshot.activity_type,
                    "window_title": &snapshot.window_title,
                    "process_name": &snapshot.process_name,
                }),
            );

            current_activity = Some(snapshot);
        }

        // 等待下一次轮询
        sleep(Duration::from_secs(config.poll_interval_secs)).await;

        // 重新读取配置（支持热更新）
        let new_config = load_config(&db);
        if new_config.poll_interval_secs != config.poll_interval_secs
            || new_config.enabled != config.enabled
        {
            tracing::info!(
                "[activity] 配置更新: enabled={}, interval={}s",
                new_config.enabled,
                new_config.poll_interval_secs
            );
            config = new_config;
        }
    }
}
