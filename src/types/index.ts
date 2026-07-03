// ==================== Session ====================
export interface Session {
  id: string;
  title: string;
  created_at: number;
  updated_at: number;
  message_count: number;
  summary: string | null;
  is_archived: boolean;
}

// ==================== Message ====================
export interface Message {
  id: string;
  session_id: string;
  role: "user" | "assistant" | "system";
  content: string;
  created_at: number;
  model: string | null;
  tokens_used: number | null;
  metadata: string | null;
}

// ==================== Memory ====================
export type MemoryCategory =
  | "preference"
  | "fact"
  | "habit"
  | "relationship"
  | "work"
  | "emotion"
  | "interest";

export interface Memory {
  id: string;
  content: string;
  category: MemoryCategory;
  importance: number;
  source_session: string | null;
  source_message: string | null;
  created_at: number;
  last_recalled_at: number | null;
  recall_count: number;
  is_active: boolean;
  user_edited: boolean;
  tags: string | null;
}

// ==================== Config ====================
export interface ConfigEntry {
  key: string;
  value: string;
  updated_at: number;
}

export interface PersonaConfig {
  name: string;
  personality: string;
  style: string;
  nickname: string;
}

export interface TtsConfig {
  enabled: boolean;
  voice: string;
  rate: number;
  volume: number;
}

export interface ProactiveConfig {
  enabled: boolean;
  quiet_start: string;
  quiet_end: string;
  min_interval_minutes: number;
}

export interface LlmConfig {
  provider: "openai" | "deepseek" | "qwen" | "claude" | "custom";
  model: string;
  max_context_messages: number;
  temperature: number;
  api_key?: string;
  api_base?: string;
}

// ==================== Activity ====================
export type ActivityType =
  | "coding"
  | "learning"
  | "entertainment"
  | "social"
  | "work"
  | "idle";

export interface ActivityLog {
  id: number;
  activity: ActivityType;
  window_title: string | null;
  process_name: string | null;
  started_at: number;
  ended_at: number | null;
  duration_seconds: number | null;
}

// ==================== Reminder ====================
export interface Reminder {
  id: string;
  content: string;
  remind_at: number;
  is_recurring: boolean;
  recurring_rule: string | null;
  is_done: boolean;
  source_session: string | null;
  created_at: number;
}

// ==================== Events ====================
export interface StreamChunkEvent {
  session_id: string;
  chunk: string;
}

export interface StreamDoneEvent {
  session_id: string;
  message_id: string;
}

export interface ConfigChangedEvent {
  key: string;
  value: string;
}

export interface ActivityChangedEvent {
  activity: ActivityType;
  window_title: string | null;
  process_name: string | null;
}
