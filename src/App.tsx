import { useState, useCallback, useEffect } from "react";
import { useChat } from "./hooks/useChat";
import { useActivity } from "./hooks/useActivity";
import { ChatPanel } from "./components/chat/ChatPanel";
import { SettingsPanel } from "./components/settings/SettingsPanel";
import { Live2DView, type CompanionState } from "./components/live2d/Live2DView";
import { BubbleMessage } from "./components/live2d/BubbleMessage";

// 随机问候语
const GREETINGS = [
  "今天过得怎么样？ ✨",
  "嘿，在忙什么呢？",
  "记得休息一下哦~ ☕",
  "好久没聊天了，想你了 💕",
  "有什么开心事分享吗？",
  "需要我帮忙吗？ 😊",
];

type FloatingPanel = "chat" | "settings" | null;

function App() {
  const [panel, setPanel] = useState<FloatingPanel>(null);
  const [companionState, setCompanionState] = useState<CompanionState>("idle");
  const [bubbleText, setBubbleText] = useState(
    GREETINGS[Math.floor(Math.random() * GREETINGS.length)]!
  );
  const chat = useChat();
  const activity = useActivity();

  // 活动 → 陪伴状态映射
  useEffect(() => {
    if (activity.isFullscreen) return;
    if (activity.isIdle) {
      setCompanionState("sleeping");
    } else if (activity.currentActivity === "entertainment") {
      setCompanionState("excited");
    } else {
      setCompanionState("idle");
    }
  }, [activity.currentActivity, activity.isIdle, activity.isFullscreen]);

  const handleSendMessage = useCallback(
    async (content: string) => {
      setCompanionState("thinking");
      setBubbleText("");
      await chat.sendMessage(content);
      setCompanionState("idle");
    },
    [chat]
  );

  const effectiveState = chat.streamingText ? "speaking" : companionState;

  const togglePanel = (name: FloatingPanel) => {
    setPanel((prev) => (prev === name ? null : name));
  };

  return (
    <div className="h-screen w-screen flex flex-col bg-transparent overflow-hidden">
      {/* 透明拖拽区域（不可见） */}
      <div className="titlebar h-6 flex-shrink-0" />

      {/* Live2D 角色（始终可见） */}
      <div className="flex-1 relative flex items-center justify-center">
        <Live2DView state={effectiveState} width={300} height={350} />

        {/* 气泡消息 */}
        <BubbleMessage
          text={bubbleText}
          autoHide={10000}
          onHide={() => {
            setTimeout(() => {
              const msg = GREETINGS[Math.floor(Math.random() * GREETINGS.length)]!;
              setBubbleText(msg);
            }, 30000 + Math.random() * 60000);
          }}
        />

        {/* 浮动操作按钮（右侧） */}
        <div className="absolute right-1 top-1/2 -translate-y-1/2 flex flex-col gap-2">
          <button
            onClick={() => togglePanel("chat")}
            className={`w-8 h-8 rounded-full flex items-center justify-center transition-all shadow-md ${
              panel === "chat"
                ? "bg-indigo-500 text-white scale-110"
                : "bg-white/80 text-gray-500 hover:bg-white hover:text-indigo-500"
            }`}
            title="聊天"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
            </svg>
          </button>
          <button
            onClick={() => togglePanel("settings")}
            className={`w-8 h-8 rounded-full flex items-center justify-center transition-all shadow-md ${
              panel === "settings"
                ? "bg-indigo-500 text-white scale-110"
                : "bg-white/80 text-gray-500 hover:bg-white hover:text-indigo-500"
            }`}
            title="设置"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </button>
        </div>
      </div>

      {/* 浮窗：聊天面板 */}
      {panel === "chat" && (
        <div className="absolute inset-0 flex items-end justify-center pb-14 pointer-events-none">
          <div className="pointer-events-auto w-[340px] max-h-[70vh] bg-white/95 backdrop-blur-md rounded-2xl shadow-2xl border border-gray-100 overflow-hidden">
            <ChatPanel
              currentSession={chat.currentSession}
              messages={chat.messages}
              streamingText={chat.streamingText}
              isLoading={chat.isLoading}
              onSend={handleSendMessage}
              onNewSession={chat.createNewSession}
              onClose={() => setPanel(null)}
            />
          </div>
        </div>
      )}

      {/* 浮窗：设置面板 */}
      {panel === "settings" && (
        <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
          <div className="pointer-events-auto w-[340px] max-h-[70vh] bg-white/95 backdrop-blur-md rounded-2xl shadow-2xl border border-gray-100 overflow-hidden">
            <SettingsPanel onClose={() => setPanel(null)} />
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
