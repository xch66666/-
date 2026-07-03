/// 根据进程名和窗口标题分类用户活动
///
/// 优先级: coding > social > work > 浏览器消歧 > entertainment > "other"
pub fn classify(process_name: &str, window_title: &str) -> String {
    let proc = process_name.to_lowercase();
    let title = window_title.to_lowercase();

    // 1. 编程
    if contains_any(
        &proc,
        &[
            "code.exe", "code -", "intellij", "idea64", "pycharm", "webstorm", "goland",
            "rider", "clion", "terminal", "cmd.exe", "powershell", "pwsh", "wt.exe",
            "windows terminal",
        ],
    ) {
        return "coding".into();
    }

    // 2. 社交
    if contains_any(
        &proc,
        &[
            "wechat", "weixin", "qq.exe", "discord", "telegram", "slack", "teams",
            "dingtalk", "feishu", "lark",
        ],
    ) {
        return "social".into();
    }

    // 3. 办公
    if contains_any(
        &proc,
        &[
            "winword", "excel", "outlook", "powerpnt", "wps.exe", "et.exe", "wpp.exe",
            "onenote",
        ],
    ) {
        return "work".into();
    }

    // 4. 浏览器消歧（根据窗口标题区分学习 vs 娱乐）
    if contains_any(&proc, &["chrome", "firefox", "msedge", "brave", "opera"]) {
        if contains_any(
            &title,
            &[
                "youtube", "bilibili", "netflix", "twitch", "spotify", "music", "video",
                "电影", "视频", "番剧",
            ],
        ) {
            return "entertainment".into();
        }
        if contains_any(
            &title,
            &[
                "github", "stackoverflow", "docs", "tutorial", "mdn", "learn", "course",
                "wikipedia", "文档", "学习",
            ],
        ) {
            return "learning".into();
        }
        return "learning".into(); // 默认浏览归为学习
    }

    // 5. 独立娱乐应用
    if contains_any(
        &proc,
        &[
            "steam", "epicgameslauncher", "spotify", "cloudmusic", "vlc", "potplayer",
            "kmplayer",
        ],
    ) {
        return "entertainment".into();
    }

    "other".into()
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| haystack.contains(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coding() {
        assert_eq!(classify("Code.exe", "main.rs - companion"), "coding");
        assert_eq!(classify("powershell.exe", "PS C:\\dev"), "coding");
        assert_eq!(classify("idea64.exe", "MyProject"), "coding");
    }

    #[test]
    fn test_social() {
        assert_eq!(classify("WeChat.exe", "微信"), "social");
        assert_eq!(classify("discord.exe", "General"), "social");
    }

    #[test]
    fn test_work() {
        assert_eq!(classify("WINWORD.EXE", "报告.docx"), "work");
        assert_eq!(classify("EXCEL.EXE", "预算表"), "work");
    }

    #[test]
    fn test_browser_entertainment() {
        assert_eq!(classify("chrome.exe", "YouTube - 视频"), "entertainment");
        assert_eq!(
            classify("msedge.exe", "bilibili - 番剧"),
            "entertainment"
        );
    }

    #[test]
    fn test_browser_learning() {
        assert_eq!(
            classify("chrome.exe", "GitHub - repository"),
            "learning"
        );
        assert_eq!(
            classify("firefox.exe", "MDN Web Docs"),
            "learning"
        );
    }

    #[test]
    fn test_standalone_entertainment() {
        assert_eq!(classify("steam.exe", "Steam"), "entertainment");
        assert_eq!(classify("Spotify.exe", "Playlist"), "entertainment");
    }

    #[test]
    fn test_other() {
        assert_eq!(classify("explorer.exe", "此电脑"), "other");
        assert_eq!(classify("notepad.exe", "untitled"), "other");
    }
}
