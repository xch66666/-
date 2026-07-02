import { invoke } from "@tauri-apps/api/core";
import type { Session, Message } from "../types";

/** 创建新对话 */
export async function createSession(title?: string): Promise<Session> {
  return invoke("chat_create_session", { title });
}

/** 获取会话列表 */
export async function listSessions(
  includeArchived = false
): Promise<Session[]> {
  return invoke("chat_list_sessions", { includeArchived });
}

/** 获取会话消息 */
export async function getMessages(
  sessionId: string,
  limit?: number
): Promise<Message[]> {
  return invoke("chat_get_messages", { sessionId, limit });
}

/** 发送消息 */
export async function sendMessage(
  sessionId: string,
  content: string
): Promise<Message> {
  return invoke("chat_send", { sessionId, content });
}

/** 删除会话 */
export async function deleteSession(sessionId: string): Promise<void> {
  return invoke("chat_delete_session", { sessionId });
}

/** 归档会话 */
export async function archiveSession(sessionId: string): Promise<void> {
  return invoke("chat_archive_session", { sessionId });
}
