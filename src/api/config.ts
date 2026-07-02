import { invoke } from "@tauri-apps/api/core";
import type { ConfigEntry } from "../types";

/** 获取配置项 */
export async function getConfig(key: string): Promise<ConfigEntry | null> {
  return invoke("config_get", { key });
}

/** 获取所有配置 */
export async function getAllConfigs(): Promise<ConfigEntry[]> {
  return invoke("config_get_all");
}

/** 设置配置项 */
export async function setConfig(
  key: string,
  value: object
): Promise<void> {
  return invoke("config_set", { key, value: JSON.stringify(value) });
}

/** 获取并解析配置 */
export async function getParsedConfig<T>(key: string): Promise<T | null> {
  const entry = await getConfig(key);
  if (!entry) return null;
  try {
    return JSON.parse(entry.value) as T;
  } catch {
    return null;
  }
}
