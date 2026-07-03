import { invoke } from "@tauri-apps/api/core";
import type { ActivityLog } from "../types";

/** 获取当前活动信息 */
export async function getCurrentActivity(): Promise<ActivityLog | null> {
  return invoke("activity_get_current");
}

/** 获取活动统计（各活动类型总时长） */
export async function getActivityStats(
  since?: number
): Promise<Array<{ activity: string; total_seconds: number }>> {
  return invoke("activity_get_stats", { since });
}
