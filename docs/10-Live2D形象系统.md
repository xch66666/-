# Live2D 形象系统集成方案

> 让陪伴体从"一个圆头像"变成"一个活的二次元角色"

---

## 1. 什么是 Live2D

Live2D 是一种 2D 动画技术，能让平面插画产生立体动画效果。
常见于 VTuber、游戏、视觉小说。角色会呼吸、眨眼、转头、做表情。

用在我们的项目里：
```
之前：(◕‿◕)  ← 一个静态 emoji
之后：一个会呼吸、会眨眼、会做表情的二次元角色
```

---

## 2. 技术方案

### 2.1 推荐方案：pixi-live2d-display

```
PixiJS (WebGL 2D 渲染引擎)
    +
pixi-live2d-display (Live2D 模型加载器)
    =
在 Tauri 的 WebView 中渲染 Live2D 角色
```

**为什么选这个方案：**
- pixi-live2d-display 是开源免费的 JS 库
- 基于 PixiJS，用 WebGL 渲染，性能好
- 支持 Cubism 2/3/4/5 模型格式
- 支持鼠标跟踪（角色眼睛跟随鼠标）
- 支持表情切换、动画播放
- 社区活跃，文档齐全

### 2.2 渲染架构

```
┌─────────────────────────────────────────┐
│           Tauri 透明窗口                  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │         PixiJS Canvas              │  │
│  │                                    │  │
│  │     ┌─────────────────────┐       │  │
│  │     │                     │       │  │
│  │     │   Live2D 角色模型    │       │  │
│  │     │   (WebGL 渲染)      │       │  │
│  │     │                     │       │  │
│  │     │   - 呼吸动画        │       │  │
│  │     │   - 眨眼动画        │       │  │
│  │     │   - 鼠标跟踪        │       │  │
│  │     │   - 表情切换        │       │  │
│  │     │                     │       │  │
│  │     └─────────────────────┘       │  │
│  │                                    │  │
│  │   背景完全透明                      │  │
│  │   只有角色本身可见                   │  │
│  └───────────────────────────────────┘  │
│                                         │
│  气泡消息浮在角色旁边                     │
└─────────────────────────────────────────┘
```

### 2.3 技术细节

**窗口配置：**
```json
{
  "decorations": false,
  "transparent": true,
  "always_on_top": true,
  "skip_taskbar": true,
  "width": 300,
  "height": 400,
  "resizable": false
}
```

**前端依赖：**
```
pixi.js          → WebGL 2D 渲染引擎
pixi-live2d-display → Live2D 模型加载和渲染
```

**模型文件格式：**
```
model_name/
├── model.moc3          → 模型文件（Cubism 3+）
├── model.model3.json   → 模型描述文件（入口）
├── textures/
│   └── texture0.png    → 贴图
├── motions/
│   ├── idle.motion3.json   → 待机动画
│   ├── happy.motion3.json  → 开心动画
│   └── sleep.motion3.json  → 睡觉动画
├── expressions/
│   ├── default.exp3.json   → 默认表情
│   ├── happy.exp3.json     → 开心表情
│   └── sad.exp3.json       → 难过表情
└── physics/
    └── physics3.json       → 物理效果（头发/衣服飘动）
```

---

## 3. 角色状态映射

将 Live2D 的表情/动作与陪伴体状态绑定：

| 陪伴体状态 | Live2D 表情 | Live2D 动作 | 触发条件 |
|-----------|------------|------------|---------|
| idle | 默认表情 | 呼吸 + 偶尔眨眼 | 默认状态 |
| speaking | 开心/说话表情 | 轻微点头 | 气泡出现时 |
| thinking | 思考表情 | 微微歪头 | LLM 生成回复时 |
| sleeping | 闭眼/困倦表情 | 缓慢呼吸 | 深夜/长时间空闲 |
| excited | 兴奋表情 | 活泼动作 | 用户互动/好消息 |
| concerned | 担心表情 | 轻微皱眉 | 用户深夜还在/长时间工作 |

### 状态切换代码思路

```typescript
// 伪代码
import { Live2DModel } from 'pixi-live2d-display';

const model = await Live2DModel.from('models/companion/companion.model3.json');

// 根据陪伴体状态切换表情和动作
function updateCompanionState(state: CompanionState) {
  switch (state) {
    case 'idle':
      model.expression('default');
      model.motion('idle', 'normal');
      break;
    case 'speaking':
      model.expression('happy');
      model.motion('body', 'speak');
      break;
    case 'thinking':
      model.expression('default');
      model.motion('head', 'tilt');
      break;
    case 'sleeping':
      model.expression('sleepy');
      model.motion('idle', 'sleep');
      break;
    case 'concerned':
      model.expression('worried');
      model.motion('body', 'lean');
      break;
  }
}

// 鼠标跟踪（角色眼睛跟着鼠标动）
model.tracking = true;

// 点击互动
model.on('hit', (hitAreas) => {
  if (hitAreas.includes('body')) {
    model.expression('happy');
    model.motion('body', 'react');
  }
});
```

