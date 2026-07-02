import { useRef, useEffect } from "react";
import { MessageBubble, StreamingBubble } from "./MessageBubble";
import { ChatInput } from "./ChatInput";
import type { Message, Session } from "../../types";

interface ChatPanelProps {
  currentSession: Session | null;
  messages: Message[];
  streamingText: string;
  isLoading: boolean;
  onSend: (content: string) => void;
  onNewSession: () => void;
  onClose: () => void;
}

export function ChatPanel({
  currentSession,
  messages,
  streamingText,
  isLoading,
  onSend,
  onNewSession,
  onClose,
}: ChatPanelProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  return (
    <div className="flex flex-col h-full bg-gray-50/95 backdrop-blur-md rounded-2xl shadow-xl border border-gray-200/50 overflow-hidden">
      {/* 头部 */}
      <div className="flex items-center justify-between px-4 py-3 bg-white/80 border-b border-gray-100 titlebar">
        <div className="flex items-center gap-2">
          <span className="text-lg">🐾</span>
          <h2 className="text-sm font-semibold text-gray-700">
            {currentSession?.title || "Companion"}
          </h2>
        </div>
        <div className="flex gap-1" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button
            onClick={onNewSession}
            className="p-1.5 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition"
            title="新建对话"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
            </svg>
          </button>
          <button
            onClick={onClose}
            className="p-1.5 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition"
            title="收起"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      {/* 消息列表 */}
      <div className="flex-1 overflow-y-auto px-4 py-3 space-y-1">
        {messages.length === 0 && !isLoading && (
          <div className="flex flex-col items-center justify-center h-full text-gray-400">
            <span className="text-4xl mb-3">🐾</span>
            <p className="text-sm">开始聊天吧~</p>
          </div>
        )}

        {messages.map((msg) => (
          <MessageBubble key={msg.id} message={msg} />
        ))}

        <StreamingBubble text={streamingText} />

        {isLoading && !streamingText && (
          <div className="flex justify-start mb-3">
            <div className="px-3 py-2 rounded-2xl bg-white/90 shadow-sm border border-gray-100">
              <div className="flex gap-1">
                <span className="w-2 h-2 bg-gray-300 rounded-full animate-bounce" style={{ animationDelay: "0ms" }} />
                <span className="w-2 h-2 bg-gray-300 rounded-full animate-bounce" style={{ animationDelay: "150ms" }} />
                <span className="w-2 h-2 bg-gray-300 rounded-full animate-bounce" style={{ animationDelay: "300ms" }} />
              </div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* 输入框 */}
      <ChatInput onSend={onSend} disabled={isLoading} />
    </div>
  );
}
