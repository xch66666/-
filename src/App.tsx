import { useState, useCallback } from "react";
import { useChat } from "./hooks/useChat";
import { ChatPanel } from "./components/chat/ChatPanel";
import { SettingsPanel } from "./components/settings/SettingsPanel";
import { TitleBar } from "./components/common/TitleBar";
import { Live2DView, type CompanionState } from "./components/live2d/Live2DView";
import { BubbleMessage } from "./components/live2d/BubbleMessage";

type View = "companion" | "chat" | "settings";

// 随机问候语
const GREETINGS = [
  "今天过得怎么样？ ✨",
  "嘿，在忙什么呢？",
  "记得休息一下哦~ ☕",
  "好久没聊天了，想你了 💕",
  "有什么开心事分享吗？",
  "需要我帮忙吗？ 😊",
];

function App() {
  const [view, setView] = useState<View>("companion");
  const [companionState, setCompanionState] = useState<CompanionState>("idle");
  const [bubbleText, setBubbleText] = useState(
    GREETINGS[Math.floor(Math.random() * GREETINGS.length)]!
  );
  const chat = useChat();

  const handleSendMessage = useCallback(
    async (content: string) => {
      setCompanionState("thinking");
      setBubbleText("");
      await chat.sendMessage(content);
      setCompanionState("idle");
    },
    [chat]
  );

  // 流式输出时显示 speaking 状态
  const effectiveState = chat.streamingText ? "speaking" : companionState;

  return (
    <div className="h-screen w-screen flex flex-col bg-transparent overflow-hidden">
      {/* 自定义标题栏（可拖拽） */}
      <TitleBar />

      {/* 主内容区 */}
      <div className="flex-1 relative overflow-hidden">
        {/* 陪伴模式：Live2D 角色 + 气泡 */}
        {view === "companion" && (
          <div className="h-full relative flex flex-col items-center justify-center">
            {/* Live2D 模型渲染 */}
            <Live2DView
              state={effectiveState}
              width={300}
              height={350}
            />

            {/* 气泡消息 */}
            <BubbleMessage
              text={bubbleText}
              autoHide={10000}
              onHide={() => {
                // 随机显示新问候
                setTimeout(() => {
                  const msg = GREETINGS[Math.floor(Math.random() * GREETINGS.length)]!;
                  setBubbleText(msg);
                }, 30000 + Math.random() * 60000); // 30-90 秒后出现
              }}
            />

            {/* 点击角色打开对话 */}
            <button
              onClick={() => setView("chat")}
              className="absolute bottom-4 left-1/2 -translate-x-1/2 bg-white/80 backdrop-blur-sm rounded-full px-4 py-1.5 text-xs text-gray-500 hover:bg-white hover:text-gray-700 transition shadow-sm border border-gray-100"
            >
              💬 聊天
            </button>
          </div>
        )}

        {/* 对话面板 */}
        {view === "chat" && (
          <ChatPanel
            currentSession={chat.currentSession}
            messages={chat.messages}
            streamingText={chat.streamingText}
            isLoading={chat.isLoading}
            onSend={handleSendMessage}
            onNewSession={chat.createNewSession}
            onClose={() => setView("companion")}
          />
        )}

        {/* 设置面板 */}
        {view === "settings" && (
          <SettingsPanel onClose={() => setView("companion")} />
        )}
      </div>

      {/* 底部操作栏 */}
      <div className="flex items-center justify-center gap-4 py-2 bg-white/60 backdrop-blur-sm border-t border-gray-100/50">
        <button
          onClick={() => setView("companion")}
          className={`p-2 rounded-xl transition ${
            view === "companion"
              ? "bg-indigo-100 text-indigo-600"
              : "text-gray-400 hover:text-gray-600 hover:bg-gray-100"
          }`}
          title="陪伴"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
          </svg>
        </button>
        <button
          onClick={() => {
            setView("chat");
            if (!chat.currentSession) chat.createNewSession();
          }}
          className={`p-2 rounded-xl transition ${
            view === "chat"
              ? "bg-indigo-100 text-indigo-600"
              : "text-gray-400 hover:text-gray-600 hover:bg-gray-100"
          }`}
          title="对话"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
          </svg>
        </button>
        <button
          onClick={() => setView("settings")}
          className={`p-2 rounded-xl transition ${
            view === "settings"
              ? "bg-indigo-100 text-indigo-600"
              : "text-gray-400 hover:text-gray-600 hover:bg-gray-100"
          }`}
          title="设置"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>
      </div>
    </div>
  );
}

export default App;
