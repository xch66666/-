import { useState } from "react";
import { useChat } from "./hooks/useChat";
import { ChatPanel } from "./components/chat/ChatPanel";
import { SettingsPanel } from "./components/settings/SettingsPanel";
import { TitleBar } from "./components/common/TitleBar";

type View = "companion" | "chat" | "settings";

function App() {
  const [view, setView] = useState<View>("companion");
  const chat = useChat();

  return (
    <div className="h-screen w-screen flex flex-col bg-transparent overflow-hidden">
      {/* 自定义标题栏（可拖拽） */}
      <TitleBar />

      {/* 主内容区 */}
      <div className="flex-1 relative overflow-hidden">
        {/* 陪伴模式（默认）：Live2D 角色区域 */}
        {view === "companion" && (
          <div className="h-full flex flex-col items-center justify-center">
            {/* Live2D 占位（后续集成 PixiJS） */}
            <div className="text-center">
              <span className="text-6xl block mb-2 animate-bounce">🐾</span>
              <p className="text-sm text-gray-500 bg-white/70 px-3 py-1 rounded-full">
                小伴在这里~
              </p>
            </div>

            {/* 气泡消息占位 */}
            <div className="mt-4 bg-white/90 rounded-2xl px-4 py-2 shadow-sm border border-gray-100 max-w-[200px]">
              <p className="text-xs text-gray-600 text-center">
                今天过得怎么样？ ✨
              </p>
            </div>
          </div>
        )}

        {/* 对话面板 */}
        {view === "chat" && (
          <ChatPanel
            currentSession={chat.currentSession}
            messages={chat.messages}
            streamingText={chat.streamingText}
            isLoading={chat.isLoading}
            onSend={chat.sendMessage}
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
          onClick={() => setView("chat")}
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
