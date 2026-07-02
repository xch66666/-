import { useRef, useEffect } from "react";
import type { Message } from "../../types";

interface MessageBubbleProps {
  message: Message;
}

export function MessageBubble({ message }: MessageBubbleProps) {
  const isUser = message.role === "user";

  return (
    <div className={`flex ${isUser ? "justify-end" : "justify-start"} mb-3`}>
      <div
        className={`max-w-[80%] px-3 py-2 rounded-2xl text-sm leading-relaxed break-words ${
          isUser
            ? "bg-indigo-500 text-white rounded-br-sm"
            : "bg-white/90 text-gray-800 rounded-bl-sm shadow-sm border border-gray-100"
        }`}
      >
        <p className="whitespace-pre-wrap">{message.content}</p>
        <span
          className={`text-[10px] mt-1 block ${
            isUser ? "text-indigo-200" : "text-gray-400"
          }`}
        >
          {new Date(message.created_at).toLocaleTimeString("zh-CN", {
            hour: "2-digit",
            minute: "2-digit",
          })}
        </span>
      </div>
    </div>
  );
}

interface StreamingBubbleProps {
  text: string;
}

export function StreamingBubble({ text }: StreamingBubbleProps) {
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [text]);

  if (!text) return null;

  return (
    <div className="flex justify-start mb-3">
      <div className="max-w-[80%] px-3 py-2 rounded-2xl rounded-bl-sm text-sm leading-relaxed bg-white/90 text-gray-800 shadow-sm border border-gray-100">
        <p className="whitespace-pre-wrap">
          {text}
          <span className="animate-pulse ml-0.5">▌</span>
        </p>
      </div>
      <div ref={endRef} />
    </div>
  );
}
