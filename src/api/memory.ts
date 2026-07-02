import { invoke } from "@tauri-apps/api/core";
import type { Memory } from "../types";

/** 获取记忆列表 */
export async function listMemories(category?: string): Promise<Memory[]> {
  return invoke("memory_list", { category });
}

/** 搜索记忆 */
export async function searchMemories(
  query: string,
  limit = 20
): Promise<Memory[]> {
  return invoke("memory_search", { query, limit });
}

/** 更新记忆 */
export async function updateMemory(
  id: string,
  updates: {
    content?: string;
    category?: string;
    importance?: number;
    isActive?: boolean;
    tags?: string;
  }
): Promise<void> {
  return invoke("memory_update", { id, ...updates });
}

/** 删除记忆 */
export async function deleteMemory(id: string): Promise<void> {
  return invoke("memory_delete", { id });
}

/** 记忆统计 */
export async function getMemoryStats(): Promise<{
  total: number;
  category_count: number;
}> {
  return invoke("memory_stats");
}
