import { useState, useCallback, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import * as api from "../api";
import type { Session, Message } from "../types";

interface UseChatReturn {
  sessions: Session[];
  currentSession: Session | null;
  messages: Message[];
  isLoading: boolean;
  streamingText: string;
  createNewSession: () => Promise<void>;
  selectSession: (session: Session) => Promise<void>;
  sendMessage: (content: string) => Promise<void>;
  deleteSession: (id: string) => Promise<void>;
}

export function useChat(): UseChatReturn {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [currentSession, setCurrentSession] = useState<Session | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [streamingText, setStreamingText] = useState("");
  const streamingRef = useRef("");

  // 加载会话列表
  useEffect(() => {
    api.listSessions().then(setSessions).catch(console.error);
  }, []);

  // 监听流式输出事件
  useEffect(() => {
    const unlistenChunk = listen<{ session_id: string; chunk: string }>(
      "chat:stream_chunk",
      (event) => {
        if (event.payload.session_id === currentSession?.id) {
          streamingRef.current += event.payload.chunk;
          setStreamingText(streamingRef.current);
        }
      }
    );

    const unlistenDone = listen<{
      session_id: string;
      message_id: string;
    }>("chat:stream_done", async () => {
      streamingRef.current = "";
      setStreamingText("");
      setIsLoading(false);
      // 重新加载消息
      if (currentSession) {
        const msgs = await api.getMessages(currentSession.id);
        setMessages(msgs);
      }
    });

    return () => {
      unlistenChunk.then((fn) => fn());
      unlistenDone.then((fn) => fn());
    };
  }, [currentSession]);

  const createNewSession = useCallback(async () => {
    const session = await api.createSession();
    setSessions((prev) => [session, ...prev]);
    setCurrentSession(session);
    setMessages([]);
  }, []);

  const selectSession = useCallback(async (session: Session) => {
    setCurrentSession(session);
    const msgs = await api.getMessages(session.id);
    setMessages(msgs);
  }, []);

  const sendMessage = useCallback(
    async (content: string) => {
      if (!currentSession || !content.trim()) return;

      setIsLoading(true);
      streamingRef.current = "";
      setStreamingText("");

      // 乐观更新：立即显示用户消息
      const optimisticMsg: Message = {
        id: `temp-${Date.now()}`,
        session_id: currentSession.id,
        role: "user",
        content: content.trim(),
        created_at: Date.now(),
        model: null,
        tokens_used: null,
        metadata: null,
      };
      setMessages((prev) => [...prev, optimisticMsg]);

      try {
        await api.sendMessage(currentSession.id, content.trim());
      } catch (err) {
        console.error("发送失败:", err);
        setIsLoading(false);
      }
    },
    [currentSession]
  );

  const deleteSession = useCallback(
    async (id: string) => {
      await api.deleteSession(id);
      setSessions((prev) => prev.filter((s) => s.id !== id));
      if (currentSession?.id === id) {
        setCurrentSession(null);
        setMessages([]);
      }
    },
    [currentSession]
  );

  return {
    sessions,
    currentSession,
    messages,
    isLoading,
    streamingText,
    createNewSession,
    selectSession,
    sendMessage,
    deleteSession,
  };
}
