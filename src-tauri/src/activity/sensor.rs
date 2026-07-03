use std::path::Path;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::SystemInformation::GetTickCount;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowPlacement, GetWindowTextW, GetWindowThreadProcessId,
    WINDOWPLACEMENT, SW_SHOWMAXIMIZED,
};

use super::classifier::classify;

/// 当前活动快照，每次轮询采集一次
#[derive(Debug, Clone)]
pub struct ActivitySnapshot {
    pub activity_type: String,
    pub window_title: String,
    pub process_name: String,
    pub is_idle: bool,
    pub is_fullscreen: bool,
}

/// 活动传感器，封装 Win32 API
pub struct ActivitySensor {
    /// 空闲阈值（秒），超过此时间判定为 idle
    pub idle_threshold_secs: u64,
}

impl ActivitySensor {
    pub fn new() -> Self {
        Self {
            idle_threshold_secs: 30 * 60, // 30 分钟
        }
    }

    /// 采集当前活动快照
    pub fn collect_snapshot(&self) -> ActivitySnapshot {
        let idle_secs = get_idle_time_seconds();
        let is_idle = idle_secs > self.idle_threshold_secs;
        let is_fullscreen = is_foreground_fullscreen();

        let (process_name, window_title) = get_foreground_window_info();

        let activity_type = if is_idle {
            "idle".to_string()
        } else {
            classify(&process_name, &window_title)
        };

        ActivitySnapshot {
            activity_type,
            window_title,
            process_name,
            is_idle,
            is_fullscreen,
        }
    }
}

/// 获取前台窗口的进程名和窗口标题
fn get_foreground_window_info() -> (String, String) {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return ("unknown".into(), String::new());
        }

        // 获取窗口标题
        let title = {
            let mut buf = vec![0u16; 512];
            let len = GetWindowTextW(hwnd, &mut buf);
            String::from_utf16_lossy(&buf[..len as usize])
        };

        // 获取进程 ID
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        let process_name = get_process_name(pid);

        (process_name, title)
    }
}

/// 根据 PID 获取进程名（仅文件名部分）
fn get_process_name(pid: u32) -> String {
    unsafe {
        let handle = match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
            Ok(h) => h,
            Err(e) => {
                tracing::warn!("OpenProcess({pid}) 失败: {e}");
                return "unknown".into();
            }
        };

        let mut buf = vec![0u16; 1024];
        let len = GetModuleFileNameExW(Some(handle), None, &mut buf);
        let _ = CloseHandle(handle);

        if len == 0 {
            return "unknown".into();
        }

        let full_path = String::from_utf16_lossy(&buf[..len as usize]);
        Path::new(&full_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or(full_path)
    }
}

/// 获取用户空闲时间（秒）
fn get_idle_time_seconds() -> u64 {
    unsafe {
        let mut lii = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        if GetLastInputInfo(&mut lii).as_bool() == false {
            tracing::warn!("GetLastInputInfo 调用失败");
            return 0;
        }

        let now = GetTickCount();
        let idle_ms = now.wrapping_sub(lii.dwTime);
        (idle_ms / 1000) as u64
    }
}

/// 检测前台窗口是否全屏（排除自身窗口）
fn is_foreground_fullscreen() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return false;
        }

        // 排除自身进程窗口
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == std::process::id() {
            return false;
        }

        let mut placement = WINDOWPLACEMENT {
            length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
            ..Default::default()
        };

        if GetWindowPlacement(hwnd, &mut placement).is_err() {
            return false;
        }

        placement.showCmd == SW_SHOWMAXIMIZED.0 as u32
    }
}
