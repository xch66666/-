# Companion App - AI 开发上下文

## 项目概述
Windows 11 桌面 AI 陪伴体。不是聊天应用，是一个住在桌面上的 AI 小伙伴。
核心形态：Live2D 二次元角色 + 气泡消息 + 可展开对话面板。
AI 能感知用户在做什么，在合适的时机主动说话，有记忆和人设。

## 版本控制
- **GitHub 仓库**: https://github.com/xch66666/-
- 每完成一个小功能就 commit，使用中文 commit message

## 技术栈
- 框架: Tauri 2.0 (Rust 后端)
- 前端: React + TypeScript + Tailwind CSS
- 形象渲染: PixiJS + pixi-live2d-display (Live2D 二次元角色)
- AI: Claude API / OpenAI API
- 数据库: SQLite
- 语音: Edge-TTS (后期)
- 平台: Windows 11

## 产品核心（开发时始终记住）
1. 它是"陪伴"不是"工具" —— 存在感设计，像一个室友
2. 活动感知是基础 —— 不知道用户在干嘛就无法适时陪伴
3. 主动说话是灵魂 —— 不是用户找AI，是AI找用户
4. 知道什么时候闭嘴 —— 防打扰和主动说话同样重要
5. 轻量化 —— 常驻桌面，内存 < 80MB，CPU < 3%
6. Live2D 形象 —— 会呼吸、眨眼、做表情，让陪伴有温度

## 功能优先级
- P0: 桌面悬浮 Live2D 角色 + 气泡 + 活动感知 + 主动说话 + 对话 + 记忆
- P1: 人设系统 + 防打扰机制 + 系统托盘 + Live2D 表情状态映射
- P2: 语音输出 + 任务执行 + 自定义 Live2D 模型导入
- P3: 语音输入 + 外观自定义

## 已完成
- [ ] 项目搭建
- [ ] 桌面悬浮 Live2D 角色组件
- [ ] 气泡消息组件
- [ ] 活动感知 (Win32 API)
- [ ] 陪伴决策引擎
- [ ] 对话面板
- [ ] 记忆系统
- [ ] 人设系统
- [ ] Live2D 表情与状态映射

## 技术约定
- IPC 通道命名：模块名:动作名（如 companion:speak）
- 数据库：~/.companion/companion.db
- 所有时间使用 Unix 毫秒时间戳
- 活动感知数据只在本地处理，不上传
- Live2D 模型文件存放在 public/models/ 目录

## 文档
docs/ 目录下有完整规划文档，修改前请参考。
重点参考：docs/10-Live2D形象系统.md