---

## 4. 模型来源

### 4.1 免费模型（开发/个人使用）

| 来源 | 说明 |
|------|------|
| **Live2D 官方示例模型** | Haru, Hiyori, Mao 等，可免费用于学习和非商业用途 |
| **Booth.pm** | 日本创作者平台，有大量免费/付费 Live2D 模型 |
| **Nizima** | Live2D 官方市场，有免费模型 |
| **GitHub 开源模型** | 一些开源 Live2D 项目附带模型 |

### 4.2 自定义模型（如果想要独特形象）

```
方案 A：找画师 + Live2D 建模师
  - 画师画分层的 PSD 原画
  - 建模师用 Live2D Cubism Editor 制作动画
  - 费用：500-5000 元（取决于复杂度）
  - 推荐平台：米画师、B站找UP主

方案 B：用 Live2D Cubism Editor 自己做
  - 官方有免费版（功能有限但够用）
  - 需要一定学习成本（约 1-2 周）
  - 教程：B站搜 "Live2D 建模教程"

方案 C：先用官方免费模型开发
  - 开发阶段用 Haru 或 Hiyori 等官方模型
  - 功能做完后再换自定义模型
  - 推荐：先走这条路
```

---

## 5. 性能影响

| 指标 | 静态头像 | Live2D 模型 | 可接受？ |
|------|---------|------------|---------|
| 内存占用 | ~30MB | ~60-80MB | 可以 |
| GPU 占用 | 几乎为0 | 低（WebGL 2D） | 可以 |
| CPU 占用 | ~0.1% | ~1-3% | 可以 |
| 启动时间 | <1秒 | 2-3秒（加载模型） | 可以 |
| 安装包大小 | ~5MB | ~15-30MB（含模型文件） | 可以 |

**结论：性能影响在可接受范围内。**

优化策略：
- 模型纹理压缩（使用 KTX2 格式）
- 空闲时降低渲染帧率（从 60fps 降到 30fps）
- 全屏/最小化时暂停渲染
- 模型文件按需加载（切换形象时才下载）

---

## 6. 实现计划

### 建议开发顺序

```
Phase 1 初期：先用简单圆形头像开发（不影响主流程）
     ↓
Phase 1 后期：集成 pixi-live2d-display，用官方免费模型替换头像
     ↓
Phase 3：支持用户自定义模型（导入/切换）
     ↓
可选：制作专属模型
```

### 集成步骤

```
1. 安装依赖
   pnpm add pixi.js pixi-live2d-display

2. 下载一个官方免费模型放到 public/models/ 目录

3. 创建 Live2D 组件，替换原来的圆形头像
   - PixiJS Application 初始化
   - 加载模型
   - 设置呼吸/眨眼自动动画
   - 启用鼠标跟踪

4. 连接状态系统
   - 监听陪伴体状态变化
   - 切换对应表情/动作

5. 连接交互
   - 点击角色 → 展开对话面板
   - 点击特定区域 → 触发反应动画
   - 拖动窗口 → 角色跟随移动
```

---

## 7. 用户自定义形象

后期可以做成这样：

```
设置页面 → "形象" 标签页

┌─────────────────────────────┐
│  当前形象                    │
│  ┌───────┐                  │
│  │ Live2D │  Haru           │
│  │ 预览   │  [更换形象]      │
│  └───────┘                  │
│                             │
│  可选形象：                  │
│  ┌────┐ ┌────┐ ┌────┐      │
│  │Haru│ │Hiyo│ │Mao │ [+]  │
│  └────┘ └────┘ └────┘      │
│                             │
│  [导入自定义模型]            │
│                             │
│  大小：[━━━━━━━━━━] 100px   │
│  位置：右下角               │
│  [x] 鼠标跟踪               │
│  [x] 呼吸动画               │
│  [x] 眨眼动画               │
└─────────────────────────────┘

用户可以：
- 从预设形象中选择
- 导入自己的 Live2D 模型（选择 .model3.json 文件）
- 调整大小和位置
- 开关各种动画效果
```

---

## 8. Live2D 授权说明

| 用途 | 授权要求 |
|------|---------|
| 个人使用/学习 | 免费 |
| 免费发布的软件 | 免费（年收入 < 1000万日元） |
| 商业软件 | 需要购买 Live2D Cubism SDK 授权 |

**对于你的项目（自己用）：完全免费，没有授权问题。**

---

## 9. 备选方案（如果觉得 Live2D 太重）

| 方案 | 优势 | 劣势 |
|------|------|------|
| **Lottie 动画** | 更轻量，JSON 格式 | 交互性不如 Live2D |
| **Spine 2D** | 游戏级 2D 动画 | 付费，偏游戏风格 |
| **帧动画 PNG** | 最简单，不需要额外库 | 不够流畅 |
| **CSS 动画 SVG** | 最轻量 | 表现力有限 |

**建议：直接用 Live2D，它就是最适合这个场景的。**
