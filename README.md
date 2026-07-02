# Companion App - AI 桌面陪伴助手

一个轻量级 Windows 11 桌面应用，AI 能主动聊天、感知用户状态、记住对话、执行简单任务。

## 仓库

GitHub: https://github.com/xch66666/-

## 文档

所有规划文档在 `docs/` 目录下，按以下顺序阅读：

1. [项目总览](docs/00-项目总览.md) - 从这里开始
2. [开发全流程指南](docs/01-开发全流程指南.md) - 整体流程
3. [技术栈选型详解](docs/02-技术栈选型详解.md) - 技术介绍
4. [产品需求文档 PRD](docs/03-产品需求文档PRD.md) - 功能定义
5. [系统架构设计](docs/04-系统架构设计.md) - 架构和模块
6. [数据库设计](docs/05-数据库设计.md) - 表结构
7. [接口设计](docs/06-接口设计.md) - 前后端通信
8. [AI 协作指南](docs/07-AI协作指南.md) - **如何用 AI 帮你开发**
9. [开发进度追踪](docs/08-开发进度追踪.md) - 进度管理

## 技术栈

- **框架**: Tauri 2.0 (Rust)
- **前端**: React + TypeScript + Tailwind CSS
- **AI**: Claude API / OpenAI API
- **数据库**: SQLite
- **语音**: Edge-TTS
- **平台**: Windows 11

## 快速开始

```bash
# 1. 确保已安装 Node.js, Rust, pnpm
# 2. 克隆仓库
git clone https://github.com/xch66666/-.git
cd -
# 3. 安装依赖并启动
pnpm install
pnpm tauri dev
```
